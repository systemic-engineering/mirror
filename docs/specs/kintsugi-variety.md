# kintsugi-variety — @io crossing minimization as the formal kintsugi objective

*2026-06-02. Reed + Alex. Status: load-bearing recognition; spec.*

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

When @mirror computation has accumulated Fisher information `I(t)` after `t` steps, the minimum achievable error at @io crossing is bounded by:

```
δ_min(t)  ≥  1 / √I(t)
```

This is the Cramér–Rao bound. It tells us that **delaying @io crossing reduces the minimum committable error** — not as a heuristic, but as a theorem. Kintsugi's bias is precisely the policy that delays crossing until `I(t)` is large enough that `δ_min(t)` is below an acceptable threshold.

The Fisher-Rao gradient flow machinery from Kerimkulov et al. (2023) IS the @fate inference update rule on the probability manifold. The natural gradient on the Fisher manifold is the mathematically-correct way to run @mirror computation steps in a way that maximally narrows the posterior per unit of compute.

See [[../../../systemic.engineering/practice/insights/math/numerics/information-geometry-variety-preserving]].

---

## 4. The Knapsack framing — and that the bound is achievable

Items = operations in the computation graph. Weight = @io crossing cost. Value = variety maintained. Kintsugi packs the @mirror bag as tightly as possible; lifting `@code → @mirror` moves items from the "must cross" pile to the "stays in bag" pile.

This framing is **structurally identical** to the I/O-optimal computation problem studied by Saha & Ye (ICML 2024). They proved that FlashAttention's I/O complexity is *optimal* and established the reduction from communication complexity to I/O complexity for transformer attention. The implication for mirror:

- The Knapsack lower bound on @io crossings is *known to be achievable* for the class of computations mirror runs.
- The red-blue pebble game machinery (Sobczyk 2024, partial-computation extension) gives us a concrete algorithmic framework for computing the optimal crossing schedule on a DAG.
- The SP-DAG memory-peak minimization (Herrmann et al. 2025) gives us the algorithm for the special case where the op graph is series-parallel.

See [[../../../systemic.engineering/practice/insights/math/numerics/io-complexity-computation-graphs]].

---

## 5. The compounding non-linearity

A naive additive model treats N crossings as `N · δ`. This is wrong. Each crossing destroys posterior branches; the next crossing starts from an impoverished posterior, not the original prior. The recurrence is:

```
posterior_i+1 = collapse(posterior_i, output_i)
δ_{i+1}      = f(residual_variance(posterior_{i+1}))  with f monotone in variance
```

Since `residual_variance(posterior_{i+1}) > residual_variance(posterior_i ∩ alt_branches)`, we get `δ_{i+1} > δ_i`. **Error accumulates super-linearly.**

The right penalty structure for the kintsugi objective is therefore not `∑ δ_i` but something like `∑ δ_i · e^{i · κ}` for some decoherence constant `κ` — or, more precisely, the Saha & Ye communication-complexity formulation, which already captures the sequential-dependence structure correctly.

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

## 7. @fate tournament IS the variety-maintenance mechanism

The [[kintsugi-tournament]] machinery is not just "explore more interpretations." It is **structurally the requisite-variety apparatus**. Multiple agents running in parallel on the same glass = multiple posterior branches held simultaneously. The tournament does not collapse until the HAL line forces it to. The tournament's depth, width, and selection rule are tunable parameters of the variety-maintenance policy.

This recontextualizes [[parse-as-fate-tournament]] and [[substrate-native-fate-tournament]]: the tournament is the mechanism by which mirror satisfies the Ashby constraint. Without the tournament, |R(@mirror)| collapses to 1 (a single interpretation) and Ashby fails for any ambiguous disturbance.

The tournament's loss function is the kintsugi objective from §1. The natural gradient on the Fisher-Rao manifold is the tournament's update rule. The HAL line is where @io crossing becomes mandatory.

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

1. **The decoherence constant `κ`.** The compounding penalty `e^{i·κ}` needs an empirical anchor. The Saha & Ye communication-complexity formulation may already give it implicitly; we need to translate.

2. **Per-operation vs. per-glass `variety_hold` resolution.** The chain discipline in [[properties-on-glass]] §2.3 suggests we wait until a concrete operation surfaces that needs the finer granularity. For now: per-operation in the type signature, but the property is declared at the glass altitude and cascades to its operations.

3. **Tournament depth as a function of disturbance variety.** Ashby says `|R| ≥ |D|`. The tournament's branching factor must scale with the input's ambiguity. We don't yet have a measurement of |D| at parse time. The Siegenfeld & Bar-Yam complexity profile gives the formal answer; we need the implementation.

4. **Interaction with [[reality-shard-as-crdt]].** The CRDT layer is the bounded semilattice of shards. Kintsugi merging on shards is also a variety-preservation operation: two informed shards merged together preserve more variety than each alone. The kintsugi objective on the shard substrate is the join operation. This needs to be made precise.

5. **The 16 = 12 gauge + 4 Higgs decomposition.** The noncommutative-geometry numerics work (Wulkenhaar / Dąbrowski 2023 / Khalkhali 2025) suggests that the flang 16-channel structure has a principled decomposition into gauge + Higgs channels. If true, the gauge channels are the variety-preserving (mirror-side) channels and the Higgs channels are the variety-collapsing (@io-side) channels. This would give a structural interpretation of the [[architecture-flang-mirror-numerical-split]].

   See [[../../../systemic.engineering/practice/insights/math/numerics/noncommutative-geometry-standard-model]].

---

## 11. Prior art and references

Numerics sweep, 2026-06-02:

| Cluster | Reference | Role |
|---|---|---|
| Requisite variety | Siegenfeld & Bar-Yam (2022) [[../../../systemic.engineering/practice/insights/math/numerics/requisite-variety-optimization]] | Multi-scale Ashby, computable `C(s)` |
| Information geometry | Kerimkulov et al. (2023) [[../../../systemic.engineering/practice/insights/math/numerics/information-geometry-variety-preserving]] | Fisher-Rao gradient flows = @fate update rule |
| Spectral dimension | Villegas et al. (Nature Physics 2022) [[../../../systemic.engineering/practice/insights/math/numerics/spectral-graph-laplacian-dimension]] | LRG; numerical `d_s` |
| I/O complexity | Saha & Ye (ICML 2024) [[../../../systemic.engineering/practice/insights/math/numerics/io-complexity-computation-graphs]] | Knapsack bound is achievable |
| Lazy evaluation | Xia et al. (ICFP 2024) [[../../../systemic.engineering/practice/insights/math/numerics/lazy-evaluation-deferred-computation]] | Operational deferred semantics |
| Noncommutative geometry | Dąbrowski et al. (2023), Khalkhali et al. (2025) [[../../../systemic.engineering/practice/insights/math/numerics/noncommutative-geometry-standard-model]] | 16-channel structural interpretation |

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
- Memory: `architecture-kintsugi-bias-lift` — the prior framing this supersedes (lift @code → @mirror is now a *consequence*, not a primitive bias).

---

## 12. What this spec changes

- The kintsugi bias is no longer "prefer @mirror." It is the minimum-variety-loss policy on the op graph. "Prefer @mirror" falls out as a consequence.
- `variety_hold` becomes a first-class property in `@epistemologic/properties`. The verdict feeds the @fate loss composite directly. No invented loss term — it composes from declared properties per the [[../feedback-loss-from-epistemologic-properties]] discipline.
- The @fate tournament's structural role is named: it IS Ashby's requisite-variety apparatus. Without the tournament, mirror cannot handle ambiguous inputs at all.
- The numerics research is no longer background reading. The spec cites it directly: Saha-Ye gives the achievability of the Knapsack bound, Kerimkulov gives the gradient update rule, Villegas gives the `d_s(σ)` decay curve. These are the policy.

## 13. Where the verdict flows on the wire

The variety verdict computed per §6 lives on every pq response
at the wire altitude. Per [[../../../prism/docs/specs/pq]] §2.4,
the Beam returned from `focus`/`project`/`refract` carries an
`imperfect` field of shape `Success | Partial | Failure`. The
Partial verdict's loss term IS `variety_loss(op)` for the op that
produced the Beam; the kintsugi objective
(`∑ variety_loss(op) · crossing_cost(op)`) accumulates monotonically
as pq chains compose.

**This is why pq is sub-Turing AND variety-aware at the wire.** The
three-op algebra is closed (per pq §9), and each composition step
carries the variety verdict through the `imperfect` channel into
`@epistemologic/properties`. The agent reading the response sees
the kintsugi objective's running cost; the substrate sees the same
verdict in its property chain. The wire altitude does not invent
the objective; it carries it.

When the kintsugi loop (per [[kintsugi-tournament]]) iterates above
pq, it invokes pq chains as primitives and reads back the
`imperfect` field as the per-iteration variety loss. The loop's
convergence is lattice ascent on the @mirror side; the wire
provides the per-step measurement.

---

*“Kintsugi minimizes how often we have to commit.” — the architecture, finally said in eleven words.*
