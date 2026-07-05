# Doc/code seam — bottom-up landing sequence (revised TDD roadmap)

*2026-07-05. Mara. Derived spec revising `docs/specs/doc-code-seam-shards.md`
(landed `20c99a2` 2026-07-04) per Alex 2026-07-05 direction: bottom-up
substrate-honest ordering that lets prism-kind emerge naturally as the
first concrete USE.*

Composes with:
- `docs/math/liquid-types/README.md` (this tick — the load-bearing math).
- `docs/math/kintsugi/doc-code-seam.md` (`20c99a2` — the canonical
  compiler-fit doc; §5 landing order).
- `docs/math/prism-kind/README.md` (`bdb148a` — the first concrete USE).
- `docs/specs/doc-code-seam-shards.md` (2026-07-04 — the eight-shard
  spec being revised here).
- `bootstrap/src/tokenize.rs` (`ee7903e` — Docblock AST-node emission
  above `---` LANDED).

Status: **derived spec revising `doc-code-seam-shards.md`**. Ready for
Reed's 🔴 RED pass in follow-up TDD ticks per
`[[feedback-write-red-in-session]]`.

Per `[[feedback-craft-not-deliver]]` no shards land this tick.

---

## §0. What changed vs `doc-code-seam-shards.md`

Alex 2026-07-05 direction: bottom-up. The whole substrate is bottom-up.
Land the eight-shard cascade FIRST; prism-kind emerges naturally as
the first concrete USE.

Three questions I asked of the existing spec:

1. **Is `docblock.mirror` really first?** Or does `liquid_extraction`
   need to land first so `docblock` can consume its verdict machinery?
2. **Do all eight shards need to land at once?** Or does a minimal
   subset unlock prism-kind (Alex's target)?
3. **Where's the load-bearing precondition?** What's the earliest
   tick that fires the auto-classifier for real (not spec-only)?

Substrate-honest answers, derived from the math (`liquid-types/README.md`)
+ the substrate (grep on 30 top-level shards):

1. **`docblock.mirror` IS first** — but for a bottom-up reason the
   original spec didn't name: it declares the `doc_claim` carrier
   that ALL other shards (including `liquid_extraction`) consume as
   their input type. Carrier-first ordering per
   `[[feedback-no-bare-types]]`. Confirmed.
2. **Minimal viable path is 4 shards, not 8.** Shards 5-8 land
   AFTER prism-kind because they consume prism-kind's five-signal
   verdict as their qualifier set. The full eight-shard cascade
   composes UNDER prism-kind, not around it. This IS the closure
   Alex named.
3. **Load-bearing precondition** for auto-classifier firing: only
   the family-root (`docblock.mirror`) needs to land before the
   auto-classifier's grep procedure can run on real files. Signal
   grep does not require the eight-shard cascade — it requires the
   `doc_claim` carrier + one property that names the audit surface.

Revised ordering:

```
TICK 1: shards/docblock.mirror                       (family-root)
TICK 2: shards/epistemologic/liquid_extraction.mirror (sibling family-root)
TICK 3: shards/epistemologic/pact/prism_kind_declared.mirror  (prism-kind §2.1)
TICK 4: shards/kintsugi/fracture/prism_kind_ambiguous.mirror  (prism-kind §2.2)
─── auto-classifier fires for real at TICK 3+4 close ───
TICK 5: shards/epistemologic/property/docblock_grounded.mirror
TICK 6: shards/kintsugi/fracture/docblock_ungrounded.mirror
TICK 7: shards/epistemologic/property/docblock_coherent.mirror
TICK 8: shards/kintsugi/fracture/docblock_incoherent.mirror
TICK 9: shards/epistemologic/property/docblock_no_extraction_pattern.mirror
TICK 10: shards/kintsugi/fracture/docblock_extractive.mirror
```

Ten shards, not eight. TICK 3+4 IS the prism-kind sixth #53 bilateral
NATURALLY, composing on TICK 1+2's carriers per `bdb148a` §2.3.

---

## §1. TICK 1 — `shards/docblock.mirror` (family-root)

**Load-bearing FIRST landing**. Zero preconditions beyond the tokenizer
change (LANDED at `ee7903e`).

### Signature

```
prism @docblock {
  focus  docblock
  project docblock
  split  docblock
  shift  docblock
  settle docblock
}
```

### Carriers (per `[[feedback-no-bare-types]]`)

```
type doc_claim = {
  site:      ref,
  text:      ref,
  kind:      claim_kind,
  predicate: ref,          # empty_ref if unextracted
  citation:  ref,          # empty_ref if uncited
}

type claim_kind =
  | grounded_claim
  | motivating_claim
  | forward_promise
  | candidate_claim

type docblock = {
  site:       ref,
  claims:     ref,         # ordered [doc_claim]
  above_seam: ref,         # bool as ref
}

type audit_boundary = ref  # content-addressed OID

type docblock_verdict =
  | well_formed
  | overreach
  | incoherent
  | underdeclares
  | both_survive
```

### Actions

```
extract_claims(d: docblock) -> ref { \ }
project(d: docblock) -> audit_boundary { \ }
audit_docblock(d: docblock) -> docblock_verdict
  requires docblock_well_audited(d)
{ \ }
docblock_well_audited(d: docblock) -> verdict { \ }
```

### Landing precondition

- `bootstrap/src/tokenize.rs` emits `Docblock` AST nodes above `---`
  (LANDED at `ee7903e`).
- Zero shard preconditions.

### Composition dependencies

- Depends on `@prism @meta @glass @kintsugi @epistemologic
  @epistemologic/property @third` (all LANDED).
- No downstream shard depends on TICK 1's ACTIONS being discharged —
  they can all remain `\`.

### Audit test targets (text-check discipline per
`bootstrap/tests/kintsugi_surface_shard.rs`)

1. `docblock_shard_declares_docblock_prism`.
2. `docblock_shard_declares_doc_claim_carrier`.
3. `docblock_shard_declares_claim_kind_variant` (four variants).
4. `docblock_shard_declares_docblock_carrier`.
5. `docblock_shard_declares_audit_boundary`.
6. `docblock_shard_declares_verdict_variant` (five variants).
7. `docblock_shard_declares_extract_claims_action`.
8. `docblock_shard_declares_project_action`.
9. `docblock_shard_declares_audit_action`.
10. `docblock_shard_declares_bilateral_predicate`.
11. `docblock_shard_requires_bilateral_on_audit`.
12. `docblock_shard_inherits_prism`.
13. `docblock_shard_inherits_kintsugi`.
14. `docblock_shard_inherits_epistemologic`.

---

## §2. TICK 2 — `shards/epistemologic/liquid_extraction.mirror` (sibling family-root)

**Depends on TICK 1** (consumes `doc_claim` as input type).

### Signature

```
prism @epistemologic/liquid_extraction {
  focus  liquid_extraction
  project liquid_extraction
  split  liquid_extraction
  shift  liquid_extraction
  settle liquid_extraction
}
```

### Carriers

```
type extractor_input = doc_claim

type predicate_shape = ref   # splinter(@epistemologic/property/ast)

type extraction_verdict =
  | satisfiable
  | unsatisfiable
  | partial
  | unextractable
```

Note verdict shape matches Reed's landed `@projection.preview` verdict
(`satisfiable | unsatisfiable | partial`) plus one branch
(`unextractable` — the claim itself isn't extractable to a liquid
predicate). Adequacy per liquid-types/README.md §2.

### Actions

```
extract_predicate(i: extractor_input) -> ref { \ }
liquid_extraction_sound(i: extractor_input, v: extraction_verdict) -> verdict { \ }
```

### Landing precondition

- TICK 1 (`docblock` family-root LANDED so `doc_claim` type resolves).

### Composition dependencies

- Depends on `@docblock` (TICK 1).
- Consumed by TICK 5 + TICK 7 (both docblock_coherent-family
  properties reference `liquid_extraction`).
- NOT consumed by TICK 3 + TICK 4 (prism-kind operates on structural
  signals, not on extracted liquid predicates — this is why TICK 3+4
  can fire BEFORE the full liquid extractor is empirically bounded).

### Audit test targets

1. `liquid_extraction_declares_prism`.
2. `liquid_extraction_declares_extractor_input`.
3. `liquid_extraction_declares_predicate_shape`.
4. `liquid_extraction_declares_verdict_variant` (four variants).
5. `liquid_extraction_declares_extract_predicate_action`.
6. `liquid_extraction_declares_bilateral_predicate`.
7. `liquid_extraction_inherits_epistemologic`.
8. `liquid_extraction_inherits_docblock`.

---

## §3. TICK 3 — `shards/epistemologic/pact/prism_kind_declared.mirror` (prism-kind property side; sixth #53 bilateral)

**Depends on TICK 1 + TICK 2.** IS the sixth #53 bilateral instance
NATURALLY per `bdb148a` §2.3. IS the earliest tick that fires the
auto-classifier for real.

### Signature

Per `bdb148a` §2.1:

```
pact @epistemologic/pact/prism_kind_declared {
  declared_kind(file: ref) -> option(prism_kind) { \ }
  computed_signals(file: ref) -> kind_signals { \ }
  prism_kind_declared(file: ref) -> transparency { \ }
}
```

### Carriers

Per `bdb148a` §2.1:

```
type prism_kind =
  | marker
  | family_root
  | species_root
  | recursive_base

type kind_signals = {
  inherits:            bool,        # signal 1
  carrier_density:     u32,         # signal 2
  cross_family_import: u32,         # signal 3
  cites_marker_row:    bool,        # signal 4
  cites_form_process:  bool,        # signal 4b
  primary_thin:        bool,        # signal 5
}
```

### Landing precondition

- TICK 1 (needs `docblock` for `computed_signals` to grep the docblock's
  `kind:` field).
- TICK 2 (needs `liquid_extraction` for the composition to compose
  through `docblock_kind_matches_signals` — see §5).
- Grep procedure implemented behind the `\` (either as Rust bootstrap
  helper OR as forward-promised body).

### Composition dependencies

- Consumes: `doc_claim` (TICK 1), `docblock` (TICK 1).
- Consumed by: TICK 4 (`prism_kind_ambiguous` fracture body).
- Downstream: prism-kind auto-classifier operational; can run
  against all 30 top-level shards.

### Audit test targets

1. `prism_kind_declared_declares_pact`.
2. `prism_kind_declared_declares_prism_kind_variant` (four variants:
   marker, family_root, species_root, recursive_base).
3. `prism_kind_declared_declares_kind_signals_carrier` (six fields).
4. `prism_kind_declared_declares_declared_kind_action`.
5. `prism_kind_declared_declares_computed_signals_action`.
6. `prism_kind_declared_declares_verdict_action`.
7. `prism_kind_declared_inherits_glass`.
8. `prism_kind_declared_inherits_docblock`.

---

## §4. TICK 4 — `shards/kintsugi/fracture/prism_kind_ambiguous.mirror` (prism-kind fracture side; sixth #53 bilateral)

**Depends on TICK 3.** Closes the sixth #53 bilateral pair.

### Signature

Per `bdb148a` §2.2:

```
fracture @kintsugi/fracture/prism_kind_ambiguous {
  classify(sigs: kind_signals) -> prism_kind { \ }
  agreement_count(sigs: kind_signals, k: prism_kind) -> u32 { \ }
  fracture_body(file: ref, sigs: kind_signals) -> splinter(ast) { \ }
}
```

### Landing precondition

- TICK 3 (`prism_kind_declared` pact LANDED so `kind_signals` carrier
  resolves).
- `@kintsugi/surface` LANDED at `e910dd6` (three-mode algebra
  supplier) — already LANDED.

### Composition dependencies

- Consumes: `kind_signals` (TICK 3), `@kintsugi/surface` (LANDED).
- Discharges via `splinter(@meta/ast)` per `#54` (LANDED).
- **CLOSES THE SIXTH #53 BILATERAL PAIR**.

### Audit test targets

1. `prism_kind_ambiguous_declares_fracture`.
2. `prism_kind_ambiguous_declares_classify_action`.
3. `prism_kind_ambiguous_declares_agreement_count_action`.
4. `prism_kind_ambiguous_declares_body`.
5. `prism_kind_ambiguous_requires_ashby_variety_match`.
6. `prism_kind_ambiguous_inherits_kintsugi_surface`.

### **What fires at TICK 3+4 close**

The auto-classifier fires for real. `computed_signals(file)` runs
against all 30 top-level `shards/*.mirror` files (per `bdb148a` §4
empirical discriminator run). The verdict routing lands. The @onto
placement verdict from `d6a05ad` §3.3 becomes a first-class typed
opacity map (not reader-frame narrative).

**THIS is Alex's target**. Bottom-up: four shards, not eight, unlock
prism-kind's operational form. The remaining six shards (TICK 5-10)
extend the audit surface to the full doc-claim-altitude discipline.

---

## §5. TICK 5 — `shards/epistemologic/property/docblock_grounded.mirror`

**Depends on TICK 1 + TICK 2 + TICK 4.** First of the docblock-audit
trio. Extends the audit surface post-prism-kind.

### Signature

```
prism @epistemologic/property/docblock_grounded {
  focus docblock_grounded
  project docblock_grounded
  split docblock_grounded
  shift docblock_grounded
  settle docblock_grounded
}

docblock_grounded(d: docblock) -> verdict { \ }
```

### Predicate substance

```
docblock_grounded(d) ⇔
  ∀ claim ∈ extract_claims(d):
    claim.kind ∈ {motivating_claim, forward_promise} ∨
    (claim.citation ≠ empty_ref ∧
     ancestor_exists_on_main(claim.citation) ∧
     cited_content_matches_claim(claim.citation, claim.text))
```

### Landing precondition

- TICK 4 close (auto-classifier operational; this property audits
  docblocks that classify as `well_formed` under the classifier).

### Composition dependencies

- Consumes: `docblock` (TICK 1), extraction machinery (TICK 2).
- Consumed by: `audit_docblock` in TICK 1 (through `\` discharge).
- Sibling to TICK 7 + TICK 9.

### Audit test targets

1. `docblock_grounded_declares_prism`.
2. `docblock_grounded_declares_predicate`.
3. `docblock_grounded_inherits_docblock`.
4. `docblock_grounded_inherits_property_family`.

---

## §6. TICK 6 — `shards/kintsugi/fracture/docblock_ungrounded.mirror`

**Depends on TICK 5.** Operational half of TICK 5 (seventh #53
bilateral instance).

### Signature

```
glass @kintsugi/fracture/docblock_ungrounded {
  focus fracture_body
  project fracture_body
  split fracture_body
  shift fracture_body
  settle fracture_body
}

docblock_ungrounded_body(c: doc_claim, ctx: kintsugi_context) -> ref
  requires ashby_variety_match(kintsugi_lock)
{ \ }
```

### Landing precondition

- TICK 5 (property side).
- `@kintsugi/surface` LANDED.

### Composition dependencies

- Consumes: `doc_claim` (TICK 1), `kintsugi_context` (LANDED at
  `e910dd6`), `@kintsugi/surface` four-class algebra (LANDED).
- Discharges via `splinter(@meta/ast)` per `#54`.
- Routes via `@kintsugi/surface`'s `ashby_mismatch` class per
  `compiler-error-surface.md` §3.1.
- CLOSES the seventh #53 bilateral pair.

### Audit test targets

1. `docblock_ungrounded_declares_glass`.
2. `docblock_ungrounded_declares_body`.
3. `docblock_ungrounded_requires_ashby_variety_match`.
4. `docblock_ungrounded_inherits_kintsugi_surface`.

---

## §7. TICK 7 — `shards/epistemologic/property/docblock_coherent.mirror`

**Depends on TICK 1 + TICK 2 + TICK 4.** Second of the docblock-audit
trio.

### Signature + predicate substance

```
prism @epistemologic/property/docblock_coherent {
  focus docblock_coherent
  project docblock_coherent
  split docblock_coherent
  shift docblock_coherent
  settle docblock_coherent
}

docblock_coherent(d: docblock) -> verdict { \ }
```

Predicate substance:

```
docblock_coherent(d) ⇔
  ∀ p, q ∈ extract_predicates(d): ¬contradicts(p, q)
∧ ∀ p ∈ extract_predicates(d), decl ∈ below_seam(d.site):
    ¬contradicts(p, decl_predicate(decl))
```

### Landing precondition

- TICK 2 (`liquid_extraction` — `extract_predicates` requires it).

### Composition dependencies

- Consumes: `docblock` (TICK 1), `liquid_extraction` (TICK 2).
- Sibling to TICK 5 + TICK 9.

### Audit test targets

1. `docblock_coherent_declares_prism`.
2. `docblock_coherent_declares_predicate`.
3. `docblock_coherent_inherits_liquid_extraction`.

---

## §8. TICK 8 — `shards/kintsugi/fracture/docblock_incoherent.mirror`

**Depends on TICK 7.** Operational half (eighth #53 bilateral instance).

### Signature

```
docblock_incoherent_body(d: docblock, ctx: kintsugi_context) -> ref
  requires ashby_variety_match(kintsugi_lock)
{ \ }
```

### Landing precondition + Composition dependencies

Same shape as TICK 6, but routes via `@kintsugi/surface`'s
`contradiction` class (RIGOROUS per `compiler-error-surface.md` §3.2
via `[ω,ω]` Bateson-bind).

### Audit test targets

1. `docblock_incoherent_declares_glass`.
2. `docblock_incoherent_declares_body`.
3. `docblock_incoherent_requires_ashby_variety_match`.

---

## §9. TICK 9 — `shards/epistemologic/property/docblock_no_extraction_pattern.mirror`

**Depends on TICK 1 + TICK 2 + TICK 4.** Third of the docblock-audit
trio.

### Signature + four sub-checks

```
prism @epistemologic/property/docblock_no_extraction_pattern {
  focus docblock_no_extraction_pattern
  project docblock_no_extraction_pattern
  split docblock_no_extraction_pattern
  shift docblock_no_extraction_pattern
  settle docblock_no_extraction_pattern
}

docblock_no_extraction_pattern(d: docblock) -> verdict { \ }
```

Four sub-checks:

1. `no_unmarked_superlatives`.
2. `hedged_claims_marked_defer`.
3. `citations_content_match`.
4. `claim_size_matches_landing_size`.

### Audit test targets

1. `docblock_no_extraction_pattern_declares_prism`.
2. `docblock_no_extraction_pattern_declares_predicate`.
3. `docblock_no_extraction_pattern_names_four_subchecks`.

---

## §10. TICK 10 — `shards/kintsugi/fracture/docblock_extractive.mirror`

**Depends on TICK 9.** Operational half. NINTH #53 bilateral instance.

### Signature

```
docblock_extractive_body(d: docblock, ctx: kintsugi_context) -> ref
  requires ashby_variety_match(kintsugi_lock)
{ \ }
```

Routes to THREE surface classes based on sub-check:
- Sub-check 1 → `ashby_mismatch`.
- Sub-check 3 → `out_of_band`.
- Sub-check 4 → `contradiction`.
- Sub-check 2 → routes to `@epistemologic/liquid_extraction` for
  kind-reclassification.

### Audit test targets

1. `docblock_extractive_declares_glass`.
2. `docblock_extractive_declares_body`.
3. `docblock_extractive_requires_ashby_variety_match`.
4. `docblock_extractive_names_three_routing_classes`.

---

## §11. Cross-cutting: Tomm question shapes (unchanged from `doc-code-seam-shards.md` §Cross-cutting)

Each fracture body's spawn discharge emits a Tomm-shaped question at
reader-frame altitude per `@kintsugi/surface` spec §5:

- TICK 6 body → Tomm circular (§5.2) — name the ancestor.
- TICK 8 body → Tomm linear-then-reflexive (§5.3) — name two
  sites, reflect on meeting boundary.
- TICK 10 body → three Tomm shapes based on sub-check.
- **NEW**: TICK 4 body → Tomm reflexive-answerability (§5.4 +
  candidate extension per `liquid-types/README.md` §5.3): "Alex/Pack:
  does the shard's kind claim retain a route to being corrected by
  its own signals?" Fires on `≤ 2/5 agreement`. THIS IS THE FIRST
  TOMM QUESTION THE MACHINERY EMITS FOR REAL — at TICK 4 close.

---

## §12. Cross-cutting: circular-reflexive discipline (unchanged)

Per `63bdecc` §6: every shard's own docblock must survive its own
`audit(this_docblock, depth=3)`. At each landing tick:

- All ten shards' own docblocks land at self-audit verdict
  `both_survive`, NOT `real_survives`.
- Promotion of the two-channel discipline pending independent
  second witness.

---

## §13. Cross-cutting: substrate-honest verbatim usage

- All landed ancestors named at OID or spec-path.
- No new primitives invented.
- `ashby_variety_match(kintsugi_lock)` composed against landed
  `@epistemologic/cybernetic/coherence-parametric.ashby_variety_match`.
- `imperfect<applied, refused, ref>` return shape reuses landed
  `@kintsugi/*` fracture-body discipline.

---

## §14. What is the load-bearing FIRST landing?

**TICK 1 — `shards/docblock.mirror`.**

Why: it declares the `doc_claim` and `docblock` carriers that
everything downstream (including `liquid_extraction`) consumes as
their input type. Carrier-first ordering per
`[[feedback-no-bare-types]]`.

What TICK 1 enables:
- `doc_claim` becomes a resolvable type across the substrate.
- `docblock` becomes a resolvable type across the substrate.
- Downstream shards can reference `doc_claim` / `docblock` without
  redeclaring.
- The tokenizer at `ee7903e` has a landing target for the AST nodes
  it emits.
- Every subsequent tick's `requires docblock_well_audited(d)` clause
  resolves.

What TICK 1 does NOT yet enable:
- The auto-classifier is NOT operational (needs TICK 3+4).
- The docblock audit surface is NOT operational (needs TICK 5-10).
- The extractor is NOT operational (needs TICK 2 + body discharge).

Load-bearing = it's the precondition every downstream tick depends on.
NOT = it fires the classifier by itself.

---

## §15. The earliest tick that fires the auto-classifier for real

**TICK 3+4 close.**

At TICK 3, `computed_signals(file)` is declared. Its body `\` can
discharge via a Rust bootstrap helper OR via forward-promised
substrate discharge. Either way, TICK 4 (the fracture body)
consumes the signals and emits verdicts.

At TICK 4 close, running `classify(file)` against all 30 top-level
`shards/*.mirror` files returns the empirical discriminator result
per `bdb148a` §4 predicted table.

**Empirical discriminator run per `bdb148a` §4 procedure**:
1. Implement `computed_signals(file)` as a grep-verifiable procedure.
2. Run against all 30 top-level `shards/*.mirror` files.
3. Emit the auto-classifier verdict per shard.
4. Compare against Loki-ratified classification + @onto's ambiguity.
5. Non-empty disagreement set → Route B substrate-honest. Signal
   weight calibration if disagreement is uncalibrated.

The auto-classifier's verdict on @onto's placement (per `d6a05ad`
§3.3) becomes a first-class typed opacity map. THIS is the substrate
change that `liquid-types/README.md` §8 predicts as non-empty.

---

## §16. Where prism-kind's bilateral pair lands NATURALLY

**TICK 3+4.** Per `bdb148a` §7 deferred:

> "Shard landings for `@epistemologic/pact/prism_kind_declared` and
> `@kintsugi/fracture/prism_kind_ambiguous`."

Under this revised ordering, TICK 3+4 IS that landing tick. It sits
BETWEEN TICK 2 (liquid_extraction family-root) and TICK 5-10
(docblock-audit trio) because:

- Prism-kind CONSUMES `doc_claim` + `docblock` from TICK 1.
- Prism-kind does NOT need `liquid_extraction`'s body discharge
  (only structural signals from grep).
- Prism-kind PROVIDES structural signals that the docblock-audit
  trio (TICK 5-10) may consume as input to their sub-Turing
  decision procedures.

Bilateral count post-TICK 3+4:

- #53 instance 1: `@epistemologic/pact/keyword_matches_depth` +
  `@kintsugi/fracture/keyword` (LANDED 2026-06-10).
- #53 instance 2: `@epistemologic/pact/gate_matches_diff_closure`
  + `@kintsugi/fracture/gate` (LANDED 2026-06-16).
- #53 instances 3-5: syntax-substrate-native family (LANDED
  2026-06-19).
- #53 instance 5-parametric: `restart_intensity_well_formed` +
  `restart_storm` (LANDED `e7bd6ec` this month).
- #53 instance 5-routing: `@kintsugi/surface` routing-composition
  against `ashby_variety_match(kintsugi_lock)` (LANDED `e910dd6`).
- **#53 instance 6: `prism_kind_declared` + `prism_kind_ambiguous`
  (TICK 3+4, this revised spec)**.
- #53 instances 7-9: docblock trio (TICK 5-10).

Naturally emergent per Alex's direction: prism-kind IS the first
concrete USE of the doc-as-declaration machinery.

---

## §17. Where the first Tomm question the machinery emits fires for real

**TICK 4.**

At TICK 4 close, `classify(f)` runs against the 30 top-level shards.
The predicted verdicts per `bdb148a` §4 include at least four shards
with `≤ 2/5 agreement`:

- `@epistemologic` (Signal 2 weak).
- `@smarts` (Loki grin).
- `@loop` (Loki collapse candidate).
- `@onto` (candidate — 2/2/1 count per `bdb148a` §2.4).

Each fires the Tomm reflexive-answerability question:

> "Alex/Pack: does the shard's kind claim retain a route to being
> corrected by its own signals? Or has the shard's declared kind
> absorbed the signal-count opacity it should surface?"

This is the FIRST Tomm question the machinery emits for real
(not for spec-only reader-frame). Fires at TICK 4 close.

Per `liquid-types/README.md` §5.3, this is the **Tomm
answerable-shape** — a candidate fourth Tomm altitude for
`@kintsugi/surface` spec §5. Forward-promised for
`@kintsugi/surface`'s next amendment tick.

---

## §18. Composition dependency summary

```
TICK 1 (docblock)                                — precondition: tokenizer at ee7903e
   ↓
TICK 2 (liquid_extraction)                        ↓
   ↓                                              ↓
TICK 3 (prism_kind_declared) ← TICK 2 not-needed  ↓
   ↓                                              ↓
TICK 4 (prism_kind_ambiguous)                     ↓
   ↓                                              ↓
── auto-classifier operational ──                 ↓
                                                  ↓
TICK 5 (docblock_grounded)  ←─────────────────────┤
   ↓                                              │
TICK 6 (docblock_ungrounded)                      │
                                                  │
TICK 7 (docblock_coherent) ← TICK 2 REQUIRED ─────┘
   ↓
TICK 8 (docblock_incoherent)

TICK 9 (docblock_no_extraction_pattern)
   ↓
TICK 10 (docblock_extractive)
```

Bundles: (1), (2), (3, 4), (5, 6), (7, 8), (9, 10). Each of the
five bundles closes a #53 bilateral pair.

Landing atomicity: bundles CAN be committed atomically (both members)
OR sequentially (property first, fracture second per `[[feedback-
always-tdd-no-shortcuts]]` 🔴 → 🟢 pair). Reed's discretion at TDD
pair-tick altitude.

---

## §19. Empirical discriminator + composition-claim discipline

Per `[[feedback-composition-claims-need-empirical-test]]`:

**Discriminator run at TICK 4 close**:

1. Grep-implement `computed_signals(file)` (Rust helper OR
   substrate body discharge).
2. Run against all 30 top-level `shards/*.mirror` files.
3. Emit the auto-classifier verdict per shard.
4. Compare against `bdb148a` §4 predicted table.
5. Non-empty disagreement set → Route B substrate-honest.

**Discriminator run at TICK 10 close**:

1. Run `audit_docblock(d)` against every shard's docblock.
2. Compare against verdict predicted in `liquid-types/README.md`
   §8 table.
3. Non-empty disagreement set → doc-code seam substrate-honest.

Both runs are load-bearing empirical tests. Neither has fired yet;
the discipline is spec-only until TICK 4 + TICK 10 close.

---

## §20. What is DEFERRED (revised)

- Tokenizer change LANDED (was DEFERRED at `20c99a2` §6.1).
- `extract_predicate` body — forward-promised at TICK 2 landing;
  body discharges via `splinter(@epistemologic/property/ast)` at
  the property altitude.
- `computed_signals` body — forward-promised at TICK 3 landing.
- Empirical discriminator runs at TICK 4 + TICK 10 close.
- Signal-weight calibration based on discriminator run.
- Whether the `prism_kind` axis has 3 or 4 values (form/process
  split per #55 candidate).
- Full cross-altitude projection composition mechanics.
- Kintsugi loop empirical composition with runtime pipeline.

---

## §21. Substrate references

- `docs/math/liquid-types/README.md` (this tick — load-bearing math).
- `docs/math/kintsugi/doc-code-seam.md` (`20c99a2`) — canonical
  compiler-fit doc.
- `docs/math/the-tower/projection-surface.md` (`63bdecc`).
- `docs/math/prism-kind/README.md` (`bdb148a`) — first concrete USE.
- `docs/math/onto/README.md` (`d6a05ad`) — @onto grounding.
- `docs/specs/doc-code-seam-shards.md` (2026-07-04) — the eight-shard
  spec this doc revises.
- `bootstrap/src/tokenize.rs` (`ee7903e`) — Docblock AST nodes.
- `bootstrap/tests/kintsugi_surface_shard.rs` — text-check discipline
  pattern.
- `[[architecture-property-fracture-bilateral]]` (#53).
- `[[architecture-kintsugi-loop-altitude-portable]]` (#59).
- `[[architecture-form-process-partition-at-family-root]]` (#55).
- `[[architecture-hilbert-turing-godel-recognition-107]]` (#107).
- `[[feedback-substrate-already-had-the-word]]` (fifteenth+).
- `[[feedback-craft-not-deliver]]`.
- `[[feedback-composition-claims-need-empirical-test]]`.
- `[[feedback-no-bare-types]]`.
- `[[feedback-write-red-in-session]]`.
- `[[feedback-always-tdd-no-shortcuts]]`.
- `[[feedback-legibility-over-foundation-when-collapsing]]`.

*2026-07-05. Mara. Derived spec. Bottom-up landing sequence. Revises
`docs/specs/doc-code-seam-shards.md` per Alex 2026-07-05 direction.
Actionable TDD roadmap: 10 ticks, 5 bilateral pairs, prism-kind
emergent at TICK 3+4 as the sixth #53 instance. Load-bearing FIRST
landing: TICK 1 (`shards/docblock.mirror`). First tick that fires
the auto-classifier for real: TICK 4. First Tomm question the
machinery emits: TICK 4 close.*
