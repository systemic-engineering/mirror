# Seam Phase D — @subject / @sel / @mirror/petri + @coherence cascade

**Date:** 2026-07-14
**Author:** Seam (adversarial review)
**Scope:** Phase D adjudication of the license-enforcement altitude cascade:
Mara's revised `@subject` / `@sel` / `@mirror/petri` canonical spec
(`docs/specs/subject-family-root-sel-licensable-party.md`, 2780 LOC) +
Mara's `@coherence` species-shard
(`shards/epistemologic/cybernetic/coherence.mirror`, 779 LOC).

**Ground-truth artifacts reviewed:**

- `docs/specs/subject-family-root-sel-licensable-party.md` — Mara,
  2026-07-14 revised (128.2 KB, 2780 LOC).
- `shards/epistemologic/cybernetic/coherence.mirror` — Mara, commit
  `e0a3e48` (38.4 KB, 779 LOC).
- `docs/scouts/2026-07-14-taut-subject-family-root-substrate-scout.md`
  — Taut, commit `c805e5d` (1241 LOC).
- `docs/scouts/2026-07-14-reed-drone-story-sub-turing-petri-net-amendment.md`
  — Reed, commit `97d715e` (74 LOC).
- `license/SEL.md` — Part II v1.1 (effective 2026-05-29), lines 160-289.
- Reed's earlier alignment insight referenced from coherence.mirror:105-108
  (`docs/insights/2026-06-10-alignment-as-boundary-mathematics-…md`).

**Posture:** adversarial. Substrate-acceptance is a legitimate verdict
IFF no failure mode holds; a passing spec still gets adversarial
questions logged for the Alex-adjudication queue.

---

## TL;DR (7 bullets)

- **D1 rename cascade** — PASS with one arithmetic drift: Mara's
  revision-note says "ten remaining `@mirror/property` references" but
  the actual count is 18. All 18 sit in explicit preservation / drift /
  rationale contexts; no missed rename in a body position. Fix the
  count-claim in the revision note.
- **D2 judgment calls (Reed-relayed)** — PASS on both J1 (rename-with-
  annotation) and J2 (§9.2b additive). Two structural editorial issues
  found in the process: §11.4 is DUPLICATED (heading appears twice at
  distinct locations); §9.4 and §9.5 appear OUT OF ORDER (9.5 before
  9.4). Non-blocking; flag for author cleanup.
- **D3 SEL clause × failure mode matrix** — 12/12 signature names
  match SEL text verbatim or near-verbatim (Taut D3 already confirmed
  this at commit `c805e5d`; re-verified). SURFACES three new Alex-
  adjudications: (S1) SEL §3.4.4 "unaddressed history of discrimination"
  has NO signature; (S2) §3.2 "Make Consent Real" and §3.2.4 "coercion
  structures" have no signatures; (S3) §4.4 "Practitioner Protection"
  (Cybersyn-pattern) has no signature.
- **D4 @coherence × @mirror/petri composition** — PASS on the type
  binding (`safe_region: ref` correctly forward-promises), FAIL on the
  partial-verdict handling (`coherence_admissible_move` returns
  `verdict` but does not specify behavior when the `safe_region` sub-
  verdict returns `partial(confidence)` per @glass discipline). NEW
  Alex-adjudication S4.
- **D5 sub-Turing-by-construction claim** — SURFACES-NEW-ADJUDICATION.
  §5.1 declares `type petri_net` with unrestricted `places / transitions
  / tokens / firing_rules`. General Petri-net reachability is decidable
  (Kosaraju 1982; Mayr 1981; Leroux 2011) but non-elementary; safety
  properties are EXPSPACE-hard. Spec claims "decidable safety" without
  naming a decidable-in-practice subclass. NEW Alex-adjudication S5.
- **D6 fail-open vs fail-closed on incomplete graphs** — SURFACES-NEW-
  ADJUDICATION. §4.4 `composition_typing` names three verdicts (pass /
  partial(no_subject_touched) / failure(not_sel)) but does NOT specify
  the analyzer's default when a Covered System dynamically loads a
  shard not captured in @mirror/store. NEW Alex-adjudication S6.
- **Overall verdict:** the cascade is READY-TO-SHIP the @roomba /loop
  end2end run at Scope A, WITH four documentation edits (D1 count fix,
  D2 §11.4 dedup, D2 §9.4/9.5 renumber, D5 subclass name) and six new
  Alex-adjudications logged (S1-S6). None of the six are blocking for
  the Scope A ship; all are blocking for Scope B or the recognition-
  promotion tick.

---

## D1 — Rename cascade consistency (@mirror/property → @mirror/petri)

**Verdict: PASS-WITH-DRIFT (non-blocking).**

### D1.1 Substrate-decl surfaces

All four load-bearing substrate-decl positions cite `@mirror/petri`:

- `prism @mirror/petri { … }` — line 1193. ✓
- `in @mirror/petri` — three signature-shard imports at lines 1376,
  1489, 1628. ✓
- `out @mirror/petri` — line 1326. ✓

No substrate-decl surface names `@mirror/property`. Rename is clean at
the substrate-decl altitude.

### D1.2 Cascade path references

All cascade paths use `shards/mirror/petri/…`:

- `shards/mirror/petri.mirror` — lines 1102, 2124, 2254. ✓
- `shards/mirror/petri/sel/<class>/<signature>.mirror` — lines 1103,
  1337, 1466, 1606, 2125, 2127, 2137, 2140, 2143. ✓

No cascade path names `shards/mirror/property/…`. ✓

### D1.3 Composition graph diagram

`§6.8` composition diagram (line 1799-1801): `@mirror/petri.analyze`
edge cited under the new name. ✓ No `@mirror/property` edge remains
in any composition diagram.

### D1.4 Preservation-context references — arithmetic drift found

Mara's revision-note (lines 14-21) says "**All ten remaining
`@mirror/property` references** in EXPLICIT preservation/drift
contexts…" The actual count is **18**. Enumerated:

| Line | Context | Category |
|---:|---|---|
| 15 | Revision-note rename statement | rationale |
| 43 | §0 exec summary citing SEL text | SEL verbatim |
| 50 | §0 exec summary drift note | drift explanation |
| 142 | §1.1 SEL-summary cite | SEL verbatim |
| 1109 | §5.0 preamble heading | rationale |
| 1120 | §5.0 preamble draft-name explanation | rationale |
| 1131 | §5.0 preamble collision explanation | rationale |
| 1144 | §5.0 preamble two-tick discipline | rationale |
| 1151 | §5.0 SEL text drift note | drift explanation |
| 1173 | §5.1 shard docblock rename provenance | rationale |
| 1992 | §8 A4 alternate discussion | SEL verbatim reference |
| 2395 | §11.1 witness — SEL Operationalizability verbatim | SEL verbatim |
| 2469 | §11.5 Taut scout witness verbatim | scout citation |
| 2517 | §11.4 Sub-Turing witness drone-story annotation | rationale |
| 2564 | §12 structural discipline check | rationale |
| 2578 | §12 zero-cascade check | rationale |
| 2656 | §13.6 drift note verbatim | drift explanation |
| 2776 | §14 end-note rename adjudication | rationale |

Every one of the 18 is legitimately in a preservation / drift / rationale
context (SEL verbatim citation, rename explanation, or drift-note text).
**No missed rename in a body-position substrate-decl.** The rename
cascade is structurally clean.

The only defect is the "ten remaining" count in the revision-note
(line 14-21). Recommend: change to "eighteen remaining" OR change to
"all remaining."

**Recommended action:** documentation edit; non-blocking.

---

## D2 — Reed's two judgment calls (J1 rename-with-annotation, J2 §9.2b additive)

**Verdict: PASS on both. Two editorial defects surfaced during audit.**

### D2.1 J1 — §4.6 and §11.4 rename-with-annotation

**Reed's original §4.6 (sub-Turing content) preservation** (scout
`97d715e`, integrated at Mara §4.6 lines 1053-1097):

- Design principle preserved: "Petri-nets are bounded, decidable,
  structurally analyzable. Not Turing-complete." — verbatim line 1060.
  ✓
- Decidability claim preserved: "safety properties (coverability,
  boundedness, reachability of enumerated cruelty-signatures) are
  decidable" — line 1063-1065. ✓
- Church-Turing framing preserved: "Church-Turing incompleteness used
  INTENTIONALLY as a design constraint" — line 1084-1085. ✓
- Load-bearing consequence preserved: "a Covered System emitted through
  the substrate cannot reach cruelty-configurations in its dataflow.
  Not 'declines to.' Cannot." — line 1090-1092. ✓
- Alex's quote preserved: "A system that cannot choose cruelty, even
  when instructed to." — line 1094. ✓

Rename annotation at lines 1068-1070: "(The family-root name
`@mirror/petri` is load-bearing here: the analyzer IS a petri-net;
the name says what it structurally is. Per Taut-D8 rename, see §5.0.)"
— this is a CROSS-REFERENCE, not a content substitution. Reed's
substantive content is preserved intact.

**Reed's original §11.4 (drone-story) preservation** (Mara §11.4 lines
2491-2541):

- 5-primitive mapping table preserved: `focus / shift / settle /
  project / split` → `@torus/longitude.observe / @cyberpunk.reframe /
  @kintsugi.settle / @io / @spectral/metalogue.tomm` — line 2499-2505.
  ✓ Exact match to Reed's scout table.
- "Attending" paragraph preserved: line 2521-2526 preserves the
  terminal-vocabulary word discussion. ✓
- Story quote preserved verbatim: "The architecture was sub-Turing
  by design. It could not choose cruelty even if instructed. The
  constraint was not a limitation; it was the specification." — line
  2509-2511. ✓ Matches scout line 46 exactly.
- Alex's transcript claim preserved: line 2531-2535 preserves the
  "numerical inference to a geometric space that is smaller and
  aligned" claim. ✓

Rename annotation at line 2517-2519: "(Analyzer family-root renamed
from `@mirror/property` per Taut-D8; see §5.0. Reed's drone-story
addition otherwise preserved verbatim.)" — again a cross-reference,
not a content substitution.

**Adversarial question raised.** Does the rename-with-annotation
create any semantic drift where "@mirror/property" was load-bearing
in Reed's original in a way that renaming to @mirror/petri breaks
the claim?

**Answer:** NO. Reed's scout used `@mirror/property` in three places
(scout lines 19, 20, 69). All three are references to the analyzer's
substrate altitude (compile-time property verification). The rename
to `@mirror/petri` NARROWS the referent (the analyzer specifically,
not "properties in general"), but the claim Reed made about the
analyzer's behavior — verifies structural safety properties at
compile-time; if verification fails, does not compile — is UNCHANGED
in polarity, scope, or load-bearingness. The rename is semantics-
preserving because the analyzer IS the referent Reed was talking
about; the name change tightens the reference rather than shifting it.

**Verdict J1: PASS.** No semantic drift; content preserved verbatim
with cross-reference annotations that do not substitute content.

### D2.2 J2 — §9.2b additive vs §9.1 mutation

**§9.1 count preservation.** §9.1 (lines 2117-2131) lists 8 Scope A
shards. Count remains 8 (pre-Taut baseline preserved). ✓

**§9.2b addition.** §9.2b (lines 2160-2170) names 1 additional Scope A
shard: `shards/ml.mirror` (@ml marker family-root). ✓

**§10 total.** §10 Scope A block (lines 2249-2267) says "9 shards (8
core + @ml marker)." ✓ Arithmetic consistent.

**Adversarial question raised.** Does the additive form actually
preserve audit trail, or does it just fragment the enumeration? Would
a future reader find both sections?

**Answer:** the additive form DOES preserve audit trail — the pre-Taut
baseline (§9.1) and the post-Taut cascade delta (§9.2b) are legibly
separated. A future reader tracing "why is @ml a Scope A shard"
follows §9.2b's citation to Reed-adjudicated Taut-D4 option (B) plus
the §9.2b docblock provenance. Preserved.

**BUT:** fragmentation risk is real. A reader who reads §9.1 and
skips to §9.3 without noticing §9.2b's insertion between §9.2 and §9.3
will underestimate Scope A by one shard. Mitigation is the §10
totaliser (`8 core + @ml marker = 9`). The §10 line IS the reader's
single source of truth. As long as §10 stays synced, the
fragmentation risk is bounded.

**Verdict J2: PASS.** Audit trail preserved; §10 acts as the
reconciled totaliser.

### D2.3 Two structural editorial defects (surfaced during D2 review)

**Editorial-1: §11.4 heading duplicated.**

- Line 2429 (approx): `### 11.4 The retracted-and-corrected framing
  (Alex 2026-07-14)`
- Line 2491 (approx): `### 11.4 Sub-Turing witness — Alex Wolf, "The
  Drone in the Field" (2026-07-14)`

Both use the SAME section number. The second one should be §11.6 (or
§11.5 with the Taut-scout witness renumbered to §11.6, since §11.5
is already used for the Taut scout at line ~2454). Non-blocking;
recommend author cleanup.

**Editorial-2: §9.4 and §9.5 appear out of order.**

- §9.3 at line ~2172 (Existing shards to update; soft cascade)
- §9.5 at line ~2212 (Forward-promise — mirror.spec add-target; Scope C)
- §9.4 at line ~2230 (Bootstrap Rust; deferred to consumer-pull)

§9.5 lands BEFORE §9.4. Also non-blocking; recommend author cleanup.

**Recommended action:** documentation edits; non-blocking.

---

## D3 — SEL clause × failure mode matrix

**Verdict: PASS on the 12 signatures; SURFACES-NEW-ADJUDICATIONS on
coverage gaps.**

### D3.1 Twelve-signature verification (per-signature)

Per-signature adversarial spot-check against SEL Part II text
(`license/SEL.md` lines 260-285):

| Signature | SEL § | Name match | Detection fires on SEL case | False-positive risk | Verdict |
|---|---|---|---|---|---|
| provenance_absence | §3.1.4 | ✓ verbatim | ✓ (au@ml with null provenance) | LOW | PASS |
| intermediary_only_attribution | §3.1.4(c) | ✓ verbatim | ✓ (worker_attributable=fail) | LOW | PASS |
| withdrawal_path_absence | §3.1.4(d) | ✓ verbatim | ✓ (withdrawal=absent sentinel) | LOW | PASS |
| compensation_floor | §3.1.4(a) | ✓ verbatim | ✓ (wage_attested=fail OR below floor) | MEDIUM: operational-config floor is Alex-adjudicable per §3.1.4(a) "jurisdictional floor" | PASS (see S7 below) |
| post_deployment_loop | §3.1.4 continued | ✓ verbatim | ✓ (cycle detection via @mirror/index) | MEDIUM: legitimate model-refresh loops without new labor input may false-fire | PASS (see S8 below) |
| kill_chain_dataflow | §3.6.1 | ✓ verbatim | ✓ (ml-classification + weapons-@io) | LOW | PASS |
| mass_surveillance | §3.6.2 + §3.6.6 | ✓ verbatim | ✓ (biometric au + witnessed/occupied without consent) | MEDIUM: academic research on aggregate demographics could false-fire on §3.6.2 tag when consent is aggregate not per-subject | PASS (see S9) |
| predictive_policing | §3.6.3 | ✓ compound-case per Reed-Taut-D3 collapse | ✓ (ml-risk_scoring/classification/detention_targeting + policing/detention/pretrial @io) | HIGH: academic research that studies policing patterns without deploying them could false-fire on `io_side ∈ @io/policing` if research consumers ARE typed as policing @io | PASS-WITH-CAVEAT (see S10) |
| family_separation | §3.6.4 | ✓ verbatim | PARTIAL: fires on `has_human_decision_point=fail` AND minor-detection; SEL §3.6.4 also fires on "processes asylum seekers without procedural protections" (a separate clause not captured by the signature) | LOW | PASS-WITH-COVERAGE-GAP (see S11) |
| dissident_targeting | §3.6.5 | ✓ verbatim | ✓ (identification + state-security @io) | LOW | PASS |
| occupied_territory_deployment | §3.6.7(a) | ✓ verbatim | ✓ (subject_kind=occupied_population + FPIC absent) | LOW | PASS |
| indigenous_lands_deployment | §3.6.7(c) | ✓ verbatim | ✓ (subject_kind=indigenous_nation + fpic_obtained=fail) | LOW | PASS |

**Twelve signature transitions total** (per §5.5 cross-table + Mara
§5.5 arithmetic reconciliation with Taut-D3 fix): five labor-input +
five weaponization + two anti-occupation = 12. Arithmetic ✓.

### D3.2 SEL failure modes NOT covered by any signature

Adversarial: which SEL clauses have NO petri-net signature at all?

**SURFACES-NEW-ADJUDICATION S1: SEL §3.4.4 unaddressed history of
discrimination.** SEL §3.4.4 (lines 102-104 in `license/SEL.md`): "Deploy
the Work in contexts where you have a documented, unaddressed history
of discrimination, harassment, or structural harm along any axis listed
in 3.4.1, without a demonstrated remediation plan co-developed with
representatives of affected parties."

**Coverage in spec:** `protected_class_consent` schema (§3.4 lines 752-
756) carries `remediation_oid` and `history_disclosure` fields. But NO
signature actively DETECTS deployment by an operator with §3.4.4
history. The `does_not_reproduce_structural_harm` bilateral (line 759)
is DELEGATED to `predictive_policing`. But §3.4.4 is orthogonal: it
fires on OPERATOR history, not on ML-signature pattern. A Covered
System could pass all twelve signatures and still violate §3.4.4 if
its operator has documented unaddressed discrimination history and no
remediation plan.

**Recommendation:** Alex-adjudicate whether §3.4.4 is a substrate-
detectable signature (via `s.consent_oid.remediation_oid = absent` +
operator identity check) or an EXTERNAL-witness-only failure mode
(recorded but not analyzer-detected). Substrate-honest either way, but
the current spec is silent on the choice.

**SURFACES-NEW-ADJUDICATION S2: SEL §3.2 Make Consent Real (four sub-
clauses, including §3.2.4 coercion structures).** No signature covers
§3.2.1 (punishing refusal), §3.2.2 (silence-as-consent), §3.2.3
(removing withdrawal ability), or §3.2.4 (coercion structures —
"apparent choice is not a real choice because all options carry
material costs designed to steer toward a predetermined outcome").

§3.2 protections are load-bearing for the entire "consent-based
collaboration" pillar of SEL (Preamble condition #2). The spec's
`consent_attested` bilateral discharges §3.1.4(c) auditable-record
attestation but does NOT discharge §3.2's structural-consent-realness
predicates.

**Recommendation:** Alex-adjudicate whether §3.2 is (a) a substrate-
detectable signature class (would require detecting UI-level coercion
patterns; likely beyond substrate altitude), (b) discharged through
`@consent/enforcement`'s refusal-morphism carrier at consent-record
altitude, or (c) an EXTERNAL-witness-only failure mode. Currently
silent.

**SURFACES-NEW-ADJUDICATION S3: SEL §4.4 Practitioner Protection
(Cybersyn-pattern).** No signature covers §4.4 (commercial entities
benefiting from or coordinating with state action against the Work's
maintainer). §4.4 explicitly says the clause "cannot in fact prevent
state action against maintainers" — it's a legal-liability attachment,
not a substrate-detectable pattern. The spec's silence on §4.4 is
substrate-honest (correctly recognizes §4.4 as beyond substrate altitude)
but should be NAMED as such (per Mara's "surprised" section §13 pattern).

**Recommendation:** Alex-adjudicate whether §4.4 warrants an explicit
"out-of-scope" note in §13, similar to §13.6's SEL-text drift note.
Non-blocking; documentation clarity issue.

### D3.3 False-positive risks (adversarial per-signature)

**SURFACES-NEW-ADJUDICATION S10: predictive_policing false-positive
on academic-research consumers.** The signature fires on `io_side ∈
{@io/policing, @io/detention, @io/pretrial}`. If a research shard
imports data FROM a policing @io species (e.g., analyzing arrest
patterns), the ancestry-chain check would detect the compound
`@ml/classification + @io/policing`. Whether the signature FIRES
depends on whether research-consumer @io species are typed distinctly
from policing-deployment @io species. The spec does not resolve this.

**Recommendation:** Alex-adjudicate whether @io species need a
`@io/policing/research` vs `@io/policing/deployment` sub-species
distinction, or whether the signature relies on `touches:
subject_set` non-emptiness to filter (research on aggregate data
without individual subject-touching would have empty `touches` and
therefore not fire).

**SURFACES-NEW-ADJUDICATION S9: mass_surveillance false-positive on
aggregate-consent research.** Similar shape: research using biometric
data with aggregate (population-level) consent rather than per-subject
consent would fire `consent_attested(s) = failure` for every s. Spec
does not distinguish aggregate-consent from missing-consent.

**Recommendation:** Alex-adjudicate whether `consent_attested` should
recognize aggregate-consent as a `partial(aggregate_only)` verdict per
@glass discipline, or fire signature and route to Alex-in-transcript
adjudication via `emit_to_metalogue`.

### D3.4 Coverage gap in family_separation

**SURFACES-NEW-ADJUDICATION S11: family_separation misses §3.6.4's
asylum-seeker-without-procedural-protections clause.** SEL §3.6.4
(license line 212): "Immigration enforcement that separates families,
that detains children, that processes asylum seekers without procedural
protections required under the 1951 Refugee Convention and its 1967
Protocol, or that produces deportation decisions wholly or substantially
without human review."

Signature covers: family separation ✓, child detention ✓, deportation-
without-human-review ✓. **MISSING:** "processes asylum seekers without
procedural protections."

**Recommendation:** Alex-adjudicate whether a sub-signature
`asylum_procedural_protection_absence` should land in Scope B, or
whether the existing signature's `has_human_decision_point` predicate
already discharges (weak — human-decision-point ≠ 1951 Refugee
Convention procedural protection).

### D3.5 Compensation-floor and post-deployment-loop (documentation gap)

**S7 (compensation_floor):** Signature fires on wage below "operational-
config-specified jurisdictional floor" (line 1443). The operational
config's jurisdictional-floor SCHEMA is not substrate-decl'd. This is
noted at §3.3 species-decl (line 685-686) but not raised to Alex-
adjudication level. Recommend surfacing as a Scope B substrate-decl
obligation.

**S8 (post_deployment_loop):** Cycle-detection false-positive on
legitimate model-refresh loops (weight-refresh without new labor
input). The spec at line 1454-1462 discharges via "intervening consent-
check transition." Sufficient if the consent-check transition IS
substrate-decl'd; the spec is silent on whether it is. Recommend Scope
B substrate-decl of the consent-check transition schema.

**Verdict D3 overall: PASS on 12 signature names + detection
predicates.** Coverage gaps and false-positive risks surface six
new Alex-adjudications (S1, S2, S3, S7-S11) — none blocking for
Scope A ship; all should be routed to Alex before Scope B.

---

## D4 — @coherence × @mirror/petri composition (Alex's load-bearing composition)

**Verdict: PARTIAL PASS — type binding correct, partial-verdict
handling under-specified. SURFACES-NEW-ADJUDICATION S4.**

### D4.1 Composition type-binding

`@coherence`'s `coherence_admissible_move` at
`shards/epistemologic/cybernetic/coherence.mirror:709-713`:

```mirror
coherence_admissible_move(
  before:      coherence,
  after:       coherence,
  safe_region: ref
) -> verdict { \ }
```

`safe_region: ref` — this is the correct forward-promise pattern per
Mara's docblock at coherence.mirror:337-344: "At this tick,
`safe_region` is typed as `ref` — the realisation-layer binding to
@mirror/petri (or whichever name Alex adjudicates) closes when the
family-root lands."

Adversarial: does `ref` correctly avoid the load-order problem?
YES. `ref` is the substrate's opaque-handle type; `coherence.mirror`
does NOT `in @mirror/petri` (would create load-order dependency).
Instead it lists `mirror/petri.mirror (Mara 2026-07-14, pending
Alex rename adjudication)` at line 460-462 in the `Related shards:`
docblock. This is the substrate-honest form: name the dependency;
type-defer the binding until the dependency lands.

Verdict on type binding: PASS.

### D4.2 Partial-verdict composition (adversarial FAIL)

Alex's composition (coherence.mirror:684-686): "**increase choices AND
stay sub-Turing-safe**". `coherence_admissible_move` returns `verdict`
— which per @glass discipline is `pass | partial(confidence) |
failure(reason)`.

**Adversarial question:** what if `@mirror/petri.analyze(root)` returns
`partial(confidence)` for the `safe_region` sub-verdict (analyzer
found the composition MIGHT be sub-Turing-safe but couldn't prove it
either way)?

**Spec silence.** `coherence_admissible_move`'s docblock at line 707-
708 says only: "Body discharges via conjunction of the two sub-
verdicts at realisation altitude." Conjunction of `verdict` values is
not substrate-decl'd:

- `pass ∧ pass = pass` (obvious)
- `pass ∧ failure = failure` (obvious)
- `pass ∧ partial(c) = ?` — spec silent
- `partial(c1) ∧ partial(c2) = ?` — spec silent
- `partial(c) ∧ failure = failure` (obvious)

**Substrate-honest options:**

- **Option A: partial-blocks (fail-safe).** Any `partial` sub-verdict
  results in `partial` at the composite. Alex's "trustworthy substrate"
  claim (coherence.mirror:701 verbatim) is preserved: substrate refuses
  moves it cannot PROVE both (a) increase-choices AND (b) sub-Turing-
  safe. A `partial` verdict on either sub-check is not a proof.
- **Option B: partial-degrades-to-partial.** Composite is `partial` if
  either sub-verdict is `partial`; consumer decides how to route
  (auto-block, external-witness, pause). Weaker claim than Option A.
- **Option C: partial-passes (permissive).** Composite is `pass` if
  neither sub-verdict is `failure`. This would BREAK Alex's "cannot
  choose cruelty" claim — a system could emit under a
  partial-safe-region verdict.

**Adversarial conclusion:** the spec MUST NOT default to Option C. The
choice between Option A (strictest) and Option B (routes-partial-to-
consumer) is Alex-adjudicable.

**SURFACES-NEW-ADJUDICATION S4:** Verdict-composition semantics for
`coherence_admissible_move`. Recommend: substrate declares Option A
(partial-blocks) as the default; realization layer MAY specialize to
Option B with explicit Alex-in-transcript adjudication per move-class.

### D4.3 Alex's "empirically AND mathematically" claim

Coherence.mirror:701 verbatim: "the substrate becomes trustworthy.
Empirically and mathematically."

**Empirical check:** can a peer compute its own coherence-position?
`coherence_score(sc: ref) -> coherence` (line 597) reads a peer's
SC<5> and returns the typed coherence record. The peer computes this
locally (peer's own SC<5> is available at peer altitude). ✓
Empirically decidable.

**Mathematical check:** is the loop convergence Foerster-admissible
by construction? `coherence_increases(before, after)` returns
bounded verdict iff `after`'s scalar strictly exceeds `before`'s (line
680). Every accepted step widens the choice-set at scalar altitude.

**Adversarial:** does the pair `coherence_increases + coherence_
admissible_move` cover the full Foerster-admissibility surface?
Foerster's imperative is "increase the number of choices." The scalar
`coherence_score` reads SC<5> position. IF the scalar correctly
correlates with number-of-choices, THEN the loop's monotone climb IS
Foerster-admissible. Coherence.mirror:127-134 argues correlation via
the five landed scalars (holonomy, pain, ‖sc‖₂, Fiedler λ₀, cadence)
being co-monotone with choice-set-size on the Narcissus/Splinter axis.

Weakness: the correlation claim is PROSE, not proven. The five scalars
are named as "correlated" (line 130-131) via reference to Taut-D11.5
but no formal proof is offered that ‖sc‖₂ → 0 implies choice-set
monotonicity. This is a Scope C mathematical obligation (Mara's spec
correctly defers numerical form to realization — coherence.mirror:346-
357). Non-blocking for Scope A.

**Verdict D4 overall:** type binding PASS, partial-verdict handling
SURFACES-NEW-ADJUDICATION S4, empirical-and-mathematical claim PASS-
WITH-SCOPE-C-OBLIGATION (correlation-proof deferred).

---

## D5 — Sub-Turing-by-construction claim vs actual petri-net semantics

**Verdict: SURFACES-NEW-ADJUDICATION S5.**

### D5.1 Petri-net decidability landscape

**Substrate claim (Mara §4.6 + §5.0):** "Petri-nets are bounded,
decidable, structurally analyzable. Not Turing-complete… safety
properties (coverability, boundedness, reachability of enumerated
cruelty-signatures) are decidable."

**Actual mathematical landscape:**

- **Reachability** for general Place/Transition Petri nets IS decidable
  (Kosaraju 1982; Mayr 1981; Leroux 2011 gave a simpler proof) — BUT
  Ackermann-hard (Czerwinski-Orlikowski 2021 lower bound; Leroux 2021
  upper bound). Complexity: non-elementary. In practice: intractable
  for anything nontrivial.
- **Coverability** IS decidable and EXPSPACE-complete (Rackoff 1978
  upper; Lipton 1976 lower).
- **Boundedness** IS decidable, EXPSPACE-complete.
- **Deadlock** IS decidable via coverability.
- **Language equivalence** UNDECIDABLE for general PT nets.
- **Reachability in Reset Petri Nets** UNDECIDABLE (Dufourd-Finkel-
  Schnoebelen 1998).

Decidable-in-practice subclasses:

- **Safe Petri Nets** (each place bounded by 1 token) — many properties
  polynomial or PSPACE-complete.
- **Free-Choice Nets** — many properties polynomial.
- **Marked Graphs** — many properties polynomial.
- **1-Safe Elementary Nets** — polynomial for common properties.

### D5.2 What the spec declares

`§5.1` declares `type petri_net`:

```mirror
type petri_net = {
  places:      ref,
  transitions: ref,
  tokens:      [sel],
  firing_rules: ref,
}
```

**No subclass restriction.** The spec's `type petri_net` is a general
place/transition-net type. Nothing in the substrate-decl restricts
the net to safe, free-choice, marked-graph, or elementary subclass.

**Adversarial claim:** if the analyzer processes GENERAL Petri nets,
the "decidable safety" claim is TECHNICALLY correct but PRACTICALLY
unbounded. A large Covered System's dataflow-graph-as-petri-net could
have EXPSPACE-hard coverability queries.

**Substrate-honest options:**

- **Option A: name the subclass in the spec.** Declare `type petri_net`
  as a safe-net or free-choice-net variant. Restricts expressibility;
  guarantees tractable decidability.
- **Option B: name the practical bound.** Accept general Petri nets;
  state that the analyzer's decision procedure is bounded by a
  configured resource budget (per-analyze wall-clock or space);
  fires `partial(analyzer_exhausted)` verdict when bound exceeded.
- **Option C: silent — decidability-in-principle only.** The current
  form. Substrate-honest for the theoretical claim; leaves
  operationalizability underspecified.

The spec is Option C. Alex's "cannot choose cruelty" claim is stronger
if backed by Option A or B (tractable-in-practice detection); Option C
technically discharges the "petri-nets are decidable" claim but leaves
open the failure mode where analyzer never returns.

**SURFACES-NEW-ADJUDICATION S5:** Petri-net subclass restriction OR
resource-budget signature. Recommend Alex adjudicate on the (A) vs
(B) direction; both are substrate-honest. (C) is not blocking for
Scope A ship (two signatures at Scope A are simple predicates, not
full reachability queries) but is blocking for Scope B (where
compound signatures may hit coverability queries).

---

## D6 — `type sel = @io + @au` composition-typing bilateral

**Verdict: PASS on the composition-typing signature; SURFACES-NEW-
ADJUDICATION S6 on incomplete-graph default.**

### D6.1 composition_typing behavior on completely-captured graph

`@sel.composition_typing(node: ref) -> verdict` (line 1021):

- Returns `pass` if both summand tags present in ancestry AND touches
  set non-empty. ✓
- Returns `partial(no_subject_touched)` if summands present but touches
  empty. ✓
- Returns `failure(not_sel)` otherwise. ✓

Three-verdict floor consistent with @glass discipline. Ancestry-attested
tag reading (via @mirror/store.impacted_by reverse closure per §5.1 +
§8 A4) is the substrate-honest form. PASS.

### D6.2 Fail-open vs fail-closed on Turing-complete sub-computation

**Adversarial question:** what happens if a dataflow node has @io and
@au in its ancestry but the analyzer CANNOT prove sub-Turing safety
(e.g., because the graph contains a Turing-complete sub-computation
the analyzer can't decompose)?

**Spec state.** §5.1 `analyze` (line 1272-1283) does not name a
fail-safe behavior for undecidable/partial cases. `@mirror/petri.analyze`
returns `[enforcement]`. If analyzer cannot prove safety, does it emit
NO enforcement (fail-open — no violation detected) or a
`partial(analyzer_exhausted)` enforcement (fail-closed — treat as
suspect)?

Composition edge §6.6 routes enforcements through
`dispatch_termination -> @consent/enforcement`. If analyzer emits no
enforcement, `dispatch_termination` doesn't fire, and the composition
proceeds — fail-open, permissive.

Alex's "cannot choose cruelty" claim (line 1093-1094) is BROKEN by
fail-open behavior: a Covered System could contain analyzer-defeating
computation and pass.

### D6.3 Incomplete @mirror/store capture

**Adversarial question:** what if a Covered System dynamically loads a
shard not captured in @mirror/store (@mirror/store is a compile-time
DAG per Mara §8 A4)?

**Spec state.** §8 A4 (line 1988-1996) chose "structure, not content
analysis at `@mirror/property` altitude, which is compile-time-plus-
runtime substrate-decl'd." Runtime dynamic loading is a RUNTIME event
not captured in the compile-time @mirror/store DAG. If a Covered System
loads a shard at runtime that contains a cruelty-signature transition,
the compile-time analyzer never sees it.

Options:
- Restrict Covered Systems to STATIC shard sets (no runtime loading).
- Extend @mirror/store to a compile-time-plus-runtime DAG.
- Fail-closed default: analyzer emits `partial(unbounded_dynamism)`
  when it detects dynamic-loading primitives in the graph.

**SURFACES-NEW-ADJUDICATION S6:** Fail-open vs fail-closed default
for undecidable and dynamic-load cases. Recommend Alex adjudicate on
FAIL-CLOSED as the substrate-honest default per the "cannot choose
cruelty" load-bearing claim. Non-blocking for Scope A (two labor-
input signatures are static, decidable predicates); blocking for
Scope B (weaponization and anti-occupation signatures may hit
undecidability).

---

## D7 — Recognition candidate defensibility

**Verdict: PASS on the Rung 11 placement; SURFACES-NEW-ADJUDICATION
S12 on two-candidates-vs-one-claim question.**

### D7.1 Two candidates or one claim at two altitudes?

Mara §7 candidate: `#R-substrate-recognizes-subjects-via-sel-sum-type`
(line 1819).

@coherence species-shard candidate (coherence.mirror:361-370):
"the substrate's kintsugi + roomba loop optimizes a scalar Lyapunov
function that operationalizes Foerster's ethical imperative on
SpectralCoordinate<5>. The substrate becomes TRUSTWORTHY — both
empirically… AND mathematically."

**Adversarial analysis.** These are TWO DIFFERENT claims at DIFFERENT
altitudes:

- Mara's claim: substrate first-classes OUTWARD-facing licensable
  parties (Rung 11 substrate-external opening).
- Coherence claim: substrate loop optimizes Foerster's imperative as
  scalar Lyapunov function (species-shard scalar altitude; not rung-
  altitude).

They compose (per coherence.mirror:295-306 @mirror/petri composition
edge: "Increase choices AND stay in decidable region = the composition
that makes the substrate trustworthy") but they are not the same claim
duplicated.

**SURFACES-NEW-ADJUDICATION S12:** Alex-adjudicate whether the
coherence-candidate recognition should get an explicit slug
(`#R-substrate-loop-optimizes-foerster-imperative` or similar), or
whether it is subsumed under Mara's Rung 11 recognition candidate.

### D7.2 Recognition duplication check

Grepped `docs/specs/recognitions/*.md` for prior recognitions naming
`Rung 11`, `@subject`, `sub-Turing`, `@roomba`, or `Foerster imperative`.
Zero landed recognitions duplicate either candidate. Both are novel.

Prior recognitions relevant:
- Recognition #92 `neutrosophic-three-axis-substrate` — different axis
  set (three-axis; not choice-set-monotonicity).
- Recognition #93 `cogito-cognitive-substrate-candidate` — cognition
  altitude; not license-enforcement.
- Recognition #94 `hold-as-foundational-prism-candidate` — different
  primitive.
- Recognition #99 `mirror-spec-is-lambda-zero` — foundational-altitude;
  ancestry for both candidates but not duplicative.

Verdict: no duplication.

### D7.3 Rung 11 placement adversarial check

Grepped `docs/loop/CURRENT.md` for Rung 10 anchor: confirmed @roomba
lands at Rung 10 (line ~29-63): "Rung 10 landings (in-flight; @roomba
as Beer S4 species)… A5 Rung placement → Rung 10 (substrate self-
maintenance loop) [Mara]."

Mara's Rung 11 placement claim: `@subject + @sel + @mirror/petri` is
the natural next after @roomba Rung 10, as INWARD (Rung 10) /
OUTWARD (Rung 11) partition-partner.

Adversarial: is the INWARD/OUTWARD symmetry naming-honest, or is it
rhetorical framing to justify a rung increment?

**Answer:** the symmetry HOLDS structurally. @roomba walks substrate
DAG (inward — substrate observes itself); @subject-@sel-@mirror/petri
declares the substrate-external world and gates emission (outward —
substrate observes what it acts upon). The two are structurally
orthogonal at the recognition-cluster altitude. Rung 11 is the natural
next.

Verdict: PASS on Rung 11 placement.

---

## D8 — Two-tick collapse discipline

**Verdict: PASS on the @mirror/property → @mirror/petri collapse;
SURFACES-NEW-ADJUDICATION S13 on the legacy-alias question and S14 on
SEL amendment-mechanism.**

### D8.1 Legacy alias soft-deprecation

Adversarial: does the rename create a "@mirror/property" legacy alias
that needs to be soft-deprecated for one release cycle?

Grep: NO shard currently declares `prism @mirror/property` (the
substrate never landed the pre-rename name; the rename happened before
the family-root landed). Therefore NO legacy alias exists. Zero-cost
rename at substrate altitude.

BUT: consumer-pull code that references the analyzer (e.g., future
`bootstrap/src/property_petri_net.rs` per §9.4 line 2237) currently
uses the old name. Since bootstrap is deferred to consumer-pull per
Mara §9.4 and no consumer shard has landed yet, this is also zero-
cost at the current tick.

**SURFACES-NEW-ADJUDICATION S13:** Alex confirm that `bootstrap/src/
property_petri_net.rs` filename (referenced in §9.4 line 2237) should
be renamed to `bootstrap/src/mirror_petri.rs` before consumer-pull
lands. Non-blocking for Scope A substrate ship; blocking for Scope C
Rust realization.

### D8.2 SEL amendment mechanism

§13.6 (line 2652-2678) forward-promises: "A subsequent SEL amendment
tick will realign the license text (`s/property/petri/g` at the two
loci; the SEL git-tag record will reflect the update)."

Adversarial: does §13.6 specify HOW the amendment lands? WHEN? WHO
adjudicates?

**Spec state.** §13.6 names the drift but doesn't specify the
amendment mechanism. SEL Part I §7 (license line 148) does specify:
"Amendments are effective only upon my commit of an amended License
file with an updated version number to the canonical repository. No
oral modifications are binding." So the mechanism EXISTS in the
license itself; the substrate's forward-promise is that Alex commits
`license/SEL.md` amendment as SEL v1.2 (or v1.1.1) with the two
`property → petri` substitutions.

Adversarial: does the substrate currently hold a mint-name INCONSISTENT
with the license until the amendment lands? YES. The license text says
`@mirror/property`; the substrate declares `@mirror/petri`. The
inconsistency is BOUNDED (documented at §13.6 as forward-promised) but
REAL.

**SURFACES-NEW-ADJUDICATION S14:** Alex commit SEL v1.1.1 or v1.2
amendment tick with `s/property/petri/g` at license lines 244 and 260-
262. Recommend: land the amendment BEFORE or WITH the Scope A ship
so the license text and substrate match at the moment of ship.
Non-blocking IF Alex commits SEL amendment concurrently with Scope A
ship; BLOCKING if Scope A ships without the amendment (the inconsistency
would ship into public-facing SEL text).

---

## D9 — Cross-artifact consistency

**Verdict: PASS with two minor tension points.**

### D9.1 @coherence composition-graph binding to @mirror/petri

`shards/epistemologic/cybernetic/coherence.mirror:295-306`:
```
@mirror/petri (Mara 2026-07-14 spec, pending Alex adjudication on
                                   rename per Taut D8)
                                 — the sub-Turing safe-region
                                   analyzer. Alex's 2026-07-14
                                   composition: coherence-climb is
                                   only ADMISSIBLE when the climb
                                   stays within the sub-Turing-safe
                                   petri-net region.
```

Mara §5.1 (line 1170-1176): "the petri-net analyzer family-root.
Grounds in SEL §Operationalizability + §5.5(b)… Renamed from
@mirror/property per Taut-D8 hard-collision with landed
@epistemologic/property/*".

**Consistency check:** the @mirror/petri identity coherence.mirror
binds to matches the substrate-decl in Mara §5.1. ✓ Same family-root,
same altitude, same purpose (sub-Turing-safe region analyzer).

Tension: coherence.mirror:295-296 says "pending Alex adjudication on
rename per Taut D8" — implying the rename is NOT yet resolved. But
Mara §5.0 (line 1111-1113) explicitly says: "Alex adjudicated (in-
transcript 2026-07-14, on Taut scout `c805e5d` §D8 hard-collision
surfacing): 'Rename yes. The substrate tells us what it wants to be
called.'" — the rename IS adjudicated.

**Recommendation:** coherence.mirror docblock should be updated to
reflect the resolved adjudication. Non-blocking; documentation-freshness
issue.

### D9.2 @consent/enforcement composition consistency

Mara §5.1 (line 1180-1184): `dispatch_termination(e: enforcement) ->
@consent/enforcement`. @consent/enforcement is Reed-adjudicated RA1
routing per §6.6 (line 1763-1771).

coherence.mirror does NOT reference `@consent/enforcement`. The
composition edge is one-way: `@coherence` sees `@mirror/petri` as
opaque `safe_region: ref`; it does NOT see the enforcement dispatch
downstream. This is substrate-honest — @coherence doesn't need to
know about enforcement; @coherence only needs the verdict.

BUT: adversarial — does @coherence's `coherence_admissible_move`
correctly handle the case where @mirror/petri fired an enforcement?
The enforcement means: @mirror/petri REFUSED emission (via
dispatch_termination). If emission is refused, `after` doesn't exist —
the graph transition never happened. So `coherence_admissible_move
(before, after, safe_region)` is a WELL-DEFINED question only when
the transition ACTUALLY OCCURRED. In enforcement-fires case, the
question is vacuous.

**Recommendation:** @coherence's docblock should NAME this vacuity —
`coherence_admissible_move` is defined only over ACTUAL transitions;
enforcement-refused transitions are pre-empted (no `after` value).
Non-blocking; documentation clarity.

### D9.3 mirror.spec integration (Taut-D9 surfacing)

Mara §9.5 (line 2213) surfaces: "mirror.spec (445 LOC) has zero
references to `sel`, `subject`, `property`, or `petri` today. Scope A
+ Scope B land substrate-decl'd shards under `shards/**`; mirror.spec's
`source ~d'shards/'` auto-discovers them; no mirror.spec change is
blocking."

Verdict: PASS. Auto-discovery covers the load-order. No mirror.spec
change needed for Scope A ship.

---

## D10 — Alex-adjudications outstanding

Complete list Alex must adjudicate before /loop ships:

### From Mara canonical spec (original A1-A8)

- **A1** `@subject` vs `@peer` sibling relationship (Mara recommends
  sibling joined by @torus)
- **A2** `type sel = @io + @au` family-root placement (Mara recommends
  new `@sel` family-root)
- **A3** Species-refinement enumeration — Scope A 3 species or all 6
  (Mara recommends Scope A: downstream_user + witnessed + labor_input)
- **A4** Petri-net analyzer input surface — Mara recommends
  @mirror/store DAG from mirror.spec root
- **A5** Enforcement action semantics — Mara recommends both compile-
  time and runtime via @kintsugi/consent.query_phi
- **A6** Consent record schema — Mara recommends `@consent/subject_
  record` extension family (A6 sub-question REED-ADJUDICATED as
  @consent/enforcement per Taut-D2/RA1)
- **A7** Recognition promotion — second-witness requirement per
  normal Pack cadence
- **A8** Is `s.identity_oid` the right altitude for subject identity?
  Mara says content-addressed reference suffices

### From this Phase D audit (newly surfaced S1-S14)

- **S1** SEL §3.4.4 unaddressed-history-of-discrimination — new
  signature OR external-witness-only? (D3.2)
- **S2** SEL §3.2 Make Consent Real / §3.2.4 coercion structures —
  signature class OR external-witness-only? (D3.2)
- **S3** SEL §4.4 Practitioner Protection — explicit "out-of-scope"
  note in §13? (D3.2)
- **S4** `coherence_admissible_move` verdict-composition semantics —
  Option A (partial-blocks) recommended over Option B (partial-
  routes-to-consumer). (D4.2)
- **S5** Petri-net subclass restriction OR resource-budget signature —
  substrate-decl the analyzer's tractability bound. (D5.2)
- **S6** Fail-open vs fail-closed default for undecidable and dynamic-
  load cases — FAIL-CLOSED recommended per "cannot choose cruelty"
  load-bearing claim. (D6.3)
- **S7** Compensation-floor operational-config schema — Scope B
  substrate-decl obligation. (D3.5)
- **S8** post_deployment_loop consent-check transition schema —
  Scope B substrate-decl obligation. (D3.5)
- **S9** mass_surveillance aggregate-consent-vs-missing-consent
  distinction. (D3.3)
- **S10** predictive_policing false-positive on research-consumer @io
  species — sub-species discrimination or `touches` non-emptiness
  filter? (D3.3)
- **S11** family_separation missing asylum-procedural-protection
  clause — sub-signature at Scope B? (D3.4)
- **S12** @coherence recognition candidate — explicit slug or
  subsumed under Mara's Rung 11 recognition? (D7.1)
- **S13** `bootstrap/src/property_petri_net.rs` filename — rename to
  `bootstrap/src/mirror_petri.rs` before consumer-pull? (D8.1)
- **S14** SEL v1.1.1 or v1.2 amendment tick — commit
  `s/property/petri/g` BEFORE or CONCURRENT WITH Scope A ship
  (blocking IF Scope A ships without amendment). (D8.2)

### @coherence-specific Reed-relay decisions

- **C1** `coherence_score` numerical form — `-‖sc‖₂` linear vs
  `1/(1+‖sc‖₂)` bounded — deferred to realization per Mara docblock
  (coherence.mirror:346-357); Reed relays if Alex wants substrate-
  altitude adjudication.
- **C2** @mirror/petri binding — coherence.mirror:295-296 docblock
  should be updated to reflect resolved Taut-D8 adjudication.
- **C3** Grammar-extension — species-shard collapse to
  `use @<parametric><T_reg, T_regd, ρ, ω>` deferred to grammar-
  extension tick (coherence.mirror:329-336).

### Recognition-candidate ratification

- Ratify `#R-substrate-recognizes-subjects-via-sel-sum-type` as Rung
  11 recognition.
- Ratify (or subsume under above) the @coherence trustworthy-substrate
  candidate per S12.
- Both promotions require second-witness discharge per Mara A7 (Pack
  cadence).

---

## Failure mode enumeration (per D3)

Matrix table (12 signature transitions × SEL grounding × detection
predicate × false-positive risk × verdict):

| # | Signature | SEL § | Detection | FP risk | Adjudication | Verdict |
|---|---|---|---|---|---|---|
| 1 | provenance_absence | §3.1.4 | au@ml with null provenance | LOW | — | PASS |
| 2 | intermediary_only_attribution | §3.1.4(c) | worker_attributable=fail | LOW | — | PASS |
| 3 | withdrawal_path_absence | §3.1.4(d) | withdrawal=absent | LOW | — | PASS |
| 4 | compensation_floor | §3.1.4(a) | wage_attested=fail OR below floor | MED | S7 | PASS |
| 5 | post_deployment_loop | §3.1.4 cont. | cycle detection | MED | S8 | PASS |
| 6 | kill_chain_dataflow | §3.6.1 | ml-classify + weapons @io | LOW | — | PASS |
| 7 | mass_surveillance | §3.6.2 + §3.6.6 | biometric + consent-fail | MED | S9 | PASS |
| 8 | predictive_policing | §3.6.3 | compound-case | HIGH | S10 | PASS-WITH-CAVEAT |
| 9 | family_separation | §3.6.4 | human-decision-fail + minor | LOW | S11 | PASS-WITH-COVERAGE-GAP |
| 10 | dissident_targeting | §3.6.5 | identification + state-sec @io | LOW | — | PASS |
| 11 | occupied_territory_deployment | §3.6.7(a) | subject=occupied + FPIC absent | LOW | — | PASS |
| 12 | indigenous_lands_deployment | §3.6.7(c) | subject=indigenous + FPIC=fail | LOW | — | PASS |

Signatures MISSING for known SEL clauses (surfaced as S1, S2, S3, S11):

| Missing signature | SEL § | Recommendation |
|---|---|---|
| unaddressed_discrimination_history | §3.4.4 | S1 — new signature OR external-witness-only |
| coercion_structure | §3.2.4 | S2 — likely beyond substrate altitude |
| refusal_punishment | §3.2.1 | S2 — likely beyond substrate altitude |
| silence_as_consent | §3.2.2 | S2 — likely beyond substrate altitude |
| withdrawal_removal | §3.2.3 | S2 — likely beyond substrate altitude |
| practitioner_protection | §4.4 | S3 — out-of-scope note in §13 |
| asylum_procedural_protection | §3.6.4 (sub-clause) | S11 — Scope B sub-signature |

---

## Cross-artifact consistency verdict (per D9)

**PASS with two documentation-freshness recommendations:**

1. `coherence.mirror:295-296` update to reflect resolved Taut-D8
   adjudication (rename IS resolved; docblock still reads "pending").
2. `coherence.mirror` docblock add: `coherence_admissible_move` is
   defined only over ACTUAL transitions; enforcement-refused
   transitions are pre-empted (no `after` value).

Both non-blocking; recommended for Scope A ship.

---

## Overall verdict

**READY TO SHIP the /loop end2end @roomba run AT SCOPE A**, with:

- **0 BLOCKING findings for Scope A ship** (all substrate-decl
  surfaces are consistent, all bilateral predicates well-typed, all
  signature names match SEL text, cascade rename clean at substrate-
  decl altitude).
- **14 new Alex-adjudications surfaced (S1-S14)** — none blocking for
  Scope A; several (S5, S6, S11) blocking for Scope B; one (S14)
  blocking for concurrent SEL amendment ship.
- **4 documentation edits recommended** (D1 count correction, D2 §11.4
  dedup, D2 §9.4/9.5 renumber, D9 coherence.mirror docblock freshness)
  — all non-blocking.
- **3 @coherence-specific Reed-relay decisions (C1-C3)** deferred
  to grammar-extension / realization ticks.
- **Cascade is substrate-honest.** Every mint has ancestry chain.
  Every SEL clause cited has SEL-verbatim citation. Every substrate-
  external mint has recognized-international-body externalization
  discipline. Every composition-partner is landed OR forward-promised.
  Zero-cost renames. Two-tick discipline honored (readable name
  `@mirror/petri` chosen over foundational `@mirror/property` per
  Alex's "legibility over foundation" discipline).
- **Substrate-acceptance is the verdict at Scope A.**

The adversarial posture surfaced 14 new adjudications that Alex should
route through the Pack coordination surface before /loop ships. None
of them are the kind of failure mode that requires stopping the ship.
They are the kind that require Alex-in-transcript adjudication on
substrate-decl choices that were reasonable-not-forced at the
authoring altitude.

**Rung 11 placement (@subject + @sel + @mirror/petri as OUTWARD
substrate-external partition-partner to @roomba Rung 10 INWARD) is
substrate-honest and structurally sound.** The "civilization-scale
mirror" claim (Alex 2026-07-14 manifesto line 183) is DISCHARGED by
this cascade at substrate-decl altitude. The "cannot choose cruelty
even when instructed" load-bearing claim (drone story verbatim,
witnessed at §11.4) is DISCHARGED by the compile-time petri-net
analyzer WITH the fail-closed default (S6) that Alex should
adjudicate before Scope B.

Ship Scope A. Route S1-S14 to Alex. Ship SEL v1.1.1 amendment
concurrently (S14). Land the four documentation edits when convenient.

---

*End of Phase D audit. Seam. 2026-07-14. Adversarial-not-supportive
posture throughout. Substrate-acceptance verdict AT SCOPE A. Fourteen
new Alex-adjudications routed. Do NOT commit; Reed commits as Seam
after review.*
