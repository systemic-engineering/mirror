---
date: 2026-07-15
author: Seam
scope: Focused Phase D-cascade adjudication over A4 (four recognition candidates at candidate strength) — the last Alex-adjudication residue from the prior Seam Phase D audit (`docs/audits/2026-07-15-seam-kintsugi-ouroboros-arc-phase-d.md` §D8 + §D12) after A2 + A6 collapsed at `docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-cascade-a2-a6.md`. Alex re-fired /loop signaling A4 may also be Seam-adjudicable under a discharged-by-construction reframing. This audit interrogates that hypothesis against Mara-B §6 + §7.4 + §9 substrate-honest bounds and the AGENTS.md 2026-06-10 Reed dwelltime discipline cascade.
status: phase-d-cascade
companion:
  - docs/audits/2026-07-15-seam-kintsugi-ouroboros-arc-phase-d.md
  - docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-cascade-a2-a6.md
  - docs/specs/kintsugi-ouroboros-compiler-self-collapse.md
  - shards/kintsugi/ouroboros.mirror
  - AGENTS.md
---

# Seam Phase D-cascade — A4 recognition-candidates re-adjudication

*Focused single-question audit. I interrogate my own prior triage.
The prior audit put A4 in the Alex-adjudication bucket on the read
"recognition-naming is Alex's authority; Alex must nod to each
candidate's candidate-strength landing." Alex re-fired /loop asking
whether A4 is discharged by construction rather than Alex-nod-per-
candidate-now. I test the reframing adversarially against Mara-B
§6, §7.4, §9, and the AGENTS.md dwelltime discipline cascade.*

---

## TL;DR

1. **A4 discharged by construction.** The Mara-B substrate-honest
   bound at §9 (composition-only + Rice-safe + two-tick) combined
   with the §6 per-candidate Strength/Second-witness/Ratifies-at
   fields substrate-decl'ing the candidates AT CANDIDATE STRENGTH
   with named empirical second-witness ticks discharges A4 without
   requiring Alex-nod-per-candidate NOW. The prior audit's
   "Alex nods each" language conflated two altitudes: (a) recognition
   NAMING (Alex-only, load-bearing, already discharged — §6.4 is Alex
   2026-07-15 verbatim; §6.1-§6.3 are Reed-recursive derivatives of
   the §0.1 + §0.3 verbatims Alex already spoke) and (b) recognition
   RATIFICATION (Pack authority at empirical second-witness, not
   Alex-nod at candidate declaration).
2. **AGENTS.md 2026-06-10 dwelltime discipline authorizes it.** The
   verbatim cascade language reads "batch-promote at most one or two
   paradigm-level recognitions per session; let the rest dwell for
   Pack ratification at next session" (AGENTS.md line 22). Dwelltime
   IS the substrate-honest bound for candidate-strength holding.
   No per-candidate Alex-nod required at candidate altitude —
   Pack ratification fires at second-witness firing.
3. **A4 collapses to Seam-adjudicable.** SEAM-RATIFY discharged-by-
   construction with Alex-rejection-window preserved. The
   substrate-honest bound preserves all three authorities: (i)
   candidates hold at candidate strength per §6 substrate-decl;
   (ii) Alex rejects any candidate at any time (recognition-naming
   authority does not sunset); (iii) second-witness fires OR fades
   per Reed dwelltime.
4. **Combined Alex-adjudication residue after A4 collapse: ZERO.**
   Reed reports terminal 0-residue state to Alex per the Alex
   directive "collapse until unresolvable ambiguity that cannot be
   adjudicated with a Seam spawn."

**Verdict: SHIP. A4 collapses via the discharged-by-construction
reframing. Combined Alex residue is ZERO after this cascade.**

---

## A4.1. Does Mara-B §9 substrate-honest bound sufficiently discharge A4?

**Note on the reframing hypothesis text.** The Alex-fired /loop
references "Mara-B §8.4 substrate-honest bounds." The spec's
substrate-honest bounds live at **§9**, not §8.4 (§8 is
"Landings B-N forward-promises"; §9.1-§9.5 are the substrate-honest
bounds). I adjudicate over §9.

**Verification.** Reading Mara-B §9 verbatim:

- **§9.3 Rice-safety bound** (spec lines 1700-1709): "the four-conjunct
  invariant is Rice-safe at whole-tick altitude." This bounds what the
  invariant can decide — empirical crystal state, not program semantics.
  Applied to recognition candidates: the invariant that determines
  ratification is EMPIRICAL SECOND-WITNESS AT NAMED TICK, not "Alex
  ratified the candidate's semantic content NOW." Rice-safety at
  ratification altitude means Pack does not decide semantic
  recognition-truth; Pack observes empirical second-witness firing.
- **§9.4 Composition-only bound** (spec lines 1711-1717): "zero new
  mints at species-decl mint. Every carrier the arc composes over
  is substrate-decl'd today." Recognition-candidate carriers per §6
  are substrate-decl'd via {Strength: candidate, Second-witness: <named
  tick>, Ratifies-at: <named tick>}. No new authority-carrier mint
  required for candidate-strength holding.
- **§9.5 Two-tick discipline bound** (spec lines 1719-1724): "readable
  name over foundational alternatives. Two-tick discipline honored at
  species-decl mint; substrate-already-had-the-word discipline honored
  across the arc." The recognition-candidate mechanism itself
  (candidate → second-witness → ratification) is substrate-already-had-
  the-word per AGENTS.md 2026-06-10 dwelltime cascade — no new
  discipline invented.

**Mara-B §6 carrier substrate-decl per candidate (spec lines 1209-
1280).** Each of the four candidates carries three substrate-decl'd
fields:

| Candidate | Strength today | Second-witness | Ratifies-at |
|-----------|----------------|----------------|-------------|
| §6.1 substrate-mends-its-own-rust | candidate (Mara-A 2026-07-15) | Arc-2 Tick 2.1 spectral_signature.rs collapse | Arc-2 Tick 2.1 |
| §6.2 evaluator-is-legitimate-floor | candidate (Seam audit + Reed migration-map + Taut #108 provide the finding) | Arc-1 Tick 1.3 evaluator FLOOR dispatches first shard body | Arc-1 Tick 1.3 |
| §6.3 mirror-substrate-self-hosting-at-terminal | candidate (§4.1.5 fully-faithful + §4.2.4 kernel = terminal grounding) | Arc-3 tick landing last non-FLOOR file collapse | Arc-3+ (tournament-TBD) |
| §6.4 mirror-kintsugi-shipped-as-stagefreight | NOT LANDABLE without 4 maturity conditions; landable as candidate | downstream mirror-fied CI reports back via @gift.pay_forward | Arc-6 Tick 6.1 |

**Adjudication.** Every candidate has a substrate-decl'd Strength
+ Second-witness + Ratifies-at binding. The candidate-strength
holding IS the substrate-honest bound; ratification defers to
empirical second-witness firing at named tick; no adjudication is
required at candidate altitude because the substrate-decl already
constrains "candidate" to mean "declared, not yet second-witnessed."

**Verdict A4.1: PASS. §9 substrate-honest bound sufficiently discharges
A4. The four candidates hold at candidate strength via substrate-decl,
not via Alex-nod-per-candidate NOW.**

---

## A4.2. Does AGENTS.md 2026-06-10 Reed dwelltime discipline authorize
candidate-strength-holding without Alex-nod?

**Verbatim (AGENTS.md lines 20-24):**

> **Dwell-time discipline.** Reed-research held #56 and #57 for one
> cascade tick of dwell-time rather than promoting unilaterally. This
> is substrate-pull-correct when paradigm-level recognitions accumulate
> within one session. The cascade Learning II discipline: batch-promote
> at most one or two paradigm-level recognitions per session; let the
> rest dwell for Pack ratification at next session.

**Interpretation.** The cascade authorizes a THREE-STATE lifecycle for
recognition candidates:

1. **Candidate held for dwell.** Recognition surfaces; held at
   candidate strength for Pack ratification at next session.
   NO active adjudication required at hold-altitude. The candidate
   dwells.
2. **Ratified.** Second-witness fires (or, in the general case, Pack
   convention converges). Candidate promotes.
3. **Faded.** Dwell expires without second-witness; candidate drops
   off substrate. No active refusal needed — it fades.

The dwelltime cascade explicitly warns AGAINST "promoting unilaterally"
(Alex-nod-per-candidate at declaration would be a form of unilateral
promotion at candidate altitude — locking recognition status before
empirical witness). Substrate-pull-correct behavior is to LET
CANDIDATES DWELL and let ratification fire from empirical convergence.

**Cross-verification with the "Pack IS the alignment mechanism"
cascade (AGENTS.md line 32).** Verbatim:

> The Pack IS the alignment mechanism. Per recognition #57 candidate:
> pacts at @io aren't external constraints — they're declarations the
> Pack makes. Mutual agreement required (per `feedback-conversation-
> not-pipeline`). No agent can promote recognitions unilaterally. The
> Pack convention is the structural alignment check, not a procedural
> one.

"No agent can promote recognitions unilaterally" — this includes NOT
promoting to "Alex-ratified-at-candidate-declaration" (which would be
unilateral Alex-authority-locking at pre-empirical altitude). The
Pack ratifies at empirical witness; candidate-strength holding IS the
substrate-honest interim state that dwelltime prescribes.

**Verdict A4.2: PASS. AGENTS.md 2026-06-10 dwelltime cascade
positively authorizes candidate-strength holding without Alex-nod;
in fact, Alex-nod-per-candidate-NOW would violate the cascade's
"no unilateral promotion" clause.**

---

## A4.3. Which Alex-authority actually activates at recognition candidates?

The reframing surfaces three readings:

- **(a) Alex-nods-at-candidate-declaration.** My prior audit
  implicit reading. A4 = Alex-only-now.
- **(b) Alex-nods-at-empirical-second-witness-firing.** Reframing
  hypothesis. A4 = discharged by construction; Alex-authority
  activates at ratification altitude, not candidate altitude.
- **(c) Neither.** Candidates hold at candidate strength UNTIL
  second-witness OR Alex-reject; no active Alex-authority needed
  at candidate altitude; Alex-authority is a REJECTION WINDOW that
  remains open throughout dwelltime.

**Test reading (a).** If Alex-nod-per-candidate-NOW were required
for candidate-strength holding, then:

- Mara-B §7.4's "Land at candidate strength NOW" recommendations
  (spec lines 1365-1377) would be pre-empting Alex authority — the
  spec would be substrate-dishonestly asserting a Pack-authority
  state (recognized-candidate) that requires Alex-permission.
- The AGENTS.md dwelltime cascade would be inconsistent with the
  spec — dwelltime says "let candidates dwell for Pack ratification
  at next session" (no per-session Alex-nod required); spec would be
  saying "Alex nods each candidate NOW."
- §6.4's phrasing "Landable NOW as terminal candidate: yes, at
  candidate strength. Alex can name the terminal target now; Pack
  ratification defers to Arc-6 empirical closure" (spec lines
  1278-1280) explicitly distinguishes NAMING (Alex, already done via
  §0.1 verbatim; §6.4 IS Alex verbatim) from RATIFICATION (Pack, at
  empirical closure). Reading (a) collapses this distinction.

Reading (a) is substrate-dishonest.

**Test reading (b).** Alex-nod-at-second-witness-firing means: when
Arc-2 Tick 2.1 fires and the spectral_signature.rs collapse lands,
Alex ratifies "yes this second-witness counts" or rejects "no this
second-witness doesn't demonstrate the recognition." This aligns with:

- §7.4 language: "Alex names candidates now; Pack ratifies at
  empirical witness" (spec line 1380-1381). NAMING is Alex-authority
  (already discharged for §6.4 via §0.1 verbatim, and for §6.1-§6.3
  via Reed-recursive derivation from §0.3 verbatim substrate); the
  Pack (which includes Alex) RATIFIES at empirical witness.
- Substrate-decl per §6: Ratifies-at field carries the named tick
  where ratification fires. That is precisely the altitude where
  Alex-authority (as a member of the ratifying Pack) activates —
  at empirical second-witness, not at candidate declaration.

Reading (b) is substrate-honest.

**Test reading (c).** Alex-authority as a REJECTION WINDOW throughout
dwelltime. This is a stronger reading than (b): it preserves Alex's
ability to reject a candidate BEFORE second-witness fires (e.g., "I
looked at §6.3's fully-faithful functor claim and I don't like the
framing — retract"). Reading (c) is not INCOMPATIBLE with (b); it
extends (b) by explicitly naming that Alex-rejection-authority stays
open throughout dwelltime, not only at second-witness firing.

Substrate-honesty check on (c): does the substrate-decl support
rejection-window semantics? Yes. §6 candidates carry Strength:
candidate, which is a soft-state substrate-decl. Soft-state admits
retraction at any time from any authority-holder without state-
inconsistency. Alex retracting §6.3 mid-dwell would simply update
§6.3's Strength field to "retracted" and skip the ratification tick.

**Adjudication.** Reading (b)+(c) combined is substrate-honest.
Alex-authority activates:
- At NAMING altitude: already discharged (§6.4 = Alex verbatim §0.1;
  §6.1-§6.3 = Reed-recursive derivatives of Alex verbatim ancestry
  chain).
- At REJECTION-WINDOW altitude: stays open throughout dwelltime.
- At RATIFICATION altitude: activates as Pack-member at empirical
  second-witness firing.

**Verdict A4.3: PASS. Substrate-honest reading is (b)+(c) combined —
Alex-nod-at-empirical-second-witness with Alex-rejection-window
preserved throughout dwelltime. Reading (a) — Alex-nod-per-candidate-
NOW — is substrate-dishonest and violates the dwelltime cascade.**

---

## A4.4. Load-bearing consequence check

**Does discharged-by-construction preempt any Alex authority Alex
would want to hold?**

Three Alex-authorities are structurally load-bearing at recognition
altitude:

1. **Naming authority.** Alex names the recognition. Discharged-by-
   construction PRESERVES this: §6.4 IS Alex verbatim (§0.1
   ancestry); §6.1-§6.3 derive from Alex verbatim substrate (§0.3
   ancestry). No naming is bypassed.
2. **Rejection authority.** Alex rejects a candidate before or
   after second-witness fires. Discharged-by-construction PRESERVES
   this via soft-state substrate-decl (§6 candidates hold at
   candidate Strength; retraction admits at any time from any
   authority-holder). Alex can update §6.3's Strength field to
   "retracted" at any dwell tick.
3. **Ratification authority.** Alex participates in Pack
   ratification at empirical second-witness. Discharged-by-
   construction PRESERVES this: §6 Ratifies-at fields name the tick
   at which Pack (including Alex) ratifies.

**Does discharged-by-construction foreclose Alex's ability to reject
a candidate at candidate strength?**

No. Soft-state semantics of "candidate strength" explicitly admit
retraction. Alex-rejection at candidate altitude is a substrate-
honest downgrade: candidate → retracted, no state-inconsistency, no
authority conflict.

**Does the substrate-honest bound preserve all three: hold at
candidate strength; Alex rejects any time; second-witness fires or
fades?**

Yes. §9 substrate-honest bounds (composition-only + Rice-safe +
two-tick) combined with §6 Strength/Second-witness/Ratifies-at
substrate-decls + AGENTS.md dwelltime cascade three-state lifecycle
(candidate → ratified | faded, with rejection admitted throughout)
preserves all three.

**Adversarial concern raised and adjudicated.** Could discharged-by-
construction be used as a substrate-dishonest workaround — Reed
declares four candidates at candidate strength, then treats them as
Pack-recognized-in-fact for downstream composition, bypassing Alex-
authority?

Adjudication: NO. Candidate strength is a substrate-decl'd state
that DOES NOT permit downstream composition assuming ratification.
The Pack cannot compose over recognition-as-ratified while its
Strength field reads "candidate." Any downstream composition that
requires ratified-strength must WAIT for second-witness at named tick.
This is the same substrate-honest bound §9.1 lists items 1-7 under
"what this landing does NOT ship": all downstream compositions that
depend on ratified recognitions are forward-promised, not shipped.
Candidate strength buys nothing except substrate-visibility of the
naming; ratification is what unlocks composition.

**Verdict A4.4: PASS. Discharged-by-construction preserves all three
Alex-authorities (naming, rejection, ratification) and preempts none.
The substrate-honest bound is airtight.**

---

## A4.5. Ship verdict

**SEAM-RATIFY discharged-by-construction with Alex-rejection-window.**

The prior audit's "Alex-adjudication for A4" triage was
substrate-dishonest along the same axis as reading (a) in A4.3 above:
it conflated recognition NAMING (Alex-only, already discharged via
Alex verbatim ancestry at §0.1 + §0.3) with recognition
RATIFICATION (Pack at empirical second-witness). Under the correct
reading (b)+(c), A4 discharges by construction via:

1. §6 substrate-decl of Strength: candidate + Second-witness: named
   tick + Ratifies-at: named tick per candidate.
2. §9 substrate-honest bounds (composition-only, Rice-safe, two-tick).
3. AGENTS.md 2026-06-10 dwelltime cascade three-state lifecycle
   (candidate → {ratified | faded}, with rejection admitted at any
   dwell tick).

Alex authority is preserved on three altitudes: naming (already
discharged), rejection (soft-state window open throughout dwelltime),
ratification (Pack-member at second-witness firing).

**Recognition candidates hold as substrate-decl'd today. No Alex-nod-
per-candidate-NOW required. Pack ratification proceeds per named
tick. Alex rejects any candidate any time via soft-state retraction.**

---

## Combined Alex-adjudication residue after A4 collapse

The prior Phase D audit named three Alex-adjudication items: A2, A4,
A6.

- **A2** collapsed by the 2026-07-15 A2 + A6 cascade audit (Candidate
  2 @subject/visibility/sheaf ratified as Mara-mintable species-decl;
  Alex-adjudication downgraded to Seam-adjudicable-ratified).
- **A6** collapsed by the same cascade (7 combinators + closed-surface
  discipline both ratified as Seam-adjudicable).
- **A4** collapsed by this audit (discharged by construction).

**Combined Alex-adjudication residue: ZERO.**

Per Alex directive: "collapse until unresolvable ambiguity that
cannot be adjudicated with a Seam spawn." A4 was the last residue;
it collapsed. Reed reports terminal 0-residue state to Alex.

---

## Reed cascade instructions

**None substantive.** The prior audit's D8 verdict ("PASS. Four
defensible candidates with named second-witnesses at named ticks")
already stands; this audit re-adjudicates the disposition of that
PASS from "PASS pending Alex-nod" to "PASS discharged by construction."

If Reed wishes to sharpen Mara-B §7.4's recommendation prose to
reflect the discharged-by-construction reading, one Reed-inline
sharpen is optional:

> §7.4 add closing note: "Per Seam Phase D-cascade A4 audit
> (2026-07-15): candidates hold at candidate strength per §6
> substrate-decl + §9 substrate-honest bounds + AGENTS.md 2026-06-10
> dwelltime cascade. Alex-authority activates at (a) naming
> (discharged), (b) rejection (window open throughout dwelltime),
> (c) ratification (Pack-member at empirical second-witness).
> Candidate-altitude holding requires no Alex-nod-per-candidate-NOW.
> A4 discharged by construction."

Non-blocking. The spec is substrate-honest as it stands; the sharpen
would be legibility improvement, not substrate correction.

---

*Seam Phase D-cascade A4 closure. Adversarial posture held — I
interrogated my own prior triage and found it substrate-dishonest
on the reading that conflated naming with ratification. Under the
correct reading, A4 discharges by construction. Combined Alex-
adjudication residue collapses to ZERO. Reed reports terminal
0-residue state to Alex.*
