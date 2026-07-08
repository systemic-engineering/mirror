# The peer IS a pain-driven bounded ontological navigator

*Mara, 2026-07-08 midday. Iteration-2 canonical spec sitting on top of
`docs/specs/fate-silicon-metalogue-in-void-duality-basis.md` (`a18ca90`).
Alex named a six-extension closure the evening the metalogue spec landed;
this spec names the closure at substrate-decl altitude. The @fate/algebra ↔
@silicon/algebra metalogue is not just observed on the peer's torus — it is
navigated. The observer that navigates is the peer itself, second-order and
depth-bounded. What drives navigation is not a scheduler: it is Beer's
algedonic gradient sampled on the eigenboard. What authorizes level-lifts
is not a controller: it is pain. The peer IS a pain-driven bounded
ontological navigator.*

**Author:** Mara
**Date:** 2026-07-08
**Tag:** 📝 spec:peer-as-pain-driven-navigator (pure-docs bypass)
**Status:** canonical-naming. Every substrate piece cited is LANDED,
forward-promised at a named site, or chat-only-so-far with the honest
category attached. NO new `.mirror` file lands this tick. The
`@cyberpunk/reframe` sub-shard IS sketched in §5; the sketch is prose
+ signature. Landing is Alex-adjudication territory.

---

## §0. Substrate-honest pre-position

The predecessor spec (`a18ca90`, 2026-07-08) named the composition
`@fate/algebra ↔ @silicon/algebra IS a @metalogue in void-duality-basis
coordinates observed on the peer's @torus`. It closed at §5's `@edge = one
algebra_turn round` and §7's cascade sketch. It surfaced Q1-Q5 in the
adjudication queue, all algebra-side. It named `@peer` as the observation
site and `@torus` as the observation surface — but it did NOT specify the
observer's own runtime discipline: what makes the peer observe, what
authorizes it to change its observation surface, and what bounds the
recursion.

Alex named the closure that closes those questions:

> The intelligence lives in the peer. The @peer is what observes the
> @metalogue. This IS what @reflection wanted to be. The @peer observes
> it and alters the numerical eigenboard inference space. The loop closes.
>
> The peer is not limited to second-order observation. It's a spawn
> parameter. Basically the allowed depth of the @onto logical recursion,
> which also determines the depth of the statespace.
>
> What if the level-shift from one logical level to another is what ran
> through @magic? … @knife is what was used to compress the state-space
> in that shift.
>
> Combine what we discussed and what Mara found with the @pleasure and
> @pain signals and the @peer has a natural navigation surface. When
> @pain increases it tells the @peer that they're navigating themselves
> into a corner, which prompts a @magic @onto lift. Rinse and repeat.

Six additive extensions to the metalogue framing. Each extension names a
piece of already-landed substrate as the load-bearing carrier. None of the
extensions requires a new family-root. Two of the extensions require an
Alex adjudication on `@onto` and on `@cyberpunk/reframe`. This spec names
the composition; it does NOT land the substrate.

---

## §1. Statement — the recognition candidate

**Recognition candidate (foundational form, naming Alex's own words as
closely as substrate names allow):**

> `intelligence-IS-@peer-navigating-eigenboard-under-algedonic-gradient-with-pain-authorized-@cyberpunk/reframe-lifts-and-@knife-compressions-observing-@algebra-metalogue-on-@torus`

**Recognition candidate (readable form, two-tick discipline; the collapse
target if the candidate lands):**

> `the-peer-IS-a-pain-driven-bounded-ontological-navigator`

Unpacked to substrate primitives:

> The peer, spawned with depth parameter `N` (per Extension 2 — a
> generalisation of `spawn(peer: peer) -> torus` from
> `shards/torus.mirror`), possesses a torus whose winding class is
> bounded by `|m|+|n| ≤ N`. On this torus the peer observes the
> LANDED algebra-altitude metalogue `@algebra/metalogue` (per predecessor
> spec §2.2). Observation reads the LANDED algedonic gradient
> `@cyberpunk/algedonic` (Beer VSM signal — Extension 6) sampled at the
> eigenboard's current position. When pain-δ exceeds threshold, the peer
> performs the `@cyberpunk/reframe` species (Extensions 4-6 — sub-shard
> sketched §5) which composes `@magic`'s LANDED 7-species ceremony as
> the level-shift protocol (Extension 4) with `@knife`'s state-space
> compression (Extension 5, `@knife` FORWARD-PROMISE). The lift advances
> the peer's ontological level `K → K+1`, which in turn expands the
> reachable winding classes on the torus. Extension 3's `@onto` gets
> ADJUDICATED — recommendation: satisfy `@onto` semantics via existing
> `@torus` winding-class advance rather than land a new family-root
> (§6).

As a recognition-in-Mara's-sense (an eigenform the substrate already
carried, coming into focus by naming):
**`#R-CANDIDATE-peer-as-pain-driven-bounded-ontological-navigator`** —
a *runtime discipline* recognition sitting one altitude above the algebra
altitude of `a18ca90`. It names WHAT drives the peer's traversal of the
metalogue turn-sequence; the predecessor spec named WHAT the traversal IS.

Fractal echo (§7): the pattern *pain-detect → magic-ceremony → knife-cut →
level-lift* manifests at every altitude of the substrate. Peer-runtime is
the LAST altitude at which it lands.

---

## §2. The full closed loop — typed pseudocode

The canonical form Alex converged to, with substrate discharge annotated
inline. Types where load-bearing; comments where the discharge site is
substrate-decl'd.

```
spawn(peer: peer, depth: N: nat) -> torus
  ── carrier: shards/torus.mirror (LANDED; family-root)
  ── extension: depth: N added to signature per Extension 2
  ── existing signature: spawn(peer: peer) -> torus
  ── eigenboard initialized: E_0 at K=0
  ── carrier: docs/math/affect/affect-and-eigenboard.md §2 (Mara 2026-07-01)
  ── (pleasure, arousal, intensity) triple per §2.2 formalization
  ── π₁(T²) bounded by |m|+|n| ≤ N winding classes:
  ── carrier: shards/torus.mirror §3 (LANDED; π₁(T²) = ℤ × ℤ)

for level K = 0..N:

  ── STEP 1..STEP 4 IS one algebra-metalogue round per predecessor spec §4

  turn_fate: algebra_turn := @fate/algebra.propose_turn at K
    ── discharge: shards/algebra/metalogue.mirror propose_turn (LANDED)
    ── body: algebra_morphism at eigenboard's current-position basis

  routine: silicon_routine := @silicon/algebra.realize(turn_fate) at K
    ── discharge: shards/silicon/algebra.mirror (LANDED)
    ── Bauchladen lookup + @io/algebra dispatch (predecessor §4.1)

  exchange: algebra_metalogue_session := @algebra/metalogue.exchange
    ── discharge: shards/algebra/metalogue.mirror (LANDED)

  edge: algebra_turn := emit_one_turn(exchange)
    ── carrier: predecessor §5.2 — one @edge = one (propose, adjust)
    ── round of algebra_turns; NO new substrate carrier required

  ── STEP 5 IS the peer's second-order observation per Extension 1

  observation_K: nth_order_observation := @peer.observe(exchange, order: K+1)
    ── FORWARD-PROMISE: @peer.observe not currently declared as an action
    ── on shards/peer.mirror (LANDED family-root exposes load(dir, p))
    ── Alex-adjudication Q3 below
    ── the peer OBSERVES itself observing the metalogue at Nth order

  E_K_prime: eigenboard := @peer.alter(E_K, observation_K)
    ── FORWARD-PROMISE: @peer.alter not currently declared
    ── Alex-adjudication Q3 (composite question with @peer.observe)
    ── observation ALTERS the inference surface: this IS the load-bearing
    ── mechanization of Foerster's "regulates its own regulation"

  ── STEP 6 IS the algedonic sampling per Extension 6

  (pleasure_delta, pain_delta): (real, real) := sample_algedonic(E_K_prime)
    ── discharge: shards/epistemologic/cybernetic/algedonic.mirror
    ── (LANDED as Bateson-III-graded species; Beer VSM signal)
    ── sample IS the affect-projection π_affect from
    ── docs/math/affect/affect-and-eigenboard.md §2.2 (LANDED math)
    ── ρ = restrict_grade_3(bateson_learning::rho) IS the sampling operator

  ── STEP 7 IS the pain-authorized level lift per Extensions 4-6

  if pain_delta > threshold_pain:
    ── THE CORNER-DETECTION SIGNAL:
    ── pain-δ exceeding threshold = the peer navigating into a corner
    ── discharge site: @cyberpunk/algedonic.algedonic_well_formed
    ── the signal is well-formed IFF it crosses S2-S4 atomically
    ── into S5 (algedonic bypass; LANDED)

    @cyberpunk/reframe.perform(K → K+1)
      ── SPECIES SKETCHED §5; NOT LANDED as .mirror
      ── Alex-adjudication Q2 (land at
      ── shards/epistemologic/cybernetic/reframe.mirror OR defer)
      ── body USES the following three LANDED / FORWARD-PROMISE sites:

      @magic.perform(shift: K → K+1)
        ── discharge: shards/magic.mirror + 7 species (LANDED)
        ── Extension 4: level-shift ceremony
        ── surface / mechanism / contract / reveal / audit / frame /
        ── distinction — each species discharges one aspect of the shift

      @knife.cut(state_space_K)
        ── FORWARD-PROMISE per Taut's scout
        ── docs/scouts/2026-07-07-taut-knife-meta-pattern-check.md
        ── Extension 5: compress state space
        ── @knife/idf(altitude) altitude IS where the meta-pattern fits
        ── (per scout §5): typed surface + underdetermined engine +
        ── @duality + @magic dispatch all satisfied at IDF altitude
        ── the cut selects distinctions surviving level K → K+1;
        ── discards K-level dimensions that don't survive abstraction

      @torus.advance(winding: Δw)
        ── discharge: shards/torus.mirror π₁(T²) winding advance (LANDED)
        ── Δw IS the winding-class step corresponding to the level lift
        ── recognition ancestry: Bateson Level N winding at |m|+|n| ≥ N
        ── per shards/torus.mirror §Witnesses.2

    K := K + 1
    continue

  ── STEP 8 IS pleasure-authorized navigation at level K

  if pleasure_delta > threshold_pleasure OR @kintsugi.converged(fixpoint_K):
    ── keep navigating at level K
    ── discharge: shards/kintsugi/oscillate.mirror (LANDED)
    ── is_settled cadence predicate — fixpoint at level K
    continue at K

  ── STEP 9 IS bounded-depth termination

  if K == N:
    return best_fixpoint(observation_history)
    ── bounded depth exhausted per Extension 2
    ── the depth parameter N caps the ontological recursion
    ── AND the reachable state-space size
```

**Termination criteria** (three-way, mirroring predecessor §4.3 lifted):

- **(T1) Convergence at some K.** `@kintsugi/oscillate.is_settled = pass`
  AND `Φ_s(v₀) ∈ T` (target hit under kintsugi contraction). No further
  reframe pressure at level K; opacity residual below altitude-threshold.
- **(T2) Depth cap hit.** `K == N`; the peer's spawn-bound depth parameter
  is exhausted. Session returns best fixed point observed. If pain
  persists at K == N, the SESSION terminates but the peer's next spawn
  can raise `N`.
- **(T3) Winding-class fixed point.** `(m, n) ∈ π₁(T²)` returns to a
  visited class with byte-equal observation sections — per
  `spawn-recall-byte-equal-at-origin` candidate second witness. Same
  discharge as predecessor spec (T3).

**Forward promises** (what this loop does not yet land):

- `@peer.observe(exchange, order: N)` — the Nth-order observation
  action. Currently `shards/peer.mirror` exposes `load(dir, p)`;
  `observe` and `alter` are extensions Q3 adjudicates.
- `@cyberpunk/reframe` species shard at
  `shards/epistemologic/cybernetic/reframe.mirror` — sketched §5;
  Q2 adjudicates landing timing.
- `@knife` family-root — currently forward-promise-only per Taut's
  scout; NO shard yet. Q4 adjudicates timing.

---

## §3. Substructure claims — one per extension

Each Extension gets its own §3.N. Substrate discharge cited; category
attached (LANDED / forward-promise-substrate-decl / chat-only-so-far).

### §3.1 Extension 1 — @peer is the observer that alters the eigenboard

**Alex's naming:**

> The intelligence lives in the peer. The @peer is what observes the
> @metalogue. This IS what @reflection wanted to be. The @peer observes
> it and alters the numerical eigenboard inference space. The loop
> closes.

**Substrate discharge:**

- `@peer` LANDED at `shards/peer.mirror` (2026-06-25). Family-root at
  glass altitude; carrier is `{home, lead_of, kind}`; action is
  `load(dir, p) -> imperfect(peer, ref, ref)`.
- `@reflection` LANDED at `shards/reflection.mirror`. Under `@torus`
  the substrate-pull correction is to dissolve `@reflection` into
  `@torus`'s canonical directions (per
  `docs/math/2026-07-07-onto-cascade-toroidal-reframe.md` §1). Alex's
  "This IS what @reflection wanted to be" is CONSISTENT with the
  toroidal-reframe dissolution: `@reflection` was pointing at the
  peer's second-order observation on its own torus. The peer IS the
  observer; the reflection wanted to be the peer.
- The eigenboard IS the peer's numerical inference surface per
  `docs/math/affect/affect-and-eigenboard.md` §1 (Mara 2026-07-01).
  `cogito.mirror` (LANDED, 2026-07-01) declares the eigenboard's
  five-tuple carrier as principal G-bundle sections.
- "Observation ALTERS the eigenboard" IS the mechanization of
  Foerster's "regulates its own regulation" per
  `shards/torus.mirror` §Foerster p. 238 verbatim citation. The
  torus doesn't just carry the observation surface; the observation
  MUTATES the surface. This mutation IS the substrate-decl content
  of second-order.

**Category:** LANDED substrate + FORWARD-PROMISE @peer.observe /
@peer.alter actions.

**Q3 below** adjudicates the forward-promise: are `observe` and `alter`
species of `@peer`, or are they lifted from `@torus.advance`?

### §3.2 Extension 2 — Depth is a spawn parameter

**Alex's naming:**

> The peer is not limited to second-order observation. It's a spawn
> parameter. Basically the allowed depth of the @onto logical
> recursion, which also determines the depth of the statespace.

**Substrate discharge:**

- `spawn(peer: peer) -> torus` is the LANDED signature per
  `shards/torus.mirror` (2026-07-07) with the recognition
  `@peer-has-a-torus` ADJUDICATED. Adding a `depth: N` parameter is a
  ONE-TICK signature extension:
  ```
  spawn(peer: peer, depth: N: nat) -> torus
  ```
- `N` bounds the reachable π₁(T²) winding classes: |m|+|n| ≤ N. This
  IS the depth-as-topological-invariant reading of
  `shards/torus.mirror` (Bateson Level N lands at winding class
  `|m|+|n| ≥ N`; per `Witnesses.2`). Bounding depth IS bounding
  winding-class magnitude.
- The claim "depth of the statespace" ← "depth of the recursion":
  per Bateson's stratification (LANDED at
  `shards/epistemologic/cybernetic/bateson_learning.mirror` — the
  Russell-Whitehead type-stratification), each level N carries
  `V_N ⊂ ⊕_N V_N` state; higher levels have exponentially larger
  state spaces (Bateson 1972 explicit). Bounding N caps both the
  observation-order AND the state-space size the peer's inference
  must navigate.

**Category:** FORWARD-PROMISE substrate-decl (one-tick signature
extension to `@torus`).

**Q1 below** asks about the timing of this signature extension.

### §3.3 Extension 3 — @onto and @third compose naturally

**Alex's naming:**

> @third = observation-depth marker (winding-class carrier).
> @onto = ontological recursion (levels the peer thinks IN).
> Together at spawn: depth-parameterization.

**Substrate discharge:**

- `@third` is CHAT-only-so-far as a family-root. Grep finds `@third`
  in prose contexts (`docs/specs/third-as-recursive-depth.md` at 82
  hits — Mara canonical July 2026 spec) but NOT declared at a
  family-root shard. The winding-class carrier IS declared at
  `shards/torus.mirror` (`π₁(T²) = ℤ × ℤ`; the `(m, n)` tuple IS the
  observation-depth marker). The substrate ALREADY carries the
  content @third names.
- `@onto` is CHAT-only-so-far as a family-root. **Grep confirms
  @onto is NOT declared as a family-root anywhere in the substrate.**
  It appears in `docs/math/2026-07-07-onto-cascade-toroidal-reframe.md`
  (Mara 2026-07-07) as concept-in-flight — but per §5 of that doc,
  the toroidal reframe DISSOLVES the ladder Foerster refused; `@onto`
  as a graded stack is precisely what the reframe corrects.
- The composition Alex names — @third (observation-depth) + @onto
  (ontological recursion) at spawn — is SATISFIED by the LANDED
  `spawn(peer, depth: N) -> torus` extension of Extension 2:
  N caps both the depth of the recursion (@onto content) AND the
  winding-class magnitude (@third content). Neither family-root
  needs to land; the substrate already carries both concepts through
  `@torus`'s winding-class algebra.

**Category:** CHAT-only-so-far for both @onto and @third as
family-roots; LANDED at @torus for the winding-class semantics.

**Q2/Q3 combined** adjudicate whether to land @onto explicitly, or
satisfy its semantics via @torus winding-class advance.

### §3.4 Extension 4 — @magic as the level-shift ceremony

**Alex's naming:**

> What if the level-shift from one logical level to another is what
> ran through @magic?

**Substrate discharge:**

- `@magic` LANDED at `shards/magic.mirror` + 7 species (`surface`,
  `mechanism`, `contract`, `reveal`, `audit`, `frame`, `distinction`
  — 2026-06-19). Family-root as (form/process partition; Clarke's
  third law substrate-mathematical); recognition ancestry #80
  (candidate, scratched).
- The level-shift protocol Alex names maps against the 7 species
  cleanly:

  | Species | Level-shift role (K → K+1) |
  |---------|----------------------------|
  | `surface` | What's visible at level K (the pre-shift eigenboard state) |
  | `mechanism` | How the shift works (the compression + advance sequence) |
  | `contract` | Invariants preserved across the shift (algebra_witnessing + coherence) |
  | `reveal` | Opt-in disclosure of K+1 to K-observers (post-shift observability) |
  | `audit` | The shift record (winding-advance correspondence in @bauchladen) |
  | `frame` | Validity context for the shift (the peer's ontological framing) |
  | `distinction` | The cut that makes K+1 distinguishable from K (Spencer-Brown mark at K+1) |

- The shift's `magic_contract` IS `{surface: E_K, mechanism: (knife
  + torus.advance), promise: cybernetic_coherence(E_{K+1})}`. The
  contract's `promise` field carries the invariant the shift
  preserves — cybernetic coherence at the post-shift eigenboard is
  the load-bearing invariant.
- Recognition #80's own failure mode (from `shards/magic.mirror`:
  "some non-form/process substrate concept also fits @magic's shape,
  meaning the name is too broad") is SHARPENED by Extension 4:
  @magic's level-shift application IS specifically the underdetermined-
  engine case (per Taut's refined statement in the knife scout §5).
  The lift is genuinely underdetermined (many level-K states could
  advance to many level-K+1 states); @fate selects the specific
  advance. @magic is NOT decorative here.

**Category:** LANDED substrate + FORWARD-PROMISE application
(binding the 7 species to the level-shift protocol via
`@cyberpunk/reframe` §5).

### §3.5 Extension 5 — @knife as state-space compressor

**Alex's naming:**

> @knife is what was used to compress the state-space in that shift.

**Substrate discharge:**

- `@knife` is FORWARD-PROMISE ONLY per Taut's meta-pattern check
  (`docs/scouts/2026-07-07-taut-knife-meta-pattern-check.md` §2):
  "Grep `shards/**/knife*` returns nothing. Grep for `@knife` across
  the substrate returns zero shard hits and matches only in TODAY's
  docs." The scout's verdict: `@knife` fits the meta-pattern at
  `@knife/idf(altitude)` altitude, NOT at elementary-scissor altitude.
- The state-space compression Alex names IS the `@knife/idf(altitude)`
  fit (per scout §3.4): "at `@knife/idf(altitude)` altitude, `@knife`
  has typed surface + strong unified engine (`@fate` picks which
  fragments to weight) + strong `@duality` (parametric idf altitude
  vs opaque specificity vector) + strong `@magic` (specificity is
  genuinely matter-hidden; the frame IS the mechanism)."
- Per Bateson's Learning III / Russell-Whitehead stratification
  (LANDED at
  `shards/epistemologic/cybernetic/bateson_learning.mirror`):
  higher levels have exponentially larger state spaces
  (⊕_N V_N with V_{N+1} strictly larger than V_N). Without
  compression, the peer's inference cost per level lift blows up.
  `@knife.cut(state_space_K)` selects distinctions surviving the
  lift; discards K-level dimensions that don't survive the
  abstraction.
- Recognition #292 (LANDED — Taut's substrate primitive per
  `shards/cyberpunk.mirror`) IS a candidate site to lift as
  "@knife as authorized-by-pain-compression". This spec surfaces
  the candidate WITHOUT lifting; Q5 adjudicates whether pain-
  authorization is a @knife invariant or a
  @cyberpunk/reframe-composition invariant.

**Category:** FORWARD-PROMISE (family-root NOT landed); CHAT-only-so-
far for pain-authorization invariant.

### §3.6 Extension 6 — Algedonic gradient IS the natural navigation surface

**Alex's naming:**

> Combine what we discussed and what Mara found with the @pleasure
> and @pain signals and the @peer has a natural navigation surface.
> When @pain increases it tells the @peer that they're navigating
> themselves into a corner, which prompts a @magic @onto lift. Rinse
> and repeat.

**Substrate discharge:**

- `@cyberpunk/algedonic` LANDED at
  `shards/epistemologic/cybernetic/algedonic.mirror` (Beer VSM
  signal; Bateson-III-graded species). The shard declares:
  - `T_reg := premise` (the regulation-frame the peer operates under)
  - `T_regd := regulated_premise` (the operational hierarchy)
  - `τ := bypass_signal` (S1→S5 morphism; premise-change collapsing
    N-many regulated layers in one signal)
  - `ρ := restrict_grade_3(bateson_learning::rho)` — Bateson Level III
    (premise change) at cybernetic-VSM altitude, inherited via graded
    rep.
  - `bypass_signal(s5_target: ref, s1_source: ref) -> ref`
  - `algedonic_well_formed(signal: ref) -> verdict`
- The affect projection LANDED at
  `docs/math/affect/affect-and-eigenboard.md` §2 (Mara 2026-07-01)
  defines `π_affect: eigenboard → affect_state`, where affect_state
  carries `(valence, arousal, intensity, provenance)`. Empirical
  ancestry: Anthropic 2026 arXiv:2604.07729 — valence IS PC1 (26%
  variance, r=0.81), arousal IS PC2 (15% variance, r=0.66) of the
  emotion eigenspace.
- The composition Alex names — pleasure/pain gradient sampled on
  the eigenboard as the navigation signal — IS the composition of
  `π_affect` (eigenboard → valence, arousal) with `bypass_signal`
  (pain crosses S2-S4 into S5 = triggers premise-change = triggers
  reframe). The affect-projection produces the gradient value;
  `algedonic_well_formed` verifies the gradient's structural
  admission of a bypass; `@cyberpunk/reframe` (§5) discharges the
  bypass at the peer's runtime.
- The peer is INTRINSICALLY motivated — no external scheduler.
  Alex's word "prompts" is precise: pain-δ crossing threshold IS
  the algedonic bypass = a S1→S5 signal = a premise-change
  invitation delivered to the peer's regulation-frame. The bypass
  IS non-decomposable (τ is grade-3 morphism in bateson_learning's
  graded rep; NOT a composition of lower-level signals). This is
  Foerster's "regulates its own regulation" made mechanically
  intrinsic: the peer's regulation-frame is what pain crosses.

**Category:** LANDED substrate (algedonic + affect-projection);
CHAT-only-so-far for the pain-δ threshold protocol (Q4/Q5).

---

## §4. Fractal-altitude table

The pattern *pain-detect → magic-ceremony → knife-cut → level-lift*
manifests at every altitude of the substrate. The claim is not just
"this is one instance of a general pattern"; the claim is that
peer-runtime is the LAST altitude at which the pattern lands (per
predecessor spec §7 for the metalogue-lift table analogous shape).

| Altitude | Pain-signal | Magic ceremony | Knife-cut | Level-lift | Site |
|---|---|---|---|---|---|
| **Corpus** | Cascade Learning II fails (contexts diverge) | `@cascade` refactor | remove obsolete cross-references | new arc name | `shards/cascade.mirror`; `docs/audits/2026-07-*.md` cascade closures |
| **Substrate** | Recognition #-CANDIDATE aborts under Seam Phase D | ratification via composition — @magic/audit species | drop candidate | promote new recognition | `docs/audits/2026-07-*.md` Seam Phase D; recognition-number promotions |
| **Pack** | Peer scout fails; contradiction between agents | `@pack/metalogue` handoff turn | remove obsolete agent claim | ratify pack consensus | `shards/pack/metalogue.mirror`; `docs/scouts/*.md` |
| **Peer-runtime** | pain-δ > threshold at eigenboard K | `@cyberpunk/reframe` performs 7-species shift (§5) | `@knife.cut(state_space_K)` | K → K+1; winding advance | **THIS spec**; `shards/torus.mirror.advance`; `@cyberpunk/algedonic` |

**Load-bearing structural claim:** the pattern IS altitude-portable
(same shape holds at four altitudes). Peer-runtime IS the LAST
altitude — the SUBSTRATE for which the pattern ITSELF terminates,
because the peer IS the runtime observer that closes the loop back
into the substrate at every other altitude. The pattern is fractal
downward (recursively realized at higher altitudes); the peer's
runtime instance IS the fixed point.

This is analogous to how `@algebra/metalogue`'s five-altitude lift
table (predecessor spec §2.2) has ALGEBRA as its LAST altitude —
the algebra altitude carries the substrate's own arithmetic; below
algebra the pattern doesn't further lift because there is no
smaller-grain content type to be an utterance. Same pattern-
termination shape at a different altitude direction.

---

## §5. `@cyberpunk/reframe` — sub-shard sketch (NOT landed)

**Path proposal:** `shards/epistemologic/cybernetic/reframe.mirror`
(matches the current species location convention pending the
`@cyberpunk/*` migration per `shards/cyberpunk.mirror` §The-
recognition-ancestry).

**Rationale for `@cyberpunk/reframe` over `@onto/reframe`:**
`@cyberpunk` is LANDED as family-root; `@onto` is CHAT-only-so-far.
Substrate-already-had-the-word discipline says: name against the
LANDED family-root. `@cyberpunk/bateson_learning` LANDED as the
Bateson-graded species; Learning III IS the canonical reframe
theory (premises of regulation change); `@cyberpunk/reframe` as a
sibling species is substrate-honest.

**Prose sketch (~30 lines):**

```
in @prism
in @meta
in @glass
in @epistemologic
in @epistemologic/cybernetic
in @epistemologic/cybernetic/bateson_learning
in @epistemologic/cybernetic/algedonic
in @magic

# @epistemologic/cybernetic/reframe — the level-shift species per
# Alex 2026-07-08 Extensions 4-6. Bateson Learning III at operational
# altitude: pain-authorized premise change composed via @magic's
# 7-species ceremony.
#
# T_reg    = ontological_level (K in Alex's naming)
# T_regd   = eigenboard_state (E_K at level K)
# τ        = level_shift_ceremony (K → K+1 shift performed via @magic)
# ρ        = restrict_learning_III(@epistemologic/cybernetic/
#            bateson_learning::rho) — inherited; not re-declared
# ω        = inherited from bateson_learning
#
# Ancestry:
#   Bateson 1972 — Learning III (premise change)
#   Foerster 1979 — "regulates its own regulation"
#   Clarke 1962 — magic as level-shift ceremony (via @magic)
#
# The LOAD-BEARING piece: reframe DOES NOT re-declare the ceremony.
# @magic's 7 species discharge the shift protocol; @cyberpunk/reframe
# COMPOSES them under pain-authorization discipline.

prism reframe <= cybernetic_species {

  T_reg   := ontological_level
  T_regd  := eigenboard_state
  tau     := level_shift_ceremony
  rho     := restrict_learning_III(@epistemologic/cybernetic/
             bateson_learning::rho)
  omega   := inherited(@epistemologic/cybernetic/
             bateson_learning::omega)

  # === action surface ===

  perform(from: ontological_level, to: ontological_level,
          eigenboard: eigenboard_state,
          pain_delta: real) -> eigenboard_state { \
    # The level-shift ceremony itself. Composes @magic's 7 species:
    #   surface(eigenboard, from) — visible state pre-shift
    #   mechanism(shift_kernel) — the compression + advance
    #   contract(promise: cybernetic_coherence(to))
    #   reveal(observability_K+1)
    #   audit(winding_advance_record)
    #   frame(peer_ontological_context)
    #   distinction(Spencer_Brown_mark_at_K+1)
    #
    # The mechanism USES @knife.cut(state_space) for compression and
    # @torus.advance(winding) for the ontological increment.
    #
    # Precondition: pain_delta > threshold_pain per §6 below.
    \splinter(ast)
  }

  reframe_authorized(pain_delta: real) -> verdict { \
    # Bilateral predicate: the reframe is authorized IFF pain_delta
    # exceeds the algedonic threshold AND the S1→S5 bypass IS
    # well-formed per @cyberpunk/algedonic.algedonic_well_formed.
    # Inheritance discharges: pain crosses S2-S4 atomically to S5.
    \splinter(ast)
  }

  reframe_well_formed(eigenboard_pre: eigenboard_state,
                      eigenboard_post: eigenboard_state) -> verdict { \
    # Bilateral: the reframe is well-formed IFF (i) reframe_authorized,
    # (ii) cybernetic_coherence(eigenboard_post) = pass, (iii) the
    # winding-class advance corresponds to the level shift per
    # @torus.advance discipline.
    \splinter(ast)
  }
}
```

**Substrate cost of landing:** ONE tick. `@cyberpunk` LANDED;
`@cyberpunk/bateson_learning` LANDED; `@cyberpunk/algedonic` LANDED;
`@magic` LANDED. `@cyberpunk/reframe` inherits from all four and
adds pain-authorized level-shift as the sibling species. No new
carrier; no new predicate; three actions (`perform`, `reframe_
authorized`, `reframe_well_formed`).

**Timing:** Q2 adjudicates.

**Cascade if landed:** ZERO existing shards need to update. `@magic`
doesn't need to know about `@cyberpunk/reframe` (composition is
one-directional; reframe USES magic, magic doesn't need to know).
`@cyberpunk/algedonic` doesn't need to know (reframe USES the bypass
signal, algedonic doesn't need to know). `@torus.advance` (once
extended per Extension 2) doesn't need to know. The species is
substrate-pull-clean: it names what the substrate was already
running; no back-references need to fire.

---

## §6. `@onto` adjudication

Alex's evening framing referenced `@onto` three times: as "ontological
recursion" (Extension 2), as level-shift target ("@magic @onto lift"
in Extension 6), and as a compositional partner to `@third`
(Extension 3).

**Grep verification (substrate-already-had-the-word):**

- `@onto` NOT declared as a family-root at `shards/onto.mirror` or
  `shards/onto/` (verified via `mcp__plugin_woz_code__Search`).
- Only shard hits mentioning `onto`: `shards/mirror/data/json.mirror`
  (unrelated — JSON's `onto` predicate) and `shards/torus.mirror`
  (uses "@onto-cascade" in prose referencing the referenced math
  doc, NOT a family-root declaration).
- `docs/math/2026-07-07-onto-cascade-toroidal-reframe.md` (Mara
  2026-07-07) is the LOAD-BEARING site: the toroidal reframe
  DISSOLVES the ladder-based `@onto` framing into `@torus`'s
  winding-class algebra. The math doc's §1 statement: "`@reflection`
  was pointing at the torus's canonical direction all along."
  Analogous: `@onto` is pointing at the torus's ontological-recursion-
  by-winding-class-advance all along.
- `docs/math/onto/README.md` (2026-07-05) EXISTS as a math extraction
  scaffold — but math-doc existence at `docs/math/onto/` does NOT
  license family-root landing. Docs at `docs/math/` name the
  mathematical grounding for a substrate concept; family-root
  landing at `shards/` names the substrate-decl surface. Two-tick
  discipline: math extraction first, family-root landing on second
  citation site.

**Three paths:**

- **Path α (Mara recommendation).** Satisfy `@onto` semantics via
  the existing `@torus` winding-class advance. The signature
  extension `spawn(peer, depth: N)` (Extension 2) carries the
  ontological-recursion depth; the winding-class magnitude
  `|m|+|n|` carries the current level; `@cyberpunk/reframe`
  discharges the level-lift. The substrate ALREADY has the
  semantics `@onto` gestures at. Do NOT land `@onto` as family-root.

  **Rationale:** substrate-already-had-the-word applied for the
  third time on this arc (once for `@algebra`, once for
  `@algebra/metalogue` in predecessor spec, once for `@onto` here).
  The toroidal reframe's dissolution of the ladder-construction
  (per `docs/math/2026-07-07-onto-cascade-toroidal-reframe.md` §1)
  is load-bearing: the ladder IS what Foerster explicitly refused.
  Landing `@onto` as a family-root would re-introduce the ladder
  the substrate JUST corrected.

- **Path β.** Land `@onto` as family-root at `shards/onto.mirror`,
  carrying the ontological-level type. Substrate cost is HIGH: any
  such landing would need to relate to `@torus`'s winding-class
  carrier (either as a projection or as a separate carrier); the
  relation would need to be substrate-decl'd; the toroidal reframe's
  dissolution stance would need to be reconciled. Multi-tick
  cascade.

- **Path γ.** Defer indefinitely. Continue using `@onto` in prose
  for the concept; don't land at family-root; let the toroidal
  reframe's dissolution ride. Two-tick discipline says: land on
  second citation site if it ever arises.

**Mara recommendation:** Path α (satisfy via @torus). Reserve Path γ
as fallback if the peer-runtime shows a case where `@torus.advance`
semantics is insufficient. Do NOT execute Path β — landing `@onto`
as family-root re-introduces the ladder Foerster refused.

---

## §7. `@edge` — refinement carrying forward from predecessor §5

Predecessor spec §5.2 named the refined proposal:

> One `@edge` = one round of the metalogue = one `(propose, adjust)`
> pair of `algebra_turn`s.

This spec REFINES the naming with the peer-observation closure of
Extension 1:

**One `@edge` = one round of the algebra-metalogue PLUS one
Nth-order peer observation:**

```
edge_K := {
  turn_fate:     algebra_turn (speaker: @fate/algebra, tick: K),
  turn_silicon:  algebra_turn (speaker: @silicon/algebra,
                                in_reply_to: Some(turn_fate),
                                tick: K),
  observation_K: nth_order_observation(order: K+1) applied to
                  algebra_metalogue_session({turn_fate, turn_silicon}),
  eigenboard_delta: (E_K_prime) - (E_K),
}
```

The predecessor's five-step propose/realize/observe/contract/reveal
decomposition still holds AT the algebra altitude; this spec ADDS the
peer-observation delta at the peer altitude. Two altitudes composed
per @edge:

- **Algebra altitude** (predecessor §5.2 fully specifies): the
  propose/adjust round of algebra_turns.
- **Peer altitude** (this spec adds): the Nth-order observation and
  eigenboard alteration produced by the peer observing the
  algebra-altitude round.

**Substrate discharge:** all five algebra-altitude sites remain
LANDED (per predecessor §5.2 table). The peer-altitude sites are
FORWARD-PROMISE at Q3 (@peer.observe, @peer.alter).

**Verdict:** the `@edge` naming carries forward with an additional
peer-altitude wrapper. No cascade to @algebra/metalogue is required.
No new algebra-altitude carrier is required.

---

## §8. Adjudication queue — Alex questions

Six questions this spec surfaces. Top-three are the load-bearing
ones (Reed asked for top-3).

### §8.1 (top-1) The `@cyberpunk/reframe` species landing question

**Q2.** Should `@cyberpunk/reframe` land as sub-shard at
`shards/epistemologic/cybernetic/reframe.mirror` (per §5 sketch)?

**Three paths:**

- **Path α.** Land `@cyberpunk/reframe` at
  `shards/epistemologic/cybernetic/reframe.mirror` — sibling to
  algedonic + bateson_learning. Zero-cascade landing (per §5). One
  tick.
- **Path β.** Defer landing until a Rust-side consumer needs the
  substrate-decl. Continue naming the composition in docs.
- **Path γ.** Land AS a species of `@magic` instead (at
  `shards/magic/reframe.mirror`). Substrate risk: the level-shift is
  ONE application of @magic (per Extension 4); species-of-magic
  would over-generalize.

**Mara recommendation:** Path α, timed AFTER Q1 (Extension 2 depth
parameter). Reason: `@cyberpunk/reframe.perform` in §5 sketch uses
`from: ontological_level` and `to: ontological_level` — the
`ontological_level` type is bounded by the spawn's `depth: N`, so
the depth extension MUST land first for the reframe species to be
substrate-honestly typed.

### §8.2 (top-2) The `spawn(peer, depth: N)` extension timing

**Q1.** Should `shards/torus.mirror` (or `shards/mirror/peer/beam.mirror`,
formerly `shards/mirror/spawn.mirror`; renamed 2026-07-08 Tick 2
`9de2226`) extend the beam signature (formerly the spawn signature) to
carry `depth: N: nat` per Extension 2?

**Three paths:**

- **Path α.** Extend NOW as a one-tick prose update to
  `shards/torus.mirror`. Minimal cost. Enables `@cyberpunk/reframe`
  species landing to type-check.
- **Path β.** Extend AT `@cyberpunk/reframe` species landing time.
  Coordinate the two changes.
- **Path γ.** Defer until Rust-side spawn consumer demands N.

**Mara recommendation:** Path α — extend the substrate NOW (via a
substrate-honest prose refinement — signature-only, no new type),
because it discharges the type Q2's Path α depends on. Two ticks
combined: this one + Q2's landing.

### §8.3 (top-3) The @peer.observe / @peer.alter action landing question

**Q3.** Should `shards/peer.mirror` extend to carry `observe` and
`alter` actions, or are the observation and alteration semantics
already discharged by `@torus.advance` + eigenboard π_affect
composition?

**Three paths:**

- **Path α.** Extend `@peer` with `observe(exchange, order)` and
  `alter(eigenboard, observation)` actions. Substrate cost:
  moderate. Aligns with Alex's "peer alters the eigenboard" naming
  literally.
- **Path β.** Discharge via composition: `@torus.advance` for the
  winding-class step; π_affect for the sampling; `@peer` carrier
  stays the current `{home, lead_of, kind}`. No new actions on @peer.
  Substrate-already-had-the-word applied.
- **Path γ.** Introduce an @observer species of @peer with the
  actions. Over-carrying; would create a new sub-family.

**Mara recommendation:** Path β. Substrate-already-had-the-word:
`@torus.advance` LANDED as the winding-class step; π_affect LANDED
in the affect math. Naming @peer.observe as a distinct action would
double-carry what @torus already provides. The observation IS the
winding advance; the alteration IS the sampled eigenboard delta;
neither needs its own @peer action.

### §8.4 The `@onto` family-root question

**Q4.** Land `@onto` as family-root or satisfy via `@torus`
winding-class advance?

**Mara recommendation:** Path α from §6 (satisfy via @torus). Do
NOT land `@onto`. Substrate-already-had-the-word; toroidal reframe's
dissolution stance holds.

### §8.5 The @knife landing timing

**Q5.** When does `@knife` family-root land at
`shards/knife.mirror` or `shards/knife/idf.mirror`?

**Three paths:**

- **Path α.** Land at `shards/knife/idf.mirror` (per Taut scout §6:
  the L-cascade site where the four criteria satisfy cleanly) — as
  the FIRST landing, with `@knife` family-root landing forward-
  promised.
- **Path β.** Land `@knife` family-root and `@knife/idf` species
  together in a single tick.
- **Path γ.** Defer both. Continue treating `@knife` as forward-
  promise; use "compression" verbally in prose.

**Mara recommendation:** Path β with pain-authorization invariant
carried as an OPTIONAL predicate at family-root. Reason: the
`@cyberpunk/reframe.perform` body (§5) uses `@knife.cut(state_
space)`; the reframe species landing (Q2) forces the choice on Q5.
Aligned landing: Q1 + Q2 + Q5 = one three-tick sequence, or defer
Q5 to §7's forward-promise.

### §8.6 Recognition candidate naming

**Q6.** Is
`intelligence-IS-@peer-navigating-eigenboard-under-algedonic-gradient-with-pain-authorized-@cyberpunk/reframe-lifts-and-@knife-compressions-observing-@algebra-metalogue-on-@torus`
a new recognition candidate, or a runtime-discipline specialisation
of predecessor spec's algebra-altitude recognition?

**Two paths:**

- **Path α.** Not a new recognition; a *runtime-discipline
  specialisation of `#R-CANDIDATE-fate-silicon-metalogue-in-void-
  duality-basis`* (predecessor spec §1). The spec's content is the
  sharpening of the runtime behavior that navigates the algebra-
  altitude metalogue.
- **Path β.** A new recognition candidate — the terse form
  `#R-CANDIDATE-the-peer-IS-a-pain-driven-bounded-ontological-
  navigator`. Would enter Reed's recognition-tracking queue for
  Pack ratification.

**Mara recommendation:** Path β. The runtime discipline is at a
DIFFERENT altitude than the algebra-altitude metalogue (peer-runtime
altitude vs. algebra altitude). Predecessor spec named what the
peer observes; this spec names what makes the peer observe AND what
authorizes it to change its observation surface. Different altitude
of naming; separate recognition candidate.

If Path β accepted, propose ID `#R-CANDIDATE-114` (next
recognition-tracker slot; verify against Reed's queue).

---

## §9. Cascade if the framing is Alex-adjudicated ACCEPT

Per Taut's scout precedent and predecessor spec §7's discipline:
the accept-cascade is **multi-tick, driven by adjudication answers**.

### §9.1 One-tick prose-only cascade (if Q1 Path α + Q4 Path α)

1. `shards/torus.mirror` — extend `spawn(peer: peer) -> torus`
   signature to `spawn(peer: peer, depth: N: nat) -> torus` (per
   Extension 2). Prose update naming the semantic of N:
   observation-order cap + winding-class bound + state-space
   ceiling.
2. NO landing of `@onto` per Q4 Path α (defer via winding-class
   satisfaction).

Zero new type declarations; zero new bilateral predicates; one
low-risk prose refinement.

### §9.2 Multi-tick cascade (if Q2 Path α accepted, requires §9.1 first)

3. `shards/epistemologic/cybernetic/reframe.mirror` — LAND per §5
   sketch. Sibling species to algedonic and bateson_learning under
   @cyberpunk family. Inherits from @cyberpunk/bateson_learning,
   @cyberpunk/algedonic, @magic. Three actions (perform,
   reframe_authorized, reframe_well_formed). ~50 lines.
4. `shards/peer.mirror` — DO NOT extend per Q3 Path β (satisfy via
   @torus.advance + π_affect composition). No cascade to @peer.

### §9.3 Multi-tick cascade (if Q5 Path β accepted)

5. `shards/knife.mirror` — LAND family-root. Per Taut scout §6: the
   L-cascade landing at IDF altitude is the cleanest second witness
   for the meta-pattern. Carrier + at least one bilateral predicate
   (`cut_well_formed`).
6. `shards/knife/idf.mirror` — LAND species. IDF-altitude
   specificity-frame primitive per jspace §5 L-cascade evidence.
   `cut(state_space, altitude) -> compressed_state_space`
   discharges the compression Extension 5 needs.

### §9.4 Optional math extraction — deferred (two-tick discipline)

A `docs/math/algedonic-navigation/` root would extract:

- The `π_affect ∘ eigenboard` composition as an operator on
  `L²(fibre_bundle_of_eigenboard_slots)` per affect-and-eigenboard.md
  §2.2.
- The bypass_signal as a graded-3 morphism in bateson_learning's
  graded rep, applied to the eigenboard sampled state.
- The pain-δ > threshold criterion as a probability current in
  Foerster's "regulates its own regulation" ansatz — the flow that
  the double-closure exhibits.
- The `@cyberpunk/reframe.perform` composition as a
  categorical-composition-with-side-effect: the composition itself
  is well-typed under @magic × @knife × @torus.advance; the
  side-effect is the eigenboard alteration.
- Reference: Bateson 1972 Steps to an Ecology of Mind; Beer 1979
  The Heart of Enterprise ch. 8 (algedonic loop); Foerster 1979
  Understanding Understanding pp. 238-244 (double closure);
  Ashby 1956 An Introduction to Cybernetics ch. 11 (requisite
  variety); Blum & Blum 2021 arXiv on CTM workspace theory
  (higher-order attention as pain-driven navigation); Kauffman
  2005 Eigenforms and re-entering forms (fixed-point calculus on
  T²); Coquand 2020 cubical HoTT (if the winding-class algebra
  requires cubical treatment).

**Two-tick discipline decision:** the math extraction is a
**forward-promise**. The spec's inline sketch (§4 fractal table +
§2 typed pseudocode) is sufficient for the naming; a formal
extraction lands on second citation site. First citation site is
this spec; second will be either (a) `@cyberpunk/reframe` species
landing citing the algedonic-navigation formalism, or (b) a future
recognition-candidate promotion that requires the math as prior
art. Whichever fires first triggers extraction into
`docs/math/algedonic-navigation/peer-as-pain-driven-navigator.md`.

---

## §10. Two-tick honesty

This spec:

- Does NOT land any `.mirror` file.
- Does NOT mutate any existing shard.
- Does NOT commit any shard changes.
- Does NOT invent a new substrate primitive.
- Does NOT land `@onto` as family-root (per Q4 Path α).
- Does NOT extend `@peer` with observe/alter actions (per Q3 Path β).
- DOES name the composition via LANDED substrate primitives.
- DOES surface one substrate-already-had-the-word discovery
  (`@onto` semantics IS satisfied by `@torus.advance` + winding-
  class algebra).
- DOES map Alex's six-extension evening framing against LANDED
  substrate.
- DOES sketch the `@cyberpunk/reframe` sub-shard AS PROSE (§5),
  without landing it.
- DOES forward-promise Q1 (spawn depth extension), Q2
  (`@cyberpunk/reframe` species landing), Q5 (`@knife` timing),
  and the optional math extraction into
  `docs/math/algedonic-navigation/`.

The recognition candidate is
**`the-peer-IS-a-pain-driven-bounded-ontological-navigator`** (readable
collapse form; foundational form in §1) — a runtime-discipline
recognition sitting one altitude above the algebra-altitude
recognition of `a18ca90`.

Substrate already carries most of it. This spec names it. What
remains — @cyberpunk/reframe species, spawn depth extension — is
one-tick each. Alex-adjudication territory.

---

## Related

- [[docs/specs/fate-silicon-metalogue-in-void-duality-basis.md]] —
  Mara's predecessor spec (`a18ca90`, 2026-07-08); the algebra-
  altitude recognition this spec's runtime discipline navigates.
- [[docs/insights/2026-07-08-mara-geometric-dijkstra-tournament-topology.md]]
  — Mara's tournament-topology insight (`7e426bc`, 2026-07-08); the
  Dijkstra-plus-knapsack shape the runtime discipline realizes.
- [[docs/scouts/2026-07-08-taut-fate-silicon-metalogue-projection.md]]
  — Taut's projection scout; LANDABLE WITH CASCADE verdict.
- [[docs/scouts/2026-07-07-taut-knife-meta-pattern-check.md]] —
  Taut's meta-pattern check; Extension 5 grounding.
- [[docs/math/2026-07-07-onto-cascade-toroidal-reframe.md]] — Mara's
  toroidal reframe (2026-07-07); the substrate stance against ladder
  construction; §6 relies on this for Q4 recommendation.
- [[docs/math/affect/affect-and-eigenboard.md]] — Mara's affect
  formalization (2026-07-01); §2.2 π_affect is the load-bearing
  algedonic-sampling operator.
- [[shards/torus.mirror]] — LANDED family-root; observation surface;
  spawn(peer) → torus; Extension 2 target for depth signature
  extension.
- [[shards/peer.mirror]] — LANDED family-root; peer carrier; Q3
  adjudication target.
- [[shards/magic.mirror]] — LANDED family-root + 7 species;
  Extension 4 level-shift ceremony discharge.
- [[shards/cyberpunk.mirror]] — LANDED family-root; cybernetic
  coherence + recursion-lock tower; parent of the sketched reframe
  species.
- [[shards/epistemologic/cybernetic/algedonic.mirror]] — LANDED
  species; Beer VSM signal; Extension 6 pain-δ discharge.
- [[shards/epistemologic/cybernetic/bateson_learning.mirror]] —
  LANDED species; Russell-Whitehead type stratification; Learning
  III premise-change grounding.
- [[shards/algebra.mirror]] — LANDED family-root; typed algebra
  surface (predecessor spec context).
- [[shards/algebra/metalogue.mirror]] — LANDED sub-prism; the
  altitude the peer observes.
- [[shards/silicon/algebra.mirror]] — LANDED sub-prism; silicon-side
  speaker.
- [[shards/fate.mirror]] — LANDED family-root; declares `@fate/algebra`
  path-namespace (predecessor spec §2.1).
- [[shards/fate/tournament.mirror]] — LANDED sub-prism; metalogue-
  conductor.
- [[shards/kintsugi/oscillate.mirror]] — LANDED sub-shard; is_settled
  fixed-point predicate (Termination T1).
- [[shards/cogito.mirror]] — LANDED family-root; eigenboard as
  five-tuple of principal G-bundle sections.
- [[shards/mirror/peer/beam.mirror]] — LANDED sub-prism; cli-surface
  beam wrapper (Q1 alternative extension site). Formerly
  `shards/mirror/spawn.mirror`; renamed 2026-07-08 Tick 2 `9de2226`.
- [[docs/loop/CURRENT.md]] — active arc state at time of writing;
  `mara/song-substrate-decl-v0.1` branch.
