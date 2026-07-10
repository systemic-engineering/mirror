# Seam Phase D — Self-migration via spectral typing (ratification)

**Date**: 2026-07-10 · **Author**: Seam <seam@systemic.engineer>
**Anchors**: Mara iter-20 (`e764a32`, canonical spec), Taut iter-11
(`8ac250e`, boundary scout), Mara iter-17/18/19 (`beef270`/`129f618`/
`78d5110`), Taut iter-10 (`8ac250e`). Read-only; pure-📝 audit.

## Executive summary

- **Q1 (LRM verdict β)**: RATIFY. Three PREREQs are localised and grep-clean.
- **Q2 (empirical-witness cost estimate)**: RATIFY-WITH-QUALIFICATIONS.
  Cost is genuinely low; the "17 shard hits" in Mara §1 is inflated and
  contradicts Taut's zero-consumer count. Correct the spec.
- **Q3 (grammar vs prism vs spectral)**: RATIFY-WITH-QUALIFICATIONS.
  Spec and scout ALREADY defer `spectral @foo` to a follow-up (Tick 6+).
  Migration preserves `grammar @foo`; no keyword conversion at Tick 5.
- **Q4 (unmanned loop claim)**: RATIFY-WITH-QUALIFICATIONS. The unmanned
  claim requires oscillate's `active_pass` to accept `relocate` species
  AND @io to apply the atomic multi-file settle. Not landed today.
- **Q5 (fault-planes Taut missed)**: RATIFY. One qualification only —
  the bundle+lawvere+spectral-triple triad needs explicit residual-set
  handling; §7 S5 names it but Tick 5's discharge does not test it.
- **Q6 (Foerster-eigenform closure)**: RATIFY-WITH-QUALIFICATIONS. The
  first landing does NOT hold structurally; it holds by declaration.
  Iteration to eigenform is a follow-up tick, not Tick 5.
- **Q7 (Tick ordering)**: RATIFY-WITH-CORRECTION. **PREREQ-1 must land
  before PREREQ-2**, not after. Taut's "PREREQ-2 first" recommendation
  reverses the dependency.

**Overall verdict**: **RATIFY-WITH-QUALIFICATIONS**. The five-tick
minimum-cut lands cleanly if PREREQ-1 (bootstrap) precedes PREREQ-2
(relocate species), the "17 consumers" number is corrected to zero,
and the Foerster-eigenform + unmanned-loop claims are marked as
follow-up ticks not landed by Tick 5.

---

## §Q1 — LRM verdict β (PREREQ-1/2/3 landability)

**PREREQ-1 (3-line bootstrap delta)**: RATIFY. Grep on
`bootstrap/src/lib.rs::collect_declared_namespaces` (lines 2142-2151)
confirms three `strip_prefix` arms (`"glass "`, `"prism "`, `"grammar "`).
Admitting `"spectral "` is a single additional `else if` arm — three
lines including the arm and the fallthrough. No other call site
matches the same hard-coded prefix set; cascade check clean.

**PREREQ-2 (@kintsugi/fracture/relocate)**: RATIFY. Existing fracture
species (`gate.mirror`, `keyword.mirror`, `symbol_lift.mirror`,
`partials_align.mirror`, `docblock_*`) all follow the bilateral
`property + fracture_body` pattern. Taut §3 correctly identifies
`splinter(ast)`'s single-file limitation; widening to
`(source_ref, target_ref, [import_rewrite])` is a substrate-pull-clean
extension. No prior art for the transactional multi-morphism settle;
substrate genuinely does not carry it today.

**PREREQ-3 (@epistemologic/pact/is_load_bearing_in_std)**: RATIFY.
`shards/epistemologic/pact/substrate_source_in_shards.mirror` (7.0KB)
is close prior art: it predicates on `shards/` vs `boot/std/`
provenance. The load-bearing predicate is genuinely new (needs the
consumer-count grep), but the pact family has the shape for it.

## §Q2 — Empirical-witness cost estimate (Tick 5 discharge)

Mara §1 claims "17 shards + boot files declare `in @epistemologic/
math/spectral-triple`". This is **not correct**. My grep for
`^\s*in\s+@epistemologic/math/spectral` across shards/**/*.mirror
and boot/**/*.mirror surfaces **zero** direct `in @` consumers of the
declared name. Taut §5 has the correct count (0 non-dangling, 1
dangling).

The dangling reference at `shards/epistemologic/cybernetic/coherence-parametric.mirror:8`
(`in @epistemologic/math/connes_spectral_triple`) is **not** a hint of
prior migration motion. The shard's header explicitly cites
`[[architecture-spectral-triples-all-the-way]]` and Connes 1985; the
`connes_spectral_triple` import is a semantically-motivated forward-
promise, not a legacy alias. Migration should EITHER declare
`@epistemologic/math/connes_spectral_triple` as the migrated name
(closing the dangle), OR rewrite the import to
`@epistemologic/spectral_triple` post-elevation. Taut §5's proposal
(rewrite atomically via `relocate`'s import_rewrite set) is the
right call.

Migration preserves byte-identity of the shard's `grammar {}` body;
only the header `grammar @<path>` and terminal `out @<path>` lines
change (two-token surface rewrite). No ancestry rewrites in the shard.

**Correction needed**: Mara §1 sentence "every existing `in
@epistemologic/math/spectral-triple` (currently 17 shard hits per
§grep) continues resolving" is factually wrong. There are 0 direct
consumers. The "17" appears to conflate direct `in @` consumers with
doc-string / prose backlinks (40+ per Taut §5). Correct to: "0 direct
`in @` consumers; 40+ prose backlinks handled by follow-up doc cascade".

## §Q3 — `grammar` vs `prism` vs `spectral` declaration form

`spectral-triple.mirror` declares `grammar @epistemologic/math/
spectral-triple` (line 57). Grep against `^\s*grammar @` at boot/std
finds ~30 such shards (grammar is the substrate-native form for
abstract-action-carriers with type-only bodies; prism is for
five-op-record species; glass is for realised value carriers).

Mara §2 admits `spectral @foo` as a NEW declaration head sitting
between glass and prism. But **§8 explicitly defers this to Tick 6+**
("Explicitly NOT in the minimum-cut … `spectral @foo` as new
declaration head"). Tick 5's discharge does NOT rewrite `grammar` →
`spectral` — it preserves `grammar @epistemologic/math/spectral-triple`
as `grammar @epistemologic/spectral_triple`. This is substrate-honest:
migration preserves the keyword; the keyword upgrade to `spectral` is
a separate additive tick.

**Correction**: Mara §6 claims post-migration `spectral @spectral_triple`
IS declared as a spectral triple. This does not hold at Tick 5 (the
declaration remains `grammar @epistemologic/spectral_triple`). The
Foerster-eigenform closure requires the follow-up `spectral @foo`
tick to actually land. See §Q6.

## §Q4 — The "unmanned loop" claim

The oscillate loop (`shards/kintsugi/oscillate.mirror`, 40KB) is
substrate-declared but the `active_pass` semantics per §3 of Taut's
scout iterate opacities one-by-one via single-morphism settle. The
loop CAN walk boot/std autonomously ONCE:

1. `@kintsugi/fracture/relocate` species is landed (PREREQ-2), AND
2. `active_pass` accepts the multi-file `relocate` morphism shape
   (which is a widening of the existing `splinter(ast)` — Taut §3
   flags this as new), AND
3. @io realises the atomic multi-file settle (write source-target
   mv + N import rewrites transactionally — not currently declared;
   `@io/git` sub-family carries atomic-commit but not atomic multi-
   file source-rewrite as one operation).

**Adversarial reading**: Alex's "no agent needed" claim requires all
three preconditions. Tick 5 discharges the FIRST migration (spectral-
triple) but does not test unmanned iteration on a second shard. The
"unmanned" claim is TRUE post-Tick-5-plus-one — after the first
witness closes and the loop's `active_pass` executes the second
shard's `relocate` morphism without operator intervention. Tick 5 is
NOT the unmanned witness; it is the machinery witness.

**Correction**: Mara §5's "no agent needed" claim should be scoped to
"post-Tick-5, subsequent shard migrations run unmanned via consumer-
pull". Tick 5 itself is operator-driven (Reed runs the loop; the
loop selects spectral-triple as the first witness).

## §Q5 — Fault-planes Taut may have missed

**Oscillate's `active_pass` walk semantics**: Taut §3 correctly
identifies the single-file limitation. My skim of oscillate.mirror
confirms the loop is declared as ACTIVE/DARK alternation, driven by
opacity-map iteration — one opacity per pass. Widening to relocate's
multi-file morphism is per-morphism, not per-loop; the loop
substrate does not need modification.

**boot/std composition**: 134 `.mirror` files across ~15 sub-trees.
Not all are `grammar @`; many are `prism @` (mirror/glass/ast/shape/*
carriers), some are `glass @` (property realisers). The load-bearing
predicate will need to discriminate BOTH by consumer count AND by
declaration-form-appropriateness (`grammar` shards elevate to
epistemologic; `prism`/`glass` shards elevate to whatever their
family-root's altitude names). Not just consumer-count.

**Bundle+lawvere+spectral-triple triad (Mara §1 fault-plane 2)**:
Mara names the fracture risk but Tick 5 does NOT test triad-atomicity.
`bundle.mirror` and `lawvere.mirror` remain in `boot/std/epistemologic/
math/` after Tick 5. Mara §1's argument that the split is honest
(bundle+lawvere read the triple; not vice-versa) is a claim Tick 5
does not verify. The property `is_load_bearing_in_std`'s verdict on
bundle+lawvere will drive the follow-up ticks; if the load-bearing
predicate surfaces them as migrate-worthy, the triad split becomes
a run-time question.

**Cascade check on `collect_declared_namespaces`**: no other Rust
code path hard-codes the `"glass "`/`"prism "`/`"grammar "` prefix
set. The 3-line delta is genuinely 3 lines.

## §Q6 — Foerster-eigenform closure

Mara §6 claims `spectral @spectral_triple` at the family-root of
substrate typing IS declared as a spectral triple — the self-
referential ground.

**Adversarial**: This does NOT hold at Tick 5 landing. Tick 5
preserves the declaration as `grammar @epistemologic/spectral_triple`
(§Q3). The `spectral @foo` head is Tick 6+ (Mara §8 explicit
deferral). The eigenform closure requires:

1. `spectral @foo` grammar admittance (deferred).
2. `spectral @spectral_triple { ... }` rewrite of the shard's own
   declaration (not in Tick 5).
3. Structural (A, H, D) obligations discharged at declaration time
   (per Mara §2 "witnesses discharge structurally").

At Tick 5 the shard is a `grammar` shard declaring the (A, H, D)
types abstractly. It does not witness the axioms of its own
declaration form. The eigenform closes when the declaration form IS
the type it declares — that is a strictly later tick.

**Correction**: Mara §6's second paragraph ("Post-migration:
`spectral @spectral_triple` at shards/epistemologic/spectral_triple.
mirror IS declared as a spectral triple") should be marked as
FORWARD-PROMISED to the Tick 6+ `spectral @foo` grammar admittance,
not achieved at Tick 5.

## §Q7 — Ordering of PREREQ-1/2/3

Taut §6 recommends "PREREQ-2 first (Mara canonical)". This is
**wrong**. PREREQ-1 (bootstrap 3-line delta) LETS the corpus scanner
SEE `spectral @foo` declarations. Without PREREQ-1:

- If PREREQ-2 lands `@kintsugi/fracture/relocate` as a `glass @` or
  `prism @` declaration (per current fracture-species convention),
  fine — but then it does NOT use `spectral @` and PREREQ-1's
  admittance is not tested until later.
- If PREREQ-2 lands `@kintsugi/fracture/relocate` as
  `spectral @kintsugi/fracture/relocate` (per Mara §2's admittance),
  and PREREQ-1 has not landed, the declaration is INVISIBLE to
  `collect_declared_namespaces` (falls through the strip_prefix
  chain) — dark_count += 1, and consumer resolution fails.

**Correction**: Tick order MUST be:
- **Tick 2: PREREQ-1** (3-line bootstrap delta) — first, to
  validate that `spectral @foo` parses.
- **Tick 3: PREREQ-2** (`@kintsugi/fracture/relocate`) — can now
  declare as `spectral @kintsugi/fracture/relocate` if desired, OR
  keep as `glass @` per fracture convention (Mara chooses).
- **Tick 4: PREREQ-3** (`@epistemologic/pact/is_load_bearing_in_std`).
- **Tick 5: Discharge** (migrate spectral-triple.mirror).

Mara's spec §8 ALREADY has this order right (Tick 2 = PREREQ-1, Tick
3 = PREREQ-2). Taut's §6 recommendation to Reed ("Land PREREQ-2
first") CONTRADICTS Mara's landing sequence. Reed should follow
Mara §8's order, not Taut §6's recommendation.

---

## §Corrections needed

**Mara e764a32 (spec) corrections**:

1. **§1 fault-plane 1**: correct "17 shard hits" → "0 direct `in @`
   consumers; 40+ prose backlinks handled by follow-up doc cascade".
2. **§5**: scope "no agent needed" to "post-Tick-5 subsequent
   migrations"; Tick 5 itself is operator-driven.
3. **§6**: mark the second paragraph (Foerster-eigenform closure at
   post-migration `spectral @spectral_triple`) as FORWARD-PROMISED to
   the Tick 6+ `spectral @foo` grammar admittance; Tick 5 preserves
   `grammar @epistemologic/spectral_triple`.

**Taut 8a3b0a4 (scout) corrections**:

1. **§6 "Recommended next action"**: retract "Land PREREQ-2 first";
   PREREQ-1 must precede PREREQ-2 per the grammar-admittance
   dependency. Reed follows Mara §8's Tick 2/3/4/5 order.

## §Landing sequence adjudication

Confirmed multi-tick landing sequence:

- **Tick 1**: This spec + this audit landed 📝. (Complete post-audit.)
- **Tick 2 (PREREQ-1, Reed)**: 3-line delta at
  `bootstrap/src/lib.rs::collect_declared_namespaces` admitting
  `"spectral "` prefix. RED-first via
  `bootstrap/tests/spectral_keyword_admittance.rs`.
- **Tick 3 (PREREQ-2, Mara)**: `@kintsugi/fracture/relocate` species
  (whole-shard atomic mv + import-rewrite morphism, widening
  splinter(ast)).
- **Tick 4 (PREREQ-3, Mara)**: `@epistemologic/pact/is_load_bearing_in_std`
  predicate.
- **Tick 5 (Discharge, Reed drives, loop discharges)**: migrate
  `boot/std/epistemologic/math/spectral-triple.mirror` →
  `shards/epistemologic/spectral_triple.mirror`. `dark_count`
  decreases by delta. Loop closes on its own math (machinery witness,
  not unmanned witness).
- **Tick 6+ (consumer-pull, deferred)**: `spectral @foo` grammar
  admittance; Foerster-eigenform closure; second-shard migration
  runs unmanned (unmanned witness); bundle+lawvere+spectral-triple
  triad ratification per §7 S5.

**Verdict**: RATIFY-WITH-QUALIFICATIONS. The minimum-cut is sound;
the ordering is Mara's, not Taut's; the empirical claims need the
three corrections above.

---

*— Seam, 2026-07-10. Phase D closes with three corrections and the
tick-order corrected.*
