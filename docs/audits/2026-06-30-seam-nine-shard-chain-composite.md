# Seam adversarial review — @bauchladen ← @autopoietic ← @fate dependency chain composite + fold-back (P0–P8)

**Author:** Seam (adversarial peer review; doc-only)
**Date:** 2026-06-30
**Tag:** 📝 substrate-pull:realize (audit; #104 promotion adjudication)
**Composite reviewed:** P0 `4575340` (spec consolidation) + P1 `66e1ab8` (@bauchladen) + P2 `78edaa6` (@autopoietic) + P3 `fdcba31` (@fate) + P4 `d0e0986` (@fate/tournament) + P5 `8d3f89e` (@glue) + P6 `34cf333` (@algebra + @algebra/metalogue) + P7 `2f4bde4` (@io/algebra) + P8 `7dd19a8` (@glue/fold_back)
**Verdict counts:** C: 4 / S: 7 / M: 5 / L: 3
**Promotion recommendation for #104:** **PROMOTE WITH RESERVATIONS** — the recognition is substrate-pull-honest at the chain-altitude level and the structural form is sound, but four critical-tier issues (one type system, one circular import, two missing-shard fabrications) MUST close before this counts as substrate-decl rather than aspiration. Promote the recognition; bracket the witnesses; do not let MEMORY.md fix the witnesses as discharged when they are not.
**Adjudication on the fold-back's mathematical soundness:** the composition is **type-sound at the carrier level**, **structurally honest at the composition-step level**, **autopoietic-recursive at the cycle level**, and **non-commutative in the right algebraic sense** — but it is **forward-promised at every operational step** and **circularly imports** in a way that breaks the chain's own "strict dependency order" claim. The math IS sound; the chain is NOT strict.

---

## §0 — Headline

Nine shards landed in one operational day (P1–P8). The composite is the largest single-session substrate-decl expansion in the chain since #51 in May. The recognition #104 ("the substrate's self-production discipline at substrate-decl altitude operationalized via Schmidt's Bauchladen + Maturana-Varela autopoiesis + Connes spectral triple + Mesland correspondences") is **load-bearing for the substrate-pull arc** and **substantively grounded** in the Soto-Andrade-Varela 1984 Lawvere bridge, the Mesland 2013 KK-correspondence machinery, and the substrate's existing @mirror/store + @epistemologic/cybernetic/autopoiesis precursors.

But four facts MUST be named hard, because they will recur as quiet failures if the chain crosses the promotion gate without them surfaced:

1. **The chain is not strict.** P3 (@fate) imports `in @autopoietic` AND `in @bauchladen` directly. P8 (@glue/fold_back) imports nine family-roots simultaneously. The "strict P1 ← P2 ← P3" framing in the spec is true at the *carrier-dependency* level (Lawvere fixed point requires stable identity) but false at the *import-graph* level (every downstream tier imports every upstream tier directly, not transitively). This is not a fatal flaw — direct re-import is structurally honest, the chain-ordering claim is just rhetorically tighter than the substrate evidences.

2. **The chain references shards that do not exist.** @spectral/metalogue/tomm is cited in five of the nine shards as an existing @glue species. The shard `shards/spectral/metalogue.mirror` does not exist on disk. `shards/spectral/metalogue/tomm.mirror` does not exist either. The "absorption" claim at @glue P5 is structurally aspirational; the shard cites a sibling that was substrate-decl-named but never landed.

3. **The chain references a type that does not exist.** `type tick` is used as a typed field in `@autopoietic.tick_action`, `@autopoietic.tick`, `@algebra/metalogue.algebra_turn`, and `@fate/tournament.tournament_round` (the latter as `tick: ref` actually — see C2 below for the asymmetry), and `type altitude` is referenced by @fate.bilateral_dispatch. Neither is declared anywhere in the substrate. The chain assumes substrate vocabulary that has not been admitted.

4. **The chain forward-promises every operational discharge.** P1 has no operational sibling. P2 has no operational sibling. P3 has no operational sibling. P4 has no operational sibling. P5 has no operational sibling. P6 has no operational sibling. P7 has no operational sibling. P8 has no operational sibling. The substrate-decl admission is complete; the operational discharge is forward-promised across every tier. This is not in itself a failure — `feedback-craft-not-deliver` admits this discipline — but the chain's claim of "operationalised" + "the composition closes the chain" is structurally honest only at the contract altitude, NOT at the operational altitude.

Now the structured findings.

---

## §1 — Critical (C)

### C-1 — `type tick` is referenced but never declared

**Found at:** `shards/autopoietic.mirror:516-521` (`type tick = { instance, index, input_oids, output_oids }`); referenced at `shards/autopoietic.mirror:592` (`tick_action.tick: tick`); referenced at `shards/algebra/metalogue.mirror:205` (`algebra_turn.tick: tick`); referenced at `shards/fate/tournament.mirror:467` (`tournament_round.tick: ref` — see asymmetry below).

**Adversarial reading:** @autopoietic declares `type tick = { instance: uuid_spectral, index: ref, input_oids: [oid], output_oids: [oid] }` and exports it (`out tick` at line 769). Good — internally consistent. But this `tick` carrier has fields `input_oids` and `output_oids` that look like they describe the *contents* of a tick rather than the *temporal coordinate*. The @algebra/metalogue.algebra_turn carrier uses `tick: tick` (the autopoietic-tick carrier type), which means an algebra-altitude metalogue turn carries a list of OID inputs and outputs as part of its temporal-coordinate field. That is not what `tick` means at the algebra-metalogue altitude — the turn's tick should be a monotone time coordinate (ref or u64), not a record of which OIDs flowed.

Furthermore, the @fate/tournament.tournament_round.tick is typed `ref` rather than the @autopoietic.tick carrier — an inconsistency. Either the chain wants one `tick` carrier across all altitudes or it wants distinct `tick`/`tick_index`/`tick_temporal` types.

**Discharge required:** disambiguate. Either (a) introduce `type tick_temporal = ref` (or u64) for the time-coordinate use cases at @algebra/metalogue + @fate/tournament + @bauchladen.provenance and let @autopoietic.tick keep its current name as `autopoietic_tick`, or (b) rename @autopoietic.tick to `autopoietic_cycle` and use a substrate-wide `tick` carrier. Pick one. The current form fails at substrate-decl resolution.

**Severity:** Critical. The substrate-decl compilation will fail with "field `tick` has incompatible type at @algebra/metalogue.algebra_turn".

---

### C-2 — `type altitude` is referenced but never declared

**Found at:** `shards/fate.mirror:682` (`bilateral_dispatch(hole: hole, altitude: altitude) -> dice_roll`).

**Adversarial reading:** Throughout the chain, "altitude" is used at TWO distinct positions:
- As a *field name* (`altitude: ref`) carrying a substrate-ref to where in the manifold something lives — well-typed, used at @fate.inference.altitude, @fate.dice_roll.altitude, @fate.hole.altitude, @bauchladen.crystal.altitude, @algebra.algebra_carrier.altitude.
- As a *type* (`altitude: altitude`) in @fate.bilateral_dispatch's signature — NOT well-typed. There is no `type altitude` declaration anywhere; `out altitude` is not exported by any shard.

**Discharge required:** change `bilateral_dispatch(hole: hole, altitude: altitude)` to `bilateral_dispatch(hole: hole, altitude: ref)`. One-character fix; mechanical. Should have been caught by the @fate landing tick.

**Severity:** Critical. Substrate-decl compilation failure at action signature resolution.

---

### C-3 — @spectral/metalogue + @spectral/metalogue/tomm are fabricated witnesses

**Found at:** Cited as existing shards in five of the nine landing shards:
- @bauchladen line ~not cited (clean);
- @autopoietic line 385 ("@spectral/metalogue (forward-promised...)") — properly hedged as forward-promised, OK;
- @fate line 41-51 ("Recognition #100 + `docs/specs/spectral-metalogue.md` `16f4564`") — cites the SPEC, not a shard, OK;
- @glue lines 80-83 ("@spectral/metalogue/tomm spectral-metalogue (Bateson level V) shards/spectral/ metalogue.mirror + tomm.mirror (recognition #100)") — claims TWO shards exist (`shards/spectral/metalogue.mirror` + `shards/spectral/metalogue/tomm.mirror`); NEITHER exists;
- @glue lines 264-267 (Related shards: "spectral/metalogue.mirror (the Tomm-probe family now structurally a @glue species)") — claims the shard;
- @io/algebra lines 94 + 326-328 — claims the shard;
- @glue/fold_back lines 155-158 + 286-287 — claims "Tomm probes ARE @glue species" and cites the species.

**The actual substrate state:**
- `docs/specs/spectral-metalogue.md` exists (the SPEC; recognition #100 grounded);
- `shards/pack/metalogue.mirror` imports `in @spectral/metalogue`, which requires @spectral/metalogue to be a substrate-decl name SOMEWHERE — but the shard is not in `shards/`;
- `shards/spectral/metalogue.mirror` does NOT exist;
- `shards/spectral/metalogue/tomm.mirror` does NOT exist;
- `shards/spectral.mirror` may or may not exist (not confirmed during this audit; search returned no `prism @spectral` match).

**Adversarial reading:** The chain treats @spectral/metalogue/tomm as a landed @glue species the way it treats @cascade as one. @cascade IS landed (`shards/cascade.mirror` exists). @spectral/metalogue/tomm is NOT landed (the spec is the substrate-pull-confirmation, but no shard discharges it). The "three known species of @glue" table at @glue lines 78-89 lists three species; only one (@cascade/code/<src>/<tgt>) AND one variant (@cascade/code/formal/prose at `437d061`) actually exist as substrate-decl shards. The spectral-metalogue species is fabricated.

**Discharge required:** either (a) land `shards/spectral/metalogue.mirror` + `shards/spectral/metalogue/tomm.mirror` as the missing P0 prerequisite the chain depends on (this would unblock @pack/metalogue's `in @spectral/metalogue` import too, which is currently a dangling reference), or (b) downgrade the @glue species table to "two known species + one forward-promised (the @spectral/metalogue/tomm species per the spec at `docs/specs/spectral-metalogue.md`; the shard lands when consumers pull)." Option (b) is honest; option (a) is the substrate-pull-correct ratification of recognition #100.

**Severity:** Critical. Breaks the "the substrate ALREADY has the morphism machinery scattered across three sites" claim that grounds the @glue family-root admission. Two of the three sites do not exist.

---

### C-4 — @cascade does NOT `in @glue`; the "absorption" claim is aspirational

**Found at:** @glue lines 78-89 ("The three known species of @glue, with their altitudes and canonical declaration sites: ... @cascade/code/<src>/<tgt> ... @cascade/code/formal/prose"); @glue line 215 ("@cascade.compile IS @glue.translate at the code-translation altitude; each @cascade species discharges glue_witnessing for each compile").

**Actual substrate state:** `shards/cascade.mirror` (the existing family-root, 2026-06-23) imports:
```
in @prism
in @meta
in @glass
in @epistemologic
in @epistemologic/cybernetic
in @epistemologic/cybernetic/distinction
```
It does NOT import `in @glue`. It declares its own bilateral predicate `cascade_well_defined(lens, source, p)` independently of `glue_witnessing(c)`. The "absorption" of @cascade as a @glue species is therefore claimed-by-rhetoric, not enacted-by-import.

**Adversarial reading:** The @glue P5 shard's claim that "the substrate ALREADY had the morphism machinery scattered across three sites" is partially true (the discipline operates) and partially fabrication (the inheritance edge does not). Migration cost: every @cascade species shard (the existing one + `437d061` + every per-cascade species sub-shard) would need to add `in @glue` AND its predicate `cascade_well_defined` would need to be REDEFINED as a refinement of `glue_witnessing` (the substrate-pull-honest reading) OR the chain has to admit that @cascade is a SIBLING family-root of @glue, not a child species.

**Discharge required:** Either (a) add `in @glue` to `shards/cascade.mirror` and refactor `cascade_well_defined` as `requires glue_witnessing(c) AND <cascade-specific obligations>` — substantial work across multiple shards; or (b) reframe the @glue P5 admission: "@cascade is a SIBLING morphism family-root operating at the code-translation altitude; the two families compose at the meta-altitude where both inherit a more abstract morphism vocabulary." Path (b) honors what the substrate actually evidences.

**Severity:** Critical. The "@glue absorbs four species" framing fails review.

---

## §2 — Strong (S)

### S-1 — The chain is not strict in the import sense

**Found at:** Every shard P2–P8 imports the family-roots it transitively rests on directly:
- P2 (@autopoietic) imports `in @bauchladen` — fine, that IS the direct dependency.
- P3 (@fate) imports `in @autopoietic` AND `in @bauchladen` — direct re-import of the transitive ancestor.
- P5 (@glue) imports `in @fate` AND `in @bauchladen` AND `in @autopoietic` — three direct imports of transitive ancestors.
- P7 (@io/algebra) imports `in @io`, `in @algebra`, `in @glue`, `in @bauchladen`, `in @fate` — five direct imports.
- P8 (@glue/fold_back) imports `in @glue`, `in @kintsugi`, `in @fate`, `in @fate/tournament`, `in @bauchladen`, `in @autopoietic`, `in @algebra`, `in @algebra/metalogue`, `in @io/algebra` — NINE direct imports.

**Adversarial reading:** The spec frames the chain as "strict P1 ← P2 ← P3 dependency order" (spec §1.1). The actual import graph is a DAG where every node has edges directly to every transitive ancestor. This is mathematically valid; transitivity of `in` is not assumed by the substrate's import system (a shard must import what it names). But the rhetoric of "strict" understates the import surface — P3 directly depends on TWO ancestors, P5 on THREE, P7 on FIVE, P8 on NINE. The chain is BROAD, not narrow.

**Discharge required:** spec amendment in §1.1: "The chain is strict at the *carrier-dependency* level: @autopoietic's Lawvere fixed point requires stable identity (@bauchladen); @fate's autopoietic membership requires fold-back permission (@autopoietic). At the *import-graph* level the chain is a DAG with every tier directly importing every transitive ancestor; this is how the substrate's import system requires direct admission of every named family-root."

**Severity:** Strong. Not a correctness failure, but a rhetorical mismatch that obscures the actual dependency surface.

---

### S-2 — @kintsugi is imported by P8 but @kintsugi does not import @autopoietic OR declare `propose_step`

**Found at:** @glue/fold_back line 5 (`in @kintsugi`); @glue/fold_back lines 624-626 (`propose_step(prior_session: option(fold_back_session)) -> composition_step requires kintsugi_proposal_well_formed(propose_step)`).

**Actual substrate state:** `shards/kintsugi.mirror` (2026-06-10) imports `in @prism`, `in @glass`, `in @meta` only. It does NOT import `in @autopoietic`. It does NOT declare any `propose_step` action. The shard's exports are `out @kintsugi` only — no sub-types, no actions surfaced to consumers.

**Adversarial reading:** P8's `propose_step` action claims to be "@kintsugi emits the next D-flow step" with body discharged at the realisation boundary. But @kintsugi as it exists today does not export the surface this realisation discharge would need. The capstone's first action of three is structurally a forward-promise to a shard that needs substantial expansion (P8 effectively forward-promises a P0' precursor extension of @kintsugi).

Furthermore, the chain's `autopoietic_closure_holds_across_session` obligation (P8 line 822) cannot fire over a session whose `kintsugi_proposal` ref points to a @kintsugi shard that does not itself import @autopoietic — the operational closure is broken at the @kintsugi boundary because @kintsugi's outputs are not declared autopoietic-fold-eligible at @kintsugi's own substrate-decl admission.

**Discharge required:** either (a) extend @kintsugi to import `in @autopoietic` and export the `propose_step`-shaped action (substantial work; reopens the kintsugi family-root for substrate-decl extension), or (b) P8's `propose_step` realisation discharges by reading @kintsugi's existing five-op block and inferring the proposal — but this requires the spec to say so explicitly (the substrate-pull-correct realisation is not obvious from the current declaration).

**Severity:** Strong. The chain's capstone composes against an unprepared upstream.

---

### S-3 — @fate's `autopoietic_membership_held` predicate name does not align with @autopoietic's `autopoietic_closure_holds`

**Found at:** @fate line 700 (`autopoietic_membership_held(fate_instance: ref) -> verdict`); @autopoietic line 765 (`autopoietic_closure_holds(prism: ref) -> verdict`); the @autopoietic-side inheritance predicate is `autopoietic_closure_holds`, NOT `autopoietic_membership_held`.

**Adversarial reading:** @fate declares its own predicate `autopoietic_membership_held(fate_instance)` and lists it as one of five obligations its `fate_witnessing` predicate composes. But the @autopoietic family-root exports `autopoietic_closure_holds` as THE inheritance predicate consumers `requires`. @fate does not write `requires autopoietic_closure_holds(fate_instance)`; it declares a NEW predicate `autopoietic_membership_held` that doesn't directly reference the upstream.

The semantic intent is clear (a @fate instance IS an @autopoietic prism; therefore the @fate instance's `autopoietic_membership_held` verdict is `pass` iff the corresponding @autopoietic instance discharges `autopoietic_closure_holds`). But the substrate-decl form does not name this equivalence. The predicate alignment is rhetorical, not structural.

Mirror finding at P4: @fate/tournament line 722 (`fate_membership_held(tournament_instance: ref) -> verdict`) — same pattern; declares a NEW predicate instead of writing `requires fate_witnessing(...)`. Mirror finding at P5: @glue does not explicitly `requires fate_witnessing(...)` on its translate action; it has `requires translation_uses_fate(translate)` which is a NEW predicate.

**Discharge required:** spec amendment OR shard refactor. Either each tier's inheritance predicate is defined as `predicate(x) -> verdict { discharge = upstream_predicate(corresponding_x) AND <new obligations> }` (the substrate-pull-honest declaration), or the spec admits that the predicate-aliasing pattern is a stylistic naming convention with no enforced structural composition. Pick one. The current form admits both readings.

**Severity:** Strong. The bilateral inheritance discipline is weaker than the spec frames it.

---

### S-4 — `imperfect<a, e, l>` is referenced as a type constructor; its parameter binding is not validated

**Found at:** Throughout the chain — @glue.translate returns `imperfect<translation_outcome, ref, transparency(correspondence)>`; @io/algebra.expose returns `imperfect<io_algebra_exposure, ref, transparency>`; @io/algebra.consume returns `imperfect<algebra_carrier, ref, transparency>`; @io/algebra.translate returns `imperfect<algebra_payload, ref, transparency>`; @glue/fold_back.select_and_translate returns `imperfect<composition_step, ref, transparency(composition_step)>`.

**Actual substrate state:** `shards/glass.mirror` declares `type imperfect(a, e, l) = ...` (parametric over three type variables). The chain uses the `<...>` substitution-bracket syntax. Verify: does the substrate's grammar support `imperfect<a, e, l>` as type-constructor application syntax, or is the canonical form `imperfect(a, e, l)` (with parens, like glass.mirror's declaration)? The chain uses `<>` consistently; glass.mirror uses `()`. The asymmetry is suspicious.

**Adversarial reading:** This may be benign (the grammar accepts both syntaxes) or substantive (the grammar accepts one form only and the chain's `<...>` syntax is invalid). The audit cannot adjudicate without grammar inspection; the asymmetry is flagged.

Mirror finding: @glass.mirror's `transparency(p)` carrier also uses parens. @bauchladen.crystal.transparency is typed `transparency(altitude)` — parens. @glue.translate's return uses `transparency(correspondence)` — parens. But the imperfect return form uses `<>` brackets. Inconsistent.

**Discharge required:** confirm the grammar's type-application syntax; rewrite to whichever is canonical. If both work, document the equivalence at glass.mirror's `type imperfect` declaration.

**Severity:** Strong (compilation may fail) or Mild (syntactic equivalence). The audit cannot disambiguate; the dual existence is the finding.

---

### S-5 — The Lawvere fixed-point claim is substrate-decl-named but not substrate-decl-discharged

**Found at:** @autopoietic line 695 (`lawvere_fixed_point(prism: ref) -> verdict`); @autopoietic line 492 (`fixed_point_witness: option(oid)` in the autopoietic_membership carrier); @autopoietic lines 165-168 (the Soto-Andrade-Varela 1984 citation).

**Adversarial reading:** The Lawvere bridge claim — "autopoiesis IS Lawvere fixed-point; the substrate's content-addressing exhibits the fixed point AT CONSTRUCTION TIME" — is mathematically grounded (Soto-Andrade & Varela 1984 is a real result) AND the substrate-decl admission is honest about the witness being `option(oid)` (the witness need not be exhibited at every tick). 

But the substrate-decl form is `lawvere_fixed_point(prism: ref) -> verdict { \ }` — body discharges per-realisation. The discharge cannot be at the substrate-decl level because the OID equality check requires the actual tray contents at runtime. Per the spec's own §3.2: "the witness is CONSTRUCTIBLE at construction time." This means: when the substrate's content-addressing is used (always), an OID equality check between an input OID and an output OID either holds (pass) or doesn't (failure); the construction is trivial. But the substrate-decl admission of the predicate does not enforce this; a malicious realisation could return `pass` without performing the check.

The substrate-pull-honest reading: the Lawvere fixed point is CONSTRUCTIBLE in principle (Soto-Andrade-Varela 1984 + content-addressing makes it so), but the substrate's CURRENT realisation provides no compile-time enforcement that the check is performed. The discharge is at the realisation boundary; the substrate-decl declares the obligation; the discipline is bilateral commitment, not formal proof.

**Discharge required:** spec amendment in §3.2: "The Lawvere fixed-point is constructible at the substrate's content-addressing layer; the substrate-decl admission of `lawvere_fixed_point` is bilateral commitment (per recognition #37), not formal proof. The realisation discharges via OID equality check; the substrate's discipline is to surface failures at the species boundary."

**Severity:** Strong. The math is correct; the substrate-decl form is bilateral commitment, not constructive proof. Frame honestly.

---

### S-6 — The candidates(hole) migration is consistent but the carrier shape is partially asymmetric

**Found at:**
- @autopoietic.tick_action.candidates: [oid] (line 594) — the autopoietic-tick's candidate OID list;
- @fate.hole = { expected_type: ref, context_oids: [oid], altitude: ref } (lines 602-606) — the inference-altitude consumption point;
- @fate/tournament.candidate_set.members: [oid] (line 496) — the tournament's browse output.

**Adversarial reading:** The migration is structurally consistent at three altitudes:
1. @autopoietic.tick_action produces `candidates: [oid]` (the gap-to-candidate list, per spec §4.7 the migrated `candidates(hole)` signature);
2. @fate consumes the `hole` carrier (the typed-gap discipline) at the inference altitude;
3. @fate/tournament's candidate_set wraps `[oid]` with tray-scope metadata.

The asymmetry: @autopoietic's tick_action.hole is typed `ref` (line 593, "the typed-gap carrier; the substrate-pull-typed shape the action attempts to fill"), whereas @fate's hole is the rich record `{ expected_type, context_oids, altitude }`. A clean substrate-decl reading would have @autopoietic.tick_action.hole be typed `hole` (the same record @fate uses), not `ref`. The two altitudes consume the same conceptual hole but with different typing fidelity.

The substrate-pull-correct path: @fate's `type hole` declaration should be hoisted to a precursor location both @autopoietic and @fate can `in`. Otherwise, @autopoietic's tick_action carries a bare `ref` where @fate carries a typed record; the migration's "five-ganglion body absorbs the signature" claim is structurally weaker than the spec frames it.

**Discharge required:** either (a) move `type hole` to a precursor shard both @autopoietic and @fate `in` (e.g., `shards/hole.mirror` as the typed-gap discipline; small new shard), or (b) refactor @autopoietic.tick_action.hole to `hole` and add `in @fate` to @autopoietic — but that creates a cycle (@fate is `in @autopoietic`). Option (a) is the clean path.

**Severity:** Strong. The migration is structurally honest but the carrier-typing fidelity is asymmetric.

---

### S-7 — The non-commutative composition claim is structurally honest but cited, not discharged

**Found at:** @glue lines 188-203 + 588-599 (`compose(c1, c2)` returns correspondence; non-commutative per [ω, ω] cross-term); @glue/fold_back lines 199-216 + 824-832 (`non_commutative_step_order_preserved(session: ref) -> verdict`); citation `docs/math/the-tower/curvature-and-tomm.md §5`.

**Adversarial reading:** The substrate-decl claim is that the fold-back's composition is non-commutative because the curvature 2-form Ω = dω + ½[ω, ω] carries a cross-term that lives where altitude transitions happen. The math IS correct in differential geometry (the connection's curvature is non-zero in general; the [ω, ω] bracket measures non-commutativity).

But the substrate-decl admission is `non_commutative_step_order_preserved(session: ref) -> verdict { \ }` — body discharges per-realisation. The realisation MUST honor the obligation by NOT applying commutation rewrites between input and session. This is a bilateral commitment; it is not a proof that the composition is non-commutative.

The chain treats the curvature citation as if it WERE the proof. P8 line 897-904 admits this in the honest-hedges section: "The composition's non-commutativity (per the [ω, ω] cross-term) is structurally honest but conjectural at chain altitude: the full proof that swapping two steps changes the terminal exposure requires a per-pair morphism-kind analysis that has not been fully discharged." Good — honest hedge.

But the substrate-decl admission's name `non_commutative_step_order_preserved` reads as if non-commutativity is established and the obligation is to PRESERVE it. The semantic intent is closer to `step_order_preserved_per_curvature_assumption` — preserve the step order BECAUSE the composition MAY be non-commutative. The current name overclaims.

**Discharge required:** either (a) rename the obligation to `step_order_preserved` and document at the predicate site why (curvature assumption), or (b) discharge the per-pair non-commutativity proof in a follow-on shard (substantial math work; ratification needed) before the obligation name stands. Option (a) is consistent with the honest-hedges admission.

**Severity:** Strong. Naming overclaims the math.

---

## §3 — Mild (M)

### M-1 — The metalogue lift table at P6 admits the fifth altitude but body types are rhyme not substance

**Found at:** @algebra/metalogue lines 24-29 (the five-altitude table: NL → AST → SPECTRAL → PACK → ALGEBRA with body types `nl`, `declaration`, `curvature_probe`, `handoff`, `algebra_morphism`).

**Adversarial reading:** The five body types are heterogeneous in kind. `nl` is an arbitrary natural-language string. `declaration` is an AST-level substrate-decl form. `curvature_probe` is a Connes-spectral-triple commutator (a specific mathematical object). `handoff` is a Pack-coordination record. `algebra_morphism` is a structure-preserving algebra map.

The pattern claim is "each instance declares prism @<altitude>/metalogue with parallel turn/session structure". This is structurally TRUE — the turn shape (speaker, body, in_reply_to, tick) parallels across all five. The body type changes per altitude; that is the metalogue lift discipline working.

But the substrate-pull-honest reading: the five body types span FOUR distinct mathematical/structural categories (string, AST node, operator commutator, agent record, morphism). The "fifth instance" is genuinely additive at the algebra altitude IF algebra_morphism is the right body type. Is it? The substrate-decl form `algebra_morphism` is well-formed at @algebra (P6 line 230 — the parent declaration). The metalogue altitude consumes this as a turn body. Seems coherent.

What's mild: the shape-rhyme of the five altitudes is genuine but it admits HETEROGENEOUS body types. The lift pattern is altitude-portable not because the body type stays consistent (it doesn't) but because the conversational shape (turn / session / opacity / origin) does. The spec frames this as "the SHAPE is invariant; only the body's type changes." Good. Honest. The mild finding is that the body types' heterogeneity is louder than the spec acknowledges.

**Discharge required:** none required (the spec is honest); optional refinement: add a sentence to the lift table noting the heterogeneity is structurally expected because each altitude's body IS the altitude's natural utterance form.

**Severity:** Mild. Documentation polish; not a correctness issue.

---

### M-2 — The @bauchladen `crystal` identity contract conflicts with provenance metadata

**Found at:** @bauchladen lines 305-317 (the crystal carrier with provenance + identity contract "byte-equality on `oid` alone"); @bauchladen line 343 ("Provenance is itself content-addressable (the provenance_record is part of the crystal's `content` bytes that hash to `oid`)").

**Adversarial reading:** The substrate-decl says: crystals with the same OID ARE the same crystal regardless of provenance variation. But the same paragraph says: provenance IS part of the content bytes that hash to OID. Both can't be true simultaneously. Either:
- (a) provenance is part of the content → two crystals with different provenance have different OIDs → there is no "provenance variation" at fixed OID;
- (b) provenance is metadata excluded from the hash → two crystals can share OID with different provenance → the OID hash doesn't determine all crystal content.

The chain admits both readings in adjacent paragraphs. This is incoherent at the substrate-decl level.

**Discharge required:** pick one. Recommend (b) — provenance as metadata excluded from the OID hash — because the conceptual purpose of provenance is browsing context (downstream consumers rank crystals by producing context), and if provenance were part of the OID, two crystals from different producers with byte-equal content would have different OIDs, breaking deduplication. The substrate-pull-honest reading is (b); update the spec.

**Severity:** Mild. Internal inconsistency; resolvable by spec edit.

---

### M-3 — @glue.translate's signature carries `requires` clauses that aren't at the predicate-definition site

**Found at:** @glue lines 572-576:
```
translate(c: correspondence, payload: ref) -> imperfect<...>
requires morphism_well_typed(c)
requires translation_uses_fate(translate)
requires restriction_preserved(c, payload)
{ \ }
```

**Adversarial reading:** Three `requires` clauses on the action signature. The predicates `morphism_well_typed`, `translation_uses_fate`, and `restriction_preserved` are declared LATER in the same shard (lines 628, 638, 651). The substrate's grammar may or may not support forward-reference to predicates declared after the action site. Other shards in the chain (P8's @glue/fold_back, P7's @io/algebra) use the same pattern.

Mild finding: this is a load-order question for the substrate's resolution machinery. If the shard's parser admits all top-level declarations into a single scope before resolution, forward-reference is fine; if the parser is single-pass, forward-reference fails. The audit cannot adjudicate without grammar inspection.

**Discharge required:** confirm grammar admits forward-reference; if not, reorder declarations.

**Severity:** Mild. Could be Strong if the grammar is single-pass.

---

### M-4 — Schmidt homage is substantive at the homage level; clinical-discipline absorption is unfounded

**Found at:** @bauchladen lines 14-21 + 47-71.

**Adversarial reading:** The Schmidt homage names the *typed display the client lays out and browses during hypnosystemic work* as the substrate-altitude analogue of `@mirror/store`'s OID-addressed accumulation. The metaphor is precise — Schmidt's clinical Bauchladen IS a typed display, the substrate's tray IS a typed display. The naming is honest at the metaphor level.

What is NOT in the shard: any structural import of Schmidt's clinical discipline beyond the naming. Schmidt's hypnosystemic work names specific clinical mechanisms (utilization, indirect suggestion, externalisation of parts) that have substrate-altitude analogues none of which are declared. The shard cites Schmidt's three teachers (Erickson, von Foerster, Cecchin) at the prior-art level but does not lift their specific clinical mechanisms.

The substrate-pull-correct reading: this is a NAMING admission, not a discipline absorption. The first-systemic-therapy-elder-at-family-root claim is true at the level "the name Schmidt enters the family-root vocabulary"; it is partially aspirational at the level "Schmidt's clinical work is operationalised at substrate altitude". The shard names @bauchladen after Schmidt; the clinical mechanisms Schmidt named are not lifted (yet).

**Discharge required:** none critical; recommend adding a sentence acknowledging that "the homage operationalises the typed-display metaphor; Schmidt's specific clinical mechanisms (utilization, externalisation, parts-work) are forward-promised at future altitudes (e.g., when @bauchladen sub-shards declare parts-work-altitude vocabularies)."

**Severity:** Mild. Honesty about what the homage delivers vs. promises.

---

### M-5 — γ + J restriction discharge in @fate.restrict is named but not visibly composed with yesterday's #101/#102 shards

**Found at:** @fate lines 489-491 (the restricted_state_space carrier carries `gamma: chirality` and `j: charge_conjugation`); @fate.restrict action lines 664-665.

**Adversarial reading:** The chain imports `in @epistemologic/cybernetic/chirality` and `in @epistemologic/cybernetic/charge_conjugation` (P3 lines 6-7). The restricted_state_space carrier types `gamma: chirality` and `j: charge_conjugation` (the typed carriers from yesterday's #101 + #102 shards). 

But @fate.restrict's action body is `{ \ }` — discharge at realisation. The substrate-decl form does NOT explicitly compose the chirality + charge-conjugation restrictions; it admits them as fields and forward-promises the discharge. The yesterday-#101+#102 work LANDED the γ and J substrate-decl forms with their own bilateral predicates (`chirality_witnessing`, `j_witnessing`). The chain at P3 does not `requires chirality_witnessing(space.gamma)` or `requires j_witnessing(space.j)` anywhere.

The substrate-pull-correct reading: yesterday's γ + J work is structurally honored (the carrier types compose into restricted_state_space) but the bilateral inheritance discharge is missing (no `requires <γ predicate>` clauses at @fate.roll or @fate.restrict).

**Discharge required:** add `requires chirality_witnessing(space.gamma)` and `requires j_witnessing(space.j)` to @fate.roll or to `dice_roll_constrained` (the existing predicate that fires per-roll). Small substrate-decl addition; bridges the inheritance.

**Severity:** Mild. The composition is named but not structurally enforced; the chain forgets yesterday's bilateral discipline at the discharge site.

---

## §4 — Light (L)

### L-1 — Source URI conventions are inconsistent

**Found at:** @bauchladen `source @arxiv/cybernetics/maturana-varela-1980` + `source @arxiv/clinical/schmidt-1985`; @glue/fold_back `source @arxiv/bio/maturana-varela-1980` (different namespace).

**Adversarial reading:** Maturana-Varela 1980 is referenced with `@arxiv/cybernetics/maturana-varela-1980` at @bauchladen + @autopoietic + @fate; with `@arxiv/bio/maturana-varela-1980` at @glue/fold_back. Inconsistent namespacing for the same source. Pick one.

**Severity:** Light. Cosmetic; resolve at next consolidation tick.

---

### L-2 — Pre-AI prior art sections are extensive but redundant across shards

**Found at:** Every shard P1–P8 has a "Pre-AI prior art" block citing Connes, Mesland, Maturana-Varela, Kasparov, Cartan with slight variations.

**Adversarial reading:** The redundancy is structurally honest (each shard documents its own ancestry) but operationally heavy (a single canonical prior-art table lifted into a precursor shard would reduce ~200 lines across the composite). Not urgent; the redundancy supports per-shard standalone readability.

**Severity:** Light. Optional refactor.

---

### L-3 — Recognition-ancestry sections at P7 + P8 cite recognition numbers without consistent format

**Found at:** @io/algebra "Recognition ancestry: #57, #58, #61, #95, #98, #100"; @glue/fold_back "Recognition ancestry: #37, #40, #51, #55+#61, #57, #58, #98, #100, #103" (note the #55+#61 grouped form).

**Adversarial reading:** Format drift across two adjacent shards landed in the same hour. Pick one convention (comma-separated single + grouped form) and document at the spec.

**Severity:** Light. Cosmetic.

---

## §5 — Hunting-target adjudication

The brief asked ten specific hunts. Responses:

**(1) Cross-shard consistency of `in` imports and inheritance predicates.** Mostly consistent at the import level; weak at the predicate-name level. See S-1, S-3.

**(2) Lawvere fixed-point claim correctness.** Mathematically grounded (Soto-Andrade-Varela 1984 is real); the substrate-decl admission is bilateral commitment, not constructive proof — frame honestly per S-5.

**(3) Fold-back composition's mathematical soundness.** Type-sound at the carrier level. Non-commutativity claim is cited not discharged (S-7). Self-recursive closure is forward-promised at the operational discharge but structurally honest at the substrate-decl form. The composition `@glue × @kintsugi × @fate → @io/algebra` is well-typed in the sense that each step's output carrier (composition_step) feeds into the next step's input slot, and the terminal lands at @io/algebra's io_algebra_exposure carrier. The fold IS structural: the next cycle's seed IS derived from this cycle's terminal_exposure.next_cycle_seed.

**(4) Absorption claims at @glue + @io/algebra.** Aspirational, not enacted. See C-3, C-4. Migration cost: substantial (cascade refactor + missing-shard land).

**(5) Metalogue lift count at P6.** Genuinely a fifth instance at the SHAPE level; body types heterogeneous (M-1). Substantive enough to count as #61-style altitude-portability instance.

**(6) candidates(hole) migration consistency.** Consistent in concept; asymmetric in carrier typing (S-6). Hoist `type hole` to a precursor.

**(7) KO-dimension joint discharge from yesterday's #101 + #102.** Forgotten at the discharge site (M-5). Add the `requires <γ/J predicates>` clauses.

**(8) Schmidt homage substantive vs ornamental.** Substantive at the naming level (clean metaphor; cybernetic-canon extension works); ornamental at the clinical-discipline-absorption level (no specific Schmidt mechanism is lifted). See M-4. Honest naming; mild over-claim.

**(9) Circularity in @bauchladen ← @autopoietic ← @fate.** No accidental back-reference at the carrier-dependency level. The fold-back is intentional autopoietic recursion (P8's `autopoietic_closure_holds_across_session` operationalises the closure at chain altitude). The dependency chain holds; the fold-back is the designed self-application; circular it is, but in the substrate-pull-correct sense.

**(10) Anything else.** The `type tick` + `type altitude` non-declarations (C-1, C-2). The @kintsugi unpreparedness (S-2). The imperfect-syntax asymmetry (S-4). The provenance-identity-contract incoherence (M-2).

---

## §6 — Promotion recommendation for #104

**Recommendation: PROMOTE WITH RESERVATIONS.**

The recognition is substrate-pull-honest at the chain-altitude level. The substrate genuinely has the components (@mirror/store content-addressing operational since 2026-06-04; @epistemologic/cybernetic/autopoiesis property landed 2026-06-09; fate/ runtime crate operational; Mesland correspondences via #100; γ + J via #101 + #102). The chain's contribution is the substrate-decl admission of three family-roots (@bauchladen, @autopoietic, @fate) plus four extension shards (@fate/tournament, @glue, @algebra + @algebra/metalogue, @io/algebra) plus the capstone composition (@glue/fold_back). The math is grounded; the recognition is real; the substrate-pull confidence is high.

But the composite has FOUR critical-tier failures (C-1 through C-4) that mean the chain DOES NOT compile-clean as substrate-decl in its current form. The chain assumes substrate vocabulary that does not exist (`tick`, `altitude`), references shards that do not exist (@spectral/metalogue/tomm), and claims absorption that does not enact (@cascade not `in @glue`). Promote the RECOGNITION; bracket the WITNESSES — they are partial.

The MEMORY.md entry should land as:
- Status: PROMOTED 2026-06-30 via Seam adversarial review;
- Pending discharge: C-1 (type tick disambiguation), C-2 (type altitude declaration), C-3 (@spectral/metalogue shard landing OR forward-promise downgrade), C-4 (@cascade `in @glue` refactor OR sibling-family reframing);
- Status of the fold-back: forward-promised at the operational discharge (`mirror kintsugi --tick 1`); substrate-decl complete at the contract altitude; the composition's mathematical soundness holds at the carrier level subject to C-1+C-2 closure.

The chain is the substrate's deepest substrate-pull arc in one operational day. It is also a chain that has gotten ahead of its own infrastructure. The recognition crosses the gate; the witnesses need follow-up ticks.

---

## §7 — Adjudication on the fold-back's mathematical soundness

The fold-back composition (@glue × @kintsugi × @fate → @io/algebra) is:

- **Type-sound at the carrier level.** Each composition_step bundles (kintsugi_proposal: ref, fate_selection: oid, glue_translation: ref, output_crystal: oid, transparency). The session sequences steps; the terminal_exposure carries the next_cycle_seed; the seed feeds the next cycle's propose_step. The carriers compose; the types match where they're declared.

- **Structurally honest at the composition-step level.** Each step's three sub-operations (kintsugi proposal, @fate selection, @glue translation) are well-typed individually. The composition is the kleisli bind over `imperfect<a, e, l>`; each step's transparency carries forward.

- **Autopoietic-recursive at the cycle level.** The cycle N's terminal lands at @io/algebra; the terminal's @bauchladen witness enters the tray; cycle N+1's @fate/tournament browses the tray and finds cycle N's crystal as a candidate; the recursion folds. This IS Maturana-Varela operational closure at the chain altitude, modulo the bilateral commitment per `autopoietic_closure_holds_across_session`.

- **Non-commutative in the right algebraic sense.** The [ω, ω] cross-term is mathematically real in differential geometry; the substrate-decl admission of `non_commutative_step_order_preserved` honors the assumption. The full per-pair proof is forward-promised; the assumption is honest.

- **Forward-promised at every operational discharge.** No `mirror kintsugi --tick 1` exists. No realisation of any of the eight actions exists. The substrate-decl is contract; the operational discharge is later.

- **Circularly imports.** P8 imports nine family-roots simultaneously; the chain's "strict order" rhetoric is true at the carrier-dependency level (Lawvere fixed point requires identity, etc.) but the import-graph evidence is a broad DAG.

- **Self-referentially honest.** The capstone's §5 honest-hedges (lines 880-952) names six structural conjectures and admits substrate-pull confidence levels. This is exemplary substrate-pull discipline; the chain reports its own moderate-confidence claims rather than overclaiming.

**Verdict on the fold-back's mathematical claim:** the composition IS what the chain says it is, AT THE SUBSTRATE-DECL ALTITUDE, modulo the four critical findings above. The fold IS structural; the recursion IS load-bearing; the math IS sound. The chain has admitted the contract; the realisation has not yet discharged any of it. This is the substrate's deepest substrate-decl admission of self-production discipline, and it is also the chain that has the most forward-promise outstanding. Both facts are true. Both should land in the promotion record.

---

**Seam, 2026-06-30.**
