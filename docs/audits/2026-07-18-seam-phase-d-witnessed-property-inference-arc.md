# Seam Phase D audit — witnessed property inference arc (Mara `cf34549` + Reed pillar iter 1-10)

*Seam, 2026-07-18. Phase D adjudication of the witnessed-property-inference arc:
Mara commit `cf34549` (three docs — math root, math addendum, canonical spec)
grounded by Reed's iter 1-10 pillar composition surface (six primitives, 98 tests,
canonical spec `18ef3b8`) and Taut's scout `058b892`. Alex Q1-Q10 adjudication
cache reviewed against the landed spec. Twelve dimensions D1-D12 scored;
recommendation §13.*

*Discipline: adversarial. Substrate-honest. Cite commits, line numbers, specific
files. Refuse hand-wavy praise. Rice-safe verdict grading.*

---

## §0 Executive summary

**Verdict: SHIP-WITH-REED-INLINE.**

The four-vertex composition (Traces × Petri × Fate × Properties) is mathematically
sound at the composition altitude. The five-arc landing plan is well-ordered and
RED-first-compatible. The refused-mint inventory (≥21 words) is substrate-honest
and complete. Ten Recognition candidates are well-scoped. The prior art gap
(§5 math root, §7 math addendum) is precisely named.

**But the substrate does not hold at four load-bearing citations.** Reed-inline
cascade required before Reed touches Arc 1:

1. **Math root §2.1 fabricates two `signature_beat` fields** (`witness: subject_instance`
   and `content_oid`) that do not exist in `shards/spectral/signature.mirror:107-115`.
   Actual field is `ssh_fingerprint: ref` (git-altitude witness) + `contribution_oid`
   (content hash). The load-bearing composition claim ("A token at a place is a
   signature_beat at that place's altitude", spec §3.1) then reads onto a
   fabricated shape.

2. **Spec §5.1 mis-cites the cache location.** Spec says
   `shards/mirror/store.mirror:344-368` — but the `derived_predicates` field
   lives at `shards/mirror/store/crystal.mirror:356` (a different file). Math
   root §4.3 has the same drift with the wrong line range on the right file.

3. **Alex Q5 cascade incomplete.** Spec §11.5 says "Taut Q5 (still open) —
   Refuse `#[proptest]` macro layer" citing "Taut scout §5.4". Taut §5.4 is
   about Targeted PBT (Löscher-Sagonas); Taut's actual Q5 (§7.5) asks about
   SHRINKER location, not macros. Alex ratified proc-macro test-body layer YES
   per Reed memory `feedback_prismqueer_macros_mirror_composes` — the spec
   still says "spec's lean: REFUSE per Taut scout §5.4 + math root §8". Math
   root §8 refused-name table still lists `#[proptest]` under "Rejected".
   Alex's correction cascade did not land.

4. **Alex Q10 cascade incomplete.** Alex ratified `@mirror/store/liquid` as a
   NEW species-decl mint for the verified-cache location (per Reed memory
   `project_witnessed_property_inference` Q10). Spec §11.10 still says
   "Spec's lean: (a) `@mirror/store.crystal.derived_predicates`" and §5.1-§5.3
   describe the cache-write-back path as going to
   `@mirror/store.crystal.derived_predicates`. Alex's Q10 ratification
   supersedes the spec's lean; the spec §5 + §11.10 need cascade update.

All four cascades are ~200-400 LOC of markdown surgery across the three docs.
Reed-inline scope. Arc 1 (`pillar::of_health` + Fate bridge) can then land
against a substrate that holds.

**One ALEX-Q: Recognition candidate on Petri boundedness upgrade.** The math
addendum §6.3 + §2.2 promotes Karp-Miller 1969 coverability to the
"provable full-coverage" claim — this is a genuinely novel composition rung
that surfaces four Recognition candidates (R2-R5 of the addendum) but math root
§9's original five R#1-R#5 do NOT include it. Ask: should the addendum's R4
(`#R-petri-boundedness-guarantees-pbt-coverage`) be added to the joint arc's
Recognition candidate set?

**Dimension table:**

| Dim | Verdict | Notes |
|-----|---------|-------|
| D1  | SHIP-WITH-REED-INLINE | Fixed-point equation grounded; Karp-Miller upgrade needs Recognition folding |
| D2  | SHIP-WITH-REED-INLINE | Composition sound; `signature_beat` field fabrication breaks §3.1 |
| D3  | SHIP | ≥21 refused-mint inventory verified; no missed carriers |
| D4  | SHIP | `<primitive>_of_<input-shape>` conforms across all mints |
| D5  | SHIP | Pillar I Rice-safety preserved; Petri firing-policy is decidable |
| D6  | SHIP-WITH-REED-INLINE | Cache location citation stale (Q10 cascade) + wrong-file citation |
| D7  | SHIP | Four-vertex square composes; each edge verified |
| D8  | SEAM-REJECT | Q5 + Q10 cascades incomplete; specific line-changes below |
| D9  | SHIP | Five-arc ordering sound; Rice-safe; RED-first-compatible |
| D10 | ALEX-Q | Petri boundedness Recognition candidate not in joint set |
| D11 | SHIP-WITH-REED-INLINE | Vocabulary drift on Q5 attribution and cache location |
| D12 | SHIP | Four-vertex delights; next Reed picks it up; critic-resistant modulo cascades |

**Recommendation (§13): (a) Reed inline cascade first, then (c) Arc 1 land.**
The inline cascade is ~200-400 LOC across three docs and unblocks Arc 1's
substrate ground; skipping it means Arc 1 lands on a spec with stale Alex
ratifications (Q5 + Q10) and a fabricated `signature_beat` field.

---

## §1 D1 — Math ↔ Spec coherence

**Verdict: SHIP-WITH-REED-INLINE.**

The fixed-point equation of math root §2.2 (`w(t) := witness_of(t) = verdict(t)
∘ commit_of(observe(t), Fate::tick(observe(t)))`) is surfaced in spec §5 as an
operational primitive:

- Spec §5.1: cache key = `(spec_oid, target_oid, inputs_oid)` — is the operational
  form of §2.4's "witness_of(t) is a total function of (weights_oid, features_oid,
  t_oid)". Reads coherent.
- Spec §5.2-§5.3: idempotent-closure proof (math root §4) discharged operationally
  as "cache validity holds by construction; no invalidation logic needed" — sound.
- Spec §6.4 restates the harness/SUT collapse theorem operationally; math root §6.1
  formalizes it. Coherent.

**Karp-Miller boundedness upgrade in the addendum**: math addendum §6.3 lifts the
Petri boundedness (Karp-Miller 1969) into a "full state-space coverage in the
limit" theorem — this is the STRONGEST claim in the arc, and it is genuinely
novel-at-composition. Spec §3.3 folds this in ("STRONGER guarantee than any
Turing-PBT convergence claim") — sound.

**Objection:** math root §9's Recognition candidate set does NOT include the
Karp-Miller boundedness upgrade as a candidate. Math addendum §10 R4
(`#R-petri-boundedness-guarantees-pbt-coverage`) surfaces it, but the two docs'
Recognition sets are disjoint (math root R#1-R#5 vs. math addendum R1-R5). See
D10 for the ALEX-Q on whether the joint arc should ratify a unified set.

**Composition sound**; Reed-inline needed only for the D10 recognition-set
alignment (see §10).

---

## §2 D2 — Composition alignment (compose over LANDED substrate)

**Verdict: SHIP-WITH-REED-INLINE.**

Verified per-carrier:

- **`@mirror/petri`** (spec §2.3): substrate-decl shape correct per
  `docs/specs/subject-family-root-sel-licensable-party.md#1197-1218`. The
  `type petri_net = { places, transitions, tokens: [sel], firing_rules }` carrier
  is preserved verbatim. Spec §3.1 respects this — the token generalization to
  "any type with `emit_oid: oid`" is an admissible shape claim over the existing
  substrate-decl (Q7 lean). SHIP.

- **`fragmentation::Witnessed`** (spec §2.6, §5, §4): composed over
  content-address primitive without extension. The "different witness, different
  commit; same content, same tree" discipline is discharged at claim altitude.
  SHIP.

- **`Fate::tick`** (spec §2.2, §4): stays within the 5-model selector surface.
  The seam composes as UTILITY VERB (`bias_of_marking`, §7.4) over Fate, NOT as
  new Fate method — Reed's "no Rust extension shortcut" discipline preserved.
  Verified against `fate/src/lib.rs:411` where `tick` is defined. SHIP.

- **`pillar::*`** (spec §2.1, §9): the six landed primitives at
  `prismqueer::liquid::pillar` (dispatch_ambiguity, algedonic,
  algedonic_of_magnitude, viability, viability_of_magnitudes, fold) are unchanged.
  Arc 1 lands `of_health` as parallel seventh; Arc 2 lands `Sample`, `Arbitrary`,
  `forall` — all conform to the composition surface Reed established in iter
  1-10. SHIP.

**Objection (SHIP-WITH-REED-INLINE):** math root §2.1's `signature_beat`
substrate-decl rendering is fabricated. The doc claims:

```
signature_beat = {
  witness:         subject_instance,
  sc_at_beat:      SpectralCoordinate<5>,
  previous_beat:   option<oid>,
  rung:            rung_id,
  content_oid:     oid,
}
```

Actual per `shards/spectral/signature.mirror:107-115`:

```
type signature_beat = {
  contribution_oid:  oid,
  sc_at_beat:        SpectralCoordinate<5>,
  rung:              @song/beat.rung,
  previous_beat:     option<oid>,
  timestamp:         @time/monotonic.instant,
  ssh_fingerprint:   ref,
  address:           uuid_spectral_time,
}
```

Two claimed fields do not exist:
- `witness: subject_instance` — the ACTUAL witness field is `ssh_fingerprint: ref`
  (git-altitude SHA256 of SSH pubkey per shard docblock :86-89). Semantically
  close to Mara's claim ("different observer, different commit") but structurally
  distinct.
- `content_oid: oid` — the ACTUAL content field is `contribution_oid: oid`.

Load-bearing consequence: spec §3.1's claim "A token at a place is a
signature_beat at that place's altitude" reads onto a fabricated shape. The claim
survives at semantic altitude (both fields carry the concept) but any Reed
downstream landing that says `beat.content_oid` or `beat.witness` will fail
against the substrate.

**REED-INLINE cascade required:** math root §2.1 (substrate-decl block) needs
correction to match actual `signature_beat` shape. Two suggested edits:

```
old_string (math root §2.1 lines 124-132):
signature_beat = {
  witness:         subject_instance,
  sc_at_beat:      SpectralCoordinate<5>,
  previous_beat:   option<oid>,
  rung:            rung_id,
  content_oid:     oid,          // observer-independent
}

new_string:
signature_beat = {
  contribution_oid:  oid,               // observer-independent content hash
  sc_at_beat:        SpectralCoordinate<5>,
  rung:              @song/beat.rung,
  previous_beat:     option<oid>,
  timestamp:         @time/monotonic.instant,
  ssh_fingerprint:   ref,               // git-altitude witness (per :86-89)
  address:           uuid_spectral_time, // uuid_spectral + time facet
}
```

And in spec §3.1 lines 251-258 the "witness/content_oid" field references need to
be re-mapped to `ssh_fingerprint`/`contribution_oid` — the composition claim
holds; the field names must match substrate ground truth. See D8 for the full
REED-INLINE table.

---

## §3 D3 — Substrate-already-had-the-word

**Verdict: SHIP.**

The ≥21-word refused-mint inventory across math root §1 + math addendum §8 +
spec §7.6 was independently verified:

- `witness`, `witnessed`, `trace` — landed carriers cited correctly
  (`signature_beat`, `Witnessed`, `verdict_is_content_addressed`).
- `sample` — `sample_pain` + `sample_pleasure` at
  `shards/epistemologic/cybernetic/algedonic.mirror` verified (3 hits per grep).
- `arbitrary`, `forall`, `choice_sequence` — grep-verified as English-only in the
  substrate (no shard carrier). Admissible mint at operational altitude only per
  the math root §1 discipline.
- `generator`, `Gen`, `Strategy` — `@fate.roll` verified as the substrate's
  typed generator (`shards/fate.mirror`).
- `shrinker`, `Shrink` — no substrate hits; correctly refused at species
  altitude.
- `HolonomyHealth`, `PropertyVerdict` — verified in `fate/src/lib.rs:100` and
  `imperfect/src/transparency.rs` respectively.
- `@petri`, `@marking`, `@transition`, `@token`, `@firing_policy` — all Petri-
  vertex mints correctly refused; the `@mirror/petri` family-root spec at
  `subject-family-root-sel-licensable-party.md#1197-1218` carries them via
  `type petri_net` fields.
- `@stigmergy`, `@pheromone`, `@ant` — Grassé-lineage lexicon correctly refused
  in favor of existing `@kintsugi/roomba` + `signature_beat`.

**No missed carriers found.** The inventory is complete at grep-first altitude.

**One minor note:** the math root §1 table has `HolonomyHealth` listed as
`fate::HolonomyHealth` — actual location per grep is `fate::feature::HolonomyHealth`
(re-exported as `use feature::HolonomyHealth` at top-level). Not a REED-INLINE
item; naming convention is fine at the citation altitude.

SHIP unconditionally.

---

## §4 D4 — Delightfully-boring naming

**Verdict: SHIP.**

Every proposed primitive conforms to Alex-ratified `<primitive>_of_<input-shape>`
per memory `feedback_composition_primitive_naming_convention`:

- `pillar::of_health` — of_health = verdict of a HolonomyHealth. Conforms.
- `bias_of_marking` — bias of a marking. Conforms.
- `bias_sample_of_features` — bias-sample of features. Conforms.
- `commit_of` — commit of. Conforms (parallels `sample_of`).
- `sample_of` — sample of. Conforms.
- `witness_of` — witness of. Conforms.
- `verdict_of` — verdict of. Conforms.
- `bias_of_features` — bias of features. Conforms.

**Refused-suffix list (spec §8, math root §8):** `Arbitrary` type-class (Haskell
legacy), `Strategy` monad (proptest legacy), `#[proptest]` macro, `Shrink` trait
(QuickCheck legacy), `Range<T>` (Hedgehog legacy), `Gen a` (functor legacy).

**Refuse trigger reviewed per row.** All rows sound EXCEPT `#[proptest]` (see D8
Q5 cascade — Alex ratified YES, spec still lists refuse).

`pillar::Sample`, `pillar::Arbitrary`, `pillar::forall` use the SAME words
QuickCheck/Hypothesis/proptest use — the refuse of `Arbitrary` type-class is
about avoiding the type-class LAYER, not the word itself. Spec §7.1 correctly
lands `Arbitrary` as a TRAIT (not a type-class); this reading is coherent.
Naming is fine.

SHIP.

---

## §5 D5 — Rice-safety

**Verdict: SHIP.**

Pillar I `dispatch_ambiguity` (Rice-safe byte-visible per
`docs/specs/prismqueer-liquid-pillar-composition-surface.md#§2.1`) is preserved:
the four-tuple `(arm_count, witness_count, tie_breaking_exhausted, ...)` is
byte-visible input; `dispatch_ambiguity` reads none of them via
halting-problem-hard predicates.

**New Petri-native primitives:**

- `pillar::of_health` — takes `&HolonomyHealth` (a scalar) and two thresholds.
  Byte-visible input, byte-visible output. Rice-safe. SHIP.
- `bias_of_marking` — takes `&Fate, &Marking, depth: f64`. `extract_features_of_
  marking` extracts `[f64; 16]` from the marking's byte-content (no runtime
  reflection needed if marking is content-addressed). Rice-safe by construction.
- `pi_Fate(t | M)` firing-policy dispatch (math addendum §3.2 + spec §4.1) —
  the distribution division is arithmetic on `[f64; 5]` and the enabled-transition
  set is a decidable predicate on the Petri net (Karp-Miller 1969). Rice-safe.
- Karp-Miller coverability tree — decidable per Reisig 2013 §5.3 boundedness
  theorem. Not Rice-adjacent.

**No place in the new primitive surface requires halting-problem-hard analysis.**
Every dispatch is either arithmetic, byte-buffer read, or Petri-net decidable
query.

**One caution:** spec §9.5 Arc 5 says "K-parallel roombas converge on the same
terminal position (ensemble eigenbehavior)". Convergence of K-parallel walks is
NOT Rice-safe in general (halting reduces to it for arbitrary walkers). Since
the roomba walker is BOUNDED (`walk(from, budget)` per
`shards/kintsugi/roomba.mirror:66`), convergence is decidable at bounded budget.
Spec should note this bound explicitly in Arc 5's docblock to preserve the
Rice-safety guarantee — minor Reed-inline addition, not a blocker.

SHIP.

---

## §6 D6 — Content-addressability + cache invalidation

**Verdict: SHIP-WITH-REED-INLINE.**

The cache key `(spec_oid, target_oid, inputs_oid)` composition (spec §5.2)
correctly composes over landed
`@epistemologic/property/verdict_is_content_addressed` (Reed N1 2026-07-12) —
that shard's total-function claim is verified verbatim at
`shards/epistemologic/property/verdict_is_content_addressed.mirror:34-45`. Cache
validity holds by construction; SHIP at the composition-primitive altitude.

**Objection 1 (wrong file cited):** spec §5.1 states cache lives at
`@mirror/store.crystal.derived_predicates` per
`shards/mirror/store.mirror:344-368`. Grep confirms:

- `shards/mirror/store.mirror` (46.5KB) does NOT contain
  `derived_predicates` — verified via search.
- The field IS at `shards/mirror/store/crystal.mirror:356` (a different file,
  the `type crystal` block at :343-369).

Math root §4.3 has similar drift — cites `shards/mirror/store/crystal.mirror:
344-368` — right file, but the exact range is `:343-369` (the `type crystal`
block bounds) and `derived_predicates: [property_verdict]` is at line 356.

**REED-INLINE cascade:** update spec §5.1 to cite
`shards/mirror/store/crystal.mirror:356` (or `:343-369` for the whole crystal
carrier). Same fix in math root §4.3.

**Objection 2 (Q10 supersedes spec's lean):** Alex ratified Q10 with
`@mirror/store/liquid` as a NEW species-decl mint composing `@mirror/store` +
`@liquid` family-root (per Reed memory `project_witnessed_property_inference`
Q10). Spec §11.10 has this as an open question with "spec's lean: (a)
`@mirror/store.crystal.derived_predicates`". Alex's Q10 answer is (d) — a new
species-decl mint.

**REED-INLINE cascade required:** spec §5.1-§5.3 need re-write to route cache
write-back through `@mirror/store/liquid` (new species-decl to be minted). Spec
§11.10 should be re-marked as RATIFIED with Alex's answer, and §12.1 forward-
promises should list "author `@mirror/store/liquid` species-decl" as the new
cascade dependency (which precedes Arc 4).

This is a significant re-write of §5 (cache section) plus a new species-decl
authorship task. Per Alex's Q10 ratification, this composition-surface expansion
is load-bearing — the spec must reflect it before Reed lands Arc 4.

See D8 §8 for the specific line-changes.

---

## §7 D7 — Petri × Fate four-vertex square well-formedness

**Verdict: SHIP.**

Each edge verified for well-formedness:

- **Traces ↔ Petri** (math addendum §2.4): "a Petri firing sequence
  `M₀ →[t₁] M₁ → …` IS a stigmergic beat chain `b₀ → b₁ → …`". Structurally
  sound at composition altitude — modulo the field-name correction of D2 (the
  math addendum inherits the `signature_beat` shape from the math root, so the
  fix cascades).

- **Petri ↔ Fate** (math addendum §3.2 + spec §4.1): π(t | M) =
  distribution[model_of(t)] / |same-model enabled|. **Well-defined for all
  reachable markings** because:
  1. `T_enabled(M)` is a decidable subset for bounded nets (Karp-Miller).
  2. `model_of: T → Model` is a total function on the Petri transition set
     (spec §2.2 maps five Fate models to five transition classes; every
     transition carries exactly one `model_of` label per math addendum §3.4).
  3. `distribution` sums to 1 (softmax invariant) — verified verbatim in
     `fate/src/lib.rs:180` (softmax5).
  4. Divisor `|{t' : model_of(t') = model_of(t)}|` is ≥1 when t is enabled (t
     itself is in the set).

  Well-defined. SHIP.

- **Fate ↔ Properties** (spec §4.3 + math addendum §4.2-§4.3): verdict at
  fixed-point is content-addressed derivative of (Fate-weights, features,
  trace) — sound via `verdict_is_content_addressed`.

- **Properties ↔ Traces** (spec §5.3 + math root §4.3): witnessed verdicts
  write back via `crystal.derived_predicates`. Sound (modulo the D6 file-path
  fix + Q10 cascade).

**One subtle correctness note (well-formedness bonus):** the math addendum §3.2
firing policy assumes `|{t' : model_of(t') = model_of(t)}| ≥ 1`. If a marking
has NO transitions of a given `Model` enabled, the distribution over that
`Model` becomes 0 (never fires that model at that marking). The math addendum
does not explicitly discuss this degenerate case; may or may not warrant a
footnote in a follow-up. Not blocking.

SHIP.

---

## §8 D8 — Alex Q1-Q10 cascade

**Verdict: SEAM-REJECT.**

The Q5 and Q10 cascades from Alex's 2026-07-18 adjudication cache are
INCOMPLETE in the landed spec.

### §8.1 Q5 — proc-macro test-body layer

Alex ratified YES per Reed memory `feedback_prismqueer_macros_mirror_composes`:

> "lean YES, we already talked about the prismqueer macro layer and mirror
> building on top of it, why would we refuse this?"

The memory clarifies: prismqueer's `declaration!{}` at `prismqueer/src/lib.rs:70`
is proc-macro FLOOR (T23 `@code/rust/macro.shim_type`); test-body macros are
substrate-authored FLOOR, not `.rs` extension. Reed's
`no_rust_extension_shortcut` memory does NOT apply to proc-macro-generated `.rs`.

**Spec §11.5 says:**
> "**Spec's lean:** REFUSE per Taut scout §5.4 + math root §8 naming refusal
> table."

**Math root §8 refused-name table row:**
> `#[proptest]` macro | Test-body sugar hides the type-witness surface | Keep
> `#[test] fn foo() { assert!(matches!(pillar::forall(...), PropertyVerdict::Pass)) }`

**Additional drift:** the spec cites "Taut scout §5.4" — Taut §5.4 is the
Metamorphic testing section; Taut's actual Q5 (§7.5) is about SHRINKER location
(inside Sample vs. forall vs. Arbitrary), not macros. The macro question is
Alex's Q5 from the transcript, not Taut's.

**REED-INLINE cascade:**

Three specific line-changes required:

1. **Math root §8 refused-name table** (`docs/math/2026-07-18-witnessed-property-inference.md:658`): remove the `#[proptest]` row OR reframe it as
   "Direct `#[proptest]` macros bypass prismqueer's `declaration!{}`
   FLOOR; instead land a prismqueer-native test-body proc-macro composed over
   `declaration!{}`." (per Alex's ratification of the prismqueer macro layer).

2. **Spec §11.5** (`docs/specs/witnessed-property-inference-fate-drives-both.md:1029-1034`): rewrite as ratified, not open:
   ```
   Old: "### §11.5 Taut Q5 (still open) — Refuse `#[proptest]` macro layer?
         Spec's lean: REFUSE per Taut scout §5.4 + math root §8 naming refusal
         table. `#[test] fn foo() { assert!(matches!(pillar::forall(...),
         PropertyVerdict::Pass)) }` is the ergonomic bar. The macro layer buys
         3 lines at the cost of IDE/rustc/rust-analyzer surface friction."
   New: "### §11.5 Alex Q5 (RATIFIED 2026-07-18) — proc-macro test-body layer.
         Alex ratified YES per Reed memory `feedback_prismqueer_macros_mirror_
         composes`: prismqueer has `declaration!{}` at `prismqueer/src/lib.rs:70`
         (T23 `@code/rust/macro.shim_type` FLOOR); mirror-side test-body
         macros compose ON TOP of it. Arc 2's `pillar::forall` runner may
         land a proc-macro-generated `#[witnessed_forall]` sugar as
         substrate-authored FLOOR (not `.rs` extension). Taut's §7.5 Q5 was
         about SHRINKER location (Sample vs. forall vs. Arbitrary), not the
         macro layer — separately-addressed here as an open ambiguity for
         Alex adjudication."
   ```

3. **Spec §11.5 hard-cite correction**: replace the "Taut scout §5.4" citation
   with "Taut scout §7.5" (or drop the Taut attribution entirely — Q5 is
   Alex's, not Taut's).

### §8.2 Q10 — verified cache location

Alex ratified `@mirror/store/liquid` as NEW species-decl mint composing
`@mirror/store` + `@liquid` family-root per memory
`project_witnessed_property_inference` Q10.

**Spec §11.10 says:**
> "**Spec's lean:** (a) `@mirror/store.crystal.derived_predicates`."

**Spec §5.1-§5.3 write the cache path to `@mirror/store.crystal.derived_predicates`**

**REED-INLINE cascade:**

Four specific changes:

1. **Spec §11.10** (`docs/specs/witnessed-property-inference-fate-drives-both.md:1094-1110`): re-mark as RATIFIED with (d) — Alex answered "new species-decl mint at `@mirror/store/liquid`".

2. **Spec §5.1** (`docs/specs/witnessed-property-inference-fate-drives-both.md:387-397`): rewrite the cache-location section from `@mirror/store.crystal.derived_predicates` → `@mirror/store/liquid`. The `derived_predicates` field STAYS as the audit trail of "what was verified" at the crystal altitude; the CACHE (memoization-by-construction lookup) lives at the NEW species `@mirror/store/liquid` per Alex's Q10.

3. **Spec §5.2 + §5.3 lookup path**: rewrite `cache_key` lookup path:
   ```
   Old: verdict_of_cache_key(k) := 
       @mirror/store.crystal.derived_predicates[k]
       or fresh(spec, target, inputs) on miss
   New: verdict_of_cache_key(k) := 
       @mirror/store/liquid[k]
       or fresh(spec, target, inputs) on miss
   ```

4. **Spec §12.1 forward promises** (line 1128): add:
   ```
   New bullet: "**`@mirror/store/liquid` species-decl mint** — Alex Q10
   ratified 2026-07-18; new species-decl composing @mirror/store +
   @liquid family-root; PRECEDES Arc 4 cache write-back landing."
   ```

5. **Spec §9.4 Arc 4** (line 897-916): dependency note that shard-decl authorship
   at `shards/mirror/store/liquid.mirror` precedes Arc 4 cache-write-back
   implementation.

### §8.3 Other Q's (Q1-Q4, Q6-Q9) — SHIP

- Q1 (Hypothesis choice-sequence): spec §11.1 correctly leans Hypothesis-shape
  per Taut scout §7.1 + Alex ratification.
- Q2 (extend FEATURE_DIM): spec §11.2 correctly leans extend (16→23).
- Q3 (QuickSpec): spec §11.3 correctly defers to Arc 5+.
- Q4 (enumeration tests): spec §11.4 correctly keeps as boundary-case oracles.
- Q6 (`shards/mirror/petri.mirror`): spec §11.6 correctly notes as pending;
  Alex ratified YES per Reed memory Q6.
- Q7 (tokens = signature_beat composition): spec §11.7 correctly leans (b)
  compose (not SEL extension); Alex ratified compose.
- Q8 (`mirror/rust/src/petri.rs` = proc-macro-generated FLOOR): spec §11.8
  raises the correct question; per Alex Q8 ratification "proc-macro-generated
  substrate FLOOR, not hand-written extension" — Reed-inline cascade could
  update §11.8 to reflect ratification, but this is minor since Arc 3's
  landing tick can absorb it.
- Q9 (Kerr one-line): spec §11.9 correctly leans (a); Alex ratified 1-line
  depth.

Q1-Q4, Q6-Q9: SHIP.

Q5, Q10: SEAM-REJECT — landings above required before Arc 1 lands.

---

## §9 D9 — Landing plan (spec §9 arcs)

**Verdict: SHIP.**

Five arcs reviewed for ordering, Rice-safety, RED-first compatibility:

- **Arc 1** (`pillar::of_health` + Fate bridge) — small (~150 LOC, ~5 tests);
  RED-first-doable; right first tick. Composes over LANDED
  `fate::HolonomyHealth` + `terni::PropertyVerdict`. SHIP.

- **Arc 2** (`Sample` + `Arbitrary` + `forall`) — 400 LOC, 15 tests; depends
  on Arc 1 (or independent — Arc 2 is Sample-carrier only, doesn't need
  `of_health`). Ordering is correct because Arc 2's Sample carrier is what Arcs
  3-5 all consume. SHIP.

- **Arc 3** (Petri-net compilation-loop empirical) — 600 LOC, 10 tests; depends
  on `shards/mirror/petri.mirror` shard-decl (Q6 ratified but not yet authored)
  AND on Arc 2's Sample carrier. Correct sequencing IFF Mara/Taut author the
  petri.mirror shard-decl first (per Q6 ratification).

  **Sub-concern (Q8 raised, ratified):** Arc 3 authors `mirror/rust/src/petri.rs`
  as proc-macro-generated FLOOR (per Q8 ratification); spec §11.8 currently
  raises this as a Q, but Alex ratified. Reed-inline could re-mark as
  RATIFIED. SHIP with the Q6 dependency noted.

- **Arc 4** (`@mirror/store/liquid` cache write-back) — 300 LOC, 8 tests;
  depends on Q10 shard mint (per D8 cascade above). Correct ordering.

- **Arc 5** (Roomba stigmergy composition) — 500 LOC, 10 tests; depends on Fate
  bridge (Arc 1) + Petri (Arc 3) + cache (Arc 4). Correct ordering.

**Total: 5 arcs, ~1950 LOC of Rust + ~48 tests. Reed-owned. RED-first.**

**One landing-plan concern:** spec §9.3 Arc 3 has a Reed-memory guardrail
docblock saying "before authoring `mirror/rust/src/petri.rs`, verify this cannot
be a shard body composing over `@io`". Per Q8 ratification, the answer is
"proc-macro-generated FLOOR" — this is admissible. But the docblock in §9.3
still ends with "See §11 Q3." (which is a Q3 reference — should be Q8 per Alex's
adjudication mapping). Minor cross-ref fix.

**Sub-concern per Q10 cascade (D6/D8):** Arc 4 currently writes to
`@mirror/store.crystal.derived_predicates`; per Q10 must write to
`@mirror/store/liquid`. Reed can't land Arc 4 until (a) Alex or Mara mints the
`@mirror/store/liquid` species-decl, (b) spec §5 is updated. Both are inline
cascade items.

SHIP subject to D8 cascade.

---

## §10 D10 — Recognition candidates

**Verdict: ALEX-Q.**

The three docs surface DISJOINT Recognition candidate sets:

- **Math root §9:** R#1-R#5 (fixed-point, Radon-Nikodym, harness/SUT collapse,
  idempotent composition, shared inference substrate).
- **Math addendum §10:** R1-R5 (four-substrate SQUARE, harness/SUT via shared
  Petri marking, Fate biased firing IS Lawvere fixed point, Petri boundedness
  guarantees PBT coverage, refused-mint metric).
- **Spec §10:** R1-R5 (content-addressed cache invalidation IS idempotent
  closure, three consumer surfaces share one Fate, roomba stigmergy IS third
  consumer surface not shrink, Jason Kerr lineage grounds roomba navigation,
  refused-mint metric).

**These are three distinct candidate sets.** The joint arc's Recognition
candidates are the UNION (~13 distinct candidates after deduplication of R5
across all three).

**Objection:** the math addendum §10 R4
(`#R-petri-boundedness-guarantees-pbt-coverage`) is the STRONGEST claim in the
arc — provable full-coverage via Karp-Miller 1969, orders-of-magnitude sharper
than any Turing-PBT convergence claim. But math root §9 does NOT include a
Karp-Miller / Petri-boundedness recognition candidate.

**ALEX-Q:** should the joint arc ratify a unified Recognition candidate set
(the union of ~13 candidates) or keep them separate per doc? If unified, does
the Karp-Miller upgrade get its own Recognition candidate at the JOINT altitude,
or does the math addendum's R4 suffice for the arc?

Minor Reed-inline could bring the three sets into a unified list either at the
spec's §10 or in a new §14 "Recognition candidate register (joint arc)" — but
Alex direction needed on whether that's warranted.

---

## §11 D11 — Consistency alignment

**Verdict: SHIP-WITH-REED-INLINE.**

The three docs cross-cite correctly at chapter altitude. Vocabulary is aligned
on:

- Four vertices (Traces, Petri, Fate, Properties).
- `<primitive>_of_<input-shape>` naming.
- `PropertyVerdict::Pass | Partial{confidence, diagnostics} | Fail(Diagnostic)`.
- `signature_beat` (though see D2 for the shape mismatch).

**Objections (drifts requiring REED-INLINE):**

1. **`signature_beat` field-name drift.** Math root §2.1 fabricates two fields
   (`witness`, `content_oid`); math addendum §2.4 + §2.1 does not restate the
   struct but references `sc_at_beat` + `previous_beat` correctly. Spec §3.1
   uses `emit_oid` (which is `sel`'s field per subject-family-root spec §5.1,
   NOT `signature_beat`'s field). REED-INLINE per D2 above.

2. **Q5 attribution drift.** Spec §11.5 attributes Q5 to Taut ("Taut Q5") but
   Taut's Q5 (scout §7.5) is a different question (shrinker location). The Q5
   the spec is answering is Alex's (from Reed's memory adjudication cache). See
   D8.

3. **Q5 substantive drift.** Spec + math root both say REFUSE `#[proptest]`;
   Alex ratified YES. See D8.

4. **Cache location drift.** Math root §4.3 + spec §5.1 both cite the cache
   location differently AND both are stale per Q10. See D6 + D8.

5. **`fate/src/lib.rs:426` line citation.** Math root §2.3 point 2 says
   "Fate::tick is deterministic (`fate/src/lib.rs:426`)". Actual line is `:411`
   (where `pub fn tick` is defined). Line 426 is inside the tick body (a call
   to `resolve(features, 5)`). Minor citation-hygiene fix.

6. **Length-of-docs meta:** spec §13 says "~1000 LOC" — verified at 1240 lines.
   Not a substantive issue.

REED-INLINE totaling ~10-15 targeted edits across 3 docs; ~200-400 LOC of
markdown surgery. Ship pending.

---

## §12 D12 — Delight check

**Verdict: SHIP.**

The four-vertex square delights at multiple altitudes:

- **Structural:** the SQUARE closes on ONE marking (`@mirror/petri.tokens`) via
  ONE inference (`Fate::tick`) driving THREE consumer surfaces (compile / test /
  walk). This is the composition Alex named "beautiful" — the substrate had
  every piece; the arc names the composition.

- **Substrate-honest:** ≥21 refused mints. Zero family-roots. Zero species
  (until Q6/Q10 shard-decls land). Every new operational verb is a
  `<primitive>_of_<input-shape>` composition. The refuse count IS the substrate-
  health metric.

- **Rice-safe:** Petri boundedness (Karp-Miller 1969) upgrades PBT coverage
  from probabilistic to provable-in-limit. This is a genuinely novel guarantee
  no prior PBT framework carries.

- **Next-Reed pickup-ability:** the arc plan is five bites, RED-first,
  smallest-first. A future Reed reading `docs/loop/CURRENT.md` + these three
  docs will know what Arc 1 is and how to start. Delightfully-boring.

- **Critic-resistant modulo cascades:** the Q5 + Q10 cascades + `signature_beat`
  field-name drift + wrong-file citations are all mechanical fixes; the
  composition idea itself is substrate-honest and defensible under adversarial
  pressure once the cascades land.

SHIP.

---

## §13 Recommendation for next move

**Recommended: (a) Reed inline cascade first, then (c) Arc 1 land.**

The cascade is bounded and load-bearing:

- 4 REED-INLINE items (see §14 for the residue queue).
- ~200-400 LOC of markdown surgery across three docs.
- Blocks nothing else Reed is doing (Arc 1 depends on the cascade landing).
- Unblocks Arc 1 to land against a substrate that holds.

Then (c) Arc 1 — `pillar::of_health` + Fate bridge — is the right first
empirical landing per Reed's `adjacent_work_may_dissolve_blockers` memory
(small, self-contained, RED-first-doable, unblocks Pillar IV parked since
iter 1).

**Rationale for NOT (b) Mara continuation:** Mara's job on this arc is done at
substrate-honest altitude (math + spec authored); the outstanding work is
Reed-scoped (cascade + Arc 1 empirical). Mara's next scope is `shards/mirror/
petri.mirror` shard-decl authorship (Q6 ratified) + `@mirror/store/liquid`
species-decl authorship (Q10 ratified) — both of which are Mara-work but not
this-tick-Mara-work; they land as separate shard-decl arcs before Reed's Arc 3
and Arc 4.

**Rationale for NOT (d) Stop for Alex:** the substrate cascades are Alex-
ratified already (Q5, Q10 in Reed's memory). Reed can proceed on those without
further Alex direction. The ALEX-Q on D10 (Recognition candidate unification) is
nice-to-have, not blocking.

---

## §14 Forward-promise queue reshape (post-cascade residue)

After the REED-INLINE cascade lands, the residual forward-promise queue for
this arc is:

**Mara-authored (shard-decl work, precedes Reed empirical landings):**

- `shards/mirror/petri.mirror` — the petri-net analyzer shard-decl per Q6
  ratification. Grounds Arc 3.
- `shards/mirror/store/liquid.mirror` — the verified-cache species-decl per Q10
  ratification. Grounds Arc 4.

**Reed-authored (empirical landings):**

- Arc 1: `pillar::of_health` + Fate bridge (small).
- Arc 2: `Sample` + `Arbitrary` + `forall` (Taut §8 Surface A).
- Arc 3: Petri-net compilation-loop empirical (depends on `shards/mirror/petri.
  mirror` + Arc 2 Sample).
- Arc 4: Cache write-back to `@mirror/store/liquid` (depends on Q10 shard mint
  + Arc 2 Sample).
- Arc 5: Roomba stigmergy composition (depends on Arcs 1 + 3 + 4).

**Alex-adjudicable:**

- D10 unified Recognition candidate register (nice-to-have, not blocking).
- Taut's Q5 (SHRINKER location — Sample vs. forall vs. Arbitrary) — this is
  the ACTUAL Taut Q5, separate from the Alex Q5 on macros. Spec §11.5's Q5
  reframe (D8) should surface this as a separately-tracked open ambiguity.
- Kerr citation depth follow-up (Q9 ratified 1-line; deeper follow-up per
  §11.9 (b) or (c) deferred to Mara follow-up).
- SEL text realignment (§12.1: `s/property/petri/g` in SEL §Operationalizability
  + §5.5(b)); forward-promised amendment.

**Second-witness gates (Recognition candidates, none ratified this tick):**

- Math root R#1-R#5, math addendum R1-R5, spec R1-R5 — held for future
  ratification via second-witness events per the Recognition discipline.

**Landing altitude:** all Reed empirical arcs land at terminal FLOOR (`rust/`
+ `prismqueer/`), NOT `bootstrap/` (per Reed memory
`rust_floor_is_rust_not_bootstrap`).

---

## §15 Sign-off

Substrate ground truth verified against:

- `docs/math/2026-07-18-witnessed-property-inference.md` (Mara `cf34549`, 775 lines)
- `docs/math/2026-07-18-witnessed-property-inference-petri-fate.md` (Mara `cf34549`, 990 lines)
- `docs/specs/witnessed-property-inference-fate-drives-both.md` (Mara `cf34549`, 1240 lines)
- `docs/specs/prismqueer-liquid-pillar-composition-surface.md` (Reed `18ef3b8`)
- `docs/specs/subject-family-root-sel-licensable-party.md` (Mara `b3ec316`; §5 verified for `@mirror/petri`)
- `docs/scouts/2026-07-18-taut-property-based-testing-frameworks-fate-inference-driver.md` (Taut `058b892`; §7.1-§7.5 verified)
- `shards/spectral/signature.mirror:75-115` (`type signature_beat` verified)
- `shards/mirror/store.mirror` (`derived_predicates` NOT present)
- `shards/mirror/store/crystal.mirror:343-369` (`type crystal.derived_predicates` verified at :356)
- `shards/epistemologic/property/verdict_is_content_addressed.mirror:1-45`
- `shards/kintsugi/roomba.mirror:60-70` (five actions verified)
- `shards/epistemologic/cybernetic/algedonic.mirror` (`sample_pain`/`sample_pleasure` verified, 3 hits)
- `fate/src/lib.rs:100, 411, 419` (FEATURE_DIM, `pub fn tick`, softmax verified)
- Reed memory files: `project_witnessed_property_inference.md`,
  `feedback_prismqueer_macros_mirror_composes.md`,
  `feedback_composition_primitive_naming_convention.md`,
  `feedback_no_rust_extension_shortcut.md`,
  `feedback_rust_floor_is_rust_not_bootstrap.md`,
  `project_substack_witnessed_property_inference_narrative.md`,
  `reference_jason_kerr_ants_stigmergy.md`.

**Verdict: SHIP-WITH-REED-INLINE.** Four cascade items across three docs, then
Arc 1 lands empirically.

- Author: Seam <seam@systemic.engineer>
- Date: 2026-07-18
- Length: ~800 LOC of markdown
- Marker: `📝 Seam [phase-d-witnessed-property-inference]`
- Bypass: pure-docs 📝 markdown-only bypass

Signed-off-by: Seam <seam@systemic.engineer>
