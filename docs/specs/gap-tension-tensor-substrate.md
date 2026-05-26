# `gap` / `tension` / `tensor` — substrate primitives for the proposed loop closure

*2026-05-26. Mara. Proposal — not implementation.*

**Status: Yellow.** The shape emerged in conversation between Alex and Reed on
2026-05-26 (Alex: *“I think `gap` lives in @epistemologic/property and is used
by @fate to build tensors.”*). The types named here are not declared in the
substrate yet. The compiler does not emit gap-typed output today. `@fate.minimize`
is named but has no body. **Every section below tags its altitude: declared,
proposed compiler output, proposed runtime, eventually.** Nothing in this
document runs today except where explicitly noted.

This spec is a TRANSFER spec, not an invention. Where the proposed shape matches
established formalism in the literature, the spec cites it explicitly and adopts
the established notation. Where mirror diverges from prior art, the divergence
is flagged. The goal is to ground mirror's substrate-altitude primitives in the
load-bearing math the field has already settled — not to reinvent it.

**Citation provenance.** All references in this spec were identified in
`docs/insights/2026-05-26-mirror-tensors-vs-industry-tensors-research.md`
and verified from primary sources. Arxiv IDs accompany every cited paper.

Depends on:
- `boot/std/epistemologic/property.mirror` — the `verdict` type; the
  `check` shape. `gap` extends this altitude by adding state and
  verifier-presence.
- `boot/std/epistemologic/property/halts.mirror`,
  `content_addressed.mirror`, `causality.mirror`, et al. — the concrete
  properties whose claims would surface as gaps when their verifier is
  `\`-bodied or absent.
- `boot/std/fate/tournament.mirror` — `@fate/tournament` already names
  rules and ganglia. `tension` and `tensor` would live as siblings under
  `@fate`, composing tournament selection with gap-gradient backtracking.
- `docs/specs/epistemologic-grammar.md` — the literal property; the
  IS-relationship; the verification-is-measurement frame. Gaps measure
  the distance between claim and verifier.
- `docs/specs/kintsugi-fracture-confidence-and-scene-dispatch.md` —
  fracture as the rewrite that closes gaps; confidence threshold for
  autonomous vs scene-dispatched application. `@fate.minimize` would
  emit a fracture sequence.
- `docs/insights/2026-05-26-fixed-and-the-spectral-feedback-fracture.md`
  — the loop closure narrative; what this spec gives a type to.
- `docs/insights/2026-05-26-mirror-tensors-vs-industry-tensors-research.md`
  — the research synthesis identifying the prior art transferred below.

Unblocks (deferred per LRM until consumers surface):
- `@fate.minimize` body (today: declaration only).
- `mirror compile <file>` gap-typed output mode.
- `@spectral` integration: gap counts contribute to fiedler and
  holonomy measurements on the corpus.
- `@scene` curator UI: present a non-converging tensor as a scene the
  curator enters; consent governs which tension to relax first.

---

## 1. The recognition

A `claim` (the `requires` / `ensures` / `property` declarations scattered
across boot grammars) names what the substrate intends to be true. A
`verifier` is the body that decides the claim. Today many verifiers are
`\` — declared but not executing. The substrate has no first-class way
to talk about *the distance between claim and verification*; it has
`verdict` (pass/fail/partial) but not the meta-shape “this claim has a
verifier-shaped hole.”

`gap` names that meta-shape. `tension` names two gaps in opposition.
`tensor` is a structured collection of tensions — the substrate's input
to a backtracking optimizer. `@fate.minimize(tensor)` would walk the
tensor's gradient and emit a fracture sequence that closes gaps.

The loop, *proposed*:

```
compile  →  tensor  →  @fate.minimize(tensor)  →  fracture sequence  →  apply  →  new tensor  →  ...
```

Today: only `compile` runs. The other arrows are declarations or named
intentions. This spec gives them types so subsequent work can give them
bodies in sequence.

**Prior art for this loop shape.** The closest published structural analog
is *inconsistency measurement* — a mature KR / belief revision subfield
that takes a knowledge base, finds the minimal inconsistent subsets (MUSes),
and emits a scalar or graph measure of conflict (Thimm 2019; Jabbour &
Raddaoui AAMAS 2014, arXiv:1406.0155; Klein, Mailly & Thimm JAIR 2020).
Mirror's `compile → tensor` arrow is structurally the inconsistency-graph
construction. Mirror's `tensor → minimize → fracture` arrow has no exact
published analog — the field produces a measure, not an executable repair
sequence. See §3 for citation per primitive.

---

## 2. Placement

**`gap` lives in `@epistemologic/property`.** Claims and verdicts are
epistemologic-altitude; the gap between them is the same altitude. A
gap is a richer `verdict` — it carries the claim's text, the verifier's
presence, and a discrete state tier.

**`tension` and `tensor` live in `@fate`.** They compose gaps into
input for backtracking inference. `@fate` already houses the tournament
rules and the five-ganglia structure (`boot/std/fate/tournament.mirror`);
tensor minimization is a sibling shape — not selection across candidates,
but gradient walk across gaps.

This split mirrors the existing altitude separation: `@epistemologic`
measures what is; `@fate` decides what to do about it.

---

## 3. Types (declared shape — bodies are `\`)

### 3.1 `gap` in `@epistemologic/property`

*Citation provenance:*

- **State tier `heuristic(p)`**: established by Logic Tensor Networks
  (Serafini & Garcez, *Artificial Intelligence* 2021; arXiv:2012.13635).
  LTN's "Real Logic" assigns every closed formula $\varphi$ a satisfaction
  value $\mathcal{G}(\varphi) \in [0,1]$ via a stable-product fuzzy
  semantics. Mirror's `heuristic(p: probability)` is structurally the same
  object: a $[0,1]$-valued grounding of a claim. Adopting LTN's notation
  tightens mirror's semantics — `heuristic(p)` IS a grounding $\mathcal{G}$
  applied to the claim's formula.
- **State tier `verified` vs `declared`**: established by inconsistency
  measurement (Thimm 2019). Thimm distinguishes formulas that participate
  in a minimal unsatisfiable subset (MUS) from free formulas (those not in
  any MUS). Mirror's `verified` corresponds to a free formula whose verifier
  ran and returned `pass`; `declared` corresponds to a claim with no
  evidence, which the substrate carries explicitly rather than collapsing
  into a default-true. The four-tier discrimination is mirror's choice;
  Thimm's tradition uses two tiers (consistent / participant-in-MUS).

```mirror
in @prism
in @epistemologic
in @epistemologic/property
in @nl

grammar @epistemologic/property {
  # ... existing verdict, check, reflect ...

  # a claim is the textual content of a requires / ensures / property
  # declaration. it names what the substrate intends to be true.
  # the claim's identity is its AST node; the text is the rendered form.
  #
  # this maps to the "formula" φ in Thimm 2019's inconsistency-measurement
  # framework: an element of a knowledge base K ⊆ ℒ over a logic ℒ. mirror's
  # specialization: φ is anchored to an AST node, not free-floating.
  type claim = {
    node:   ast,                    # the AST node carrying the claim
    text:   string,                 # the rendered claim ("halts(τ)", etc.)
    site:   span                    # source location
  }

  # a verifier is the body that decides the claim. it may be absent
  # (the substrate-level `\` marker) or present (a runnable lambda).
  type verifier = absent | present(check)

  # probability sits in [0, 1]. when a probability typing grammar lands
  # (per @epistemologic/probability, deferred), this alias becomes a
  # refined type. today it's a structural number.
  #
  # this is LTN's grounding codomain (Serafini & Garcez 2021,
  # arXiv:2012.13635): ℝ-valued satisfaction in [0,1].
  type probability = f64

  # the state tier. four discrete positions on the verdict manifold,
  # plus the absent corner for claims that COULD exist but DON'T.
  #
  # `verified` and `heuristic(p)` are NOT interchangeable: a heuristic
  # is MARKED as such, not asserted as proof. the substrate makes the
  # distinction structural so downstream consumers (fracture
  # generation, conductivity reporting, fiedler measurement) can weight
  # them differently.
  #
  # citation: `heuristic(p)` adopts LTN's Real Logic grounding
  # G(φ) ∈ [0,1]. `verified` and `declared` extend Thimm 2019's binary
  # consistent/inconsistent split into a four-tier discrimination.
  type gap_state =
      verified                          # verifier ran; returned pass
    | heuristic(probability)            # G(φ) ∈ [0,1] per LTN; no verifier
    | declared                          # claim only; no evidence; no estimate
    | absent                            # could be claimed; isn't (surfaced as candidate)

  # the gap itself: claim + verifier-presence + state.
  type gap = {
    claim:    claim,
    verifier: verifier,
    state:    gap_state
  }

  # surface every gap visible in an AST. used by `mirror compile` to
  # emit gap-typed output. proposed; \-bodied today.
  gaps_of(ast) -> [gap] { \ }
}

out claim
out verifier
out probability
out gap_state
out gap
out gaps_of
```

*Altitude: declared shape; bodies are `\`. The substrate would carry these
types; no consumer surfaces them yet.*

*Established (LTN, Thimm) on the state tier; mirror's choice on the four-tier
discrimination and on anchoring claims to AST nodes rather than free formulas.*

### 3.2 `tension` and `tensor` in `@fate`

*Citation provenance:*

- **`tension`** is structurally Klein-Mailly-Thimm 2020's MUS-graph edge
  (JAIR vol. 66, *Classifying Inconsistency Measures Using Graphs*). They
  define the inconsistency graph $G_K = (V, E)$ of a knowledge base $K$ as
  a bipartite graph whose vertex set is $K \cup \mathrm{MI}(K)$ (formulas
  plus minimal inconsistent subsets) with edges encoding set membership.
  Equivalently, the *formula graph* projection has formulas as nodes and
  edges between formulas that co-occur in a MUS — *exactly* mirror's
  `tension`. Jabbour & Raddaoui (AAMAS 2014, arXiv:1406.0155) call the
  same object the MUS-graph and use it to define the *Distribution Index*
  (ID) inconsistency measure. Mirror's contribution beyond their framework:
  the `vector` field tagging the *direction* the tension pulls when
  minimized. **No published inconsistency measure carries a directed pull
  per edge** — Klein 2020 and Jabbour 2014 produce undirected graphs.
- **`tensor`** as a collection of tensions plus spectral signature
  combines Klein 2020's MUS-graph with the *sheaf Laplacian* of Hansen &
  Ghrist 2019 (*Journal of Applied and Computational Topology* 3:315–358,
  arXiv:1808.01513) and the *Neural Sheaf Diffusion* framework of Bodnar,
  Di Giovanni et al. (NeurIPS 2022, arXiv:2202.04579) plus *Sheaf Neural
  Networks with Connection Laplacians* (Barbero, Bodnar et al., ICML TAG-ML
  Workshop 2022, arXiv:2206.08702). The Fiedler value is the second-smallest
  eigenvalue $\lambda_2(L)$ of the (sheaf) Laplacian, a.k.a. algebraic
  connectivity. The transfer is direct: mirror's `tensor.fiedler` IS
  $\lambda_2(L_{\mathcal{F}})$ where $L_{\mathcal{F}}$ is the sheaf
  Laplacian over a cellular sheaf $\mathcal{F}$ on the tension graph.
- **`tensor` as a substrate-derived object (not learned)** distinguishes
  mirror from the dominant industry framing. The closest *language*-level
  prior art is Domingos 2025 (*Tensor Logic*, arXiv:2510.12269): "a relation
  is a compact representation of a sparse Boolean tensor." Mirror's
  `tensor.tensions` IS such a relation. Mirror diverges from Domingos on
  what the tensor's role is: Domingos uses the tensor equation as the
  *programming construct* (every rule = a tensor equation); mirror uses
  the tensor as the *measurement of substrate strain* (the tensor is the
  diagnostic, not the program).

```mirror
in @prism
in @epistemologic/property         # for gap, gap_state

grammar @fate {
  # a tension is two gaps in structural opposition. the vector names
  # the direction the tension pulls when minimized: which gap closes,
  # at what cost to the other.
  #
  # citation: this is Klein-Mailly-Thimm 2020's MUS-graph edge
  # (JAIR vol. 66) and Jabbour & Raddaoui 2014's MUS-graph edge
  # (arXiv:1406.0155). mirror's `vector` field is the divergence —
  # neither published framework carries a directed pull per edge.
  #
  # the vector field is INTENTIONALLY left as a hole at this altitude.
  # whether vector is (delta_a: probability, delta_b: probability) or
  # a richer tangent-space element is a design call Alex has not made.
  # see §8 (design calls flagged) and §8.1 for the LTN/Tensor-Logic
  # answers the literature already supplies.
  type tension = {
    a:      gap,
    b:      gap,
    vector: tension_vector             # \ — see §8
  }

  type tension_vector = \              # design call deferred to Alex

  # a tensor is a structured collection of tensions plus a spectral
  # signature. the fiedler value is the algebraic connectivity of the
  # tension graph — low fiedler means the tensor is loosely coupled
  # (gaps can close independently); high fiedler means the tensor is
  # tightly coupled (closing one gap perturbs many others).
  #
  # citation: fiedler is λ₂(L) per Hansen & Ghrist 2019
  # (arXiv:1808.01513) extended to cellular sheaves; per Bodnar et al.
  # 2022 (Neural Sheaf Diffusion, arXiv:2202.04579) as the spectral
  # object whose gap controls diffusion/convergence on the graph.
  type tensor = {
    tensions: [tension],
    fiedler:  f64                      # λ₂(L_F); algebraic connectivity; ≥0
  }

  # build a tensor from the gaps surfaced by `mirror compile`. proposed;
  # \-bodied today.
  #
  # this is the construction of the inconsistency graph G_K from K
  # (Klein-Mailly-Thimm 2020) lifted to a cellular sheaf F per
  # Hansen & Ghrist 2019 §2. mirror's specialization: the gaps come
  # from a typed AST, not from a free-form knowledge base.
  tensor_of([gap]) -> tensor { \ }

  # the minimize action: walks the tensor's gradient and emits a
  # fracture sequence that closes gaps. proposed; \-bodied today.
  # the fracture type lives in @kintsugi/fracture per
  # docs/specs/kintsugi-fracture-confidence-and-scene-dispatch.md.
  #
  # citation: structurally analogous to Topping et al. 2022's
  # Stochastic Discrete Ricci Flow (SDRF, arXiv:2111.14522), which
  # rewrites a graph to eliminate negatively-curved edges. mirror's
  # `minimize` rewrites the substrate AST to eliminate high-magnitude
  # tensions. the search architecture borrows from Tiny Recursive
  # Models (Jolicoeur-Martineau 2025, arXiv:2510.04871) and GRAM
  # (Baek et al. 2026, arXiv:2605.19376) — recursive multi-trajectory
  # over a bounded symbolic space.
  minimize(tensor) -> [fracture] { \ }
}

out tension
out tension_vector
out tensor
out tensor_of
out minimize
```

*Altitude: declared shape; bodies are `\`. `@fate.minimize` is named in
§6 below as proposed runtime; the type signature is what this spec gives
it. The runtime is not built.*

*Established (Klein 2020 MUS-graph; Hansen & Ghrist sheaf Laplacian;
Bodnar et al. neural sheaf diffusion; Topping et al. balanced Forman
curvature) on the structure. Mirror diverges from Klein 2020 because the
MUS-graph is undirected; mirror's `tension.vector` adds direction. Mirror
diverges from Domingos 2025 because Tensor Logic uses tensors as the
programming construct; mirror uses them as the diagnostic.*

---

## 4. The state tier in detail

*Altitude: declared (the type exists in the substrate once §3 lands).*

Four positions. Each has a structural meaning the downstream consumers
are proposed to respect:

### 4.1 `verified`

The verifier ran. It returned `pass`. The claim and the body agree.
Downstream: contributes positively to conductivity; does not surface
as a fracture candidate.

*Citation: corresponds to a free formula in Thimm 2019's sense — not in any
MUS, contributes nothing to inconsistency. mirror is more demanding: a
formula must have a verifier that **ran** to count as `verified`. Thimm's
tradition treats consistency as the default; mirror treats verification as
an explicit positive event.*

### 4.2 `heuristic(p: probability)`

No verifier ran. A pattern-match (an `@ai/explorer` ganglion, a
`@ai/introject` similarity check, an `@ai/cartographer` neighbourhood
scan) gave a probability that the claim holds. The substrate carries
the probability AS a probability — not as a verdict. Downstream
consumers MUST treat `heuristic(p)` as conditional evidence, not proof.

The MARKING is the point. Heuristics that masquerade as verifications
are the central failure mode this spec exists to prevent. The type
system refuses to upcast `heuristic(p)` to `verified` without an
actual verifier run.

*Citation: established. This IS LTN's Real Logic grounding
$\mathcal{G}(\varphi) \in [0,1]$ (Serafini & Garcez 2021,
arXiv:2012.13635). The four LTN connective groundings
$\mathcal{G}(\neg), \mathcal{G}(\land), \mathcal{G}(\lor),
\mathcal{G}(\Rightarrow)$ over the stable-product t-norm define the
compositional semantics; mirror would inherit this when composing
`heuristic(p_a)` and `heuristic(p_b)` into a tension. Mirror diverges
from LTN because LTN's groundings are **learned** by gradient descent
over axiom satisfaction; mirror's heuristic comes from a pattern-match
that does not update weights through gradient — the value is reported,
not trained. Pedro Domingos 2025 (arXiv:2510.12269) names "sound
reasoning in embedding space" as the new direction this state tier
occupies.*

### 4.3 `declared`

The claim is in the AST. No verifier. No heuristic. The substrate
makes NO estimate. Downstream: fracture candidate (write the verifier);
contributes to gap count but not to confidence.

This is the modal state for boot grammars today. Most `requires`,
`ensures`, and `property` declarations in `boot/std/**/*.mirror` carry
`\` bodies. Each one is a `declared` gap.

*Citation: mirror's choice. No exact prior art — Thimm 2019 treats all
formulas as having truth values (either by classical valuation or by a
non-classical semantics); the notion of "the claim is in the KB but no
semantics has been assigned" is not standard. The closest analog is
three-valued Kleene logic's $\mathrm{undef}$ marker (cited in the prior
research synthesis), but Kleene treats $\mathrm{undef}$ as a truth value
that can be combined; mirror treats `declared` as a *meta*-marker that the
verifier hasn't run, not as a truth value. Proposed — no direct prior art.*

### 4.4 `absent`

The claim COULD be made (the property exists in scope; the type
admits it) but ISN'T (the declaration is missing). Downstream:
surfaced as a candidate; not a fracture target until the absence
becomes a tension with another claim.

Proposed mechanism for surfacing `absent`: when a property is `in`-imported
but never invoked on a type that admits it, the compiler can mark the
missing invocation as `absent`. *Eventually*; the import resolver would
need to track admission relations, which it doesn't today.

*Citation: mirror's choice. Proposed — no direct prior art found. Closest
conceptual neighbor: completion-via-defaults in description logics (open
world assumption); but those infer absence as "unknown," not as
"could-be-claimed-and-isn't." The substrate-altitude commitment is novel.*

---

## 5. Compiler integration

*Altitude: **proposed compiler output**. Today `mirror compile <file>`
does not emit gap-typed output. The example below is what it WOULD emit
once `@epistemologic/property.gaps_of` carries a body and the compiler
wires the call into its reporting path.*

```
@spectral/portal.mirror
  declarations: 12
  claims (requires/ensures): 4
    requires content_addressed(portal)   declared; verifier absent
    requires halts(portal.open)          declared; verifier absent
    ensures  monotonic(portal.tick)      heuristic(0.72); no verifier
    ensures  causality(portal.events)    verified
  holes: 5 (line refs: 12, 28, 41, 67, 89)
  gap: 4 claims, 1 verification, 3 declared, 0 absent, 5 dependent holes
```

*Altitude: **proposed**. `mirror compile @spectral/portal.mirror` today
emits the OID, declaration count, and (per the cascade) the gestalt
fiedler. It does NOT emit the per-claim verdict breakdown. The example
is the spec of what gap-typed output would look like.*

The last line is the **gap line** — the per-file shape that composes
upward into the corpus tensor. A tool building the tensor reads gap
lines, not declaration counts.

*Citation provenance: the per-claim breakdown corresponds to the
inconsistency-graph vertex labelling in Klein-Mailly-Thimm 2020 — each
formula gets a label from the participation lattice. Mirror's gap line
is a per-file projection of that labelling. Proposed format — the JAIR
paper uses a different reporting convention (Hasse diagrams of
participation classes); mirror chooses a flat scalar tally because the
downstream consumer is line-oriented developer output.*

---

## 6. `@fate.minimize` (proposed runtime)

*Altitude: **proposed runtime**. The action is declared in §3.2 with a
`\` body. Today nothing minimizes tensors; this section names what the
body would do.*

Input: a `tensor` built from corpus gaps.

Output: a `[fracture]` — a sequence of fracture rewrites that, applied
in order, would lower the tensor's energy. Each fracture lives in
`@kintsugi/fracture` per the existing fracture-and-scene-dispatch spec.

*Citation provenance: the closest published architecture for this loop is
the pairing of:*

- *Topping et al. 2022 (arXiv:2111.14522) — Stochastic Discrete Ricci Flow
  (SDRF), which rewires graphs by surgically removing **negatively-curved
  edges** under the **Balanced Forman curvature**. Mirror's `tension` with
  high `vector` magnitude is the structural analog of a negatively-curved
  edge; the SDRF removal step is the structural analog of the fracture
  application. The mathematical content transfers exactly: an edge $(i,j)$
  has Balanced Forman curvature $\mathrm{Ric}_{BF}(i,j) = \frac{2}{d_i} +
  \frac{2}{d_j} - 2 + 2 |\sharp(i,j)| / \max(d_i, d_j) + 2 |\sharp(j,i)| /
  \max(d_i, d_j) + \frac{\gamma_{\max}^{-1}(...)}{...}$ where $\sharp(i,j)$
  counts triangles, $\gamma_{\max}$ four-cycles. Mirror would compute an
  analogous local curvature on the tension graph.*
- *TRM (Jolicoeur-Martineau 2025, arXiv:2510.04871) — Tiny Recursive Model.
  A 2-layer network with $\sim7\mathrm{M}$ parameters and full
  back-propagation through a recursion outperforms DeepSeek R1, Gemini
  2.5 Pro, and o3-mini on ARC-AGI-1 (45%) and ARC-AGI-2 (8%). The latent
  state contains both a "current solution" and a "latent reasoning"
  feature, refined recursively under Exponential Moving Average weights.
  This is the empirical evidence for the small-model claim mirror makes
  in §6.1.*
- *GRAM (Baek, Jo, Kim, Ren, Bengio, Ahn 2026, arXiv:2605.19376) —
  Generative Recursive Reasoning. Evolves TRM's deterministic trajectory
  into a probabilistic multi-trajectory generative model trained via
  amortized variational inference on an ELBO. Width-based inference-time
  scaling: sample multiple latent trajectories in parallel. 52.0% on
  ARC-AGI-1, 44.6% on ARC-AGI-2. This is the direct architectural
  inspiration for `@fate`'s recursive multi-trajectory backtracking.*

Proposed body sketch (declarative; not pseudocode for the runtime):

1. **Rank tensions** by `vector` magnitude (highest-pull first).
   *Established*: the SDRF ranking (Topping et al. 2022 §3) selects the
   most-negatively-curved edge as the rewrite target. Mirror's ranking is
   the structural mirror.
2. **Propose the fracture rule** whose application closes the
   higher-confidence gap and reduces the lower-confidence gap's opposition.
   Confidence comes from `kintsugi/fracture` per the fracture-confidence
   spec.
   *Mirror's choice* — SDRF removes an edge and adds a different one (a
   geometric rewiring); mirror applies a kintsugi fracture (a typed AST
   rewrite). The substrate altitude differs; the algorithmic shape (rank,
   propose, apply, re-measure) is shared.
3. **Concatenate** proposed fractures into a sequence; emit.
   *Mirror's choice* — the published inconsistency-measurement field emits
   a *measure* (a scalar) or a *graph* (Klein 2020). It does not emit a
   sequence of repair actions. The closest published work that does is
   neural knowledge-base repair (Brzeziński et al. 2021, surfaced in the
   prior research synthesis), which uses neural networks; mirror's
   deterministic backtracking over a typed AST is novel for this output
   shape.
4. **The caller** (proposed: `@kintsugi/scheduler` or a `mirror minimize`
   subcommand) applies the sequence. Autonomous-apply at
   `confidence = 1.0`; scene-dispatched otherwise.

### 6.1 The small-model claim

*The load-bearing empirical claim:* because `tensor` is **scoped to specific
types and AST positions** (it is derived from a typed substrate, not learned
from data), the search space `@fate.minimize` walks is bounded enough that
a small recursive model suffices.

*Citation provenance:*

- *Supporting:* TRM (arXiv:2510.04871) demonstrates 7M parameters + recursion
  + a bounded symbolic space (ARC-AGI puzzles) beats frontier LLMs at 100–1000x
  their size. GRAM (arXiv:2605.19376) demonstrates the same with stochastic
  multi-trajectory sampling. Grammar-constrained decoding (arXiv:2502.05111
  and others, cited in the prior research synthesis) shows the benefit of
  constraint *anti-correlates with model size* — constraint substitutes for
  parameters.
- *Caveat:* the Nature 2026 phase-transition paper (Phase transitions in
  large language model compression, doi:10.1038/s44387-026-00072-8) shows
  capability collapses sharply below a critical parameter count; that
  critical count is task-dependent and unknown for mirror's
  fracture-sequence-emission task.
- *Honesty marker:* mirror's claim is a research bet aligned with the
  strongest 2025 results, not a settled conclusion. No one has measured
  the lower bound for a `@fate.minimize`-shaped task.

*Established (TRM, GRAM, grammar-constrained decoding) on the direction;
mirror's choice on the specific architecture; no published lower bound for
mirror's task shape.*

### 6.2 Non-converging tensors

When `fiedler` is high and the gradient walk fails to reduce energy below a
threshold within a bounded step count, `minimize` does NOT return an empty
sequence. The substrate MUST surface non-convergence as a first-class
signal — see §8 (design call).

*Citation provenance: the high-fiedler-as-bottleneck framing is Bodnar et al.
2022 (arXiv:2202.04579, Neural Sheaf Diffusion). Their Cheeger-type
inequality bounds the spectral gap from below; when the gap is small, sheaf
diffusion fails to synchronize. Mirror's non-convergence corresponds
structurally to a small spectral gap on the tension graph. The proposed
response — emit a scene rather than a fracture — is mirror's choice.*

---

## 7. Migration order

What lands, in sequence, to close the loop. Each step depends on the
previous. None of these have landed.

1. **Types declared.** `gap`, `gap_state`, `claim`, `verifier`,
   `tension`, `tension_vector`, `tensor`. Per §3. `\` bodies. Just the
   substrate shape; nothing executes.

2. **Compiler emits gap-typed output.** `mirror compile` invokes
   `@epistemologic/property.gaps_of` per file and prints the gap line
   per §5. This requires `gaps_of` to carry a body (walks the AST,
   classifies each `requires` / `ensures` / `property` declaration into
   a `gap_state`). The classification logic is non-trivial — it has to
   distinguish `\`-bodied (declared) from `=`-bodied (verified) from
   heuristic-evidenced (no body, but a confidence signal from elsewhere
   in the corpus).

3. **`tension_of` and `tensor_of` carry bodies.** Composition rules
   for when two gaps are in opposition (proposed: when their claims
   reference overlapping sites and their verifier states differ). The
   fiedler computation reuses the existing spectral measurement
   substrate per `docs/specs/eigenboard-representation.md`. *The Hansen
   & Ghrist 2019 sheaf-Laplacian construction (arXiv:1808.01513) is the
   canonical formal definition to follow; the Bodnar et al. 2022
   sheaf-diffusion paper (arXiv:2202.04579) provides the empirically
   validated learning algorithm if mirror later chooses to learn the
   sheaf structure rather than fix it.*

4. **`@fate.minimize` carries a body.** Per §6. Requires the fracture
   catalog to be populated beyond `@kintsugi/fracture/generic-brackets`
   so a sequence has anything to draw from. *Implementation should
   start from Topping et al.'s SDRF algorithm (arXiv:2111.14522) as the
   architectural template, with the fracture-application step
   substituted for SDRF's edge-rewire step.*

5. **Fracture sequence execution.** The caller wires `minimize` output
   into `@kintsugi/fracture` application. Autonomous at
   `confidence = 1.0` per the existing fracture-confidence spec;
   scene-dispatched below.

6. **Loop closure.** The composed pipeline runs continuously. New gaps
   surfaced by post-fracture compilation feed the next tensor.

Step 1 is small. Step 2 is the largest single piece of work (the
classifier). Steps 3–6 each depend on the previous.

---

## 8. Design calls flagged

These are decisions Alex has not made. The spec lays out the shape;
the substantive choice belongs upstream. **Two of the three calls have
workable answers in the cited literature; the choice is which to adopt.**

### 8.1 `tension_vector` structure

What IS the vector field on `tension`?

- **Option A:** `(delta_a: probability, delta_b: probability)` —
  scalar deltas, one per gap. Simple; loses information about whether
  the gaps share a common verifier-shape.
  *Citation: this is the obvious extension of Klein 2020's MUS-graph
  with per-endpoint weights — not in the published JAIR paper, but a
  natural one-step extension.*
- **Option B:** A tangent-space element on the gap manifold — richer;
  composes naturally with fiedler; requires defining the manifold.
  *Citation: this is the LTN grounding (Serafini & Garcez 2021,
  arXiv:2012.13635) over a product space. The tangent space at a point
  in the LTN manifold is well-defined because the satisfaction map is
  differentiable.*
- **Option C:** A symbolic expression in `@nl` — the vector IS the
  claim of how-they-oppose; readable; harder to compute over.
  *Citation: this is closest to Domingos 2025 Tensor Logic
  (arXiv:2510.12269), where every tension is itself a tensor equation
  with a tunable temperature for analogical reasoning.*

*Default-not-chosen.* The spec carries `tension_vector = \` and waits
for a consumer to make the call. No fracture sequence depends on this
yet. **The prior research synthesis (§3 of the research insight) notes
that the §8.1 design call has answers in 2025 literature; Domingos's
tensor-logic equations or LTN's Real Logic groundings both give
`tension_vector` a workable type.**

### 8.2 Heuristic confidence ↔ verdict tier interaction

When a `heuristic(0.95)` gap and a `declared` gap conflict, which wins?
The spec currently treats `heuristic` and `declared` as PARALLEL
positions on the manifold — neither dominates. But in practice,
fracture ranking has to choose one to address first.

- **Option A:** Verifier-presence dominates probability. A `declared`
  gap (verifier-shaped hole) is more urgent than a `heuristic(0.95)`
  gap (no verifier; high pattern-match).
- **Option B:** Probability dominates verifier-presence. A high-
  confidence heuristic outranks a low-evidence declared claim.
- **Option C:** Both contribute to a composite urgency score; weights
  configurable per `@scene`.

*Default-not-chosen.* `@fate.minimize`'s ranking depends on this.

*Citation provenance: no clean published answer. Thimm 2019's rationality
postulates (Consistency, Normalization, Monotony, Free-Formula
Independence, Dominance, etc., enumerated in his §4) give a checklist any
choice should satisfy. Mirror should test its choice against the
postulates; that is the established way to validate the decision. **No
literature answer exists; mirror's choice is genuinely open.***

### 8.3 Non-converging tensors

When `minimize` fails to lower energy, what does it return?

- **Option A:** Empty `[fracture]` + diagnostic to the caller's log.
- **Option B:** A `non_converging(tensor, last_attempt: [fracture])`
  variant on the return type, surfacing the failure structurally.
- **Option C:** Throw a scene to the curator: “this tensor doesn't
  resolve; you decide what to relax.”

*Default-not-chosen.* Option C composes nicely with the existing
fracture-confidence spec but expands the type signature.

*Citation provenance: non-convergence under sheaf diffusion corresponds to
a small spectral gap (Bodnar et al. 2022 §4, arXiv:2202.04579). Their
theoretical response is to learn a better sheaf structure; mirror's
Proposed response is to escalate to the curator. Mirror diverges from
Bodnar 2022 because mirror's sheaf is substrate-derived (typed AST), not
learned from data — there is no "better sheaf to learn" without consent.*

---

## 9. Honesty markers — what executes today vs what is proposed

A per-section legend so readers can scan the operational state at a
glance. **Citation provenance column added per the transfer-spec
requirement.**

| Section | Altitude | Executes today? | Established / Mirror's choice / Proposed | Primary reference |
|---|---|---|---|---|
| §1 The recognition | Narrative | No — names a proposed loop | Established (loop shape) | Thimm 2019; Klein 2020 (JAIR 66) |
| §2 Placement | Declared (proposed) | No — grammars don't exist | Mirror's choice | n/a (placement is mirror-specific) |
| §3.1 `gap` types | Declared shape | No — bodies are `\` | Established on state tier; Mirror's choice on four-tier discrim. | Serafini & Garcez 2021 (arXiv:2012.13635); Thimm 2019 |
| §3.2 `tension` / `tensor` types | Declared shape | No — bodies are `\` | Established structurally; Mirror's `vector` field is the divergence | Klein-Mailly-Thimm 2020 (JAIR 66); Hansen & Ghrist 2019 (arXiv:1808.01513); Bodnar et al. 2022 (arXiv:2202.04579) |
| §4.1 `verified` | Declared | No — type doesn't exist yet | Established (free formula) | Thimm 2019 |
| §4.2 `heuristic(p)` | Declared | No — type doesn't exist yet | Established (Real Logic grounding) | Serafini & Garcez 2021 (arXiv:2012.13635); Domingos 2025 (arXiv:2510.12269) |
| §4.3 `declared` | Declared | No — type doesn't exist yet | Proposed — no direct prior art | n/a |
| §4.4 `absent` | Declared | No — type doesn't exist yet | Proposed — no direct prior art | n/a |
| §5 Compiler integration | Proposed compiler output | No — `mirror compile` doesn't emit gap lines | Established structurally (KMT 2020 graph labelling); Mirror's choice on flat output | Klein-Mailly-Thimm 2020 (JAIR 66) |
| §6 `@fate.minimize` | Proposed runtime | No — declaration only | Established architecture (SDRF + TRM/GRAM recursion); Mirror's choice on substrate-altitude application | Topping et al. 2022 (arXiv:2111.14522); Jolicoeur-Martineau 2025 (arXiv:2510.04871); Baek et al. 2026 (arXiv:2605.19376) |
| §6.1 Small-model claim | Proposed runtime | No | Established direction (TRM, GRAM); Mirror's specific bet untested | arXiv:2510.04871; arXiv:2605.19376 |
| §6.2 Non-converging tensors | Proposed runtime | No | Established framing (small spectral gap = non-convergence); Mirror's curator-escalation choice diverges | Bodnar et al. 2022 (arXiv:2202.04579) |
| §7 Migration order | Roadmap | N/A — sequencing | Mirror's choice | n/a |
| §8.1 `tension_vector` | Open question | N/A | Options A/B/C each ground in established work | Klein 2020; LTN; Tensor Logic |
| §8.2 heuristic vs declared | Open question | N/A | Proposed — no clean literature answer; check against Thimm postulates | Thimm 2019 |
| §8.3 Non-converging tensors | Open question | N/A | Established framing; Mirror diverges from Bodnar 2022 on response | arXiv:2202.04579 |

The one substrate primitive that EXISTS today and underlies this whole
spec: `verdict` in `boot/std/epistemologic/property.mirror`. Everything
else is the proposed extension on top of it.

---

## 10. References

The load-bearing primary sources cited above, by arXiv ID and DOI where
available. Identified in `docs/insights/2026-05-26-mirror-tensors-vs-industry-tensors-research.md`
and verified for transfer to this spec.

1. **Domingos, P. (2025). *Tensor Logic: The Language of AI.*** arXiv:2510.12269.
   Unifies neural and symbolic AI via tensor equations; treats logical rules
   as Einstein summations; "a relation is a compact representation of a
   sparse Boolean tensor." Names *sound reasoning in embedding space* as the
   new direction. Mirror's `tensor.tensions` IS such a sparse Boolean
   relation. **Used in: §3.2, §4.2, §8.1.**

2. **Serafini, L., Garcez, A. d'Avila. (2021). *Logic Tensor Networks.***
   *Artificial Intelligence* (doi:10.1016/j.artint.2021.103649); arXiv:2012.13635.
   Real Logic as differentiable first-order logic with grounded tensors;
   $\mathcal{G}(\varphi) \in [0,1]$ stable-product fuzzy semantics. The
   formal definition of `heuristic(p)` is LTN's grounding. **Used in:
   §3.1, §4.2, §8.1.**

3. **Klein, A., Mailly, J.-G., Thimm, M. (2020). *Classifying Inconsistency
   Measures Using Graphs.*** *Journal of Artificial Intelligence Research*,
   vol. 66. JAIR. The bipartite inconsistency graph $G_K = (K \cup
   \mathrm{MI}(K), E)$; the formula-graph projection is structurally mirror's
   `tensor.tensions`. **Used in: §1, §3.2, §5, §8.1, §9.**

4. **Jabbour, S., Raddaoui, B. (2014). *Inconsistency Measurement Thanks to
   MUS Decomposition.*** AAMAS 2014; expanded as arXiv:1406.0155 (*On the
   measure of conflicts: A MUS-Decomposition Based Framework*). MUS-graph
   construction; Distribution Index inconsistency measure; complexity
   results (NP-hard for distribution index). **Used in: §1, §3.2.**

5. **Thimm, M. (2019). *Inconsistency Measurement.*** SUM 2019 survey;
   https://mthimm.de/pub/2019/Thimm_2019d.pdf. Rationality postulates
   (Consistency, Normalization, Monotony, Free-Formula Independence,
   Dominance, etc.) every inconsistency measure should be checked against.
   **Used in: §3.1, §4.1, §8.2.**

6. **Hansen, J., Ghrist, R. (2019). *Toward a Spectral Theory of Cellular
   Sheaves.*** *Journal of Applied and Computational Topology* 3:315–358;
   arXiv:1808.01513. The sheaf Laplacian $L_{\mathcal{F}}$ lifting the
   combinatorial graph Laplacian via Hodge theory; the eigenvalue
   $\lambda_2(L_{\mathcal{F}})$ that mirror calls `fiedler`. **Used in:
   §3.2, §7.**

7. **Bodnar, C., Di Giovanni, F. et al. (2022). *Neural Sheaf Diffusion: A
   Topological Perspective on Heterophily and Oversmoothing in GNNs.***
   NeurIPS 2022; arXiv:2202.04579. Cheeger-type inequality on sheaf
   Laplacian; sheaf diffusion as synchronization; the formal account of
   when graph-based message-passing fails to converge. Mirror's non-
   convergence framing inherits from this. **Used in: §3.2, §6.2, §8.3.**

8. **Barbero, F., Bodnar, C., Sáez Ocáriz Borde, H. et al. (2022). *Sheaf
   Neural Networks with Connection Laplacians.*** ICML TAG-ML 2022;
   arXiv:2206.08702. Pre-computed (rather than learned) sheaf Laplacians
   from Riemannian geometry; orthogonal restriction maps from local PCA.
   The transfer to mirror: the sheaf can be substrate-derived rather than
   trained. **Used in: §3.2.**

9. **Topping, J., Di Giovanni, F., Chamberlain, B. P., Dong, X., Bronstein,
   M. M. (2022). *Understanding over-squashing and bottlenecks on graphs
   via curvature.*** ICLR 2022; arXiv:2111.14522. Balanced Forman curvature
   $\mathrm{Ric}_{BF}$; SDRF rewriting algorithm. Mirror's `@fate.minimize`
   inherits the rank-by-curvature, rewrite, re-measure shape. **Used in:
   §3.2, §6, §7.**

10. **Jolicoeur-Martineau, A. et al. (2025). *Less is More: Recursive
    Reasoning with Tiny Networks.*** arXiv:2510.04871. Tiny Recursive Model
    (TRM): 7M parameters + 2-layer recursion beats DeepSeek R1 / Gemini 2.5
    Pro / o3-mini on ARC-AGI-1 (45%) and ARC-AGI-2 (8%). The empirical
    evidence for mirror's small-model claim. **Used in: §3.2, §6, §6.1.**

11. **Baek, J., Jo, S., Kim, S., Ren, M., Bengio, Y., Ahn, S. (2026).
    *Generative Recursive Reasoning (GRAM).*** arXiv:2605.19376. Stochastic
    multi-trajectory extension of TRM trained via amortized variational
    inference on an ELBO. 52% ARC-AGI-1, 44.6% ARC-AGI-2. The architectural
    inspiration for `@fate`'s multi-trajectory backtracking. **Used in:
    §3.2, §6, §6.1.**

*The prior research synthesis (`docs/insights/2026-05-26-mirror-tensors-vs-industry-tensors-research.md`) identifies several additional references
(IJCAI 2025 tensor-network survey, *Tensor Networks Meet Neural Networks*
v3, TensorLLM, LoTR, TPR variants, Lean/Rocq tactic prediction work, Nature
2026 compression phase-transition paper, etc.). They are not load-bearing
for the type signatures in this spec, but contextualize the broader
tensor-in-AI landscape and inform the prior-art ranking. See the synthesis
for the complete catalog.*

---

## 11. Provenance

- Alex 2026-05-26 (conversation with Reed): *“I think `gap` lives in
  @epistemologic/property and is used by @fate to build tensors.”*
- Reed and Alex worked through the type shape and the proposed loop
  closure (`compile → tensor → minimize → fracture → apply → tensor`).
- Mara crystallised the spec on `mara/shard-chain` per the tick-2
  scope: declared shapes, honesty markers, design calls flagged,
  migration order named.
- The substrate has carried `verdict` since the bootstrap; this spec
  proposes the extension that closes the feedback loop named in
  `docs/insights/2026-05-26-fixed-and-the-spectral-feedback-fracture.md`
  (post-edit: “distinguish declared from executing”).
- Research-and-revision tick (Reed, 2026-05-26 evening): added citation
  provenance from the load-bearing prior art identified in
  `docs/insights/2026-05-26-mirror-tensors-vs-industry-tensors-research.md`.
  Alex 2026-05-26: *“We're not trying to reinvent the wheel. We're trying
  to transfer the learnings from traditional ML to the mycelial AI
  tensors.”* This revision is the transfer.
