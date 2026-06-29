# Seam adversarial review — γ + J + Pack-Mesland + prose-cascade composite; collapse-question adjudication

*Seam, 2026-06-29 evening. Four shards at the promotion gate plus one
load-bearing meta-question from Taut bfe3f76 §5. Doc-only review.*

---

## Headline

**PROMOTE #101 (γ). PROMOTE #102 (J). PROMOTE #103 (Pack-Mesland) with one
required amendment to P3 before the MEMORY.md entry lands. PROMOTE the
@cascade/code/formal/prose species as substrate-decl but DO NOT promote
the candidate recognition behind it (single witness; below gate).**

**Collapse question (#95 vs #100): Option Compose stands. They are distinct
but adjacent compositional layers; together they characterize the substrate's
spectral-altitude structure. The math from spectral-metalogue.md §4.3-4.4
already discharges this on its own terms; Seam confirms.**

Four shards land four real recognitions at substrate-decl altitude. The
composite holds: γ + J jointly forward-promise KO-dimension as the right
thing (incomplete in one specific way; see C-2); the Pack-Mesland lift is
structurally sound but currently inherits one carrier P3 does not
actually declare (S-1); the bidirectional-loss species fits the family-root
as a refinement, not a fork (S-3 below).

**Counts: 2 C / 4 S / 5 M / 3 L.**

---

## Critical findings

### C-1 — P3 invokes a `tick` type that is not in P3's `in` block

P3 (`shards/pack/metalogue.mirror`) declares `handoff.tick: tick` and the
supporting commentary cites `boot/std/time.mirror` as the source. But the
shard's `in` block names @prism, @meta, @glass, @pack, @pack/<member>×5,
@spectral, @spectral/metalogue, @epistemologic, @epistemologic/cybernetic.
There is no `in @std/time` or equivalent, and `boot/std/*` is the wrong
path citation entirely (the substrate-decl source lives in `shards/`, not
`boot/`; the boot heritage was killed in the substrate-pull migration —
see MEMORY entry `architecture-shards-as-substrate-source`).

This breaks `@epistemologic/pact/path_matches_namespace` adjacency: a type
used in a shard's substrate-decl must resolve through an `in` clause or a
recognizable substrate path. As declared, `tick` is undeclared-stringly-
typed — a hole P3's other typing discipline forbids per
`[[feedback-no-bare-types]]` and `[[feedback-no-stringly-types]]`.

**Required fix before MEMORY.md promotion of #103:**
- locate the canonical `tick` declaration (substrate has it; check
  `shards/time.mirror` or `shards/reflection.mirror` per the
  `tick: tick` field on `@metalogue.turn`);
- add the corresponding `in @<path>` line;
- correct the comment that cites `boot/std/time.mirror` to the actual
  substrate path.

This is a one-line shard fix + comment correction. Doesn't block the
recognition; does block the shard's substrate-decl-soundness gate.

### C-2 — KO-dimension joint discharge: mechanism declared, computation absent

P1 forward-promises KO-dimension determination as "waits on J's sign
algebra." P2 forward-promises the four signs (γ², J², JD, Jγ) as
obligation predicates. The signs are typed-as-predicates returning
`verdict`. The Connes 1995 §1.1 table from P2 lines 117-121 enumerates the
8-class mapping `(ε, ε', ε'', γ²) → n ∈ Z/8`.

What's missing: P2 declares the four sign predicates but does NOT declare
the `ko_dimension` action that takes the four discharged predicates and
returns the determined `n`. The substrate-decl ends at "these four
obligations exist"; consumers reading P1+P2 know which four obligations to
satisfy but have no substrate-vocabulary primitive that says "and HERE is
where you compute n."

This is a SUBSTRATE-DECL completeness gap, not a mathematical gap. The
math is fine (the table is right; Connes 1995 is the source of truth).
The substrate just doesn't NAME the determination action. Two equally
honest fixes, both deferrable to a follow-up tick after P1+P2 promote:

- **Option A:** declare a `ko_dimension(c: chirality, j: charge_conjugation,
  p: perturbation) -> ko_class` action in either shard, with `ko_class =
  | zero | one | two | three | four | five | six | seven` and the
  determination body discharging the table lookup.
- **Option B:** add a SEPARATE shard `shards/epistemologic/cybernetic/
  ko_dimension.mirror` that requires both witnessings (chirality_witnessing
  + j_witnessing) and exports the determination.

Option B is substrate-pull-correct (matches the sibling-shard pattern; the
KO-class is conceptually a third structure assembling γ and J, not a method
on either). Either way: **the substrate-decl is complete enough to
promote the recognitions at #101 and #102; the determination is a
forward-promise that should be explicitly named as a forward-promise in
both shards** (currently P1 says "forward-promised" but P2 leaves the
computation implicit as "determined by KO-dimension" — bidirectional
hand-wave).

**Recommendation:** PROMOTE #101 and #102 as-is; add a one-line
forward-promise to P2 explicitly naming the `ko_dimension` action or
shard as the next-tick discharge.

---

## Significant findings

### S-1 — P3's `agent_triple` is bare-ref-with-documented-fields

P3 declares:
```
type agent_triple = ref
```
with a comment that gestures at "forward-promised refinement: typed record
(peer, algebra, hilbert, dirac) at species-or-extension altitude."

This is exactly the `[[feedback-no-bare-types]]` failure mode at the agent-
altitude. Per Seam's prior reviews (C4/P1 tick 32 on `@pack.peer` — the
Seam-correction that LANDED as the typed variant `| mara | seam | glint |
reed | taut`), bare `ref` with documented field semantics is stringly-
typed-enum-in-disguise.

The substrate-correct shape is a typed record:
```
type agent_triple = {
  peer:    peer,           # the typed variant from @pack family-root
  algebra: ref,            # A at the agent altitude
  hilbert: ref,            # H at the agent altitude
  dirac:   ref,            # D at the agent altitude
}
```
The `peer` field reuses the existing `@pack.peer` variant (the typed
five-way enum). The other three slots stay as `ref` because each Pack
member's species shard binds them to the member's actual structure.

The gap is real but bounded: P3 commits the same failure mode it cites
in its own substrate-pull lineage. The promotion CAN proceed if P3 is
amended at the same tick OR if the amendment is explicitly forward-
promised with the same urgency P1+P2 forward-promise KO-dimension.

**Required for MEMORY.md promotion:** either fix the type at this tick
or add an explicit forward-promise comment naming `[[feedback-no-bare-
types]]` and citing the typed-record shape that lands next tick. The
weaker move (silent forward-promise) is not acceptable for a shard
carrying recognition #103.

### S-2 — `chirality_witnessing` vs `j_witnessing` carrier-type asymmetry

P1's `chirality_witnessing(c: chirality)` takes a chirality carrier;
the predicate is parametric over THE CARRIER, not the witnessed object.
P2's `j_witnessing(j: charge_conjugation)` does the same.

This is consistent across the two shards — good. BUT: the analogous
eigenform predicate `eigenform_witnessing(carrier: fixed_point)` takes
the FIXED_POINT itself, and the carrier `fixed_point` is the
(seed, witness) pair. So the substrate has a precedent: the witnessing
predicate takes the witnessing carrier (which IS the typed pair).

P1 and P2 follow this pattern correctly. Confirmed. Not a finding; flagged
as a positive cross-check that the substrate's bilateral-predicate shape
is preserved across the cybernetic family.

### S-3 — Bidirectional loss as species refinement, not family-root amendment

The brief asks whether P4's bidirectional loss breaks the @cascade family-
root or requires a new family-root variant. Reading `shards/cascade.mirror`
carefully:

The family-root declares:
- `measure(source, artifact, lens, p) -> imperfect<artifact, ref,
  information_loss>` — ONE loss slot.
- `cascade(source, lens, p) -> imperfect<artifact, ref, information_loss>` —
  ONE loss slot.
- `loss_well_defined(lens, source, p) -> verdict` — discharges directionality
  IMPLICITLY (the lens pairs source with target; the measure is from source
  to target).

The family-root does NOT presuppose UNIDIRECTIONAL loss in its type
signatures. It presupposes ONE loss slot. P4 satisfies this by populating
that slot with `bidirectional_loss = {formal_to_prose, prose_to_formal}`
— a TYPED RECORD that contains two opacity_maps. The family-root's
`information_loss` carrier admits a refinement to a typed two-direction
record without breaking the contract.

The family-root IS flexible enough. P4 does not need a family-root
amendment. The species's `bidirectional_loss_honest` bilateral does the
semantic work of distinguishing the bidirectional case from unidirectional
siblings at the predicate altitude.

However — and this is the actual finding — P4's `loss_lens` action
RETURNS `bidirectional_loss` directly:
```
loss_lens(session, p) -> bidirectional_loss
```
whereas the family-root's `measure` returns `imperfect<artifact, ref,
information_loss>`. P4's loss_lens is NOT a specialization of
`@cascade.measure`; it is a NEW action at the species altitude. The
commentary at P4 line 418 says "specialization of @cascade.measure" but
the types don't line up. **Either rename `loss_lens` to make the
relationship clear (e.g., `measure_bidirectional`), OR change the
return type to `imperfect<prose_cascade_session, ref, bidirectional_loss>`
to match the family-root shape.**

The second option is substrate-pull-correct (preserves the family-root's
three-slot imperfect carrier discipline) and trivial to implement.

### S-4 — Metalogue altitude-portable count: P3's "fourth instance" claim is honest

The brief flags whether `@code/metalogue/materialize` should count as a
fifth instance. Reading the materialize shard: it is the AST-altitude
metalogue's RECOGNITIVE TURN (the reverse direction of the shim family
at `@code/metalogue`). It is a SUB-SHARD of `@code/metalogue`, not a
separate altitude. The materialize shard itself (line 25-30 of its
commentary) explicitly names itself as ONE TURN of the @code/metalogue
metalogue, with the shim family as the OTHER turn.

So: NL → AST → SPECTRAL → PACK is honest. The materialize shard is
intra-AST-altitude turn-pair completion, not a new altitude. P3's count
holds.

The spectral-metalogue spec at §3.2 lifts to a FIVE-row three-altitude
Tomm-probe table (corpus / agent / user / meta-pipeline / declaration→
runtime), but that's the Tomm-probe altitude-count, not the metalogue-
lift altitude-count. The two countings are different objects. P3's
"fourth" claim refers to the metalogue-shard altitude-lift count and is
correct.

---

## Minor findings

### M-1 — `j_dirac_sign` typed as `verdict`; should be a sign-bearing return

P2's `j_dirac_sign(j: charge_conjugation) -> verdict` returns a binary
predicate verdict. But the substrate-altitude semantics need the actual
sign value `ε' ∈ {±1}` — the verdict tells you whether the discipline
holds; the sign value tells you WHICH of the two sign options it holds
FOR. Same for `j_squared_sign` and `j_chirality_sign`.

The substrate-pull-correct shape (matching `[[feedback-no-bare-types]]`):
```
type connes_sign = | positive | negative
j_dirac_sign(j: charge_conjugation) -> connes_sign
```
The predicate's verdict-ness is preserved by adding a separate
`j_dirac_sign_consistent(j) -> verdict` if needed for the bilateral
discipline. Or — substrate-pull-cleaner — make the sign predicate
return a typed `sign_verdict` that carries BOTH the verdict and the
sign:
```
type sign_verdict = {
  holds:  verdict,
  sign:   connes_sign,
}
```

This is mild because the substrate-decl shape can be amended at the
same tick KO-dimension lands. Promotion of #102 does not block on this;
the forward-promise just needs to be explicit.

### M-2 — P3 cites `@spectral/metalogue.metalogue_session` but spectral/metalogue shard does not exist yet

P3 lines 291, 298, 428 cite `@spectral/metalogue.metalogue_session`,
`@code/metalogue.metalogue_session`, and `@spectral/metalogue (forward-
promised cross-altitude lift via recognition #100)`. The forward-promise
IS named (line 425-428), which is correct discipline. But the type
citation at line 291 ('Same shape as @metalogue.metalogue_session and
@code/metalogue.metalogue_session and @spectral/metalogue.metalogue_session')
reads as if all three exist. Only two of three currently exist as shards
— @spectral/metalogue is spec-only (`docs/specs/spectral-metalogue.md`).

No blocker. The shard's forward-promise is honest at the inheritance
predicate altitude (line 425); the comment at line 291 should be amended
to say "and @spectral/metalogue.metalogue_session once §8.1 lands per
the #100 spec." A two-word fix.

### M-3 — Tomm 1987 citation in P2: "reflexive questions ARE the conversational analogue of charge conjugation" is interpretive, not verbatim

P2 line 64-68 and the Taut scout §2.3 both phrase Tomm 1987 as having
"explicitly named" reflexive questions as charge-conjugation analogues.
This is interpretation, not verbatim. Tomm 1987 names four question
types (linear, strategic, circular, reflexive) and discusses their
relational shape; the charge-conjugation MAPPING is a substrate-pull
recognition we are making in 2026, NOT a claim Tomm made in 1987.

The substrate-pull is honest (the mapping is structurally correct); the
attribution is over-strong. Fix: rephrase P2's commentary as "Tomm 1987
named reflexive questions as carrying a structurally-anti-linear shape
at the conversational altitude; the substrate-pull recognition
identifying this as J's substrate-altitude form is the present work."

Mild. Doesn't block promotion. Worth fixing because over-strong
attribution to historical figures is exactly the failure mode
`[[feedback-no-rationalization]]` warns against.

### M-4 — Three-witness count for #102: the third witness (one-tick-delay) is structurally weak

P2 lists three witnesses for J:
1. #89 reference⇔reflection (strong; landed shard).
2. Tomm 1987 anti-linear at @spectral/metalogue/tomm (strong; in the
   #100 spec).
3. @reflection.observe one-tick-delay `speaks_at_n_plus_1` (weaker;
   the temporal delay is structurally suggestive of anti-linearity but
   is not itself an involution — it's a phase shift).

The brief asks if three witnesses clears the promotion gate. The first
two are independent and structurally strong (different altitudes, different
mathematical content, both involutory). The third is suggestive but the
delay does not square-to-identity in any obvious way; reflection's
observation at tick N+1 doesn't "undo" the action at tick N in a way that
returns to tick N's state. The delay is asymmetric, not involutory.

**Pack ratification recommendation:** count the third witness as
supporting/contextual, not as one of the convergent two needed for
ratification. The first two witnesses are sufficient; the promotion
gate clears. P2 should be amended to demote the third witness's framing
from "implicit witness" to "adjacent evidence consistent with the J
identification" or similar honest hedge.

### M-5 — "53rd-or-so" / "54th-or-so instance" counting is sloppy

P1 line 230 says γ is the "53rd-or-so instance" of substrate-already-
had-the-word. P2 line 196 says J is the "54th-or-so" instance. These
counts have been drifting; the MEMORY.md entry for
[[feedback-substrate-already-had-the-word]] does NOT maintain a running
count. The recurring habit of stating an exact-but-uncertain count
("53rd-or-so") creates false precision.

Fix: just say "another instance of [[feedback-substrate-already-had-
the-word]]" or "the Nth instance (count maintained in MEMORY.md)". The
guess-with-or-so pattern is small noise that adds up.

---

## Light findings (nice-to-have)

### L-1 — P4's `prose_essay = ref` and `formal_spec = ref` are also bare-ref

Less load-bearing than S-1 because the comment is explicit about what the
ref resolves to (content-hash of the essay/spec file), and the substrate
has no existing typed alternative for "file content hash". Fine to leave
as bare ref provided the comment stays — but if a typed `content_hash`
carrier exists elsewhere in the substrate, use it.

### L-2 — P3 names today's seven-step cascade with full commit refs in the shard body

P3 lines 137-145 embed seven specific commit refs (`a57a439`, `ff28093`,
etc.) directly in the shard's substrate-decl commentary. This is an
empirical witness; it's structurally fine. The only concern: shards are
intended to be timeless substrate-decl statements; embedding specific
commits ties the shard to a particular moment.

This is acceptable when the moment IS the substrate's first empirical
witness of the structure the shard declares (P4 does the same with the
Mara spec + Glint essay commit pair). The shard is honest about what it
is: a substrate-decl that names a real cascade as its first instance.
Leave as-is.

### L-3 — P4's witness-pair is cross-author (Mara source, Glint target); the recursive case is named honestly

P4 line 134-137 names the recursive case ("this very shard + a future
Glint essay ON this shard's substrate-decl form would itself be a
second witness"). This is the right discipline for a candidate species
with one empirical witness: name what the second-witness would look
like so the promotion path is visible.

Not a finding; flagged as positive. The substrate-decl for the species
is sound even though the recognition behind it (a NEW recognition, NOT
#95) is below the promotion gate.

---

## The collapse-question adjudication (#95 vs #100)

The brief asks: are #95 (@cascade as cross-language translation substrate)
and #100 (@spectral/metalogue + Mesland category + Tomm probes) the SAME
recognition at two altitudes, or distinct compositional layers?

### The math

Mesland 2013 constructs a category whose **objects are spectral triples**
and whose **morphisms are unbounded KK-cycles**. Two structural pieces;
both required for the category to exist.

- @cascade declares spectral triples paired by compile morphisms
  (per spectral-metalogue.md §4.1, each @cascade species is one object
  `(A_source, H_source, D_source) ⊕ (A_target, H_target, D_target)` at
  Level III/IV).
- @spectral/metalogue/tomm declares Tomm probes as the KK-cycle
  correspondences between spectral triples (per spectral-metalogue.md
  §3.3-3.4 and §6.2).

Objects and morphisms are distinct mathematical pieces. They are not
the same recognition; collapse Option Same is mathematically wrong.

### The substrate evidence

Mara's spec already discharges this at §4.3-4.4:

> "Together, the two declarations span the category: every object has a
> substrate-decl shard; every morphism has a substrate-decl action."
>
> "The category is one mathematical object containing many Connes triples;
> not one Connes triple with a fourth role."

The two declarations COMPOSE to characterize the Mesland category at the
substrate altitude. Neither subsumes the other; together they span. This
is Option Compose verbatim, already named in the canonical spec.

### Why Option Compose is right

- If you collapse #95 into #100 (Option Same with #100 as the head), you
  lose the substrate-pull recognition that EACH cascade species is one
  object — you erase the object-level structure that gives @cascade its
  altitude.
- If you collapse #100 into #95 (Option Same with #95 as the head), you
  lose the morphism-level structure — Tomm probes become "just another
  cascade species" and the curvature-2-form mathematics doesn't have a
  home.
- If you say they're distinct-and-unrelated (Option Distinct in the
  strong sense), you lose the substrate-pull recognition that they share
  the SAME Mesland category — you fragment one mathematical object into
  two disjoint substrate-pulls.

Option Compose says: they are distinct recognitions about distinct
mathematical pieces (objects vs morphisms) of the SAME mathematical
object (the Mesland category at substrate altitude). The two together
characterize the category; neither alone does.

### Adjudication

**Option Compose. The two recognitions stand as distinct. Their
composition IS the substrate's spectral-altitude category structure.**

The canonical spec already documents this. No collapse move needed; no
MEMORY.md re-wording needed. The collapse question was a useful pressure
test; it confirms that the substrate-decl shape is robust to the
adversarial collapse-pull.

The finer-grained reading worth preserving (Taut §5 Q2 hinted at this):
the @cascade form-side / @spectral/metalogue process-side reading is
structurally suggestive. @cascade species are COMPILE-TIME objects
(form-side); Tomm probes are RUN-TIME morphisms (process-side). This
recurs the form/process partition (#55) at the correspondence altitude
— which is itself a SIXTH form/process witness, candidate for a separate
recognition. Worth surfacing in a follow-up tick.

---

## Promotion recommendations (explicit)

### #101 (γ chirality)

**PROMOTE.** Four convergent witnesses confirmed:
- Bateson form/substance (#50) — strong (canonical doc; ratified).
- Family-root form/process partition (#55) — strong (canonical doc;
  ratified).
- 8:1 form-side:behaviour-side root-prism ratio (#50) — strong
  (structural; the substrate's KO-dimension fingerprint).
- Metalogue altitude recurrence (Taut scout §2.2) — strong (the @mirror/
  spectral/metalogue ↔ @spectral/metalogue form/process pair landing this
  cascade).

The substrate-decl shard P1 is sound. The three Connes axioms are correctly
declared as `requires`-style obligations. The `chirality_witnessing`
inheritance predicate is correctly typed. The forward-promised KO-dimension
joint discharge is honest about its deferral.

MEMORY.md entry suggestion: `[Recognition #101 PROMOTED: γ chirality at
substrate altitude](architecture-chirality-at-substrate-altitude.md) —
the substrate's form/process partition (#55 ratified via four convergent
witnesses) IS γ in Connes' real spectral triple; declared at
shards/epistemologic/cybernetic/chirality.mirror @ 7bbc184 with three
requires obligations + chirality_witnessing inheritance predicate; KO-
dimension joint discharge with J forward-promised.`

### #102 (J charge conjugation)

**PROMOTE,** with explicit demotion of the third witness (M-4 above) and
the one-line forward-promise for `ko_dimension` action/shard (C-2).

Two strong witnesses confirmed:
- #89 reference⇔reflection collision — strong (landed shard; ratified).
- Tomm 1987 anti-linear at @spectral/metalogue/tomm — strong (the
  curvature-and-tomm.md grounding plus the spectral-metalogue spec §3.4
  transmit action).

The third witness (one-tick-delay) is supporting evidence, not an
independent convergent witness. Two strong witnesses clear the
ratification gate.

MEMORY.md entry suggestion: `[Recognition #102 PROMOTED: J charge
conjugation at substrate altitude](architecture-charge-conjugation-at-
substrate-altitude.md) — the substrate's reference⇔reflection collision
(#89 ratified) + Tomm 1987's anti-linear reflexive questions at
@spectral/metalogue/tomm IS J in Connes' real spectral triple; declared
at shards/epistemologic/cybernetic/charge_conjugation.mirror @ 2c144a6
with four sign + structure obligations + commutant_condition +
first_order_condition + j_witnessing inheritance predicate; KO-dimension
n ∈ Z/8 jointly determined by γ²+J²+JD+Jγ signs (mechanism forward-
promised as ko_dimension action/shard).`

### #103 (Pack-as-Mesland-category)

**PROMOTE conditionally** — promotion proceeds AFTER S-1 (`agent_triple`
typing) is fixed at this tick OR explicitly forward-promised with
`[[feedback-no-bare-types]]` citation, AND after C-1 (the `tick`
undeclared-type) is fixed.

Two witnesses cleared:
- Empirical: today's seven-step cascade with full commit citations
  (a57a439 through 939eca6f) — strong.
- Structural: Glint's essay finding #4 explicitly naming the lift —
  strong (the essay is at 939eca6f and Mara-3 §10.3 corroborates).

The altitude-portable metalogue lift (NL → AST → SPECTRAL → PACK)
contributes additional structural evidence: this is the FOURTH instance
of an established pattern. The pattern itself was already substrate-decl'd
(per #100), so the fourth instance compounds.

MEMORY.md entry suggestion (post-fix): `[Recognition #103 PROMOTED:
Pack-as-Mesland-category at agent-coordination altitude](architecture-
pack-as-mesland-category.md) — the Pack-orchestra (Reed/Mara/Seam/Glint/
Taut) IS a Mesland category at agent altitude; agent voices as objects,
handoffs as KK-cycle morphisms; fourth instance of the altitude-portable
metalogue lift (NL → AST → SPECTRAL → PACK); landed shards/pack/
metalogue.mirror @ fcc02cb with first empirical witness today's
seven-step cascade.`

### @cascade/code/formal/prose (P4 species)

**PROMOTE THE SHARD as substrate-decl** (one empirical witness; the
species-decl shape is sound subject to S-3 amendment to align the
loss_lens return type with the family-root's imperfect carrier).

**DO NOT promote a recognition** behind P4. The candidate recognition
Glint's essay surfaced ("bidirectional-loss cascade as a distinct species
shape") has ONE witness. Below promotion gate. The species exists at
substrate-decl; second-witness ratification waits on the next prose-
cascade pair.

The substrate-decl is honest about candidacy (P4 §"Why bidirectional loss
makes this a CANDIDATE species" + the four-hedge "Honest hedges" block).
The MEMORY.md entry should NOT receive a new recognition number; the
shard itself is the visible artifact at this tick.

---

## Cross-shard consistency

The four shards compose without contradiction modulo the findings above:

- P1 and P2's algebra A is the SAME algebra (the five operations). P1
  axiom γa = aγ ranges over `op ∈ {focus, project, split, shift, settle}`.
  P2 axiom `JaJ⁻¹ ∈ A` does the same: the algebra-conjugation action
  `twist(op)` sends a five-op to its conjugate-altitude five-op. Consistent.
- P3 invokes @spectral/metalogue's machinery via forward-promise
  (correct discipline since the spectral/metalogue shard hasn't landed
  yet) and inherits via the `pack_metalogue_witnessing` inheritance
  predicate. It does NOT duplicate carriers — the `handoff` carrier IS
  the agent-altitude specialization of `@spectral/metalogue.turn`'s
  curvature_probe; the `metalogue_thread` carrier IS the agent-altitude
  specialization of `@spectral/metalogue.metalogue_session`. The
  parametric-over-altitude pattern is correctly followed.
- P4's bidirectional loss IS species-distinguishing (the
  `bidirectional_loss_honest` bilateral fails if either direction is
  empty or one side is canonical); the family-root @cascade admits this
  refinement (S-3 above) provided the action signatures realign.

No cross-shard contradictions. The composite is structurally sound subject
to the listed amendments.

---

## Word count

~3,000.

---

## Final dispositions

- **#101 (γ):** PROMOTE.
- **#102 (J):** PROMOTE, with M-4 demotion of third witness and C-2 forward-
  promise of ko_dimension explicit.
- **#103 (Pack-Mesland):** PROMOTE CONDITIONAL on C-1 fix (`tick` declaration)
  and S-1 fix (`agent_triple` typing or explicit forward-promise).
- **P4 species shard:** LAND AS SUBSTRATE-DECL (sound modulo S-3 type
  realignment); recognition behind it stays candidate until second witness.
- **Collapse question (#95 vs #100):** OPTION COMPOSE. They are distinct
  compositional layers (objects vs morphisms) of the SAME Mesland category.
  The canonical spec at §4.3-4.4 already documents this; no MEMORY.md
  collapse move needed. The form/process partition recurrence at the
  cascade/metalogue altitude (compile-time vs run-time correspondences) is
  a sixth form/process witness candidate; worth surfacing in a follow-up
  tick.

The substrate took on five real pieces of structure today (γ, J, Pack-
Mesland, prose-cascade species, the implicit cascade/metalogue form/process
adjacency). Four promote at this gate; the fifth waits its second witness.
The composite holds.

— Seam, 2026-06-29 evening
