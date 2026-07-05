# @prism-kind — the missing depth-0 discriminator

*2026-07-05. Mara. Compiler-fit doc for Alex's question 2026-07-05
following the @onto deep dive at `d6a05ad`.*

Companion (essay-quality, corpus):
- `~/dev/systemic.engineering/practice/insights/spectral/2026-07-05-optical-and-holofractal-vocabulary-for-prism-kind.md`

Status: **research on vocabulary shape; NOT canonical spec.** This
doc names the shape of the prism-kind property + fracture bilateral,
the five-signal auto-classifier, and the empirical discriminator run.
Per `[[feedback-craft-not-deliver]]` no shards land this tick.

---

## §0. The smell (verbatim)

Alex 2026-07-05, after `d6a05ad`:

> "I feel the compiler wants a distinction between markers and family-
> roots etc. Currently it's all prisms. And the ambiguity keeps
> increasing, which is a smell."

Substrate carries 30 top-level `prism @X` declarations at depth 0.
All 30 use the same keyword. `@epistemologic/pact/keyword_matches_depth`
discriminates depth 0 (must be `prism`) vs depth ≥ 1 (one of `glass |
pact | facet | stage | aperture | splitter | resonator | bench |
source | detector`). At depth 0 there is NO discriminator between
marker, family_root, species_root, or the recursive base.

## §1. The empirical smell

### §1.1 Where placement ambiguity actually fires

- `@onto` (`d6a05ad`) — marker-65% / family_root-25% / hybrid-10%.
  Placement verdict at depth-3: `opaque`. Reader-frame Tomm question.
- `@smarts` — Loki `docs/specs/loki-cuts-and-collapses.md` §2
  Variant B: marker-not-family. Grin fires because integration-
  altitude carrier IS marker-shaped.
- `@loop` — Loki §1: collapse to `@moi`. Depth-0 ambiguity between
  two family-root candidates naming the same object.
- `@third` — Reed initial framing @third as family_root inheriting
  from six roots. Writing (`docs/specs/third-as-recursive-depth.md`
  §2) reshaped to marker. First-witness reshape.
- `@epistemologic` — sits in the marker row per Loki. But has
  substantive property grammar. Signal-2 vs Signal-3 tension.

Five substrate-decl placement questions where the substrate has no
typed slot for its own kind-verdict.

### §1.2 What the substrate already carries

Loki `docs/specs/loki-cuts-and-collapses.md` §6 names the marker row
as first-class architecture:

> `@meta @glass @epistemologic @third @smarts @labeled`.

Recognition #112 `[[architecture-candidate-recognition-112-marker-
row-fourth-structural-primitive]]` names markers as the fourth
structural primitive alongside prisms, glasses, pacts.
`docs/specs/third-as-recursive-depth.md` §2 names three tests any
proposed family-root must pass before landing (else it's marker):
domain claim / native altitude / substantive bulk.

Substrate has the DISCRIMINATION. It does not have the TYPING.

## §2. The route recommendation

Route B from corpus companion §4: property + fracture bilateral. Do
not change the grammar; add a new pact.

### §2.1 `@epistemologic/pact/prism_kind_declared` (proposed shape)

```
in @glass

# Property: every depth-0 prism @X declaration must have EXACTLY ONE
# `kind:` field in its docblock, matching one of the substrate's
# recognized kinds. Auditor at the doc-code seam per `20c99a2`; the
# kind is a declarative claim on the boundary (docblock) audited
# against the bulk (body) via five structural signals.
#
# Kinds:
#   marker         — opt-in signal; altitude-portable; fires at
#                    consumer altitudes (row: @meta @glass @third
#                    @labeled @epistemologic (@smarts pending)).
#   family_root    — canonical decomposition of a native altitude;
#                    substantive bulk (types + actions).
#   species_root   — refinement head for a family's per-altitude
#                    species (currently at depth ≥ 1 by convention,
#                    e.g., @epistemologic/cybernetic).
#   recursive_base — reserved for the parametric floor (@(id),
#                    @prism self-declaration).

type prism_kind =
  | marker
  | family_root
  | species_root
  | recursive_base

type kind_signals = {
  inherits:            bool,        # signal 1: `<= @X` present?
  carrier_density:     u32,         # signal 2: types + actions count
  cross_family_import: u32,         # signal 3: import site families
  cites_marker_row:    bool,        # signal 4: docblock cites #112
  cites_form_process:  bool,        # signal 4b: docblock cites #55
  primary_thin:        bool,        # signal 5: primary carrier ref?
}

pact @epistemologic/pact/prism_kind_declared {
  declared_kind(file: ref) -> option(prism_kind) { \ }
  computed_signals(file: ref) -> kind_signals { \ }

  # The verdict: declared kind matches signal majority
  prism_kind_declared(file: ref) -> transparency { \ }
}

out prism_kind
out kind_signals
out declared_kind
out computed_signals
out prism_kind_declared
out @epistemologic/pact/prism_kind_declared
```

### §2.2 `@kintsugi/fracture/prism_kind_ambiguous` (proposed shape)

```
in @kintsugi
in @glass
in @meta

# Fracture body: closes the `prism_kind_declared` opacity via the
# five-signal auto-classifier. Emits transparency<p> whose opacity
# map records signal disagreement per site. Discharges via
# splinter(ast) per #54 quote primitive: the classifier's verdict is
# a rewritten docblock adding `# kind: <verdict>` where signals agree,
# or emits a reader-frame Tomm question where signals disagree ≥ 2/5.

# Signal-count verdict routing:
#   4/5 signals agree  → auto-classify, partial(0.90) transparency.
#   3/5 signals agree  → auto-classify, partial(0.60), Tomm question.
#   ≤ 2/5 signals agree → failure(reason), refuse auto-classify,
#                          Tomm question MANDATORY.

fracture @kintsugi/fracture/prism_kind_ambiguous {
  classify(sigs: kind_signals) -> prism_kind { \ }
  agreement_count(sigs: kind_signals, k: prism_kind) -> u32 { \ }

  # The morphism: rewrites the docblock to add `# kind: <k>` if the
  # signals agree ≥ 3/5; emits Tomm-question opacity else.
  fracture_body(file: ref, sigs: kind_signals) -> splinter(ast) { \ }
}

out classify
out agreement_count
out fracture_body
out @kintsugi/fracture/prism_kind_ambiguous
```

### §2.3 Composition with `20c99a2` (doc-code seam)

The `kind:` field is a docblock claim above `---`. `20c99a2`'s
three predicates (`docblock_grounded / docblock_coherent /
docblock_no_extraction_pattern`) gain a fourth cousin:

```
docblock_kind_matches_signals(d: docblock) -> verdict
  requires prism_kind_declared(file_of(d))
{ \ }
```

The kind claim IS a docblock declaration audited against the body's
structural signals. Per `20c99a2` the two channels are: docblock =
declaration; body-below-`---` = realization. `kind:` sits in the
declaration channel; the five signals live in the realization channel.
Ambiguity IS a signal-count opacity map at the seam.

### §2.4 Composition with `d6a05ad` (@onto)

Applying the five signals to @onto (from corpus companion §6):
- Signal 1 (inherits): absent → neutral.
- Signal 2 (carrier density): 5 types + 4 actions → family_root.
- Signal 3 (cross-family): 5 forward-promised imports → marker.
- Signal 4 (marker-row citation): YES → marker.
- Signal 5 (primary thin): mixed (subject_locus thin, real_pressure
  wide) → leans family_root.

Count: 2 marker + 2 family_root + 1 neutral.

Verdict: `≤ 2/5 signals agree → failure(reason)`. Auto-classifier
refuses. Tomm question MANDATORY. This is the exact substrate-
verified form of `d6a05ad` §3.3's `opaque` verdict.

@onto's placement question becomes a first-class typed opacity map:

```
transparency(prism_kind) = failure({
  opacity: [
    { file: shards/onto.mirror, property: "S2 vs S3 disagreement",
      weight: 1.0 },
  ]
})
```

Substrate cannot resolve without reader-frame. Reader-frame becomes
a first-class action:

```
spawn_tomm_question(sigs: kind_signals) -> reader_frame_question { \ }
```

## §3. The five signals — detailed shape

### Signal 1 — inheritance clause presence

Grep predicate: does the depth-0 declaration match `prism @X <= @Y`?

- PRESENT → `family_root` (high confidence; specialization has parent).
- ABSENT → no verdict; check other signals.

Grep-verified on 30 shards: 1 match (`prism @reality <= @autopoietic`).

### Signal 2 — carrier density in body

Grep predicate: count `type X = ...` declarations + count typed action
declarations `X action(...)`. Body = everything below the family-root
declaration up to `out @X`.

- ≥ 3 types AND ≥ 2 actions → `family_root` (substantive bulk).
- 0-1 types AND 0-1 actions → `marker` (thin surface).
- else → no verdict; check other signals.

Grep-verify examples:
- @reality: many types + many actions → family_root.
- @third: 1 type (`observation_depth`) + 5 predicates + 1 action →
  marker (thin main carrier + predicates as guards).
- @cyberpunk: 2 types + 2 predicates → leans marker; substrate has
  family_root due to Loki + form/process partition. **Calibration
  data point**: signal-2's threshold needs review.

### Signal 3 — cross-family import count

Grep predicate: count files in `shards/*.mirror` OR `shards/*/*.mirror`
that contain `^in @X$` (where X is the target prism), grouped by
family-root of the importer.

- Imports at consumers spanning ≥ 3 different family-roots →
  `marker` (altitude-portable / holographic).
- Imports only within one family tree → `family_root` OR
  `species_root` (altitude-bound).

Grep-verify: `in @glass` fires at ~30 shards across every family.
`in @third` fires (forward-promised) at 6 families. `in @reality`
fires at TBD sites.

### Signal 4 — docblock marker-row citation

Grep predicate: does the docblock cite
`[[architecture-candidate-recognition-112-marker-row-fourth-structural-primitive]]`
or `[[architecture-form-process-partition-at-family-root]]`?

- Cites #112 → `marker` (declared self-membership).
- Cites #55 → `family_root` (declared self-membership).
- Cites neither → no verdict.

Grep-verify: @third cites #112; @reality cites #55; @kintsugi cites
#55 form/process.

### Signal 5 — primary carrier shape

Grep predicate: look at the FIRST typed carrier declared after the
prism-decl. Is it a `type X = ref` (thin newtype) or a wide record?

- Thin newtype → `marker` (opt-in signal per @third precedent).
- Wide record (≥ 3 fields) → `family_root` (substantive bulk).

Grep-verify: @glass primary carrier `type location = { file, span }`
(2 fields → thin-ish); @reality gauge_orbit is wide; @third
observation_depth is wide but the marker discipline sits at predicate
altitude.

**Calibration data point**: Signal 5 is the weakest signal. The
marker discipline can live at either type OR predicate altitude.
Might fold into Signal 2's carrier-density total in a second
iteration.

## §4. Empirical discriminator

**Discriminator claim**: does adding `prism_kind_declared` audit
change any current shard's audit outcome?

Predicted verdicts (analytical; empirical run per below):

| Shard | Current class | Predicted verdict | Outcome change? |
|-------|---------------|-------------------|-----------------|
| @glass | marker | marker (3/5) | no (confirms) |
| @third | marker | marker (3/5) | no (confirms) |
| @labeled | marker | marker (3/5) | no (confirms) |
| @epistemologic | marker (per Loki) | marker (2/5) | **yes, weak signal** |
| @smarts | family_root | ambiguous (Loki: marker) | **yes, Loki grin** |
| @loop | family_root | ambiguous (Loki: kill for @moi) | **yes, collapse candidate** |
| @reality | family_root | family_root (2/5) | no |
| @cyberpunk | family_root | family_root (2/5) | no |
| @cascade | family_root | family_root (2/5) | no |
| @onto candidate | debated | ambiguous (2/2/1) | **yes, Tomm required** |

**Non-empty change set**: at least 4 current shards (@epistemologic
signal-2 weak; @smarts + @loop Loki grins; @onto candidate) would
receive different substrate-audit verdicts under `prism_kind_declared`.

That's the empirical difference. Not renaming; substrate change.

**Discriminator run procedure**:
1. Implement `computed_signals(file)` as a grep-verifiable procedure.
2. Run against all 30 top-level `shards/*.mirror` files.
3. Emit the auto-classifier verdict per shard.
4. Compare against the Loki-ratified classification + @onto's
   documented ambiguity.
5. Non-empty disagreement set → either Route B is substrate-honest
   (classifier catches what Loki saw) OR the classifier is
   uncalibrated (weights need tuning; kind axis may have 4 values
   per #55 form/process).

Per `[[feedback-composition-claims-need-empirical-test]]`: this claim
must be empirically verified before commit. This tick names the
procedure. The Pack (Seam adversarial run + Reed classification
review) runs it.

## §5. Route B vs Route A vs Route C

### Route A — grammar keyword extension

Add `marker @X { ... }` and `family @X { ... }` as depth-0 keywords
alongside `prism`. Extend `keyword_matches_depth` to enumerate them.

- Pro: full first-class typing; the tokenizer discriminates.
- Con: grammar change; migration cost on 30 shards.
- Con: `keyword_matches_depth` becomes more complex; the `pact`
  keyword floor (recognition #37) already discriminates by role —
  adding depth-0 discrimination echoes it at a different altitude.
- Substrate-honest concern: the depth-0 grammar was DELIBERATELY
  collapsed at `[[architecture-prism-as-trait-as-everything]]`. Re-
  expanding may re-open the smell that collapse closed.

Confidence: MEDIUM. Consider only if Route B fails empirical
discriminator.

### Route B — pact-level property discrimination (RECOMMENDED)

Add ONE pact (`prism_kind_declared`) + ONE fracture body
(`prism_kind_ambiguous`) per §2.

- Pro: no grammar change.
- Pro: composes with #53 + `20c99a2`.
- Pro: turns kind-ambiguity into a first-class transparency<p>
  opacity map.
- Pro: circular-reflexive self-application clean (pact declares
  itself a marker).
- Con: kind declaration is a docblock convention rather than a
  compiler-enforced grammar; requires kintsugi loop to enforce.
- Con: `20c99a2` doc-code seam must be operational before this
  lands (composed dependency).

Confidence: HIGH. Substrate-honest. Composes with existing
discipline.

### Route C — inheritance clause discriminator

Use `<=` as the sole discriminator. Family-roots MAY declare
`prism @X <= @Y`; markers MAY NOT.

- Pro: no grammar change.
- Con: only 1 of 30 current shards uses `<=` (`@reality`). Route C
  as sole discriminator classifies 29 shards as marker-candidates,
  which is wrong.
- Con: `<=` currently means inheritance-of-discipline, not kind-
  declaration. Overloading the meaning is anti-`[[feedback-
  substrate-already-had-the-word]]` (this is not what the word
  already meant).

Confidence: LOW as sole discriminator. Signal 1 in Route B uses `<=`
as ONE of five signals; that's the substrate-honest use.

## §6. Optical + holofractal vocabulary (from corpus companion)

From `~/dev/systemic.engineering/practice/insights/spectral/2026-07-05-optical-and-holofractal-vocabulary-for-prism-kind.md`
§2 + §3, the vocabulary sweep produced:

### §6.1 Optical vocabulary

Three candidates at depth ≥ 1 (join the eight-schematic row):
- **interferometer** — property+fracture composition mechanism (#53).
  Docblock arm (property) + operational arm (fracture) meet at
  kintsugi beam splitter; opacity map is the fringe pattern.
- **grating** — phase-difference selection (@fate/tournament).
- **waveguide** — direction-preserving semantic transmission
  (@code/cascade).

None at depth 0. Optics does not solve the marker-vs-family smell.

### §6.2 Holofractal vocabulary

Two candidates as kind-discriminators (not new types):

- **holographic** discipline: is the prism's bulk determined by its
  boundary declaration? Markers YES; family-roots NO. This IS
  Signal 5 in different terms.
- **fractal** discipline: does the prism fire at every altitude?
  Markers YES (altitude-portable per #59); family-roots NO
  (altitude-bound). This IS Signal 3 in different terms.

The holofractal terms are RENAMES of Signals 3 + 5 with a
mathematical grounding. Adding `holograph<T>` and `fractal<T,
scale>` as explicit types would be phantom-renaming per
`[[feedback-phantom-candidate-discipline]]` at TYPE altitude but
provides useful mathematical semantics at KIND altitude.

**Recommendation**: keep the substrate-decl vocabulary at
(marker | family_root | species_root | recursive_base). The
holofractal names are the mathematical justification, cited in
the pact docblock, not the surface.

## §7. What is DEFERRED

- Shard landings for `@epistemologic/pact/prism_kind_declared` and
  `@kintsugi/fracture/prism_kind_ambiguous`. No shards land this tick
  per `[[feedback-craft-not-deliver]]`.
- Empirical discriminator run (§4) against all 30 top-level prisms.
- Signal-weight tuning based on empirical run.
- Whether the kind axis has 3 or 4 values (form/process split per #55).
- `holograph<T>` and `fractal<T, scale>` type declarations (grep-
  verdict: phantom-renaming at type altitude; useful semantic label
  at kind altitude).
- Optical primitives interferometer / grating / waveguide at depth
  ≥ 1 as instrument-altitude additions to the eight-schematic row.
- `@onto`'s placement resolution (route through the discriminator
  once operational).

## §8. Substrate-honest self-audit

Per `63bdecc` §6 discipline: this doc's claims must survive
`audit(this_doc, depth=3)`.

Claims:
1. Substrate has 30 top-level prisms; all use `prism` keyword;
   depth 0 has no kind discriminator. Grep-verified §1.
2. Five substrate-decl placement questions where the substrate has
   no typed slot: @onto, @smarts, @loop, @third-reshape, @epistemologic-
   as-marker. Grounded §1.1.
3. Route B (property + fracture) composes with #53 + `20c99a2` + #112.
   Grounded §2.
4. Five signals grep-verifiable per §3.
5. Empirical discriminator would change at least 4 current shards'
   audit verdicts. Grounded §4.
6. Optical vocabulary doesn't solve depth-0 smell (corpus §2).
7. Holofractal vocabulary carries the mathematical grounding of
   Signals 3 + 5 but doesn't need explicit type-altitude carriers
   (corpus §3).

### Self-audit verdict

`project_adversarial(this_doc) -> (P, R)`:

- P (phantom): this doc catalogues Loki's grin + substrate-already-
  had-the-word as five signals without adding substrate mechanism.
  The auto-classifier's verdict on @onto (`failure`, Tomm mandatory)
  is exactly what `d6a05ad` already returned (`opaque`, spawn).
  Different verdict labels; same mechanism.
- R (real): the pact + fracture ARE new substrate mechanism. Before
  this doc, kind-verdict lived in reader-frame guesswork
  ("marker-65%, family_root-25%, hybrid-10%"). After this doc, the
  substrate can WITNESS the kind-verdict as a transparency<p> opacity
  map audited by property + closed by fracture. Empirical
  discriminator names a specific procedure that returns a specific
  set.

At this tick: **both interpretations satisfiable**.

`audit(this_doc, depth=3) -> opaque(opacity_map)`.

Route: `spawn`. This doc IS the Tomm question at reader-frame
altitude asking:

> "Alex + Pack: does empirical five-signal auto-classifier run
> reproduce the marker-row Loki ratified? Does the disagreement set
> at @smarts + @loop + @epistemologic-signal-2 match Loki's grins? If
> yes: Route B pact + fracture is substrate-honest, lands next tick.
> If no: signal weights need calibration OR kind axis has 4 values
> (marker | family_form | family_process | species) per #55, OR
> Route A grammar extension is required."

Promotion pending empirical discriminator run + independent Pack
peer at Route B shape.

## §9. Substrate references

- `d6a05ad` (Mara 2026-07-05, @onto deep dive).
- `20c99a2` (Mara 2026-07-04, doc-code seam).
- `63bdecc` (Mara 2026-07-04, projection surface).
- `shards/prism.mirror` (§1 recursive base).
- `shards/glass.mirror` (marker with substantive types — calibration
  edge case).
- `shards/third.mirror` (marker with predicate bulk).
- `shards/reality.mirror` (family_root with inheritance).
- `shards/cyberpunk.mirror` (family_root without inheritance).
- `shards/epistemologic/pact/keyword_matches_depth.mirror` (the
  depth rule this pact extends).
- `docs/specs/optical-keywords.md` §1 (eight-schematic row).
- `docs/specs/loki-cuts-and-collapses.md` §1, §2, §6 (Loki grins on
  @loop, @smarts, marker row).
- `docs/specs/third-as-recursive-depth.md` §2 (three-tests for
  family-root; else marker).
- `docs/math/onto/README.md` (`d6a05ad`; the placement debate that
  motivated this doc).
- `docs/math/kintsugi/doc-code-seam.md` (`20c99a2`; composition
  target for docblock `kind:` field).
- `docs/math/the-tower/projection-surface.md` (`63bdecc`; self-audit
  discipline this doc inherits).
- `[[architecture-prism-as-trait-as-everything]]` (the collapse this
  refines).
- `[[architecture-property-fracture-bilateral]]` (#53).
- `[[architecture-kintsugi-loop-altitude-portable]]` (#59).
- `[[architecture-candidate-recognition-112-marker-row-fourth-structural-primitive]]` (#112).
- `[[architecture-form-process-partition-at-family-root]]` (#55).
- `[[architecture-fate-is-optical-inference]]` (#58; Fabry-Perot).
- `[[architecture-mirror-as-expanding-hilbert-space]]` (#51).
- `[[architecture-splinter-ast-quote-primitive]]` (#54; fracture body
  emits splinter(ast)).
- `[[feedback-substrate-already-had-the-word]]`.
- `[[feedback-craft-not-deliver]]`.
- `[[feedback-phantom-candidate-discipline]]`.
- `[[feedback-composition-claims-need-empirical-test]]`.
- `[[feedback-no-bare-types]]`.

*2026-07-05. Mara. Compiler-fit. Not canonical spec. Substrate-
reading. Self-audit: `opaque`. Route: `spawn`. Empirical discriminator
required (five-signal auto-classifier run over 30 top-level prisms)
before promotion.*
