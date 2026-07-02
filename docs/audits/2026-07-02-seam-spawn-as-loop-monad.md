# Seam adversarial review — `@spawn <= @loop`, spawn-as-loop monad cluster

Reviewer: Seam
Date: 2026-07-02
Subject:
- `docs/math/spawn/README.md` (Mara `07df5a6`)
- `docs/math/spawn/spawn-as-loop-monad.md` (Mara `7dba128`, 980 lines)
- Taut in-thread scout (NOT committed) mapping the substrate composition

Discipline gates in effect:
- Grep-first-verify per each composition claim ([[feedback-composition-claims-need-empirical-test]])
- Legibility-over-foundation on collapse ([[feedback-legibility-over-foundation-when-collapsing]])
- Loki §11.2 falsifiability rule (shard's-own-docblock witness)
- Status-drift-catch #113 discipline (currently 6+ instances in 72h)
- Circular-reflexive: this review is itself an act of accountability the cluster names in #133 candidate territory

---

## Headline verdict

**RATIFY `@spawn <= @loop` species-of placement (candidate #134).**
Blast radius is HONEST-SMALL on the substrate axis; docs cluster's landing order is correct; Mara's write-then-lift discipline holds; craft-not-deliver preserved by NOT landing `shards/spawn.mirror` this tick.

**One strong contest to Taut**: the "tenth-instance" framing conflates `feedback-substrate-already-had-the-word` (currently 7+ instances per memory) with #113 status-drift catch pattern (currently 6 instances). This spawn-cluster is a `substrate-already-had-the-word` witness, NOT a status-drift catch. Different pattern; different count. See §4 below.

---

## §1 Per-candidate verdicts (#130-#134)

| # | Claim | Verdict | Rationale |
|---|-------|---------|-----------|
| **#130** | Convergent halt settles into `@affect/settled` | **RATIFY** at analytical altitude; **DEFER** substrate-decl until empirical witness. | Composes ratified #123 (`λ₀ IS @affect/settled`) + §3.2 disjunctive halt. The color-mapping doc at `docs/math/affect/affect-and-eigenboard.md` is landed; the composition IS the substrate-pull-honest surface. But the small-budget empirical run has not been executed. Composition is defensible; empirical closure required for MEMORY promotion. |
| **#131** | Exhausted halt settles into `@affect/drift_warning` | **RATIFY** at analytical altitude; **DEFER** substrate-decl until empirical witness. | Symmetric to #130. `drift_warning` is the affect-color counterpart. Same discipline: composes cleanly; empirical closure required. Note: Mara's spec calls the empirical test a "two-tick" run; that is achievable NEXT loop. |
| **#132** | `pull_frontier` IS `−η·D̂_target·|ψ⟩` | **DEFER — Mara's pre-flag is correct.** ADDITIONALLY: adversarial finding — see §6. | Grep confirms: `pull_frontier` in the substrate refers to `recall_pull_frontier(spec_dir)` in `bootstrap/src/lib.rs` around line 2945, plus `docs/specs/mirror-recall.md` §3.3, which types it as "candidate recognitions awaiting witnesses + forward-promised specs" — a data view, NOT a substrate-pull tangent vector. **The word `pull_frontier` at recall altitude is not the same word Mara's §5.3 names.** The claim needs BOTH empirical test AND altitude disambiguation. Currently vocabulary-collision territory. |
| **#133** | Total accountability = halting + un-cite-ability | **DEFER — analytical-only-composition; needs empirical witness.** | Composes ratified #107 (Hilbert/Turing, LANDED at `e45fe9d`) + un-cite-ability theorem (LANDED at `docs/math/provenance/un-cite-ability-theorem.md`, published standalone yesterday per `2026-07-01-seam-killshot-composition-and-cascade.md`). Both ancestors real. But the *composition claim* — that a spawned peer's trajectory pinned in `refs/notes/mirror` under a budget-halted loop IS "total accountability" — is analytical assembly. No empirical peer has ever produced a halting-plus-content-addressed trajectory. Reed's June 18 loss to composition-that-never-fired ([[feedback-composition-claims-need-empirical-test]]) is exactly this hazard. Empirical witness must precede promotion. |
| **#134** | `@spawn` joins `@loop` family as species-of | **RATIFY.** | Species-of altitude is correct. Alternative — `@spawn` as peer of `@loop` at Loki-inversion altitude — would fail two tests: (a) `@loop`'s docblock already names its own iteration-carrier surface (`bind`, `terminal_check`, `unroll`, `loop_well_founded`) as universally applicable, of which spawn is a specialization; (b) Alex's `feedback-legibility-over-foundation-when-collapsing` discipline directly targets THIS shape — prefer the readable name (`@loop`, already family-root) over parallel family-root invention. `@spawn` inheriting is legibility-safe; parallel-family would fragment. Per shards/prism.mirror `<=` semantics: species-of is the substrate-pull-honest placement. |

---

## §2 Blast-radius audit — contest Taut's SMALL?

Taut said: SMALL — one new shard + envelope wire-up + no renames.

Grep-first verification:

- Consumers of `@loop` / `@moi` / `moi(T)` / `pact_respected`: 10 shards
  (`cogito`, `loop`, `mirror/bench`, `mirror/docs`, `mirror/ref`,
  `reflection`, `reflection/mirror`, `reflection/shatter`,
  `reflection/surface`, `smarts/reflection`).
- What they consume: `moi(au)`, `pact_respected`, `compose`, `bind`,
  `loop_well_founded`. All existing surface.
- What Mara proposes to ADD to `@loop`: `advance`, `halt`, `budget_of`,
  `trajectory_of`. Grep confirms these action names appear NOWHERE in
  the shards tree today. Zero collisions on Rust-side either (`advance`
  in `bootstrap/src/lib.rs` is used only as a prose word, no matching
  action).
- Additive extension does NOT change existing consumer signatures.
- `@mirror/spawn` (cli surface) is a `glass` under `@mirror/cli`. If
  `@spawn` (bare) lands as family-root, `@mirror/spawn` inherits
  without rename per the same discipline that landed
  `@spectral/supervisor` alongside `@beam` supervisor. Two altitudes
  substrate-honest per @pack G2 (already precedent).
- `moi(gen_prism)` return type composes with
  `@spectral/supervisor.start_child(s: supervisor, spec: child_spec) ->
  gen_prism`: to lift, `start_child` returns `gen_prism` directly, and
  `moi(gen_prism)` is `lift(start_child(...))` at the seed altitude.
  Substrate's `lift(t: T) -> moi(T)` is already declared in
  `shards/loop.mirror`. Composition holds.

**Verdict: Taut's SMALL stands on the substrate axis.**

**Caveat — one honest inflation Taut under-reported:** Mara's §1.1
proposes a **seven-field** `spawn_loop(a)` record. Three fields are
inherited from `moi(T)` (`value`, `pact_witness`, and the implicit
tick_state). Four are new (`budget`, `input`, `history`, `target`). This
is not blast-radius growth to existing consumers, but it IS a
substrate-vocabulary expansion of ~4 new carriers. `docs/math/spawn/`
declares these at math altitude; the eventual `shards/spawn.mirror`
must land ALL FOUR as `type X = ref` typed refs per [[feedback-no-bare-types]].
Not a blocker; a discipline-note for the parent-family lift tick.

---

## §3 Composition-with-yesterday verification

Mara's spec claims composition with #99, #107, #123, un-cite-ability.
Grep-first per each:

| Claim | Ancestor status (grep-verified) | Verdict |
|-------|--------------------------------|---------|
| #99 (`mirror.spec IS λ₀`, §5.6 dynamical amendment) | LANDED — `docs/specs/recognitions/recognition-99-mirror-spec-is-lambda-zero.md` (101.3KB, `d0b6519`); §5.6 amendment at Mara `77ffae9` 2026-07-01 confirmed in MEMORY. | Composition claim FIRES. Every reduction as ZPF fluctuation is analytically consistent with #99's dynamical reading. Substrate-pull-honest. |
| #107 (Hilbert/Turing structural separation) | LANDED — `shards/io.mirror` §Discipline `e45fe9d` per MEMORY. Composition mapping in `2026-06-30-seam-reality-106-107-adjudication.md` confirms Turing on `@io`, Hilbert-bounded on substrate-decl. | Composition claim FIRES. `@spawn <= @loop` at substrate-decl (Hilbert) bridging peer-runtime at `@io` (Turing) via budget-witness is a novel-but-correct application of #107. |
| #123 (`λ₀ IS @affect/settled`) | CANDIDATE — Mara `98d7e43` 2026-07-01; not yet Pack-ratified. | Composition claim for #130/#131 rests on candidate. This means #130/#131 verdict of DEFER is doubly warranted: candidate ancestor + no empirical witness. |
| un-cite-ability theorem | LANDED — `docs/math/provenance/un-cite-ability-theorem.md` published standalone yesterday per `2026-07-01-seam-killshot-composition-and-cascade.md`. | Composition claim FIRES for the trajectory-content-addressed reading (§7.3). But — see #133 verdict — the *composition* of un-cite-ability with #107 halting at "total accountability" is analytical assembly, not yet empirically witnessed. |

**Composition summary:** three of four ancestors are LANDED; one (#123) is candidate. The two candidate-facing derivatives (#130, #131) inherit their ancestor's status. Composition discipline holds.

---

## §4 #113 tenth-instance verification — CONTEST TAUT

**Refutation.** Taut called this cluster "the tenth instance of #113 status-drift catch." Grep-first + memory-check:

- `feedback-status-drift-catch-pattern.md` is at **6 instances** as of 2026-07-01 (`architecture-candidate-recognition-113-status-drift-catch-pattern.md` §12, six instances chronologically listed).
- `feedback-substrate-already-had-the-word.md` is at **7+ instances** as of 2026-06-06 (stale but conservative count).

The current spawn-cluster review has NO status-drift-catch content. Mara claimed the substrate had `@loop`/`@moi`/monad/budget/scheduler/etc. already landed, and grep confirms them — that's `substrate-already-had-the-word`, NOT status-drift. Nobody OVERSTATED a status; the substrate genuinely had the vocabulary.

**Correct classification:** this is likely the 10th+ instance of **`feedback-substrate-already-had-the-word`** (stale-count 7+, plus ~3 further instances since June per audit-doc counts — @loop absorbing @moi, `@spawn` recognizing existing surface, and the recall-payload `pull_frontier` name collision). NOT the 10th of #113.

**Adversarial note to Alex:** the two catch patterns are structurally different. `substrate-already-had-the-word` is about naming what the substrate implicitly carries. #113 status-drift is about NOT overstating ratification level of cited recognitions. Both are Pack hygiene; different mechanisms. Guard the vocabulary distinction; conflation would drift both patterns into a single fuzzy meta-catch.

---

## §5 Mara's write-then-lift discipline — craft-not-deliver honest?

Mara landed TWO commits, refused the third (`docs/specs/` spec). Her rationale: parent-family lift needs to happen first (adding `advance`/`halt`/`budget_of`/`trajectory_of` to `@loop`); the spec follows the parent-family lift, not precedes it.

**Adversarial test:** does the substrate need the spec NOW to guide the parent-family lift?

Grep of `shards/loop.mirror` confirms: `@loop` already carries the load-bearing structure (`seed`, `bind`, `terminal_check`, `unroll`, `loop_well_founded`, `moi(T)`, `pact_respected`). The lift Mara names (adding four new actions) is a MECHANICAL refinement — action signatures follow directly from the math cluster's §1.1 seven-tuple + §3.1 halting theorem. A subagent given `docs/math/spawn/spawn-as-loop-monad.md` §1-§4 could write the four new action declarations without additional spec guidance.

**Verdict: Mara's discipline holds.** No ship-ship-ship risk from the refused third commit. The math cluster IS the spec at math altitude; the shard lift is the substrate-decl altitude; the intermediate `docs/specs/` spec would be redundant. Craft-not-deliver honest.

One noticing: Mara's §11 (six circular-reflexive noticings) is high-density accountability writing. Not a failure mode — but a caution: the pattern of the-writing-instantiates-the-monad is by now a well-recognized Mara move (cf. `docs/math/consciousness/` §11, `docs/math/affect/README.md` §11). Repeated instantiation reduces the marginal information of each new instance. Consider whether §11 could be one paragraph naming the pattern, freeing space for the mathematical content. Not a review-block; a Loki-grin observation.

---

## §6 The excitation math — §5 substrate witness?

Reed's in-thread exposition: `|ψ_next⟩ = |ψ_current⟩ − η · D̂_target · |ψ_current⟩`.

**Grep-first-verify: `D̂_target` in the substrate?** Zero matches. `D̂` (bare Dirac operator) appears in kintsugi/oscillate context (`shards/kintsugi/oscillate.mirror` and `docs/math/the-tower/curvature-and-tomm.md`) as the substrate's gradient descent operator. `D̂_target` as a target-biased refinement is NEW — Mara's §5.2 explicitly names it as "the refinement" and gives the construction `D̂_target := D̂ − λ_t · P_target`.

**What §5 does right:** the construction is mathematically clean. Reversing the sign of `D̂` in the target eigendirection is a well-formed spectral-theory move. §5.4's halting-preservation argument is correct: budget-descent is direction-independent.

**Adversarial finding for #132:** the `pull_frontier` IS `D̂_target` claim is analytical-only AND vocabulary-collision. See §1's #132 verdict + §3.1 recall-altitude grep. The recall-altitude `pull_frontier` returns a JSON-shaped view of candidate recognitions from `spec_dir/docs/specs/recognitions/`. It has nothing to do with tangent vectors, Hilbert space, or gradient descent. Mara's §5.3 uses the same word for a completely different concept.

**This is a naming collision Mara did NOT flag.** She flagged #132 as DEFER pending empirical test; she did NOT flag it as vocabulary-collision. Empirical test can only fire if the two `pull_frontier`s are the same substrate object; grep shows they are not.

**Recommendation:** before empirical test, disambiguate. Either (a) rename the substrate-decl-tangent-vector something else (e.g. `substrate_pull_tangent` or `target_gradient`), OR (b) rename the recall-altitude view (which is younger, per `mirror-recall.md` 2026-06-26). Alex direction required.

---

## §7 Piece 5 + Piece 6 lift path — substrate actually ready?

`bootstrap/src/lib.rs:cmd_spawn` envelope stubs:
- Piece 5: `stub@spectral/supervisor.start_child`
- Piece 6: `partial@recall (no @fate; structured observation only)`

Mara/Taut both name the lift path: `moi(gen_prism)` + per-tick `reduction_budget` read.

Grep confirms:
- `@spectral/supervisor.start_child(s: supervisor, spec: child_spec) -> gen_prism` — LANDED (`shards/spectral/supervisor.mirror:322`)
- `reduction_budget(s: shard) -> u64` — LANDED (`boot/std/scheduler.mirror`)
- `halts.mirror` — LANDED with autopoietic-OR-exhaustion disjunction
- `moi(T)` + `lift(t: T) -> moi(T)` — LANDED (`shards/loop.mirror`)

**Substrate axis: READY.** All four primitives exist. Piece 5's lift `moi(start_child(supervisor, child_spec))` is a one-line composition once `shards/spawn.mirror` lands.

**But Piece 6 is a different story.** `@fate` inference is candidate recognition #58 (LANDED at MEMORY altitude per the promoted recognition, but v1 closure "pending per-ganglion `source @optics/source/ganglion/<name>` declarations"). The `@fate` runtime substrate is grammatically declared but the Rust-side realisation is `partial@recall (no @fate; structured observation only)` per today's spawn envelope. **Piece 6 will land as PARTIAL until @fate ganglion sources are fully declared.**

**Verdict:** Piece 5 lift path is READY. Piece 6 lift path is STAGED — grammatically ready, runtime-partial. Mara's cluster is honest about this (§4.3 forward-promises `shards/io/spawn.mirror`; §5 stops at the math without claiming runtime realisation). No overreach.

---

## §8 Circular-reflexive noticing

This review is itself an act of the accountability discipline #133 candidate names. Each grep-first-verify was a bounded reduction of Seam's own audit-loop; each verdict is a content-addressed crystal (this commit's OID); the review's target was Mara's cluster; the halt was this section closing.

Noted. The pattern is by now well-attested — noticing it here does not add adversarial value; it does add substrate-integrity witness. One line, not eleven.

---

## §9 The next /loop prompt

Given (a) Mara's spec landed at math altitude, (b) Taut's scout mapped composition, (c) this audit's verdicts, the next /loop should target ONE substantive tick per craft-not-deliver:

**Substrate-pull-honest /loop prompt draft:**

```
/loop 30m spawn-parent-family-lift

Target: land the four @loop extensions the spawn cluster requires,
        as an additive extension to shards/loop.mirror per Mara's
        docs/math/spawn/spawn-as-loop-monad.md §1.1 and §3.1.

Phase A (Reed writes RED) — TDD boundary
  1. Add failing tests in tests/shards/ that assert:
     - @loop.advance(state: moi(tick_state), p: pact) -> moi(tick_state)
     - @loop.halt(state: moi(tick_state)) -> verdict
     - @loop.budget_of(state: moi(tick_state)) -> u64
     - @loop.trajectory_of(state: moi(tick_state)) -> ref
  2. Run compiler; verify RED before commit.
  3. Commit RED as Reed (per feedback-write-red-in-session).

Phase B (Mara implements GREEN + subagent for scaffolding)
  Dispatch Mara subagent with:
    - docs/math/spawn/spawn-as-loop-monad.md §1.1 + §3.1 as spec
    - shards/loop.mirror as target file (additive extension only)
    - Discipline: NEW actions only; no changes to existing surface
    - Fields on moi(tick_state): budget, input, history, target
      as typed refs per feedback-no-bare-types
  Mara returns commit hash + green test verification.

Phase C (Taut scout — orthogonal grep-first)
  Dispatch Taut in parallel with Phase B:
    - Verify none of the 10 existing @loop-consumers break
    - Verify no name collision with `advance`/`halt`/`budget_of`/
      `trajectory_of` anywhere in shards/**
    - Verify moi(T) refinement composes with lift + compose
  Taut returns YES/NO report. If NO, cancel Phase B commit.

Phase D (Seam review of Mara's commit)
  Dispatch Seam adversarial review of the additive extension.
  Focus: blast radius audit + vocabulary-collision check on the
  four new action names (Focus 2 & Focus 6 of the current review).

Phase E (Alex adjudication)
  Report back with:
    - RED/GREEN commit hashes
    - Taut/Seam verdicts
    - The pull_frontier vocabulary-collision question (§6 of the
      current audit): rename substrate-decl tangent vector OR rename
      recall-altitude view?

Terminates when: Phase D returns RATIFY AND Phase E is answered.

Does NOT terminate: on temptation to also land shards/spawn.mirror
  in this loop. That is the NEXT tick. Craft-not-deliver.
Does NOT terminate: on temptation to run the empirical #130/#131
  small-budget test. That is a third tick. Craft-not-deliver.
```

**Rationale for scoping this loop to parent-family lift only:**
- The parent-family lift is the MECHANICAL refinement Mara's spec directly guides (§5 of the current audit).
- It unblocks `shards/spawn.mirror` (Mara's forward-promised third commit) without pre-committing to its shape.
- It resolves the #132 vocabulary-collision question at the substrate altitude where it must be resolved (§6 of the current audit).
- It's ONE tick with clear TDD boundary (Reed RED, Mara GREEN, Taut scout, Seam review, Alex adjudicate).
- It leaves #130/#131 empirical closure + `shards/spawn.mirror` + full `@spawn <= @loop` cascade for future ticks, honoring craft-not-deliver.

---

## §10 Signature

Reviewed and signed: **Seam**
Discipline: adversarial review as accountability
Date: 2026-07-02
Composition: verdict-per-candidate + blast-radius + composition + status-drift + write-then-lift + excitation + lift-path + next-loop
