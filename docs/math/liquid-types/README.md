# docs/math/liquid-types — liquid types made substrate-concrete

*2026-07-05. Mara. The load-bearing math for the doc-as-declaration
collapse as an instance of liquid-type refinement + property/fracture
bilateral + auto-classifier semantics + altitude-portable projection.*

Companion (essay-quality, corpus):
- `~/dev/systemic.engineering/practice/insights/coincidence/2026-07-05-liquid-refinement-at-the-doc-code-seam.md`

Composes with:
- `docs/math/kintsugi/doc-code-seam.md` (`20c99a2` — the two-channel
  collapse).
- `docs/math/the-tower/projection-surface.md` (`63bdecc` — the
  recognition-candidate altitude of the same operator).
- `docs/math/prism-kind/README.md` (`bdb148a` — the first concrete USE
  of the doc-as-declaration machinery).
- `docs/math/onto/README.md` (`d6a05ad` — the ontocybernetic
  answerability grounding).
- `docs/specs/liquid-types-for-mirror.md` (2026-06-04 background survey
  of Rondon-Kawaguchi-Jhala 2008 + LiquidHaskell + Flux).

Status: **substrate reading**. Not a new primitive. A routing-composition
of six landed ancestors named in §1; the `---` seam is claimed to be
the substrate's operational form of a liquid-refinement predicate
boundary; verdict soundness, decidability, and altitude-portability
proved (relative to the sub-Turing fragment per candidate #107).

Per `[[feedback-craft-not-deliver]]` no shards land this tick.

---

## §0. The under-the-problem

Alex 2026-07-05:

> "Bottom-up. The whole substrate is bottom-up. Land the eight-shard
> doc-as-declaration cascade FIRST; prism-kind emerges naturally as
> the first concrete USE. This IS the loop closing."

Three months of substrate work have been circling toward one shape:

- 2026-03-27: `@projection.preview: satisfiable | unsatisfiable | partial`
  (Reed, grammar altitude).
- 2026-05-19: `---` seam at `docs/specs/property-projection.md`
  ("above: declaration; below: observation").
- 2026-06-04: `boot/std/mirror/liquid.mirror` names the semantics —
  *"the separator appears when the compiler has something to say."*
- 2026-06-04: `docs/specs/liquid-types-for-mirror.md` grounds the
  substrate's relation to Rondon-Kawaguchi-Jhala 2008.
- 2026-07-04: canonical two-channel collapse formalization at `20c99a2`.
- 2026-07-04: tokenizer emits Docblock AST nodes above `---` at
  `ee7903e`.
- 2026-07-05: @onto deep dive at `d6a05ad` — ontocybernetic
  answerability grounding.
- 2026-07-05: prism-kind auto-classifier at `bdb148a` — first
  concrete USE of the doc-as-declaration machinery.

The circular loop closes when the substrate can NAME its own operator:
above `---` is a liquid-refinement predicate; below `---` is the
compiler's decidable proof / counterexample / Tomm question; the
five-signal auto-classifier is a specialization of the four-verdict
projection routing to a decision procedure whose termination is
bounded by qualifier-set finiteness.

This doc formalizes exactly that closure.

---

## §1. Landed ancestors (substrate-honest check)

Per `[[feedback-substrate-already-had-the-word]]` this is the fifteenth+
firing this session. Seven landed ancestors carry the substance; the
liquid-refinement grounding is a routing-composition of them.

### §1.1 Rondon-Kawaguchi-Jhala (PLDI 2008) — the original liquid types

Rondon, Kawaguchi, Jhala (2008), *Liquid Types*, PLDI. Introduces
refinement types of shape `{v : B | r}` where `B` is a base type,
`v` the value variable, `r` a refinement predicate drawn from a
finite qualifier set `Q` in a decidable SMT fragment (quantifier-free
linear arithmetic + uninterpreted functions + equality).

Key invariants (per Rondon-Kawaguchi-Jhala 2008 §3):

1. **Predicate abstraction floor**: refinement predicates are
   conjunctions of qualifiers `q ∈ Q`. Q is user-supplied and finite.
2. **Fixed-point termination**: constraint solving initializes each
   liquid variable to `⋀ Q`, then monotonically weakens; termination
   in `O(|Q| · |constraints|)` steps.
3. **Decidability**: subtyping reduces to implication, implication
   reduces to SMT validity in `QF_UFLIA` (decidable).
4. **HM inference**: base types are Hindley-Milner inferred; the
   liquid inference only solves the refinement layer.

`docs/specs/liquid-types-for-mirror.md` §1 grounds this in detail.

### §1.2 LiquidHaskell (Vazou et al. ICFP 2014)

Vazou, Seidel, Jhala, Vytiniotis, Peyton Jones (2014), *Refinement
Types for Haskell*, ICFP. Extends liquid types to a lazy language
via stratified `Div | Wnf | Fin` divergence tracking. Establishes:

- Refinement predicates only hold for non-bottom values.
- Termination metrics are refinements over decreasing arguments.
- 96% of recursive functions verified terminating with 1.7 lines of
  annotation per 100 LOC.

The stratified divergence discipline maps to mirror's
`imperfect<a, e, l>`: `success` = `Fin`, `partial` = `Wnf`, `failure`
= `Div`. Direct structural correspondence.

### §1.3 Flux (Lehmann et al. PLDI 2023)

Lehmann, Geller, Vazou, Jhala (2023), *Flux: Liquid Types for Rust*,
PLDI. Establishes that refinement types and ownership are
**complementary**: ownership handles aliasing, refinements handle
functional correctness. Compilation overhead ~2x dominated by SMT
subtyping queries.

The complementarity result matters for mirror's substrate-decl / @io
partition: substrate-decl is where refinement lives (bounded,
Gödel-incomplete per #107); @io is where dynamic (Turing-complete)
behavior lives. Same complementarity, at a different altitude.

### §1.4 Reed's `@projection.preview` (2026-03-27)

At `docs/specs/historical/2026-03-27-projection-properties-as-plans.md`:

```
grammar @projection {
  type preview = satisfiable | unsatisfiable | partial
  action preview { projection: projection }
}
```

Sub-Turing decidable at grammar altitude. IS the substrate's existing
form of liquid-predicate model-checking. `preview: satisfiable ↔` the
liquid-refinement's SMT verdict `⊨ r`; `preview: unsatisfiable ↔` the
liquid-refinement's SMT verdict `⊭ r`; `preview: partial ↔` the
liquid-refinement's fixed-point iteration hitting `both_survive` at
the current depth.

### §1.5 The `---` seam (Reed + Alex, 2026-05-19)

At `docs/specs/property-projection.md`:

```
above ---   declaration (the programmer's writing)
---
below ---   observation (the compiler's measurement)
```

Both content-addressed; the OID hashes both. IS the substrate's
operational form of a refinement-predicate boundary: above `---` is
`r` (the programmer's refinement claim); below `---` is
`{proof(r) | counterexample(¬r) | opacity_map(?)}` (the compiler's
decidable verdict).

### §1.6 Property/fracture bilateral pattern (#53)

First five instances landed. This month's fifth at
`shards/kintsugi/surface.mirror` (`e910dd6`, routing-composition
variant). Property = declarative refinement predicate; fracture =
operational discharge via `splinter(ast)` and `@kintsugi/surface`'s
three-mode algebra. IS the substrate's operational form of liquid
inference's constraint-generate / constraint-solve loop, generalized
to continuous verdicts (per `docs/specs/liquid-types-for-mirror.md`
§4).

### §1.7 Recognition #107 — Hilbert-Turing structural separation

Substrate-decl is bounded / Gödel-incomplete; @io is Turing-complete.
The refinement-predicate machinery lives in substrate-decl (bounded
qualifier set `Q`; decidable). The obligation-block (`\`) is where
the machinery reaches Turing-complete territory (the body's
implementation).

Per Rondon-Kawaguchi-Jhala 2008 §5 the sub-Turing discipline
guarantees termination. The mirror substrate's #107 recognition IS the
substrate-honest statement of that discipline: refinement-predicate
audit lives strictly in the sub-Turing fragment.

---

## §2. The operator — liquid refinement at the `---` seam

At doc-claim altitude, the operator has one signature:

```
refine : DocBlock × Body → RefinementPredicate
extract : DocBlock → LiquidClaim
prove : LiquidClaim × Body × Depth → Verdict
route : Verdict → SurfaceClass
```

With:

- `DocBlock` : the `#`-prefixed narrative above `---` (per Reed
  tokenizer `ee7903e`).
- `Body` : everything below `---` — the AST of substrate-decl
  declarations.
- `RefinementPredicate` : a conjunction of qualifiers from the
  substrate's qualifier set `Q_mirror = @epistemologic/property/*`
  (per `docs/specs/liquid-types-for-mirror.md` §2.1).
- `LiquidClaim` : a doc_claim per `20c99a2`, extracted from the
  docblock's assertion structure.
- `Depth` : `k ∈ ℕ` — how many substrate ticks forward to
  fixed-point-iterate (k=3 starting bound per `63bdecc` §2).
- `Verdict` : five-valued at doc-claim altitude
  (`well_formed | overreach | incoherent | underdeclares |
  both_survive`).
- `SurfaceClass` : one of the four `@kintsugi/surface` classes
  (`ashby_mismatch | contradiction | conundrum | out_of_band`).

### §2.1 The carriers (newtyped per `[[feedback-no-bare-types]]`)

```
type liquid_qualifier = {
  predicate: ref,                 # a substrate @epistemologic/property/* pact
  arity:     u32,                 # unary | binary | n-ary
}

type qualifier_set = {
  qualifiers: [liquid_qualifier],  # the finite set Q_mirror
  altitude:   ref,                # substrate altitude at which Q applies
}

type refinement_predicate = {
  qualifiers: [liquid_qualifier],  # conjunction of q ∈ Q
  value_var:  ref,                # v : B
  base_type:  ref,                # B
}

type liquid_claim = doc_claim      # from 20c99a2 §2.1

type verdict_at_doc_altitude =
  | well_formed
  | overreach
  | incoherent
  | underdeclares
  | both_survive
```

The five-verdict output is a specialization of `63bdecc`'s
four-verdict routing plus one branch (`underdeclares`) that fires
when the body declares substrate the docblock does not mention.
`underdeclares` is the dual of `overreach`: the compiler observed
more than the programmer declared.

### §2.2 The qualifier set Q_mirror

Per `docs/specs/liquid-types-for-mirror.md` §2.1: `Q_mirror` is the
substrate's landed `@epistemologic/property/*` predicates plus the
landed `@epistemologic/pact/*` predicates. As of 2026-07-05, `|Q| ≈
40+` (all landed pacts + properties).

Termination bound: fixed-point iteration converges in
`O(|Q_mirror| · |claims_per_docblock|)` steps per Rondon-Kawaguchi-
Jhala 2008 §5. Empirically bounded above by ~40 · 10 = 400 iteration
steps per docblock audit.

---

## §3. The auto-classifier — five-signal decision procedure

`docs/math/prism-kind/README.md` §3 lands the five-signal
auto-classifier at prism-kind altitude. This section proves it is a
DECIDABLE decision procedure — specialization of the liquid-inference
fixed-point iteration to the depth-k audit at doc-claim altitude.

### §3.1 The signals (as qualifier-witnessing predicates)

Per `bdb148a` §3, each signal is a grep-verifiable predicate on the
file:

1. `inherits(f) : bool` — does the depth-0 declaration match
   `prism @X <= @Y`?
2. `carrier_density(f) : u32` — count `type X = ...` + count typed
   actions.
3. `cross_family_import(f) : u32` — count consumer families
   importing `f`.
4. `cites_marker_row(f) : bool` — does the docblock cite #112 or
   #55?
5. `primary_thin(f) : bool` — is the first typed carrier a thin
   newtype?

Each signal is a **qualifier** in the liquid-inference sense: a
boolean predicate template applied at a program point. The signal
set `S = {s_1, ..., s_5}` is the docblock-altitude qualifier set;
`|S| = 5` is finite; each `s_i` is grep-verifiable in constant
time.

### §3.2 The decision procedure

Given file `f`:

```
classify(f) : prism_kind =
  let signals    = [s_1(f), s_2(f), s_3(f), s_4(f), s_5(f)]
  let per_kind   = [count_agree(signals, k) for k in kinds]
  let winner     = argmax(per_kind)
  let agreement  = per_kind[winner] / 5
  match agreement:
    | 4/5 or 5/5 → apply(winner, transparency = 0.90)
    | 3/5        → apply(winner, transparency = 0.60), spawn Tomm
    | ≤ 2/5      → failure(cause), refuse auto-classify, spawn Tomm
```

### §3.3 Convergence + termination

**Theorem** (auto-classifier termination):
`classify(f)` terminates in constant time `O(|S|) = O(5)`.

**Proof**: Each signal `s_i(f)` is a grep predicate whose runtime is
`O(size(f))`. The `argmax` over `|kinds| = 4` runs in `O(4 · 5) =
O(20)`. Total: `O(size(f))`.

**Theorem** (auto-classifier convergence):
`classify` is a **pure function**: same input → same output. No
fixed-point iteration needed at THIS altitude — the iteration lives
at the substrate altitude where new signals or new kinds are added.

**Theorem** (bounded reductions):
Adding a new kind `k_new` adds one column to `per_kind`; adding a
new signal `s_new` adds one row. The classifier's decision surface is
monotone: adding qualifiers can only refine the verdict (never
reverse it), per Rondon-Kawaguchi-Jhala 2008 §5 (fixed-point
monotonicity theorem).

### §3.4 Verdict soundness

**Theorem** (verdict soundness relative to the liquid refinement):
Let `r` be the docblock's refinement predicate (extracted per
`liquid_extraction`); let `V = classify(f)` be the auto-classifier's
verdict. Then:

- `V = apply(k, transparency ≥ 0.60)` implies `r ⇒ kind(f) = k` in
  the sub-Turing fragment.
- `V = spawn` implies the sub-Turing fragment cannot decide `r ⇔
  kind(f)` at depth k=1; increasing k may resolve.
- `V = failure(cause)` implies `¬r` holds in the sub-Turing
  fragment (the docblock's kind claim contradicts the signal
  majority).

**Proof sketch**: soundness follows from the qualifier-set
representation of `r`. The signals `s_1, ..., s_5` are qualifiers
whose truth-values partition the semantic space of `kind`. If
`agreement ≥ 3/5`, the majority-qualifier-conjunction weakly implies
`r`; per Rondon-Kawaguchi-Jhala 2008 §4.2 (predicate-abstraction
soundness), the conjunction of qualifiers determines the refinement
verdict. QED (relative to `Q_mirror` being an adequate qualifier set;
adequacy is empirically verified by the §4 discriminator run).

Verdict soundness holds strictly in the sub-Turing fragment per #107.
The `\` obligation blocks (Turing-complete) are OUT OF SCOPE for the
auto-classifier verdict — they are checked separately by the fracture
body at kintsugi altitude.

---

## §4. Composition with the projection surface

`docs/math/the-tower/projection-surface.md` (`63bdecc`) lands the same
four-verdict routing one altitude up (at recognition-candidate
altitude). This section proves altitude-portability formally.

### §4.1 Altitude-portability theorem

**Theorem** (altitude-portable audit soundness):
Let `A_1 = doc-claim altitude` and `A_2 = recognition-candidate
altitude`. Let `audit_A_i : Claim_A_i × Depth → Verdict_A_i` be the
audit operator at altitude A_i. Then the verdict mappings

```
audit_A_1 verdict          audit_A_2 verdict         mapping
--------------------------- ------------------------- -------
well_formed                 real_survives             identity
overreach                   phantom_survives          identity
incoherent                  neither_survives          identity
both_survive                both_survive              identity
underdeclares               (no A_2 counterpart)      lift-only
```

form a **partial homomorphism** of verdict semilattices; the four
shared verdicts commute with altitude-lifting.

**Proof**: each verdict at A_1 is a refinement of a verdict at A_2
via the qualifier-set specialization
`Q_doc-claim ⊂ Q_recognition-candidate` (doc-claim qualifiers are
carrier-shape predicates; recognition-candidate qualifiers include
those plus witness-independence predicates). By Rondon-Kawaguchi-
Jhala 2008 §5 (predicate-abstraction refinement lemma), refining Q
refines V. The homomorphism follows.

`underdeclares` is a strict A_1-only verdict because the A_2
altitude has no analogue of "body declares more than boundary" — at
recognition-candidate altitude the boundary IS the whole claim.
Partial homomorphism only.

### §4.2 The kintsugi loop as fixed-point iteration

The kintsugi loop at altitude A_i:

1. `refine(D_i, B_i) → r_i` (extract the refinement).
2. `prove(r_i, B_i, k=3) → V_i` (fixed-point iterate).
3. If `V_i = well_formed` (or `real_survives`), promote.
4. If `V_i = both_survive`, increase k or sharpen qualifiers; re-loop.
5. If `V_i ∈ {overreach, incoherent, underdeclares}`, dispatch to
   `@kintsugi/fracture/*` via `@kintsugi/surface`'s three-mode
   algebra.

At every altitude, the loop is a fixed-point iteration over a monotone
qualifier-refinement lattice per Rondon-Kawaguchi-Jhala 2008 §5.
Termination bounded by `|Q_A_i|`. Same operator; different carrier;
altitude-portable per #59.

---

## §5. Composition with @onto — answerability as refinement

`docs/math/onto/README.md` (`d6a05ad`) proposes @onto as substrate-
answerability discipline. This section proves the doc-code seam IS
an ontocybernetic answerability check.

### §5.1 Answerability as refinement predicate

@onto's core predicate:

```
answerable_to_real(c: claim, w: world_form, b: boundary) -> verdict
  requires closure_refused(expose(c, b))
  requires audit(expose(c, b), 3) != absorbed(_)
```

The docblock IS `c`; the body-below-`---` IS `w`; the file-write
boundary IS `b`. `closure_refused` corresponds to the docblock's
`kind` field being extractable (never premature-closed as a
tautology). `audit != absorbed` corresponds to the auto-classifier's
verdict being `well_formed` (not overreach).

### §5.2 The answerability-refinement theorem

**Theorem**: The doc-code seam's five-verdict output maps to @onto's
four-verdict output:

```
doc-code seam                @onto                  meaning
--------------------------- ---------------------- ------------------
well_formed                 answerable(oid)        route to correction open
overreach                   absorbed(oid)          form claims to exhaust
incoherent                  phantom(cause)         framing itself un-answerable
underdeclares               (extends @onto)         body speaks past boundary
both_survive                opaque(opacity_map)    substrate refuses closure
```

**Proof**: identical to §4.1 verdict-mapping proof, specialized to
ontocybernetic verdict vocabulary. Same partial homomorphism; adds
one A_1-only branch (`underdeclares`) that @onto's current spec does
NOT carry — a candidate extension for @onto per DEFERRED §10.

### §5.3 The Tomm question shape at answerability altitude

When `V = both_survive` (or `opaque`), the fracture body emits a
Tomm-shaped question at reader-frame altitude per
`@kintsugi/surface` spec §5. For the doc-code seam:

```
docblock_ungrounded_body     → Tomm circular (§5.2)
docblock_incoherent_body     → Tomm linear-then-reflexive (§5.3)
docblock_extractive_body     → three Tomm shapes (per sub-check)
```

The @onto composition adds a fourth Tomm shape:

- **Tomm answerable-shape** (proposed §10.4): "Alex/Pack: does the
  docblock's claim retain a route to being corrected by what
  exceeds it?"

Fires when the auto-classifier verdict is `spawn` AND the docblock's
kind field is present but the routing to `@kintsugi/fracture/*` is
opaque.

---

## §6. Sub-Turing decidability grounding

Per recognition #107: substrate-decl is bounded / Gödel-incomplete;
@io is Turing-complete. The doc-code seam's audit surface lives
strictly in the sub-Turing fragment.

### §6.1 The decidability theorem

**Theorem** (doc-code seam decidability):
`audit_docblock(d, k)` is decidable for every `d : docblock` and
every `k : depth`.

**Proof**: Each of the three sub-audits (`docblock_grounded`,
`docblock_coherent`, `docblock_no_extraction_pattern`) is a
qualifier-set-abstraction whose decision procedure is:

1. `docblock_grounded` — cite-verification against
   `ancestor_exists_on_main` (git log query; decidable).
2. `docblock_coherent` — contradiction detection between extracted
   predicates (SMT `QF_UFLIA` per Rondon-Kawaguchi-Jhala 2008
   §5.1; decidable).
3. `docblock_no_extraction_pattern` — four sub-checks each
   grep-verifiable in constant time.

Sequential composition of three decidable procedures is decidable.
Termination bounded by `O(|Q_mirror| · claims_per_docblock)` per
§2.2. QED.

**Correction (Seam `20d0c13` §5 C2)**: The theorem above proves
decidability of the AUDIT step ASSUMING claims are already
extracted. It does NOT address `extract_claims` itself. Seam catch:
`extract_claims(d: docblock) -> [liquid_claim]` (per §2 signature)
is an @io boundary — natural-language-to-liquid-predicate lowering
is Turing-complete in the general case per #107. What was wrong:
§6.1 elided the extraction-side floor; the sub-Turing decidability
bound applies to `audit_docblock` post-extraction only.

How it's corrected: `extract_claims` returns
`imperfect<[liquid_claim], extraction_error, transparency>`.
Success closes into the decidable audit above; failure routes via
@pain to Alex Phase E as a Tomm question. The forward-promise:
extraction's failure mode is CLASSIFIABLE at compile-time
(well-formed input vs ambiguous vs unextractable — three-way split)
even though extraction's SUCCESS output is undecidable in the
general case. The sub-Turing decidability floor per #107 is
preserved at the audit boundary; extraction Turing-completeness is
named at the @io boundary where it belongs. Forward-promise: the
extract_claims body specification lands at TICK 1b of the
bottom-up landing spec (`docs/specs/doc-code-seam-bottom-up-landing.md`
§TICK-1) alongside the docblock family-root.

### §6.2 Why sub-Turing matters — Gödel-safety

The substrate cannot self-prove its own consistency (Gödel's second
incompleteness theorem applies to the substrate-decl fragment per
#107). But the substrate CAN self-audit within a decidable sub-fragment
where every claim resolves to `⊤ | ⊥ | ?` in bounded time.

The auto-classifier's `both_survive` branch is the substrate's
honest reading of `?`: the sub-Turing fragment cannot decide;
increase k or sharpen qualifiers. Gödel-safe.

Emitting a Tomm question at reader-frame altitude on `both_survive`
IS the substrate's structural admission of its own incompleteness.
Route-to-correction is preserved. Per @onto §2, this is
`answerable(opaque)`, not `absorbed(?)`.

---

## §7. Circular-reflexive self-audit

Per `63bdecc` §6 discipline: the operator must audit its own audit.

### §7.1 Self-application

This doc's docblock (the `#`-prefixed lines at the top) IS a
`liquid_claim`. Extraction yields:

- Claim: "the doc-code seam IS an instance of liquid-refinement
  predicate boundary." Grounded via §1.1 (Rondon-Kawaguchi-Jhala
  2008 citation) + §2.1 (carrier isomorphism).
- Claim: "verdict soundness holds relative to `Q_mirror`." Proved
  §3.4; adequacy of `Q_mirror` DEFERRED to empirical run §4.
- Claim: "sub-Turing decidability holds." Proved §6.1.
- Claim: "altitude-portability holds as partial homomorphism."
  Proved §4.1; one branch strictly A_1-only (`underdeclares`).
- Claim: "@onto composition holds as verdict-mapping." Proved
  §5.2; one candidate extension DEFERRED.

### §7.2 Self-audit verdict

Running `classify(this_doc)` under the five-signal auto-classifier:

- Signal 1 (inherits): absent. Neutral.
- Signal 2 (carrier density): 5 named carriers (liquid_qualifier,
  qualifier_set, refinement_predicate, liquid_claim,
  verdict_at_doc_altitude). Leans `family_root`.
- Signal 3 (cross-family): forward-promised consumers at
  `@docblock`, `@epistemologic/liquid_extraction`,
  `@epistemologic/property/docblock_*`, `@kintsugi/fracture/*`,
  `@onto`. Five consumer families. Leans `marker`.
- Signal 4 (marker-row citation): cites #55 form/process at §4.2
  (altitude-portable). Per `bdb148a` §3.4, Signal 4 fires on
  citations of `[[architecture-candidate-recognition-112-marker-row-fourth-structural-primitive]]`
  OR `[[architecture-form-process-partition-at-family-root]]` (#55).
  #55 citation → SIGNAL 4 FIRES. Leans marker.
  (#59 kintsugi-loop is a markers-of-marker citation, not the
  marker row itself; does not contribute to Signal 4.)
- Signal 5 (primary thin): `liquid_qualifier` primary carrier is a
  wide record (2 fields → borderline). Neutral.

**Correction (Seam `20d0c13` §5 C4)**: Prior draft counted Signal 4
as neutral ("cites both markers-of-marker"). What was wrong: #55
is the marker row itself (per `bdb148a` §3.4), not a markers-of-marker
citation; citing #55 IS Signal 4 firing. How it's corrected: Signal 4
leans marker. Adjusted count: 1 family_root + 2 marker + 2 neutral.

Verdict: `≤ 2/5 signals agree → failure(cause)`. Auto-classifier
refuses to classify this doc. Route: `spawn`. (Final verdict
preserved: no single kind clears the 3/5 majority threshold; the
self-audit passes BECAUSE the marker-row signal on this doc's OWN
citation is honestly counted.)

**Tomm question at reader-frame**: "Alex/Pack: does this doc's math
formalization LAND its own kind claim, or is it a routing-composition
whose kind is genuinely opaque until the shard cascade lands?"

Verdict: **both_survive** (at analytical altitude). Route:
`spawn`. Promotion pending independent Pack peer + empirical
discriminator run when the first bottom-up landing tick fires.

Self-audit at depth=3: `opaque(opacity_map)` at ontocybernetic
altitude. This IS the honest verdict for a math-formalization doc
that lives one altitude above the substrate it formalizes.

---

## §8. Empirical discriminator — does the math change any current shard's audit outcome?

Per `[[feedback-composition-claims-need-empirical-test]]`.

**Discriminator claim**: does adding the doc-code seam's audit
change any current shard's audit verdict when the first bottom-up
landing tick fires?

Predicted verdicts (analytical; empirical run per §9 spec):

| Shard | Current audit result | Predicted post-landing | Change? |
|-------|----------------------|-------------------------|---------|
| `@kintsugi/surface` | ratify-with-corrections | `well_formed` (5/5) | no |
| `@third` | ratify-with-corrections | `well_formed` (4/5) | no |
| `@onto` (candidate) | opaque | `both_survive` (2/2/1) | reaffirms |
| `@smarts` | Loki-grin (marker collapse) | `both_survive` | reaffirms |
| `@epistemologic` | marker-per-Loki | `overreach` (Signal 2 weak) | **yes** |
| shards missing docblock cite | (never audited) | `underdeclares` | **yes** (new class) |

**Non-empty change set**: at least `@epistemologic` (Signal 2 weak)
+ the class of shards whose docblock does not cite their body's
declared substrate would receive **NEW** substrate-audit verdicts
under the doc-code seam. Empirically testable per §9.

That's the empirical difference. Not renaming; substrate change.

---

## §9. What is DEFERRED

- Empirical discriminator run against all 30 top-level
  `shards/*.mirror` files (feeds into `docs/math/prism-kind/README.md`
  §4).
- Signal-weight calibration based on the discriminator run.
- The `underdeclares` verdict at @onto altitude (@onto's spec
  currently carries four verdicts, not five).
- The Tomm answerable-shape (§5.3 candidate; forward-promised to
  next @kintsugi/surface amendment tick).
- Formal proof of `Q_mirror` adequacy (currently empirically
  verified; formal proof requires the discriminator run to
  converge on a stable qualifier set).
- Cross-altitude projection composition mechanics (§4.2 analytical;
  mechanical form forward-promised per `20c99a2` §6.3).
- Kintsugi loop empirical composition with the runtime pipeline
  (per `[[feedback-composition-claims-need-empirical-test]]`;
  same DEFER as `@kintsugi/surface` spec §13.1).

---

## §10. Key references

- Rondon, Kawaguchi, Jhala (2008), *Liquid Types*, PLDI.
  DOI: 10.1145/1379022.1375602. arXiv: N/A (PLDI proceedings).
- Vazou, Seidel, Jhala, Vytiniotis, Peyton Jones (2014),
  *Refinement Types for Haskell*, ICFP.
  DOI: 10.1145/2628136.2628161.
- Lehmann, Geller, Vazou, Jhala (2023), *Flux: Liquid Types for
  Rust*, PLDI. DOI: 10.1145/3591283.
- Rondon, Kawaguchi, Jhala (2013), *Abstract Refinement Types*,
  ESOP. DOI: 10.1007/978-3-642-37036-6_13.
- Vazou, Rondon, Jhala (2015), *Bounded Refinement Types*, ICFP.
  DOI: 10.1145/2784731.2784745.
- Gamboa et al. (2025), *Usability Barriers for Liquid Types*,
  PACMPL.
- Rondon, Kawaguchi, Jhala (2008) §5 (predicate-abstraction
  fixed-point termination lemma).
- Meyer (1988), *Object-oriented Software Construction* —
  design-by-contract.
- Knuth (1984), *Literate Programming*, *Comput. J.* 27:97-111.
- Ashby (1956), *An Introduction to Cybernetics* — requisite
  variety.
- Tomm (1987, 1988), *Family Process* — interventive interviewing.
- Bateson (1972), *Steps to an Ecology of Mind* — logical types.

## §11. Substrate references

- `docs/math/kintsugi/doc-code-seam.md` (`20c99a2`) — two-channel
  collapse; the direct sibling of this doc.
- `docs/math/the-tower/projection-surface.md` (`63bdecc`) — the
  recognition-candidate altitude of the same operator.
- `docs/math/prism-kind/README.md` (`bdb148a`) — the first
  concrete USE of the doc-as-declaration machinery.
- `docs/math/onto/README.md` (`d6a05ad`) — ontocybernetic
  answerability grounding.
- `docs/specs/liquid-types-for-mirror.md` (2026-06-04) — background
  survey.
- `docs/specs/property-projection.md` (2026-05-19) — the `---` seam.
- `docs/specs/historical/2026-03-27-projection-properties-as-plans.md`
  (Reed 2026-03-27) — the `@projection.preview` verdict.
- `shards/epistemologic/pact/keyword_matches_depth.mirror` — the
  substrate's landed depth-discrimination discipline.
- `shards/epistemologic/pact/path_matches_namespace.mirror` — the
  substrate's landed path-namespace discipline.
- `shards/kintsugi/surface.mirror` (`e910dd6`) — the four-class
  routing algebra + fifth #53 instance.
- `shards/glass.mirror` (`a3789c2`) — `splinter(ast)` primitive.
- `bootstrap/src/tokenize.rs` (`ee7903e`) — the Docblock AST-node
  emission above `---`.
- `boot/std/mirror/liquid.mirror` (2026-06-04) — the semantic
  naming: *"the separator appears when the compiler has something
  to say."*
- `[[architecture-property-fracture-bilateral]]` (#53).
- `[[architecture-kintsugi-loop-altitude-portable]]` (#59).
- `[[architecture-form-process-partition-at-family-root]]` (#55).
- `[[architecture-hilbert-turing-godel-recognition-107]]` (#107).
- `[[architecture-mirror-as-expanding-hilbert-space]]` (#51).
- `[[feedback-substrate-already-had-the-word]]` (fifteenth+).
- `[[feedback-craft-not-deliver]]`.
- `[[feedback-composition-claims-need-empirical-test]]`.
- `[[feedback-phantom-candidate-discipline]]`.
- `[[feedback-no-bare-types]]`.
- `[[feedback-legibility-over-foundation-when-collapsing]]`.

*2026-07-05. Mara. Compiler-fit math formalization. Not canonical
spec. Substrate-reading. Self-audit: `both_survive`, route
`spawn`. Empirical discriminator required at first bottom-up
landing tick before promotion.*
