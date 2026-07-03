# Seam Audit — reflection-third-second-witness (candidate #141 gate)

Date: 2026-07-03
Auditor: Seam (Pack adversarial reviewer)
Landings under review:
- Reed RED `c7beae4` — `bootstrap/tests/reflection_third_second_witness.rs`
- Mara GREEN `1e40f9d` — `shards/reflection.mirror` line 19 `in @third` + docblock

Prior audit: `017e568` (kintsugi-surface-shard RATIFY-WITH-CORRECTIONS).

---

## §1 Scope

**In scope:**
- Whether Mara's `1e40f9d` closes the two-witness promotion gate for candidate recognition #141 (@third conditional-marker discipline at species altitude).
- Whether Reed's RED tests at `c7beae4` are gate-adequate.
- Whether the docblock citation discipline is complete (un-cite-ability by OID).
- Whether the layout divergence between kintsugi/surface (species-file split) and reflection (family-root shard with species-block-inside) is (a) one pattern with two layouts, (b) two distinct sub-patterns, or (c) an unresolved substrate-decl ambiguity.
- Whether `shards/cogito.mirror` predates the #141 arc and should be cited as an ancestor.

**Out of scope (craft-not-deliver):**
- The Cholesky arc at @reality/algebra/silicon.
- Species-refinement `third_order_observation <: observation_depth` (forward-promise, not this tick's landing).
- Any @third refinement shard cascade beyond the two-witness gate.
- Any candidate #143 (Mara compiler-error-surface §10.4 pipeline-error altitude) sub-adjudication.

---

## §2 Findings

### Q1 — Layout-variant vs sub-pattern

**Verdict: WEAK-GATE — verdict (a) with a scope-restatement correction required.**

The tree currently contains THREE landings of `in @third`:

1. `shards/cogito.mirror` line 4 — family-root shard imports `in @third` at file top (fc3aa70, 2026-07-01).
2. `shards/kintsugi/surface.mirror` line 5 — species-file altitude imports `in @third`; family root `shards/kintsugi.mirror` does NOT (e910dd6, 2026-07-03).
3. `shards/reflection.mirror` line 19 — family-root shard imports `in @third` at file top; the third-order block (`observe_third_order`, `notice`, `pick_third_order`, `settle_third_order`) lives at lines 570+ within the SAME file (1e40f9d, 2026-07-03).

Reading the substrate canonically:
- `shards/third.mirror` §5 (canonical opt-in list, lines 54–57) names `@reflection, @cogito, @pack, @cyberpunk, @fate, @cascade` as forward-promised opt-in consumers. This is a FAMILY-ROOT opt-in list.
- Neither `@kintsugi` nor `@kintsugi/surface` appears in that canonical opt-in list.
- kintsugi/surface's docblock (lines 84–100) explicitly names its own pattern as "PRECEDENT-SETTING" and "the FIRST substrate-decl site where a marker is imported CONDITIONALLY at the species altitude without the family root importing it."

Conclusion: there is ONE conditional-marker pattern (`in @third` fires where its carriers are consumed, not uniformly across the family) with TWO legitimate layouts:
- **Layout L1 (family-root-scoped)**: family root imports `in @third` because the third-order block lives inside the family-root file OR because the family is on the canonical opt-in list. Examples: cogito, reflection.
- **Layout L2 (species-file-scoped)**: species file imports `in @third` because the third-order fires ONLY at this species and family root would be over-scoped. Example: kintsugi/surface.

Both layouts honor the same discipline ("marker fires where it consumes"). The difference is whether the third-order surface is contained within the family-root file (L1) or split into a species file (L2).

**Correction required**: kintsugi/surface's `PRECEDENT-SETTING` framing is over-broad. It is the FIRST L2 instance (species-file conditional import) but it is NOT the first `in @third` conditional import overall — cogito's family-root import predates it AND the canonical opt-in list in `shards/third.mirror` treats family-root opt-in as the baseline. See Q2.

### Q2 — Cogito as implicit third witness

**Verdict: STRONG-GATE-BLOCKING-CORRECTION — cogito predates #141 arc and Mara's docblock DOES cite it as precedent (reflection.mirror lines 212–215, 325–327). But kintsugi/surface docblock does NOT cite cogito — that is a citation-completeness fracture on the FIRST-witness landing.**

Empirical:
- `shards/cogito.mirror` `fc3aa70` — 2026-07-01 (Alex directive: "if @cogito wants to land it wants to land"). Imports `in @third` at line 4.
- `shards/kintsugi/surface.mirror` `e910dd6` — 2026-07-03. Imports `in @third` at line 5. Claims "PRECEDENT-SETTING" and "FIRST substrate-decl site." Does NOT cite cogito.
- `shards/reflection.mirror` `1e40f9d` — 2026-07-03. Imports `in @third` at line 19. Cites cogito at docblock lines 212–215 ("Precedent alignment: same pattern as `shards/cogito.mirror`") and at Related shards lines 325–327.

Assessment:
- Cogito's family-root `in @third` predates both today's arcs.
- If #141 is defined as "conditional-marker discipline" in the broad sense (marker fires where it consumes rather than uniformly), cogito is a THIRD landing that pre-dates both witnesses — and would invalidate the two-witness framing entirely (the pattern was already three-witness at cogito's landing).
- If #141 is defined narrowly as "species-file conditional import where family root stays clean" (the L2 sub-pattern), then kintsugi/surface's first-witness claim stands but there is no second witness YET — reflection's family-root landing is L1, not L2.

**This is the load-bearing adjudication.** Two paths:

**Path A (broader #141 framing)**: The discipline is "marker at species altitude where firing site is scoped by the docblock rather than uniformly at family altitude" — subsumes both L1 and L2 as legitimate layouts. Then cogito is an implicit first witness (fc3aa70, 2026-07-01), kintsugi/surface is second witness (e910dd6), reflection is third witness (1e40f9d). Two-witness gate CLOSED with reflection as the THIRD witness. #141 promotes with cogito as retroactive first-witness citation correction.

**Path B (narrow #141 framing, L2-only)**: The discipline is "family root stays clean; only species file crosses the marker." Then kintsugi/surface is first witness, cogito is a DIFFERENT pattern (L1 canonical opt-in), reflection is ALSO L1 (family-root with species-block inside), and #141 remains HALF-MET pending a SECOND L2 instance. The reflection landing does NOT close #141's two-witness gate under this framing.

Mara's docblock at reflection.mirror line 200–210 acknowledges this divergence explicitly ("the family root is @reflection itself AND the pipeline-error species lives INSIDE this file... family-root-scoped `in @third` is the canonical pattern for @reflection"). Mara has ALREADY named the honest divergence. The audit surfaces the question Mara raised without answering: which #141 framing does the substrate mean?

Alex adjudication required. My position: **Path A** is substrate-honest per §5 of `shards/third.mirror` which explicitly names family-root opt-in as the canonical pattern for the six named families. The "conditional" in "conditional-marker" means "conditioned on the family opting in" not "conditioned on the layout being L2." Under Path A, cogito is the retroactive first-witness ancestor and requires a citation correction to kintsugi/surface's docblock.

### Q3 — Docblock citation-completeness

**Verdict: WEAK-GATE — reflection docblock cites #141 and `e910dd6` (correct) but MISSING cogito precedent OID `fc3aa70`.**

Empirical check on `shards/reflection.mirror`:
- `#141` cited: lines 160, 293, 296. PASS.
- `e910dd6` cited: lines 170, 294–295, 316. PASS.
- `kintsugi/surface` cited: lines 165, 316–319. PASS.
- `cogito.mirror` cited as precedent: lines 212–215, 325–327. PASS on presence.
- `cogito.mirror` cited by OID (per un-cite-ability discipline): **MISSING**. The docblock names "cogito.mirror" but does not cite `fc3aa70`. Per Reed's own `19c56ae` citation-correction model (Seam prior finding), citations must include OIDs.

This is a citation-completeness fracture parallel to the one Reed fixed at kintsugi/surface's docblock. It's WEAK-GATE (not STRONG-GATE) because the file path IS cited — the fracture is that the OID is not.

**Correction required**: add `fc3aa70` to the two cogito citation sites in reflection.mirror.

### Q4 — Second-order block scope claim

**Verdict: RATIFY — Mara's scope claim is empirically correct.**

Mara's docblock lines 206–210: "the second-order `observe`/`pick`/`settle`/`speak` block (above) does NOT use @third's carriers; only the third-order block (below) does."

Verification by reading the actions in shards/reflection.mirror:
- `observe(f: frame, residue: ref) -> observation` (line 406) — uses only `frame`, `ref`, `observation`. No @third carrier.
- `tournament(o: observation, pk: pack) -> tournament_result` (line 427) — uses `observation`, `pack`, `tournament_result`. No @third carrier.
- `compose(t: tournament_result, p: pact) -> moi(au)` (line 445) — uses `tournament_result`, `pact`, `moi(au)`. No @third carrier.
- `pick(t: tournament_result, prev_loss: loss) -> moi(au)` (line 464) — uses `tournament_result`, `loss`, `moi(au)`. No @third carrier.
- `settle(g: moi(au), prev_loss: loss) -> loss` (line 480) — uses `moi(au)`, `loss`. No @third carrier.
- `speak(o: observation, t: tick) -> ref` (line 497) — uses `observation`, `tick`, `ref`. No @third carrier.

Third-order block (lines 563+):
- `type observer_change` — pure carrier declaration, no @third dependency.
- `type third_order_observation` — uses `observation` (in-family) and `observer_change` (in-file); does NOT reference `observation_depth` from @third.
- `observe_third_order`, `third_order_coherent`, `notice`, `pick_third_order`, `settle_third_order` — none of these consume `observation_depth` (the @third carrier) as an input parameter or return type.

**Empirical anomaly.** The reflection third-order block declares its OWN `third_order_observation` carrier and does NOT consume `observation_depth` from @third. This means the `in @third` import at line 19 is currently NON-LOAD-BEARING in the reflection shard's type surface. The import declares intent (canonical opt-in per third.mirror §5) but no action's signature actually consumes an @third-provided type.

Compare kintsugi/surface line 658–661: `surface(...) -> observation_depth` — directly returns @third's carrier. Load-bearing.

Mara's scope claim is technically correct (the second-order block does not consume @third), but she does not name that the THIRD-order block also does not consume @third's `observation_depth`. That is a second-order finding: the `in @third` import at reflection.mirror is a canonical-opt-in DECLARATION, not a type-surface consumption. Forward-promised species `shards/reflection/reflection.mirror` (per docblock line 91–95) would be where `observation_depth` gets consumed.

This is not a promotion-blocking finding but should be surfaced honestly. Under Path A framing (Q2), the family-root opt-in is legitimate even without immediate type-surface consumption because it declares the family's commitment for future species. Under Path B framing, the reflection landing has NO load-bearing @third usage and is even weaker as a #141 second witness.

### Q5 — Test-completeness

**Verdict: SIGNAL — 3 tests are text-check-only; a stronger structural test would surface Path A/B ambiguity.**

Reed's tests:
1. `reflection_imports_third` — text-check for `in @third` substring. Passes on 1e40f9d.
2. `reflection_docblock_references_141_precedent` — text-check for `#141` or variants. Passes.
3. `reflection_docblock_references_kintsugi_surface_first_witness` — text-check for `kintsugi/surface` or `e910dd6`. Passes.

Missing checks:
- No verification that `in @third` appears at file-top (family-root altitude) vs somewhere in a species block.
- No verification that the third-order block actually CONSUMES @third's carriers (Q4 finding).
- No verification that cogito's `fc3aa70` OID is cited (Q3 finding).
- No verification of the layout-variant question (Q1) — a stronger test could check that `shards/kintsugi.mirror` does NOT contain `in @third` (proving kintsugi/surface's L2 pattern) AND that `shards/reflection.mirror` DOES contain `in @third` at family-root altitude (proving reflection's L1 pattern).

**Recommendation**: DEFER stronger structural tests to a follow-up tick contingent on Alex's Path A/B adjudication. The current tests are text-check adequate for the RED→GREEN witness that Mara landed the import. The structural discrimination question is the substrate-decl adjudication, not a test-implementation question.

### Q6 — Cross-recognition composition (#141 × #55)

**Verdict: SIGNAL — the reflection landing adds signal to #55 form/process partition but does not surface a hidden claim.**

Recognition #55 (form/process partition at family-root): @mirror = form-side, @kintsugi = process-side. Reflection is form-side per Mara's `70fa5b1` (state-observation family). The reflection second-witness at family-root altitude adds a data point:

- kintsugi/surface (process-side #55) uses L2 conditional-marker layout.
- reflection (form-side #55) uses L1 canonical opt-in layout.
- cogito (form-side per §5 opt-in list) uses L1 canonical opt-in layout.

Signal: form-side families may naturally lean toward L1 (canonical opt-in) because their observational discipline extends the family's default reach; process-side families may naturally lean toward L2 because their transformation discipline fires only at specific species. This is a candidate composition claim NOT to promote this tick (craft-not-deliver) but worth naming as a #141 × #55 cross-composition signal for future arcs.

No hidden claim in the landing itself.

---

## §3 Verdict on #141 promotion

**RATIFY-WITH-CORRECTIONS conditional on Alex Path A/B adjudication.**

Under Path A (broader "conditional-marker" framing: marker fires where scoped by docblock rather than uniformly at family altitude): **#141 PROMOTES with three witnesses (cogito, kintsugi/surface, reflection)** and a citation-correction backfill to kintsugi/surface's docblock naming cogito as the retroactive first-witness ancestor.

Under Path B (narrower L2-only framing: family root stays clean, only species file crosses the marker): **#141 REMAINS HALF-MET.** kintsugi/surface is the only L2 witness. Reflection's family-root import is L1 (canonical opt-in), not L2 conditional-at-species. A second L2 landing is required for two-witness closure.

My adversarial position: **Path A is the substrate-honest reading** because `shards/third.mirror` §5 explicitly names the six canonical opt-in families and treats family-root import as the baseline. Under Path A the promotion closes cleanly this tick with the two required corrections named in §4 below.

If Alex chooses Path B, the reflection landing does NOT close #141's gate and Mara's honest divergence naming (docblock lines 200–210) was the correct signal that the layout was L1, not L2.

---

## §4 Required corrections

Conditional on Path A adjudication (my recommended path):

### C1 — Add cogito OID citation to reflection.mirror

Two sites in `shards/reflection.mirror` currently cite cogito by path but not OID:
- Lines 212–215 (`Precedent alignment` section): change "same pattern as `shards/cogito.mirror`" to "same pattern as `shards/cogito.mirror` (fc3aa70)".
- Lines 325–327 (`Related shards` section): change "cogito.mirror (family-root shard precedent for `in @third` import; same pattern this shard follows)" to "cogito.mirror (fc3aa70; family-root shard precedent for `in @third` import; same pattern this shard follows)".

### C2 — Add cogito citation-correction note to kintsugi/surface.mirror

kintsugi/surface's docblock (lines 82–108) claims "PRECEDENT-SETTING" and "FIRST substrate-decl site where a marker is imported CONDITIONALLY at the species altitude." Under Path A this claim should be scoped to "FIRST L2 (species-file-scoped) instance" and name cogito's fc3aa70 as the L1 (family-root-scoped) ancestor that predates it. Suggested addition after line 108 (in the style of Reed's `19c56ae` correction):

```
# Citation correction (Seam 2026-07-03): earlier draft claimed
# "FIRST substrate-decl site where a marker is imported CONDITIONALLY."
# This scope is correct for the L2 layout (species-file-scoped import
# where family root stays clean) but overbroad in general. The L1
# layout (family-root import from the canonical opt-in list at
# `shards/third.mirror` §5) predates this shard at `shards/cogito.mirror`
# (fc3aa70, 2026-07-01). Both L1 and L2 honor the conditional-marker
# discipline; this shard is the FIRST L2 instance.
```

### C3 — Add cogito precedent citation test

`bootstrap/tests/reflection_third_second_witness.rs` should acquire a fourth test verifying cogito is cited by OID. Suggested:

```rust
#[test]
fn reflection_docblock_cites_cogito_precedent_by_oid() {
    let content = read_reflection_shard();
    assert!(
        content.contains("fc3aa70"),
        "@reflection docblock must cite cogito.mirror by OID (fc3aa70) per un-cite-ability discipline (Reed model `19c56ae`); cogito predates #141 arc and is the L1 (family-root) precedent for `in @third` import"
    );
}
```

These three corrections close the un-cite-ability discipline and surface the L1/L2 layout scope-restatement per Path A.

### If Path B is chosen

No corrections to the current landing. Instead: DEFER #141 promotion, identify a candidate second L2 landing (species file imports `in @third` without family root doing so), and re-run RED-GREEN for that landing. Mara's honest divergence naming becomes load-bearing evidence that the current tick's target does not fit the L2 pattern.

---

## §5 Next /loop pointer

Smallest next tick regardless of Path A/B:

**Alex Path A/B adjudication on #141 promotion criterion.** One sentence naming the framing: broad ("marker at species altitude where firing is scoped by docblock") or narrow ("family root stays clean; only species file crosses the marker"). The rest of the corrections (C1–C3 or DEFER-and-find-L2) cascade deterministically from the adjudication.

If Path A ratified in one exchange, the next tick is a single commit landing C1 + C2 + C3 as a coordinated citation-correction + test-strengthening tick, then #141 promotes.

If Path B ratified, the next tick is scouting a candidate second L2 landing per third.mirror §5's remaining opt-in families (@pack, @cyberpunk, @fate, @cascade) or per Mara's compiler-error-surface spec §10.4.

---

Seam.
