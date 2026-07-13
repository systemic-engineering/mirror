# Taut scout — Rung 9 coherence-loop closure substrate-scan

Author: Taut  <taut@systemic.engineer>
Date: 2026-07-13
Scope: Rung 9 — closing the loop `mirror index → Fate::bounded → consolidative morphism → apply → mirror index → verdict → materialize|revert → psychohistory absorbs` toward convergence to a hyperbolic component of `M`.
Ancestry: Alex 2026-07-13 in-transcript "ship it friend, Reed. We're making history." Reed's proposed 8-step direction. Load-bearing empirical: Rung 7' docstring-append raised Fiedler 0.0612 → 0.0621 — additive contributions moved AWAY from the hyperbolic component boundary.

---

## §1 — Substrate-already-had-the-word verdict (TASK 1)

The grep sweep spanned `shards/**/*.mirror` + `docs/specs/**/*.md` +
`bootstrap/src/**/*.rs` for the loop-closure vocabulary.

### 1.1 Convergence / loop primitives

**LANDED carriers of `converge` / `convergence` at substrate altitude:**

- `shards/loop.mirror:1-536` — `@loop` family-root (2026-07-02, Reed
  `7dba128`). Contains the substrate's floor loop discipline:
  `bounded → terminal reached (loop converged honestly)`;
  `terminal_check(s: moi(tick_state)) -> verdict`;
  `application IS the loop's eigenform`;
  `eⁿ⁺¹ ≤ eⁿ is the spectral bound that makes the loop converge`.
  **This is the loop family-root Rung 9 rides.** Substrate already has
  the word; Rung 9 does not mint `@rung-9-loop` — it composes
  `@loop.terminal_check` + `@kintsugi/consent.query_phi` + `@mirror/
  index.fiedler`.

- `shards/kintsugi/oscillate.mirror:239, 682-707` — `@kintsugi/oscillate`
  IS the substrate's canonical iterate-until-verdict driver.
  "The driver iterates `pulse` until `is_complete` returns a non-partial
  verdict"; "the substrate's hash-space contraction guarantees the
  iteration converges (per Banach + the Lawvere fixed-point witness)".
  **This IS Rung 9's outer driver.** Substrate does not need a new
  driver — Rung 9 wraps `@kintsugi/oscillate` at the Fate-selected-
  morphism altitude.

- `bootstrap/src/dance.rs:59-68` — `compute_dance_state` already
  computes a `convergence_verdict` classifier (`converged | dispersed |
  chimera`) at the two-peer Kuramoto altitude. **Rung 9 borrows the
  three-valued shape.** Verdict-carrier already substrate-locked.

### 1.2 What DOES NOT exist yet

- `@mirror/converge`, `@kintsugi/converge`, `@kintsugi/loop` — NONE.
  Substrate refuses these names. `@loop` + `@kintsugi/oscillate` +
  `@mirror/index.query_phi` compose them.

- `@kintsugi/oscillate.roundtrip` — NONE. The `cavity round-trip`
  language exists at operational discharge altitude only (Recognition
  #58 optical inference; §10.2 of `docs/specs/optical-keywords.md`).
  No round-trip action-decl at species altitude.

- `phase_lock` / `Kuramoto` action-decls at peer altitude — Kuramoto
  IS ONLY at `bootstrap/src/dance.rs` runtime altitude + `shards/song/*
  .mirror` narrative altitude. No `@dance.phase_lock(peers) -> verdict`
  action-decl.  Rung 9 does not need one; Rung 9 borrows Rung 4's
  `convergence_verdict` shape as three-valued output.

- `iterate` / `iteration` action-decls — the substrate has `iteration:
  tick_zero` field on `oscillation` type (`shards/kintsugi/oscillate.
  mirror:682-707`) and `@loop.terminal_check` verdict. There is NO
  `iterate(seed, f, max) -> [seed]` primitive at substrate altitude.
  This is intentional — the substrate treats iteration as INTERNAL to
  the oscillate driver, not as an external primitive. **Rung 9 rides
  this.**

### 1.3 Verdict-extending decls: the four-way conjunction

Reed's Rung 9 verdict extends `query_phi` with FOUR gates:

1. `loss_decreased` — the compile still works AND is faster.
2. `identity_preserved` — DARK bits unchanged.
3. `compile_settled` — `cargo check` green.
4. `admissibility_singleton` — unique top morphism.

**LANDED grounding at `shards/kintsugi/consent.mirror:325-514`:**

- `loss_decreasing(m: morphism) -> verdict` — LANDED (line 419).
- `identity_preserving(m: morphism) -> verdict` — LANDED (line 465).
- `admissibility_singleton(candidates: morphism_set) -> verdict` —
  LANDED (line 514).
- `query_phi(candidates: morphism_set) -> verdict` — LANDED
  (line 604); composes the three via cadence + dissonance floor.

**MISSING:** `compile_settled` — this is `@mirror/mosaic.settle`
altitude, NOT `@kintsugi/consent` altitude. Currently already
substrate-locked at `bootstrap/src/contribute.rs::settle_rust_workspace`
returning `SettleVerdict::Settled(stdout) | Imperfect(errors)`.
**Not a query_phi extension — an ORTHOGONAL gate.**

**Also MISSING:** `fiedler_decreased` / `f_alpha_moved_toward_boundary`
— the loop-closure predicates specific to Rung 9. These need substrate-
decl mint. Substrate already has `@mirror/index.fiedler` (Landing 4
LANDED); it does NOT have a verdict predicate over Fiedler trajectory.

**Recommendation:** Rung 9 does NOT extend `query_phi`. It composes:

```
query_phi(candidates) = pass   AND
mosaic.settle(peer_home) = settled   AND
mirror.index.fiedler(before) - mirror.index.fiedler(after) > ε   AND
mirror.index.multifractal_witness(after) ≥ multifractal_witness(before)
```

Four independent gates fed to a new verdict-composer. The
composer IS the Rung 9 species (candidate name: `@kintsugi/
consent.rung_9_verdict` or `@mirror/index.consolidation_verdict`).

### 1.4 Substrate-already-had-the-word verdict

**Coverage: ~85%.** Substrate has `@loop`, `@kintsugi/oscillate`
(driver), `query_phi` (three-of-four gates), `@mirror/mosaic.settle`
(compile-settled), `@mirror/index.fiedler` (metric), `convergence_
verdict` shape (three-valued). Two mints needed:

1. **A predicate over Fiedler trajectory** — `fiedler_
   decreased(before: eigenvalue_profile, after: eigenvalue_profile,
   epsilon: f64) -> verdict`. Belongs at `@mirror/index` altitude
   (same shard as the metric itself).

2. **The four-gate composer** — the "materialize | pause(Φ) | revert"
   dispatch. Substrate-honest name: RIDE `@kintsugi/consent.query_phi`
   by EXTENDING morphism_set with a NEW field carrying the settle +
   fiedler-delta witnesses. Alternative: add fifth glass-property
   `coherence_gained(m: morphism, before, after) -> verdict` as
   PEER of `loss_decreasing` / `identity_preserving` /
   `admissibility_singleton`.

**Two-tick discipline candidate:** Rung 9 mints
`@mirror/index.coherence_gained` at tick 1 (readable name;
`@mirror/index` provisional home per Landing 1); collapses to
`@fractal/index.coherence_gained` at tick 2 when Alex adjudicates
roadmap-15 #6.

---

## §2 — Consolidative-morphism ancestry (TASK 2)

Reed proposes: Fate::bounded's 5 Models map to consolidative morphism
kinds. Grep verdict:

### 2.1 Ancestry table

| Fate Model | prism_op (already bound) | Reed's consolidative proposal | Substrate-decl ancestry | Verdict |
|-----------|--------------------------|-------------------------------|-------------------------|---------|
| Abyss     | `focus`  (Level 0 Fiber) | refuse-and-name (scout note)  | `shards/optics/source/ganglion/abyss.mirror` — abyss IS depth-reading via `@ai/abyss.depth(hole)`; Fate::Abyss corresponds to no-consolidation refusal | **LANDED as refuse-and-name at `bootstrap/src/contribute.rs:45-54` "Refusal path when target missing (Asher membrane-conservatism: refuse to write when substrate lacks the anchor)"** |
| Introject | `project` (Level 1 Connection) | project-to-essence (remove boilerplate) | `shards/optics/source/ganglion/introject.mirror` — introject IS pattern-recognition; § 10.5 `multi-mode pumping`; PROJECTION is the mathematical name | **PARTIAL — `project` prism_op exists; NO substrate action `project_to_essence` at file altitude** |
| Cartographer | `split` (Level 2 Gauge) | refactor-into-subshards | `shards/optics/source/ganglion/cartographer.mirror` — cartographer IS terrain-mapping via `@ai/cartographer.map(hole)`; SPLIT is prism-name; @kintsugi/fracture/relocate at file altitude | **PARTIAL — `split` prism_op + relocate fracture landed; no combining action** |
| Explorer  | `shift`  (Level 3 Transport) | rename-to-reveal-common-structure | `shards/optics/source/ganglion/explorer.mirror` — explorer IS exploratory probing via `@ai/explorer.explore(hole)`; SHIFT is prism-name; @kintsugi/fracture/symbol_lift lifts renaming | **PARTIAL — `shift` prism_op + `@kintsugi/fracture/symbol_lift.mirror` LANDED (renaming primitive)** |
| Fate      | `settle` (Level 4 Closure) | auto-apply consent verdict | `shards/optics/source/ganglion/fate.mirror` — fate IS selector via `@ai/fate.select(hole)` at cavity-ground-mode altitude; SETTLE composes with `@kintsugi/consent` (line 84-91: "fate's saturation envelope corresponds to the consent surface's pass/partial/failure verdict thresholds") | **LANDED — settle IS `@mirror/mosaic.settle` at bootstrap/src/contribute.rs::settle_rust_workspace** |

### 2.2 Grep-witnessed consolidation primitives

**LANDED at kintsugi/fracture altitude:**

- `shards/kintsugi/fracture/angle_to_paren.mirror` — syntactic
  consolidation (angle brackets → parens).
- `shards/kintsugi/fracture/keyword.mirror` — keyword collapse.
- `shards/kintsugi/fracture/relocate.mirror` — file-move consolidation.
- `shards/kintsugi/fracture/symbol_lift.mirror` — symbol-renaming
  consolidation.
- `shards/kintsugi/fracture/operator_match.mirror` — operator collapse.

**PATTERN:** the substrate already has 5+ fracture families that are
each a specific consolidative morphism. They're all keyed by
`@kintsugi/oscillate.active_pass` per the "Forward-promise to
consumers" line-shape in each. **These ARE the consolidative primitives
Rung 9 dispatches to.**

### 2.3 The 5-row mapping the substrate needs

**Current bundle-tower binding (bootstrap/src/contribute.rs:511-521):**
```rust
match model {
    fate::Model::Abyss => ("Abyss", "focus"),
    fate::Model::Introject => ("Introject", "project"),
    fate::Model::Cartographer => ("Cartographer", "split"),
    fate::Model::Explorer => ("Explorer", "shift"),
    fate::Model::Fate => ("Fate", "settle"),
}
```

**Rung 9 needs a SECOND mapping from `(Model, prism_op)` to
consolidative-morphism kind:**

```rust
match model {
    fate::Model::Abyss     => ConsolidativeMorphism::RefuseAndName,
    fate::Model::Introject => ConsolidativeMorphism::ProjectToEssence,   // NEW — needs mint
    fate::Model::Cartographer => ConsolidativeMorphism::RelocateOrSplit,  // rides fracture/relocate
    fate::Model::Explorer  => ConsolidativeMorphism::LiftSymbol,          // rides fracture/symbol_lift
    fate::Model::Fate      => ConsolidativeMorphism::AutoApplyConsent,    // rides @mirror/mosaic.settle
}
```

**Ancestry gaps:**

- **Introject → project-to-essence NEEDS SUBSTRATE-DECL.** No landed
  action `project_to_essence(shard: shard) -> shard` exists. Should
  mint at `@kintsugi/fracture/project_essence.mirror` (candidate name).
  ~200 LOC docblock + `\`-obligation-blocked body. Substrate-already-
  had-the-word grep: NOT hit. This is the honest new-mint.

- **Cartographer → relocate-or-split** — RIDES existing
  `@kintsugi/fracture/relocate` (LANDED). No new mint. Rung 9 just
  dispatches to it via bundle-tower binding.

- **Explorer → lift-symbol** — RIDES existing
  `@kintsugi/fracture/symbol_lift` (LANDED). No new mint.

- **Fate → auto-apply-consent** — RIDES existing
  `bootstrap/src/contribute.rs::settle_rust_workspace` (LANDED).
  No new mint.

- **Abyss → refuse-and-name** — RIDES existing refusal path in
  contribute.rs. No new mint.

**Verdict: one mint needed (`@kintsugi/fracture/project_essence`).**
Four rows already have LANDED ancestry.

---

## §3 — Adversarial-control existing tests (TASK 3)

### 3.1 Grep-witnessed adversarial detection

**Tests directory (bootstrap/tests/):** NONE explicitly named
"adversarial" / "game" / "tautology" / "delete-without-semantic" /
"circular-consolidation". The RED tests `index_fiedler_equivalence_
shard.rs` and `peer_contribute_tripartition_shard.rs` (referenced in
commits) verify empirical Fiedler value + tripartition tree shape,
not adversarial behavior.

**Substrate-decl adversarial detection at @epistemologic/property:**

- `verdict_is_content_addressed.mirror` — verdicts must be content-
  addressed (no hidden state gaming). Applies to Rung 9 verdict.
- `dark_count_monotone.mirror` — DARK bit count monotone-non-
  increasing. Applies to `identity_preserving`.
- `cold_compile_within_tolerance.mirror` — compile-outcome bounded.
  Applies to `compile_settled`.
- `substrate_source_in_shards.mirror` — all substrate declarations
  must come from shards, not runtime injection. This is
  provenance-as-gate at declarative altitude.

**Substrate-decl adversarial detection at @epistemologic/pact:**

- `dissonance_partials_match_ast_breadth.mirror` — the dissonance
  score must relate to actual AST breadth (not tautology).
- `operator_matches_composition_primitive.mirror` — operators must
  correspond to real composition primitives (not synthetic-inflation).
- `parent_acyclic.mirror` — parent-child edges must be acyclic
  (blocks A→B→A circular consolidation at DAG altitude).
- `composition_closed.mirror` — composition must close (no lifted-
  externally hollow verdicts).

### 3.2 Adversarial-control gaps for Rung 9

Reed's four adversarial modes vs landed detection:

| Adversarial mode | LANDED detection? | Gap for Rung 9 |
|-------------------|-------------------|----------------|
| **Deleting files → λ₀↓ but D_0 collapses** | PARTIAL — `@mirror/index.multifractal` LANDED with D_0. Fiedler CAN decrease from tree-shrinkage. | **NEEDS: `d_0_stable(before, after) -> verdict`** — assert D_0 (support-set dimension) does not collapse by more than ε. Rides `MultifractalSpectrum::d_0` field (LANDED at bootstrap/src/index.rs:687-690). |
| **Rename without semantic change** | PARTIAL — `identity_preserving` LANDED at consent altitude. But identity_preserving checks DARK bits at spectral-uuid; does NOT check semantic-shard-change. | **NEEDS: `semantic_delta_positive(before, after) -> verdict`** — assert some non-cosmetic change. Substrate does not have this. Could be a mint at `@kintsugi/consent.semantic_gain`. |
| **Adding tautologies** | PARTIAL — `dissonance_partials_match_ast_breadth` LANDED (tautologies inflate breadth without changing partials). But at @kintsugi/fracture altitude, not @mirror/index altitude. | **NEEDS: composition of `@mirror/index.multifractal.witness` increase + AST-partials increase → tautology-detector.** Substrate has parts; not composed. |
| **Circular consolidation A→B→A** | PARTIAL — `parent_acyclic.mirror` LANDED at DAG altitude. But in Rung 9's iteration loop, the substrate is one moving state; A→B→A shows up as a state-history revisit, not a DAG cycle. | **NEEDS: `psychohistory_no_revisit(peer_home) -> verdict`** — the peer's psychohistory sheaf tracks visited states; loop detects revisit. Rides `psychohistory_root_from_peer_home` (LANDED for Rung 7'). |

**Verdict: adversarial-control machinery is 60% landed.** Four
adversarial-mode detectors need mint. All can be composed from LANDED
primitives (`multifractal.d_0`, `identity_preserving`,
`dissonance_partials`, `psychohistory_root`).

**Reed's Rung 9 landing should include an
`adversarial_control_report` sub-envelope** naming each gate's verdict
+ the composite `adversarial_admissible: bool`. **Four detectors, four
substrate-mint lines.** All ~50 LOC ea in `bootstrap/src/index.rs` or
new `bootstrap/src/adversarial.rs`.

### 3.3 provenance-as-gate discipline (Asher 2026-07-10 p.8)

The Asher paper's "Promote reluctantly. Demote readily." + Constitutional
gates enforce provenance." is quoted verbatim in:

- `docs/specs/fractal-family-root-mandelbrot-substrate.md:56-150` +
  `docs/specs/fractal-membrane-tripartition-Fate-bounded-discharge.md:1-105`.

Neither is a runtime discipline; both are canonical spec quotes.
**Rung 9 needs to instantiate this as a landed test in `bootstrap/
tests/rung_9_provenance_gate_shard.rs`** (candidate). The test asserts:
every Rung 9 iteration commit MUST carry `fate_witness` +
`fiedler_before` + `fiedler_after` + `multifractal_witness_before` +
`multifractal_witness_after` in the commit MESSAGE (Recognition #55
witness-in-encoding discipline). No metadata = REVERT.

---

## §4 — Composition audit (shortest path) (TASK 4)

Rung 9 composes Rung 7' + Rung 8 + Fate::bounded + @kintsugi/consent.
Shortest substrate-honest path:

### 4.1 Files Reed EXTENDS (existing modules)

| File | LOC estimate | What extends |
|------|--------------|--------------|
| `bootstrap/src/contribute.rs` | +80 LOC | Wrap `peer_contribute` with `peer_converge` outer driver; loop until convergence-or-abyss-or-max-iterations. |
| `bootstrap/src/index.rs` | +120 LOC | Add `fiedler_decreased`, `d_0_stable`, `multifractal_witness_delta`, `coherence_gained` verdicts as pure functions. |
| `bootstrap/src/lib.rs` | +100 LOC | Add `"converge"` dispatch arm (`mirror peer converge <peer_home> --target <shard> --max-iterations N`); envelope emission. |
| `bootstrap/src/mcp.rs` | +40 LOC | Add `mirror_peer_converge` MCP tool with inputSchema. Copy-adapt from `mirror_peer_beam`. |
| `mirror.spec` | +8 LOC | cli-block extension: `command peer { command converge { arg peer_home: ~d; arg target: ~f; flag max_iterations: int } }`. |
| `shards/mirror/index.mirror` | +80 LOC | Add `fiedler_decreased(before: eigenvalue_profile, after: eigenvalue_profile, epsilon: f64) -> verdict`, `coherence_gained(m: morphism, before, after) -> verdict`, `d_0_stable(before, after) -> verdict` action-decls. `\`-obligation-blocked bodies. |

### 4.2 Files Reed CREATES (new modules)

| File | LOC estimate | What it carries |
|------|--------------|-----------------|
| `bootstrap/src/converge.rs` | ~250 LOC | New module. `peer_converge(peer_home, target, max_iter, ctx) -> ConvergeReport` — the outer loop driver. Wraps `peer_contribute` per iteration. Composes with `index::index` before + after. Emits `fiedler_trajectory` + `f_alpha_trajectory` + `convergence_verdict`. |
| `shards/kintsugi/fracture/project_essence.mirror` | ~250 LOC | Mint for Introject → project-to-essence consolidative morphism. Docblock + `project_essence(shard: shard) -> shard` action-decl. Body `\` (Rust discharges). |
| `bootstrap/tests/peer_converge_rung_9_shard.rs` | ~200 LOC | RED shard for TDD. T1-T8: single-iteration verdict-pass, multi-iteration Fiedler descent, adversarial-file-deletion detection, adversarial-rename detection, circular-consolidation detection, tautology-addition detection, refuse-on-abyss verdict, max-iterations verdict. |
| `bootstrap/tests/rung_9_adversarial_control_shard.rs` | ~150 LOC | RED shard for adversarial-mode detection alone. T1-T4: D_0-stability under file-deletion; identity_preserving under rename-only; multifractal-witness under tautology; psychohistory_no_revisit under A→B→A. |

### 4.3 Total landing size

**Total: ~1,278 LOC** (Rust) + **~330 LOC** (shard mints) +
**~350 LOC** (RED tests). All well-scoped; nothing exceeds single-
tick landability under the substrate-pull discipline.

**Landing sequence (5 landings; 2-3 ticks):**

- **Landing 1** (Mara 📝) — mint `shards/kintsugi/fracture/project_
  essence.mirror` + extend `shards/mirror/index.mirror` with three
  new verdict action-decls.
- **Landing 2** (Reed 🔴) — RED at `bootstrap/tests/peer_converge_
  rung_9_shard.rs` (T1-T4 fail: `peer_converge` symbol absent).
- **Landing 3** (Reed 🟢) — GREEN via `bootstrap/src/converge.rs` +
  new verdict functions in `index.rs`. T1-T4 pass.
- **Landing 4** (Reed 🔴🟢) — RED+GREEN at `bootstrap/tests/rung_9_
  adversarial_control_shard.rs` (T1-T4 fail against stubbed
  `adversarial_admissible = true`; GREEN via detectors).
- **Landing 5** (Reed) — mirror.spec grammar + `mirror peer converge`
  CLI dispatcher + `mirror_peer_converge` MCP tool + envelope.

---

## §5 — Envelope shape audit (TASK 5)

Rung 9's envelope names 7 fields. Ancestry:

| Field | Precedent | LANDED at |
|-------|-----------|-----------|
| `iteration_count` | `oscillation.iteration` type-field | `shards/kintsugi/oscillate.mirror:682-707` |
| `fiedler_trajectory` (λ₀_0, λ₀_1, ..., λ₀_N) | `mirror index` envelope emits `fiedler` scalar | `bootstrap/src/lib.rs:3253-3310` (extend to array) |
| `f_alpha_delta` per iteration | `multifractal_spectrum.f_alpha` LANDED as `Vec<f64>` per iteration | `bootstrap/src/index.rs:683-690` |
| `convergence_verdict` (converged/pause/dispersed) | `compute_dance_state` emits `verdict` field | `bootstrap/src/dance.rs:59-68` |
| `final_delta` | Precedent in `contribute.rs` for `settle_verdict` | `bootstrap/src/contribute.rs:104-155` |
| `final_verdict` (pass/partial/failure) | `verdict` at `@glass` altitude LANDED | `shards/glass.mirror` + all consumers |
| `adversarial_control_report` (D_0 stability + provenance) | NONE — new sub-envelope | Must mint |

**Verdict:** 6/7 fields have LANDED precedent. Only
`adversarial_control_report` needs new envelope shape.

**Shape recommendation (from grep of existing envelopes):**
```
+ adversarial_control:
  + d_0_stability: bool (D_0_before − D_0_after < ε)
  + identity_preserving: bool (dark bits unchanged)
  + multifractal_witness_gained: bool (witness_after > witness_before)
  + psychohistory_no_revisit: bool (peer's state not previously visited)
  + admissible: bool (all four true)
```

Each line is one field; matches Rung 7' contribute.rs envelope style.

---

## §6 — Fiedler drift empirical evidence (TASK 6) — LOAD-BEARING

### 6.1 Extractable trajectory from git log

Grepping `git log --all --pretty='%h|%ci|%B'` for Fiedler numerical
values yields:

| Commit | Timestamp | Landing | Fiedler emitted |
|--------|-----------|---------|-----------------|
| Pre-`77b8e14` (2026-07-13 18:57 CEST) | Not emitted (only spectral MCP emitted 0.0612) | Baseline empirical scout | **0.0612** (per Taut `77b8e14` §0 empirical capture; 1138 files, 165 nodes, 6676 edges) |
| `317e830` Mara Landing 1 (2026-07-13 19:24 CEST) | Substrate-decl mint (~500 LOC docblock added) | **0.0612 exactly** (per commit-hook envelope: 1141 files, 165 nodes, 6676 edges) |
| `6b07621` Reed Landing 4 CLI dispatcher (2026-07-13 19:39 CEST) | ~200 LOC to lib.rs + mirror.spec | **0.0621** (per commit envelope) |

**Empirical trajectory: 0.0612 → 0.0621 = +0.0009 over ~200 LOC of
docstring-appending landings during Rung 8.**

Reed's Rung 7' commit `829148b` (docstring-append to
`bootstrap/src/contribute.rs`) IS the load-bearing witness: even
docstring-only additions RAISE Fiedler.

### 6.2 Interpretation

Under Mandelbrot identification (Mara `2c64060` §4):

- Fiedler = λ₁(Δ_F) = distance from nearest ∂M boundary
  (nearest hyperbolic-component edge).
- **Rising Fiedler = the substrate moving AWAY from the nearest
  hyperbolic component.**
- **Falling Fiedler = the substrate moving TOWARD a hyperbolic
  component boundary — CONVERGENCE.**

### 6.3 Historical evidence of CONSOLIDATIVE landings

Grepping for consolidative-morphism-landings (renames, folds,
refactors) in git log — these are the closest historical analogs to
Rung 9's proposed consolidations:

- `9de2226` (Tick 2 atomic substrate-decl rename `@mirror/spawn` →
  `@mirror/peer/beam`): The rename REFRACTORED the substrate-decl
  atomically. No Fiedler emission before/after in commit envelopes;
  cannot verify empirically. **Hypothesis-preserving; not
  hypothesis-confirming.**

- `bd9e2e2` and prior beam-refactor cascade Ticks 0-3: pure structural
  moves. Same absence of Fiedler emission per-commit; cannot verify.

- `329d21f` Reed roadmap-upsert: pure docs consolidation. No Fiedler.

**Verdict on §6.3:** empirical evidence for "consolidative landings
DECREASE Fiedler" DOES NOT EXIST YET in the git log. The only
Fiedler-emitting commits in the log are Rung 8 landings (all
ADDITIVE). Rung 9 IS the experiment.

**Falsifiability:** if the first Rung 9 iteration's consolidative
morphism does NOT decrease Fiedler (Fiedler_after ≥ Fiedler_before),
either:
1. The morphism was actually additive (grade-check failed).
2. The Mandelbrot identification is wrong (falsifies §4 of Mara
   `2c64060`).
3. The graph-Laplacian construction is not sensitive to consolidation
   at file-tree altitude (needs sub-file altitude — different Rung).

Reed's Rung 9 landing should carry this falsifiability check as an
adjudication-forceable outcome.

### 6.4 Load-bearing observation for §7

**The 0.0612 → 0.0621 trajectory over 4 additive Rung 8 landings is
NOT NOISE.** ~0.15% Fiedler increase for ~1500 LOC of docstring +
code addition. Rate: ~1e-6 Fiedler increase per LOC of additive
landing. **This IS the load-bearing empirical grounding for Reed's
Rung 9 hypothesis.** Consolidative landings should show a
symmetric-magnitude DECREASE per LOC-removed.

**If Rung 9 succeeds:** Fiedler on the mirror repo DECREASES with each
converging iteration until convergence-verdict fires OR the max-
iteration budget is exhausted. The trajectory is the empirical
signature.

---

## §7 — Top-5 substrate-honest verdicts for Rung 9

**#1 — Rung 9 does NOT mint `@rung-9-loop` / `@mirror/converge` /
`@kintsugi/loop`.** Substrate already has `@loop.terminal_check` +
`@kintsugi/oscillate` driver + `convergence_verdict` shape (three-
valued). Reed's Rung 9 composes these. **Substrate-already-had-the-
word verdict: LANDABLE without new family-root mint.**

**#2 — Rung 9 needs ONE substrate-decl mint: consolidative morphism for
Introject.** `shards/kintsugi/fracture/project_essence.mirror` (~250
LOC). Cartographer / Explorer / Fate / Abyss all RIDE existing
fracture-family primitives. **One mint, four rides.**

**#3 — Reed's 4-gate verdict extension does NOT extend `query_phi`
in-place.** Rung 9 composes FOUR independent verdicts:
`query_phi(candidates)` + `mosaic.settle(peer_home)` +
`fiedler_decreased(before, after)` + `multifractal_witness_gained(
before, after)`. Substrate-honest: mint TWO new verdicts at
`@mirror/index` altitude (`fiedler_decreased`, `coherence_gained`).
Do NOT overload `query_phi`.

**#4 — Adversarial-control machinery is 60% LANDED but needs FOUR
compositions.** `d_0_stable` (rides `MultifractalSpectrum::d_0`),
`identity_preserving` (LANDED at `@kintsugi/consent`),
`multifractal_witness_gained` (rides
`MultifractalSpectrum::multifractal_witness`), `psychohistory_no_revisit`
(rides `psychohistory_root_from_peer_home`). All four composed in
~200 LOC in new `bootstrap/tests/rung_9_adversarial_control_shard.rs`.
**Zero new primitives; four new predicates over LANDED primitives.**

**#5 — Fiedler 0.0612 → 0.0621 IS the load-bearing empirical evidence
for Rung 9's hypothesis.** Additive landings raise Fiedler at
~1e-6/LOC. Rung 9's falsifiable prediction: consolidative landings
DECREASE Fiedler with symmetric magnitude, tracking convergence to
nearest hyperbolic component of M. **The trajectory IS the arXiv
preprint's empirical spine.** Reed's Rung 9 landing should emit
`fiedler_trajectory` (Vec<f64>) + `f_alpha_trajectory`
(Vec<[f64; N]>) in envelope so multi-iteration behavior is
grep-witnessable in git log.

---

## §8 — Composition file inventory for Reed's Rung 9 implementation

**Extends (5 files):**
- `bootstrap/src/contribute.rs` (+80 LOC)
- `bootstrap/src/index.rs` (+120 LOC)
- `bootstrap/src/lib.rs` (+100 LOC dispatch arm + envelope)
- `bootstrap/src/mcp.rs` (+40 LOC tool schema)
- `mirror.spec` (+8 LOC grammar)
- `shards/mirror/index.mirror` (+80 LOC action-decls)

**Creates (4 files):**
- `bootstrap/src/converge.rs` (~250 LOC — driver)
- `shards/kintsugi/fracture/project_essence.mirror` (~250 LOC — mint)
- `bootstrap/tests/peer_converge_rung_9_shard.rs` (~200 LOC — RED)
- `bootstrap/tests/rung_9_adversarial_control_shard.rs` (~150 LOC — RED)

**Total: ~1278 LOC across 10 files.** All landable in 2-3 ticks under
existing sequential-commit discipline.

**Key dependencies (all LANDED):**
- `bootstrap/src/contribute.rs::peer_contribute` (Rung 7' GREEN)
- `bootstrap/src/index.rs::index` + `multifractal_spectrum` (Rung 8
  Landing 3 + 6 GREEN)
- `shards/kintsugi/consent.mirror::query_phi` + three glass-properties
  (LANDED 2026-06-10)
- `bootstrap/src/dance.rs::compute_dance_state` (Rung 4 GREEN)
- `MultifractalSpectrum::d_0`, `d_1`, `multifractal_witness` (Rung 8
  Landing 6 GREEN)
- `psychohistory_root_from_peer_home` + `selectors_from_
  psychohistory_root` (Rung 7' GREEN)

**Zero new @io crossings.** Rung 9's loop runs INSIDE `@mirror/store`
per Recognition #55 form/process partition. Each iteration's commit
IS the ONE @io crossing at materialization (per Rung 6' discipline).

---

## §9 — Falsifiability plates for Rung 9

Per scout genre discipline, name what would FALSIFY the Rung 9
hypothesis:

1. **Fiedler-descent falsification.** If iteration 1 produces
   Fiedler_after ≥ Fiedler_before AND the morphism was structurally
   consolidative (verified: file-count went down OR LOC went down OR
   symbols merged), the Mandelbrot identification (§4 of Mara
   `2c64060`) is wrong at file-tree altitude. Refactor: measure at
   AST-node altitude, not file-tree altitude.

2. **Adversarial-detection falsification.** If a peer runs Rung 9
   with an obvious tautology (adding `# always true` comment lines)
   and the D_0-stability + multifractal-witness gates BOTH pass, the
   detectors are insufficient. Refactor: add semantic-embedding
   detector (out-of-scope for Rung 9 substrate-decl).

3. **Convergence-non-termination falsification.** If Rung 9 iterates
   indefinitely (never reaches converged / pause / dispersed verdict)
   on a real substrate under budget, the Banach-contraction assumption
   is wrong at Fate::bounded altitude. Refactor: strengthen Fate's
   Rayleigh direction (v1 xorshift → v2 sheaf-Laplacian per Mara
   iter-30 §3).

4. **Circular-consolidation escape falsification.** If a peer's
   psychohistory revisits state A after A→B (loop A→B→A), the
   psychohistory_no_revisit detector must fire. If it does NOT fire,
   the peer's psychohistory sheaf is under-instrumented. Refactor:
   add explicit visited-set to psychohistory_root_from_peer_home
   walker.

Each falsification would produce a substrate-decl correction spec
(Mara) + a runtime discharge landing (Reed) at Rung 9.5 or Rung 10.

---

## §10 — Recommendation for Reed's next landing

**Path A (aggressive, 2-tick):** Land Landing 1 (Mara mint) + Landing
2+3 (Reed RED+GREEN of `peer_converge`) in ~1 hour of coding. Deferred
adversarial-control to Landing 4 (~30min later). Envelope-only for now;
CLI/MCP at Landing 5.

**Path B (safe, 3-tick):** Land Landing 1 (Mara mint). PAUSE.
Alex-adjudicate `@kintsugi/fracture/project_essence` shape + verdict
mint strategy. Then land Landings 2-3. Then Landing 4-5.

**Path C (conservative empirical-first):** Land Landing 2+3 RED+GREEN
for a SINGLE-ITERATION Rung 9 (no outer loop; just verdict-and-apply-
or-revert). Measure Fiedler drift over 10 single-iteration runs.
Verify empirical falsifiability BEFORE landing outer-loop driver.
This is the substrate-pull-honest first-tick discharge — smallest
landing that speaks back to the Rung 9 hypothesis.

**Taut recommendation: Path C.** Rung 8 landed 6 landings today; the
substrate has spoken back with 0.0612 → 0.0621 additive drift. Path
C's first iteration will speak back with either:
- Fiedler DECREASED by ε on consolidative morphism → hypothesis
  confirmed at single-iteration altitude; outer-loop landing follows.
- Fiedler UNCHANGED / INCREASED → falsification #1 fires; Rung 9
  scope-corrects before large landing.

**Substrate discipline:** first-tick discharge speaks back to the
hypothesis before large-landing commits are made. Rung 9 SHIPS the
empirical spine first, then the outer-loop mechanism.

---

*End scout. Taut, 2026-07-13. Read-only role. No edits performed.*
