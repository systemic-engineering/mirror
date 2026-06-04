# kintsugi-variety — @io crossing minimization as the formal kintsugi objective

*2026-06-02. Reed + Alex. Status: load-bearing recognition; spec.*

*§3 and §4 tightened 2026-06-02. Reed + Alex + you. Status: load-bearing recognition — the precise correspondence per [[../../../prism/docs/specs/pq]] §6.5.*

Kintsugi's bias is not a heuristic. It is the **minimum-variety-loss policy** on a computation graph whose crossings of the @io boundary are decoherence events. The objective is computable. Each piece of it now has prior art that turns the architecture from a design intuition into something grounded in current numerics literature.

This spec formalizes the objective, names the implementation seam (the `variety_hold` property), connects it to [[kintsugi-tournament]] and [[parse-as-fate-tournament]], and records the four load-bearing references from the 2026-06-02 numerics sweep.

---

## 1. The objective in one sentence

Kintsugi minimizes the expected committed error delta across the @io crossings of a computation graph, subject to maintaining requisite variety in @mirror through the @fate tournament.

```
min   E[ ∑_i δ_i ]
over  the placement of @io crossings in the op graph
s.t.  |R(@mirror)| ≥ |D|  (Ashby)
      δ_i ∝ √(residual posterior variance at crossing i)  (Cramér–Rao)
      δ_{i+1} > δ_i        (compounding non-linearity — see §5)
```

Where:
- `δ_i` = committed error delta at the i-th @io crossing.
- `R(@mirror)` = variety (posterior support) maintained inside @mirror.
- `D` = variety of disturbance / input ambiguity space.
- The constraint is Ashby's law of requisite variety, formalized at multiple scales per [[../../../systemic.engineering/practice/insights/math/numerics/requisite-variety-optimization]].

---

## 2. Ashby grounding — variety as posterior support

In @mirror, a computation holds a posterior distribution over interpretations. The variety |R| is the support of that posterior — the number of distinguishable states the system can still be in. Each @mirror computation step is a Bayesian update: it narrows the support. @io crossing is the MAP-step: the system is forced to commit to a point estimate from whatever posterior it holds at that moment.

**Multi-scale variety, not single-scalar variety.** The Siegenfeld & Bar-Yam (2022) formalization gives us a complexity profile `C(s)` over scales `s`, not a single number. This matters for mirror because the variety required at the lexer altitude is not the variety required at the resolver altitude or the @fate altitude. Each tier of the [[scheduler-tower]] has its own `C(s)` requirement.

See [[../../../systemic.engineering/practice/insights/math/numerics/requisite-variety-optimization]] for the formal definition and computability result.

---

## 3. The error delta at crossing time — Cramér–Rao

At each @io crossing, an agent must commit to a point estimate from the @mirror posterior. We pin the four hooks the Cramér–Rao bound needs:

- **What is estimated.** The committed value at the @io boundary — the agent's collapse from posterior to a single interpretation.
- **Which estimator.** The MAP value of the @mirror posterior at the moment of commitment.
- **Bias.** The MAP is biased in general; we use the biased Cramér–Rao form.
- **Error metric.** `δ` is the standard deviation of the MAP estimator under the posterior.

Under these hooks, the per-crossing bound is:

```
δ_min²(t)  ≥  (∂_θ E[T])² / I(t)
```

where `T` is the MAP estimator, `I(t)` is the Fisher information accumulated through `t` @mirror computation steps, and `∂_θ E[T]` is the slope of the estimator's expected value. The asymptotic proportionality `δ ∝ 1/√I(t)` in §1's constraint absorbs the slope factor into a per-crossing constant. The formal bound is per-crossing; §1's constraint is the decay rate.

The implication: **delaying @io crossing reduces the minimum committable error** — not as heuristic, but as a theorem. Kintsugi's bias is precisely the policy that defers crossing until `I(t)` is large enough that `δ_min(t)` is below an acceptable threshold.

The Fisher-Rao gradient flow machinery from Kerimkulov et al. (2023) is the @fate inference update rule. The natural gradient on the Fisher manifold is the geometrically-correct way to accumulate `I(t)` per unit of @mirror compute, and the precise correspondence is named in [[../../../prism/docs/specs/pq]] §6.5.4: **the Kerimkulov entropy-regularised policy mirror descent flow IS the `project({kintsugi})` Banach iteration `M_{n+1} = T(M_n)`** on the shard matrix held by the canonical `LAPACKPrism` impl. The substrate altitude reads off the bound at §6.5.5: `Beam.imperfect.loss` IS the per-iteration residual posterior variance — actual numbers, not metaphor. The remaining open question (§10) narrows to whether @fate's *tournament* dynamics above the per-iteration flow recover Kerimkulov's global-convergence guarantees end-to-end.

See [[../../../systemic.engineering/practice/insights/math/numerics/information-geometry-variety-preserving]].

---

## 4. The Knapsack framing — and that the bound is achievable

Items = operations in the computation graph. Weight = @io crossing cost. Value = variety maintained. Kintsugi packs the @mirror bag as tightly as possible; lifting `@code → @mirror` moves items from the "must cross" pile to the "stays in bag" pile.

This framing connects to the I/O-optimal computation problem studied by Saha & Ye (ICML 2024). They prove a tight I/O lower bound for transformer attention specifically (`IO(Attention) = Θ(N²d/√M)`) and — more importantly for our purposes — establish a *reduction technique* from communication complexity to I/O complexity that is applicable to any computation graph. With LAPACKPrism named as the canonical Prism impl (see [[../../../prism/docs/specs/pq]] §6.5.3), **the Saha–Ye bound applies to LAPACKPrism's memory traffic literally, not metaphorically**: the shard matrix lives partly in HamiltonScheduler-governed fast memory and partly in `.frgmnt/` disk spillover; the pebble model is the cost model for pq chains. The implications for mirror sharpen:

- The Knapsack lower bound on @io crossings is **known to be tight for attention** and **applied to pq chains via the Saha–Ye reduction technique against LAPACKPrism's row/column-select / projection / rank-1-update operations** (per the operation table in [[../../../prism/docs/specs/pq]] §6.5.2). Whether mirror's compiled op graphs hit the bound is now a substrate-altitude measurement, not a philosophical open question.
- The red-blue pebble game machinery (Sobczyk 2024, partial-computation extension) gives the formal cost model for the pq operation table: `focus` loads a row/column (red→blue if cold), `project` keeps red pebbles (in-cache LAPACK ops), `settle` commits red→blue (the rank-1 update materialises to durable storage). Partial computations are exactly the `Beam.imperfect = Partial` verdicts.
- The SP-DAG memory-peak minimization (Herrmann et al. 2025) gives a polynomial-time algorithm when the op graph is series-parallel. Pq chains under the [[../../../prism/docs/specs/pq]] §9 sub-Turing closure are SP by construction (no fixed point at the wire altitude); the polynomial result applies.

See [[../../../systemic.engineering/practice/insights/math/numerics/io-complexity-computation-graphs]].

---

## 5. The compounding non-linearity

A naive additive model treats N crossings as `N · δ`. This is wrong. Each crossing destroys posterior branches; the next crossing starts from an impoverished posterior, not the original prior. The recurrence is:

```
posterior_i+1 = collapse(posterior_i, output_i)
δ_{i+1}      = f(residual_variance(posterior_{i+1}))  with f monotone in variance
```

The claim is that `residual_variance(posterior_{i+1}) > residual_variance(posterior_i ∩ alt_branches)`, so `δ_{i+1} > δ_i`. **Error accumulates super-linearly.**

The right penalty structure for the kintsugi objective is therefore not `∑ δ_i` but something like `∑ δ_i · e^{i · κ}` for some decoherence constant `κ`.

**Regime caveat.** §3 describes the residual variance under continued @mirror computation — where `I(t)` accumulates and `δ_min(t)` decreases. §5 here describes the residual variance after the @io collapse step — where the posterior is replaced by a degenerate distribution at the committed MAP and subsequent computation starts from a biased prior. These are different regimes; the compounding claim above is heuristic under this informal framing. A rigorous bias-propagation model that closes the gap is §10's open question (see §10.5).

---

## 6. Implementation seam — the `variety_hold` property

The objective is encoded as a property on glass-operation pairs per [[properties-on-glass]]:

```mirror
in @prism
in @epistemologic
in @io

# A scalar in [0.0, 1.0] declaring how much variety this op preserves.
# 1.0 = stays in @mirror (Bayesian update, no commitment).
# 0.0 = forces @io crossing (full commitment to a point estimate).
property variety_hold : scalar in [0.0, 1.0]
  for op in glass
  = inferred via @fate

# Verdict: the loss this op contributes to the kintsugi objective.
verdict variety_loss = (1.0 - variety_hold)

# The composed objective lives in @epistemologic/properties,
# matching the discipline from [[../feedback-loss-from-epistemologic-properties]].
property kintsugi_objective : scalar
  for graph in computation
  = ∑ variety_loss(op) · crossing_cost(op) over op_graph
      # subject to the compounding non-linearity from §5
```

**Granularity.** `variety_hold` lives on the *glass-operation pair*, not on glasses alone. The same glass may have operations that preserve variety (a lens read) and operations that collapse it (an @io commit). The property is declared per-operation; the verdict cascades through `/` per [[properties-on-glass]] §2.

**Inference source.** `variety_hold` is not user-declared. It's @fate-inferred from the operation's type signature: operations whose codomain crosses the @io boundary get `0.0`; operations that stay within the @mirror substrate get `1.0`; operations whose codomain is partially-committed get a value in `(0.0, 1.0)`. The natural-gradient update from Kerimkulov et al. §3 gives the inference rule.

---

## 7. @fate tournament as a variety-maintenance instance

The [[kintsugi-tournament]] machinery is scoped to one specific application: tournament-shaped resolution between competing glass morphisms at the migrate altitude (the runtime.mirror / nl.mirror duplicates in Tick 4b.3). What it does in that instance is hold multiple posterior branches in parallel and defer collapse until the HAL line forces it.

The pattern itself — tournament-shaped structure holding multiple posterior branches and deferring collapse — generalises across mirror's altitudes. [[parse-as-fate-tournament]] and [[substrate-native-fate-tournament]] are sibling instantiations: the same pattern at parse altitude and substrate altitude respectively. The conjecture this spec defends as an open question (§10.6) is that tournament-shaped machinery is mirror's general requisite-variety apparatus — not just one tool among many. The conjecture is plausible; the proof is open.

For each instance: the tournament's loss function is the kintsugi objective from §1. The natural gradient on the Fisher-Rao manifold is the tournament's update rule. The HAL line is where @io crossing becomes mandatory.

---

## 8. Spectral grounding — d_s(σ) gives the decay rate

The rate at which `I(t)` accumulates depends on how fast information propagates on the underlying graph. The spectral dimension `d_s(σ)` characterizes this: it's the scaling of `I(t) ∼ t^{d_s/2}`. So the optimal crossing point has a closed form when `d_s` is known.

The Laplacian Renormalization Group (Villegas et al., Nature Physics 2022) computes `d_s` numerically on real graphs. The simplicial-complex extension (Marié-Tena et al. 2024) extends this to the higher-order structure the [[eigenboard-representation]] sheaf operates over. These give us the *actual numerical value* of the `δ_min(t)` decay curve, not just its functional form.

This is why the numerics research is load-bearing: it's not background reading. It determines the policy.

See [[../../../systemic.engineering/practice/insights/math/numerics/spectral-graph-laplacian-dimension]] and the upstream [[cosmos-mirror-scaffold]] for the d_s(σ) experiment.

---

## 9. Deferred computation as the operational discipline

The Xia et al. (ICFP 2024) bidirectional demand semantics for lazy cost analysis gives the formal theory for the operational side. Lazification (Fernández-Reyes et al. CC 2023) provides the automatic-transformation algorithm: take a strict computation, rewrite it to defer evaluation as long as possible. **This is the operational version of the kintsugi bias.** Substrate-pull at the evaluation-order layer.

See [[../../../systemic.engineering/practice/insights/math/numerics/lazy-evaluation-deferred-computation]].

---

## 10. Open questions

1. **The decoherence constant `κ`.** The compounding penalty `e^{i·κ}` needs an empirical anchor. The Saha & Ye communication-complexity formulation may already give it implicitly via the pebble-game cost on LAPACKPrism's operation graph (per [[../../../prism/docs/specs/pq]] §6.5); the open question narrows to *which constant of the bound* `κ` corresponds to.

2. **Per-operation vs. per-glass `variety_hold` resolution.** The chain discipline in [[properties-on-glass]] §2.3 suggests we wait until a concrete operation surfaces that needs the finer granularity. For now: per-operation in the type signature, but the property is declared at the glass altitude and cascades to its operations.

3. **Tournament depth as a function of disturbance variety.** Ashby says `|R| ≥ |D|`. The tournament's branching factor must scale with the input's ambiguity. We don't yet have a measurement of |D| at parse time. The Siegenfeld & Bar-Yam complexity profile gives the formal answer; we need the implementation.

4. **Interaction with [[reality-shard-as-crdt]].** The CRDT layer is the bounded semilattice of shards. Kintsugi merging on shards is also a variety-preservation operation: two informed shards merged together preserve more variety than each alone. The kintsugi objective on the shard substrate is the join operation. This needs to be made precise.

5. **Formal bias-propagation model.** §5's compounding claim `δ_{i+1} > δ_i` is heuristic. A rigorous bound on `δ_{i+1}/δ_i` under repeated @io collapses requires a bias-propagation model that tracks how the i-th crossing's MAP commitment biases the (i+1)-th posterior. This is a measurable quantity per [[../../../systemic.engineering/practice/insights/math/numerics/information-geometry-variety-preserving]] — pending instantiation.

6. **Tournament-as-variety-apparatus conjecture.** §7 conjectures that tournament-shaped machinery is mirror's general requisite-variety apparatus across all altitudes. The proof would either (a) exhibit a non-tournament variety-maintenance mechanism in mirror (falsification) or (b) show that all variety-maintenance instances reduce to tournament structure (confirmation). Either result sharpens the architecture.

---

## 11. Prior art and references

Numerics sweep, 2026-06-02:

| Cluster | Reference | Role |
|---|---|---|
| Requisite variety | Siegenfeld & Bar-Yam (2022) [[../../../systemic.engineering/practice/insights/math/numerics/requisite-variety-optimization]] | Multi-scale Ashby, computable `C(s)` |
| Information geometry | Kerimkulov et al. (2023) [[../../../systemic.engineering/practice/insights/math/numerics/information-geometry-variety-preserving]] | Fisher-Rao gradient flows as formal language for @fate update; full identification open |
| Spectral dimension | Villegas et al. (Nature Physics 2022) [[../../../systemic.engineering/practice/insights/math/numerics/spectral-graph-laplacian-dimension]] | LRG; numerical `d_s` |
| I/O complexity | Saha & Ye (ICML 2024) [[../../../systemic.engineering/practice/insights/math/numerics/io-complexity-computation-graphs]] | Reduction technique applicable per-graph; tight for attention; mirror-instance open |
| Lazy evaluation | Xia et al. (ICFP 2024) [[../../../systemic.engineering/practice/insights/math/numerics/lazy-evaluation-deferred-computation]] | Operational deferred semantics |

In-spec dependencies:

- [[kintsugi-tournament]] — the tournament IS the variety-maintenance mechanism.
- [[parse-as-fate-tournament]] — parse-time variety preservation.
- [[substrate-native-fate-tournament]] — substrate-altitude tournament.
- [[properties-on-glass]] — where `variety_hold` is declared.
- [[reality-shard-as-crdt]] — CRDT merge as variety-preservation on shards.
- [[error-as-question]] — @io crossing as answering Reflection's question.
- [[reflection-model]] — Reflection observes the variety and writes morphisms.
- [[scheduler-tower]] — each tier has its own `C(s)` requirement.
- [[eigenboard-representation]] — the sheaf on which `d_s(σ)` is computed.
- [[architecture-flang-mirror-numerical-split]] — the 16/5 decomposition the variety property eventually projects through.
- [[au-and-conductivity]] — `au` is the output type of @fate; conductivity is variety-preservation under transport.
- [[liquid-types-for-mirror]] — `variety_hold` is a liquid type when refined.
- [[io-safety-properties]] — the @io boundary properties that gate crossings.

In-corpus dependencies:

- [[../../../systemic.engineering/practice/insights/coincidence/void-dual-geometry]] — the eight dualities, the void axis where variety bottoms out.
- Memory: `architecture-kintsugi-variety-io` — the synthesis note for this spec.
- Memory: `architecture-kintsugi-bias-shift` — the prior framing this supersedes (shift @code → @mirror is now a *consequence*, not a primitive bias).

---

## 12. What this spec changes

- The kintsugi bias is no longer "prefer @mirror." It is the minimum-variety-loss policy on the op graph. "Prefer @mirror" falls out as a consequence.
- `variety_hold` becomes a first-class property in `@epistemologic/properties`. The verdict feeds the @fate loss composite directly. No invented loss term — it composes from declared properties per the [[../feedback-loss-from-epistemologic-properties]] discipline.
- The @fate tournament's structural role is named: it IS Ashby's requisite-variety apparatus. Without the tournament, mirror cannot handle ambiguous inputs at all.
- The numerics research is no longer background reading. The spec cites it directly: Saha-Ye gives the reduction technique that, instantiated per-graph, would prove Knapsack-achievability; Kerimkulov gives the gradient update rule's formal language; Villegas gives the `d_s(σ)` decay curve. These are the policy.

## 13. Where the verdict flows on the wire

The variety verdict computed per §6 rides every pq response through the `imperfect` channel — see [[../../../prism/docs/specs/pq]] §2.4, §6.5.5, and §9 for the mechanism. The `Beam.imperfect.loss` field carries the Cramér-Rao residual variance as an actual number; the canonical `LAPACKPrism` impl computes it from the rank-1 update's residual norm. The kintsugi loop ([[kintsugi-tournament]]) iterates above pq, invoking chains as primitives and reading back `imperfect` as the per-iteration variety loss. The wire altitude carries the objective; it doesn't invent it.

---

*“Kintsugi minimizes how often we have to commit.” — the architecture, finally said in eleven words.*
