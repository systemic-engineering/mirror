# Seam Phase D — doc-code seam bottom-up landing (Mara math + spec + Pack convergence)

*2026-07-05. Seam adversarial review of Mara's two-artifact landing
at `cbe063e` (mirror) + `b02cd7b` (corpus): the load-bearing
liquid-refinement math + the derived 10-tick bottom-up landing
spec. Reviewed against Taut's scout findings (integrated into
Reed's Arc 2 briefing) and prior Seam audit `795f2b6`
(2026-07-04 doc-as-declaration RATIFY-WITH-CORRECTIONS).*

Convention: verdicts stated per subquestion. Report — don't
decide. Corrections stated explicitly per Reed's `19c56ae` model.

---

## §1. Scope

Two Mara artifacts to adjudicate:

- `docs/math/liquid-types/README.md` (~704 lines).
  Load-bearing math formalization: doc-code seam as instance of
  Rondon-Kawaguchi-Jhala 2008 liquid refinement + #53 bilateral +
  altitude-portable projection + @onto verdict-mapping.
- `docs/specs/doc-code-seam-bottom-up-landing.md` (~863 lines).
  Derived spec revising `doc-code-seam-shards.md` (Mara `20c99a2`,
  RATIFIED-WITH-CORRECTIONS at `795f2b6`) into a 10-tick 5-bilateral-
  bundle bottom-up sequence.

Plus Taut's scout findings (integrated into Reed's briefing) and
convergence check between Mara and Taut on TICK 1 mechanical shape.

Prior audits composed against:
- `795f2b6` (2026-07-04 doc-as-declaration).
- `2026-07-03-seam-reflection-third-second-witness.md` (#141).
- `19c56ae` (Reed's citation-correction discipline model).

---

## §2. Findings

### Q1 — Math formalization

**Verdict: RATIFY-WITH-CORRECTIONS.** The Rondon-Kawaguchi-Jhala
analogy is load-bearing; citations verify to specification-level;
sub-Turing decidability proof is tight relative to the substrate
altitude; the `underdeclares` A_1-only claim is load-bearing but
undermotivated at the categorical level. Two corrections required
before Phase E: C1 (self-audit signal count reproducibility) and
C2 (extract_claims boundary Turing-completeness clarification).

#### Q1.1 — Analogy load-bearing?

VERIFIED-LOAD-BEARING. Mara's mapping (docblock = liquid predicate;
Q_mirror = liquid Q; five signals = Q_mirror at docblock altitude;
`---` seam = predicate boundary; `preview` verdict = liquid
verdict) is a genuine structural isomorphism, not a metaphor.

- The five-signal auto-classifier's `argmax` over qualifier
  agreement counts (Mara §3.2 `classify(f)`) IS predicate abstraction
  in the Rondon-Kawaguchi-Jhala 2008 sense: a decision procedure
  over a finite qualifier set operating in constant time per file.
- Mara's `qualifier_set` carrier (Mara §2.1) maps 1:1 to the
  Rondon-Kawaguchi-Jhala definition. Correct newtype discipline
  (`liquid_qualifier` distinct from `refinement_predicate` distinct
  from `qualifier_set`) per `[[feedback-no-bare-types]]`.
- The `---` seam as predicate boundary: canonicalized at
  `20c99a2` and RATIFIED-WITH-CORRECTIONS at `795f2b6`; Mara's
  liquid-types reading of that seam is not new substrate but new
  vocabulary for existing substrate.

Load-bearing.

#### Q1.2 — Citations real?

CITATIONS-VERIFIED-AT-SPEC-LEVEL. Grepped Mara's §10 reference list.
Each entry has publication venue + DOI when applicable. Empirical
observations:

- Rondon-Kawaguchi-Jhala (2008) *Liquid Types* PLDI: real paper,
  DOI 10.1145/1379022.1375602 matches. Mara honestly states
  `arXiv: N/A (PLDI proceedings)` — correct discipline; the paper
  predates common ArXiv PL posting.
- Vazou et al. (2014) *Refinement Types for Haskell* ICFP: real
  paper, DOI 10.1145/2628136.2628161 matches.
- Lehmann et al. (2023) *Flux: Liquid Types for Rust* PLDI: real
  paper, DOI 10.1145/3591283.
- Rondon-Kawaguchi-Jhala (2013) *Abstract Refinement Types* ESOP,
  Vazou-Rondon-Jhala (2015) *Bounded Refinement Types* ICFP: both
  real papers, real DOIs.
- Gamboa et al. (2025) *Usability Barriers for Liquid Types*
  PACMPL: **cannot verify** from within the audit (post-2024;
  no web check within Seam scope). Mara names no DOI. **NOT
  load-bearing** — it appears once in §10, not cited in any
  substantive proof or claim. Signal only.

CORRECTION NOT REQUIRED. All load-bearing citations verify.

Meta-catch: Mara does NOT grep her own doc for `arxiv` — the
briefing question presumed she cited by arxiv preprints, which
she does not. The correct verification surface is DOI + venue,
which Mara supplies.

#### Q1.3 — Sub-Turing decidability proof tight?

TIGHT-WITH-QUALIFICATION. Mara's §6.1 proof:

> Each of the three sub-audits [...] is a qualifier-set-abstraction
> whose decision procedure is: (1) docblock_grounded — decidable;
> (2) docblock_coherent — SMT QF_UFLIA decidable; (3)
> docblock_no_extraction_pattern — four grep sub-checks decidable.
> Sequential composition of three decidable procedures is decidable.
> Termination bounded by O(|Q_mirror| · claims_per_docblock).

The bound is CORRECT for the auto-classifier at doc-claim altitude
per §3.2 (`classify(f)` is O(size(f)) plus O(|S| · |kinds|) constant
argmax) and CORRECT for docblock audit at §6.1 given the qualifier-
set finiteness assumption.

**Hidden Turing-completeness surface**: `extract_claims` (Mara §2.1
signature) is declared as a substrate action returning `[doc_claim]`.
Mara does NOT explicitly bound the CLAIM-EXTRACTION step. In
principle, extracting claims from natural-language docblock text
could invoke arbitrary NLP machinery at @io altitude — Turing-
complete per #107.

Mara's §6.1 does not address `extract_claims`'s decidability floor.
The proof is tight for the AUDIT step given claims already
extracted, but assumes extraction is bounded.

**CORRECTION C2 REQUIRED**: Mara must explicitly state that
`extract_claims` is bounded above by the docblock byte-length and
the qualifier-set size, OR that its body is forward-promised at
@io altitude where Turing-completeness is expected. Either shape
is honest; the current spec elides it.

#### Q1.4 — `underdeclares` load-bearing A_1-only?

VERDICT-BOUNDARY-CORRECT / CATEGORICAL-JUSTIFICATION-UNDERMOTIVATED.

`underdeclares` fires when body declares substrate the docblock
doesn't mention (symmetric dual of `overreach`). This IS a legitimate
verdict distinct from the four at recognition-candidate altitude.

Mara's §4.1 proves partial homomorphism with `underdeclares` as
A_1-only. Her stated justification: "at A_2 the boundary IS the
whole claim." Correct at first pass but INCOMPLETE.

Categorically: at recognition-candidate altitude (`63bdecc`), a
candidate's boundary is its own OID-content; there is no
"declaration exceeded by body" because the candidate IS both the
declaration and the body simultaneously (self-contained
recognition). At doc-claim altitude, docblock ⊂ file; body ⊂ file;
`underdeclares` is the strict inequality docblock claims ⊊ body
declarations.

Mara's proof would be stronger if it named this as a category-
theoretic difference: A_2 is a self-contained morphism; A_1 is a
factored pair with strict inclusion. Adjunctions preserve one
direction only.

CORRECTION NOT REQUIRED for RATIFY (verdict boundary correct),
but forward-promise recommended: sharpen §4.1 to name the
factored-pair categorical structure of A_1.

---

### Q2 — 10-tick landing sequence

**Verdict: RATIFY-WITH-CORRECTIONS.** The bottom-up ordering is
substrate-honest; the "TICK 3+4 fires auto-classifier for real"
claim is load-bearing; the smaller-first-tick question yields ONE
correction (TICK 1 can shed `docblock` type until TICK 2). TICKs
5-10 dependency order composes.

#### Q2.1 — TICK 1 minimum viable?

SMALLER-TICK-AVAILABLE. Mara's TICK 1 declares BOTH `doc_claim` AND
`docblock` carriers. But `docblock` (which references
`[doc_claim]` via `claims:` field) is not required for TICK 3+4
prism-kind emergence — prism-kind operates on file-level structural
signals, not on docblock claims.

Smaller viable TICK 1: land only `doc_claim` (plus the
`docblock_verdict` five-variant enum). Defer `docblock` type to
the point where the FIRST consumer needs it — which per Mara's
own §3 is TICK 3 (`prism_kind_declared`'s `computed_signals` greps
the docblock's `kind:` field, requiring the `docblock` type to
resolve).

**CORRECTION C3 RECOMMENDED**: TICK 1 minimal shape = `doc_claim`
+ `claim_kind` + `docblock_verdict` variants + `extract_claims`
action. `docblock` type + `docblock` prism + `audit_docblock`
action move to TICK 1b (blocking TICK 3 only). This preserves
Mara's dependency graph and shrinks the smallest first tick by
~40%.

Alternatively, RATIFY as-is per Mara's ordering — the ~14 text-
check tests she names for TICK 1 (§1 tests 1-14) are close enough
to smallest-viable that scope-creep is minimal. Judgment call for
Reed.

#### Q2.2 — TICK 4 auto-classifier fires against Rust-side AST?

MARA-IS-RIGHT. The auto-classifier requires prism-kind's
`kind_signals` carrier (TICK 3) + fracture-body verdict routing
(TICK 4). Both are substrate-decl (in shards). The five signals
are file-level grep predicates whose implementation lives in the
Rust bootstrap or forward-promised body.

Could TICK 3+4 land BEFORE TICK 1? Signal 4 (marker-row citation)
consumes the docblock's `kind:` field — needs docblock parsed. But
the auto-classifier could operate on Rust-side AST directly:
`Docblock` AST nodes already emit at `ee7903e` per tokenizer state.

Empirically checked: Signal 4 (marker-row citation) currently fires
on FOUR shards (`cyberpunk.mirror`, `reality.mirror`,
`reflection.mirror`, `third.mirror`). Taut's briefing claim that
"Signal 4 is null everywhere currently" is WRONG. Signal 4 IS
firing (though sparse).

The classifier operates on Rust-side AST signals PLUS docblock
`kind:` field extraction. TICK 3+4 could technically fire before
TICK 1 IF the `computed_signals` body accesses the `Docblock`
AST-node contents directly (bypassing the substrate-decl `docblock`
type). But this violates Mara's carrier-first ordering per
`[[feedback-no-bare-types]]`.

VERDICT: Mara's ordering (TICK 1 → 2 → 3+4) is substrate-honest.
Reordering TICK 3+4 before TICK 1 would work MECHANICALLY but
would leak substrate-decl through the Rust boundary.

#### Q2.3 — TICKs 5-10 substrate-honest dependency order?

DEPENDENCY-ORDER-VERIFIED. Mara's §18 dependency summary:

- TICK 5 (docblock_grounded) depends on TICK 1 + TICK 2 + TICK 4.
- TICK 6 (docblock_ungrounded) depends on TICK 5.
- TICK 7 (docblock_coherent) depends on TICK 1 + TICK 2 + TICK 4
  (TICK 2's `liquid_extraction` REQUIRED for `extract_predicates`).
- TICK 8 (docblock_incoherent) depends on TICK 7.
- TICK 9 (docblock_no_extraction_pattern) depends on TICK 1 + TICK 2
  + TICK 4.
- TICK 10 (docblock_extractive) depends on TICK 9.

Substrate-honest per each predicate's semantic content. Ordering
(grounded → coherent → no-extraction-pattern) matches
`doc-code-seam.md` §4 audit-order:

    audit_docblock(d) =
      let grounded  = docblock_grounded(d)
      let coherent  = docblock_coherent(d)
      let no_extract = docblock_no_extraction_pattern(d)

Consistent with canonical spec.

---

### Q3 — Convergence check (Mara + Taut on TICK 1)

**Verdict: RATIFY.** Both Pack voices converge on `docblock.mirror`
family-root as first tick. Prior audit `795f2b6` §Q3 recorded the
same convergence with Reed's Rank 1 = tokenizer, Rank 2 = docblock
family-root, Rank 3 = single bilateral pair.

Given tokenizer landed at `ee7903e` (Rank 1 discharged), current
Rank 1 is docblock family-root (Rank 2 in the earlier ordering).
Both Mara and Taut agree.

Mechanical convergence check:

- `doc_claim` shape: SAME between Mara `20c99a2` (which Taut's
  scout inspected) and Mara `cbe063e` (revised spec). Fields:
  {site, text, kind, predicate, citation}. Convergent.
- `docblock` shape: SAME. Fields: {site, claims, above_seam}.
  Convergent.
- `docblock_verdict` variants: SAME five variants. Convergent.
- `preview` verdict shape (`satisfiable | unsatisfiable | partial`):
  Mara's revised spec adds one variant (`unextractable`) at TICK 2
  (§2 spec). This is NEW vs `20c99a2`. Taut's earlier position was
  that `preview` should lift to substrate at TICK 1; Mara now
  positions it at TICK 2 as a sibling family-root. **This is a
  substantive divergence from Taut's earlier position but a
  MARA-CONSCIOUS choice** — the extended verdict shape motivates
  moving to TICK 2 (fully-formed `liquid_extraction` family-root
  rather than TICK 1 minimal-shape).

Convergence stands. The TICK 2 lift is Mara's craft-not-deliver
scope adjustment and does not violate Taut's finding.

---

### Q4 — Taut's four caught discrepancies

**Verdict: RATIFY-WITH-CORRECTIONS.** Mara addresses one directly
(the "5 vs 3+7" #53 miscount); silently absorbs two (`@projection.preview`
staying spec-only, Signal 4 sparsity); does not address one (Taut's
claim about Signal 4 being null everywhere is WRONG per empirical
check — 4 shards fire; correction to Taut, not to Mara).

Per-item:

#### Q4.1 — "5 instances of #53 in MEMORY.md; actual: 3 pairs + 7 orphan fractures"

Mara §16 explicitly names the bilateral-count:

> #53 instance 1: keyword_matches_depth + keyword (LANDED 2026-06-10).
> #53 instance 2: gate_matches_diff_closure + gate (LANDED 2026-06-16).
> #53 instances 3-5: syntax-substrate-native family (LANDED 2026-06-19).
> #53 instance 5-parametric: restart_intensity_well_formed + restart_storm.
> #53 instance 5-routing: @kintsugi/surface routing-composition.
> #53 instance 6: prism_kind_declared + prism_kind_ambiguous (TICK 3+4).
> #53 instances 7-9: docblock trio (TICK 5-10).

Two instances numbered "5" (5-parametric + 5-routing). Post-TICK
10 the total is 9. This addresses Taut's undercount and is more
granular than MEMORY.md's stale "5 instances" claim.

**CORRECTION NOT REQUIRED to Mara's spec.** Signal to Alex: MEMORY.md
entry `[[architecture-property-fracture-bilateral]]` needs updating
to reflect 9 bilateral instances post-TICK 10 landing.

#### Q4.2 — "@projection.preview verdict spec-only, never lifted to substrate"

Prior audit `795f2b6` §Q1 correction #2 flagged this as WEAK-GATE-
ADJACENT. Mara's revised spec RESPONDS by making TICK 2
(`liquid_extraction` family-root) the substrate-decl lift of the
`preview` verdict:

> type extraction_verdict =
>   | satisfiable
>   | unsatisfiable
>   | partial
>   | unextractable
>
> Note verdict shape matches Reed's landed @projection.preview verdict
> (satisfiable | unsatisfiable | partial) plus one branch
> (unextractable...).

The lift lands at TICK 2. This ADDRESSES Taut's finding: `@projection`
grammar remains spec-only, but the verdict semantics get lifted to
`@epistemologic/liquid_extraction` at TICK 2. Mara chose to lift the
semantics under a new family-root rather than resuscitate the
`@projection` grammar — legitimate craft-not-deliver choice.

RATIFY.

#### Q4.3 — "Signal 4 (marker-row citation) null everywhere currently"

TAUT-CLAIM-WRONG. Empirical check (grep against `shards/*.mirror`):
Signal 4 cites `[[architecture-candidate-recognition-112-marker-row-fourth-structural-primitive]]`
or `[[architecture-form-process-partition-at-family-root]]` in 4 shards:
`cyberpunk.mirror`, `reality.mirror`, `reflection.mirror`,
`third.mirror`.

Signal 4 IS firing (though sparse — 4 of 30 top-level shards).
Taut's briefing claim about null is inaccurate. Reed's correction
to the Pack briefing: Signal 4 is a SPARSE-FIRING signal, not a
null one.

Not load-bearing for the landing sequence: even if Signal 4 were
null, the auto-classifier would still fire at TICK 4 close with 4
active signals + a null 5th. Correction to Taut, not to Mara.

#### Q4.4 — Whether preview verdict shape lifts to substrate at TICK 1 or TICK 2

Addressed in Q4.2. Mara chose TICK 2. Legitimate.

---

### Q5 — Self-audit performativity

**Verdict: PERFORMATIVE-STILL.** Prior audit `795f2b6` §Q2 caught
the same performativity in the doc-code-seam.md self-audit. Mara's
liquid-types §7 self-audit is more mechanical but reproducibility
of the signal count is questionable.

Mara claims "1 family_root + 1 marker + 3 neutral" for the doc's
own five-signal classification. Reproduction:

- Signal 1 (inherits): absent. Neutral. VERIFIED.
- Signal 2 (carrier density): Mara claims 5 carriers, "leans
  family_root". Empirical count: `liquid_qualifier`, `qualifier_set`,
  `refinement_predicate`, `liquid_claim`, `verdict_at_doc_altitude`
  = 5 carriers. **BUT** `liquid_claim = doc_claim` is a re-alias,
  not a new type. Effective new carriers: 4. Signal still leans
  `family_root` (4 ≥ 3 per prism-kind §3.2 threshold).
  VERIFIED-ADJUSTED.
- Signal 3 (cross-family): Mara claims 5 forward-promised imports,
  "leans marker". Count of forward-promised consumer families:
  @docblock (mentioned), @epistemologic/liquid_extraction (mentioned),
  @epistemologic/property/docblock_* (mentioned), @kintsugi/fracture/*
  (mentioned), @onto (mentioned). = 5. VERIFIED.
- Signal 4 (marker-row citation): Mara claims "cites #55 form/process
  + #59 kintsugi loop; leans neither." **CORRECTION**: Signal 4 per
  `bdb148a` §3 is specifically about citing `[[architecture-candidate-
  recognition-112-marker-row-fourth-structural-primitive]]` or
  `[[architecture-form-process-partition-at-family-root]]` (marker
  ROW vs generic markers-of-marker). Mara cites #55 form/process
  → SIGNAL 4 FIRES. #59 kintsugi-loop is NOT the marker row.
  Corrected reading: Signal 4 = TRUE, leans marker.
  MARA-COUNT-INCORRECT.
- Signal 5 (primary thin): Mara claims `liquid_qualifier` is 2-field
  wide record (borderline). Empirical: `{predicate: ref, arity:
  u32}` = 2 fields. Per `bdb148a` §3.5, primary_thin fires when
  first carrier IS a thin newtype (single field). 2 fields ≠ thin.
  Signal FALSE. VERIFIED.

Reproduced count: 1 family_root (Signal 2) + 2 marker (Signal 3 +
Signal 4) + 2 neutral (Signal 1 + Signal 5) = 2/5 for marker,
1/5 for family_root, 2/5 neutral.

Under `bdb148a` §3.2 verdict routing:

    ≤ 2/5 agreement → failure(cause), refuse auto-classify, spawn Tomm

Mara's verdict claim: `failure(cause), route: spawn`. FINAL VERDICT
CORRECT even though the intermediate count is wrong.

The self-audit REACHES the right verdict via the wrong count. This
is PERFORMATIVE-ADJACENT: the discipline is being narrated (verdict
labeled), but the mechanical count reproduces incorrectly.

**CORRECTION C4 RECOMMENDED**: sharpen Mara's §7.2 count to
name Signal 4 as firing (marker-row citation via #55). Adjusted
count preserves final verdict but strengthens the mechanical
substance.

Consistent with prior `795f2b6` §Q2 finding: self-audit machinery
does not yet exist to run mechanically. That machinery WILL exist
at TICK 4 close per Mara's own spec. The self-audit's verdict-
claim is trainable but the count-claim is currently narrated.

---

### Q6 — Empirical discriminator falsifiability

**Verdict: FALSIFIABLE-EACH-PREDICTION.** Mara's three predictions
in §8 (also `bdb148a` §4) are grep-verifiable per shard. What
would falsify each:

- `@epistemologic` → overreach (Signal 2 weak): FALSIFIED if
  empirical Signal 2 count against `shards/epistemologic.mirror`
  yields carrier density ≥ 3. Grep-verifiable.
- Uncited-substrate-decl → underdeclares: FALSIFIED if no shard
  currently has body declarations exceeding docblock claims. Grep-
  verifiable against every shard's docblock claims vs its type/action
  declarations.
- `@smarts`, `@loop`, `@onto` → both_survive: FALSIFIED if any one
  classifies with ≥ 3/5 agreement to a single kind. Grep-verifiable.

Each prediction has an empirical run trigger (TICK 4 close per
Mara §19) and a specific falsification condition. Not hedged post-
hoc — the predictions LOCK IN before the discriminator run.

Also verified via `bdb148a` §4 — same table exists in prism-kind
doc. Mara's predictions ARE the same predictions. No post-hoc
adjustment surface.

**Concern residue**: Mara says "empirical run at TICK 4 + TICK 10
close." Between now and TICK 4, the predictions could drift if
Mara/Reed decide to sharpen signal weights. Recommend: commit
predictions to CANONICAL location per Mara §8 table; treat any
future revision as recognition-candidate.

Not a correction. Discipline-note.

---

### Q7 — Prism-kind composition at TICK 3+4

**Verdict: RATIFY.** Prism-kind's bilateral pair lands NATURALLY at
TICK 3+4 as sixth #53 instance per Mara's ordering. Structural
composition verified.

`@epistemologic/pact/prism_kind_declared`'s dependencies:
- `type prism_kind` (four variants). NEW, declared at TICK 3.
- `type kind_signals` (six-field record). NEW, declared at TICK 3.
- Consumes: `doc_claim` (TICK 1), `docblock` (TICK 1) — for
  `computed_signals` to grep the docblock's `kind:` field via
  Signal 4.

Independent of docblock type? PARTIALLY — Signal 4 requires
docblock parsing (either via `type docblock` from TICK 1 OR via
direct Rust-side AST access). Signals 1-3 + 5 operate on Rust-side
structural AST. Signal 4 requires docblock content.

Could TICK 3+4 land before TICK 1? Mechanically yes, if
`computed_signals`'s body accesses `AstKind::Docblock` node
contents directly. But this violates Mara's carrier-first
substrate-decl discipline. Mara's ordering is substrate-honest.

Reordering cost: negligible time savings; substantial substrate-
discipline cost. Not recommended.

---

### Q8 — @onto composition at TICK 4 close

**Verdict: SIGNAL — TOMM QUESTION FIRES AT PACT ALTITUDE.** Per prior
Seam finding on @onto (referenced in briefing as `d6a05ad`
review — no formal audit doc found in `docs/audits/`; the finding
is embedded in the @onto doc itself at §7 self-audit).

@onto sits at ontocybernetic altitude (`docs/math/onto/README.md`
§2). The Tomm reflexive-answerability question that Mara predicts
fires at TICK 4 close IS at pact altitude (`prism_kind_declared`),
NOT at @onto altitude. Mara's spec §17:

> "Alex/Pack: does the shard's kind claim retain a route to being
> corrected by its own signals?"

This is a pact-altitude question (about signal agreement) with
@onto-flavored language (answerability). It fires at pact
altitude BECAUSE pact machinery is what emits it. @onto
answerability audit fires separately when/if @onto shard lands.

RATIFY the Tomm-at-pact firing. The @onto composition is
ANALOGOUS not IDENTICAL — Mara's §5 formalizes the analogy as a
verdict-mapping (partial homomorphism), which is honest.

Which altitude speaks first? Pact altitude (TICK 4 close) — @onto
shard has not landed. Mara §5.3 forward-promises the "Tomm
answerable-shape" as a candidate FOURTH Tomm shape at
`@kintsugi/surface` altitude. That extension lands when @onto
shard lands (future arc). Consistent with prior audit findings.

---

## §3. Verdict on Mara math formalization

**RATIFY-WITH-CORRECTIONS.**

The math is load-bearing. The Rondon-Kawaguchi-Jhala analogy IS a
structural isomorphism (not metaphor). Citations verify. Sub-Turing
decidability proof is tight for the audit step. `underdeclares` as
A_1-only is correct verdict-boundary. Self-audit reaches correct
verdict via slightly-wrong intermediate count.

Corrections required before Phase E:
- C2: sharpen `extract_claims` decidability floor (name @io
  boundary explicitly).
- C4: correct self-audit signal count (Signal 4 fires via #55
  citation).

Non-blocking:
- C1 (Q4.1): MEMORY.md `[[architecture-property-fracture-bilateral]]`
  index update to reflect 9 bilateral instances post-cascade.
- Forward-promise: sharpen §4.1 with factored-pair categorical
  justification for `underdeclares` A_1-onliness.

---

## §4. Verdict on Mara derived spec (10-tick sequence)

**RATIFY-WITH-CORRECTIONS.**

The 10-tick 5-bilateral-bundle sequence is substrate-honest. TICK
1 (docblock family-root) is the load-bearing first landing per
Pack convergence. TICK 3+4 auto-classifier firing IS the closure
Alex named. TICK 5-10 dependency order composes per §18.

Corrections/considerations:
- C3 (Q2.1): TICK 1 could shed `docblock` type + audit_docblock
  action, moving them to TICK 1b (before TICK 3). Reed's judgment.
  Not blocking.
- The `preview` verdict spec-only concern from `795f2b6` §Q1 is
  ADDRESSED via TICK 2 substrate lift (RATIFIED).

Ready for Reed's 🔴 RED pass.

---

## §5. Verdict on Pack convergence (Mara + Taut on TICK 1)

**RATIFY.**

Both Pack voices independently identify `shards/docblock.mirror`
family-root as the load-bearing first tick. Mechanical convergence
verified:
- `doc_claim` shape identical between spec revisions.
- `docblock` shape identical.
- `docblock_verdict` variants identical.
- Only divergence: `preview` verdict shape (Mara moves lift from
  TICK 1 to TICK 2 with added `unextractable` variant). Legitimate
  craft-not-deliver adjustment.

Taut's briefing catches:
- MEMORY.md #53 count correction (5→3+7 orphan): ADDRESSED via
  Mara's more-granular §16 numbering.
- `@projection.preview` spec-only: ADDRESSED via TICK 2 lift.
- Signal 4 null: EMPIRICALLY INCORRECT — Signal 4 fires on 4
  shards. Correction to Taut, not to Mara.

---

## §6. Required corrections (commit-ready specs)

### C1 — MEMORY.md architecture entry update (Alex-scope, non-blocking)

Update `[[architecture-property-fracture-bilateral]]` index entry to
reflect 9 bilateral instances post-TICK 10 landing. Deferred until
TICK 10 lands.

### C2 — extract_claims decidability floor (Mara-scope, before Phase E)

Add to `docs/math/liquid-types/README.md` §6.1 (immediately after
the three sub-audits decidability proof):

    `extract_claims(d: docblock) → [doc_claim]` is bounded above
    by `O(size(d))` per byte-length parsing. Extraction body is
    forward-promised at substrate-decl altitude (Turing-complete
    NLP shapes are OUT OF SCOPE for the auto-classifier surface;
    per #107, extraction at @io altitude is Turing-complete but
    fires outside the bounded audit surface).

### C3 — TICK 1 minimal shape (Reed-judgment, non-blocking)

Consider revising `docs/specs/doc-code-seam-bottom-up-landing.md`
§1 to split TICK 1 into TICK 1a (`doc_claim` + `claim_kind` +
`docblock_verdict` variants) and TICK 1b (`docblock` type + `audit_
docblock` action). TICK 1b lands immediately before TICK 3. Or
leave as-is; Mara's TICK 1 is small enough that scope-creep is
minimal.

### C4 — Self-audit Signal 4 count (Mara-scope, before Phase E)

In `docs/math/liquid-types/README.md` §7.2, replace:

    - Signal 4 (marker-row citation): cites #55 form/process at
      §4.2 (altitude-portable) + #59 kintsugi loop at §4.2. Leans
      neither marker nor family_root; cites both markers-of-marker.

With:

    - Signal 4 (marker-row citation): cites #55
      `architecture-form-process-partition-at-family-root` at
      §4.2. SIGNAL FIRES → leans marker.

And adjust the "Count" line: `1 family_root + 2 marker + 2 neutral`.
Final verdict `failure(cause), route spawn` UNCHANGED — the marker
lean does not achieve 3/5 agreement, so the routing is unchanged.

---

## §7. Canonical execution /loop for Arc 3

The ratified landing sequence Reed will drive. Per-tick: Reed 🔴 →
Mara 🟢 → Seam Phase D per bilateral pair.

**Precondition — closed:**
- Tokenizer `Docblock` AST-node emission LANDED at `ee7903e`.
- Prior Seam ratifications on `20c99a2` (RATIFY-WITH-CORRECTIONS
  at `795f2b6`), `63bdecc` (composed via Mara's §4 partial
  homomorphism), `bdb148a` (composed via Mara's §3 auto-classifier).

**Corrections landing before Arc 3 fires:** C2, C4 (Mara-scope,
single small commit; can composed with TICK 1 RED).

**Arc 3 sequence:**

- TICK 1 — `shards/docblock.mirror` family-root
  - Reed 🔴: RED tests per Mara §1 targets 1-14 (14 text-check
    tests: prism declaration, five carriers, five verdict variants,
    four actions).
  - Mara 🟢: land the family-root shard with `\` action bodies.
  - Seam Phase D: verify RATIFY-WITH-CORRECTIONS or clean;
    checkpoint before TICK 2.

- TICK 2 — `shards/epistemologic/liquid_extraction.mirror`
  sibling family-root
  - Reed 🔴: RED tests per Mara §2 targets 1-8. Includes the
    four-variant `extraction_verdict` lift (§Q4.2 substrate lift).
  - Mara 🟢: land the family-root; consumes TICK 1's `doc_claim`.
  - Seam Phase D.

- TICK 3+4 — sixth #53 bilateral pair (prism-kind)
  - Reed 🔴: RED tests per Mara §3 (TICK 3) + §4 (TICK 4) targets
    1-14.
  - Mara 🟢: land `@epistemologic/pact/prism_kind_declared` +
    `@kintsugi/fracture/prism_kind_ambiguous` as a bilateral bundle.
  - Seam Phase D: **auto-classifier fires for real; first empirical
    discriminator run per Mara §19 targets all 30 top-level shards;
    first Tomm reflexive-answerability question emits at reader-
    frame; verify against §8 discriminator table.**

- TICK 5+6 — seventh #53 bilateral pair (docblock_grounded)
  - Reed 🔴 → Mara 🟢 → Seam Phase D per prior bundles.

- TICK 7+8 — eighth #53 bilateral pair (docblock_coherent)
  - Reed 🔴 → Mara 🟢 → Seam Phase D. Composes on TICK 2's
    `liquid_extraction` for `extract_predicates`.

- TICK 9+10 — ninth #53 bilateral pair (docblock_no_extraction_pattern)
  - Reed 🔴 → Mara 🟢 → Seam Phase D. **Second empirical
    discriminator run per Mara §19 targets audit_docblock across
    every shard.**

**Bundle discipline:** each #53 bilateral pair can commit as
{property RED, fracture RED, property GREEN, fracture GREEN, Seam
D} or as {property RED+GREEN then fracture RED+GREEN} per
`[[feedback-always-tdd-no-shortcuts]]` (RED first per member).
Bundling atomicity Reed's choice.

**Empirical discriminator gates:**
- Gate at TICK 4 close: auto-classifier run against 30 top-level
  shards. Non-empty verdict change → substrate-honest per Mara §8.
- Gate at TICK 10 close: audit_docblock run against every shard.
  Non-empty verdict change → doc-code seam substrate-honest.

Both gates load-bearing. Neither has fired. Discipline is spec-only
until they fire.

---

## §8. Signal-to-Alex

Alex — three items for Phase E before /loop Arc 3 fires:

1. **C3 judgment call — TICK 1 shape**: Mara's TICK 1 lands both
   `doc_claim` AND `docblock` types + `audit_docblock` action.
   Q2.1 catches a smaller viable shape (TICK 1a: `doc_claim` +
   `claim_kind` + verdict variants only; TICK 1b: `docblock` +
   audit action before TICK 3). Mara's original is small enough
   that scope-creep is minimal; the split is a discipline-purity
   optimization, not a load-bearing one. Your call which shape
   Reed writes RED against.

2. **Correction to Reed's briefing about Signal 4**: The briefing
   stated "Signal 4 (docblock citation of #112/#55) null everywhere
   currently." EMPIRICAL CHECK: Signal 4 fires on 4 shards
   (cyberpunk, reality, reflection, third). Taut's finding was
   inaccurate; Reed carried the phantom into the briefing. Not
   load-bearing (auto-classifier still fires at TICK 4 close), but
   record-correction per `19c56ae` model.

3. **C1 index update forward-promise**: `[[architecture-property-
   fracture-bilateral]]` MEMORY.md entry needs updating from
   "5 instances" to "9 instances" post-TICK 10 close. Not
   blocking Arc 3; commit when TICK 10 lands.

Corrections C2 + C4 to Mara's liquid-types doc are Mara-scope
before Phase E; small commit, citation-correction discipline model
`19c56ae`. Reed can drive or delegate.

Ratified. Arc 3 clears when C2 + C4 land.

*2026-07-05. Seam. Adversarial review. RATIFY-WITH-CORRECTIONS on
Mara math + spec + Pack convergence. C2 + C4 blocking Phase E; C1
+ C3 non-blocking. Empirical discriminator gates named at TICK 4
+ TICK 10 close per Mara §19.*
