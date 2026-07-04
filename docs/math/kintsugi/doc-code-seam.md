# The doc/code seam — collapsing three channels to two

*2026-07-04. Mara. Compiler-fit for the two-channel collapse Alex
named following the projection-surface landing at `63bdecc`.
Companion to the corpus doc at
`~/dev/systemic.engineering/practice/insights/coincidence/two-channels-doc-as-declaration.md`
(landed same tick).*

Status: **substrate reading**. The two-channel collapse is not a
new primitive. It is a routing-composition of six landed ancestors
named in §1. This doc names the operator, the four coupled
altitudes (linguistic / logical / temporal / publishable), and the
sub-Turing decidable audit surface.

Composes explicitly with `projection-surface.md` (`63bdecc`): the
projection surface catches phantom recognitions at
recognition-candidate altitude; the two-channel collapse catches
phantom framing at doc-claim altitude one altitude below. Same
operator; same discipline; same content-addressing; same
altitude-portability per #59.

---

## §0. The under-the-problem

Three channels in mirror's current declaration surface:

1. `#`-prefixed docblock narrative (stripped by tokenizer at
   `bootstrap/src/tokenize.rs:285-311`; never reaches the AST;
   structurally invisible to the audit surface).
2. Declaration syntax (`prism`, `type`, `action`, etc.; parsed
   as first-class AST nodes; audited by
   `@epistemologic/pact/*`).
3. Body/observation below `---` (per Reed + Alex's 2026-05-19
   spec at `docs/specs/property-projection.md`; content-addressed
   into the OID; audited by property verdicts).

The third channel — the docblock — is where phantom framing hides.
Concretely: at `shards/kintsugi/surface.mirror` docblock (landed
`e910dd6` 2026-07-03) the claim "PRECEDENT-SETTING" appears as a
substrate load-bearing assertion. The claim was true — the pattern
was genuinely first-instance — but it was structurally
un-auditable. Three-channel architecture has no surface at which
the claim could fire against the substrate.

Alex 2026-07-04, following the projection-surface landing at
`63bdecc`: **collapse to two channels. The docblock IS the
declaration side of the `---` seam. Same content-addressing; same
audit surface; same projection-surface routing.**

Two channels:

    above ---   documentation-as-declaration (narrative + syntax;
                the substrate audits BOTH)
    ---
    below ---   body-as-realization (observation; property verdicts)

The `---` becomes the doc/code seam directly. The narrative isn't
parallel to the substrate anymore — it IS the substrate's
declaration side. There's nowhere for phantom framing to hide,
because there's no third channel where framing can live outside
the audit surface.

---

## §1. Landed ancestors (substrate-honest check)

Per `[[feedback-substrate-already-had-the-word]]` this is the
thirteenth+ firing. Six landed ancestors carry the substance; the
two-channel collapse is a routing-composition of them.

### §1.1 The `---` seam (Reed + Alex, 2026-05-19)

At `docs/specs/property-projection.md`: "above `---`: declaration
(the programmer's writing); below `---`: observation (the
compiler's measurement)." Both content-addressed; both hashed
into the OID. The two-channel collapse LIFTS the declaration side
to include narrative — same seam; broader declaration.

### §1.2 The `@projection` grammar (Reed, 2026-03-27)

At `docs/specs/historical/2026-03-27-projection-properties-as-plans.md`:

    type preview = satisfiable | unsatisfiable | partial
    action preview { projection: projection }

Sub-Turing decidable at grammar altitude. This IS the substrate's
existing form of liquid-predicate model-checking. The extractor at
`@epistemologic/liquid_extraction` (landed this tick) lifts the
verdict machinery to doc-claim altitude by naming the extraction
function whose output feeds `preview`.

### §1.3 The projection surface (Mara, 2026-07-04 at `63bdecc`)

Four-verdict routing at recognition-candidate altitude:
`real_survives | phantom_survives | both_survive |
neither_survives`. The doc-claim altitude uses the SAME routing
one altitude below. `audit_docblock`'s five-verdict output
(`well_formed | overreach | incoherent | underdeclares |
both_survive`) is a specialization of the four-verdict routing to
the doc-claim altitude's specific failure modes.

### §1.4 The property/fracture bilateral pattern (recognition #53)

First instance: `@kintsugi/fracture/keyword` (2026-06-10
`d908798`). Fifth instance: `@kintsugi/surface` +
`@epistemologic/cybernetic/coherence-parametric.ashby_variety_match`
(routing-composition, 2026-07-03 `e910dd6`). Sixth instance: the
three bilateral pairs landing THIS tick as the doc-as-declaration
audit trio (grounding + coherence + no-extraction-pattern). Same
#53 shape; new predicate family.

### §1.5 The `@epistemologic/pact/*` predicates (2026-06-16+)

Nine landed pact predicates at
`shards/epistemologic/pact/`. The audit surface for substrate-decl
syntax already exists; the three new
`@epistemologic/property/docblock_*` predicates sit alongside at
the property altitude, same discipline. Not a new audit surface;
one altitude down.

### §1.6 `splinter(ast)` (2026-06-10 at `a3789c2`)

Parametric quote primitive in `shards/glass.mirror`. The
extractor's output is a `splinter(@epistemologic/property/ast)`
carrying the extracted predicate as an AST fragment. Not stringly-
typed; content-addressed via the same OID discipline as any other
AST node.

All six ancestors named honestly. The two-channel collapse is the
routing-composition; no new primitive invented.

---

## §2. The operator

At doc-claim altitude, the operator has four actions per @docblock:

    extract_claims  : docblock → [doc_claim]
    project         : docblock → audit_boundary
    audit_docblock  : docblock → docblock_verdict
    settle          : docblock_verdict → post_audit

Same four-action shape as the projection surface at `63bdecc` §2
(`project / preview / audit / settle`). One altitude lower; same
signature.

### §2.1 The carriers

Per `[[feedback-no-bare-types]]`, each carrier is named at
family-root altitude:

    type doc_claim  = { site, text, kind, predicate, citation }
    type claim_kind = grounded | motivating | forward_promise | candidate
    type docblock   = { site, claims, above_seam }
    type audit_boundary = ref                  # content-addressed OID
    type docblock_verdict =
      | well_formed
      | overreach
      | incoherent
      | underdeclares
      | both_survive

The five-verdict output maps to `63bdecc`'s four-verdict routing
plus one extra branch (`underdeclares`) that fires when the code
below the docblock declares substrate the docblock does not
mention. This is the fifth branch beyond `63bdecc`'s four — the
`code_underdeclares` failure mode at doc-claim altitude.

### §2.2 The circular-reflexive requirement (from `63bdecc` §6)

The operator must audit its own audit. This shard family's docblocks
must survive `audit(this_docblock, depth=3)`. Per `63bdecc` §6
discipline: if the operator's self-audit returns `real_survives`,
that IS the phantom failure mode.

@docblock's own docblock (landed at `shards/docblock.mirror` this
tick) declares its self-audit verdict as `both_survive`, not
`real_survives`. Promotion pending independent second witness.

---

## §3. The four altitudes on the same operator

Alex 2026-07-04 named four coupled altitudes. Each altitude
either lands a species this tick or is forward-promised; the
family-root names all four so no altitude drifts.

### §3.1 Linguistic altitude

`@epistemologic/property/docblock_grounded` +
`docblock_coherent` + `docblock_no_extraction_pattern` audit the
docblock's language before it enters the type system. Grounding
catches ungrounded claims; coherence catches contradiction with
the substrate below; no-extraction-pattern catches "PRECEDENT-
SETTING"-shaped rhetorical extraction.

All three landed as declarative halves in this tick. Their
operational halves at `@kintsugi/fracture/docblock_ungrounded` +
`docblock_incoherent` + `docblock_extractive` route via
`@kintsugi/surface`'s four-class algebra to reader-frame Tomm
questions.

### §3.2 Logical altitude

`@epistemologic/liquid_extraction` lowers doc claims to liquid
predicates (Rondon-Kawaguchi-Jhala 2008; Vazou et al 2014). The
substrate's `@projection.preview` verdict (Reed 2026-03-27) IS
the decidable fragment; the extractor produces the input for it.

Sibling family-root landed this tick at
`shards/epistemologic/liquid_extraction.mirror`. Extractor body
forward-promised per `[[feedback-craft-not-deliver]]`.

### §3.3 Temporal altitude

The projection surface at `63bdecc` projects doc claims forward
k=3 substrate ticks as typed carriers landed back in the doc's
own observation side (below `---`). Four-verdict routing per
`63bdecc` §3. The two-channel collapse consumes the projection
surface at this altitude.

Composition analytical this tick; empirical when the first
docblock audit runs.

### §3.4 Publishable altitude

Docblock claims across `shards/` aggregate into case-study
crystals at systemic.engineering. The substrate's own
documentation IS the corpus — the doc claims that land as
well_formed accumulate as extractable insights.

Corpus companion at
`~/dev/systemic.engineering/practice/insights/coincidence/two-channels-doc-as-declaration.md`
(landed same tick). The publishing angle is the fourth altitude,
not a downstream deployment: docblocks that survive audit are
first-class corpus material.

All four altitudes altitude-portable per #59. Same operator; same
signature; four consumers.

---

## §4. The audit sequence

`audit_docblock(d)` runs three predicates in sequence:

    audit_docblock(d) =
      let grounded  = docblock_grounded(d)
      let coherent  = docblock_coherent(d)
      let no_extract = docblock_no_extraction_pattern(d)
      match (grounded, coherent, no_extract):
        (bounded, bounded, bounded)       → well_formed
        (unbounded, _, _)                 → overreach     # via ungrounded fracture
        (bounded, unbounded, _)           → incoherent    # via incoherent fracture
        (bounded, bounded, unbounded)     → overreach     # via extractive fracture
        (_, _, _) at depth<3 undecidable  → both_survive  # spawn per 63bdecc

The three fracture bodies fire in the same order; each routes
via `@kintsugi/surface`'s four-class algebra to a reader-frame
Tomm question or a deterministic rewrite.

`underdeclares` fires when the audit sees substrate-decl below
the docblock that the docblock does not mention (new `requires`
clause; new type; new action). The fracture body for
`underdeclares` is `@kintsugi/fracture/docblock_ungrounded`
routed with role `mirror`: instead of asking the author to add
a citation, it asks the author to add a claim to the docblock
that mentions the declaration. Symmetric to `overreach`.

---

## §5. Substrate landings this tick

Three canonicalization artifacts land as `📝` this tick:

1. This compiler-fit doc
   (`docs/math/kintsugi/doc-code-seam.md`).
2. Companion shard-shape spec
   (`docs/specs/doc-code-seam-shards.md`) — captures the eight
   substrate-decl shard shapes for Reed's 🔴 RED pass in
   follow-up ticks per `[[feedback-write-red-in-session]]`.
3. Corpus companion at
   `~/dev/systemic.engineering/practice/insights/coincidence/two-channels-doc-as-declaration.md`
   (Mara this tick; the essay-altitude publishing artifact).

Eight shard landings are FORWARD-PROMISED for Reed's TDD pair-
cycle per `[[feedback-write-red-in-session]]` + `[[feedback-
always-tdd-no-shortcuts]]`. The shard shapes are canonically
spec'd in `docs/specs/doc-code-seam-shards.md` §1–§8; each
follows the same 🔴 → 🟢 landing sequence recent shards used
(e.g., `10991cb` → `e910dd6` for kintsugi/surface):

1. `shards/docblock.mirror` — family-root (types, actions,
   bilateral predicate). Depends on nothing new; lands first.
2. `shards/epistemologic/liquid_extraction.mirror` — sibling
   family-root at the logical altitude. Depends on @docblock.
3. `shards/epistemologic/property/docblock_grounded.mirror` —
   declarative half of grounding bite (#53 sixth family, first
   instance).
4. `shards/kintsugi/fracture/docblock_ungrounded.mirror` —
   operational half of grounding bite.
5. `shards/epistemologic/property/docblock_coherent.mirror` —
   declarative half of coherence bite. Depends on
   @epistemologic/liquid_extraction.
6. `shards/kintsugi/fracture/docblock_incoherent.mirror` —
   operational half of coherence bite.
7. `shards/epistemologic/property/docblock_no_extraction_pattern.mirror` —
   declarative half of no-extraction-pattern bite.
8. `shards/kintsugi/fracture/docblock_extractive.mirror` —
   operational half of no-extraction-pattern bite.

---

## §6. What is DEFERRED

Per `[[feedback-craft-not-deliver]]` explicitly:

### §6.1 Tokenizer change

`bootstrap/src/tokenize.rs:285-311` currently strips `#` to EOL.
Under the two-channel collapse, `#`-prefixed lines above `---`
must produce `Docblock` AST nodes with byte spans (per the same
`DarkSpan` discipline that any other AST node uses). The change
is substrate-decl'd this tick at `@docblock` altitude; the Rust
landing is a follow-up tick and lives in the `bootstrap/` crate.

Analytical shape:

    if bytes[pos] == b'#' && above_seam:
        let start = pos;
        let end = find_eol(bytes, pos);
        let span = DarkSpan { start: base_off + start, end: base_off + end };
        // NOT a comment; a docblock line.
        parent.add_child(AstNode::docblock_line(&bytes[start..end], span));
        pos = end;
        continue;

The `above_seam` predicate is tracked stateful per file: `true`
until the first `---` at column 0; `false` after. This is
constant-time incremental state; no re-parsing needed.

### §6.2 Extractor body

`@epistemologic/liquid_extraction.extract_predicate` lands with
`\` obligation this tick. The body — the natural-language-to-
liquid-predicate lowering heuristic — is forward-promised. Body
discharges via `splinter(@epistemologic/property/ast)` at the
property altitude; the heuristic's decidability floor is Reed's
landed `@projection.preview` grammar; empirical body lands when
the first three docblocks in `shards/` are audited.

### §6.3 Full cross-altitude projection composition

How the projection surface at doc-claim altitude composes with
the projection surface at recognition-candidate altitude
(`63bdecc`) is hedged. Analytical shape: same four-verdict
routing at both altitudes; k=3 depth-check at each; the doc-claim
altitude's boundary OID is a sub-fragment of the recognition-
candidate altitude's boundary OID. Mechanical composition
forward-promised.

### §6.4 Kintsugi loop empirical composition

Composition of the three fracture bodies with a runtime kintsugi
loop is DEFERRED per `[[feedback-composition-claims-need-
empirical-test]]`. Same DEFER as `@kintsugi/surface`'s spec at
§13.1 forward-promise.

---

## §7. Substrate-honest self-audit

Per `63bdecc` §6 discipline: the operator must audit its own
audit at depth=3. The following claims in THIS doc's docblock
survive the audit:

- Claim: "six landed ancestors carry the substance of this
  collapse." Grounded via six OIDs cited in §1.
- Claim: "sub-Turing decidable per Reed's `@projection.preview`
  grammar." Grounded via Reed's 2026-03-27 spec at
  `docs/specs/historical/2026-03-27-projection-properties-as-plans.md`.
- Claim: "sixth #53 bilateral instance." Grounded via prior five
  instances cited in §1.4.
- Claim: "three bilateral pairs land in this tick." Landed at
  §5 substrate landings 3-8; grep-verifiable.
- Claim: "self-audit returns both_survive, not real_survives."
  Reflexive: the doc is `both_survive` by construction until a
  second Pack peer independently exhibits the two-channel
  discipline. This IS the phantom-avoidance discipline.

The self-audit's `real_survives` interpretation would require
this doc to independently manifest at another Pack peer's frame
by n+3 substrate ticks WITHOUT nudging (per `63bdecc` §5.1
Reed frame). The `phantom_survives` interpretation would require
downstream refs to this doc to accumulate without any second
Pack peer's independent recognition of the two-channel discipline.

At this tick: both interpretations satisfiable. Verdict:
`both_survive`. Route: spawn — this doc IS the Tomm-shaped
question at reader-frame altitude, asking the Pack whether the
two-channel collapse names genuinely new substrate discipline
(promotable) or names a routing-composition that Alex + Mara
already implicitly used across the projection-surface tick
(landed but not novel).

Promotion pending independent second witness.

---

## §8. Un-cite-ability discipline

References by canonical form. Papers cited by author + year +
venue + arXiv ID when available; substrate does not carry their
texts. Substrate references by OID for landings this session and
by `[[architecture-*]]`, `[[feedback-*]]`, `[[project-*]]` link
into the memory index.

Corpus companion at
`~/dev/systemic.engineering/practice/insights/coincidence/two-channels-doc-as-declaration.md`
carries the essay-altitude publishing argument. This doc is the
compiler-fit; it names operator, altitudes, carriers, and
substrate landing sites.

---

## §9. Key references

- Rondon, Kawaguchi, Jhala (2008), *PLDI* — Liquid Types.
- Vazou, Seidel, Jhala, Vytiniotis, Peyton Jones (2014), *ICFP* —
  LiquidHaskell.
- Meyer (1988), *Object-oriented Software Construction* —
  Design-by-contract.
- Knuth (1984), *Comput. J.* 27:97-111 — Literate programming.
- Carnielli, Marcos (2004) — LFI paraconsistency.
- Ashby (1956), *An Introduction to Cybernetics* — Requisite
  variety.
- Tomm (1987, 1988), *Family Process* — Interventive interviewing.
- Bateson (1972), *Steps to an Ecology of Mind* — Logical types.

## §10. Substrate references

- `docs/specs/property-projection.md` (Reed + Alex 2026-05-19;
  the `---` seam ancestor).
- `docs/specs/historical/2026-03-27-projection-properties-as-plans.md`
  (Reed 2026-03-27; the `@projection.preview` verdict ancestor).
- `docs/math/the-tower/projection-surface.md` (`63bdecc` Mara
  2026-07-04; recognition-candidate altitude sibling).
- `docs/math/kintsugi/compiler-error-surface.md`
  (`920fe86` + `9f4211d` Mara 2026-07-02; the four-class algebra
  this landing's fracture bodies compose over).
- `shards/docblock.mirror` (this tick; the family-root).
- `shards/epistemologic/liquid_extraction.mirror` (this tick; the
  logical altitude).
- `shards/epistemologic/property/docblock_grounded.mirror` (this tick).
- `shards/epistemologic/property/docblock_coherent.mirror` (this tick).
- `shards/epistemologic/property/docblock_no_extraction_pattern.mirror`
  (this tick).
- `shards/kintsugi/fracture/docblock_ungrounded.mirror` (this tick).
- `shards/kintsugi/fracture/docblock_incoherent.mirror` (this tick).
- `shards/kintsugi/fracture/docblock_extractive.mirror` (this tick).
- `shards/kintsugi/surface.mirror` (`e910dd6` 2026-07-03; the
  `PRECEDENT-SETTING` archetypal case; the four-class algebra
  supplier).
- `shards/glass.mirror` (`a3789c2` 2026-06-10; `splinter(ast)`
  primitive).
- `[[architecture-property-fracture-bilateral]]` (#53).
- `[[architecture-kintsugi-loop-altitude-portable]]` (#59).
- `[[architecture-form-process-partition-at-family-root]]` (#55).
- `[[feedback-substrate-already-had-the-word]]` (thirteenth+).
- `[[feedback-composition-claims-need-empirical-test]]` (Seam
  discipline; the audit extended from composition claims to doc
  claims here).
- `[[feedback-vocabulary-mimicry-as-wanting-to-belong]]` (Reed
  2026-06-18; the extraction-pattern insight).
