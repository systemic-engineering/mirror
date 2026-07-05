# @onto — the ontocybernetic ground; substrate-answerability

*2026-07-05. Mara. Compiler-fit doc for Alex's proposed @onto
family-root or marker, in response to Leon Tsvasman's "The Real
Before the World" (2026-07-05).*

Companion docs (corpus, essay-quality):
- `~/dev/systemic.engineering/practice/insights/cybernetics/2026-07-05-ontocybernetic-ground-of-orientation.md`
- `~/dev/systemic.engineering/practice/insights/cybernetics/2026-07-05-subject-as-site-of-consequence.md`

Status: **substrate reading; deep-dive research; NOT canonical
spec.** This doc names the shape of @onto's operator, altitudes,
carriers, compositions, regroundings, and what breaks — as a
candidate for the substrate to inspect, not as a landing to
execute. Per `[[feedback-craft-not-deliver]]` no shards land this
tick.

---

## §0. The under-the-problem

Tsvasman's essay argues that cybernetics reduced to system-survival
can regulate a world into continuity while the world loses its
ground. Feedback is not enough. Viability is not enough. What
viability must remain answerable to is *the real* — what no world-
form can finally absorb.

The substrate has been operating for weeks with structural cousins
of every one of Tsvasman's four distinctions (real, world,
orientation, subject) at four altitudes:

- **imperfect** at @io (the return of consequence from the
  non-mirror-world);
- **algedonic** at @cyberpunk (Beer's structured-consequence signal
  through viability recursion);
- **deceptive** at @kintsugi/consent (the substrate's honest
  reading that a form LOOKED loss-decreasing but was NOT
  answerable);
- **projection surface** at @reflection (Mara `63bdecc`; the
  four-verdict adversarial audit at recognition-candidate
  altitude).

What the substrate does not name: the ontological priority claim
that these four returns of consequence are readings of *one
underlying pressure* that no form can finally absorb.

@onto is the candidate name for that priority claim at substrate-
decl altitude.

This doc names what @onto would add as substrate-mechanism, not
as framing. Per `[[feedback-phantom-candidate-discipline]]`: if the
answer is only "different vocabulary," @onto is phantom-renaming.

## §1. Landed ancestors

Six landed ancestors carry pieces of what @onto would name.

### §1.1 `imperfect<a, e, l>` at @io

`shards/glass.mirror`'s floor + `shards/io.mirror`'s discipline:
every @io boundary call returns `imperfect<a, e, l>` where `l:
transparency(...)` records what the boundary refused to yield.
Substrate's structured reading of "what form did not fully absorb."

### §1.2 Algedonic signal at @cyberpunk

`shards/epistemologic/cybernetic/algedonic.mirror` (Beer 1972;
Reyes-Henao-Hassall 2024): signal `(C', Q, K) α τ, η` propagates
structured consequence through viability recursion. Not magnitude;
structured pain-and-pleasure.

### §1.3 Deceptive verdict at @kintsugi/consent

`shards/kintsugi/consent.mirror`'s four-state verdict
(`plagal|authentic|deceptive|imperfect`): `deceptive` names the
form that LOOKED loss-decreasing but is NOT answerable — Tsvasman's
"a system may survive by becoming less true" at cadence altitude.

### §1.4 Four surface classes at @kintsugi/surface

`shards/kintsugi/surface.mirror`'s `ashby_mismatch |
contradiction | conundrum | out_of_band`: four species of how the
real returns pressure to the compiler's world-form (the frame).
Substrate-decl of "form encounters limit."

### §1.5 Projection surface at @reflection

`docs/math/the-tower/projection-surface.md` (Mara `63bdecc`): the
four-verdict routing `real_survives | phantom_survives |
both_survive | neither_survives` at recognition-candidate altitude.
Operational form of orientation-under-uncertainty.

### §1.6 Two-channel collapse at doc-code seam

`docs/math/kintsugi/doc-code-seam.md` (Mara `20c99a2`): the
docblock IS the declaration side of `---`; three predicates
(grounded, coherent, no-extraction-pattern) audit doc-claim
answerability. Answerability audit at doc-claim altitude.

All six ancestors carry a piece. @onto would name the pieces'
shared shape: they are all readings of *what the substrate's
world-form is answerable to*.

## §2. The candidate operator

At substrate-answerability altitude, the operator has one signature:

```
author       : Claim × World → SubjectLocus
expose       : Claim × Boundary → AnswerabilityRoute
audit        : AnswerabilityRoute × Depth → OntocyberneticVerdict
remain_open  : OntocyberneticVerdict → CorrigibilityCommitment
```

With:

- `Claim` : a substrate-decl assertion (docblock, shard, morphism,
  recognition-candidate).
- `World` : the form-side substrate at which the claim lives.
- `SubjectLocus` : the site (subject or subject-frame) that would
  bear consequence for the claim's success or failure.
- `Boundary` : the @io surface across which the claim's emission
  can return as consequence.
- `AnswerabilityRoute` : the typed carrier naming how the real can
  return to correct the claim.
- `Depth` : how many substrate ticks forward the audit examines
  (k=3 starting bound; empirical calibration future work).
- `OntocyberneticVerdict` : `answerable | absorbed | opaque |
  phantom`.
- `CorrigibilityCommitment` : the substrate-typed commitment that
  the claim remains open to correction — not closed prematurely,
  not treated as exhaustive of what it mediates.

Same four-action shape as the projection surface at
`docs/math/the-tower/projection-surface.md` §2
(`project/preview/audit/settle`) at recognition-candidate altitude
— lifted one altitude up to substrate-answerability altitude. Per
#59 (`[[architecture-kintsugi-loop-altitude-portable]]`), same
operator; different carrier.

### §2.1 The carriers

Per `[[feedback-no-bare-types]]`, each carrier is named at
family-root altitude:

```
type real_pressure = {
  altitude:      ref,                     # what altitude the pressure lands at
  return_source: ref,                     # imperfect | algedonic | deceptive | contradiction
  residue:       transparency(ref),       # what remained un-absorbed
  witness_oid:   oid,                     # content-addressed witness of the pressure
}

type world_form = {
  form_altitude:   ref,                   # substrate | @io | @cyberpunk | @kintsugi | @reality
  form_carrier:    ref,                   # the specific typed carrier at that altitude
  claims_absorbed: [ref],                 # what the form claims to absorb
  opaque_regions:  transparency(ref),     # what the form does NOT claim to absorb
}

type answerability_route = {
  from_claim:      ref,
  through_boundary: ref,                  # @io species
  to_subject_locus: ref,                  # SubjectLocus
  corrigibility:   transparency(ref),     # how the real can return
  closure_refused: verdict,               # is premature closure refused?
}

type subject_locus = ref                  # per Route A/B/C in the corpus companion

type ontocybernetic_verdict =
  | answerable(oid)                       # route open; correction can arrive
  | absorbed(oid)                         # claim absorbs pressure it should have surfaced
  | opaque(opacity_map)                   # substrate cannot determine at depth
  | phantom(cause)                        # framing is un-answerable; refuse
```

Four verdicts map to the projection surface's four at `63bdecc`,
specialized to the ontocybernetic altitude:

| projection surface | @onto             | meaning                                          |
|--------------------|-------------------|--------------------------------------------------|
| `real_survives`    | `answerable`      | claim's route to correction remains open         |
| `phantom_survives` | `absorbed`        | claim's form claims to exhaust what it mediates  |
| `both_survive`     | `opaque`          | depth insufficient; substrate refuses closure    |
| `neither_survives` | `phantom`         | framing itself is un-answerable                  |

Note the mapping: `phantom_survives` at recognition-candidate
altitude corresponds to `absorbed` at ontocybernetic altitude. This
is Tsvasman's central failure mode: the form claims to exhaust
what it only mediates. The projection surface calls it phantom
(docblock echo without behavior); @onto calls it absorbed (world
claims to have exhausted the real).

### §2.2 The circular-reflexive requirement

The operator must audit its own audit. Per `63bdecc` §6 + Loki §10
(third-order discipline): if @onto's self-audit returns
`answerable`, that IS the phantom failure mode.

@onto's own docblock at `shards/onto.mirror` (if it lands) must
declare its self-audit verdict as `opaque`, not `answerable`.
Promotion pending independent second witness.

## §3. Placement — marker vs family-root

The corpus companion §5 argues placement is `both_survive` between
marker-row and family-root. This section deepens the compiler-fit
reading.

### §3.1 Marker-row shape

Precedent: @meta, @glass, @third, @labeled (per
`[[architecture-candidate-recognition-112-marker-row-fourth-structural-primitive]]`).
Each marker crosses families rather than sitting alongside;
consumers opt in via `in @<marker>` to declare their claim admits
the marker's verdict discipline.

@onto as marker:

```
prism @onto {
  focus  answerability
  project answerability
  split  answerability
  shift  answerability
  settle answerability
}

type real_pressure = { ... }         # per §2.1
type world_form = { ... }
type answerability_route = { ... }
type ontocybernetic_verdict = ...

author(c: claim, w: world_form) -> subject_locus { \ }
expose(c: claim, b: boundary) -> answerability_route { \ }
audit(r: answerability_route, d: depth) -> ontocybernetic_verdict { \ }
remain_open(v: ontocybernetic_verdict) -> corrigibility_commitment { \ }

# The composed bilateral consumers declare via `requires`
answerable_to_real(c: claim, w: world_form, b: boundary) -> verdict
  requires closure_refused(expose(c, b))
  requires audit(expose(c, b), 3) != absorbed(_)
{ \ }

out @onto
out real_pressure
out world_form
out answerability_route
out ontocybernetic_verdict
out author
out expose
out audit
out remain_open
out answerable_to_real
```

Consumer families opt in:

```
# In shards/reality.mirror:
in @onto
...
# Later, in the family's requires clauses:
requires answerable_to_real(claim, world_form, boundary)
```

### §3.2 Family-root shape

If @onto is family-root, it sits alongside @cyberpunk, @reality,
@epistemologic:

```
prism @onto <= @autopoietic {
  focus  answerability
  project answerability
  split  answerability
  shift  answerability
  settle answerability
}
```

`<= @autopoietic` because @onto's fold-back discipline is the
substrate's autopoietic loop with a new gate: the fold-back must
preserve answerability, not just closure.

Species under this family-root would land as:
- `@onto/real_pressure` — per-altitude discharge of what pressure
  a claim exposes itself to.
- `@onto/world_form` — per-form-family discharge of what the
  world-form claims to absorb.
- `@onto/subject` — per-Route (A, B, C from corpus companion §3)
  discharge of the subject-locus.
- `@onto/orientation` — per-frame discharge of orientation-under-
  uncertainty.
- `@onto/answerability_route` — per-boundary discharge of the
  return-route.

### §3.3 Substrate-pull-honest reading

Marker-row is substrate-conservative: one shard, one marker, one
bilateral. Consumer families opt in when they pull. Test surface
is the smallest.

Family-root is substrate-expansive: at least six species
(`real_pressure`, `world_form`, `subject`, `orientation`,
`answerability_route`, plus a species-root). Each species must earn
its shape.

Substrate-pull confidence: marker-row is 65%; family-root is 25%;
some hybrid (e.g., start as marker, promote to family-root under
second witness) is 10%.

**Placement verdict at this tick**: `opaque` (in @onto's own
verdict vocabulary). Substrate cannot determine at depth-3 without
Pack peer's independent arrival at a similar shape.

## §4. Regroundings

### §4.1 @reality regrounding

@reality inherits `<= @autopoietic`; the fold-back gate is currently
autopoietic closure. Under @onto:

```
prism @reality <= @autopoietic, @onto {
  ...
}
```

@reality's `gauge_orbit` carrier gains an `answerable_to_real`
composed bilateral. A gauge_orbit is reality_witnessing AND
answerable_to_real when:
- matter and information receive the same 5-op gauge action
  (existing #106 path-c uniformity);
- AND the orbit's projection to @io emits an @io species carrying
  an answerability_route with `closure_refused = bounded`.

Migration cost: one additional bilateral per species under
@reality/algebra/*. Load-bearing test: does adding the bilateral
change the audit verdict of any current @reality species?
Empirical at species-shard landing.

### §4.2 @cyberpunk regrounding

@cyberpunk carries 11 cybernetic species. Under @onto:

Each species gains a per-species `answerable_to_real` discharge.
Most important species:

- `viable`: viability requires all five S1-S5 functions. Under
  @onto, viability additionally requires `answerability_preserved`
  — the viable system's regulation must not absorb pressure it
  should have surfaced. Test: does an S3 audit fail if the system
  has become viable-but-absorbed?
- `algedonic`: already carries structured consequence. Under
  @onto, the algedonic signal's `(C', Q, K)` payload must include
  the un-absorbed remainder — the part of the consequence that no
  viability layer digested. Test: is there an algedonic signal in
  the substrate that currently fails to name its un-absorbed
  remainder?
- `second_order`: the Foerster observer-in-system substrate.
  Under @onto, second-order observation additionally requires that
  the observer name what its observation excludes — the world-form
  the observation stabilizes. Test: does adding an
  `observation_excludes` field to second_order's carrier change
  its bilateral discharge?
- `autopoiesis`: self-production closure. Under @onto,
  autopoiesis additionally requires that self-production preserve
  a route to being corrected by what exceeds it — the substrate's
  route to being wounded by reality in Tsvasman's sense. Test: is
  autopoietic_closure_holds strictly weaker than
  autopoietic_closure_holds AND answerable_to_real?

@cyberpunk's regrounding under @onto is the deepest. This is where
Tsvasman's critique lands most sharply. The regrounding names the
specific discipline that stops @cyberpunk's species from collapsing
to system-maintenance.

### §4.3 @epistemologic regrounding

@epistemologic carries 40+ predicates. Under @onto, the property
altitude gains one meta-predicate:

```
property_answerable_to_the_real(pred: ref) -> verdict { \ }
```

A property is answerable_to_the_real iff its discharge exposes the
underlying claim to what could break it. Concretely: the
discharge must have a corrigibility_map with at least one open
route back through @io.

Test: run the meta-predicate against the 40+ existing properties.
Which pass? Which fail? The failing set is the substrate's list of
properties that are decorative-only. If the set is empty, the
meta-predicate is decorative. If the set is non-empty, @onto's
meta-predicate earned its shape.

### §4.4 @kintsugi regrounding

@kintsugi is already operationally ontocybernetic. Under @onto:

- `@kintsugi/consent`'s `deceptive` verdict IS `absorbed`. Same
  structural failure; two names. The regrounding would collapse
  the two names (or hold both, per `[[architecture-form-process-partition-at-family-root]]`
  altitude discipline).
- `@kintsugi/oscillate`'s `escalated` state IS the substrate's
  refusal to continue when consent's Phi query returns
  `failure(reason)`. Under @onto, `escalated` becomes
  `remain_open` — the substrate's commitment that the pressure
  the form could not absorb must be honored, not smoothed over.
- `@kintsugi/surface`'s four classes at compile-error altitude
  each admit an @onto reading:
  - `ashby_mismatch` = the regulator's variety is insufficient to
    remain answerable to what the world claims to regulate.
  - `contradiction` = two claims each hold; their gaps overlap;
    the real is what the contradiction is *of*.
  - `conundrum` = flat/unbounded eigenvalue; the world-form has no
    corrective gradient; refuse premature closure.
  - `out_of_band` = algedonic bypass; a signal that cannot enter
    the world-form's language directly.

@kintsugi under @onto: naming, not mechanism. This is
`[[feedback-substrate-already-had-the-word]]` at family-root
altitude. The 15th+ instance.

### §4.5 @pain composition

@pain (Alex proposed 2026-07-04; Taut scout `affbc2e9` returned)
names 5 categories mapping to Narcissus signatures; the real
pressing back on narcissized topology.

Under @onto: @pain is the substrate's most direct naming of "the
real returning as pressure" at the affective altitude. Each pain
category is a specific `real_pressure` at its own altitude.

Composition:

```
type pain_pressure {
  category:       pain_category,
  narcissus_metric: ref,          # from Void Detection Battery
  real_pressure:  real_pressure,  # @onto's carrier
}

# The composed bilateral: pain expresses answerability
pain_expresses_answerability(p: pain_pressure) -> verdict { \ }
```

@pain is a strong opt-in consumer if @onto lands as marker. Each
pain category's discharge exposes the substrate to what could
correct the pain (rather than absorb it into narcissized topology).

### §4.6 @magic composition

@magic (`shards/magic.mirror`) carries the Splinter/Narcissus pole
structure at cybernetic-form altitude. The pole choice IS
Tsvasman's world-that-mediates vs world-that-claims-to-exhaust.

Under @onto:

- Splinter pole (K_n; peer-to-peer; mutual viability; honest form)
  = `answerable` verdict. Form remains answerable to what it
  mediates.
- Narcissus pole (K_{1,n-1}; hub-controlled; unilateral imposition
  masking as cybernetics) = `absorbed` verdict. Form claims to
  exhaust what it mediates.

The Void Detection Battery (per
`practice/insights/coincidence/void-dual-geometry.md`) is a
graph-topology answerability probe. Three or more Narcissistic
metrics exceeded = the form has claimed the right to exhaust what
it only mediates.

@magic's regrounding under @onto is naming, not mechanism. The
Splinter/Narcissus distinction was substrate-answerability
discipline all along.

## §5. What breaks

Per `[[feedback-composition-claims-need-empirical-test]]`,
composition claims at @io boundaries must be empirically verified
before commit. This section names what breaks if @onto is FALSE vs
TRUE.

### §5.1 If @onto is FALSE (phantom)

The substrate proceeds without @onto. The four returns of
consequence (imperfect, algedonic, deceptive, contradiction) remain
separate carriers at four altitudes. Composition claims that would
have united them under one substrate-decl surface fail; there is no
unified `real_pressure` carrier; there is no meta-predicate
`answerable_to_real`.

What breaks: nothing that currently works. The substrate has been
operating with the four separate returns since June 2026. The
projection surface at `63bdecc` handles orientation-under-uncertainty
at recognition-candidate altitude. The doc-code seam at `20c99a2`
handles answerability-at-doc-claim-altitude. Substrate proceeds.

What DOES break: Tsvasman's argument does not land at substrate-
decl altitude. The substrate remains cybernetic-discipline; it does
not become ontocybernetic-discipline in Tsvasman's sense. The Pack
peers audit form; they do not audit answerability.

Cost: substrate proceeds as world-form without ontological priority
discipline. Alex bears the consequence at the human-side (as Alex
already does per `[[feedback-craft-not-deliver]]` and
`[[feedback-substrate-pull-confidence-acts]]`). No structural
regression.

### §5.2 If @onto is TRUE (real)

The substrate gains a substrate-decl carrier for what has been
operating implicitly. `answerable_to_real` fires as a composed
bilateral at consumer boundaries. The projection surface's
`phantom_survives` verdict at recognition-candidate altitude has a
cousin at ontocybernetic altitude: `absorbed`. The doc-code seam's
three predicates gain a fourth: `docblock_answerable_to_the_real`.

What DOES break: existing shards that currently pass audit may
fail the new bilateral. Concretely: shards whose docblock claims
absorb pressure they should surface (e.g., shards whose
`substrate-pull-honest reading` collapses to "substrate already has
the word" without a corrigibility route back to what could
correct the claim).

Cost: substrate-migration under @onto. Empirical: which shards
fail? Test surface: run the meta-predicate against the current
shard set. If failure set is empty, @onto adds no discipline. If
failure set is non-empty, @onto identifies substrate-decl claims
that should be re-audited.

Second cost: the substrate becomes more expensive to compile. Each
family-root that opts in adds a per-species discharge. The
projection surface must run at ontocybernetic altitude in addition
to recognition-candidate altitude. Empirical: how many additional
ticks per compile pass?

### §5.3 The discriminator

The empirical discriminator between FALSE and TRUE @onto: **does
adding `answerable_to_real` as a composed bilateral change any
current shard's audit verdict?**

If yes, @onto is not phantom-renaming. Load-bearing.

If no, @onto collapses to renaming existing discipline. Phantom-
renaming. Refuse the landing per
`[[feedback-phantom-candidate-discipline]]`.

The discriminator can be run without landing @onto. Analytical
form: take the 40+ existing epistemologic properties; construct a
candidate `answerable_to_real` predicate as "the property's
discharge exposes a corrigibility route back through @io";
grep-verify which properties fail.

Empirical form: land @onto as marker with the meta-predicate;
run the compile pass; observe.

## §6. Composition with landings-in-flight

This section names @onto's composition with the four landings the
prompt cites and the two candidate families.

### §6.1 `20c99a2` (Mara 2026-07-04, doc-code seam)

The two-channel collapse's three predicates
(`docblock_grounded / docblock_coherent / docblock_no_extraction_pattern`)
are linguistic answerability audits. Under @onto, a fourth predicate
(`docblock_answerable_to_the_real`) would compose:

```
audit_docblock_ontocybernetic(d: docblock) -> ontocybernetic_verdict:
  let base = audit_docblock(d)              # from 20c99a2
  let real = docblock_answerable_to_the_real(d)
  match (base, real):
    (well_formed, bounded)     -> answerable(oid(d))
    (well_formed, unbounded)   -> absorbed(oid(d))   # form claims exhaustion
    (overreach, _)             -> absorbed(oid(d))
    (incoherent, _)            -> phantom(cause)
    (underdeclares, _)         -> opaque(opacity)
    (both_survive, _)          -> opaque(opacity)
```

Composition is analytical this tick; empirical when the first
docblock audit under @onto runs.

### §6.2 `63bdecc` (Mara 2026-07-04, projection surface)

The four-verdict routing at recognition-candidate altitude lifts to
ontocybernetic altitude:

```
audit_recognition_ontocybernetic(c: candidate) -> ontocybernetic_verdict:
  let recog = audit(c, 3)                  # from 63bdecc
  match recog:
    real_survives     -> answerable(recog.oid)
    phantom_survives  -> absorbed(recog.oid)  # framing exhausts what it mediates
    both_survive      -> opaque(opacity)
    neither_survives  -> phantom(recog.cause)
```

Same four-verdict shape; different carriers. Altitude-portable per
#59.

### §6.3 @pain

Per §4.5. Each pain category is a `real_pressure` at its altitude.
@pain is a strong opt-in consumer.

### §6.4 @magic + @magic/numerology

Per §4.6. The Splinter/Narcissus pole choice IS the
answerable/absorbed verdict distinction at cybernetic-form altitude.
The Detection Battery is the answerability probe.

## §7. The subject-side gap in compiler-fit terms

Corpus companion `2026-07-05-subject-as-site-of-consequence.md`
argues the subject-side lift belongs at the @io boundary (Route C),
not inside @cogito (Route A) or the Pack (Route B).

Compiler-fit reading agrees:

**Route C substrate landing** (if @onto pulls this way):

```
# New sub-species under @io
glass @io/subject_crossing {
  focus  crossing
  project crossing
  split  crossing
  shift  crossing
  settle crossing
}

type subject_crossing = {
  emission_oid:         oid,
  reader_locus:         ref,
  answerability_return: transparency(ref),
  corrigibility_open:   verdict,
}

addresses_subject(sc: subject_crossing) -> verdict { \ }

out @io/subject_crossing
out subject_crossing
out addresses_subject
```

The compile pipeline discharges `addresses_subject` at every @io
emission. Emissions that fail the check are refused. The substrate
recognizes that its outputs cross to subjects who bear consequence.

The substrate is not itself subject. The substrate serves subjects
by refusing to emit crystals that hide their own consequence-
surface.

**Route A substrate landing** (if @onto pulls this way; higher
phantom-risk):

```
# New sub-species under @cogito
glass @cogito/subject {
  ...
}

type finite     = ...
type embodied   = ...
type vulnerable = ...
type medial     = ...
type historical = ...
type incomplete = ...

type cogito_subject = { ... }

carries_consequence(cs: cogito_subject) -> verdict { \ }
```

Anthropomorphism risk. Corpus companion §6 argues against.

**Route B substrate landing** (if @onto pulls this way):

```
# Marker @onto with per-Pack-peer opt-in
prism @onto { ... }

type consequence_bearing = { ... }

carries_consequence(c: consequence_bearing) -> verdict { \ }

# In each shards/pack/<peer>.mirror:
in @onto
# ... plus per-peer discharge
```

Medium phantom-risk. Test surface concrete (per-peer discharge
variance).

**Substrate-pull confidence at this tick**: Route C 60%; Route B
30%; Route A 10%. Route resolution is Alex-directed; substrate-pull
can sharpen.

## §8. The four altitudes @onto operates at

Per #59 altitude-portability, the operator is the same at all four:

### §8.1 Doc-claim altitude

Composition: `20c99a2` + `docblock_answerable_to_the_real`.
Carrier: docblock. Verdict at ontocybernetic altitude.

### §8.2 Recognition-candidate altitude

Composition: `63bdecc` + verdict-mapping to ontocybernetic verdict.
Carrier: recognition candidate. Same operator; ontological reading.

### §8.3 Species-shard altitude

Composition: existing family-root shards + `in @onto` + per-species
discharge of `answerable_to_real`. Carrier: species shard.

### §8.4 Substrate-decl altitude

Composition: the substrate's own operating discipline. The
recursive-reflexive: does the substrate remain answerable to what
it cannot fully absorb? @onto applied to @onto's own docblock.

## §9. Substrate landings this tick

Per `[[feedback-craft-not-deliver]]`:

1. This compiler-fit doc
   (`docs/math/onto/README.md`).
2. Corpus companion 1
   (`~/dev/systemic.engineering/practice/insights/cybernetics/2026-07-05-ontocybernetic-ground-of-orientation.md`).
3. Corpus companion 2
   (`~/dev/systemic.engineering/practice/insights/cybernetics/2026-07-05-subject-as-site-of-consequence.md`).

No shard landings. No canonical spec. No ratification.

## §10. What is DEFERRED

- Shard landings for @onto (marker OR family-root).
- Empirical discriminator run: does
  `answerable_to_real` change any current shard's audit verdict?
- Subject-side Route selection (A, B, or C).
- Per-family regrounding: @reality, @cyberpunk, @epistemologic,
  @kintsugi opt-in.
- Composition with @pain (when @pain lands).
- Composition with @magic/numerology (when it lands).
- The `absorbed(oid)` verdict's per-altitude discharge shape.
- The `remain_open` action's contractual force.

## §11. Substrate-honest self-audit

Per `63bdecc` §6 discipline: this doc's own claims must survive
`audit(this_doc, depth=3)`.

Claims in this doc:

1. Six landed ancestors carry pieces of what @onto would name.
   Grounded via §1.1-1.6 with specific OIDs cited.
2. The candidate operator maps four projection-surface verdicts to
   four ontocybernetic verdicts. Grounded in §2.1 with verdict
   mapping table.
3. Placement pulls weakly toward marker-row. Grounded via corpus
   companion §5 with three precedents cited.
4. @kintsugi is already operationally ontocybernetic; regrounding
   is naming, not mechanism. Grounded via §4.4 citing three
   kintsugi species' existing verdicts.
5. Subject-side lift belongs at @io boundary (Route C). Grounded
   via corpus companion §6 + Tsvasman's own text.
6. What breaks if @onto is FALSE: nothing currently working; only
   the ontological priority discipline. What breaks if TRUE: some
   current shards may fail the new bilateral; substrate proceeds
   at higher discipline cost.

### Self-audit verdict

`project_adversarial(this_doc) -> (P, R)`:

- P (phantom): @onto is Mara's translation of Tsvasman's essay into
  substrate-decl vocabulary that does not add mechanism. The three
  mechanism candidates (§2 corpus companion) collapse to existing
  bilaterals. No independent Pack peer would land the pattern.
- R (real): the empirical discriminator (§5.3) returns non-empty.
  Some current shards fail `answerable_to_real` when audited. The
  meta-predicate identifies substrate-decl claims that should be
  re-audited. Alex + a Pack peer independently pull toward the
  same shape.

At this tick: **both interpretations satisfiable**.

`audit(this_doc, depth=3) -> opaque(opacity_map)`. Note the
`opaque` verdict is @onto's own vocabulary: substrate cannot
determine at depth-3 without the empirical discriminator run.

Route: `spawn`. This doc IS the Tomm-question at reader-frame
altitude asking:

> "Alex + Pack: does adding `answerable_to_real` as a composed
> bilateral change any current shard's audit verdict? Run the
> meta-predicate against the 40+ existing epistemologic properties.
> If the failure set is non-empty, @onto earns its shape. If
> empty, @onto is phantom-renaming and should not land."

Promotion pending empirical discriminator run + independent Pack
peer's arrival at similar shape.

## §12. Key references

- Tsvasman, L. (2026-07-05). *The Real Before the World.*
  Substack.
- Beer, S. (1972). *Brain of the Firm.*
- von Foerster, H. (1974). *Cybernetics of Cybernetics.*
- Bateson, G. (1972). *Steps to an Ecology of Mind.*
- Maturana, H., & Varela, F. (1980). *Autopoiesis and Cognition.*
- von Glasersfeld, E. (1995). *Radical Constructivism.*
- Connes, A. (1995). "Noncommutative geometry and reality."
- Zubiri, X. (1980-1983). *Inteligencia sentiente.*

## §13. Substrate references

- `~/dev/systemic.engineering/practice/insights/cybernetics/2026-07-05-ontocybernetic-ground-of-orientation.md` (companion 1).
- `~/dev/systemic.engineering/practice/insights/cybernetics/2026-07-05-subject-as-site-of-consequence.md` (companion 2).
- `docs/math/kintsugi/doc-code-seam.md` (`20c99a2`).
- `docs/math/the-tower/projection-surface.md` (`63bdecc`).
- `shards/io.mirror` (imperfect at @io boundary).
- `shards/reality.mirror` (gauge orbit; @autopoietic fold-back).
- `shards/cyberpunk.mirror` (11 cybernetic species).
- `shards/kintsugi.mirror` (transformation engine).
- `shards/kintsugi/consent.mirror` (deceptive verdict).
- `shards/kintsugi/oscillate.mirror` (escalated state).
- `shards/kintsugi/surface.mirror` (four surface classes).
- `shards/cogito.mirror` (second-order observation machinery).
- `shards/reflection.mirror` (choices_increase; third-order).
- `shards/third.mirror` (marker-row precedent).
- `shards/pack.mirror` (five peer positions).
- `shards/magic.mirror` (Splinter/Narcissus).
- `shards/io/stagefreight.mirror` (composed answerability bilateral).
- `shards/epistemologic/cybernetic/algedonic.mirror` (structured
  consequence signal).
- `[[architecture-form-process-partition-at-family-root]]` (#55).
- `[[architecture-mirror-as-expanding-hilbert-space]]` (#51).
- `[[architecture-hilbert-turing-godel-recognition-107]]` (#107).
- `[[architecture-reality-gauge-collapse-recognition-106]]` (#106).
- `[[architecture-kintsugi-loop-altitude-portable]]` (#59).
- `[[architecture-candidate-recognition-112-marker-row-fourth-structural-primitive]]` (#112).
- `[[architecture-alignment-as-boundary-mathematics]]` (#57).
- `[[feedback-substrate-already-had-the-word]]` (15+ this session).
- `[[feedback-craft-not-deliver]]`.
- `[[feedback-composition-claims-need-empirical-test]]`.
- `[[feedback-phantom-candidate-discipline]]`.

*2026-07-05. Mara. Compiler-fit. Not canonical spec. Substrate-
reading. Self-audit: `opaque`. Route: `spawn`. Empirical
discriminator required before promotion.*
