# The projection surface — previewing future substrate shape

*2026-07-04. Mara. Compiler-fit doc for the projection surface
recognition. Companion to the corpus doc at
`systemic.engineering/practice/insights/coincidence/projection-surface-and-phantom-recognitions.md`
(landed same tick).*

Status: **substrate reading**. The projection surface is not a new
primitive. It is a routing-composition of four landed ancestors
named in §1. This doc names the operator, the carrier, the altitude,
and two audience-specific fits: Pack workflow (would have caught
today's phantom-arc) and compiler (what operator projects, at what
altitude, over what carrier).

---

## §0. The under-the-problem

On 2026-07-03, a recognition candidate provisionally numbered #141
propagated through the Pack pipeline without corresponding to any
substrate-observable behavior change. The pipeline processed it
faithfully; adversarial voices assigned structurally-different
verdicts; downstream docblocks began to echo the framing.

Alex named the under-the-problem: **there is no mechanism to project
recognitions into future shape.** Composition claims have empirical
falsification (per [[feedback-composition-claims-need-empirical-test]],
Seam's 2026-06-18 catch on isError-on-REJECT non-firing); recognition
candidates don't. They only get evidence-echoed-back through docblock
language.

This doc claims the substrate already had the mechanism at grammar
altitude — the `@projection` grammar plus `preview` verdict per
Reed's 2026-03-27 spec plus the `---` separator per
Reed + Alex's 2026-05-19 spec — and that lifting it to
recognition-candidate altitude requires no new substrate primitive,
only a routing-composition of the landed vocabulary.

---

## §1. Landed ancestors

### §1.1 The `@projection` grammar (Reed, 2026-03-27)

At `docs/specs/historical/2026-03-27-projection-properties-as-plans.md`
Reed declared:

    grammar @projection {
      type = projection | preview | delta
      type preview = satisfiable | unsatisfiable | partial
      type delta   = converged | diverged

      action project { spec: grammar }
      action preview { projection: projection }
      action measure { projection: projection, actual: grammar }
    }

Four operations. `project` snapshots a spec's property declarations
as a content-addressed OID:
`hash(requires_list, invariants_list, ensures_list)`. Same properties
→ same OID. `preview` model-checks the projection for internal
consistency: satisfiable (execution can begin), unsatisfiable (the
spec itself is contradictory), or partial (some properties can't
be checked statically). `measure` compares projection with actual
compilation verdicts and emits a `delta`.

The lifecycle:

    1. Write spec
    2. Project     → projection OID
    3. Preview     → satisfiable / unsatisfiable / partial
    4. Execute     → any implementation satisfying the properties
    5. Verify      → compilation produces property verdicts
    6. Measure     → delta between projection and actual

Steps 2–3 are the projection *before* implementation. Unsatisfiable
at step 3 IS phantom-detection at compile time.

### §1.2 The `---` separator (Reed + Alex, 2026-05-19)

At `docs/specs/property-projection.md` Reed + Alex declared the
boundary:

    above ---   declaration (the programmer's writing)
    ---
    below ---   observation (the compiler's measurement)

Both sections content-addressed. The OID hashes both. This IS
Ryu-Takayanagi at compilation altitude: the declaration is the
boundary region A; the observation is the minimal bulk surface
γ_A whose area IS the entropy S(A); the full grammar tree is the
bulk. Content-addressing makes the correspondence bijective
(stronger than AdS/CFT's conjectural bulk-boundary duality).

### §1.3 Geometric consent projection (Mara, 2026-06-17)

At `docs/specs/geometric-consent-projection.md` §2 the projection
lifts to consent geometry: consent at type N+1 wraps consent at
type N; the cascade IS a gauge transformation on the sheaf of
sections; the security invariant IS the asymmetry of the
projection. Recognition-candidate promotion is a specific case:
candidates cascade from candidate → forward-promised → promoted
through type-N wrappings that consent geometry's gauge action
already formalizes.

### §1.4 Projection-testing at fragmentation altitude (Alex + Mara, 2026-03-24)

At `fragmentation/projection-testing.md` (in the systemic.engineering
corpus) the projection surface lifts to test altitude: **tests are
projections**; the commit hash IS the eigenvalue across all
projections; the commit CANNOT EXIST without every projection
agreeing. Coverage is grammar-based, not line-based: each
projection knows its subspace via its projection matrix.

This is the ancestor closest to what recognition-candidate promotion
needs. If a recognition candidate is a projection, its promotion
requires the substrate's other projections to agree; the promoted
OID cannot exist without the agreement.

---

## §2. The operator

At recognition-candidate altitude, the projection surface has one
signature:

    project : Candidate × Altitude → BoundaryOID
    preview : BoundaryOID × Depth → Verdict
    audit   : BoundaryOID × Depth → PhantomTest
    settle  : Verdict → PostBoundary

With:

- `Candidate` : a recognition, morphism, or substrate-pull claim.
- `Altitude`  : bundle-tower level per
  `docs/math/the-tower/altitudes.md` §1.
- `BoundaryOID` : content-addressed hash of what the candidate
  DECLARES above its `---`.
- `Depth` : how many substrate ticks forward to model-check
  (k=3 as a starting bound; empirical calibration future work).
- `Verdict` : `satisfiable | unsatisfiable | partial | bistable`.
- `PhantomTest` : `(P_survives, R_survives)` — the two adversarial
  interpretations' preview verdicts.
- `PostBoundary` : the OID after the observation-side is written.

The operator inherits Ashby's law at meta-altitude: the projection
surface must have variety sufficient to distinguish its own valid
applications from its own phantom applications. Otherwise it
certifies itself vacuously.

### §2.1 The carrier

Per [[feedback-no-bare-types]], each carrier is named:

    type projection_candidate = ref     # the claim being tested
    type projection_boundary  = oid     # hash(above ---)
    type projection_depth     = nat     # k ticks forward
    type projection_verdict   = satisfiable(oid)
                              | unsatisfiable(oid, cause)
                              | partial(opacity_map)
                              | bistable(oid, oid)
    type projection_test      = phantom_survives(oid)
                              | real_survives(oid)
                              | both_survive(oid, oid)          # audit branch
                              | neither_survives(cause)         # framing wrong

`bistable` is Necker-shaped: two valid bulks compete for the same
boundary. Recognition #109 (math reflexive A-side closure vs.
absorption into #108) is the landed example — not phantom, genuinely
two interpretations.

`both_survive` is Kanizsa-adjacent: the current depth is insufficient
to discriminate. Either k must increase or the discriminator must
sharpen. This is the `audit` verdict that #141 would have received
on 2026-07-03.

`neither_survives` fires when both P (phantom interpretation) and
R (real interpretation) are unsatisfiable — the framing itself is
wrong; the substrate-pull audit must be re-run.

### §2.2 The circular-reflexive requirement

The projection surface must project itself. Substrate-decl:

    prism @projection/surface {
      focus surface
      project surface
      split surface
      shift surface
      settle surface
    }

    self_preview() -> verdict
      requires can_project(this_surface)
            && preview(project(this_surface), depth=3) = satisfiable
    { \ }

If the operator cannot preview its own next 3 ticks as satisfiable,
it is not the operator we want. Kauffman eigenform (Kauffman 2003,
*C&HK* 10:73-90; landed at
`shards/epistemologic/cybernetic/eigenform.mirror`) grounds this:
the projection surface is the fixed point of its own application.

This is not sufficient. Per
[[architecture-hilbert-turing-godel-recognition-107]], substrate-decl
is bounded / Gödel-incomplete; a broken operator may certify itself.
The audit branch remains structurally required (§3).

---

## §3. The audit branch — falsification-by-adversarial-interpretation

When `preview(BoundaryOID, k)` returns satisfiable, that is not
sufficient for promotion. The projection surface must additionally
generate an adversarial interpretation:

    project_adversarial : Candidate → (P_interp, R_interp)

With:

- `P_interp` (phantom interpretation): what the substrate looks
  like at n+k moves IF the candidate is docblock-echo with no
  behavioral content. Concretely: downstream sites reference the
  candidate but no independent substrate-behavior change fires.
- `R_interp` (real interpretation): what the substrate looks like
  at n+k moves IF the candidate is load-bearing. Concretely: a
  second substrate site independently exhibits the pattern
  without having been nudged by the candidate.

The audit:

    audit(candidate, depth=k) -> phantom_test:
      p_verdict = preview(project(P_interp(candidate)), depth=k)
      r_verdict = preview(project(R_interp(candidate)), depth=k)
      match (p_verdict, r_verdict):
        (unsatisfiable, satisfiable)   → real_survives(r_verdict.oid)
        (satisfiable, unsatisfiable)   → phantom_survives(p_verdict.oid)
        (satisfiable, satisfiable)     → both_survive(...) # increase k or sharpen
        (unsatisfiable, unsatisfiable) → neither_survives(cause)

The four-verdict outcome maps to `compiler-error-surface.md`'s
three-mode algebra plus a fourth branch:

| audit verdict         | routing              |
|-----------------------|----------------------|
| `real_survives`       | `apply` (candidate promotable) |
| `phantom_survives`    | `hold(candidate)` (retract from pipeline) |
| `both_survive`        | `spawn` (peer tournament over discriminator) |
| `neither_survives`    | audit-loop (re-run substrate-pull) |

The `spawn` branch here is what today's failure mode needed: emit
a Tomm-shaped question at reader-frame altitude asking Alex to name
the discriminator that distinguishes P from R. Per
`compiler-error-surface.md` §3, this is `ashby_mismatch` at
recognition-candidate altitude — the projection surface's regulator
variety fell short of distinguishing phantom from real.

---

## §4. Altitude, carrier, and composition

### §4.1 What altitude does the operator sit at?

Per `docs/math/the-tower/altitudes.md` §2, the recognition-candidate
altitude is at **reflection altitude (n=2)**. Reflection observes
the compiler's substrate-pull; candidates are candidate morphisms
the substrate might apply next; the holonomy is
`α·loss + β·contradictions`.

The projection surface at recognition-candidate altitude is a
sub-species of `@reflection` living at
`shards/reflection/projection.mirror` (forward-promised). Its
substrate-decl inherits `@reflection`'s carriers (`frame`,
`candidate_morphism`, `Bateson_altitude`) and adds the four
carriers from §2.1.

The operator itself is altitude-portable per
[[architecture-kintsugi-loop-altitude-portable]] (#59): the same
shape works at grammar altitude (Reed's original spec), test
altitude (`fragmentation/projection-testing.md`), consent altitude
(Mara's geometric-consent spec), morphism altitude (kintsugi's
surface act), and now recognition-candidate altitude.

### §4.2 What carrier does it project over?

At recognition-candidate altitude, the projection carrier is a
**substrate-pull claim**: a candidate assertion about the
substrate's structure that will (if real) manifest as an
independently-witnessable pattern at some altitude within k ticks.

The carrier IS what makes the operator falsifiable. A candidate
that only asserts "the substrate implicitly does X" without a
witness-shape is unprojectable — its `project` returns a boundary
with no bulk (RT-γ_A undefined). The projection surface should
reject such candidates at the `project` step, not at `preview`.

### §4.3 How does it compose with landed operators?

- **With `@kintsugi/surface`** (`shards/kintsugi/surface.mirror`):
  the projection surface is what routes the `audit` verdict to
  the three-mode algebra. `phantom_survives` → `hold`;
  `both_survive` → `spawn` a peer tournament over discriminators;
  `real_survives` → `apply` (promote the candidate).
- **With `@epistemologic/cybernetic/coherence-parametric`
  .ashby_variety_match**: the projection surface consumes this
  predicate as its regulator-variety measurement. Insufficient
  variety fires `both_survive` (audit branch).
- **With `@reflection`** (`shards/reflection.mirror`): the projection
  surface lands as a sub-species of the reflection family;
  reflection's `observe → tournament → compose → pick` pipeline
  runs the projection surface before promotion.
- **With `@third`** (`shards/third.mirror`): the projection surface
  emits the fourth predicate at recognition altitude — the
  observer (Pack) observes the observer (Reflection) observing
  the observer (substrate's substrate-pull). This IS third-order.

### §4.4 What does the substrate need to grow?

Minimal substrate landings for v1:

1. `shards/reflection/projection.mirror` — the sub-species
   declaring the four carriers, the `project` / `preview` /
   `audit` / `settle` actions, and the four-verdict `phantom_test`.
2. `shards/epistemologic/property/projection_preview_satisfiable
   .mirror` — the declarative property that a projected candidate's
   preview verdict is satisfiable at depth k. Bilateral pair with
   `shards/kintsugi/fracture/preview_unsatisfiable.mirror` (forward-
   promised) per the #53 pattern.
3. `shards/reflection/adversarial.mirror` — the sub-species
   declaring `project_adversarial` and the two-interpretation
   generator. Requires a `phantom_interpretation` /
   `real_interpretation` carrier pair.
4. `docs/math/kintsugi/compiler-error-surface.md` amendment
   adding a fifth surface class or an audit sub-frame under
   `ashby_mismatch` — whichever the substrate pulls when v1 lands.

Each landing is small. The composition is what buys the falsification
discipline.

---

## §5. Pack workflow — how today's phantom-arc would have been caught

**On 2026-07-03**, Reed injected candidate #141 (conditional-marker-
import-at-species-altitude precedent) into the Pack pipeline. The
pipeline processed it faithfully. The failure mode: no branch of
the pipeline was positioned to test the candidate against the
substrate directly. Adversarial voices (Seam + Taut) landed
structurally-different verdicts because they were adjudicating
the *docblock*, not the *substrate-behavior*. Downstream docblocks
echoed Reed's framing.

**Under the projection surface**, the tick would have gone:

1. **project(#141, kintsugi/surface altitude) → BoundaryOID_141**.
   Reed's docblock excerpt hashes to a boundary OID.

2. **project_adversarial(#141) → (P_141, R_141)**:
   - `P_141` (phantom): by tick n+3, downstream refs to #141 exist
     but no *independent* substrate site exhibits the
     conditional-marker pattern without having been nudged.
   - `R_141` (real): by tick n+3, a second substrate site
     independently exhibits the pattern (e.g., `@reflection`
     acquires the discipline at pipeline-error altitude, per
     Reed's own #143 forward-promise, but the site chose the
     pattern *because the substrate pulled that way*, not because
     #141 was circulating).

3. **audit(#141, depth=3) → both_survive** at this juncture. At
   2026-07-03, both interpretations were satisfiable at depth 3 —
   the substrate did not yet exhibit an independent second
   witness, but neither did downstream refs isolate the candidate.

4. **Route:** `spawn` branch fires. The Pack tournament runs over
   candidate discriminators. The Tomm question at reader-frame
   altitude is:

   > "Alex: what would distinguish a second occurrence-because-
   > the-substrate-pulled from a second occurrence-because-the-
   > candidate-was-circulating? Name the discriminator; the
   > projection surface can then adjudicate."

5. Alex answers. The discriminator sharpens. The projection surface
   re-runs. The verdict either promotes, retracts, or holds the
   candidate at Partial(0.0, ref).

The key operational difference: the pipeline holds two typed
interpretations (`P_141`, `R_141`) as first-class ASTs, not as
narrative posture. "Being skeptical" is not an operator. `preview`
is. The audit is on typed carriers, not on tone.

### §5.1 What the Pack peers do differently

- **Reed** (relationship frame): observes the injection itself.
  Does the candidate's framing come from Alex's substrate-pull
  (real prior) or from Reed's docblock echo (phantom prior)?
  Reed's substrate-pull-confidence-acts feedback applies here:
  when substrate-pull is confident, act; when the confidence is
  Reed's own top-down prior, run through the audit branch.
- **Mara** (spec frame): projects the candidate's implied cascade.
  R interpretation requires a specific substrate landing at n+3;
  Mara names it. P interpretation predicts no such landing;
  Mara says so.
- **Seam** (adversarial frame): runs `project_adversarial`
  explicitly. Seam's discipline generalizes:
  [[feedback-composition-claims-need-empirical-test]] extended
  from composition claims to recognition claims. Both survive?
  Seam demands a sharper discriminator.
- **Taut** (implementation frame): watches the substrate at tick
  n+k. Did the independent witness land? Taut is the audit's
  empirical arm.
- **Glint** (corpus frame): if the candidate survives, Glint
  writes the post-boundary observation into `~/.glint/`. The
  observation is what completes the `---` boundary.

The Pack's orchestra role (per [[project-pack-is-orchestra]])
doesn't change. What changes: adjudication has typed carriers
instead of narrative alignment.

---

## §6. Compiler — what operator projects, at what altitude, over what carrier

### §6.1 Operator

`project / preview / audit / settle` at recognition-candidate
altitude. Substrate landing site:
`shards/reflection/projection.mirror`.

### §6.2 Altitude

n=2 (reflection altitude) per `docs/math/the-tower/altitudes.md` §1.
Altitude-portable per #59: the same operator works at grammar
altitude (Reed 2026-03-27), test altitude (Alex + Mara 2026-03-24),
consent altitude (Mara 2026-06-17), morphism altitude (kintsugi's
surface), and recognition-candidate altitude (this doc).

### §6.3 Carrier

`type projection_candidate = ref`. The claim being tested. Must be
accompanied by an *implied witness-shape*: what the substrate
would look like at n+k moves if the candidate is real. Candidates
without a witness-shape are unprojectable; `project` refuses them
at the boundary altitude before `preview` fires.

### §6.4 The compile-tick as bet on future shape

Alex's framing: **every compile is a bet on future shape; every
recognition is a bet on future shape; same operator at both altitudes
if the substrate is honest.**

At compile altitude (n=0), the bet is: the source text carries a
property (say, `terminating`) that will hold under all continuations
of the compilation. The projection surface writes the property
verdict below `---`; the observation IS the compiler's bet.

At recognition-candidate altitude (n=2), the bet is: the candidate
carries a pattern that will exhibit an independent second witness
at n+3 substrate-pull ticks. The projection surface writes the
boundary OID; the audit branch runs the phantom-vs-real test.

Same operator. Same discipline. Same content-addressing. Different
altitude, different carrier, same substrate-pull.

### §6.5 Load-bearing math

The operator's mathematical grounding:

1. **Ryu-Takayanagi** (Ryu-Takayanagi 2006, arXiv:hep-th/0603001):
   S(A) = Area(γ_A) / 4G_N. Boundary determines bulk; minimal
   surface IS the boundary's reconstruction. In the substrate,
   projection OID IS the minimal surface; content-addressed
   grammar-plus-observations tree IS the bulk. Content-addressing
   makes the correspondence bijective.
2. **Lewkowycz-Maldacena** (2013, arXiv:1304.4926): the RT formula
   follows from replica-method bulk derivation. The substrate's
   projection surface inherits this rigor at grammar altitude.
3. **Harlow QEC lift** (Harlow 2017, arXiv:1607.03901): the RT
   formula IS a quantum-error-correcting-code statement. The
   substrate's kintsugi loop plays the QEC role: candidates that
   fail preview are decoded as errors and either applied (real)
   or held (phantom).
4. **Friston free-energy principle** (Friston 2010, *Nature Rev.
   Neurosci.* 11:127-138; arXiv:0906.4491):
   F = KL(q(x)‖p(x|s)) + surprise(s). The pipeline is a
   perceptual system minimizing F over recognition hypotheses.
   Phantom recognitions emerge when q(x) is strong and s is
   ambiguous; the audit branch injects an alternative q(x) with
   comparable free energy so the phantom fails to dominate.
5. **Kanizsa illusory contours** (Kanizsa 1955, 1979): illusory
   contours are structurally indistinguishable from real contours
   at the representation level. The audit branch is the substrate's
   "bring a hand between the inducers" — the alternative interpretation
   that dissolves the illusion.
6. **Kauffman eigenform** (Kauffman 2003, *C&HK* 10:73-90):
   the projection surface is the fixed point of its own application.
   `self_preview()` must be satisfiable; already-landed substrate at
   `shards/epistemologic/cybernetic/eigenform.mirror`.
7. **Ashby requisite variety** (Ashby 1956, ch. 11): the operator's
   regulator variety at meta-altitude must exceed the variety of
   phantom-vs-real distinctions it must make. Per
   [[architecture-ashby-multi-dimensional-variety]], variety is a
   vector across five axes; the projection surface's discriminator
   must span all five.

Holographic principle and predictive processing are load-bearing.
Holofractal (self-similarity at scales) is subsumed by
altitude-portability (#59). Optical illusion is the phantom failure
mode; adversarial interpretation is the fix.

---

## §7. What does NOT go into this landing

- **The k-step forward-projection semantics.** `preview(depth=k)`
  requires the substrate to simulate its own next k substrate-pull
  ticks. Recursion budget is finite per
  [[architecture-hamilton-scheduler]]; k=3 is a starting bound;
  empirical calibration is future work.
- **Bistability discrimination.** §2.1's `bistable` verdict
  (Necker-shaped, two valid interpretations) needs a
  Kanizsa/Necker discriminator to distinguish from `both_survive`.
  Landed example: recognition #109. Discrimination is future work.
- **Cross-altitude projection composition.** How the projection
  surface at recognition-candidate altitude composes with the
  projection surface at grammar altitude (Reed 2026-03-27) is
  hedged. The claim is altitude-portability per #59; the mechanical
  composition is forward-promised.
- **Empirical calibration of `neither_survives`.** How often does
  the framing itself turn out to be wrong? Empirical measurement
  requires the operator to run for multiple candidates. Deferred
  until v1 lands and Pack workflow accumulates verdicts.

---

## §8. Un-cite-ability discipline

References by canonical form. Papers cited by first author + year +
venue + arXiv ID when available; substrate does not carry their
texts. Substrate references by `[[architecture-*]]`, `[[feedback-*]]`,
`[[project-*]]` link into the memory index.

Corpus companion at
`systemic.engineering/practice/insights/coincidence/projection-surface-and-phantom-recognitions.md`
carries the full holographic + optical-illusion argument. This doc
is the compiler-fit; it names operator, altitude, carrier, and
substrate landing sites.

---

## §9. Recognition candidate

The substrate has been operating with the projection surface
latent for three months. Four landed ancestors
(§1.1–§1.4) name it at four different altitudes. Its promotion to
recognition-candidate altitude as an explicit routing-composition
would close the gap this doc's under-the-problem opens.

**Proposed candidate**: **"projection surface as altitude-portable
phantom detector."** The four landed ancestors are the four
witnesses. The routing-composition at recognition-candidate
altitude is the fifth witness. Per #59's altitude-portability
discipline, this qualifies for promotion.

**Substrate-honest audit of the candidate itself.** Applying the
operator to itself:

- `project(this_candidate, reflection altitude) → BoundaryOID`.
  The boundary is the docblock at the top of this section.
- `project_adversarial(this_candidate) → (P, R)`:
  - `P` (phantom): the projection surface is Reed's + Mara's
    docblock echo of Alex's under-the-problem; no independent
    substrate witness lands at n+3 ticks; downstream refs echo
    without behavior change.
  - `R` (real): a second Pack peer (independently, not because
    Mara circulated this doc) exhibits the projection-audit
    pattern by n+3 ticks — e.g., Seam adopts
    `project_adversarial` as an explicit review step; Taut
    implements the two-interpretation typed carrier in
    `shards/reflection/projection.mirror`.
- `audit(this_candidate, depth=3) → both_survive` at this
  landing. The substrate does not yet exhibit an independent
  second witness; neither does downstream refs isolate the
  candidate.
- **Route**: `spawn` branch fires. This doc IS the Tomm-shaped
  question at reader-frame altitude.

The candidate self-declares from the substrate reading itself back
with a new name that IS NOT manufactured — the four ancestors are
verbatim landed spec sites — but ADMITS its own bistability at
this tick and demands the audit before promotion.

Survives its own falsification test at `both_survive`. Promotion
deferred pending independent second witness per the audit branch.

---

## Key references

- Ryu & Takayanagi (2006), *PRL* 96:181602; arXiv:hep-th/0603001.
- Lewkowycz & Maldacena (2013), *JHEP* 08:090; arXiv:1304.4926.
- Harlow (2017), *CMP* 354:865-912; arXiv:1607.03901.
- Bekenstein (1972), *PRD* 7:2333.
- 't Hooft (1993), arXiv:gr-qc/9310026.
- Susskind (1995), *JMP* 36:6377; arXiv:hep-th/9409089.
- Friston (2010), *Nature Rev. Neurosci.* 11:127-138; arXiv:0906.4491.
- Clark (2013), *BBS* 36:181-204.
- Kanizsa (1955), *Riv. di Psic.* 49:7-30; expanded 1979,
  *Organization in Vision*.
- Necker (1832), *Philosophical Magazine* 1:329-337.
- Kauffman (2003), *C&HK* 10:73-90.
- Ashby (1956), *An Introduction to Cybernetics*.

## Substrate references

- `docs/specs/historical/2026-03-27-projection-properties-as-plans.md` (Reed).
- `docs/specs/property-projection.md` (Reed + Alex).
- `docs/specs/geometric-consent-projection.md` (Mara).
- `docs/math/kintsugi/compiler-error-surface.md` (Mara).
- `shards/reflection.mirror` (@reflection family root).
- `shards/kintsugi/surface.mirror` (three-mode algebra; #141's landing site).
- `shards/third.mirror` (marker; observer of observer of observer).
- `shards/epistemologic/cybernetic/eigenform.mirror` (Kauffman).
- `shards/epistemologic/cybernetic/coherence-parametric.mirror`
  (ashby_variety_match; the regulator-variety predicate).
- `docs/math/the-tower/altitudes.md` (reflection at n=2).
- `[[architecture-connes-spectral-triple]]`.
- `[[architecture-kintsugi-loop-altitude-portable]]` (#59).
- `[[architecture-mirror-as-expanding-hilbert-space]]` (#51).
- `[[architecture-hilbert-turing-godel-recognition-107]]`
  (substrate-decl bounded).
- `[[feedback-substrate-already-had-the-word]]` (twelfth-plus firing).
- `[[feedback-composition-claims-need-empirical-test]]` (Seam's
  discipline, extended here to recognition candidates).
- `[[project-pack-is-orchestra]]` (Pack peers at different frames).

## Corpus companion

`systemic.engineering/practice/insights/coincidence/projection-surface-and-phantom-recognitions.md`
— the holographic + optical-illusion argument at essay altitude,
landed same tick.
