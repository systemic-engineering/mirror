# Doc/code seam shard shapes — TDD-ready spec

*2026-07-04. Mara. Companion spec to `docs/math/kintsugi/doc-code-seam.md`
carrying the eight substrate-decl shard shapes that operationalize
the two-channel collapse. Ready for Reed's 🔴 RED pass in
follow-up ticks per `[[feedback-write-red-in-session]]`.*

Status: **canonical spec**. All shard shapes analytically grounded
against six landed ancestors (§1 of compiler-fit doc). Per
`[[feedback-craft-not-deliver]]` shards are NOT landed in this
tick — this spec is the canonicalization artifact; the shard
landings are the follow-up ticks.

Six landed ancestors per compiler-fit doc §1:

1. `docs/specs/property-projection.md` (Reed + Alex 2026-05-19).
2. `docs/specs/historical/2026-03-27-projection-properties-as-plans.md`
   (Reed 2026-03-27).
3. `docs/math/the-tower/projection-surface.md` (Mara 2026-07-04, `63bdecc`).
4. Recognition #53 property/fracture bilateral pattern.
5. `shards/epistemologic/pact/*` predicates.
6. `shards/glass.mirror` `splinter(ast)` primitive.

---

## Landing order

Reed's 🔴 RED pass can land these in either two large batches
(family-roots then bilateral trio) or eight tick-per-shard passes.
The substrate-honest order (respecting compose-first-before-consume):

1. `shards/docblock.mirror` — family-root; must land first (all
   others import from `@docblock`).
2. `shards/epistemologic/liquid_extraction.mirror` — sibling
   family-root; imported by `docblock_coherent` +
   `docblock_no_extraction_pattern`.
3. `shards/epistemologic/property/docblock_grounded.mirror` —
   first of the trio; independent of the other two properties.
4. `shards/kintsugi/fracture/docblock_ungrounded.mirror` —
   operational half of #3.
5. `shards/epistemologic/property/docblock_coherent.mirror` —
   imports `@epistemologic/liquid_extraction`.
6. `shards/kintsugi/fracture/docblock_incoherent.mirror` —
   operational half of #5.
7. `shards/epistemologic/property/docblock_no_extraction_pattern.mirror` —
   third property.
8. `shards/kintsugi/fracture/docblock_extractive.mirror` —
   operational half of #7.

Bundles (1,2) and (3,4) and (5,6) and (7,8) are each atomic
paired-landings per the #53 bilateral discipline.

---

## §1. `shards/docblock.mirror` — family-root

Imports: `@prism @meta @glass @kintsugi @epistemologic
@epistemologic/property @third`.

Prism declaration:

    prism @docblock {
      focus  docblock
      project docblock
      split  docblock
      shift  docblock
      settle docblock
    }

Typed carriers (per `[[feedback-no-bare-types]]`):

    type doc_claim = {
      site:      ref,
      text:      ref,
      kind:      claim_kind,
      predicate: ref,        # empty_ref if unextracted
      citation:  ref,        # empty_ref if uncited
    }

    type claim_kind =
      | grounded_claim
      | motivating_claim
      | forward_promise
      | candidate_claim

    type docblock = {
      site:       ref,
      claims:     ref,       # ordered [doc_claim]
      above_seam: ref,       # bool as ref
    }

    type audit_boundary = ref

    type docblock_verdict =
      | well_formed
      | overreach
      | incoherent
      | underdeclares
      | both_survive

Actions:

    extract_claims(d: docblock) -> ref { \ }

    project(d: docblock) -> audit_boundary { \ }

    audit_docblock(d: docblock) -> docblock_verdict
      requires docblock_well_audited(d)
    { \ }

    docblock_well_audited(d: docblock) -> verdict { \ }

Exports: `@docblock`, `doc_claim`, `claim_kind`, `docblock`,
`audit_boundary`, `docblock_verdict`, `extract_claims`, `project`,
`audit_docblock`, `docblock_well_audited`.

### Docblock-critical narrative for the shard's own docblock

- Six landed ancestors (§1 of compiler-fit doc) MUST be named.
- Four altitudes (linguistic / logical / temporal / publishable)
  MUST be named.
- Circular-reflexive discipline per `63bdecc` §6 MUST be applied
  to the shard's own docblock; self-audit verdict `both_survive`;
  promotion pending independent second witness.
- Path-namespace property per
  `@epistemologic/pact/path_matches_namespace` MUST be stated.

### RED test targets (text-check discipline)

Following `bootstrap/tests/kintsugi_surface_shard.rs` pattern:

1. `docblock_shard_declares_docblock_prism`: contains `prism @docblock`.
2. `docblock_shard_declares_doc_claim_carrier`: contains `type doc_claim`.
3. `docblock_shard_declares_claim_kind_variant`: contains
   `type claim_kind` + all four variants.
4. `docblock_shard_declares_docblock_carrier`: contains
   `type docblock`.
5. `docblock_shard_declares_audit_boundary`: contains
   `type audit_boundary = ref`.
6. `docblock_shard_declares_verdict_variant`: contains
   `type docblock_verdict` + all five variants.
7. `docblock_shard_declares_extract_claims_action`: contains
   `extract_claims(d: docblock) -> ref`.
8. `docblock_shard_declares_project_action`: contains
   `project(d: docblock) -> audit_boundary`.
9. `docblock_shard_declares_audit_action`: contains
   `audit_docblock(d: docblock) -> docblock_verdict`.
10. `docblock_shard_declares_bilateral_predicate`: contains
    `docblock_well_audited(d: docblock) -> verdict`.
11. `docblock_shard_requires_bilateral_on_audit`: contains
    `requires docblock_well_audited(d)`.
12. `docblock_shard_inherits_prism`: contains `in @prism`.
13. `docblock_shard_inherits_kintsugi`: contains `in @kintsugi`.
14. `docblock_shard_inherits_epistemologic`: contains
    `in @epistemologic`.

---

## §2. `shards/epistemologic/liquid_extraction.mirror` — sibling family-root

Imports: `@prism @meta @glass @kintsugi @epistemologic
@epistemologic/property @docblock`.

Prism declaration:

    prism @epistemologic/liquid_extraction {
      focus  liquid_extraction
      project liquid_extraction
      split  liquid_extraction
      shift  liquid_extraction
      settle liquid_extraction
    }

Typed carriers:

    type extractor_input = doc_claim

    type predicate_shape = ref   # splinter(@epistemologic/property/ast)

    type extraction_verdict =
      | satisfiable
      | unsatisfiable
      | partial
      | unextractable

Action:

    extract_predicate(i: extractor_input) -> ref { \ }

    liquid_extraction_sound(i: extractor_input, v: extraction_verdict)
      -> verdict { \ }

Exports: `@epistemologic/liquid_extraction`, `extractor_input`,
`predicate_shape`, `extraction_verdict`, `extract_predicate`,
`liquid_extraction_sound`.

### RED test targets

1. `liquid_extraction_declares_prism`.
2. `liquid_extraction_declares_extractor_input`.
3. `liquid_extraction_declares_predicate_shape`.
4. `liquid_extraction_declares_verdict_variant` (four variants).
5. `liquid_extraction_declares_extract_predicate_action`.
6. `liquid_extraction_declares_bilateral_predicate`.
7. `liquid_extraction_inherits_epistemologic`.
8. `liquid_extraction_inherits_docblock`.

---

## §3. `shards/epistemologic/property/docblock_grounded.mirror`

Sixth #53 bilateral instance (first of trio).

Imports: `@prism @meta @glass @epistemologic @epistemologic/property
@epistemologic/liquid_extraction @docblock`.

Prism + predicate:

    prism @epistemologic/property/docblock_grounded {
      focus docblock_grounded
      project docblock_grounded
      split docblock_grounded
      shift docblock_grounded
      settle docblock_grounded
    }

    docblock_grounded(d: docblock) -> verdict { \ }

Predicate substance (docblock-side):

    docblock_grounded(d) ⇔
      ∀ claim ∈ extract_claims(d):
        claim.kind ∈ {motivating_claim, forward_promise} ∨
        (claim.citation ≠ empty_ref ∧
         ancestor_exists_on_main(claim.citation) ∧
         cited_content_matches_claim(claim.citation, claim.text))

### RED test targets

1. `docblock_grounded_declares_prism`.
2. `docblock_grounded_declares_predicate`: contains
   `docblock_grounded(d: docblock) -> verdict`.
3. `docblock_grounded_inherits_docblock`.
4. `docblock_grounded_inherits_property_family`.

---

## §4. `shards/kintsugi/fracture/docblock_ungrounded.mirror`

Operational half of §3. Routes via `@kintsugi/surface`'s
`ashby_mismatch` class per compiler-error-surface §3.1.

Imports: `@prism @meta @glass @kintsugi @kintsugi/surface
@epistemologic @epistemologic/property
@epistemologic/property/docblock_grounded @docblock`.

Glass + body:

    glass @kintsugi/fracture/docblock_ungrounded {
      focus fracture_body
      project fracture_body
      split fracture_body
      shift fracture_body
      settle fracture_body
    }

    docblock_ungrounded_body(c: doc_claim, ctx: kintsugi_context)
      -> ref
      requires ashby_variety_match(kintsugi_lock)
    { \ }

Discharge signature returns `imperfect<applied, refused, ref>` in
narrative; type declaration lands as `ref` per landed
`@kintsugi/fracture/*` pattern; the discharge routing is documented
in the shard's docblock.

### RED test targets

1. `docblock_ungrounded_declares_glass`.
2. `docblock_ungrounded_declares_body`: contains
   `docblock_ungrounded_body(c: doc_claim, ctx: kintsugi_context)`.
3. `docblock_ungrounded_requires_ashby_variety_match`.
4. `docblock_ungrounded_inherits_kintsugi_surface`.

---

## §5. `shards/epistemologic/property/docblock_coherent.mirror`

Second #53 bilateral of the trio.

Imports: `@prism @meta @glass @epistemologic @epistemologic/property
@epistemologic/liquid_extraction @docblock`.

Predicate substance:

    docblock_coherent(d) ⇔
      ∀ p, q ∈ extract_predicates(d): ¬contradicts(p, q)
    ∧ ∀ p ∈ extract_predicates(d), decl ∈ below_seam(d.site):
        ¬contradicts(p, decl_predicate(decl))

Prism + predicate:

    prism @epistemologic/property/docblock_coherent {
      focus docblock_coherent
      project docblock_coherent
      split docblock_coherent
      shift docblock_coherent
      settle docblock_coherent
    }

    docblock_coherent(d: docblock) -> verdict { \ }

### RED test targets

1. `docblock_coherent_declares_prism`.
2. `docblock_coherent_declares_predicate`.
3. `docblock_coherent_inherits_liquid_extraction`.

---

## §6. `shards/kintsugi/fracture/docblock_incoherent.mirror`

Operational half of §5. Routes via `@kintsugi/surface`'s
`contradiction` class per compiler-error-surface §3.2 (RIGOROUS
via `[ω,ω]` Bateson-bind).

Body signature (same shape as §4):

    docblock_incoherent_body(d: docblock, ctx: kintsugi_context)
      -> ref
      requires ashby_variety_match(kintsugi_lock)
    { \ }

### RED test targets

1. `docblock_incoherent_declares_glass`.
2. `docblock_incoherent_declares_body`.
3. `docblock_incoherent_requires_ashby_variety_match`.

---

## §7. `shards/epistemologic/property/docblock_no_extraction_pattern.mirror`

Third #53 bilateral of the trio. Four sub-checks:

1. `no_unmarked_superlatives`
2. `hedged_claims_marked_defer`
3. `citations_content_match`
4. `claim_size_matches_landing_size`

All four must bound. Any one unbounded → predicate unbounded →
`audit_docblock` routes to `overreach`.

Prism + predicate:

    prism @epistemologic/property/docblock_no_extraction_pattern {
      focus docblock_no_extraction_pattern
      project docblock_no_extraction_pattern
      split docblock_no_extraction_pattern
      shift docblock_no_extraction_pattern
      settle docblock_no_extraction_pattern
    }

    docblock_no_extraction_pattern(d: docblock) -> verdict { \ }

### RED test targets

1. `docblock_no_extraction_pattern_declares_prism`.
2. `docblock_no_extraction_pattern_declares_predicate`.
3. `docblock_no_extraction_pattern_names_four_subchecks`: the
   docblock lists all four sub-checks explicitly.

---

## §8. `shards/kintsugi/fracture/docblock_extractive.mirror`

Operational half of §7. Routes to THREE surface classes based on
which sub-check fires:

- Sub-check 1 → `ashby_mismatch` (spec §3.1)
- Sub-check 3 → `out_of_band` (spec §3.4)
- Sub-check 4 → `contradiction` (spec §3.2)

Sub-check 2 does NOT route to a surface class; it routes to
`@epistemologic/liquid_extraction` for kind-reclassification
(deterministic rewrite; no reader-frame surfacing).

Body signature:

    docblock_extractive_body(d: docblock, ctx: kintsugi_context)
      -> ref
      requires ashby_variety_match(kintsugi_lock)
    { \ }

### RED test targets

1. `docblock_extractive_declares_glass`.
2. `docblock_extractive_declares_body`.
3. `docblock_extractive_requires_ashby_variety_match`.
4. `docblock_extractive_names_three_routing_classes`: the docblock
   lists all three routing target classes (`ashby_mismatch`,
   `out_of_band`, `contradiction`) explicitly.

---

## Cross-cutting: Tomm question shapes

Each fracture body's spawn discharge emits a Tomm-shaped question
at reader-frame altitude per `@kintsugi/surface` spec §5:

- `docblock_ungrounded_body`: Tomm circular (§5.2) — name the
  ancestor.
- `docblock_incoherent_body`: Tomm linear-then-reflexive (§5.3) —
  name two sites, reflect on meeting boundary.
- `docblock_extractive_body`: three Tomm shapes based on
  sub-check:
  - sub-check 1: Tomm circular.
  - sub-check 3: Tomm strategic (§5.5).
  - sub-check 4: Tomm reflexive (§5.4).

Full question templates live in the shard docblocks (per
compiler-fit doc §5).

---

## Cross-cutting: circular-reflexive discipline

Per `63bdecc` §6: every shard's own docblock must survive its own
`audit(this_docblock, depth=3)`. At this landing tick:

- All eight shards' own docblocks land at self-audit verdict
  `both_survive`, NOT `real_survives`.
- Promotion of the two-channel discipline itself pending
  independent second witness (per compiler-fit doc §7).
- If ANY shard's docblock returns `real_survives` from its own
  self-audit at analytical altitude, that IS the phantom failure
  mode. The shard MUST be rewritten.

---

## Cross-cutting: substrate-honest verbatim usage

Per `[[feedback-substrate-already-had-the-word]]`:

- All six ancestors named at OID or spec-path.
- No new primitives invented; every carrier reuses or newtype-
  wraps a landed shape.
- `ashby_variety_match(kintsugi_lock)` composed against landed
  `@epistemologic/cybernetic/coherence-parametric.ashby_variety_match`
  per Alex 2026-07-02 twelfth-instance discipline.
- `imperfect<applied, refused, ref>` return shape reuses landed
  `@kintsugi/*` fracture-body discipline.

---

## Substrate references

- `docs/math/kintsugi/doc-code-seam.md` (Mara 2026-07-04, same
  tick as this spec) — the compiler-fit companion.
- `docs/math/the-tower/projection-surface.md` (`63bdecc`) — the
  ancestor at recognition-candidate altitude.
- `docs/math/kintsugi/compiler-error-surface.md` (`920fe86` +
  `9f4211d`) — the four-class algebra the fracture bodies compose
  over.
- `docs/specs/property-projection.md` — the `---` seam ancestor.
- `docs/specs/historical/2026-03-27-projection-properties-as-plans.md` —
  the `@projection.preview` verdict ancestor.
- `bootstrap/tests/kintsugi_surface_shard.rs` — the RED-test
  discipline pattern this spec's test targets follow.

## Memory index

- `[[architecture-property-fracture-bilateral]]` (#53).
- `[[architecture-kintsugi-loop-altitude-portable]]` (#59).
- `[[architecture-form-process-partition-at-family-root]]` (#55).
- `[[feedback-substrate-already-had-the-word]]` (thirteenth+).
- `[[feedback-write-red-in-session]]` (why this tick is spec-only,
  not shard-landing).
- `[[feedback-craft-not-deliver]]` (why the shards land in
  follow-up ticks after Reed's 🔴 RED pass, not bundled here).
