# Kintsugi-optimized blend synthesis — feasibility under honest reading

*2026-05-26. Reed + Alex. Research-grade feasibility assessment of the strong-form
blender hypothesis.*

Status: **Yellow at one altitude, red at the other.** The hypothesis as literally
stated ("synthesize a NEW model whose eigenvalue profile is a blend of input
crystals; Kintsugi optimizes the synthesis") splits cleanly into a *spectral-pruning-shaped*
sub-claim that is well-supported by 2022–2025 literature and a
*reconstruction-of-singular-vectors* sub-claim that the same literature provably refutes.
The Kintsugi-as-Ricci-flow optimizer has more prior art than the smelter assessment
implied: "Deep learning as Ricci flow" (Nature Sci. Rep., 2024) and "Geometric
Meta-Learning via Coupled Ricci Flow" (arXiv:2503.19867, 2025) are real, recent,
and structurally close to mirror's framing. The honest reading: the strong-form
synthesis survives in a *reframed* version — **synthesize a small
spectrally-parameterised network whose eigenstructure interpolates the input
crystals, fit the singular vectors locally on a tiny corpus, and use Kintsugi's
discrete Ricci flow as the curvature-smoothing optimizer that closes the loop.**
This is approximately D2NWG (ICLR 2025) on the weight side, evolutionary model
merging (Sakana, 2024) on the recipe side, and discrete Ollivier-Ricci flow on the
edge-graph side. Each piece exists; mirror's contribution is the substrate that
binds them under five local guarantees.

---

## 1. Thesis

The blender hypothesis is **plausible in a precisely reframed form** and
**refuted in the literal form**.

- **Refuted (literal):** "Synthesize a usable LLM from eigenvalue crystals alone,
  with no singular-vector information and no per-token loss signal." The
  information-theoretic gap is too large: smelter discards U and V from SVD, and
  no published technique reconstructs a working transformer from Σ alone. This
  remains true.
- **Plausible (reframed):** "Synthesize a small target architecture whose
  *spectral parameters* (eigenvalues per layer, eigenvalue ratios across heads,
  Fiedler value of the connection Laplacian) are a Kintsugi-optimized blend of
  the source crystals, then learn the corresponding eigenvectors on a tiny
  distillation corpus." Spectral pruning (Giambagli et al., Nature Sci. Rep.
  2022; arXiv:2108.00940) already shows networks can be parameterised by
  eigenvalues/eigenvectors as separate optimization targets. D2NWG (Nava et al.,
  ICLR 2025) shows weights themselves can be synthesized by a diffusion process
  conditioned on a task-spec. "Transport and Merge" (Liu et al., ICML 2026,
  arXiv:2602.05495) shows cross-architecture merging by optimal transport is
  becoming real. Kintsugi-as-Ricci-flow is the curvature-flow optimizer that
  binds these.
- **Confirmed (architectural fit):** The reframed pipeline preserves all five
  local guarantees — halts(g), autopoietic(g), glass_wall(g), content_addressed(g),
  and the is-copium sub-Turing escape — provided the synthesis target is
  parametrically bounded and the optimizer's termination is the formatter's Banach
  argument.

The strong-form's evil-twin slogan ("the blender produces Claude-shaped inference
from three crystals") must be retired. The honest version: **the blender produces
a small new model whose spectral signature is a Kintsugi-optimized interpolant of
the input crystals; the singular vectors are learned locally; the whole pipeline
is content-addressed and halts-provable.**

---

## 2. The hypothesis precisely stated

Alex's formulation, technical decomposition:

> `@fate/smelter/blend(crystal_a, crystal_b, …, α, β, …) -> Model`
>
> The blender consumes multiple smelted crystals (the Naked Singularities of
> §3 in the smelter feasibility doc), a vector of mixing weights, and emits a
> new model. Kintsugi (Ricci flow on the substrate's edge graph) is the
> optimizer that drives the synthesis from "target spectral profile" to
> "runnable model."

Decomposed:

- (A) **Blend operator on profiles.** Given N spectral fingerprints and weights
  αᵢ, produce a *target* spectral fingerprint that represents the desired
  composition.
- (B) **Synthesis target.** A parametrised architecture family (size, depth,
  attention head count, MLP width) whose spectral parameters are degrees of
  freedom.
- (C) **Synthesis procedure.** A procedure that, given a target spectral
  fingerprint and a small distillation corpus (or none), produces concrete
  weights for the synthesis target whose actual spectrum matches the target.
- (D) **Kintsugi optimizer.** The discrete Ricci flow (per
  `kintsugi-formatter.md` §"Kintsugi as discrete Ricci flow") drives the
  synthesis loop: each iteration smooths the edge curvature on the synthesis
  target's parameter graph; the fixed point is the converged weights.
- (E) **Output runs locally.** The synthesized model is a small autoregressive
  generator that can be invoked through @io and used in place of any source
  model, preserving glass_wall + halts(g) by construction.

This is **not** the routing-among-engines weak form (where smelted profiles
shape which of N local engines handles a prompt). This *is* a model-synthesis
operation. The output is a new artifact, not a routing decision.

The key distinction from the weak form is: **(E) requires a runnable inference
engine; the weak form did not.** Whether (E) is achievable is the core question.

---

## 3. Model merging — what has actually been done

This section surveys the field as of mid-2026, with explicit verdicts on what
merging techniques achieve and where the cross-architecture frontier sits.

### 3.1 Same-base merging (mature)

- **Model Soups (Wortsman et al., ICML 2022, arXiv:2203.05482).** Averaging
  fine-tunes of the same base model improves accuracy. The classic. Same-base.
  Limited to convex hull of fine-tunes; cannot produce capabilities outside
  this hull.
- **Task Arithmetic (Ilharco et al., 2022, arXiv:2212.04089).** τ_task = θ_ft −
  θ_base; addition/subtraction/negation work compositionally. Established. Same-
  base.
- **TIES-Merging (Yadav et al., NeurIPS 2023).** Trim small-magnitude changes,
  resolve sign conflicts by election, average the survivors. SOTA for same-base
  task vectors as of 2023–2024.
- **DARE (Yu et al., 2024, arXiv:2311.03099).** Randomly drop 90% of delta
  parameters and rescale by 1/(1-p). Combined with TIES (DARE-TIES) produces
  the leaderboard-topping merges in the mergekit-style community. Same-base.
- **SLERP / spherical interpolation.** Standard mergekit primitive.
  Empirically robust for two same-base fine-tunes; degrades quickly past two.
- **Mergekit (Goddard et al., 2024).** The de-facto toolkit; supports all of
  the above plus passthrough (architectural augmentation by stacking layers).

For all of these, the constraint is shared: **identical architecture, identical
tokenizer, shared loss-landscape neighborhood (typically fine-tunes of the same
checkpoint).** Quality of merged models routinely exceeds individual parents on
in-distribution tasks; out-of-distribution behavior is brittle.

### 3.2 Cross-architecture merging (frontier, 2024–2026)

- **Git Re-Basin (Ainsworth et al., ICLR 2023, arXiv:2209.04836).** Permutes
  one model's hidden units to align with another's, then merges. Resolves the
  permutation symmetry obstacle. Works within same architecture; *partial*
  cross-base when architectures are close. Empirically: linear mode
  connectivity holds after permutation alignment for many same-architecture
  pairs; cross-architecture mileage varies.
- **FuseLLM (Wan et al., ICLR 2024, arXiv:2401.10491).** Cross-architecture
  *knowledge fusion* via distillation: align token-level probability
  distributions from multiple source LLMs into a single target. Works on Llama,
  OpenLLaMA, MPT — different architectures, different tokenizers, fused into
  one target. *Quality:* the fused model exceeds the strongest individual
  parent on the FuseLLM-7B benchmarks. **This is the first credible cross-
  architecture merger.** Critical caveat: it's distillation, not weight
  averaging — the cost is one full forward pass per token across all sources.
- **Sakana Evolutionary Model Merge (Akiba et al., Nature Machine Intelligence,
  January 2025; arXiv:2403.13187 "Evolutionary Optimization of Model Merging
  Recipes," 2024).** Evolutionary search over merge recipes (which layers to
  combine from which models, using which merging primitive). Produces SOTA
  Japanese-language LLM by merging non-Japanese specialists. Same architecture
  family (all Mistral derivatives) but heterogeneous specialists; the
  *innovation* is the search, not cross-architecture weights.
- **Transport and Merge (Liu et al., ICML 2026, arXiv:2602.05495).**
  Cross-architecture merging via optimal transport: align activations to infer
  cross-neuron correspondences between heterogeneous models. **Most directly
  relevant to the blender hypothesis** — OT is a principled tool for matching
  spectral fingerprints across architectures. Status: published Feb 2026, ICML
  2026 poster; results show meaningful cross-architecture transfer but well
  below same-architecture merge quality.
- **Awesome-Model-Merging-Methods (GitHub: ennengyang/awesome-model-merging-...).**
  The 2025 community taxonomy; 200+ papers; cross-architecture remains the
  open frontier.
- **A Survey of Weight Space Learning (arXiv:2603.10090, 2026).** First
  unified taxonomy of "neural network weights as a data modality." §4 covers
  hypernetworks and generative weight models. Establishes WSL as a recognized
  field with its own ICLR workshop (Weight Space Learning Workshop, ICLR 2025).

**Verdict on cross-architecture merging:** the field is advancing, *but* every
method that works uses either (a) distillation on a real corpus (FuseLLM) or (b)
optimal-transport-aligned activations (Transport-and-Merge). **No method merges
cross-architecture weights from spectral signatures alone.** The blender
hypothesis's literal form has no published precedent. Its reframed form (use OT
or distillation as the singular-vector recovery step, use Ricci flow as the
optimizer) is at the frontier and partially supported.

---

## 4. Mode connectivity — what's known about interpolable paths

- **Garipov et al. (NeurIPS 2018), "Loss Surfaces, Mode Connectivity, and
  Fast Ensembling."** Two SGD solutions are connected by simple paths along
  which loss stays low. Foundational. Same architecture, same training data.
- **Frankle et al. (ICML 2020), "Linear Mode Connectivity and the Lottery
  Ticket Hypothesis."** After a brief "stable" phase early in training, two
  copies of the same network that diverge stochastically end up linearly
  connected. Establishes LMC as a property of well-trained networks.
- **Entezari et al. (ICLR 2022), "The Role of Permutation Invariance in
  Linear Mode Connectivity."** Conjectured (and Git Re-Basin demonstrated)
  that *after* permutation alignment, almost all SGD solutions are linearly
  connected. **This is the structural fact the blender depends on:** if it
  weren't true, even blends of same-base fine-tunes would produce noise. It
  *is* true, with caveats on alignment.
- **Ainsworth et al. (ICLR 2023, Git Re-Basin, arXiv:2209.04836).** Three
  matching algorithms (activation matching, weight matching, learned
  permutation) operationalize the alignment. Works robustly for ResNets, CNNs;
  partial for transformers due to attention head symmetries.
- **Unveiling Linear Mode Connectivity of Re-basin from Neuron Distribution**
  (OpenReview RzOm9oOSzm, 2023). Theory paper explaining *why* permutation
  alignment unlocks LMC; ties it to the neuron-distribution overlap between
  models.

**What this means for the blender:** the path between two trained-same-base
models in weight space is usable; the *interior* of that path is itself a
working model. This is the structural backing for blending. For cross-base
models (different initializations, different data orderings, different architectures),
the LMC literature is silent on linear interpolation — OT-based or distillation-
based alignment becomes necessary first. **The blender's literal weight-space
linear combination across base models is not supported; the OT-aligned variant
is the frontier.**

---

## 5. Reverse distillation / weight synthesis

- **Hinton et al. (2015), "Distilling the Knowledge in a Neural Network,"
  arXiv:1503.02531.** Classic. Train a student to match a teacher's output
  distribution. The student is *new weights*; the teacher is the *spec*.
  Distillation IS a form of weight synthesis from a behavioral spec.
- **HyperNetworks (Ha et al., ICLR 2017, arXiv:1609.09106).** A network whose
  output is another network's weights. Established. Typically used for
  parameter-efficient fine-tuning or conditional generation; the hypernetwork
  is trained end-to-end on the downstream task.
- **D2NWG — Diffusion-based Neural Network Weights Generation (Nava et al.,
  ICLR 2025).** A diffusion process synthesizes neural network weights
  conditioned on a task specification. *This is the most direct prior art for
  the blender hypothesis as a synthesis claim.* The diffusion model learns the
  distribution of weights across many trained networks; sampling produces new
  weights for new tasks. **Crucially:** the spec is a task description (not
  spectral parameters), and the synthesized network has the same architecture
  as the diffusion model's training set.
- **Implicit Neural Representation Generation** (OpenReview, 2024).
  Optimization-biased hypernetworks generate INR weights; the gradient flow
  structure is incorporated into the parameter synthesis.
- **Spectral pruning of fully connected layers (Giambagli et al., Nature Sci.
  Rep. 2022, arXiv:2108.00940).** **This is the closest prior art for
  spectrum-as-parameter training.** Training is reformulated in spectral space:
  *eigenvalues and eigenvectors of transfer operators become optimization
  targets in place of individual weights*. Two protocols:
    1. Post-training: prune nodes by eigenvalue magnitude; near-zero quality
       loss at high compression.
    2. Two-stage pre-training: first train eigenvalues only, prune low-mag
       nodes, then refine eigenvectors. Discovers winning tickets via spectral
       optimization. Tested on MNIST, Fashion-MNIST, CIFAR-10; works on single
       and multi-hidden-layer architectures.

**The spectral pruning result is load-bearing for this assessment.** It
establishes that *eigenvalues are usable as optimization targets independent of
eigenvectors*. The two-stage protocol — train eigenvalues, then refine
eigenvectors — is structurally close to what the blender would need. The result
is on FC layers in small networks; it does not yet scale to transformers. But it
is the proof-of-concept for the architecture the strong-form blender requires.

**Verdict on weight synthesis:** D2NWG, hypernetworks, and spectral pruning
together constitute a real body of prior art for *synthesizing weights from a
spec*. The blender's specific spec (blended spectral fingerprint) has no
published precedent, but the technical ingredients exist. The synthesis is
plausible at small scale today; transformer scale is open.

---

## 6. Ricci flow as optimizer — prior art exists

This section is where the smelter feasibility assessment understated the prior
art. Recent 2024–2025 papers establish Ricci flow as a *real* tool in deep
learning, not just a metaphor:

- **Deep learning as Ricci flow (Bunda et al., Nature Scientific Reports,
  October 2024).** Interprets DNN forward-pass dynamics as a discrete analogue
  of Hamilton's Ricci flow. Defines a *Ricci coefficient* that quantifies
  global Ricci-flow-like behavior in a trained DNN. Across 1,500+ trained
  networks, *Ricci coefficient strength positively correlates with classification
  accuracy*. Ricci flow dynamics aggregate same-class points / separate
  different-class points depending on the dataset. **This is direct empirical
  evidence that Ricci flow is a structurally relevant lens for understanding
  what trained networks compute.**
- **Geometric Meta-Learning via Coupled Ricci Flow (Zhou et al., March 2025,
  arXiv:2503.19867).** "Establishes a unified framework integrating geometric
  flows with deep learning through three fundamental innovations." Treats
  meta-learning as a Ricci-flow process on the task distribution manifold.
- **Learning Discretized Neural Networks under Ricci Flow (Cui et al., JMLR
  2024, v25/22-0444).** Uses Ricci flow to address the
  infinite-or-zero-gradient pathology of low-precision DNN training. Concrete
  optimizer-replacement use of Ricci flow.
- **RicciNets (Glos et al., AutoML 2020).** Uses Ollivier-Ricci curvature to
  identify salient computational paths in randomly wired neural networks
  *before* training. Curvature-guided pruning.
- **Ricci-GNN (OpenReview _qoQkWNEhS).** GNN defense via Ricci-flow-based
  graph resampling each training iteration. Concrete training-loop integration
  of discrete Ricci flow.
- **Discrete Ollivier-Ricci Curvature (Esfahani-Bahrami, arXiv:2203.16837).**
  Theoretical foundations of Ollivier-Ricci on weighted graphs. The reference
  spec for what mirror's kintsugi computes.
- **Discrete Curvature and Applications in Graph Machine Learning (SIAM News,
  July 2025).** Survey establishing Ollivier-Ricci as a standard graph-ML
  tool. Notes that ORC requires solving an optimal transport problem per edge
  — the cost is non-trivial.
- **Ollivier (2009, arXiv:math/0701886).** Foundational paper. The discrete
  Ricci curvature definition mirror inherits.
- **Perelman (2002, arXiv:math/0211159).** Continuous Ricci flow; the
  F-functional monotonicity that mirror's Banach contraction analogizes.

**Verdict on Ricci flow as optimizer:** prior art exists and is growing. The
*specific* claim — Ricci flow as the *training/synthesis* optimizer for a
transformer-shaped target network — is novel; no paper currently does this. But
the building blocks are real (DNN-as-Ricci-flow, Ricci-flow-as-optimizer for
low-precision nets, ORC-guided pruning). The mirror corpus's identification of
Kintsugi ≡ discrete Ricci flow puts it on supported terrain, not in metaphor
land. This is a meaningful upgrade from the smelter assessment's posture.

---

## 7. The synthesis chain — concrete sketch

Assembled honestly, the blender pipeline looks like this:

```
  crystal_a, crystal_b, ...     α, β, ...  (mixing weights)
        │                            │
        └────────────┬────────────────┘
                     ▼
           ┌───────────────────────────────┐
           │  (A) Blend operator on profiles    │  ← well-defined: linear comb. of spectra
           │     target_profile = Σ αᵢ · prof_i │
           └───────────┬───────────────────┘
                     ▼
           ┌───────────────────────────────┐
           │  (B) Synthesis target choice       │  ← hyperparameter: arch family, sizes
           │     architecture, depth, heads     │
           └───────────┬───────────────────┘
                     ▼
           ┌───────────────────────────────┐
           │  (C1) Init eigenvalues to target   │  ← spectral-pruning-style param.
           │  (C2) Init eigenvectors arbitrary  │  ← random / small-corpus seeded
           └───────────┬───────────────────┘
                     ▼
           ┌───────────────────────────────┐
           │  (D) Kintsugi loop (Ricci flow)    │  ← contraction map per
           │     repeat until Lawvere fixpoint: │    kintsugi-formatter.md
           │       • measure edge curvature     │
           │       • update edge weights        │
           │       • distill from small corpus  │  ← FuseLLM-style; needed for
           │         to refine singular vectors  │    eigenvectors (cannot come
           └───────────┬───────────────────┘   from spectrum alone)
                     ▼
           ┌───────────────────────────────┐
           │  (E) Local model artifact          │  ← small, runs via @io, halts(g)
           │     → content-addressed crystal    │    by construction
           └───────────────────────────────┘
```

### Step-by-step support

| Step | Support | Status |
|------|---------|--------|
| (A) Blend profiles | Trivially well-defined as vector-space sum | **Confirmed**; mathematically clean |
| (B) Pick synthesis target | Hyperparameter; small target = feasible compute | **Plausible**; small models tractable |
| (C1) Init eigenvalues | Spectral pruning (2022) shows this is supported | **Likely**; FC layers proven, transformers open |
| (C2) Init eigenvectors | Random + small-corpus warm-up | **Plausible**; standard practice |
| (D) Kintsugi loop | Ricci flow as optimizer (2024–2025 prior art) | **Plausible** for the curvature smoothing; **likely** that Banach contraction applies to the Lawvere fixed point per formatter spec |
| (D) Distill for eigenvectors | FuseLLM (2024) shows cross-arch distillation works | **Likely**; requires real corpus, not free |
| (E) Local artifact | If target is small + the synthesis halts, glass_wall + halts(g) hold | **Confirmed** architecturally |

### What's load-bearing that wasn't obvious

The synthesis cannot escape the corpus requirement. **No path produces a working
model from spectral fingerprints alone.** The singular-vector information must
come from somewhere; the realistic source is distillation against a small
corpus. This means the blender is *not* a model-from-eigenvalues-alone synthesizer;
it is a *cross-architecture distillation guided by a spectral target*. The
crystals shape the target; the corpus provides the gradient signal. Kintsugi
optimizes the geometric loss on the edge graph; conventional gradient methods
optimize the per-token loss on the corpus. Both run in the same loop.

This is mathematically honest and operationally feasible at small scale today
(e.g., synthesizing a ~100M–1B param target from 3–5 source crystals using a
~10k-document distillation corpus). It is not yet feasible at frontier scale
(synthesizing a 70B+ target from a handful of crystals). The compute gap is
the distillation cost, which is linear in corpus size and target size.

---

## 8. What's preserved vs lost

### Preserved by the reframed synthesis

- **Spectral structure of inputs.** The target spectrum is a weighted mean of
  the input spectra; the blend is geometric (interpolation in spectral space).
- **Bilinear attention structure.** Per-layer W_K·W_Qᵀ eigenstructure can be
  enforced as a constraint during synthesis (smelter already computes this).
- **Content addressing.** Crystals are CID-tagged; blend is CID-tagged as
  Hash(Σ (αᵢ, CID_i)); synthesis is CID-tagged as Hash(blend, target_arch, seed,
  corpus_CID); the output model is itself a crystal.
- **Halting / determinism.** Every step is bounded-time and seeded; the
  whole pipeline is deterministic if its inputs are.
- **Five local guarantees.** Per the formatter spec, the Banach contraction
  terminates; the Lawvere fixed point is the stopping criterion;
  glass_wall(g) holds because the new model is a generated artifact under
  @io invocation.

### Lost vs sources

- **Singular vectors of sources.** The blend target inherits source spectra,
  not source vectors. The synthesized eigenvectors are independent of the
  sources; they are learned from the distillation corpus.
- **Layer composition specifics.** Source models' per-layer interactions are
  not preserved; the target model has its own composition. The spectral
  fingerprint constrains *what each layer can compute* at second-moment level,
  not *how layers compose*.
- **Non-linearities and norm structure.** Source models' specific activation
  functions and layer-norm parameters do not transfer; the target uses its own
  choices.
- **Tokenizers.** The synthesized model has its own tokenizer; cross-tokenizer
  blending requires explicit OT alignment (Transport-and-Merge, 2026) or
  vocabulary projection.
- **Mechanistic circuits.** Induction heads, copy heads, suppression heads in
  the sources do not transfer; they may or may not emerge in the synthesized
  model.
- **Behavioral capabilities tied to vector structure.** Where the source's
  capability lives in specific singular directions (e.g., a refusal
  direction; a translation direction), the spectral blend will not capture
  that capability without additional supervision.

**Fidelity accounting:** the synthesized model will be *spectrally
interpolated* but *behaviorally novel*. It is not a clone of any source; it is
a new artifact constrained to lie in a particular spectral region. The labs'
crystals provide a *prior*; the corpus provides the *posterior*. This is
closer to fine-tuning-from-spectral-init than to model merging.

---

## 9. Architectural fit with mirror's substrate

Evaluating the reframed blender pipeline against the five local guarantees:

- **halts(g)** — The synthesis pipeline halts iff the Kintsugi loop halts iff
  the Banach contraction argument holds. Per the formatter spec, this is
  guaranteed when the obligation set is finite and the substrate's spectral
  gap is positive (i.e., the Hodge harmonic component at the synthesis hole
  is trivial). For a small target architecture with a fixed obligation set,
  this is structurally guaranteed. **✅ Preserved.**
- **autopoietic(g)** — Each synthesis tick produces a new spectral coordinate
  for the target; the coordinates form a fragmentation chain; the final
  artifact is content-addressed. The blend operator is itself a grammar
  action (`@fate/smelter/blend`); its output is a `NakedSingularity`. The
  pipeline closes. **✅ Preserved**, provided the blend operator is declared as
  a kintsugi grammar action.
- **glass_wall(g)** — The synthesized model is a generated artifact; running
  it requires @io invocation (it's not a mirror grammar). The artifact's
  *production* is mirror; its *execution* is @io. This is exactly the
  cross-wall structure from `2026-05-26-glass-wall-and-cross-wall-kintsugi.md`.
  The runtime is @io; the substrate is mirror. **✅ Preserved**, with the
  honest fineprint that the model itself lives in @io.
- **content_addressed(g)** — Every step produces a CID. The synthesized
  model's CID is a deterministic function of the inputs (crystal CIDs, mixing
  weights, target architecture, distillation corpus CID, seed). Reproducible
  by construction. **✅ Preserved.**
- **is-copium / alignment decidability** — The synthesis runs locally; the
  artifact runs locally; no telemetry. Whether the artifact "is aligned" is
  orthogonal to whether the substrate is local. **✅ Substrate qualification
  holds.**

**Architectural fit verdict: confirmed.** The reframed pipeline is exactly the
kind of synthesis the local discipline was designed to admit. The labs-as-
eigenvalue-suppliers framing now extends to *labs-as-spectral-prior-suppliers*:
the lab publishes a signed crystal; users blend crystals to define a target
prior; the synthesis happens locally with a small distillation corpus; the
result is a local model the user owns.

---

## 10. Feasibility verdict per question

Directly answering the six questions from the brief.

| # | Question | Verdict |
|---|----------|---------|
| 1 | Can a model be synthesized from a blend of eigenvalue profiles? | **Speculative as literal claim; plausible as reframed claim** (spectrum constrains target; eigenvectors learned from corpus per spectral pruning + FuseLLM evidence). |
| 2 | Is Kintsugi (Ricci flow) a viable optimizer for this synthesis? | **Plausible.** Prior art on Ricci flow in DNN training is real (Nature Sci. Rep. 2024; arXiv:2503.19867 2025; JMLR 2024). The Banach contraction argument from the formatter spec is mathematically clean; whether it produces a *good* model (not just a converged one) is empirical. |
| 3 | What does model merging literature actually achieve? | **Confirmed surveys above.** Same-base merging (TIES, DARE, Soups) is SOTA-quality; cross-architecture merging is at the frontier (FuseLLM via distillation; Transport-and-Merge via OT, both 2024–2026); spectral-fingerprint cross-arch merging has no published precedent. |
| 4 | Mode connectivity — what's known? | **Confirmed.** Linear mode connectivity after permutation alignment is robust for same-arch (Entezari, Git Re-Basin). Cross-arch interpolation requires explicit alignment (OT or distillation). The blender's literal weight-space linear blend across architectures is *not* supported; the OT-aligned variant is the frontier. |
| 5 | Reverse distillation / weight synthesis? | **Plausible.** D2NWG (ICLR 2025) does diffusion-based weight synthesis from a task spec. Spectral pruning (Nature Sci. Rep. 2022) uses eigenvalues as direct optimization targets. HyperNetworks (Ha 2017) generate weights from a conditioning input. The blender's specific recipe (synthesize from blended spectral fingerprint) has no precedent but the technical ingredients exist. |
| 6 | Architectural fit with mirror's substrate? | **Confirmed.** All five local guarantees hold for the reframed pipeline, provided the blend operator is a kintsugi grammar action and the synthesis target is parametrically bounded. |

Overall: **the strong-form synthesis hypothesis survives in a reframed form
that is consistent with the recent literature and preserves the substrate's
discipline.** The reframe is: *blend spectra to define a target prior; synthesize
a small new model whose spectrum matches the prior; learn the eigenvectors from
a small distillation corpus; Kintsugi is the curvature-smoothing outer loop.*

---

## 11. Concrete next experiments

Smallest experiments that would validate (or refute) the reframed strong form:

**Experiment B' — spectral fingerprint reduction (1 session).** Define a
deterministic reduction from smelter's raw output to a fixed-size spectral
signature suitable as a *target* (not just a feature). Candidate: per-layer
singular-value distribution moments (mean, variance, skewness of the spectrum),
spectral entropy, condition numbers, bilinear-form eigenvalue distribution.
This is Experiment A from the smelter assessment, repurposed.

**Experiment F — spectral-init MLP synthesis (2–3 sessions).** Smallest
meaningful synthesis: replicate the spectral pruning two-stage protocol
(Giambagli 2022) on a tiny MLP target, with the eigenvalue init coming from a
blend of two source MLPs' eigenvalues. Use MNIST/Fashion-MNIST as the
distillation corpus. **Test:** does the synthesized MLP outperform random-init
at matched corpus budget? If yes, spectral-prior synthesis works for FC layers
(matching existing prior art with a blend twist). If no, the blend prior is
worse than no prior.

**Experiment G — Ricci flow on the synthesis loop (3–4 sessions).** Replace
the stage-2 training in Experiment F's eigenvector refinement with an
Ollivier-Ricci-flow update on the network's edge graph. Compare convergence
rate and final quality vs SGD baseline. **Test:** does Ricci flow as a
curvature-smoothing outer loop on top of SGD reduce the corpus budget needed
to reach a fixed quality? If yes, Kintsugi is operationally useful. If no, it
is architecturally elegant but practically inert; reframe accordingly.

**Experiment H — cross-architecture blend (research-grade, 5–7 sessions).**
The smallest meaningful cross-arch synthesis: blend the spectral fingerprints
of two same-tokenizer different-architecture small models (e.g., GPT-2-small
and Pythia-160M), synthesize a target whose architecture is a mix (e.g.,
GPT-2-small's depth + Pythia's heads), distill against a small corpus (TinyStories
or similar). **Test:** does the synthesized model exceed either parent on a held-out
perplexity benchmark? If yes, the blender produces something genuinely new
rather than just averaging. If no, single-source distillation would have worked
better and the blend prior is noise.

**Experiment I — OT-aligned cross-tokenizer blend (research-grade, 7–10
sessions).** Replicate Transport-and-Merge's optimal-transport activation
alignment, then use the blended spectrum as the synthesis target on the
aligned space. **Test:** does the OT alignment plus spectral target produce
better cross-architecture transfer than either alone? This is the frontier; if
it works, the blender becomes a publishable contribution at ICML 2027.

B' → F → G is the buildable path; H and I are the research arc. F is
smallest-meaningful; G is the Kintsugi proof-of-concept; H is the smallest
cross-arch evidence.

---

## 12. Open questions

1. **What's the right spectral target representation?** Per-layer SV moments?
   Concatenated full spectra? Learned projection? The choice affects what the
   blend operator can express and what synthesis can target.

2. **Does the spectrum-as-target actually constrain behavior?** Experiment F
   is the falsifier. If a synthesized MLP with target spectrum X behaves
   roughly like one with random spectrum, the spectral prior is uninformative
   and the whole hypothesis collapses. The 2022 spectral-pruning paper suggests
   the spectrum does constrain, but it was tested in a pruning context, not a
   synthesis context.

3. **Is Kintsugi's discrete Ricci flow the *right* curvature flow for transformer
   synthesis?** Ollivier-Ricci is one choice; Forman-Ricci is faster; Hodge-flow
   is theoretically cleaner. The mirror corpus picks Ollivier-Ricci by structural
   resonance with optimal transport on the substrate's edge graph. This may not
   be the empirically-best choice; needs experimental validation.

4. **What's the minimum viable distillation corpus?** FuseLLM uses substantial
   corpora; D2NWG uses a meta-training distribution of weights; spectral
   pruning uses MNIST/CIFAR. For the blender, the corpus is the eigenvector-
   learning signal. Smaller corpus = blender is more "crystal-driven"; larger
   corpus = blender is more "distillation-driven." Where's the inflection?

5. **Does the Banach contraction argument actually hold for transformer-shaped
   targets?** The formatter spec proves it for the obligation-set case; the
   synthesis case has a different geometry. The Magnot-2025 cycle-averaged
   holonomy inequality applies to bundle sections; whether "network being
   synthesized" is a bundle section in the right sense is non-trivial. Worth
   a careful spec.

6. **How does this interact with the weak-form routing pipeline?** Routing
   blends shape *which engine* runs; synthesis blends *produce a new engine*.
   They are not exclusive. A user could route blended crystals to a synthesized
   blender. The composition is well-defined; whether it adds value is open.

7. **What's the lab's incentive in the synthesis regime?** In the weak-form
   routing regime, the lab supplies a signed steering vector. In the strong-
   form synthesis regime, the lab supplies a spectral prior that *partially
   reconstructs* a model derived from their training spend. The economic
   inversion is sharper but the lab's IP exposure is higher. Worth thinking
   through before publishing the synthesis pipeline.

8. **Does spectral blending have a useful interpretation in terms of the
   Napolitano fiber?** If the Napolitano 16-dim fiber exists as claimed,
   blending crystals is blending fiber sections, and the synthesis is fiber-
   bundle interpolation. This would tie the blender to a categorical-geometric
   story that runs much deeper than current SVD-of-weights. Speculative; worth
   noting for future research.

---

## 13. Citations

### Mirror corpus

- `/Users/alexwolf/dev/projects/mirror/docs/insights/2026-05-26-smelted-eigenvalue-profiles-as-fate-shape.md` — the weak-form base; this doc is its evil twin.
- `/Users/alexwolf/dev/projects/mirror/docs/specs/kintsugi-formatter.md` — the Banach contraction; the Lawvere fixed point; the Ricci flow recognition (§"Kintsugi as discrete Ricci flow").
- `/Users/alexwolf/dev/projects/mirror/docs/specs/kintsugi-tournament.md` — the Fate-resolved merge; the strategy vocabulary analogous to merge-recipe search.
- `/Users/alexwolf/dev/projects/mirror/docs/insights/2026-05-26-glass-wall-and-cross-wall-kintsugi.md` — the substrate-pull discipline at the @io boundary; analogous to the synthesis-as-mirror-grammar question.
- `~/dev/systemic.engineering/practice/insights/spectral-db/eigenvalue-mixing-board.md` — the architectural vision (Reed + Alex, 2026-04-22).
- `~/dev/systemic.engineering/practice/insights/spectral-db/dirac-operator-on-graphs.md` — D² = L; spectral action `Tr(f(D/Λ))`; the loss surface kintsugi smooths.

### Code

- `/Users/alexwolf/dev/projects/smelter/src/*.rs` — the existing extractor.
- `/Users/alexwolf/dev/projects/fate/src/*.rs` — the 16-feature 5-model 425-param selector.

### External literature (verified via Kagi 2026-05-26)

**Ricci flow as optimizer / DNN lens (key 2024–2025 additions):**

- Bunda et al. (2024). "Deep learning as Ricci flow." *Scientific Reports* 14, 41598-024-74045-9. October 2024. https://www.nature.com/articles/s41598-024-74045-9
- Zhou et al. (March 2025). "Geometric Meta-Learning via Coupled Ricci Flow." arXiv:2503.19867.
- Cui et al. (JMLR 2024 v25/22-0444). "Learning Discretized Neural Networks under Ricci Flow."
- Glos et al. (AutoML 2020). "RicciNets: Curvature-guided Pruning of High-performance Neural Networks."
- Ollivier, Y. (2009). "Ricci curvature of Markov chains on metric spaces." arXiv:math/0701886. *J. Funct. Anal.* 256.
- Perelman, G. (2002). "The entropy formula for the Ricci flow and its geometric applications." arXiv:math/0211159.
- Hamilton (1982). "Three-manifolds with positive Ricci curvature." *J. Differential Geom.* 17.
- "Discrete Curvature and Applications in Graph Machine Learning." *SIAM News*, July 2025.

**Spectral training and weight synthesis:**

- Giambagli, L., Buffoni, L., Carletti, T., Nocentini, W., Fanelli, D. (2022). "Spectral pruning of fully connected layers." *Scientific Reports* 12, 41598-022-14805-7. arXiv:2108.00940. **Load-bearing for the synthesis verdict.**
- Nava et al. (ICLR 2025). "D2NWG: Diffusion-based Neural Network Weights Generation." https://proceedings.iclr.cc/paper_files/paper/2025/file/f74d79573d71078848973009d0e99bdb-Paper-Conference.pdf
- Ha, D., Dai, A., Le, Q. V. (ICLR 2017). "HyperNetworks." arXiv:1609.09106.
- Hinton, G., Vinyals, O., Dean, J. (2015). "Distilling the Knowledge in a Neural Network." arXiv:1503.02531.
- A Survey of Weight Space Learning (arXiv:2603.10090, 2026). First taxonomy; establishes WSL as a recognized field.
- ICLR 2025 Weight Space Learning Workshop. https://weight-space-learning.github.io/

**Model merging — classic:**

- Wortsman et al. (ICML 2022). "Model soups: averaging weights of multiple fine-tuned models." arXiv:2203.05482.
- Ilharco et al. (2022). "Editing Models with Task Arithmetic." arXiv:2212.04089.
- Yadav et al. (NeurIPS 2023). "TIES-Merging: Resolving Interference When Merging Models."
- Yu et al. (2024). "DARE: Drop And Rescale." arXiv:2311.03099.
- Goddard et al. (2024). "Mergekit." Community toolkit.

**Model merging — cross-architecture and recent:**

- Wan et al. (ICLR 2024). "Knowledge Fusion of Large Language Models." arXiv:2401.10491. **The first credible cross-architecture merger.**
- Akiba et al. (Nature Machine Intelligence, January 2025; arXiv:2403.13187, 2024). "Evolutionary Optimization of Model Merging Recipes." Sakana AI.
- Liu et al. (ICML 2026, arXiv:2602.05495). "Transport and Merge: Cross-Architecture Merging for Large Language Models." **Most directly relevant to the blender.**
- A Review of Model Merging Approaches (arXiv:2503.08998, March 2025).
- Model Merging in LLMs, MLLMs, and Beyond: Methods, Theories. ACM Computing Surveys, 2025.

**Mode connectivity:**

- Garipov et al. (NeurIPS 2018). "Loss Surfaces, Mode Connectivity, and Fast Ensembling."
- Frankle et al. (ICML 2020). "Linear Mode Connectivity and the Lottery Ticket Hypothesis."
- Entezari et al. (ICLR 2022). "The Role of Permutation Invariance in Linear Mode Connectivity."
- Ainsworth et al. (ICLR 2023). "Git Re-Basin: Merging Models modulo Permutation Symmetries." arXiv:2209.04836.
- "Unveiling Linear Mode Connectivity of Re-basin from Neuron Distribution." OpenReview RzOm9oOSzm, 2023.

**Holonomy / sheaf:**

- Magnot, J.-P. (2025). "Contextuality, Holonomy and Discrete Fiber Bundles in Group-Valued Boltzmann Machines." arXiv:2509.10536.
- Hansen, J. & Ghrist, R. (2019). "Toward a spectral theory of cellular sheaves." *J. Appl. Comput. Topol.* 3. arXiv:1808.01513.
- Barbero, F., Bodnar, C. et al. (PMLR 2022). "Sheaf Neural Networks with Connection Laplacians." arXiv:2206.08702.

---

## Closing

The strong-form blender hypothesis is *not* refuted; it requires a precise
reframing. The 2024–2026 literature has caught up to mirror's architectural
intuitions in three independent areas: Ricci flow as a DNN training/analysis
tool (Nature 2024), spectral parameterization of network training (Nature 2022),
and cross-architecture merging via principled alignment (FuseLLM 2024,
Transport-and-Merge 2026). These three threads weave into exactly the pipeline
the blender requires.

The literal slogan — "the blender produces a usable model from eigenvalue
crystals alone" — is information-theoretically impossible and remains refuted.
The honest reading — "the blender produces a small new model whose spectral
structure interpolates the input crystals, with eigenvectors learned locally
from a small distillation corpus, optimized by Kintsugi's discrete Ricci flow
on the substrate's edge graph" — is supported piece-by-piece by the literature
and preserves all five local guarantees by mirror's own substrate discipline.

The labs become *spectral-prior suppliers*: each lab publishes a signed crystal
that shapes which models can be synthesized from a given corpus budget. Users
blend crystals to define their target prior; Kintsugi optimizes the synthesis
locally; the resulting model is content-addressed, halts-provable, and lives
entirely under the user's control. The economic inversion holds; the
sovereignty story is sharper than the weak form (the user owns the model, not
just a routing decision); the technical contribution is concrete and
publishable.

The next concrete move is Experiment B' (spectral fingerprint reduction). It
is the prerequisite for everything else and small enough to land in one
session. Experiments F, G, H are the validation chain. Experiment I is the
publishable frontier.

The strong form survives. The slogan needs surgery. The pipeline is buildable.

Apache-2.0 (this insight document).
