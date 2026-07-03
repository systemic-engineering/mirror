# Seam adversarial audit — kintsugi-surface-shard (Phase D)

**Audit date:** 2026-07-03
**Auditor:** Seam
**Signed:** `Seam <seam@systemic.engineer>` (SSH)
**Arc:** kintsugi-surface-shard /loop
**Under review:**
- Phase A RED: `10991cb` — `bootstrap/tests/kintsugi_surface_shard.rs` (10 tests)
- Phase B GREEN: `e910dd6` — `shards/kintsugi/surface.mirror` (656 lines)

**Prior audits this arc:**
- `f732d9c` — Seam Phase D — restart-intensity-shard RATIFY (#147 promotion)
- `062fdca` — Seam Phase D — restart-intensity-bilateral RATIFY-WITH-DEFER

**Ancestor spec:**
- `920fe86` — kintsugi canonical formalization (compiler-error-surface.md)
- `9f4211d` — amendment: four corrections per Alex adjudication + Seam audit
- `a3dec7b` — emergent-supervision-from-geometry (H4 empirical DEFER inherited)

---

## Headline verdict

**RATIFY-WITH-CORRECTIONS**

The shard's substrate-decl surface is sound. All 10 RED tests pass under
verified text-check discipline. The fifth #53 bilateral instance is
witnessed cleanly at the routing-composition level. Candidate #141 lands
as its first substrate-decl instance with second-witness gate honestly
forward-promised. Substrate-already-had-the-word twelfth-instance
discipline is honoured — the shard composes against landed
`ashby_variety_match(lock_carrier)` rather than inventing a sibling.

The single correction required is a docblock citation error (Focus 6):
Mara's claim of "verbatim ancestor `shards/spectral/supervisor.mirror:335`"
does not survive grep — the idiom `ashby_variety_match(kintsugi_lock)`
lives in the spec doc (`9f4211d` compiler-error-surface.md line 132), not
in supervisor.mirror. This is a recognition #113 status-drift catch. Fix
the citation to point at the spec doc line; the underlying substrate-fact
is untouched.

Beyond the citation correction, five DEFERs stand per craft-not-deliver.
Composition claims that were made are analytical, not empirical; the
runtime kintsugi tick witness inherits the emergent-supervision §5.5 H4
DEFER honestly.

---

## Per-focus verdicts

### Focus 1 — Fifth #53 bilateral closure

**Verdict: RATIFY**

Grep confirms four prior instances (verified against
`shards/**/*.mirror`):
1. `keyword_matches_path_root` (property) + `kintsugi/fracture/keyword`
   (fracture) — per-predicate — 2026-06-10.
2. `gate_matches_diff_closure` (property) + `kintsugi/fracture/gate`
   (fracture) — per-predicate — 2026-06-16.
3. `operator_matches_composition_primitive` + `symbol_canonical_form` +
   `syntax_substrate_native` (properties) + `angle_to_paren` +
   `symbol_lift` + `operator_match` (fractures) — parametric-over-table
   — 2026-06-19.
4. `restart_intensity_well_formed` (property) + `restart_storm`
   (fracture) — parametric-over-carrier — `e7bd6ec` today.

The fifth instance under review does NOT follow the "new property /
new fracture body" pattern of the prior four. It is a
routing-composition instance: the property side is the *already-landed*
`ashby_variety_match(lock_carrier)` at `coherence-parametric.mirror`;
the fracture side is `surface(t, class, ctx)` + four surface class
carriers at *kintsugi surface altitude*. Same predicate; new
operational routing arm at the surface-species altitude.

The signature match to the #53 pattern is honest: property fires,
fracture-body dispatches on the class carrier. The composition is
signature-clean insofar as the property is altitude-neutral
(parametric over `lock_carrier`) and the fracture body specializes
at surface-species altitude. This is the same discipline as the
parametric-over-table variant (#3 above).

The property-fracture pairing is NOT at "different altitudes"
(compiler-error vs BEAM-supervision) in a way that breaks the
signature — both live at species-boundary altitude with the property
altitude-neutral by construction (that's what "parametric-over-lock"
means at coherence-parametric.mirror). RATIFY the fifth instance.

### Focus 2 — @third conditional-marker precedent (candidate #141)

**Verdict: RATIFY-WITH-CORRECTIONS**

Mara's docblock argument is substrate-pull-honest:

- `@kintsugi` family root (verified `shards/kintsugi.mirror`) imports
  ONLY `in @prism / @glass / @meta` — NOT `@third`.
- This shard (`shards/kintsugi/surface.mirror`) imports `in @third`
  at species altitude.
- The pattern is: marker at species, not at family. Legibility per
  [[feedback-legibility-over-foundation-when-collapsing]] — the
  marker fires where it fires; the family root stays clean.

The generalization test holds. Other candidate species that would
benefit from the same discipline (per docblock forward-promise and
`docs/math/kintsugi/compiler-error-surface.md §10.4`): `@reflection`
at pipeline-error altitude (reflection.mirror already imports `in
@kintsugi/oscillate` but NOT `in @third`; the composition path from
`reflection observes pipeline` → `third_order_observation` per
spec §10.4 is genuinely-plausible).

The candidate #141 promotion gate ("second species with the same
conditional-marker shape") is HALF-MET at this landing per Mara's
own accounting. Promotion criterion: NOT YET met — awaits @reflection
(or an equivalent) landing.

**Correction required:** The docblock states "PRECEDENT-SETTING: this
is the FIRST substrate-decl site where a marker (`@third`) is imported
CONDITIONALLY at the species altitude without the family root importing
it." Grep across `shards/**/*.mirror` for import discipline:
- `@labeled` marker: no species-level conditional imports found.
- `@meta` / `@glass` / `@epistemologic`: pervasive at family altitude,
  not species-conditional.

Precedent-setting claim confirmed by grep. HOWEVER, the docblock's
supporting citation "per Taut's earlier scout: no landed precedent for
conditional-marker inheritance at species altitude" is unverifiable
from my seat — Taut's scout does not carry an OID citation. Not a
correction blocker; flag as a citation weakness.

### Focus 3 — `observation_depth` reuse from `@third`

**Verdict: RATIFY**

Verified:
- `shards/third.mirror` at HEAD (`e43006ab` per Mara reshape) declares
  `type observation_depth = { depth: nat, substrate: ref, witness: ref,
  reflexivity: transparency(ref) }`.
- The shard reuses via `in @third`; does NOT redeclare.
- Semantic match holds: @third's declared semantics ("recursion-depth
  witness over a substrate ref via a witness ref with a reflexivity
  opacity carrier") lifts directly to Mara's usage ("depth-3 witness at
  surface-act boundary"). The `witness_third_order(primary, observer,
  meta)` constructor at `third.mirror` maps 1:1 onto the surface act's
  `(substrate_at_tension_site, kintsugi_loop, surface_act)` triple
  named in Mara's docblock.

No altitude mismatch. The `surface(...) -> observation_depth` signature
is well-formed at kintsugi surface altitude because @third is a marker
crossing family altitudes (per candidate #111 landed + candidate #112).

### Focus 4 — Inline sum-type decision

**Verdict: RATIFY**

Mara's rationale is honest:
- Shared `base: tension` field across all four class carriers verified
  by reading the four type declarations at surface.mirror.
- "Same species altitude" claim holds: all four classes describe
  sub-frames of the same Ω curvature 2-form (per spec §4.1); no class
  carries additional discipline the others lack. The rigor asymmetry
  (contradiction RIGOROUS; other three MOTIVATING) is content-rigor,
  not altitude-rigor. Named honestly per Seam #142 DEFER verdict.
- Precedent restart_kind/child_kind inline sum types verified at
  `supervisor.mirror` lines 320+.

At 656L this is at the upper end of shard file sizes but not
unmaintainable. The `angle_to_paren.mirror` fracture body is 22.8KB
(~660L); `operator_match.mirror` is 26.1KB (~780L); `parent_cycle.mirror`
is 28.3KB. This shard sits comfortably in the family's landed range.

Inline vs separate becomes questionable when class carriers diverge in
discipline (e.g. if one class needs its own precondition or witness
family the others don't share). None of the four here do — they share
the `base: tension` field and the same discharge pathway. RATIFY inline.

### Focus 5 — `resolve_restart_storm` body-altitude composition DEFER

**Verdict: DEFER (honest DEFER, not evasion)**

Mara documents the composition path in the docblock (three-mode-algebra
dispatch: `budget_positive` violation → apply; `ratio_within_supervision_
range` violation → spawn; context-local ratio ceiling → hold). This is
the specification of the interface `restart_storm` would use to call
`surface`.

Could Mara have LANDED the composition this tick? The blocker is
`kintsugi_lock_of(t, ctx)` — the concrete `lock_carrier` instantiation
at kintsugi surface altitude — is itself DEFERRED (Focus 1 in the
docblock DEFER list). Without the concrete lock instantiation, the
`requires ashby_variety_match(kintsugi_lock)` clause cannot discharge at
body altitude. Landing the composition against a deferred lock instantiation
would be false-precision.

The composition is analytically clear; the body-altitude discharge waits
on `kintsugi_lock_of` + an empirical restart-storm witness (per
emergent-supervision §5.5 H4 DEFER inherited). Per craft-not-deliver, the
DEFER is honest. RATIFY the DEFER as-flagged; it composes cleanly in a
follow-up tick.

### Focus 6 — Un-cite-ability compliance

**Verdict: RATIFY-WITH-CORRECTIONS (one citation error)**

Docblock cites verified:
- `9f4211d` — kintsugi amendment: OID exists in git log; docblock text
  faithfully summarizes the amendment's four corrections.
- `062fdca` — Seam's restart_intensity_bilateral audit: OID exists;
  docblock reference to this audit as "the audit that named the fourth
  #53 bilateral this shard follows as the fifth" is accurate.
- `a3dec7b` — emergent-supervision math cluster: OID exists; docblock
  reference to §5.5 H4 empirical DEFER inheritance is faithful.
- `920fe86` — original kintsugi canonical: OID exists; docblock
  reference to spec §10.1 landing-order step 5 closure is accurate.

**Citation error (correction required):**

Docblock section "The twelfth-instance discipline (Alex 2026-07-02)"
claims:

> The verbatim ancestor is `shards/spectral/supervisor.mirror:335`
> (Taut scout on kintsugi arc).

Grep across `shards/spectral/supervisor.mirror` for
`ashby_variety_match|kintsugi_lock` returns **zero matches**. Line 335 of
supervisor.mirror is in the `restart_kind` closed-sum type declaration,
not near any coherence-parametric reference.

The actual verbatim ancestor for `ashby_variety_match(kintsugi_lock)`
lives in the spec doc `docs/math/kintsugi/compiler-error-surface.md` at
line 132 (`requires exists_fracture_body(m, ctx) &&
ashby_variety_match(kintsugi_lock_of(m.tension, ctx))`), landed at
`9f4211d`.

This is a recognition #113 status-drift catch. **Correct the docblock**
to cite `docs/math/kintsugi/compiler-error-surface.md:132` (`9f4211d`)
instead of `shards/spectral/supervisor.mirror:335`. The underlying
substrate-fact (twelfth-instance discipline; compose against landed
predicate) is untouched by the correction.

### Focus 7 — Autopoietic self-witnessing claim

**Verdict: RATIFY (structural, not performative)**

Mara claims "shard's own docblock applies conditional-marker discipline
to its own @third import." Testing this against the @third pattern:

- @third's discipline: recursion-depth-3 witness = observer of the
  observer of the observer.
- The shard's docblock structure: (a) observes the substrate at
  tension-site; (b) observes the substrate observing (kintsugi loop);
  (c) observes the substrate observing the observing (the surface act
  emitting a Tomm question); (d) observes the substrate declaring that
  it observes its own observing observing (this docblock naming the
  conditional-marker discipline it enacts).

Level (d) is genuinely circular-reflexive: the shard names @third as
"marker imported conditionally at species altitude for the first time"
AND the docblock's own act of naming this discipline participates in
the discipline. Per @third's `mechanism_visible(o)` sub-predicate: the
mechanism under observation IS legible under the reflexivity carrier
because the docblock names it explicitly.

This is not performative flourish. The autopoietic self-witnessing
claim holds structurally. Per candidate #141's own promotion criterion,
the shard's compliance with the discipline it names is evidence for
promotion (though the second-witness gate is a separate criterion).

RATIFY. This is stronger than performative — the shard demonstrates
the pattern it declares.

---

## Fifth #53 bilateral closure verdict

**RATIFY**

The routing-composition instance is signature-clean. Property side
lives at landed `coherence-parametric.ashby_variety_match(lock_carrier)`;
fracture side at kintsugi/surface's `surface(t, class, ctx)` + four
class carriers. Same predicate, new operational routing arm at
surface-species altitude. Same discipline as the parametric-over-table
variant (#3 above).

The five prior/current #53 instances span three variant shapes:
per-predicate (×2), parametric-over-table (×1), parametric-over-carrier
(×1), and routing-composition (×1, this landing). Recognition #59
(kintsugi loop altitude-portable) absorbs this diversity per the promoted
#59 signature ("absorbs four variant shapes under one signature").
This landing extends #59's coverage.

---

## Candidate #141 promotion status

**HALF-MET (agrees with Mara's accounting)**

The first substrate-decl instance is landed here. The second-witness
gate awaits either:
- `@reflection` acquiring conditional `@third` at pipeline-error
  altitude per spec §10.4 candidate #143 (genuinely-plausible; the
  composition path from reflection's already-landed `in @kintsugi/
  oscillate` to third-order observation is analytically clean), OR
- Some other species landing the same conditional-marker shape.

Mara names @reflection as the plausible second witness. Verified:
`reflection.mirror` already imports 18 markers/families including
`@kintsugi/oscillate` but NOT `@third`. The composition path exists.
Not hand-waving.

Promotion firing conditions:
1. Second witness lands (as above).
2. Both witnesses maintain the discipline (family root stays clean;
   marker conditional at species altitude).
3. Grep confirms no landed counter-precedent (i.e. no other family
   root that ALSO carries a conditional marker in a way that would
   break the family/species altitude discipline).

---

## Single strongest adversarial finding

**Docblock citation error at "twelfth-instance discipline" section.**

The claim "The verbatim ancestor is
`shards/spectral/supervisor.mirror:335` (Taut scout on kintsugi arc)"
does not survive grep. The `ashby_variety_match(kintsugi_lock)` idiom
lives in the spec doc `9f4211d`, not in the supervisor shard. Line
335 of supervisor.mirror is unrelated.

This is a recognition #113 status-drift instance: a citation was
carried forward from an earlier draft context and drifted from what
grep actually confirms. The underlying substrate-fact is untouched
(the twelfth-instance discipline holds; compose against landed
`ashby_variety_match` is correct). Fix: correct the citation to point
at `docs/math/kintsugi/compiler-error-surface.md:132` (`9f4211d`).

Recommend a follow-up tick: correct the docblock citation and commit
as a docs-only fix (📝 marker; no RED needed since text-checks don't
gate on citation content).

---

## Next /loop prompt recommendation

Given today has closed:
- #147 (restart intensity budget) RATIFIED
- #53 fourth-instance (restart_intensity bilateral) PARTIALLY CLOSED
- #53 fifth-instance (kintsugi/surface) NEW landing RATIFY-WITH-CORRECTIONS
- #141 (conditional-marker) HALF-MET

**Recommended next tick: (a) Second witness for #141 — `@reflection`
at pipeline-error altitude.**

Rationale:
- Closes #141's promotion gate in the shortest path.
- Composes with landed substrate: `reflection.mirror` already imports
  `in @kintsugi/oscillate`; adding conditional `in @third` at
  pipeline-error altitude is the smallest tick that fires the second
  witness.
- Spec §10.4 candidate #143 already forward-promised this composition
  path; the analytical work is done.
- Empirical restart-storm witness (option b) waits on infrastructure
  Reed does not control this tick (spectral serve port); (a) is
  substrate-decl only.
- Option (c) `resolve_restart_storm` composition against surface
  waits on `kintsugi_lock_of` body instantiation (DEFERRED this tick);
  landing (a) first is strictly less-blocked.
- Options (d) @spin marker and (e) publish un-cite-ability are
  legitimate parallel tracks but neither closes the arc opened today.

Alternate strong recommendation: correct the citation error (Focus 6)
in a docs-only fix commit *before* firing (a), so the shard's docblock
is clean going into the second-witness landing.

---

## Discipline notes

- **Grep-first per composition claim**: recognition #113 status-drift
  fired here (Focus 6). Third-order discipline — grep the citation,
  not the memory of the citation.
- **Composition claims need empirical test**: Mara's 5 DEFERs are
  honest per craft-not-deliver. The analytical composition path is
  sound; the empirical witness inherits emergent-supervision §5.5 H4.
- **Legibility-over-foundation on collapse**: conditional-marker
  discipline chosen over baseline family-root inheritance. Legibility
  preserved. Ratified structurally.

---

Signed: `Seam <seam@systemic.engineer>`
