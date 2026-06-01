# Ants, Colonies, Stigmergy — Mirror's Tournament Through Bio-Inspired Optimization

2026-05-26 — research synthesis testing Alex's associative leap from
mirror's tournament-level Lyapunov convergence to ant colony behavior.

*Status: **Yellow** — research findings; not substrate truth. Section
verdicts are marked **Established (cite)** / **Aligned-but-different** /
**Metaphor only**. Reed authored the synthesis; Alex provided the leap.*

---

## The leap

Mirror's `kintsugi-tournament` spec just landed (commit `1b177aa`) a
Lyapunov-style convergence argument at **tournament granularity**:
individual fractures may locally worsen tension; tournament rounds
globally decrease total holonomy. The proof cites Hajek 1988 (simulated
annealing), Hart-Nilsson-Raphael 1968 (A* admissibility), and the FPTAS
knapsack literature (Ibarra-Kim 1975 / Lawler 1979 / Jin 2019).
§10.F.4 surfaces three candidate formalizations of *tournament
completeness* (A*-style admissibility / depth-d exhaustiveness /
anytime B&B-PTAS) and §10.F.5 surfaces the approximation-ratio
question (what tournament budget guarantees a (1+ε)-approximate round).

Alex's associative leap: this looks structurally like **ant colony
behavior** — individual ants take suboptimal paths but the *collective*
optimizes via stigmergy (Pierre-Paul Grassé 1959), the
environment-as-substrate carrying traces that subsequent ants read and
reinforce. The leap is into a 30-year-old metaheuristic literature
(Dorigo 1992; Dorigo-Stützle 2002–2005) whose convergence proofs *exist*
and have a *specific* structural shape.

This document tests, section by section, whether the leap holds
structurally or whether it is metaphor only.

---

## What the literature says

### 1. Ant Colony Optimization (ACO) — Dorigo, Blum, Stützle

**Verdict: Established (cite). Strong structural alignment with
mirror's tournament — not identity, but family resemblance with
mechanistic shared shape.**

ACO (Dorigo 1992; Dorigo & Stützle 2004 book) is a metaheuristic for
hard combinatorial optimization in which artificial ants build solutions
by sampling a probabilistic *pheromone model* over solution components,
then update that model based on the quality of constructed solutions.
The central object is the **pheromone vector** τ over solution
components; the update rule is roughly

    τ_i ← (1 - ρ) · τ_i + ρ · Σ_{s containing i} F(s) · p(s | τ)

where ρ ∈ (0,1] is the *evaporation rate* and F is the quality function.
Dorigo & Blum (2005, *Theoretical Computer Science* 344:243–278) give the
canonical convergence survey; the two load-bearing theorems are:

- **Convergence in value** (Theorem 1, Stützle-Dorigo 2002):
  ACO_{bs,τ_min} with a fixed positive lower bound τ_min on pheromone
  values finds an optimal solution with probability 1 in the limit:
  lim_{t→∞} P*(t) = 1.
- **Convergence in solution** (Theorems 2, 3, Gutjahr 2000–2002;
  Dorigo & Blum 2005): with a time-dependent lower bound
  τ_min(t) = d/log(t+1) that decreases slowly enough, ACO_{bs,τ_min(t)}
  converges in solution — every ant eventually constructs the optimal
  with probability 1.

The **mechanism of the proof** is the structural payload here. Each
round's expected pheromone update is a contraction toward solutions of
higher quality (§2.4 of Dorigo & Blum: the "model of an ACO algorithm"
is a deterministic dynamical system obtained by replacing the stochastic
update with its expectation). §4 of the same paper proves the relation
to **stochastic gradient ascent** and the **cross-entropy method**: the
ACO update IS a stochastic gradient step on the expected quality
functional WF(τ) = Σ F(s) p(s|τ), with the gradient ∇ ln p(s|τ)
computable in closed form for the ACO probability function.

Wuhang Lin et al. (the martingale-process analysis,
researchgate/287500035) extend the convergence framing: the cost
sequence is a non-negative submartingale; the stopping time (first
iteration that finds the optimum) is finite a.s. by Doob's theorem.

**Structural alignment with mirror's tournament-level Lyapunov.**

| ACO | Mirror's `@fate` tournament |
|-----|----------------------------|
| Individual ant's solution can be globally suboptimal | Individual fracture can locally *worsen* tensor norm |
| Pheromone vector update is in expectation a contraction | Tournament round R satisfies ‖R(T)‖ < ‖T‖ off fixed points (§10.A) |
| Round = one iteration of the colony | Round = one execution of `@fate.minimize` with bounded backtracking |
| Convergence in value (∀ε ∃t: P*(t) ≥ 1−ε) | Existential round-level Lyapunov decrease |
| Stochastic gradient ascent on WF(τ) | Tournament as descent on holonomy/Fiedler value |
| Martingale stopping time finite a.s. | Lawvere fixed-point reached in O(log(1/ε)/log(1/γ)) rounds (Banach) |

**Where the structural alignment is real, not metaphor:**

1. **Both proofs require a *lower bound* to be a precondition.**
   ACO needs τ_min > 0 (or τ_min(t) decreasing slowly enough) for
   convergence in value to hold — without it, the search space
   collapses prematurely. Mirror's analogous precondition is §10.F.3:
   the tensor norm must be bounded below by 0 (with equality only at
   the fixed point). Both impose *a positive floor on the
   exploration*.
2. **Both proofs are about expectation, not pointwise descent.**
   ACO's contraction is on the expected pheromone update; the actual
   per-iteration trajectory is stochastic and can locally worsen.
   Mirror's §10.F.2 retirement says exactly the same thing at the
   fracture level: per-step ascent is permitted; round-level descent
   is required.
3. **Both proofs distinguish convergence-in-value from
   convergence-in-solution.** Mirror's §10.A (monotone convergence at
   round boundaries) is the convergence-in-value analog; mirror's
   Banach fixed-point in `kintsugi-formatter` is the
   convergence-in-solution analog. ACO needs different theorems
   (different lower-bound regimes) for the two; mirror should expect
   to need the same distinction.

**Where the alignment is structurally different:**

1. **ACO is unbounded — the colony is infinite-state-space and
   probabilistic.** Mirror's `@fate` is **sub-Turing by construction**
   (per `mirror-compile-bootstrap.md`). ACO's convergence-in-solution
   theorems require *infinite time* — they are asymptotic. Mirror
   needs *decidable bounded-budget* termination (per
   `2026-05-26-heuristic-termination-for-sub-turing-subgraphs.md`).
   This is a genuine mismatch: ACO theorems give probability-1
   convergence in the limit, not finite-time guarantees. The
   bounded-budget approximation question (§10.F.5) is exactly where
   ACO theory stops being directly transferable — see §2 below for
   the partial-but-real Wikipedia/MMAS* result.
2. **ACO has no `eigenboard sheaf`.** ACO's state space is a flat
   product of pheromone parameters; mirror's state space is a
   cellular sheaf on the five-operation graph with non-trivial
   restriction maps (per `eigenboard-representation.md`). The
   tournament outcome in mirror is a *bundle automorphism*; an ACO
   step is a vector update in R^{|C|}. Mirror inherits the proof
   *shape* from ACO but operates over richer geometry.
3. **ACO ants don't backtrack within a round.** Each ant constructs
   one solution; the colony's "backtracking" is the cross-round
   reweighting of pheromones. Mirror's `@fate` round backtracks
   *within* the round (GRAM-style multi-trajectory sampling). This is
   a fundamentally different round granularity — mirror's round is
   more expressive.

**Net.** ACO's convergence theory is the **closest established prior
art** for mirror's tournament-level Lyapunov framing. The structural
shape transfers: round-granular convergence with per-step ascent
permitted, lower-bounded state space, stochastic-gradient-style update
on an expected-quality functional, martingale termination. The
transfer is *not identity* — mirror's sub-Turing constraint, sheaf
geometry, and within-round backtracking are real differences that
mean mirror cannot simply cite ACO theorems and be done. But the
*proof shape* mirror is reaching for already exists at this level of
structural specificity in the ACO literature, and that is real prior
art, not metaphor.

---

### 2. Stigmergy — Grassé (1959) and its successors

**Verdict: Aligned-but-different. The substrate-as-environment framing
is genuinely shared; the *what counts as a trace* differs in ways that
matter.**

Stigmergy was introduced by Pierre-Paul Grassé in 1959 to explain how
termites coordinate mound-building without direct communication or
central plan. The four components, formalized by Theraulaz &
Bonabeau (1999) and tightened by Heylighen (2016, Cognitive Systems
Research): **agent, medium, trace, stimulation rule**. The trace left
by an action in the medium stimulates subsequent actions by other (or
the same) agents. Grassé distinguished two varieties:

- **Quantitative (sematectonic) stigmergy:** continuous-valued traces
  modulate response probability (pheromone gradients).
- **Qualitative stigmergy:** discrete environmental configurations
  trigger discrete actions (wasp nest-building rules).

**Mapping to mirror.**

Mirror's substrate carries a corpus — grammar files, eigenboard
sections, gestalt entries, refs/eigenboard/<agent>/tournaments/<oid>.
Fractures leave *typed records* in that corpus: section morphisms,
tournament outcomes, kintsugi ticks, beam-to-beam deltas. Subsequent
fractures read those records (Reflection observes section history
before proposing the next move; `@cogito.strategy` looks at recurring
collision patterns; the eigenboard's ancestor chain influences
bundle automorphism selection).

| Stigmergy component | Mirror analog |
|--------------------|---------------|
| Agent | A `@fate` proposer (Abyss/Introject/Cartographer/Explorer/Fate model) |
| Medium | The eigenboard sheaf + gestalt + section history |
| Trace | `tournament_outcome` record, section morphism, kintsugi tick |
| Stimulation rule | Reflection's observation function; `@cogito.strategy` |

This is **genuinely shared structure**, not metaphor. The
*indirection* through the environment is the key. Mirror's
`tournament_outcome` (`refs/eigenboard/<agent>/tournaments/<oid>`) IS
a sematectonic stigmergic trace: the post-merge section state itself
is the stimulus for the next round, not a message passed between
proposers.

**Where it differs structurally.**

The arxiv:2604.03997 *Ledger-State Stigmergy* paper (April 2026,
pre-print at the writing of this doc) names the precise place where
mirror's stigmergy diverges from biological stigmergy: **ledger
traces persist indefinitely; biological pheromones evaporate**. Mirror
is ledger-shaped — the eigenboard's section history is content-addressed
and grows monotonically; nothing is forgotten unless an explicit
garbage-collection or evaporation rule is added. That is genuinely a
different shape from biological stigmergy. See §3 below for whether
adding an evaporation analog to mirror would be a productive design
move or a category error.

The Heylighen 2007 "Stigmergy as a Universal Coordination Mechanism"
paper (pespmc1.vub.ac.be/Papers/Stigmergy-Springer.pdf) generalises
stigmergy beyond biology to chemistry, Wikipedia, market dynamics, and
human cognition. Within that wider frame, mirror's substrate-as-corpus
IS stigmergic in the Heylighen sense: traces in a shared medium
stimulating subsequent actions without direct messaging. Calling
mirror's eigenboard "a stigmergic substrate" is *more accurate than
metaphor* under Heylighen's broad definition.

**Net.** Stigmergy as Grassé / Theraulaz / Heylighen defined it IS the
shape of mirror's substrate-as-environment. The biological-pheromone
specifics (evaporation; chemical diffusion; spatial locality) are
biological details mirror doesn't share — but the *coordination
principle* (agents read and modify a shared medium; the medium's state
stimulates subsequent actions) maps directly. Naming mirror's
substrate "stigmergic" is honest under the universal-mechanism reading;
calling individual `@fate` proposers "ants" is metaphor.

---

### 3. Pheromone evaporation as forgetting

**Verdict: Aligned-but-different — the mechanism is real prior art for
an exploration-exploitation balance, but mirror's kintsugi healing is
not (yet) the same thing. There is a candidate design move here.**

In ACO, pheromone evaporation (the (1 - ρ) factor in the update rule)
plays two distinct roles documented in the Dorigo-Blum survey and in
recent work (e.g. arxiv:2601.07597 *Pheromone-Focused ACO*; the
arxiv:2501.10810 *Convergence and Running Time of Time-dependent ACO*
result below):

1. **Exploration-exploitation balance.** Without evaporation,
   pheromones accumulate unboundedly on first-found paths; the colony
   converges to local optima. With evaporation, old reinforcement
   decays, leaving the colony open to discovering new improving paths.
2. **Convergence regularization.** Stützle-Dorigo's MMAS uses an
   *explicit upper bound* F(s*)/ρ on pheromone values — evaporation
   IS the mechanism that prevents pheromone-value blowup and keeps the
   transition probabilities well-defined.

The arxiv:2501.10810 result (Liu et al., January 2025) is the most
recent direct evidence on the role of evaporation in *running time*:
the paper shows that GBAS/tdev (time-dependent *evaporation*) can have
**super-polynomial expected running time** on the single-destination
shortest-path problem, while GBAS/tdlb (time-dependent *lower bound*
on pheromone, with fixed evaporation) achieves *polynomial* running
time. The mechanism distinction matters: time-decaying the floor
(τ_min) helps; time-decaying the evaporation rate hurts. Pheromone
evaporation is essential but its specific dynamics interact non-trivially
with convergence speed.

**Mapping to mirror.**

Mirror's `kintsugi` today is the *healing* operation — it observes
tension/contradiction in the corpus and fractures + reassembles to
remove it. There is no current explicit "forgetting" mechanism: the
eigenboard's section history accumulates monotonically (per
`fragmentation-as-generated.md`); the gestalt grows.

**Candidate alignment.** Kintsugi could plausibly absorb an
evaporation-style mechanism in two places:

- **Corpus aging.** Old gestalt entries / superseded section states
  could decay in *influence weight* over time (not be deleted — the
  Merkle tree is the ledger; deletion would violate content-addressing).
  Influence-weight decay would prevent ancient state from dominating
  Reflection's observation function and would create room for new
  morphisms.
- **Fracture confidence decay.** `kintsugi-fracture-confidence-and-scene-dispatch.md`
  has confidence scores attached to fractures; those scores could
  decay with time-since-application, modeling the loss of relevance
  of old fixes as the substrate evolves around them.

**Where the alignment breaks (honestly).** Biological pheromones
evaporate from a physical substrate; mirror's substrate is
content-addressed and *cannot* lose its history (this is a hard
constraint from `kintsugi-self-hosting.md` / the OID architecture).
The arxiv:2604.03997 ledger-state-stigmergy paper makes the same point
for blockchains. So if mirror adds evaporation, it must be **influence
decay**, not actual deletion — weights diminish; the trace persists.
This is a structural design choice with real consequences (see §Open
design implications).

**Net.** Evaporation in ACO is real prior art for balancing exploration
and exploitation, with concrete running-time consequences (Liu et al.
2025). Mirror's kintsugi healing is *not currently* a forgetting
mechanism in the ACO sense — it's a tension-removal mechanism. Adding
influence-decay to mirror's gestalt would be a new design move
structurally informed by ACO; it should not be presented as "already
there." The leap holds at the level of *available design move*, not
*existing parallel*.

---

### 4. Phase transitions and the small-model claim

**Verdict: Aligned-but-different. The phase-transition phenomenon is
real and well-cited; whether it grounds mirror's small-model (∼10–100M
param) claim depends on what mirror means by "phase transition."**

The Gov-Yam et al. 2025 paper (arxiv:2506.01209, *Maximal response to
a mechanical leader at critical group size in ant collectives*, Nature
Communications 2025) is the strongest recent evidence on ant-colony
phase transitions: in cooperative transport by *Paratrechina
longicornis*, **group susceptibility to external stimuli peaks
significantly at an intermediate group size**, where the collective
sits at the transition between disordered (small) and ordered (large)
phases. The paper maps the system to an Ising model and demonstrates
that the critical-group-size regime is where a *single leader ant* can
redirect the whole group with maximum efficiency.

Additional supporting work:

- Gelblum et al. 2022 (PNAS, *Emergence of a collective sensory
  response threshold*): the threshold temperature at which ant colonies
  evacuate is a function of colony size; the response is dominated by
  social feedback. Size *changes* the colony's collective threshold.
- Li et al. 2014 (PNAS, *Chaos–order transition in foraging behavior of
  ants*): a chaos-to-order transition occurs as foraging dynamics scale.

**Mapping to mirror's small-model claim.**

Mirror's `@fate` is committed to ~10–100M parameter models (per
`2026-05-26-fate-as-recursive-multi-trajectory-backtracking.md`), much
smaller than frontier LLMs. The substrate argument is that **depth via
recursion + width via multi-trajectory sampling + structured
backtracking** substitutes for raw parameter count. The TRM/GRAM
empirical anchors (arxiv:2510.04871 / arxiv:2605.19376) show ~7M params
+ recursion + sampling reach 45–52% / 8–44.6% on ARC-AGI-1/2 — well
above frontier baselines at ∼1000× the parameter count.

**Does the ant-colony phase transition transfer?**

The Gov-Yam et al. finding is structurally suggestive: **the most
responsive regime is at *intermediate* group size, not maximum**. A
colony of millions can be *less* susceptible to a single leader's
information than a critical-size colony. If mirror's substrate has an
analogous phenomenon — there exists a *critical model size* at which
recursive backtracking is maximally effective at picking up signal from
the corpus — then frontier-scale models are *past* the critical
regime, not before it.

But this is **speculative as transferred to neural networks**. The
Gov-Yam paper is about a specific Ising-model-mappable group-dynamics
phenomenon, not about parameter count in a recursive inference system.
There's no direct theoretical bridge from ant-colony criticality to
transformer parameter counts. The structural shape (critical regime
for information susceptibility) is suggestive; the empirical anchors
for mirror's small-model claim already exist (TRM, GRAM); the
ant-colony framing is **rhetorically powerful but not load-bearing**.

The self-organized-criticality literature (Bak-Tang-Wiesenfeld sandpile;
arxiv:2409.15668 on why collective behaviors self-organise to
criticality) provides the broader frame: many natural collective
systems sit at critical points. Mirror's `kintsugi` arguably tunes the
tensor system *toward* criticality (the Fiedler-value /
spectral-gap-as-Lyapunov framing is consistent with this). But this is
a separate research program from "mirror's small models work
because ant colonies are critical" — the latter is metaphor.

**Net.** Critical-group-size in ant colonies is well-cited established
biology with a recent Nature Communications anchor (Gov-Yam et al.
2025). Whether it grounds mirror's small-model claim is *interpretive*:
the phenomenon is suggestive, the mechanism transfer is speculative,
and the empirical anchors for the claim itself live elsewhere (TRM /
GRAM). The leap holds as *rhetorical resonance*, not *structural
grounding*.

---

### 5. Decentralized halting

**Verdict: Metaphor only. The shapes look similar at a distance; the
mechanisms are fundamentally different.**

Ant colonies do stop foraging — when food sources are exhausted,
pheromone evaporation reduces trail intensity, recruitment slows,
foragers return to the nest. There is no central "halt" signal; the
termination is emergent from the pheromone dynamics.

**Mapping to mirror.**

Mirror's halting is computed by `@scheduler.reduction_budget` and the
Lawvere fixed-point check (`kintsugi-formatter.md` stage 5). The
formatter's iteration terminates when (a) the section reaches a Lawvere
fixed point, (b) the budget is exhausted, or (c) one of three named
failure modes is hit. This is **not emergent halting from
evaporation-style dynamics**; it is *decidable termination by
structural property*.

**Distributed-systems literature is the closer parallel.** Dijkstra's
diffusing-computation termination detection (Chapter 7 of Kshemkalyani
& Singhal, *Distributed Computing*, available on cs.uic.edu/~ajayk), and
the TLA+ formalizations
(deepwiki.com/tlaplus/DrTLAPlus/3.2-termination-detection), give the
established mechanisms for detecting global termination in distributed
systems without a central observer. These are concrete algorithmic
mechanisms (sweep markers, token rings, counters) — nothing like the
physical-decay dynamics of ant pheromones.

**The structural mismatch.** Ant colony foraging termination is
*physical equilibrium* — pheromone deposition equals pheromone
evaporation; no new ants are recruited; the system relaxes to a
ground state. Mirror's halting is *fixed-point detection* — the
operator T has reached a point where T(x) = x. These are not the same
mechanism; calling them both "decentralized halting" elides the
difference.

**Net.** Both shapes involve termination without a central signal.
That shared feature is too generic to ground a structural transfer.
The leap here is metaphor.

---

### 6. Recursive substructure ("colonies of cells, of types of bindings")

**Verdict: Metaphor only — but rhetorically a clean one. The
Lachmann-Sella 1995 finding actually *cuts against* mirror's
commitments.**

Lachmann & Sella (1995, Sante Fe Institute reprint), *The
Computationally Complete Ant Colony: Global Coordination in a
Matter-Free, Identity-Free System*, prove a striking result: a simple
ant-colony model in which each ant has a finite number of internal
states with environmental and ant-ant interactions can implement
**any computation a Turing machine can perform**. They construct
NOR-gates from ant-ant interactions and chain them to universal
computation.

**This is the place where the leap CUTS AGAINST mirror's design.**

Mirror is **sub-Turing by construction** (per `mirror-compile-bootstrap.md`
and the strict-and-total-classification spec). Every grammar body must
be decidable; halting must be computable; Rice's theorem must be
inapplicable. Lachmann-Sella's result is that *real* ant colonies are
Turing-complete — i.e., the biological substrate mirror is being
compared to has *more expressive power* than mirror does, not less.
If you really wanted mirror to behave like an ant colony in the
Lachmann-Sella sense, you would *give up the sub-Turing commitment*
for undecidable expressive power. That is the opposite of what mirror
is doing.

The recursive-substructure framing ("colonies of ants, of cells; types
of types, of bindings; mirror's glasses of types of bindings") is a
*rhetorical* parallel — both systems exhibit hierarchical composition,
structure-of-structure-of-structure. But the **mechanism** of mirror's
recursion is type-theoretic (parametric types, bounded quantification);
the mechanism of ant-colony recursion is dynamical-systems (state
machines coupled through environment). These are not the same
recursion.

**Net.** Metaphor only. Worse: the most precise computational
framing of ant colonies (Lachmann-Sella's Turing-completeness)
*conflicts* with mirror's commitments. Mirror is sub-Turing on
purpose. The recursive-substructure resonance is poetic but doesn't
carry weight, and pulling it harder leads to a place mirror doesn't
want to go.

---

## Synthesis: does the leap hold structurally?

**Partially.** The associative leap from mirror's tournament-level
Lyapunov to ant colony behavior holds in **two of the six tested
directions**, is **suggestive in two**, and is **metaphor in two**.
The load-bearing alignments are:

- **ACO's convergence theory** (Dorigo & Blum 2005; Stützle-Dorigo
  2002; Gutjahr 2000–2002) IS the closest established prior art for
  mirror's round-level Lyapunov framing. The shape of the proof
  (martingale on the cost; expected pheromone update as contraction;
  per-step ascent permitted; convergence in value with probability 1)
  transfers structurally to mirror's §10.A. Not identity, but family
  resemblance with shared mechanism.
- **Stigmergy in the Heylighen-universal-mechanism sense** IS the
  shape of mirror's substrate-as-environment. The four-part structure
  (agent / medium / trace / stimulation rule) maps directly onto
  mirror's eigenboard + gestalt + tournament-outcome + Reflection.

The suggestive-but-not-load-bearing parallels:

- **Pheromone evaporation as forgetting** is a real design move with
  ACO theoretical backing (Liu et al. 2025 on time-dependent variants),
  but mirror does not currently have this mechanism. Adding it would be
  *new design informed by ACO*, not *recognition of an existing
  parallel*.
- **Phase transitions and small-model claims**: the Gov-Yam et al.
  2025 critical-group-size finding is rhetorically resonant with
  mirror's small-model bet but does not theoretically ground it.
  Empirical grounding for mirror's claim lives in TRM/GRAM.

The metaphor-only directions:

- **Decentralized halting**: the shape is too generic to support
  transfer; mirror's mechanism (fixed-point detection) is unlike
  ant-colony mechanism (physical equilibrium).
- **Recursive substructure**: rhetorically clean but mechanistically
  unrelated; worse, Lachmann-Sella's Turing-completeness result *cuts
  against* mirror's sub-Turing commitment.

**Honest verdict.** The leap is *real but specific*. Naming mirror's
tournament convergence as "ACO-shaped" is accurate and adds the
30-year ACO theoretical literature as load-bearing prior art — the
Dorigo-Blum survey is exactly the kind of reference §10.A needs to
cite. Naming mirror's substrate as "stigmergic" is accurate in the
Heylighen-universal sense and clarifies what the eigenboard's section
history IS doing. The other four leap directions are weaker; some
(decentralized halting; recursive substructure) should be retired or
explicitly downgraded to metaphor before they become load-bearing in
mirror's prose. Generalising "mirror IS the mycelial AI" to "mirror
IS the ant-colony AI" would be a category error — the mycelial framing
is a *different* set of structural commitments (network topology;
belowground continuous healing) which mirror already engages with
differently (per `docs/research/mycelial-networks-and-au-tissue.md`).

---

## What this gives mirror's specific design questions

### §10.F.4 Tournament completeness

The three candidate formalizations currently named are:

(a) A*-style admissibility (Hart-Nilsson-Raphael 1968)  
(b) depth-d exhaustiveness  
(c) anytime/B&B-PTAS (Hendrich et al. 2025)

**ACO suggests a fourth candidate: stochastic-gradient completeness.**

Following the Dorigo-Blum §4 result that ACO is a stochastic
gradient ascent on the expected quality functional WF(τ) = Σ F(s)
p(s|τ), one could formalize tournament completeness as: **the
tournament's transition kernel induces a stochastic-gradient step on
an explicit expected-holonomy functional, with bounded gradient (the
lower-bound condition §10.F.3 supplies)**. The completeness guarantee
becomes: if the expected gradient is non-zero at T, the tournament
produces an improving composition with probability at least δ > 0
per round.

This would be **structurally different** from the three current
candidates:

- A*-admissibility is *deterministic* completeness (the search finds
  the optimum if it exists).
- Depth-d exhaustiveness is *combinatorial* completeness (the search
  enumerates).
- B&B-PTAS is *deterministic approximation* completeness.
- **Stochastic-gradient completeness** (proposed) is *probabilistic*
  completeness: with positive probability per round, you descend.
  Convergence-in-value follows by Borel-Cantelli; convergence-in-solution
  follows under the strict-lower-bound regime.

This is a real candidate that should be evaluated against the three
current ones. It would commit `@fate` to maintaining an *explicit
gradient* on the expected-holonomy functional, which is plausible
given the connectome / multi-trajectory structure.

**Flag for §10 follow-up:** add `(d) stochastic-gradient completeness
(Dorigo-Blum §4; Zlochin et al. 2004; martingale process)` as a
fourth candidate formalization in §10.F.4.

### §10.F.5 Approximation ratio

The current framing cites the FPTAS knapsack literature and the
TRM/GRAM empirical anchors. **ACO contributes one direct
approximation-ratio result and one structural pattern**:

- **The Wikipedia/MMAS* result on minimum-label spanning tree.**
  Wang et al. 2025 (Springer s42452-025-06809-5) prove that **1-ANT
  MMAS* achieves a (r+1)/2 approximation ratio for MLST_r in expected
  polynomial time**, where r is the maximum frequency of label
  occurrences. This is a *direct* ACO approximation-ratio result on an
  NP-complete problem. The pattern: a *very simple* ACO variant
  (1 ant, max-min pheromone) achieves a polynomial-time approximation
  guarantee, outperforming local search on constructed instances.
  Mirror's tournament-as-1-ANT-MMAS* is a plausible reduction target.
- **Structural pattern from arxiv:2501.10810 (Liu et al. 2025):**
  *time-dependent lower bounds* (τ_min(t)) give polynomial running
  time on the single-destination shortest-path problem;
  *time-dependent evaporation* gives super-polynomial. This is
  a sharp design guidance for any tournament that wants polynomial
  bounded-budget approximation: vary the *floor*, not the *decay rate*.

**Flag for §10 follow-up:** §10.F.5 could acquire:
- Wang et al. 2025 (1-ANT MMAS* MLST (r+1)/2 approximation,
  springer s42452-025-06809-5) as a third empirical anchor alongside
  TRM/GRAM.
- Liu et al. 2025 (arxiv:2501.10810) as guidance: prefer
  time-dependent lower-bound regimes over time-dependent evaporation
  for polynomial bounded-budget convergence.

### Pheromone evaporation as kintsugi forgetting

Kintsugi today does *not* implement forgetting. 
The `kintsugi-fracture-confidence-and-scene-dispatch.md` spec carries
confidence scores, but the scores are not time-decayed. If kintsugi
is to absorb a forgetting mechanism, it would be:

- **Influence-weight decay**, not deletion. Old gestalt entries /
  section states keep their content-addressed presence but lose
  influence weight in Reflection's observation function and in
  `@cogito.strategy`'s recurrence detection.
- **Decay schedule informed by Liu et al. 2025**: prefer a
  time-dependent *lower bound* on confidence (τ_min(t) = d/log(t+1))
  over a time-dependent decay rate. The former is
  convergence-time-friendly; the latter is convergence-time-hostile.
- **Fracture confidence as the natural site.** The existing
  confidence-dispatch grammar is the natural location; adding
  `time_since_last_apply` to the confidence input is a one-grammar
  change.

This is a genuine new design move. It is *not* already covered by
kintsugi's existing confidence-dispatch. It should be staged as a
future spec, not folded into existing ones.

---

## Open design implications

1. **Cite ACO in §10 of `gap-tension-tensor-substrate.md`.** The
   Dorigo & Blum 2005 survey, Stützle-Dorigo 2002 convergence proof,
   and (optionally) Wang et al. 2025 MLST result are load-bearing prior
   art for mirror's tournament-level Lyapunov framing. Adding them
   strengthens the citation chain. Specifically: §10.A should add the
   Dorigo-Blum 2005 survey and the
   stochastic-gradient/martingale convergence framing to its "how the
   proof would go" exposition.

2. **Add `(d) stochastic-gradient completeness` to §10.F.4.** As above.
   This is a fourth candidate formalization that ACO theory directly
   supplies, structurally different from the three current candidates,
   and worth Alex's design-call attention.

3. **Reframe the eigenboard's substrate role explicitly as stigmergic
   (Heylighen sense, not biological).** This is rhetorical/exposition
   work, not implementation. The clarity buy is: subsequent specs can
   reference "stigmergic substrate" with a citation to Heylighen 2007
   / 2016, instead of relying on the ambient mycelial-AI framing.
   This is *not* claiming the substrate is biological; it's claiming
   it fits a well-defined coordination-mechanism category.

4. **Stage a "kintsugi forgetting" spec for future tick.** Not now.
   The design move is real, ACO-informed, but unimplemented. Stage as
   `mirror/docs/specs/kintsugi-influence-decay.md` for a future cycle,
   citing Liu et al. 2025 for the time-dependent-lower-bound guidance.

---

## Provenance

- Alex's associative leap, 2026-05-26: "ant colonies and how they
  connect to mirror's current design questions." Specifically: do
  ACO/stigmergy/pheromone-evaporation/phase-transition/decentralized-halting/recursive-substructure
  hold as structural parallels?
- Reed: structured the six tests, ran kagi searches, read the
  Dorigo-Blum 2005 survey via mcp__pdf-reader (pages 1–22), summarised
  six recent papers via kagi summariser, returned with this synthesis.
- Discipline: per Alex's recent correction on narrative-vs-mechanism
  inflation, each section is marked Established/Aligned-but-different/Metaphor.
  Section 6 (recursive substructure) honestly flags that the
  Lachmann-Sella result *cuts against* mirror's sub-Turing commitment
  rather than supporting the leap.
- Cross-reference: `mirror/docs/research/mycelial-networks-and-au-tissue.md`
  is the prior research thread on bio-inspired network framings; this
  document is the ant-colony-specific complement, not a replacement.

---

## Citations

### ACO theory and convergence

- **Dorigo, M., Blum, C. (2005). *Ant colony optimization theory: A
  survey.*** *Theoretical Computer Science* 344(2–3):243–278.
  https://iridia.ulb.ac.be/~mdorigo/Published_papers/All_Dorigo_papers/DorBlu2005tcs.pdf.
  The canonical convergence-theory survey. Theorems 1–3
  (convergence in value and in solution); §4's reduction to stochastic
  gradient ascent and cross-entropy method. **The single most
  load-bearing reference for mirror's tournament-level Lyapunov.**

- **Stützle, T., Dorigo, M. (2002). *A short convergence proof for a
  class of ant colony optimization algorithms.*** *IEEE Transactions
  on Evolutionary Computation* 6(4):358–365.
  https://www.semanticscholar.org/paper/95b59b38262ac2312012dbc3a487012df97365c6.
  The short, clean form of the convergence-in-value theorem for
  ACO_{bs,τ_min}.

- **Gutjahr, W. J. (2000, 2002). *A graph-based ant system and its
  convergence.*** *Future Generation Computer Systems* 16(8):873–888;
  *Mathematical and Computer Modelling* 35(7–8). The first
  convergence-in-solution proofs (Theorem 4, 5 in Dorigo-Blum 2005).

- **Liu, F., et al. (2025). *Convergence and Running Time of
  Time-dependent Ant Colony Algorithms.*** arXiv:2501.10810.
  https://arxiv.org/pdf/2501.10810. GBAS/tdev (time-dependent
  evaporation) has super-polynomial running time on SDSP; GBAS/tdlb
  (time-dependent lower bound) achieves polynomial. **The sharp
  design guidance for mirror's bounded-budget regime: prefer
  lower-bound decay over rate decay.**

- **Wang, X., et al. (2025). *Performance of a simple ACO on the
  minimum label spanning tree problem.*** *Discover Applied Sciences*
  (Springer) s42452-025-06809-5. 1-ANT MMAS* achieves a (r+1)/2
  approximation ratio for MLST_r in expected polynomial time. **Direct
  approximation-ratio result transferable to §10.F.5.**

- **Wuhang Lin, et al. (n.d.). *The martingale process of ant colony
  optimization algorithms and its convergence analysis.***
  researchgate/287500035. Martingale-theoretic convergence framing;
  cost sequence as non-negative submartingale; finite-a.s. stopping time.

- **Zlochin, M., Birattari, M., Meuleau, N., Dorigo, M. (2004).
  *Model-based search for combinatorial optimization: A critical
  survey.*** *Annals of Operations Research* 131:373–395. The unifying
  framework that places ACO, SGA, and CE under one umbrella.

### Stigmergy

- **Grassé, P.-P. (1959). *La reconstruction du nid et les
  coordinations interindividuelles chez Bellicositermes natalensis
  et Cubitermes sp. La théorie de la stigmergie.*** *Insectes Sociaux*
  6:41–80. The original definition. Pre-internet; primary citation.

- **Theraulaz, G., Bonabeau, E. (1999). *A Brief History of
  Stigmergy.*** *Artificial Life* 5(2):97–116.
  https://static.ias.edu/pitp/archive/2012files/29.pdf. The canonical
  modern reformulation. Quantitative vs qualitative stigmergy;
  multistability and bifurcations in pheromone dynamics.

- **Heylighen, F. (2016). *Stigmergy as a universal coordination
  mechanism I: Definition and components.*** *Cognitive Systems
  Research* 38:4–13.
  https://www.sciencedirect.com/science/article/abs/pii/S1389041715000327.
  The four-part formalisation: agent / medium / trace / stimulation
  rule. The reading under which mirror's substrate IS stigmergic.

- **Heylighen, F. (2007). *Why is open access development so successful?
  Stigmergic organization and the economics of information.***
  pespmc1.vub.ac.be/Papers/Stigmergy-Springer.pdf. The Wikipedia /
  open-source generalization.

- **Anonymous (April 2026). *Ledger-State Stigmergy: A Formal
  Framework for Indirect Coordination Grounded in Distributed Ledger
  State.*** arXiv:2604.03997. https://arxiv.org/abs/2604.03997.
  Critically: identifies the **biological-pheromones-evaporate /
  ledger-traces-persist** asymmetry. Mirror is on the ledger side.

### Phase transitions and critical-group-size

- **Gov-Yam, et al. (2025). *Maximal response to a mechanical leader
  at critical group size in ant collectives.*** *Nature Communications*
  16, article s41467-025-61158-6.
  https://www.nature.com/articles/s41467-025-61158-6;
  arXiv:2506.01209. **The strongest recent evidence for ant-colony
  phase transitions and critical-regime susceptibility.** Suggestive
  for mirror's small-model claim but does not theoretically ground it.

- **Gelblum, A., et al. (2022). *The emergence of a collective sensory
  response threshold in ant colonies.*** *PNAS* 119(24):e2123076119.
  Group-size–dependent evacuation threshold.

- **Li, L., et al. (2014). *Chaos–order transition in foraging
  behavior of ants.*** *PNAS*.
  https://www.pik-potsdam.de/members/kurths/publikationen/2014/Li_etal_Kurths_2014_PNAS2014Li83927.pdf.
  Foraging dynamics scale through chaos–order transition.

### Recursive substructure (Turing completeness)

- **Lachmann, M., Sella, G. (1995). *The Computationally Complete Ant
  Colony: Global Coordination in a Matter-Free, Identity-Free System.***
  Santa Fe Institute reprint.
  https://sites.santafe.edu/~lachmann/publications/Lachmann_Sella_1995_The%20computationally%20complete%20ant%20colony.pdf.
  **The result that cuts against mirror's sub-Turing commitment.**
  Mirror is sub-Turing on purpose; ant colonies (in this model) are
  Turing-complete.

### Software-architecture stigmergy (context)

- **stigmergy (rescrv/stigmergy on GitHub).** An entity-component-system
  architecture inspired by emergent biological coordination, with
  capability-based security and an auction-style coordination
  mechanism. Useful contrast against mirror's grammar-typed approach.

---

*Apache-2.0.*
