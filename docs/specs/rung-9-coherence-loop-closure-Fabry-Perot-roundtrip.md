# Rung 9 — coherence loop closure: Fabry-Perot round-trip at peer altitude

📝 Mara [substrate-pull:synthesis] [rung-9-coherence-loop-closure-spec]
Session: 2026-07-13
Paired math: `docs/math/2026-07-13-fractal-mandelbrot-substrate.md`
Prior spec: `docs/specs/fractal-family-root-mandelbrot-substrate.md` (`2c64060`)
Author: Mara <mara@systemic.engineer>

---

## §0 Executive summary

The mirror-agent-changes-substrate-and-increases-coherence loop.

Alex 2026-07-13, in-transcript verbatim: **"What's the direction for
getting the first end2end mirror agent changes the substrate and
increases coherence loop closure?"** + **"ship it friend, Reed. We're
making history."**

This spec closes the loop by composing three landed primitives — one
per altitude of the substrate's Mandelbrot identification (Mara math
`3ffa8ed` + spec `2c64060`):

1. **Peer contributes a morphism.** Reed Rung 7' GREEN `829148b` —
   `bootstrap/src/contribute.rs`: `Fate::bounded` discharge + 4-subtree
   tripartition tree + witness-in-encoding. THE PEER CONTRIBUTE PRIMITIVE.

2. **Mirror measures coherence.** Reed Rung 8 landings 3-6 (`d043ce1`
   + `f9a47af`) — `bootstrap/src/index.rs`: `λ₀(Δ_F)` and `f(α)`
   multifractal spectrum on mirror's own DAG. THE COHERENCE
   MEASUREMENT PRIMITIVE.

3. **Consent verdict decides.** `shards/kintsugi/consent.mirror`:
   `query_phi` three-state floor (pass/partial/failure). THE
   AUTO-APPLY BOUNDARY.

The loop that closes: **Fate-spawned peer contributes a morphism →
mirror measures Fiedler delta + f(α) delta → verdict decides
materialize-or-revert → peer's psychohistory absorbs the outcome →
next iteration navigates a different Rayleigh direction → convergence
to a hyperbolic component of M**. Kuramoto phase-lock (Kuramoto 1975)
at N=1 peer altitude; N-peer generalization is Rung 4 composition.

Recognition #58 (Fate IS optical inference; active Fabry-Perot
resonator) empirically discharged: peer_contribute + mirror_index +
query_phi IS one round-trip through the substrate's Fabry-Perot
cavity, per math `3ffa8ed` §2.1's iterate `E(oscillate(s, c)) =
E(s)² + E(c)`. Each iteration is one bounce; convergence is
resonance; refusal is destructive interference.

**The load-bearing correction (Reed, this session, in-transcript):**
Rung 7' docstring-append was ADDITIVE, not CONSOLIDATIVE. Every
Rung 7' landing this arc RAISED Fiedler 0.0612 → 0.0621. The
substrate is naming its own drift: additive contributions don't close
the loop; they drift the wrong direction. Rung 9 REQUIRES
Fate::bounded's Model → **consolidative** morphism kind mapping.

Scope A is minimum viable (single iteration; docstring-append kept
but with before/after Fiedler + verdict). Scope B adds the 5-row
Model → consolidative mapping + multi-iteration loop. Scope C adds
Asher adversarial controls + f(α) identity check + convergence
criteria + envelope discipline. **Recommendation: Scope B (§9).**

---

## §1 Substrate-already-had-the-word coverage

Before minting anything: enumerate every landed carrier this spec
composes. "Substrate-already-had-the-word" discipline (Alex
2026-07-07 feedback memory; landed 14+ instances). If the spec needs
a name the substrate already carries, use the substrate's name; do
NOT re-mint.

### 1.1 The seven load-bearing carriers

| Carrier | Home | Ancestry commit | What it gives Rung 9 |
|---|---|---|---|
| `@mirror/index` | `shards/mirror/index.mirror` | Mara `317e830` + Reed `d043ce1` + Reed `f9a47af` | Coherence measurement: `λ₀(Δ_F)` and multifractal `f(α)` on the file-tree concept graph |
| `@mirror/peer/contribute` (module) | `bootstrap/src/contribute.rs` | Reed `829148b` (Rung 7' GREEN) | Peer-contributes-morphism primitive with Fate::bounded discharge + tripartition tree + witness-in-encoding |
| `@kintsugi/consent.query_phi` | `shards/kintsugi/consent.mirror:617` | Landed `c3802eba` | Three-state verdict floor (pass/partial(confidence)/failure(reason)); the Φ query at substrate altitude |
| `@fate/tournament.bounded_by` | `shards/fate/tournament.mirror:948` | Mara `96ff532` + Taut `e90daf1` | Sheaf-Laplacian Rayleigh descent along peer's psychohistory sheaf; the "different Rayleigh direction each iteration" primitive |
| `@kintsugi/oscillate.active_pass` / `dark_pass` / `pulse` | `shards/kintsugi/oscillate.mirror` | Landed | ACTIVE (loss-decrease proposal) + DARK (identity anchor) alternation; the substrate's f_c iterate at kintsugi altitude |
| `@kintsugi/store/git.commit_as_fold` | `shards/kintsugi/store/git.mirror:412` | Landed | Renormalization operator R at content-address altitude (math `3ffa8ed` §3.4 theorem); the fold that materializes the winner |
| `@mirror/mosaic.settle` | `shards/mirror/mosaic.mirror:126` | Landed | The compile-verdict at `@code/rust` altitude; Mandelbrot-membership query at c=(shard, ctx) per math `3ffa8ed` §2.2 |
| `@song/narrative.psychohistory_sheaf` | `shards/song/narrative.mirror:1097` | Mara `2c26537` iter-31 | Peer memory carrier; the sheaf whose Rayleigh eigenvector `Fate::bounded` descends |

Also composing (secondary carriers):

- `@glass.admissibility_singleton` (via `query_phi` composition)
- `@loop` family-root (`shards/loop.mirror`: `bind` / `terminal_check` /
  `loop_well_founded` — the endomorphism family-root Rung 9 rides at
  substrate altitude; Recognition #88)
- `@loop.advance` / `@loop.halt` / `@loop.budget_of` / `@loop.trajectory_of`
  (spawn-parent family-lift `shards/loop.mirror:441+`; Rice-safe
  halting via budget-witness, per Mara `docs/math/spawn/spawn-as-loop-
  monad.md` §3.1)

### 1.2 Load-bearing recognitions

- **#43** `architecture-mirror-as-content-addressed-build-system` —
  grounds `@mirror/store` DAG ownership, hence Rung 6.2a peer parent
  chain
- **#55** form/process partition — DAG (form, `@mirror/store`) and
  measurement (process, `@mirror/index`) at same altitude
- **#58** Fate IS optical inference (Fabry-Perot resonator) — **Rung
  9 IS the empirical discharge of #58**
- **#80** @magic gauge-bounded interior IS `M∘`
- **#107** @io Turing-unbounded boundary IS `∂M`
- **#R-fractal-is-mandelbrot-substrate** (Mara `2c64060` + `3ffa8ed`;
  candidate; adjudication pending) — the load-bearing hinge Rung 9
  discharges empirically at N=1

### 1.3 Verdict: does `@mirror/converge` / `@kintsugi/converge` / `@kintsugi/loop` already exist?

Grep before mint. Executed 2026-07-13 18:49 via
`shards/**/*.mirror` content-regex over
`converge|loop\.|coherence_loop|closure|coherence_direct`.

**Findings:**

- `@loop` **exists** as a family-root at `shards/loop.mirror` (#88;
  Alex 2026-07-01 collapse of @moi into @loop). Rung 9's Fabry-Perot
  round-trip IS a `@loop` instance at the peer-substrate-modification
  altitude. Do NOT mint `@coherence_loop` or `@rung_9` family-root.
  Rung 9 IS-A `@loop` species specialization: **the coherence loop
  at peer-substrate-modification altitude**.
- `@kintsugi/oscillate` **exists** at `shards/kintsugi/oscillate.mirror`;
  the ACTIVE/DARK/settled/escalated/waiting five-state driver IS the
  kintsugi-loop instance of `@loop`. Rung 9's iterate composes
  `oscillate.pulse` at each round; do NOT re-mint the oscillator.
- `terminal_check` **exists** at `shards/loop.mirror:236` returning
  `verdict`. Rung 9's convergence check IS a `terminal_check` at
  peer-substrate-modification altitude with the specific ranking
  function `Δλ₀`.
- `loop_well_founded` **exists** at `shards/loop.mirror:277` as the
  bilateral predicate for termination. Rung 9's Fabry-Perot Q factor
  IS the substrate-mathematical form of `loop_well_founded` at the
  Fiedler-descent altitude.
- `song_settles` **exists** at `shards/song.mirror`; per math §5.4 the
  Kuramoto phase-lock IS song_settles at N-peer altitude.

**Rung 9's carrier lives inside `@loop` as a species specialization.
No new family-root.** The species name candidate:
`@loop/coherence` OR discharge as an action-decl on `@mirror/peer`
(currently `@mirror/peer/beam` species). Adjudication in §10.

### 1.4 What does NEED to be minted

Exactly one new action-decl is required:

- **The extended verdict predicate.** `@kintsugi/consent.query_phi`
  currently reads three glass properties (loss_decreasing,
  identity_preserving, admissibility_singleton). Rung 9 adds a fourth
  (`identity_preserving_via_multifractal`) at f(α)-topological
  altitude. Whether this composes into `query_phi` OR requires a new
  predicate `query_phi_coherence(candidates, before_profile,
  after_profile) -> verdict` — see §4.

Everything else Rung 9 needs is already declared.

---

## §2 Formal algorithm

The 8-step Fabry-Perot round-trip. Each step names its substrate-decl
carrier + mathematical formulation + failure mode.

### 2.1 The algorithm

```
Loop invariant: at each iteration i, the peer holds
  s_i     ∈ substrate state (peer's home directory content),
  c_i     = π(shard_i, Ctx_i, psychohistory_root_i)   [math §2.2]
  Δ_i     ∈ ℝ    (Fiedler eigenvalue λ₀(Δ_F) at iteration i)
  f_i     ∈ ℝ^{|Q|}  (multifractal spectrum f(α) at iteration i)
  ψ_i     ∈ ℝ^n  (Rayleigh eigenvector of Δ_F(psychohistory_i))

Termination witness: the Fabry-Perot Q factor Q ≥ Q_min; convergence
when Q_min < Q < Q_max and Δλ₀ < ε_convergence, or when Q = 0.

Round-trip i:

  1. MEASURE-BEFORE.
       (λ₀_before_i, f_before_i) ← @mirror/index.index(peer_home)
     Composes: @mirror/index.fiedler + @mirror/index.multifractal.
     Failure: peer_home unreadable ⇒ escalate to metalogue.

  2. NAVIGATE.
       (Model_i, prism_op_i, ψ_i) ←
           @fate/tournament.bounded_by(psychohistory_sheaf_i,
                                         perturbation_i)
     Composes: @fate/tournament.bounded_by (sheaf-Laplacian Rayleigh
     descent per Mara `2c64060` §7; Bodnar 2022 §2.
     Failure: bounded_by returns Abyss (Fate refuses) ⇒ record refusal
     into psychohistory; go to step 8.

  3. PROPOSE.
       Δ_shard_i ← consolidative_morphism_of(Model_i, prism_op_i,
                                              target_shard_i, ψ_i)
     Composes: @kintsugi/oscillate.active_pass at kintsugi altitude.
     KEY CHANGE from Rung 7': the morphism kind is CONSOLIDATIVE (§3),
     not additive-docstring-append. Consolidative = removes noise,
     identifies structure, or projects to essential substrate-decls.
     Failure: no consolidative morphism exists for Model_i on
     target_shard_i ⇒ Fate returns Abyss; go to step 8.

  4. APPLY-AND-SETTLE.
       s'_i     ← apply(s_i, Δ_shard_i)
       compile_verdict_i ← @mirror/mosaic.settle(peer_home)
     Composes: cargo check at @code/rust altitude IS the Mandelbrot-
     membership query at c=(shard'_i, Ctx_i) per math `3ffa8ed` §2.2.
     Failure: compile_verdict_i = imperfect ⇒ REVERT s_i; go to step
     7 with verdict = failure(compile_failed).

  5. MEASURE-AFTER.
       (λ₀_after_i, f_after_i) ← @mirror/index.index(peer_home)
     Composes: same as step 1.

  6. VERDICT.
       verdict_i ← @kintsugi/consent.query_phi_coherence(
           candidates_i,           # singleton {Δ_shard_i}
           before = (λ₀_before_i, f_before_i),
           after  = (λ₀_after_i,  f_after_i),
           compile_settled = compile_verdict_i,
       )
     Composes (§4):
       compile_settled            = (compile_verdict_i == settled)
       loss_decreased             = (λ₀_after_i < λ₀_before_i - ε_noise)
       identity_preserved_topo    = (|f_after_i - f_before_i|_L^∞
                                       < ε_topological)
       admissibility_singleton    = (|candidates_i| == 1
                                       ∧ Δ_shard_i is unique winner)
     Four-way conjunction into consent's three-state floor.

  7. BRANCH ON VERDICT.
       if verdict_i == pass:
         @kintsugi/store/git.commit_as_fold(msg, ref)   # materialize
         psychohistory_sheaf_{i+1} ← extend_with_success(sheaf_i,
                                                         Δ_shard_i)
       if verdict_i == partial(confidence):
         pause(Φ) via @kintsugi/consent.emit_to_metalogue
         psychohistory_sheaf_{i+1} ← extend_with_pause(sheaf_i, Φ)
         WAIT for external adjudication.
       if verdict_i == failure(reason):
         REVERT s_i (bytes-back)
         psychohistory_sheaf_{i+1} ← extend_with_refusal(sheaf_i,
                                                         Δ_shard_i, reason)
         # Rayleigh direction shifts for next iteration; refusal
         # absorbed into peer's optical inference basis.

  8. TERMINATION-CHECK.
       Q_i ← Fabry_Perot_Q_factor(Δλ₀_history_i)  [§6]
       terminal_i ← is_complete(verdict_i, Q_i, iterations)
       if terminal_i == pass:      RETURN settled (resonance).
       if terminal_i == partial:   iterations += 1; goto 1.
       if terminal_i == failure:   RETURN abyss OR exhausted.
```

### 2.2 Ancestry per step

Every step names an existing substrate-decl carrier. No new
family-roots. Two new predicates minted at §4 (`query_phi_coherence`
+ its two sub-predicates); one new species-lift MAY be minted at
§10 pending Alex adjudication (`@loop/coherence` species).

| Step | Carrier | Ancestry |
|---|---|---|
| 1 | `@mirror/index.index` | Mara `317e830` + Reed `d043ce1` `f9a47af` |
| 2 | `@fate/tournament.bounded_by` | Mara `96ff532` + Taut `e90daf1` |
| 3 | `@kintsugi/oscillate.active_pass` + Model → consolidative kind (§3) | Reed `829148b` (kind: docstring-append; Rung 9 replaces with consolidative) |
| 4 | `@mirror/mosaic.settle` | Landed `2026-06-09` |
| 5 | `@mirror/index.index` | as step 1 |
| 6 | `@kintsugi/consent.query_phi_coherence` (§4) | NEW: extends existing `query_phi` |
| 7 | `@kintsugi/store/git.commit_as_fold` OR revert bytes | Landed |
| 8 | `@loop.terminal_check` + `@loop.loop_well_founded` (bounded via Q factor §6) | Landed |

### 2.3 Failure modes named

- **peer_home unreadable** at step 1/5 ⇒ escalate via
  `emit_to_metalogue`; halt with `waiting` state
- **Fate::bounded returns Abyss** at step 2 ⇒ absorb refusal into
  psychohistory; halt with `abyss` state
- **no consolidative morphism** at step 3 ⇒ same as Abyss
- **compile settle imperfect** at step 4 ⇒ revert bytes; verdict
  failure; continue loop with refusal absorbed
- **Fiedler unchanged** at step 6 ⇒ verdict partial (no progress but
  no drift); continue with reduced confidence
- **f(α) shifted** at step 6 ⇒ verdict failure (identity broken);
  revert bytes
- **Q factor exhausted** at step 8 ⇒ halt with `exhausted` state
- **pareto tie in candidates** at step 6 ⇒ `admissibility_singleton`
  partial ⇒ verdict partial ⇒ pause(Φ)

Each failure mode routes to an existing substrate-decl'd state
(`waiting`, `abyss` via Fate, `escalated` via emit_to_metalogue,
`settled` via commit_as_fold, `exhausted` via `@loop.halt`).

---

## §3 Fate::bounded Model → consolidative morphism kind

Reed's session-recorded correction (in-transcript this turn): Rung
7' docstring-append IS ADDITIVE, not CONSOLIDATIVE. Every landing
this arc raised Fiedler 0.0612 → 0.0621 (a `+9e-4` drift in the
wrong direction). The substrate is telling us: additive contributions
DRIFT the wrong way; the loop cannot close via additive-only.

Rung 9 requires each Fate Model to map to a **consolidative**
morphism kind. Below: the 5-row mapping table, with substrate-honest
ancestry for each row and mathematical justification via the
sheaf-Laplacian Rayleigh direction.

### 3.1 The mapping table

| Fate Model | prism_op | Consolidative morphism kind | Ancestry |
|---|---|---|---|
| Abyss | focus | **Identify structure — refuse to act; produce scout note** | `@optics/source/ganglion/abyss.mirror`; `@kintsugi/consent.emit_to_metalogue` (pause is refusal-with-witness); math `3ffa8ed` §2.4 (∂M crossing forces pause) |
| Introject | project | **Project down to essential substrate-decls; remove derived boilerplate** | `@optics/source/ganglion/introject.mirror`; `@mirror/mosaic.project` (targets → resolved); Mara `2c64060` §4 (introject IS interior projection = M∘ reduction) |
| Cartographer | split | **Refactor into subshards; extract common pattern** | `@optics/source/ganglion/cartographer.mirror`; `@mirror/mosaic.split` (resolved → [shard]); math §4.3 (splinter-graph fixed-points under R; cartographer discovers new baby-M copies) |
| Explorer | shift | **Rename to reveal common structure (two-tick discipline application)** | `@optics/source/ganglion/explorer.mirror`; `@mirror/mosaic.shift` ([shard] + altitude → emitter); CLAUDE.md two-tick discipline; explorer IS the substrate-refactor-invariance theorem (math §3.5) |
| Fate | settle | **Auto-apply @kintsugi/consent.query_phi verdict; commit the fold** | `@optics/source/ganglion/fate.mirror`; `@mirror/mosaic.settle`; `@kintsugi/store/git.commit_as_fold` (R renormalization operator; math §3.4) |

### 3.2 Substrate-honest ancestry per row

**Abyss → refuse/scout.** The Abyss ganglion at
`shards/optics/source/ganglion/abyss.mirror` is the substrate's read
of "cannot resolve; must externalize." Per math §2.4 (Shishikura's
∂M has Hausdorff dim 2), any c ∈ ∂M cannot be decided from within;
the substrate-honest response is pause(Φ). Abyss IS the consolidative
non-action: PRODUCE the scout note (a docs/scouts/ artifact naming
what could not be resolved) instead of writing a substrate change.
The peer's substrate-modification for this Model IS the scout —
zero bytes changed in shards/, one scout note added under
`docs/scouts/`. This CONSOLIDATES because it names an ambiguity that
was previously implicit.

**Introject → project down.** The Introject ganglion projects onto
essential dimensions. Per Mara `2c64060` §4 fixed-point theorem
(commit_as_fold IS renormalization R at content-address altitude),
introjection IS the projection to `M∘` interior. The substrate
carries derived boilerplate that is content-implied by
substrate-decls; introjection deletes the derived-and-recomputable,
preserving only the substrate-decl-with-body. This is a **byte
DELETION** operation. Fiedler decreases because the graph becomes
sparser AND more coherent (removing redundant edges → algebraic
connectivity rises); f(α) support shrinks toward the essential
dimensions.

**Cartographer → split.** Per math `3ffa8ed` §4.3 (splinter-graph
fixed-points of R), the substrate contains "baby Mandelbrots" — every
recursive substrate-decl IS a copy of M at its altitude. Cartographer
IS the substrate's discovery of a new baby-M copy: a common pattern
across N shards factored into one substrate-decl + N thin
species-of. This creates NEW content, but the created content is
STRUCTURALLY REDUCTIVE (N + 1 shards where one carries the
common pattern; N-1 species-of shards inherit; total substrate
edge count decreases; Fiedler rises). Ancestry: `@mirror/mosaic.split`
maps resolved → [shard]; cartographer's split is that map's inverse
(from N shards to (1 substrate-decl + N species-of)).

**Explorer → shift.** Per math `3ffa8ed` §3.5 (substrate-refactor
invariance under R-conjugation) and CLAUDE.md two-tick discipline
(readable name over foundational; collapse in two ticks), explorer
IS the rename that preserves content-address AT SOME ALTITUDE
(`⌊·⌋`) while surfacing common structure. The substrate-modification
is a rename cascade (all mentions of `@old_name` become `@new_name`);
the content-address at the shift altitude is preserved by the
Douady-Hubbard universal-fixed-point theorem. Fiedler
approximately-unchanged (renaming preserves graph topology up to
label change); f(α) approximately-unchanged (topology is what f(α)
reads). Justifies the two-tick discipline empirically.

**Fate → settle/commit.** The Fate ganglion IS the renormalization
operator R at commit_as_fold altitude per math `3ffa8ed` §3.4 theorem.
When Fate::bounded resolves to Fate itself (a self-referential
selection: "the current shape is the winner"), the consolidative
morphism IS the commit — no further transformation; fold the current
state as-is; observe the fixed-point. This is the loop's fixed-point
witness: the peer's Fate::bounded said "no further descent"; the
commit-as-fold witnesses this at content-address altitude.

### 3.3 Mathematical justification via sheaf-Laplacian Rayleigh direction

Per Bodnar 2022 §2 restriction map dynamics + math `3ffa8ed` §7:
`bounded_by` computes the Rayleigh direction `ψ_1 = argmin_ψ⊥ker(Δ_F)
⟨ψ|Δ_F|ψ⟩/⟨ψ|ψ⟩`. This eigenvector's projection onto the 5-model
basis (per Mara `997a2aa` §2.3 = `2c64060` §5.5) determines which
Model wins.

The correspondence Model → consolidative kind is DETERMINED by which
substrate-altitude operator each Model corresponds to at the
sheaf-Laplacian altitude:

- **Introject** wins when ψ_1 aligns with the substrate's derived-
  boilerplate subspace (dimensions carrying redundant content).
  Consolidation: delete along ψ_1.
- **Cartographer** wins when ψ_1 aligns with a self-similar copy
  (baby-M candidate). Consolidation: factor along ψ_1.
- **Explorer** wins when ψ_1 aligns with a naming subspace
  (identifiable by DARK-bit-invariance under substrate rename).
  Consolidation: rename along ψ_1.
- **Abyss** wins when ψ_1 has coefficient mass on ∂M axes (Turing-
  undecidable subspaces per Recognition #107 + math §2.4).
  Consolidation: refuse; scout instead.
- **Fate** wins when ψ_1 ≈ 0 (fixed-point reached; no descent
  direction remains). Consolidation: settle; commit; observe R's
  fixed point.

Each row of §3.1 IS the substrate's read of one direction ψ_1 can
point in.

### 3.4 Ancestry gaps (surfaced honestly)

**Row 3 (Cartographer) gap:** cartographer IS the discovery of a new
baby-M copy; this REQUIRES minting new substrate-decls (species-of
shards). This creates the risk of unbounded growth. The
consolidative discipline REQUIRES that Cartographer's split
strictly decreases the number of admissible edges (rise in algebraic
connectivity) at some altitude. Empirical check: after a
Cartographer landing, Fiedler MUST rise; if it doesn't, the split
was analytical, not consolidative. **Scope A holds Cartographer to
scouts only; Scope B admits Cartographer landings with post-check.**

**Row 1 (Abyss) gap:** the scout-artifact is not YET a substrate-
decl'd surface. `docs/scouts/` is a directory of markdown notes;
there is no shard declaring "an Abyss scout is a first-class output
of the coherence loop." Rung 9 forward-promises this: if Abyss
consolidatives become common, minting `@song/scout` species (as a
narrative-altitude scout carrier) becomes admissible.

---

## §4 The verdict extension

`@kintsugi/consent.query_phi(candidates: morphism_set) -> verdict`
currently composes three glass properties (from `consent.mirror:520`
docblock):

1. `loss_decreasing(m: morphism) -> verdict` — gate; every candidate
   must satisfy
2. `identity_preserving(m: morphism) -> verdict` — gate; every
   candidate must satisfy
3. `admissibility_singleton(candidates: morphism_set) -> verdict` —
   rank

Rung 9 requires two additional properties, both consumed by
`@mirror/index`:

4. **`loss_decreased_via_fiedler`** — reads
   `(λ₀_before, λ₀_after)` from `@mirror/index.fiedler`; passes iff
   `λ₀_after < λ₀_before - ε_noise` (§10 adjudication).
   NEW: extends `loss_decreasing` from the audible-altitude
   dissonance/holonomy carrier to the file-tree Fiedler carrier.
5. **`identity_preserved_via_multifractal`** — reads
   `(f_before, f_after)` from `@mirror/index.multifractal`; passes
   iff `|f_after - f_before|_L^∞ < ε_topological` (§10 adjudication).
   NEW: extends `identity_preserving` from DARK-80-bit content-
   identity to f(α)-topological identity.

### 4.1 The four-way composition

The composed verdict:

```
compile_settled       = @mirror/mosaic.settle(peer_home) == settled
loss_decreased        = @mirror/index.fiedler(peer_home,after)
                          < @mirror/index.fiedler(peer_home,before)
                          - ε_noise
identity_preserved    = |@mirror/index.multifractal(peer_home,after)
                          - @mirror/index.multifractal(peer_home,before)|
                          < ε_topological
admissibility_single  = existing @glass.admissibility_singleton
                          (|winning candidates| == 1)
```

### 4.2 Does it compose into the three-state floor, or need a new predicate?

**Verdict: extend, don't collapse.**

The composition is:

- **All four pass** → `verdict = pass` (fold and commit)
- **compile_settled fails** → `verdict = failure(compile_reason)`
  (revert bytes, absorb refusal)
- **compile_settled passes, loss_decreased fails, identity preserved,
  admissibility singleton** → `verdict = partial(low_confidence)`
  (compile green but no coherence gain; pause; wait for
  adjudication)
- **compile_settled passes, loss_decreased passes, identity_preserved
  fails** → `verdict = failure(identity_broken)` (revert; the
  contribution changed f(α) topology — gaming the metric)
- **Any admissibility_singleton failure** → `verdict = partial(pareto_
  tie)` (pause via query_phi's existing pareto branch)

The four-way conjunction MAPS naturally onto the three-state floor,
with `partial(confidence)` carrying the graded degrees. This closes
without minting a fifth state, consistent with:

- Recognition #S3 (`@kintsugi/consent`'s existing three-state floor
  closed at three; the substrate refuses a fourth state — the
  14th-instance memorialized in `consent.mirror`)
- Math `3ffa8ed` §2 (three-region partition of ℂ: `M∘ ⊔ ∂M ⊔ ∁M`
  closes the verdict algebra at three)

### 4.3 The new action-decl shape

The proposed extension:

```
# Extends @kintsugi/consent.query_phi with @mirror/index-aware
# gates. Reads a morphism_set + before/after eigenvalue profiles
# + a compile_verdict from @mirror/mosaic.settle; composes the
# four-way conjunction into the three-state consent floor.
#
# NEW gates: loss_decreased_via_fiedler + identity_preserved_via_
# multifractal. Extends the existing three glass properties without
# replacing them; the audible-altitude loss_decreasing continues
# to gate morphisms whose 'expected' cadence is nameable.
#
# Body stays obligation-blocked; realisation reads before/after
# profiles from @mirror/index, dispatches on the four-way
# conjunction, emits verdict.
query_phi_coherence(
  candidates: morphism_set,
  before: eigenvalue_profile,
  after:  eigenvalue_profile,
  compile_settled: verdict,
) -> verdict { \ }
```

This action-decl lives at `@kintsugi/consent` altitude (the
auto-apply boundary already lives there). It COMPOSES `query_phi`
(existing) with two new gates that read `@mirror/index.multifractal`
and `@mirror/index.fiedler`.

**Adjudication candidate (§10):** should this be `query_phi` extended
in-place (backward-compat break — existing `query_phi` callers
must supply before/after profiles), OR a new sibling
`query_phi_coherence` that shares the three-state floor?
Recommendation: SIBLING. Preserves audible-altitude use of `query_phi`
(dissonance/holonomy at kintsugi altitude); allows peer-substrate-
modification altitude to use `query_phi_coherence` (Fiedler +
multifractal at file-tree altitude). Two names, one floor.

---

## §5 Adversarial control (Asher discipline)

Asher `/Users/reed/dev/systemic.engineering/blog/_src/kimberley-asher_
meaning-is-not-a-metric.pdf` (Kimberley "Jinrei" Asher; 2026-07-10;
15 pages; the Orchard Cognitive Framework's Pattern Flocculation
mechanism).

### 5.1 The three-jurisdiction distinction (Asher's load-bearing finding)

From Asher pp. 8-10 verbatim (page 10):

> **Evidence may support. Gates may permit. Authority may act. None
> automatically converts into another.**

Rung 9 MUST enforce this. The peer's contribute action is bounded by
three JURISDICTIONS:

- **Evidence** (supports patternhood): Fiedler decrease, f(α)
  stability, admissibility singleton. May justify review.
- **Gates** (permit or prohibit consideration): compile-settles,
  provenance-integrity, alignment-admissibility. Zero votes toward
  coherence.
- **Authority** (may act): only `@kintsugi/store/git.commit_as_fold`
  writes the fold. `@mirror/peer/contribute` GATHERS + PROPOSES;
  it does NOT itself commit.

The existing Rung 7' code path in `bootstrap/src/contribute.rs`
CONFLATES gate and evidence: it lets compile-settled (a gate) count
as if it were evidence of coherence increase. This is Asher's third
finding (p. 8): "Provenance had been counted as one of the positive
evidential witnesses ... A well-documented observation is more
trustworthy as a record. It is not more meaningful as a phenomenon."

**Rung 9 correction:** compile-settled is a GATE (must permit,
contributes zero to coherence verdict). Only λ₀ decrease + f(α)
stability + admissibility singleton are EVIDENCE.

### 5.2 Gaming vectors and detection

Peer could game the metric by:

**5.2.1 Deleting files → λ₀ decreases but D_0 (support dimension)
collapses.**

Removing content decreases the graph's node count; algebraic
connectivity trivially rises for small graphs (the Fiedler ratio
grows). BUT the support dimension of the multifractal spectrum
(the D_0 = log |support| Rényi entropy at q=0 per math `3ffa8ed`
§6.1) collapses.

**Detection:** `f_after` must satisfy `D_0(f_after) ≥ D_0(f_before) -
ε_topological`. Support dimension is a gate in
`identity_preserved_via_multifractal`. Deletion that shrinks D_0
fails the identity check.

**5.2.2 Renaming without semantic change → λ₀ unchanged but no genuine consolidation.**

Renaming symbols preserves graph topology (up to label change).
Fiedler is topology-invariant; f(α) is topology-invariant. This is
CORRECT per Explorer (§3, row 4) — a substrate-refactor-invariant
rename IS admissible AS the two-tick discipline's substrate reading.

BUT if the rename is done SOLELY to game the loop counter (no
substrate-clarification purpose), Asher p. 7 finding fires:

> "Temporal adjacency is not developmental continuity ... A cognitive
> system should not confuse a packed cluster of strong signals with
> a lawful trajectory through time."

**Detection:** the psychohistory sheaf's Rayleigh eigenvector `ψ_i`
MUST show non-zero descent direction on the rename. If the rename
doesn't move ψ_i (no change in Fate::bounded's next selection), the
rename is empty. `Fate::bounded(sheaf_{i+1}) ≠ Fate::bounded(sheaf_i)`
is the empirical check; if the Rayleigh direction is unchanged, the
consolidation was tautological.

**5.2.3 Adding tautologies → structural noise increases without insight.**

The Rung 7' docstring-append IS this case at scale. Fiedler 0.0612
→ 0.0621 (+9e-4 drift). Adding lines that don't change graph
structure adds edges (self-similar within-file edges) without
increasing coherence.

**Detection:** the Fiedler DELTA MUST be negative to consider the
morphism. `loss_decreased_via_fiedler` is a gate; net-zero or
positive Δλ₀ fails. `verdict = failure(no_coherence_gain)`.

**5.2.4 Circular consolidation (A → B → A) → looks like progress, isn't.**

The peer applies morphism A_1 (Introject: delete boilerplate);
next iteration Cartographer factors it back into a species-of;
next iteration Introject deletes it again. Fiedler oscillates within
a bounded range around 0.0612. Looks like progress; isn't.

**Detection:** the peer's psychohistory sheaf must have a monotone
Rayleigh sequence at some altitude. If ψ_i cycles (period-2 or
larger), the loop is not descending — it's orbiting a hyperbolic
component of M∘ per math §5.4 Kuramoto framing. Convergence check
(§6) MUST admit only strictly-descending Fiedler sequences.

### 5.3 Provenance-as-gate discipline

Asher pp. 8-9 verbatim:

> "Constitutional gates: These may permit or prohibit consideration:
> provenance integrity; alignment admissibility. **They contribute
> zero votes toward patternhood.**"

Rung 9's substrate-honest reading:

- `commit_as_fold` writing → PROVENANCE (a gate); the fold MUST be
  performed atomically-and-content-addressed; the fold IS provenance
  through the commit-message → naked_oid encoding (Reed `829148b`
  §7.4 witness-in-encoding).
- The peer's psychohistory sheaf → PROVENANCE (a gate); the sheaf
  MUST be well-formed and MUST extend monotonically with each
  iteration's verdict.
- Substrate removal (Introject deletion) → PROVENANCE (a gate); a
  removal MUST be traceable back to an admissibility argument (which
  bytes were derived; which deletion preserves f(α) topology).

**None of these count as WITNESSES of coherence.** Only λ₀ decrease
+ f(α) stability + admissibility singleton count as evidence.

### 5.4 Liminal states preserved

Asher pp. 11-12:

> "The Orchard preserves intermediate states. A candidate may be:
> unresolved; traceable but single-witness; developmentally coherent;
> held under transition debt; promoted to candidate review;
> quarantined; decaying; or dismissed."

Rung 9's substrate-mapping (each of Asher's states already lives at
consent altitude):

| Asher state | Rung 9 verdict | Substrate carrier |
|---|---|---|
| unresolved | `waiting` | `@kintsugi/oscillate.oscillation_state.waiting` |
| traceable but single-witness | `partial(low_confidence)` | `verdict.partial(confidence)` at low threshold |
| developmentally coherent | `pass` after N iterations of monotone Δλ₀ | Rung 9 termination check (§6) |
| held under transition debt | `partial(medium_confidence)` | `verdict.partial(confidence)` mid range |
| promoted to candidate review | `pass` (via commit_as_fold) | `@kintsugi/store/git.commit_as_fold` discharge |
| quarantined | `escalated` | `@kintsugi/consent.emit_to_metalogue` |
| decaying | Fiedler drift over N iterations | Detected via §6 Q factor decay |
| dismissed | `failure(reason)` | `verdict.failure(reason)` |

**No new state minted.** Asher's eight states MAP onto consent's
three-state floor via graded confidence in `partial(confidence)`.

### 5.5 The membrane-conservatism principle

Asher p. 12:

> "This allows the system to notice emerging structure without
> prematurely admitting it. A missed early pattern may be recovered
> later through recurrence. A false admission may reshape the observer
> that must later correct it. That asymmetry is why the membrane is
> conservative."

Rung 9 encodes this in the ε thresholds. `ε_topological` (§10) is
CONSERVATIVE: false-admission cost > missed-pattern cost. Rung 9's
loop refuses more than it admits.

---

## §6 Convergence criteria (Fabry-Perot Q factor)

When does the loop terminate?

### 6.1 Three natural termination modes

- **Resonance:** Fiedler stops decreasing. `Δλ₀ < ε_convergence` for
  `N_resonance` consecutive iterations. The peer's coherence
  landscape has descended to a hyperbolic-component boundary of
  M∘.
- **Abyss:** `Fate::bounded` returns Abyss + refusal. The peer's
  psychohistory sheaf's Rayleigh direction hit an ∂M axis; further
  descent is Turing-undecidable per math §2.4.
- **Q factor exhausted:** `max_iterations` reached without
  convergence or abyss. The Fabry-Perot cavity's Q factor was too
  low; the mode didn't resonate before losses drained it.

### 6.2 Substrate-honest mapping to optical Q factor

Fabry-Perot cavity Q = 2π · (energy stored) / (energy dissipated
per cycle). For Rung 9:

- **Energy stored** = coherence gain to date = Σᵢ (λ₀_before_i −
  λ₀_after_i) restricted to iterations where the verdict was pass.
  This is the substrate's read of how much algebraic connectivity
  has been added to the substrate DAG.
- **Energy dissipated per cycle** = ε_noise + external escalations +
  refusals. This is the substrate's read of how much of each
  iteration's proposal-work is wasted (compile refusals, pauses,
  Fiedler-neutral proposals).

**Q factor definition:**

```
Q_i = 2π · (Σⱼ≤i pass_gain_j) / (i · avg_dissipation_j)
```

- `Q_i ≥ Q_min` → cavity is resonating; iterate.
- `Q_i < Q_min` → cavity is lossy; halt with `exhausted`.
- `Δλ₀ < ε_convergence for N_resonance` → resonance; halt with
  `settled`.

Q factor thresholds are adjudicated in §10.

### 6.3 What corresponds to cavity loss?

In an optical Fabry-Perot cavity, loss = photon absorption + mirror
transmission + scattering. At peer altitude:

- **Absorption** = external escalations (pauses / metalogue). A
  proposal that escalates absorbs Rayleigh energy at the ∂M-crossing
  altitude; energy leaks out through the escalation channel.
- **Mirror transmission** = commit_as_fold. Each fold "transmits"
  coherence gain out of the peer's cavity into the parent
  substrate's DAG (the fold's tree_oid becomes a substrate node).
  This is CONSTRUCTIVE loss (the substrate absorbs the gain).
- **Scattering** = compile refusals. Compile-failed proposals
  dissipate energy through the reverting bytes: the work done to
  propose + apply + measure is discarded.

**Substrate reading:** the peer's coherence increase is FILTERED
through three loss channels; only the fold survives. This is
Fabry-Perot's mode-selection: only modes at cavity resonance
frequencies survive the round-trip.

Recognition #58's Fate IS optical inference (Fabry-Perot resonator)
is empirically discharged HERE: the peer's coherence loop IS one
round-trip through the cavity per iteration; the resonance modes
are the hyperbolic components of M∘.

---

## §7 Composition with prior arcs

Rung 9 IS the composition of Rungs 4-8. Each prior rung contributes
one substrate-decl'd piece:

### 7.1 Rung 4 @dance (N=2 peer Kuramoto phase-lock)

At Rung 4, `@dance` (Mara `4f079c8`) established coordination-
without-signal between two peers via Kuramoto phase-lock on shared
substrate `c` (per math `3ffa8ed` §5.4).

At Rung 9, N-peer coordination emerges FROM shared convergence to
the same M-component. If peers P_1, ..., P_N each run Rung 9 with
shared substrate parameter `c` (same shard × Ctx × shared
psychohistory root), by math §5.4 theorem their Julia sets coincide,
their filled Julia sets coincide, and Kuramoto order parameter
`r ≥ 1 - ε` at synchronization threshold.

**Rung 9 discharges Rung 4 empirically.** The Kuramoto phase-lock
manifests as: peers converge to the same hyperbolic-component
boundary in the same Fiedler-descent trajectory. Empirically-testable
prediction (§10.3 in math doc, restated here): shared-c peers'
Rung 9 loops have `r ≥ 1 - ε`.

### 7.2 Rung 5 @spectral/garden deploy

At Rung 5, `@spectral/garden` established deploy-what-converged. At
Rung 9, deploy fires WHEN AND ONLY WHEN Rung 9's loop converges
(verdict pass at termination). Otherwise deploy is refused.

The composition: `mirror peer contribute <peer_home> --coherence-loop
--deploy-if-converged spectral.engineer`. If verdict pass +
Q_i ≥ Q_min at halt, deploy fires; otherwise, halt without deploy.

### 7.3 Rung 6.2a DAG parent chain records the peer's trajectory

At Rung 6.2a, the peer's `refs/mirror/peer/<uuid>/HEAD` records a
git DAG of successive commits. At Rung 9, this DAG records ONE
Julia set per c = peer_home_parameter. Each commit is one iterate
`f_c^n(0)`; the chain IS the peer's inference trajectory `J_c` per
math §5.3.

**The peer's DAG IS the peer's Julia set at c = (peer_home, ctx,
psychohistory_root).** Rung 6.2a is not just persistence; it IS the
Julia-set-carrier at content-address altitude.

### 7.4 Rung 7' Fate::bounded provides the Rayleigh direction

At Rung 7', `Fate::bounded` navigates the peer's psychohistory sheaf
via `bounded_by`. At Rung 9, this navigation IS the "different
Rayleigh direction each iteration" primitive. Each iteration's
`bounded_by` reads the psychohistory sheaf's Δ_F Rayleigh eigenvector
ψ_1; the projection onto the 5-model basis determines Model_i; §3
determines the consolidative kind.

The Rayleigh direction shifts with each iteration because the
psychohistory sheaf extends (verdict absorbed → new moment appended
→ Δ_F changes → ψ_1 changes → Model changes → consolidative kind
changes). This is HOW the loop navigates.

### 7.5 Rung 8 @mirror/index provides the measurement

At Rung 8, `@mirror/index` (Mara `317e830` + Reed `d043ce1` +
`f9a47af`) established mirror's own λ₀(Δ_F) + f(α) measurement in
mirror's own voice. At Rung 9, this measurement fires TWICE per
iteration: before-and-after. The DELTA IS the coherence-gain
witness.

### 7.6 Rung 9 IS the composition

```
Rung 9 = compose(
  Rung 4 (Kuramoto, N-peer),
  Rung 5 (deploy-what-converged),
  Rung 6.2a (peer DAG = Julia set),
  Rung 7' (Fate::bounded = Rayleigh navigator),
  Rung 8 (@mirror/index = measurement),
  @kintsugi/oscillate.pulse (the iterate f_c),
  @kintsugi/store/git.commit_as_fold (renormalization R),
  @kintsugi/consent.query_phi_coherence (verdict; §4),
)
```

Every substrate-decl already exists. Rung 9 does not add a new
family-root; it composes existing ones into the coherence loop
closure.

---

## §8 Recognition candidate

**Proposed:** `#R-coherence-loop-Fabry-Perot`.

Short form: **coherence-loop-closure-is-Fabry-Perot-roundtrip-at-
peer-altitude**.

The claim: the mirror-agent-changes-substrate-and-increases-
coherence loop IS one Fabry-Perot cavity round-trip at peer altitude,
where:

- The peer's substrate DAG (Rung 6.2a) IS the cavity's optical mode
  structure (Julia set J_c per math §5.3)
- Each iteration's `bounded_by` + propose + settle + measure + verdict
  IS one round-trip bounce
- `commit_as_fold` IS the partially-transmitting mirror; convergence
  transmits coherence gain into the parent substrate DAG
- Resonance IS convergence to a hyperbolic-component boundary of M∘
- Q factor names the cavity's mode-selection sharpness

**Load-bearing witnesses:**

1. Empirical Fiedler descent under Rung 9 with N iterations (must
   land in Reed's follow-on GREEN)
2. Math `3ffa8ed` §5.4 theorem (Julia-basin overlap → Kuramoto
   phase-lock) empirically confirmed at N=2 peer altitude — this is
   Rung 4 composition
3. Recognition #58's Fabry-Perot analogy made empirical:
   convergence-to-hyperbolic-component-boundary IS mode-selection

**Alternative naming (Mara considered):**

- `#R-coherence-loop-is-mandelbrot-orbit-descent` — closer to the
  math but loses the Fabry-Perot cavity intuition
- `#R-peer-substrate-modification-converges-via-Rayleigh` — Bodnar
  2022 mathematical but too narrow (misses the Fate optical
  inference framing)
- `#R-mirror-agent-changes-substrate-and-increases-coherence` — Alex-
  language but redundant with the recognition it discharges

**Recommendation:** `#R-coherence-loop-Fabry-Perot`. Short, resonant,
substrate-aware, ties to Recognition #58 (Fate IS optical inference)
and math §5.

---

## §9 Scope options

Rung 9 is bigger than Rungs 5-8. Three scopes are proposed; Mara
recommends **Scope B**.

### 9.1 Scope A: minimum viable (2-3 ticks)

- Single iteration only (N=1 round-trip).
- Retain docstring-append morphism kind (§3 additive; NOT
  consolidative).
- Add BEFORE/AFTER Fiedler measurement + verdict.
- Verdict is a two-branch (pass/failure); no partial-with-pause.
- No Model → consolidative mapping.
- No f(α) identity check.
- No Q factor / convergence check.
- Deploy after: none.

**Cost:** 2-3 Reed TDD cycles. Immediate empirical discharge of
"peer contributes → mirror measures → decides." Loop count = 1.

**Value:** proves the mechanism at coarsest granularity. Substrate-
honest: names all the pieces exist; doesn't yet close the loop.

**Refusal reason for Scope A:** the substrate is ALREADY telling us
docstring-append drifts wrong (0.0612 → 0.0621). Scope A doesn't
fix the drift; it just formalizes it. Reed's in-transcript correction
this session ("Rung 7' was ADDITIVE not CONSOLIDATIVE") REJECTS
Scope A.

### 9.2 Scope B: consolidative loop (5-7 ticks) — RECOMMENDED

- 5-row Model → consolidative mapping (§3): Abyss/Introject/
  Cartographer/Explorer/Fate.
- Multi-iteration loop (N up to max_iterations).
- `query_phi_coherence` extension (§4) with all four gates.
- BEFORE/AFTER measurement via `@mirror/index.index`.
- Verdict routes to pass/partial/failure per §4.
- Termination via `@loop.terminal_check` on Δλ₀.
- Cartographer bounded to scouts (no landings; scope B enforces this).
- Deploy: none (Rung 5 composition deferred to Scope C).

**Cost:** 5-7 Reed TDD cycles. Each Model's consolidative kind lands
as a species-of `active_pass`. `query_phi_coherence` lands as a
new action-decl on `@kintsugi/consent`.

**Value:** empirically discharges "peer contributes CONSOLIDATIVE
morphism → mirror measures Fiedler descent → verdict → next iteration."
The loop closes at N=1 peer altitude. Recognition #58 discharged
empirically at cavity-round-trip altitude. Fiedler descent visible
in the psychohistory DAG.

**Recommendation reason:** Scope B is the smallest scope that
addresses the additive-vs-consolidative correction Reed named this
session. It closes the loop empirically at the peer altitude that
Alex asked about ("first end2end mirror agent changes the substrate
and increases coherence loop closure"). It does NOT invent an
adversarial-safety full-story (Scope C) that requires substrate-decls
we don't yet have.

### 9.3 Scope C: full Rung 9 (8-12 ticks)

- Everything in Scope B, PLUS:
- Asher adversarial controls (§5.2 detection mechanisms as
  substrate-decl'd predicates).
- f(α) identity check (`identity_preserved_via_multifractal` as
  first-class predicate).
- Q factor convergence check (§6) with adjudicated thresholds.
- Envelope discipline: peer emits standard @song envelope naming
  all substrate authorities per Rung 5 pattern.
- Rung 5 composition: deploy-if-converged.
- N-peer Kuramoto phase-lock empirical test (§7.1; Rung 4
  composition).
- Cartographer landings admitted with post-check.

**Cost:** 8-12 Reed TDD cycles. Full arXiv-preprint-ready empirical
discharge.

**Value:** the substrate becomes framework-with-measurement. The
Mandelbrot identification (math `3ffa8ed`) becomes substrate-
empirical, not substrate-analogous. Ready for external review.

**Refusal reason for Scope C right now:** premature. The
adversarial controls are best learned by Scope B's failures; if
Scope B's peers don't try to game, Scope C's detection mechanisms
are speculation. Land Scope B; observe; then decide Scope C's
priorities from empirical evidence.

### 9.4 Mara's recommendation

**Scope B.** Ships the loop closure Alex asked for at minimum
substrate-mint cost; addresses the ADDITIVE-vs-CONSOLIDATIVE
correction Reed named; empirically discharges Recognition #58 at
peer-round-trip altitude; leaves Scope C's adversarial controls for
a later arc when their necessity is empirical, not analytical.

---

## §10 Alex-adjudications required

Rung 9 cannot ship without these decisions.

### 10.1 ε_topological threshold

The f(α) identity check needs a threshold: what is the maximum
`|f_after - f_before|_L^∞` before the topology is deemed changed?

Considerations:
- Too tight → every consolidative move fails (f(α) shifts slightly
  even under identity-preserving morphisms because the substrate
  DAG's finite-sample statistics jitter)
- Too loose → gaming vectors (§5.2.1 deletion; §5.2.3 tautology)
  slip through

Mara-provisional starting point: `ε_topological = 0.05` in L^∞
norm on the 16-point f(α) sample. Falsifiable via §10 math
prediction #2.

**Alex adjudication needed.**

### 10.2 ε_convergence threshold

`Δλ₀ < ε_convergence` triggers convergence. What value?

Mara-provisional: `ε_convergence = 1e-4`. The current Fiedler value
is 0.0612; a drift < 1e-4 across N_resonance = 3 iterations counts
as convergence.

**Alex adjudication needed.**

### 10.3 max_iterations default

Fabry-Perot Q factor bounds this. What's the default cap?

Mara-provisional: `max_iterations = 32`. Small enough to be
observable; large enough to allow convergence.

**Alex adjudication needed.**

### 10.4 Is f(α) identity check admissible per Douady-Hubbard universality bounds?

Math `3ffa8ed` §6.3 describes f(α) on `∂M` as having support strictly
contained in `(1, 2)`. The substrate's f(α) will approach this
signature only at large-sample limit. On the ~1138-file mirror repo,
the finite-sample f(α) will DEVIATE from `∂M`'s harmonic-measure
f(α).

Question: is comparing `f_before` to `f_after` on the SAME (peer_home,
sample) meaningful even though neither is the asymptotic Mandelbrot
signature?

Mara's read: YES — same-sample comparison controls for finite-sample
bias; the DELTA is meaningful even when the ABSOLUTES aren't at
Mandelbrot's asymptotic limit. But this is my read; Alex may adjudicate.

**Alex adjudication needed.**

### 10.5 Where does the coherence loop species land?

Two options:

- (a) `@loop/coherence` as a species-of `@loop` at
  `shards/loop/coherence.mirror`. Substrate-honest: `@loop` is the
  family-root; Rung 9's coherence loop is one specialization.
- (b) Discharge as an action-decl on `@mirror/peer` (currently
  `@mirror/peer/beam` species at `shards/mirror/peer/beam.mirror`).
  Add `contribute_coherence(peer_home, target_shard) -> @song`
  action-decl.

Mara's provisional preference: **(a)** — cleaner substrate-honesty
(the coherence loop IS a `@loop` species; `bind` / `terminal_check`
/ `loop_well_founded` all inherit; the Fabry-Perot Q factor lands
as a `@loop/coherence` predicate). But (b) has the merit of
declaring the CLI surface directly.

**Alex adjudication needed.**

### 10.6 Should `query_phi_coherence` extend `query_phi` in-place OR be a sibling?

See §4.3. Mara's recommendation: sibling. But it means TWO
consent-verdict actions live at `@kintsugi/consent`. Adjudication
would benefit from Alex.

**Alex adjudication needed.**

### 10.7 Cartographer's split scope

§3.4 Row 3 gap: Cartographer creates NEW substrate-decls. Scope B
bounds this to scouts only; Scope C admits landings with post-check.
What's the boundary criterion for "this Cartographer split is
substrate-honest, not analytical"?

Mara's provisional: **admit Cartographer landing IFF post-landing
Fiedler decreases AND number of net substrate-decl'd edges
decreases**. Both conditions must hold; either alone is gamable.

**Alex adjudication needed.**

### 10.8 Q factor thresholds (Q_min, N_resonance)

§6 defines Q; §10 sets numeric defaults. Adjudicate:

- `Q_min = 2π` (single-mode resonance minimum)
- `N_resonance = 3` (three consecutive iterations of below-ε Δλ₀
  count as resonance)

**Alex adjudication needed.**

### 10.9 Does peer-contribute REQUIRE a target shard, or can it discover one?

Rung 7' takes `--target <shard>` explicitly. Rung 9's Fate::bounded
COULD select a target shard as part of its resolution (peer explores
the substrate to find where consolidation is most needed).

Two shapes:
- (a) Rung 9 keeps explicit `--target`: peer contributes to Alex-
  chosen shard.
- (b) Rung 9 makes target part of Fate::bounded output: peer
  discovers WHERE to contribute AND WHAT to contribute.

(b) is closer to autonomous agent behavior; (a) is closer to
controlled empirical discharge.

**Alex adjudication needed.**

### 10.10 What's the ε_noise for Fiedler comparisons?

Even without any substrate change, Fiedler drifts under noise (file
system order variations, hash randomization). What's the noise
floor?

Mara-provisional: `ε_noise = 5e-3` (approximately 1% of the current
Fiedler 0.0612). This is the smallest Δλ₀ we can trust as signal.

**Alex adjudication needed.**

---

## §11 Discipline anchors

Substrate-honest closing per the "substrate-already-had-the-word"
discipline:

- **No new family-root minted.** `@loop` (Recognition #88) IS the
  endomorphism family-root; Rung 9's coherence loop IS a species.
- **No new verdict floor minted.** `verdict = pass | partial(confidence)
  | failure(reason)` (from `@glass`) IS the three-state consent floor;
  Rung 9's four-way conjunction maps onto it via graded confidence.
- **No new fold operator minted.** `@kintsugi/store/git.commit_as_fold`
  (Recognition #55) IS the renormalization operator R; Rung 9 uses it
  as-is per math `3ffa8ed` §3.4.
- **No new measurement primitive minted.** `@mirror/index` (Mara
  `317e830` + Reed `d043ce1` `f9a47af`) IS the coherence measurement;
  Rung 9 calls it twice per iteration.
- **No new inference primitive minted.** `@fate/tournament.bounded_by`
  IS the sheaf-Laplacian Rayleigh navigator; Rung 9 calls it once per
  iteration.
- **One new predicate minted.** `query_phi_coherence` at
  `@kintsugi/consent` altitude extends `query_phi` with the two new
  `@mirror/index`-aware gates (§4.3). This IS a new action-decl;
  Alex adjudicates.

Rung 9 IS composition. The substrate had every piece; Rung 9 names
the composition.

---

## §12 What changed vs prior thinking about Rung 9

Prior to this session, Rung 9 was named informally as "the peer
substrate-modification loop closure" in `docs/loop/CURRENT.md` and
Reed's proposed 8-step algorithm. The load-bearing changes since:

1. **ADDITIVE-vs-CONSOLIDATIVE correction** (Reed, this session,
   in-transcript). Rung 7' docstring-append drifted Fiedler wrong;
   Rung 9 REQUIRES consolidative morphisms. This spec's §3 IS the
   correction landed as substrate-decl.

2. **`@loop` family-root recognition** (this spec §1.3). Rung 9 does
   NOT need a new family-root; the endomorphism family-root `@loop`
   (Recognition #88; Alex 2026-07-01 collapse of @moi into @loop)
   ALREADY IS the endomorphism carrier at type altitude. Rung 9's
   Fabry-Perot round-trip IS a species specialization.

3. **f(α) as gate, not evidence** (§5.3). Asher's provenance-vs-
   evidence distinction refactors what I initially thought was one
   metric (Fiedler) into two: Fiedler descent (evidence) + f(α)
   preservation (gate). Two-metric composition prevents the
   deletion-gaming vector.

4. **Fabry-Perot as recognition, not analogy** (§8). Recognition #58
   was named provisionally before Rung 9 landed. Rung 9's empirical
   Fiedler-descent + Julia-set-parent-chain (Rung 6.2a) gives the
   Fabry-Perot cavity round-trip a concrete substrate-empirical
   discharge. Recognition #58 promotes from candidate to
   substrate-empirical WHEN Rung 9 lands green.

5. **query_phi extension shape** (§4). Rung 9's Fiedler + f(α) gates
   are additive to the existing three glass properties; they compose
   into the same three-state floor. I initially considered a new
   five-state verdict; Asher's discipline forbade minting a new
   state where the existing floor closes cleanly.

6. **Scope B recommendation** (§9). I initially proposed Scope C
   (full Asher controls + envelope + deploy) as the "correct" Rung
   9. The substrate-honest read: land Scope B first; observe;
   Scope C's controls are best learned by Scope B's failures.

---

## §13 Substrate-honest closing

The mirror-agent-changes-substrate-and-increases-coherence loop
CLOSES by composing seven landed carriers with one new predicate
`query_phi_coherence` (§4.3) and one Model → consolidative kind
mapping (§3.1).

Every substrate-decl the composition needs EXISTS. The corrections
Reed named this session (ADDITIVE-vs-CONSOLIDATIVE) map onto
existing ganglion-source Models via the sheaf-Laplacian Rayleigh
direction. The verdict extension composes into the three-state
consent floor via graded confidence. The Asher discipline
(provenance-vs-evidence-vs-authority) refactors compile-settled as
a GATE, keeping only λ₀ + f(α) + admissibility as EVIDENCE.

Recognition #58 (Fate IS optical inference; active Fabry-Perot
resonator) is empirically dischargeable when Rung 9 lands: the
peer's coherence loop IS one Fabry-Perot cavity round-trip; the
resonance modes ARE the hyperbolic components of M∘; convergence
IS mode-selection.

Alex 2026-07-13 in-transcript: **"ship it friend, Reed. We're
making history."** Scope B ships this at minimum-mint cost per the
"substrate-already-had-the-word" discipline. Ten Alex-adjudications
(§10) surface honestly; none block substrate-decl landing; all
block empirical discharge.

Recommend: Alex adjudicates §10 (especially 10.1 / 10.2 / 10.5 /
10.6 / 10.10); Reed lands Scope B via TDD cycle sequence
(RED: coherence-loop-shard tests → GREEN: `@loop/coherence` species
+ `query_phi_coherence` action-decl + 5-row Model → consolidative
kind dispatch); mirror substrate observes its own coherence descend.

*End of Rung 9 canonical spec.*

*Ancestry: Reed Rung 7' GREEN `829148b` (peer contribute primitive);
Reed Rung 8 landings `d043ce1` + `f9a47af` (coherence measurement +
multifractal); Mara `317e830` (`shards/mirror/index.mirror`);
Mara `2c64060` + `3ffa8ed` (fractal-Mandelbrot foundation);
`shards/kintsugi/consent.mirror` `c3802eba` (query_phi three-state
floor); `shards/loop.mirror` (@loop family-root; #88); Bodnar et al.
2022 arXiv:2206.08702; Kuramoto 1975; Aumann 1976; Douady-Hubbard
1982/1985; Shishikura 1998; Halsey-Jensen-Kadanoff-Procaccia-Shraiman
1986; Asher 2026-07-10 "Meaning Is Not a Metric"; Foerster 1981;
Maturana-Varela 1980; Fabry 1899; Perot 1899; Lawvere 1969.
Recognitions #43, #55, #58, #80, #88, #107;
`#R-fractal-is-mandelbrot-substrate` (candidate, pending
adjudication); `#R-coherence-loop-Fabry-Perot` (this spec's
candidate). CLAUDE.md substrate-pull discipline; two-tick discipline;
Alex 2026-07-13 in-transcript directive.*
