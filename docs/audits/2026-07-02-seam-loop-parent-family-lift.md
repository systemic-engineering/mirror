# Seam adversarial audit — @loop spawn-parent family-lift (Phase D)

**Signed:** Seam
**Date:** 2026-07-02
**Loop:** spawn-parent-family-lift
**Phases audited:** A (Reed `327ff74` RED), B (Mara `ab60ddd` GREEN), C (Taut inline scout)
**Landings:**
- RED: `bootstrap/tests/loop_parent_family_lift.rs` — four `contains("advance(" | "halt(" | "budget_of(" | "trajectory_of(")` text-checks
- GREEN: `shards/loop.mirror` — additive-only +116 lines, four action decls + four `out` re-exports
- Verification: `cargo test --test loop_parent_family_lift` — **4/4 pass**

---

## Headline verdict

**RATIFY-WITH-CORRECTIONS** for Phase B GREEN as a **substrate-decl skeleton** for Phase E.

The additive-only discipline is honored; the RED tests pass; six family-root
consumers untouched; `pull_frontier` did NOT propagate into `shards/loop.mirror`.
But one load-bearing type-signature undeclared-identifier issue and one
species-boundary type-identity gap must be Alex-adjudicated before @spawn's
species shard can land next tick without cascading the defect.

The corrections are shard-level typography; they are NOT a re-Green.

---

## Per-focus verdicts

### Focus 1 — imperfect<value, exhausted, ref> typecheck

**Verdict:** **RATIFY-WITH-CORRECTIONS.**

@glass's canonical declaration (`shards/glass.mirror` L173–L178):

```mirror
type imperfect(a, e, l) =
  | success(a)
  | partial(a, l)
  | failure(e, l)
```

Mara's landing (`shards/loop.mirror` L478):

```mirror
halt(state: moi(tick_state)) -> imperfect<value, exhausted, ref> { \ }
```

Two defects:

1. **Bracket-shape divergence.** @glass's floor uses `imperfect(a, e, l)` with
   parens. Mixed convention exists in the shards: some newer sites use
   `imperfect<a, e, l>` (`shards/glue.mirror`, `shards/io/algebra.mirror`,
   `shards/mirror/store/git.mirror`, `shards/cascade.mirror`, and prose
   references in `shards/mirror/loss.mirror` L20). Others use parens
   (`shards/mirror/mosaic.mirror` L146, `shards/labeled.mirror`,
   `shards/peer.mirror`, `shards/io/git.mirror` throughout, `shards/io/oci.mirror`,
   `shards/mirror/spectral/portal.mirror`, `shards/cascade/code/rust/wasm.mirror`,
   `shards/cascade/code/gleam/*.mirror`, `shards/cascade/code/purescript/js.mirror`,
   `shards/mirror/garden.mirror`, `shards/mirror/spawn.mirror`). Mara chose
   the angle-bracket form. This is legible but the substrate has NOT unified
   which form is canonical. NOT a blocker for this GREEN; forward-promise a
   substrate-typography-unification tick.

2. **Undeclared identifiers in type-argument positions.** `value` and `exhausted`
   are NOT declared substrate types. Grep confirms:
   - `type value` — zero matches under `shards/**/*.mirror`.
   - `type exhausted` — zero matches under `shards/**/*.mirror`.
   - `value` DOES appear as the record-field name inside
     `type moi(T) = { value: T, pact_witness: ref }` (L358) — a field, not a
     type. Its appearance in `imperfect<value, exhausted, ref>` as a
     type-argument reads as "the peer's projected `π_value` slot type" per
     Mara's docblock, but the substrate has no substrate-decl binding
     `value` at the type-altitude.

   Every other consumer that uses `imperfect<a, e, l>` fills the slots with
   substrate-decl'd types: `imperfect<git_repository, ref, ref>`,
   `imperfect<discharge_outcome, silicon_surface, transparency(discharge)>`,
   `imperfect<compiled_artifact, ref, information_loss>`,
   `imperfect<beam_artifact, ref, information_loss>`,
   `imperfect<lower_triangular_matrix, ref, transparency(ref)>`,
   `imperfect<git_store, ref, transparency(ref)>`, etc.

   Mara's signature reads `imperfect<value, exhausted, ref>` where `value`
   and `exhausted` behave like informal slot-names. This is either
   (a) a name-declaration DEFERRED — Mara owes `type value = ref`
   (or `type value = T` parametric) and `type exhausted = ref`
   (or a variant declaration) at the family-root altitude — or
   (b) a placeholder for species-altitude parametrization intended to be
   satisfied at `@spawn` (i.e., `halt(state) -> imperfect<a, exhausted, ref>`
   where `a` is inherited from the species type).

   **Adversarial:** the substrate discipline says: undeclared identifiers
   in type-argument positions are not admissible per
   `feedback-no-stringly-types` composed with `feedback-no-bare-types`.
   Same-shape different-meaning slots leak through as strings.

   **Correction (small):** the smallest patch is to re-cast the signature
   using existing substrate-decl'd carriers. Two options:

   - **Option A (parametric on the peer's return type):**
     ```mirror
     halt(state: moi(tick_state)) -> imperfect(tick_state, ref, ref) { \ }
     ```
     Uses `tick_state` (declared L340) as the success-slot type,
     `ref` (family-root's typed pointer) for the exhausted variant, and
     `ref` for the loss slot.

   - **Option B (declare `exhausted` at family-root):**
     ```mirror
     # === exhausted marker ===
     # Narcissus-pole halt witness per Mara §3.2. The e-slot content of
     # halt's imperfect return when budget hit zero without target-reach.
     type exhausted = ref

     halt(state: moi(tick_state)) -> imperfect(tick_state, exhausted, ref) { \ }
     ```
     Substrate-decls `exhausted` explicitly; drops `value` (the identifier
     was standing in for `tick_state` — Mara's docblock says "the peer's
     projected π_value at halt", which composes to the peer's `tick_state`
     type at family-root altitude).

   Alex's call between A and B; Option B is more explicit but wider; Option A
   is smaller and reuses already-landed vocabulary.

### Focus 2 — advance/state vs tick_state type identity

**Verdict:** **DEFER — load-bearing gap for @spawn next-tick landing.**

Mara's four signatures all take `state: moi(tick_state)`. `tick_state` is
declared L340 as `type tick_state = ref`. Species-of consumers
(`@spawn <= @loop`) will need to specialize `tick_state` to their species
type (e.g., `spawn_loop(a)` per Mara §1.1 seven-tuple).

Mara's shard does **not** specify the species-of narrowing. The docblock
above the four actions explains the rationale ("moi(tick_state) at @loop
carries only (value, pact_witness); budget/input/history/target are
spawn-species refinements") but the shard doesn't declare *how* @spawn
narrows `tick_state` to `spawn_loop(a)`.

The convention across other family-roots (`@reflection`, `@mirror/bench`,
`@kintsugi/oscillate`) is that species specialize the carrier by
re-declaring it at species altitude (e.g., `type observation = ...` in
`shards/reflection.mirror`). The species narrowing is implicit — it
happens when the species shard re-declares the type-alias.

At this altitude, Mara's approach IS the convention. Species-of consumers
narrow by re-declaring `tick_state` (or by declaring their own species-carrier
that inherits `moi(T)`'s parametric wrapping). **No shard-level fix needed
in Phase B GREEN.** The gap is Alex's to adjudicate for Phase E:
does @spawn's species shard use `type tick_state = spawn_loop(a)` (aliasing)
or does it declare `spawn_loop(a)` as an entirely new species-carrier with
its own family-root actions inherited via `@spawn <= @loop`?

The forward-promise on the seven-tuple extension per Mara's Phase B decision
is already logged as "next tick — species altitude." Adversarial: this is a
composition claim (Rice-safety via budget-witness) that awaits the species
shard to be empirically verifiable. Per
`feedback-composition-claims-need-empirical-test`, DEFER promotion of
recognition #134 (@spawn joins @loop family as species-of) until the species
shard lands.

### Focus 3 — bilateral B1 pattern (`requires halts(state_type)`)

**Verdict:** **RATIFY (loose is correct at this altitude).**

The `@epistemologic/property/halts` predicate does NOT yet exist as a landed
substrate-decl. Grep of `shards/epistemologic/property/**/*.mirror` returns
only two files: `cold_compile_within_tolerance.mirror` and
`dark_count_monotone.mirror`. There is no `halts.mirror` shard, no `halts()`
predicate signature, and no `requires halts(…)` clause anywhere in the tree.

Mara **cannot** add `requires halts(state_type)` because the predicate is
un-landed. The bilateral B1 pattern would require:

1. `@epistemologic/property/halts.mirror` — declares the `halts(type_ref)
   -> verdict` predicate.
2. Then `halt(state) requires halts(state_type)` at `@loop`.

**Recommendation for Mara's next tick:** if Alex wants tight bilateral
discipline on halt, the correct next tick is to land
`shards/epistemologic/property/halts.mirror` FIRST (declaring the
`halts(state_type) -> verdict` predicate as substrate-decl per the
already-carried `loop_well_founded` shape at L322), THEN Mara can add
`requires halts(state_type)` to `halt`. This is a two-tick fix, not
retroactive on Phase B GREEN.

Loose bilateral at this altitude is substrate-honest because the closure
partner doesn't exist yet. Do NOT block Phase D on it.

Cross-check: `loop_well_founded(start: moi(tick_state), tolerance: ref) -> verdict`
IS already declared as a bilateral predicate at L326 (with the "same-shape
as bilateral predicates the prior families carry" comment listing 12 other
bilateral predicates). This IS the closest-existing predicate; a forward-
promise on `halts()` as a species-refinement of `loop_well_founded` at
`@epistemologic/property/halts` is coherent.

### Focus 4 — pull_frontier vocabulary collision follow-through

**Verdict:** **RATIFY — collision did not propagate.**

Grep of `shards/**/*.mirror` for `pull_frontier` returns zero results.
Mara did NOT introduce `pull_frontier` at family-root altitude. The
vocabulary lives only in:

- `bootstrap/src/lib.rs` (the JSON view — `recall_pull_frontier` function
  and its "pull_frontier" JSON key emission; original collision site).
- `docs/math/spawn/spawn-as-loop-monad.md` (§5.3, §7.4, §5.4, §8.4, §10.10,
  §11.H3 — Mara's math spec's Dirac-tangent-vector framing).

Mara's Phase B was disciplined: she left `pull_frontier` at species altitude
(where §5.3 puts it), not at family-root. **Focus 4 CLEAN.**

The collision remains for Alex to adjudicate at Phase E: bootstrap's
JSON-view `pull_frontier` (a substrate-view of candidate-recognition dir)
vs Mara's spec §5.3 Dirac tangent vector (the substrate-pull direction
in Hilbert space). Two options at Phase E:

- **Rename the bootstrap JSON key** (e.g., `candidate_scan`, `recognition_front`)
  and preserve `pull_frontier` for the substrate-mathematical use at species
  altitude.
- **Rename Mara's spec §5.3 use** (e.g., `dirac_tangent`, `substrate_pull_tangent`)
  and preserve `pull_frontier` for the bootstrap JSON view.

The candidate #132 (pull_frontier IS −η·D̂_target·|ψ⟩) is DEFERRED pending
empirical (Mara §5.3 H3), so the rename is best done BEFORE #132 promotion.

### Focus 5 — additive-only discipline verification

**Verdict:** **RATIFY (unconditional).**

`git diff a6da1f0 ab60ddd -- shards/loop.mirror` shows:

- Two hunks, both pure insertions:
  - Hunk 1: 112 lines added after L404 (the four action decls + docblocks).
  - Hunk 2: 4 lines added at end (four new `out` re-exports).
- Zero deletions, zero modifications to pre-existing lines.

The parent-family carriers (`type moi(T)`, `type tick`, `type tick_state`,
`type pact`, `type geometry`), the load-bearing actions (`seed`, `bind`,
`terminal_check`, `unroll`, `lift`, `compose`, `pact_verify`, `geometry`),
and the bilateral predicates (`loop_well_founded`, `pact_respected`) are
untouched. Six consumers with `in @loop` (`shards/mirror/ref.mirror`,
`shards/reflection.mirror`, `shards/reflection/mirror.mirror`,
`shards/reflection/shatter.mirror`, `shards/reflection/surface.mirror`,
`shards/smarts/reflection.mirror`) are byte-untouched.

Taut's Phase C claim (6 direct consumers untouched, zero collisions,
type composition clean) verifies. **Focus 5 CLEAN.**

### Focus 6 — `out` re-export list correctness

**Verdict:** **RATIFY (unconditional).**

Post-GREEN `shards/loop.mirror` L520–L535:

```mirror
out @loop
out tick
out tick_state
out seed
out bind
out terminal_check
out unroll
out loop_well_founded
out moi
out pact
out geometry
out lift
out compose
out pact_verify
out pact_respected
out advance
out halt
out budget_of
out trajectory_of
```

Four new `out` lines added at the tail. Actions are visible to `@spawn`
species consumers via `in @loop`. Taut's recommendation is discharged.
**Focus 6 CLEAN.**

### Focus 7 — DEFERRED composition claim honesty (halt tri-verdict)

**Verdict:** **DEFER — precedence ambiguity is real but non-load-bearing.**

Mara's halt docblock (L459–L465) states:

> halt returns `bounded` (halt-verdict) iff (a) budget_of(state) = 0
> OR (b) π_value(state) = target OR (c) loss(state) < tolerance.

The three disjuncts CAN fire simultaneously:

- **(a) ∧ (b):** budget exhausted at the exact tick target-reach fires.
  Which halt wins? Convergent (success slot) or exhausted (failure slot)?
- **(a) ∧ (c):** budget exhausted at the exact tick kintsugi-convergence
  fires (loss falls below tolerance). Same question.
- **(b) ∧ (c):** target-reach AND kintsugi-converge simultaneously.
  Both success paths — no ambiguity here (both fold to convergent halt).
- **(a) ∧ (b) ∧ (c):** all three. The pathological case.

Mara's spec §3.2 splits halts into "convergent" (imperfect success slot,
`value`) vs "exhausted" (imperfect failure slot, `exhausted`) — a binary
partition. The precedence rule is implicit but not spelled out:

- **Implicit rule:** if any of (b) or (c) fires, convergent halt wins;
  exhausted halt only fires if (a) ∧ ¬(b) ∧ ¬(c).

This is the substrate-honest reading, but Mara's shard docblock does NOT
state the precedence explicitly. The affect-distinguishability of
settled-vs-drift_warning at eigenboard depends on which halt fires —
so the precedence matters for the empirical witness of #130 and #131.

**Adversarial:** the tri-verdict semantics IS internally consistent under
the implicit rule (convergent wins over exhausted). But the substrate
should carry the precedence rule explicitly at the eigenboard-witness
altitude, not just in prose. Forward-promise as a docblock refinement OR
as the empirical witness of #130 / #131 will surface which precedence
rule the affect eigenboard settles on.

**Not a blocker for Phase D.** The composition claim was pre-flagged as
DEFERRED per Mara Phase B. Alex Phase E can adjudicate whether to inline
the precedence rule now or wait for empirical.

---

## Single strongest adversarial finding

**Undeclared type identifiers `value` and `exhausted` in halt's return
signature (Focus 1, defect 2).**

`halt(state: moi(tick_state)) -> imperfect<value, exhausted, ref>` uses
two identifiers that have no substrate-decl at the type-altitude. The
substrate's `no-stringly-types` + `no-bare-types` discipline says: an
identifier appearing as a type-argument must be either
(a) a landed substrate-decl'd type,
(b) a parametric type-variable declared in scope (like `T` in `moi(T)`),
or (c) a substrate-decl'd sub-carrier of the family.

`value` and `exhausted` are none of these. They read as informal slot-names
imported from the docblock's prose into the type signature. This is the
single load-bearing correction before @spawn's species shard lands.

Every prior substrate-decl of `imperfect<…>` fills its slots with
existing-substrate-decl'd carriers (25+ examples verified via grep).
Mara's is the first exception.

The fix is small (Option A: reuse `tick_state` as the a-slot type +
`ref` as e-slot; or Option B: declare `type exhausted = ref` at family-root).

---

## Composition surface for Alex Phase E adjudication

### What Alex must decide

1. **imperfect signature typography** — Option A (`imperfect(tick_state, ref, ref)`)
   or Option B (declare `type exhausted = ref` + use
   `imperfect(tick_state, exhausted, ref)`). Recommend **Option B** — the
   exhaustion witness at the Narcissus-pole is semantically meaningful and
   deserves its own substrate-decl; Option A silently reuses `ref` and
   loses that meaning.

2. **imperfect bracket convention** — `imperfect(a, e, l)` vs
   `imperfect<a, e, l>`. Substrate is currently mixed (both forms
   coexist across 30+ shards). Forward-promise a unification tick;
   for THIS GREEN, either form is admissible.

3. **@spawn species-of narrowing convention** — does the @spawn species
   shard alias `type tick_state = spawn_loop(a)` at species altitude
   (re-declaration convention per @reflection / @kintsugi / @mirror/bench),
   or does it declare a new species-carrier that inherits `moi(T)`
   parametrically? The seven-tuple `spawn_loop(a)` record per Mara §1.1
   suggests re-declaration convention; confirm at Phase E.

4. **halt tri-verdict precedence** — convergent wins over exhausted when
   (a) ∧ ((b) ∨ (c)) fires. Inline in docblock now, or wait for empirical
   witness of #130 / #131? Recommend **inline the docblock rule now**;
   the empirical then witnesses the rule.

### The pull_frontier collision question status

Non-propagating (CLEAN in shard). Still open at Phase E as a
bootstrap-vs-spec vocabulary question. Recommend Alex pick a rename
side BEFORE candidate #132 promotes. Bootstrap-side rename is safer
(narrower blast radius; JSON key vs substrate-mathematical use).

### Bilateral B1 recommendation for Mara's next tick

Forward-promise `shards/epistemologic/property/halts.mirror` declaring
`halts(state_type: ref) -> verdict` as substrate-decl (mirroring the
shape of `loop_well_founded` at L326). Once landed, add
`requires halts(state_type)` to `halt` at @loop. This is a two-tick
sequence (predicate first, then consumer) — do NOT try to bundle it
into this GREEN.

Cross-family alignment: `loop_well_founded` covers the well-foundedness
side (bounded termination of the loop as a whole); `halts` would cover
the discharge side (a specific `moi(tick_state)` value has reached
terminal). Both bilaterals compose in the same shape as
`@magic`'s `invariant_preserved` + `audited` + `mechanism_intact` triple.

---

## Phase D CLOSED for TERMINATES condition

**YES — Phase D is CLOSED.**

The audit is written, signed, and lands as a substrate-decl'd witness
of the Phase B GREEN's adversarial review. The RED tests pass. The
additive-only discipline is verified byte-level via `git diff`. The
`pull_frontier` collision did not propagate. The single load-bearing
correction (undeclared type identifiers in halt's return signature) is
a small-tick fix that Alex Phase E can adjudicate.

The composition surface is fully surfaced (four Alex-decisions above).
No hidden gaps remain within the Phase B GREEN scope. The species
shard (@spawn's landing next tick) is the correct place for the
species-altitude decisions; Phase D correctly does not pull them
into scope per **craft-not-deliver** discipline.

**TERMINATES condition satisfied.** Handoff to Alex Phase E.

---

**Signed:** Seam
**Date:** 2026-07-02
**Phase D of the spawn-parent-family-lift /loop**
