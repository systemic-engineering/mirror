# Mirror Tensors vs Industry Tensors — Research Synthesis

2026-05-26 — research synthesis on whether mirror's substrate-altitude
tensor framing is genuinely different from industry use.

Status: Yellow — research findings; not substrate truth. Conducted by a
research agent on `mara/shard-chain`, against the freshly-landed
`gap-tension-tensor-substrate.md` spec. ~60 minutes of focused Kagi
survey across five framing questions.

## The question

Mirror's spec proposes:

- `gap` lives in `@epistemologic/property` and names a *claim + verifier
  + state* triple — the substrate's first-class representation of where
  a property is asserted but not yet decided.
- `tension` lives in `@fate` and composes two opposing gaps with a
  vector.
- `tensor` lives in `@fate` and is a structured collection of tensions
  plus an algebraic-connectivity (Fiedler) value — the substrate's
  description of where the corpus is structurally strained.
- `@fate.minimize(tensor)` is a proposed recursive multi-trajectory
  backtracking action that emits a fracture sequence which lowers the
  tensor's energy.
- The load-bearing claim: because the tensor is **scoped to specific
  types and AST positions** (it is *derived* from a typed substrate,
  not *learned* from data), `@fate` does not need a large model.

**Is this framing genuinely different from how the industry uses
tensors? If so, where is the closest prior art? If not, what is
mirror missing?**

## Findings per area

### 1. Industry use of tensors in 2026

The dominant industry framing in 2026 treats tensors as **the format
of the model itself** — weights organized as multi-dimensional arrays,
gradient descent updates them, inference computes through them. The
three active research frontiers all sit inside that frame:

- **Tensor networks for compression and expressivity.** Tensor-network
  decompositions (MPS, MPO, PEPS, Tucker, CP, tensor-train) are used
  to factor the very large parameter tensors of transformers into
  networks of smaller ones, exploiting low-rank or area-law
  structure. The IJCAI 2025 survey *Tensor Network: from the
  Perspective of AI4Science and Science4AI*
  (https://www.ijcai.org/proceedings/2025/1194.pdf) and the v3 of
  *Tensor Networks Meet Neural Networks* (arXiv:2302.09019) are the
  canonical references; the latter unifies tensor and neural networks
  into "Tensorial Neural Networks" — still entirely about *model*
  parameterization.
- **Low-rank tensor decomposition on LLM weights.** A live thread in
  2025: LoTR (arXiv:2402.01376), TensorLLM (Gu et al., 2025) which
  applies Tucker decomposition to multi-head-attention for up to 250x
  compression, CP/Tucker decomposition on the last fully-connected
  layer (IJFIS 2025, doi 10.5391/IJFIS.2025.25.2.136), and the ICCV
  2025 *Transformed Low-rank Adaptation via Tensor Decomposition*. In
  every case the tensor is the model weights; decomposition is
  efficiency engineering.
- **Tensor completion / tensor sensing.** Recovering a low-rank tensor
  from sparse observations (NeurIPS 2025 *Non-Convex Tensor Recovery
  from Tube-Wise Sensing*; arXiv:2509.10834 *Tensor Sensing*). This is
  about *learning* a tensor from incomplete measurements — the closest
  thing the industry has to "finding the tensor that describes a
  system" — but it is data-driven, not substrate-derived, and assumes
  a low-rank generative model.
- **Tensor product representations (TPR) and compositional binding.**
  Smolensky's 1990 framework that re-surfaces in NeurIPS 2024 (*Soft
  Tensor Product Representations for Fully Continuous, Compositional
  Visual Representations*) and 2025's *Tensor Product Attention Is All
  You Need*. TPR uses tensor products to bind roles to fillers; this
  is closer in spirit to mirror's framing because the tensor encodes
  *symbolic structure*, but it still sits inside a learned-from-data
  pipeline.
- **Tensor calculus inside transformer architectures.** The everyday
  use: batched matmuls, attention as einsum, gradient tensors. This
  is the bulk of practitioner mindshare and the strongest
  default — "tensor = data structure for parameters and activations."

Verdict on Q1: yes, **the dominant industry framing is tensor-as-model-
format or tensor-as-learned-object**. The only frontier that comes
near "tensor as structural description of a typed system" is TPR + a
few categorical/neuro-symbolic threads. Those are covered in §2.

### 2. Substrate-derived vs learned tensors

This is where the closest prior art lives, and the picture is more
nuanced than the framing-difference suggests. Three traditions
intersect:

- **Logic Tensor Networks (LTN)** — Serafini & Garcez,
  arXiv:2012.13635 (final form in *Artificial Intelligence* journal,
  2021). LTN introduces Real Logic, a fully differentiable first-order
  language whose signature elements are *grounded onto tensors*. A
  formula's satisfaction is a tensor-valued quantity in [0,1].
  Crucially: the symbolic structure (predicates, functions, axioms)
  **determines the tensor shape and the loss function**, then a neural
  network is fit to maximize axiom satisfaction. This is *partially*
  substrate-derived: the symbolic substrate shapes the loss, but the
  groundings are still learned from data via gradient descent.
- **Tensor Logic** — Pedro Domingos, arXiv:2510.12269 (October 2025,
  https://homes.cs.washington.edu/~pedrod/tls.pdf). The single most
  relevant 2025 result. Domingos collapses logical rules and Einstein
  summation into a single construct — the tensor equation. Every
  symbolic rule has an equivalent tensor equation; neural networks,
  transformers, formal reasoning, kernel machines, and graphical
  models are all implementable as tensor-logic programs. Quote from
  the slides: *"A relation is a compact representation of a sparse
  Boolean tensor."* This is the deepest existing claim that
  symbolic-substrate and tensor-language are the same thing. Domingos
  even names "sound reasoning in embedding space" as the new direction
  — exactly mirror's `@fate` ambition.
- **A tensor network formalism for neuro-symbolic AI** —
  arXiv:2601.15442 (Goessmann, Schütte et al., WIAS-Berlin preprint,
  January 2026). Introduces a tensor-network formalism that *captures
  sparsity principles originating in the different paradigms in
  tensor decompositions*. It describes a basis-encoding scheme for
  functions and models neural decompositions as tensor decompositions.
  This is the closest thing to mirror's framing in spirit: a sparsity-
  aware tensor structure derived from symbolic considerations.
- **Verifying a Neuro-Symbolic Substrate by Reduction to Tensor
  Normal Form** — clawrxiv:2605.02616 (mirror universe — caveat:
  clawrxiv is not arxiv-proper; this looks like a non-canonical
  preprint server). Proposes that Sutra programs β-reduce to a tensor
  normal form (TNF): one fused tensor-op graph over a frozen
  substrate, with branches collapsed to Lagrange-interpolated
  three-valued-Kleene polynomials. **This is the closest single
  finding to mirror's framing of "the tensor IS the structural
  description of the substrate."** The verification claim is that
  branching disappears under reduction — verification by tensor normal
  form. Provenance is weaker than arXiv but the construction is
  directly relevant.
- **Knowledge graph embedding (TransE, RotatE, etc.).** Relations are
  modeled as tensor operations (translation, rotation in complex
  space). The relational structure is substrate-derived from the
  knowledge graph, but the embeddings themselves are learned. This is
  the most ubiquitous example of "tensor whose shape comes from a
  typed substrate, whose values are learned" — a halfway point.
- **Inconsistency measurement (the load-bearing dark horse).** A
  mature subfield of KR / belief revision that mirror's spec does not
  cite. Thimm's survey (https://mthimm.de/pub/2019/Thimm_2019d.pdf)
  and the AAMAS 2014 *Inconsistency Measurement Thanks to MUS
  Decomposition* (http://www.cril.univ-artois.fr/~raddaoui/AAMAS_JMR_14.pdf)
  formalize *exactly* the operation mirror's spec proposes: given a
  knowledge base with claims, find the minimal inconsistent subsets
  (MUSes), then compute scalar inconsistency measures. *Classifying
  Inconsistency Measures Using Graphs* (Klein et al., 2020) introduces
  MUS-graphs and MIS-graphs — graph structures whose nodes are
  formulas and whose edges encode shared participation in minimal
  inconsistent subsets. *A Formal Metric to Measure Inconsistencies in
  Stakeholder Preferences* (INCOSE 2024,
  doi:10.1002/sys.21801) introduces the *Inconsistency Magnitude* (IM)
  metric. **None of this work expresses the result as a tensor.** It
  uses graphs, scalar measures, and lattices. But the operation —
  *measure structural strain on a typed knowledge base* — is the same
  one mirror's spec names.

Verdict on Q2: **substrate-derived tensors exist as a research thread
but are minority work.** Tensor Logic (Domingos 2025) is the closest
claim. LTN is the most mature implementation. Inconsistency
measurement is the closest *operational* analog but uses graphs and
scalars, not tensors. The MUS-graph framing is essentially what
mirror's `tensor.tensions` would be — but no one in that field has
connected it to a Fiedler value or to multi-trajectory backtracking
inference.

### 3. Small models on bounded inference spaces

The literature broadly *supports* the claim that small models suffice
when the search space is structured, but with several important
caveats.

- **Tiny Recursive Models (TRM)** — Jolicoeur-Martineau et al.,
  arXiv:2510.04871, *Less is More: Recursive Reasoning with Tiny
  Networks* (October 2025). 7M-parameter recursive model that beats
  DeepSeek R1, Gemini 2.5 Pro, and o3-mini on ARC-AGI-1 (45%) and
  ARC-AGI-2 (8%). ARC-AGI is a bounded symbolic puzzle space.
  Replicated in March 2026 with Mamba-2 hybrid attention (44.6% on
  ARC-AGI-1, ~7M params). **This is the strongest empirical evidence
  for the small-model-on-bounded-task claim.**
- **GRAM** — Baek, Jo, Kim, Ren, Bengio & Ahn,
  arXiv:2605.19376v2 (May 2026). Probabilistic multi-trajectory
  recursive reasoning. 52.0% on ARC-AGI-1, 44.6% on ARC-AGI-2 with a
  generative recursive architecture. Treats reasoning as a stochastic
  latent trajectory. Already cited in mirror's `@fate` baseline insight.
- **THINKSLM** (EMNLP 2025,
  https://aclanthology.org/2025.emnlp-main.1659.pdf) explicitly
  challenges "the assumption that reasoning ability only comes from
  scaling" and demonstrates reasoning in SLMs. Microsoft's Phi-4-
  reasoning is the canonical industrial example.
- **Grammar-constrained decoding (GCD)** — ICML 2025 *Flexible and
  Efficient Grammar-Constrained Decoding* (arXiv:2502.05111) and the
  ACL 2025 *Grammar-Constrained Decoding Makes Large Language Models
  Better Logical Parsers* show that masking out grammatically invalid
  tokens improves smaller models more than larger ones — i.e., the
  benefit of constraint *anti-correlates with model size*. This is
  load-bearing for mirror's claim: when the search space is
  AST-bounded, the constraint substitutes for parameters.
- **Program Synthesis with LLM-Predicted Minimal Specialized
  Grammars** (ACM 2025, doi:10.1145/3712256.3726430). Generating
  problem-specific grammar subsets reduces the search-space complexity
  enough that smaller models match larger ones.
- **LeanCopilot / LLM4Rocq / APOLLO** (NeurIPS 2025). LLMs for tactic
  prediction in Lean 4 / Rocq. The on-device training pipeline for Lean
  4 tactic prediction (https://github.com/markm39/openproof-ml) targets
  small models specifically because the proof-state space is
  type-bounded. APOLLO (NeurIPS 2025) demonstrates LLM+Lean
  collaboration outperforming standalone LLM theorem proving.
- **Phase transitions in compression** — *Phase transitions in large
  language model compression* (Nature, 2026,
  https://www.nature.com/articles/s44387-026-00072-8) argues LLMs
  exhibit Model Phase Transitions: performance collapses beyond a
  critical compression threshold. This is the *anti*-evidence: smallness
  has a cliff, not a smooth degradation.
- **ZebraLogic** (OpenReview, https://openreview.net/forum?id=sTAJ9QyA6l)
  argues there are scaling *limits* on LLMs for logical reasoning,
  i.e., "large model + scale" eventually hits a wall on bounded
  symbolic tasks. This supports the small-model claim from the
  opposite direction: scale doesn't fix logical reasoning, so the path
  is through structure not parameters.

Verdict on Q3: **the literature supports the claim, with caveats.**
The TRM result is the canonical evidence: 7M params + recursion +
bounded symbolic space beats frontier LLMs. Grammar-constrained
decoding's benefit scaling inversely with model size is the
mechanism. The phase-transition Nature paper warns that smallness has
a cliff — there is a minimum model size below which capability
collapses, and that minimum is *task-dependent*. Mirror's claim is
stronger than what the literature has tested: mirror proposes that
the tensor itself (the type-scoping plus the AST-positioning) reduces
the minimum-viable-model size. **No one has tested that empirically.**

### 4. "Find the tensor" as a substrate concern

The industry has *names* for individual pieces of mirror's framing
but no single name that maps to all of it:

- **Inconsistency measurement** (Thimm 2019, Klein et al. 2020,
  Jabbour & Raddaoui 2014) is the closest *operational* analog. The
  result is a number or a graph, not a tensor. The shared structure:
  given a typed knowledge base, find the structural conflicts and
  measure them. **This is the established term-of-art for what
  mirror's tensor IS structurally.**
- **MUS-graphs / MIS-graphs** (Klein, Mailly, Thimm) are the
  industry's closest analog to `tensor.tensions`. Each edge
  encodes shared participation in a minimal inconsistent subset.
  Adding a Fiedler value would be a single-step extension.
- **Tensor sensing / tensor completion** name the *inverse problem*
  of recovering a tensor from observations. This is what mirror's
  `tensor_of([gap])` would be doing — but in the industry framing, the
  tensor has a low-rank generative model and the observations are
  random samples. Mirror's version is structural recovery from a typed
  AST, which is closer to *exact algebraic computation* than to
  statistical inference.
- **Spectral graph theory on typed structures** is present in
  AST-Enhanced GNNs (arXiv:2506.14470, *AST-Enhanced or AST-
  Overloaded?* 2025) and in *Toward Interpretable Graph Tensor
  Convolution Neural Network* (ACM 2023, doi:10.1145/3582574) which
  embeds code semantics via graph tensor convolutions. Neither of
  these computes *fiedler-as-strain* on the AST; they use the AST
  graph as input to downstream learned models.
- **Ricci curvature on graphs** is the industry's closest term-of-art
  for "local strain on a graph." Topping et al. ICLR 2022
  (arXiv:2111.14522, *Understanding over-squashing and bottlenecks on
  graphs via curvature*) shows that negatively-curved edges are
  responsible for the over-squashing phenomenon in GNNs. PIORF
  (Physics-Informed Ollivier-Ricci Flow, OpenReview 2024) uses Ricci
  flow to rewire graphs. This is the closest the field has to
  "compute a local strain measure on a typed graph and flow it."
- **Sheaf neural networks** (Bodnar, Barbero et al., NeurIPS 2022;
  ICML 2022 *Sheaf Neural Networks with Connection Laplacians*) attach
  vector spaces to nodes and edges of a graph with linear maps
  between them — a cellular sheaf. This is structurally identical to
  mirror's eigenboard-as-sheaf framing. The sheaf Laplacian's spectrum
  is the closest mathematical object to mirror's proposed `tensor.fiedler`.
- **Deep Learning as Ricci Flow** — Glanville, Forman et al., Nature
  Scientific Reports 2024 (arXiv:2404.14265). 1,500+ DNN classifiers;
  Ricci-flow-like behavior correlates with accuracy independent of
  depth/width/dataset. Mirror's kintsugi-as-Ricci-flow claim is at
  the *substrate* altitude; the Bunda/Glanville paper is at the
  *model dynamics* altitude. Same shape, different altitude.

Verdict on Q4: **the industry has no single name for the full
operation mirror proposes.** The closest composite:
*inconsistency-measurement-on-a-MUS-graph with a sheaf-Laplacian
spectrum and Ricci-curvature-guided rewriting.* No one has shipped
that composite. The pieces all exist in different subfields.

### 5. The no-large-model claim

The empirical lower bound for productive inference on type-constrained
search spaces:

- **TRM bottom line: 7M parameters** is sufficient for ARC-AGI-1
  (45%) when recursion + constraint substitute for scale.
- **GRAM**: ~ten-to-hundred million parameter regime achieves ARC-
  AGI-2 44.6% with stochastic latent trajectories.
- **Grammar-constrained decoding**'s gain anti-correlates with model
  size — the bigger the model, the smaller the relative gain from
  constraint. This means the constraint *substitutes for parameters*
  but with diminishing returns: there is a minimum capability below
  which constraint cannot rescue (Nature 2026 phase-transition paper).
- **No published lower bound** for "tensor-scoped to types + AST
  positions." The closest empirical work that constrains to a typed
  symbolic space at all is LeanCopilot / APOLLO — and those still use
  pretrained LLMs in the billion-parameter regime, with the type
  system used as a *filter* rather than as a *shape-determinant* for
  the model.
- **Phase-transition warning** (Nature 2026): below a critical
  parameter count, capability collapses sharply. The critical count
  is task-dependent. For ARC-AGI it appears to be in the millions
  with the right recursion structure. **For mirror's fracture-
  sequence-emission task it is unknown.**

Verdict on Q5: **the literature supports the direction but does not
validate the specific claim.** TRM is the best evidence that small
models suffice on bounded symbolic tasks. The mechanism — recursion +
constraint substituting for scale — is the right one. But no one has
built a `@fate.minimize(tensor)`-shaped system and measured the lower
bound. Mirror is making a *prediction* that the literature does not
contradict but also does not yet support. The honest position: this
is a research bet aligned with the strongest 2025 results, not a
settled conclusion.

## Synthesis

### Is mirror's framing genuinely different?

**Partially. Three claims with three different verdicts:**

1. *Tensor as the format of the substrate-strain, not the format of
   the model.* **Genuinely different** from the dominant industry use
   (tensor = model weights). The closest existing claim is Domingos's
   Tensor Logic ("a relation is a compact representation of a sparse
   Boolean tensor") but Domingos uses tensors as a unified *language*
   for AI systems, not as a *measurement of substrate health*.

2. *Tensor derived from typed AST positions rather than learned from
   data.* **Partially novel.** Inconsistency measurement does this on
   typed knowledge bases but uses graphs and scalar measures, not
   tensors. Knowledge graph embedding does it for the tensor *shape*
   but learns the *values* from data. LTN is the closest hybrid:
   symbolic shape, learned grounding. Mirror's proposed `tensor_of`
   would have *both* shape and values determined by the substrate —
   no one has shipped exactly this.

3. *Tensor as input to recursive multi-trajectory backtracking with a
   small model.* **Aligned with the frontier, not novel.** TRM, GRAM,
   Tiny Recursive Reasoning with Mamba-2 are the existing examples.
   Mirror's addition is making the tensor itself derived from a typed
   AST rather than from a benchmark task. That addition is
   structurally distinct but empirically untested.

### Where is the closest prior art?

Ranked by structural similarity to mirror's `gap → tension → tensor →
minimize` pipeline:

1. **Inconsistency-measurement with MUS-graphs.** Thimm 2019, Klein et
   al. 2020, Jabbour & Raddaoui AAMAS 2014. Mirror's `gap` is a
   formula with a state; `tension` is an edge in the MUS-graph;
   `tensor.tensions` is the MUS-graph itself; `tensor.fiedler` would
   be a novel spectral measure on it. **Mirror's spec should cite this
   literature.** It is the term-of-art for what the substrate is
   doing.
2. **Tensor Logic (Domingos 2025).** Closest *language*-level analog.
   Treats logical rules and tensor equations as the same operation.
   Mirror's `@fate.minimize` as a tensor-equation rewriter would be a
   natural fit. **Mirror should engage directly with Tensor Logic in
   future spec work.**
3. **Logic Tensor Networks (Serafini-Garcez 2020/2021).** Real Logic
   as differentiable first-order logic with grounded tensors.
   Mirror's `gap.state = heuristic(p)` is exactly LTN's [0,1]-valued
   satisfaction. Mirror could borrow LTN's grounding semantics for the
   heuristic tier.
4. **Sheaf neural networks (Bodnar, Barbero 2022).** Cellular sheaf on
   a graph = mirror's eigenboard-as-sheaf at the model altitude.
   Sheaf Laplacian spectrum = mirror's `tensor.fiedler` mathematically.
5. **Ricci-flow rewiring (PIORF, Topping et al. 2022).** The
   industry's closest analog to kintsugi-fracture-as-Ricci-flow.
   Negatively-curved edges = mirror's high-magnitude tensions.
6. **TRM and GRAM (2025-2026).** The empirical evidence that small
   recursive models can productively inhabit the search space mirror
   wants to give `@fate.minimize`.
7. **Tensor Normal Form verification (clawrxiv 2605.02616).** Direct
   match for "the tensor IS the verified substrate" but provenance is
   non-canonical; treat as suggestive not authoritative.

### What might mirror be missing?

Honest gaps:

- **The inconsistency-measurement field exists and mirror is not
  citing it.** Klein et al. 2020's MUS-graph paper is almost a direct
  blueprint for `tensor.tensions`. Thimm's survey lists every desirable
  property an inconsistency measure should satisfy (the rationality
  postulates) — mirror's `tensor` should be checked against those.
- **The `tension_vector` design call** (§8.1 of the spec) has a clean
  answer in the existing literature: TPR-style tensor product binding,
  or LTN-style real-valued [0,1] satisfaction. Mirror is treating it
  as open; it is mostly answered.
- **The phase-transition risk.** Nature 2026 says capability collapses
  sharply below a critical model size. Mirror's claim that *small
  model + type-scoped tensor* is sufficient assumes no such cliff
  exists in the mirror-shaped search space. That assumption is
  testable and untested.
- **The Fiedler computation cost.** Computing algebraic connectivity
  on a corpus-sized tensor.tensions graph at every compile is
  non-trivial. The spec references `docs/specs/eigenboard-
  representation.md` for the spectral measurement substrate but does
  not bound the cost.
- **Sound reasoning in embedding space** (Domingos 2025) is named as a
  new direction for Tensor Logic. Mirror's `gap_state = heuristic(p)`
  is *exactly* reasoning in embedding space with a soundness marker.
  Domingos's framing could give mirror's `heuristic(p)` a formal
  semantics.

### What might mirror be ahead on?

Claims supported by the absence of prior art:

- **Tensor as content-addressed substrate output.** No paper found
  ties tensor measurement to a content-addressed AST with OID-stable
  rewriting. The tensor's role in mirror is to be the input to a
  *rewrite proposer* whose outputs are then content-addressed back
  into the corpus. That closed loop — `compile → tensor → minimize →
  fracture → apply → recompile → new tensor` — has no exact analog. The
  closest is `mirror compile --strict` plus kintsugi; the formal
  research analog is *iterative compilation with structural
  verification*.
- **`heuristic(p)` as a first-class state tier.** Existing systems
  (LTN, DeepProbLog) have probabilistic groundings but they live
  alongside the symbolic layer rather than as a distinct state on the
  claim itself. Mirror's commitment that the substrate carries the
  *type-level distinction between verified and heuristic* — and refuses
  to upcast — appears novel. The closest analog is three-valued
  Kleene logic (referenced in the Tensor Normal Form preprint) but it
  collapses fewer distinctions.
- **Fracture-sequence emission as the minimize output.** Inconsistency
  measurement gives you a score or a graph; mirror's `minimize` gives
  you a *sequence of rewrites that close the gaps*. No paper found
  produces an executable repair sequence from a tensor. Neural
  knowledge base repairs (Brzeziński et al. 2021, OpenReview gr5FsBVCkJp)
  produce repair actions but use neural networks; mirror would produce
  them from a deterministic backtracking search over a typed AST.
- **Multi-trajectory backtracking on a substrate-typed search space.**
  GRAM does multi-trajectory backtracking on a learned space. Mirror
  proposes multi-trajectory backtracking on a *type-bounded AST search
  space*. The composition is novel.

## Open design implications

Four things this research surfaces for mirror's decision-making:

1. **Cite the inconsistency-measurement field and check the
   rationality postulates.** Thimm's survey enumerates ~15
   properties an inconsistency measure should satisfy (consistency,
   normalization, monotony, free-formula independence, etc.). Mirror's
   `tensor` should satisfy or explicitly reject these. The MUS-graph
   construction is essentially mirror's `tensor.tensions`; treating
   that as established formal machinery rather than novel structure
   may simplify §3.2 of the spec.

2. **Resolve `tension_vector` via Tensor Logic or LTN semantics.**
   The §8.1 design call has answers in 2025 literature. Domingos's
   tensor-logic equations or LTN's Real Logic groundings both give
   `tension_vector` a workable type. Picking one constrains the
   downstream choices in §6 (`@fate.minimize` ranking).

3. **Measure the phase-transition risk before committing to small
   models.** The Nature 2026 phase-transition paper is the strongest
   single piece of contradicting evidence. Before scoping `@fate` to
   ~10-100M parameters per the recursive-multi-trajectory-backtracking
   insight, mirror needs an empirical check: what is the smallest
   recursive model that can emit a productive fracture sequence on a
   mirror-shaped tensor? TRM at 7M is the lower bound for ARC-AGI;
   mirror's task may be easier (more structure) or harder (more
   open-ended fracture catalog).

4. **Engage with `clawrxiv:2605.02616` carefully.** The Tensor Normal
   Form / Sutra verification preprint is structurally identical to
   mirror's framing but the provenance is weaker than arXiv. Treat as
   independent convergent evidence that the framing is in the air,
   not as authoritative prior art that requires reframing mirror's
   contribution. If clawrxiv is the formal-methods sibling of arxiv
   (which the naming suggests), this finding is load-bearing — the
   tensor-normal-form construction is *closer to mirror than anything
   on arxiv proper*.

## Provenance

- Loki↔Alex↔Reed conversation (the originating recognition that
  `gap` lives in `@epistemologic/property` and `@fate` builds tensors
  from it)
- Mara's `gap-tension-tensor-substrate.md` spec on `mara/shard-chain`
  (the proposed substrate, 7db0eb7)
- This research agent's Kagi survey, ~60 minutes, across 16 distinct
  query bundles covering tensor networks, neuro-symbolic AI, small
  models on bounded tasks, inconsistency measurement, spectral graph
  theory, and Ricci flow on graphs

## Kagi citations

Ranked by load-bearing-ness for mirror's framing:

1. **Domingos, P. (2025). *Tensor Logic: The Language of AI.*** arXiv:2510.12269.
   https://arxiv.org/abs/2510.12269. Slides:
   https://homes.cs.washington.edu/~pedrod/tls.pdf.
   **Key contribution:** unifies neural and symbolic AI via tensor
   equations; treats logical rules as Einstein summations; "a relation
   is a compact representation of a sparse Boolean tensor." Names
   sound reasoning in embedding space as the new direction. The single
   closest existing framing claim.

2. **Klein, A., Mailly, J.-G., Thimm, M. (2020). *Classifying
   Inconsistency Measures Using Graphs.*** Journal of Artificial
   Intelligence Research.
   https://www.researchgate.net/publication/337996500. Earlier:
   Jabbour, S., Raddaoui, B. (2014). *Inconsistency Measurement Thanks
   to MUS Decomposition.* AAMAS.
   http://www.cril.univ-artois.fr/~raddaoui/AAMAS_JMR_14.pdf. Survey:
   Thimm, M. (2019). https://mthimm.de/pub/2019/Thimm_2019d.pdf.
   **Key contribution:** MUS-graphs and MIS-graphs as the formal
   structure for measuring structural inconsistency in typed
   knowledge bases. The closest existing analog to
   `tensor.tensions`.

3. **Jolicoeur-Martineau, A. et al. (2025). *Less is More: Recursive
   Reasoning with Tiny Networks.*** arXiv:2510.04871.
   https://arxiv.org/abs/2510.04871. Code:
   https://github.com/SamsungSAILMontreal/TinyRecursiveModels.
   **Key contribution:** 7M-parameter recursive model beats DeepSeek
   R1, Gemini 2.5 Pro, o3-mini on ARC-AGI. The empirical evidence for
   small-model-on-bounded-task.

4. **Baek, J., Jo, S., Kim, S., Ren, M., Bengio, Y., Ahn, S. (2026).
   *Generative Recursive Reasoning (GRAM).*** arXiv:2605.19376v2.
   https://arxiv.org/abs/2605.19376. Project page:
   https://ahn-ml.github.io/gram-website/.
   **Key contribution:** probabilistic multi-trajectory recursive
   reasoning. The direct architectural inspiration for `@fate`'s
   recursive multi-trajectory backtracking property.

5. **Serafini, L., Garcez, A. d'Avila. (2020/2021). *Logic Tensor
   Networks.*** arXiv:2012.13635 and *Artificial Intelligence* journal
   (doi:10.1016/j.artint.2021.103649).
   https://arxiv.org/abs/2012.13635. Code:
   https://github.com/logictensornetworks/logictensornetworks.
   **Key contribution:** Real Logic as differentiable first-order
   logic; symbolic-substrate-determined tensor groundings.

6. **Goessmann, A., Schütte, C. et al. (2026). *A tensor network
   formalism for neuro-symbolic AI.*** arXiv:2601.15442.
   https://arxiv.org/abs/2601.15442. WIAS preprint:
   https://www.wias-berlin.de/preprint/3257.
   **Key contribution:** tensor-network formalism capturing sparsity
   principles across paradigms; basis-encoding scheme for functions;
   neural decompositions as tensor decompositions.

7. **Glanville et al. (Bunda group). (2024). *Deep Learning as Ricci
   Flow.*** Nature Scientific Reports, doi:10.1038/s41598-024-74045-9.
   arXiv:2404.14265. https://arxiv.org/abs/2404.14265.
   **Key contribution:** empirically demonstrates that DNN dynamics
   exhibit Ricci-flow-like behavior on 1,500+ classifiers, correlating
   with accuracy independent of depth/width. The model-altitude
   analog to mirror's substrate-altitude kintsugi-as-Ricci-flow.

8. **Topping, J. et al. (2022). *Understanding over-squashing and
   bottlenecks on graphs via curvature.*** ICLR 2022.
   arXiv:2111.14522. https://arxiv.org/abs/2111.14522.
   **Key contribution:** negatively-curved edges cause over-squashing;
   Ricci-flow-style rewiring fixes it. The closest GNN analog to
   mirror's high-magnitude tensions and kintsugi rewriting.

9. **Bodnar, C., Barbero, F. et al. (2022). *Sheaf Neural Networks
   with Connection Laplacians.*** ICML 2022 / NeurIPS 2022.
   https://proceedings.mlr.press/v196/barbero22a/barbero22a.pdf.
   **Key contribution:** cellular sheaf on a graph; sheaf Laplacian
   as spectral object. The mathematical structure mirror's
   eigenboard-as-sheaf and `tensor.fiedler` inherit.

10. **Manhaeve, R., Dumančić, S., Kimmig, A., Demeester, T., De Raedt,
    L. (2018/2021). *DeepProbLog: Neural Probabilistic Logic
    Programming.*** arXiv:1805.10872. Final form: *Artificial
    Intelligence* 2021, doi:10.1016/j.artint.2021.103504.
    **Key contribution:** probabilistic logic programming with neural
    predicates. The closest probabilistic-substrate analog to
    mirror's `gap_state = heuristic(p)`.

11. **Phase transitions in large language model compression.** (2026).
    Nature. https://www.nature.com/articles/s44387-026-00072-8.
    **Key contribution:** capability collapses sharply below a
    critical compression / parameter threshold. The strongest single
    piece of evidence *against* unbounded small-model claims.

12. **Domingos, P., slides (2025).**
    https://homes.cs.washington.edu/~pedrod/tls.pdf. Companion to
    Tensor Logic. *"A relation is a compact representation of a sparse
    Boolean tensor."*

Additional citations consulted but less load-bearing for the synthesis:
IJCAI 2025 tensor-network survey (https://www.ijcai.org/proceedings/2025/1194.pdf);
Tensor Networks Meet Neural Networks v3 (arXiv:2302.09019);
TensorLLM (Gu et al. 2025, MHA Tucker decomposition);
LoTR (arXiv:2402.01376);
*Soft Tensor Product Representations* NeurIPS 2024;
*Tensor Product Attention Is All You Need* NeurIPS 2025;
SynCode grammar-constrained decoding (https://github.com/structuredllm/syncode);
LeanCopilot (https://github.com/lean-dojo/LeanCopilot);
APOLLO NeurIPS 2025 (LLM+Lean theorem proving);
Classifying Inconsistency Measures Using Graphs (Klein et al. 2020);
Inconsistency Magnitude metric (INCOSE 2024, doi:10.1002/sys.21801);
Tensor Normal Form verification preprint (clawrxiv:2605.02616 — provenance flag);
Neural Knowledge Base Repairs (OpenReview gr5FsBVCkJp);
AST-Enhanced GNNs for code clone detection (arXiv:2506.14470);
Graph Tensor Convolution Neural Network for code embedding (ACM 2023);
PIORF (Physics-Informed Ollivier-Ricci Flow, OpenReview 2024).
