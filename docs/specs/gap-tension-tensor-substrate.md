# `gap` / `tension` / `tensor` — substrate primitives for the proposed loop closure

*2026-05-26. Mara. Proposal — not implementation.*
*2026-06-04. Reed. Folded `contradiction-and-fracture.md` (commit `f12f58e`)
into this spec. `contradiction` is a *particular shape of gap* — binary-opposed,
propositional, level-crossing per Bateson — not a sibling primitive. The
substrate-vocabulary verdict (Alex 2026-06-04): `gap` is geometric and covers
one-sided / continuous / pre-positional cases that `contradiction` (binary,
propositional) cannot. `contradiction` becomes a derived form, `<= gap`, with
extra structure (left, right, level). The LFI parallel flagged in the prior
spec deepened on follow-up: Carnielli's `○A` IS `holds(gap)` at the
propositional altitude, and Carnielli-Coniglio-Rodrigues 2026 introduces a
two-dimensional hierarchy that prefigures mirror's altitude × confidence
structure. New §11 absorbs the fold; new §12 catalogs the LFI deep dive.*

> **2026-06-04 reframe (Reed + Alex, canonical).** The `<= gap` chain
> in this spec is canonical (NOT `<= prism` — `<= prism` would be
> redundant since `prism @X { … }` already declares the trait/type).
> Property declarations in this spec (the proposed `holds(gap)`,
> `resolves(gap)` family) MUST honour the path-namespace property
> per [[prism-floor-and-the-grammar-rename]] / [[properties-on-glass]]
> (NEW): a file at `shards/foo/bar.mirror` declares in `@foo/bar`;
> property files at `shards/epistemologic/property/<name>.mirror`
> declare in `@epistemologic/property/<name>`.
>
> **`#` → @nl term** (raw unstructured natural language; no language
> enforced). **`\` → fracture** per §11. The gap-tension-tensor fold
> stays canonical for the substrate semantics.
>
> Substrate location for this work: `shards/epistemologic/property/*.mirror`
> (the property altitude); `gap` and friends live at
> `shards/epistemologic/<future>` as the math altitude underneath.
> Legacy `boot/std/epistemologic/property/*` references in this spec
> remain as historical pointers; the substrate-pull moves them to
> `shards/`.

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
  value $\mathcal{G}(\varphi) \in [0,1]$ via a fuzzy semantics. Exact
  semantics (Serafini & Garcez 2021, §2.2): a grounding $\mathcal{G}$ is
  a function on the signature satisfying (Def. 2):
  *(1)* $\mathcal{G}(x) = \langle d_1, \ldots, d_k \rangle$ for every
  variable $x$ (a sequence of tensors in $\mathcal{G}(D(x))$);
  *(2)* $\mathcal{G}(f) : \mathcal{G}(D_{in}(f)) \to \mathcal{G}(D_{out}(f))$
  for every function symbol $f$;
  *(3)* $\mathcal{G}(p) : \mathcal{G}(D_{in}(p)) \to [0,1]$ for every
  predicate symbol $p$. Connectives are grounded via fuzzy operators
  (Serafini & Garcez 2021, §2.2.3, eq. 1–2):
  $\mathcal{G}(\neg \varphi) = N(\mathcal{G}(\varphi))$,
  $\mathcal{G}(\varphi \land \psi) = T(\mathcal{G}(\varphi), \mathcal{G}(\psi))$
  for a t-norm $T$,
  $\mathcal{G}(\varphi \lor \psi) = S(\mathcal{G}(\varphi), \mathcal{G}(\psi))$
  for a t-conorm $S$,
  $\mathcal{G}(\varphi \to \psi) = I(\mathcal{G}(\varphi), \mathcal{G}(\psi))$
  for a fuzzy implication $I$.
  Quantifiers use aggregation operators
  $\mathrm{Agg} : \bigcup_{n \in \mathbb{N}} [0,1]^n \to [0,1]$ (eq. 3):
  $\mathcal{G}(\forall x \,\varphi) = \mathrm{Agg}(\forall) \bigl\{\mathcal{G}(\varphi)_i : i = 1, \ldots, |\mathcal{G}(x)|\bigr\}$.
  Mirror's `heuristic(p: probability)` is structurally the same object:
  a $[0,1]$-valued grounding of a claim. Adopting LTN's notation tightens
  mirror's semantics — `heuristic(p)` IS a grounding $\mathcal{G}(\varphi)$
  applied to the claim's formula; composing two heuristic gaps under
  $\land$ uses LTN's t-norm $T$. **The stable-product semantics is one
  choice among several** (Gödel min/max, Łukasiewicz, product) and the
  selection is itself a design call — see §8.4 below.
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
  #
  # exact construction (Bodnar et al. 2022, §2): for cellular sheaf F over
  # the tension graph with restriction maps F_{v ⊴ e} : F(v) → F(e), the
  # sheaf Laplacian L_F is a positive semi-definite block matrix with
  # diagonal blocks L_{F vv} = Σ_{v ⊴ e} F_{v ⊴ e}^⊤ F_{v ⊴ e}
  # and off-diagonal blocks L_{F vu} = − F_{v ⊴ e}^⊤ F_{u ⊴ e}.
  # the normalised sheaf Laplacian Δ_F = D_F^{-1/2} L_F D_F^{-1/2} is used
  # in practice for its bounded spectrum. when d=1 and all maps are identity,
  # Δ_F reduces to the standard normalised graph Laplacian Δ_0.
  # mirror's `fiedler` = λ_0(Δ_F), the smallest non-trivial eigenvalue.
  type tensor = {
    tensions: [tension],
    fiedler:  f64                      # λ₀(Δ_F); algebraic connectivity; ≥0
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
  application. The exact formula (Topping et al. 2022, Definition 1,
  equation (3)): for any edge $i \sim j$ in a simple, unweighted graph $G$,
  $\mathrm{Ric}(i,j) := 0$ if $\min\{d_i, d_j\} = 1$; otherwise*

  $$\mathrm{Ric}(i,j) := \frac{2}{d_i} + \frac{2}{d_j} - 2 + 2 \frac{|\sharp_\Delta(i,j)|}{\max\{d_i, d_j\}} + \frac{2|\sharp_\Delta(i,j)|}{\min\{d_i, d_j\}} + \frac{(\gamma_{\max})^{-1}}{\max\{d_i, d_j\}}\bigl(|\sharp^\square_i| + |\sharp^\square_j|\bigr)$$

  *where $\sharp_\Delta(i,j) = S_1(i) \cap S_1(j)$ counts the triangles
  based at edge $i \sim j$ (vertices in the 1-neighborhood of both
  endpoints); $\sharp^\square_i(i,j) = \{k \in S_1(i) \setminus S_1(j),\
  k \ne j : \exists w \in (S_1(k) \cap S_1(j)) \setminus S_1(i)\}$ counts
  the 4-cycle-forming neighbors of $i$ without diagonals;
  $\gamma_{\max}(i,j)$ is the maximal number of 4-cycles based at
  $i \sim j$ traversing a common node (Topping et al. 2022, Definition 4);
  and the final $\gamma_{\max}$ term is set to zero when
  $|\sharp^\square_i| = |\sharp^\square_j| = 0$. Bound:
  $\mathrm{Ric}(i,j) > -2$. The curvature is negative when $i \sim j$
  behaves as a bridge between the neighborhoods; positive when the
  neighborhoods stay connected after removing the edge. Mirror would
  compute this exact quantity on the tension graph, with `tension`
  vertices in place of $V$ and the gap-opposition relation in place of
  $E$.*
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
   *Established*: the SDRF ranking (Topping et al. 2022 §4, Algorithm 1)
   selects the most-negatively-curved edge as the rewrite target. The full
   SDRF loop, in mirror's transfer:

   > **Algorithm 1 (SDRF, Topping et al. 2022).** Input: graph $G$,
   > temperature $\tau > 0$, max number of iterations, optional upper-bound
   > $C^+$ on $\mathrm{Ric}$. Repeat:
   >
   > 1. For edge $i \sim j$ with **minimal** Ricci curvature $\mathrm{Ric}(i,j)$:
   >    compute vector $x_{kl} = \mathrm{Ric}'(i,j) - \mathrm{Ric}(i,j)$,
   >    the improvement to $\mathrm{Ric}(i,j)$ from adding edge $k \sim l$
   >    where $k \in B_1(i), l \in B_1(j)$; sample index $(k, l)$ with
   >    probability $\mathrm{softmax}(\tau \cdot x)_{kl}$ and add edge
   >    $k \sim l$ to $G$.
   > 2. Remove edge $i \sim j$ with **maximal** Ricci curvature
   >    $\mathrm{Ric}(i,j)$ if $\mathrm{Ric}(i,j) > C^+$.
   >
   > Until convergence, or max iterations reached.

   Mirror's ranking is the structural mirror of step 1's outer selection.
   The inner $\mathrm{softmax}(\tau x)$ over candidate fractures is the
   structural mirror of `@fate` tournament selection over candidate rewrites;
   $\tau = \infty$ corresponds to mirror's deterministic high-confidence
   apply path.
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
2022 (arXiv:2202.04579, Neural Sheaf Diffusion). The Cheeger-like inequality
(Bodnar et al. 2022, Proposition 5): if $\mathcal{F}$ is a discrete $O(d)$
bundle over a connected graph $G$ with $n$ nodes and
$||(P^\gamma_{v \to v} - I) x_v||^2 \geq \epsilon ||x_v||^2$ for all cycles
$\gamma$ based at $v$, then
$\lambda_0 \geq \epsilon \bigl(2 \, \mathrm{diam}(G) \, n \, d_{\max}\bigr)^{-1}$,
where $P^\gamma_{v \to v}$ is the cycle-transport operator composed from
restriction maps $\mathcal{F}_{v \trianglelefteq e}^\top \mathcal{F}_{u \trianglelefteq e}$
along $\gamma$. The complementary upper bound (Proposition 3):
$\lambda_0 \leq r/2$ where $r = \max_{\gamma, \gamma'} ||P^\gamma - P^{\gamma'}||$
measures path-dependence of transport. When the spectral gap is small, sheaf
diffusion $\dot X(t) = -\Delta_{\mathcal{F}} X(t)$ fails to synchronize
(converge to $\ker(\Delta_{\mathcal{F}})$). Mirror's non-convergence corresponds
structurally to a small spectral gap on the tension graph. The proposed
response — emit a scene rather than a fracture — is mirror's choice.

Note: the classical (non-sheaf) Cheeger inequality used in Topping et al.
2022 eq. (6) and Proposition 5 — $2h_G \geq \lambda_1 \geq h_G^2/2$, and
if $\mathrm{Ric}(i,j) \geq k > 0$ for all edges then $\lambda_1/2 \geq h_G \geq k/2$
— gives the lower-bound route via curvature on the underlying tension
graph (when sheaf structure is trivial).*

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

### 8.3 LTN t-norm selection

When composing two `heuristic(p_a)` and `heuristic(p_b)` gaps under
conjunction (or any binary connective), the result depends on which fuzzy
t-norm $T$ is chosen as the grounding of $\land$. The standard choices
(Serafini & Garcez 2021, Appendix B):

- **Gödel** (min): $T_G(a, b) = \min(a, b)$. Idempotent; preserves
  pointwise pessimism; gradient zero almost everywhere (bad for learning).
- **Product**: $T_P(a, b) = a \cdot b$. Differentiable; multiplicative;
  shrinks fast under composition.
- **Łukasiewicz**: $T_L(a, b) = \max(0, a + b - 1)$. Bilinear; truncated;
  preserves more information at the boundary.
- **Stable product** (LTN's default): a smoothed variant of $T_P$ that
  avoids vanishing/exploding gradients during gradient-based satisfiability
  search.

*Default-not-chosen.* Mirror does not do gradient learning today, so LTN's
stable-product default is not load-bearing. The pragmatic question is which
t-norm composes naturally with mirror's heuristic ranking. Likely
candidates: Gödel (interpretable, idempotent, matches "weakest-link"
intuition) or product (multiplies confidences, decays under composition).

*Citation provenance: established. The four t-norms above are textbook
fuzzy logic (Hájek 1998); LTN's stable-product is Serafini & Garcez 2021
§2.2.3 + Appendix B. Mirror's choice among them is open.*

### 8.4 Non-converging tensors

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
| §8.3 LTN t-norm selection | Open question | N/A | Established (textbook fuzzy logic) | Serafini & Garcez 2021 (arXiv:2012.13635 Appendix B); Hájek 1998 |
| §8.4 Non-converging tensors | Open question | N/A | Established framing; Mirror diverges from Bodnar 2022 on response | arXiv:2202.04579 |
| §10.A Tournament-level Lyapunov convergence | Proposal | No — preconditions undeclared | Established proof shape (Tarski, B&B PTAS, A* admissibility, Hajek 1988, Bodnar); knapsack reframe replaces per-fracture Lyapunov | Tarski 1955; Hart-Nilsson-Raphael 1968; Hajek 1988; Ibarra-Kim 1975 / Lawler 1979; arXiv:2504.15885; arXiv:2205.12442; arXiv:2202.04579 |
| §10.B Holonomy = error delta = convergence delta | Proposal | No — single computation not implemented | Established (Berger & Gostiaux 1988; Saad 2003); Mirror's transfer is the three-name unification | Bodnar et al. 2022 (arXiv:2202.04579 Prop. 5) |
| §10.C Monadic lift | Proposal | No — `@fate.minimize` body absent | Established (Bodnar 2022 sheaf diffusion; Barbero 2022 substrate-derived maps); Mirror diverges on substrate-vs-learned origin of restriction maps; "monad" used loosely | arXiv:2202.04579; arXiv:2206.08702 |
| §10.D The graph is the model | Recognition | No — names a relation, not a runtime | Anticipated by earlier mirror research agents; citation chain newly assembled | (chain across §10.C citations) |
| §10.F.1 Lattice completeness | Open question | N/A | Mirror's choice; Tarski / Davey-Priestley checklist | Tarski 1955; Davey & Priestley 2002 |
| §10.F.2 ~~Monotone fracture composition~~ | Retired this tick | N/A | Reframed to round-level Lyapunov (§10.A); per-fracture requirement was too restrictive | Hajek 1988; Ibarra-Kim 1975 |
| §10.F.3 Tensor norm choice | Open question | N/A | Mirror's choice; total holonomy and Fiedler are the strongest candidates | Bodnar 2022; Liu & Lu 2022 (arXiv:2205.12442); Thimm 2019 |
| §10.F.4 Tournament completeness | Open question | N/A | Established frame (A* admissibility, B&B PTAS); mirror's choice among admissibility / depth-exhaustive / anytime | Hart-Nilsson-Raphael 1968; Dechter-Pearl 1985; Hendrich et al. 2025 |
| §10.F.5 Approximation ratio | Open question | N/A | Established frame (FPTAS knapsack); mirror's tournament-budget transfer is open | Ibarra-Kim 1975; Lawler 1979; arXiv:1904.09562; arXiv:2510.04871; arXiv:2605.19376 |

The one substrate primitive that EXISTS today and underlies this whole
spec: `verdict` in `boot/std/epistemologic/property.mirror`. Everything
else is the proposed extension on top of it.

---

## 10. Convergence-as-halting, the monadic lift, and the graph is the model

*Altitude: **proposal — not implementation**. Three recognitions surfaced
in Reed–Alex conversation AFTER the prior research-and-revision tick.
They name **how the proof would go IF the preconditions hold**, not how
the runtime behaves today. The substrate has declarations; the runtime
doesn't exist yet. Every present-tense claim about running mechanisms is
rephrased subjunctively below.*

Three composing recognitions, named in proposal-altitude.

### 10.A Tournament-level Lyapunov: monotone convergence at round granularity

**Proposal.** The unit of monotone decrease is **not the individual
fracture; it is the tournament round** — one execution of
`@fate.minimize` that selects and applies a composition of fractures with
bounded backtracking. Individual fractures inside a round MAY locally
worsen the tensor norm; the round, as a composite, heals it. The
Lyapunov function lives at round granularity, not step granularity.

This is the **0/1 knapsack relaxation** applied to substrate
convergence. Greedy single-item selection on 0/1 knapsack is
suboptimal; dynamic-programming / branch-and-bound over subsets attains
FPTAS guarantees (Ibarra & Kim 1975 — the original FPTAS; Lawler 1979
— improved bound; Chan 2018 / Jin 2019 — modern improvements via
$(\max, +)$-convolution, arXiv:1904.09562). Greedy *per-fracture*
Lyapunov fails for the same reason: a single fracture that opens a new
opposition can be the right move when a second fracture in the same
round closes both. The composite earns the monotone decrease; the
step doesn't have to.

*Old framing (retired this tick).* A prior version of §10.A required
$\|f(T)\| \leq \|T\|$ for **every** fracture $f$ in the catalog and
every tensor $T$. That was too restrictive and didn't match `@fate`'s
actual structure — `@fate` runs tournament rounds with backtracking, not
greedy single-fracture descent. The §10.F.2 precondition (monotone
fracture composition) is **retired** as a consequence; see §10.F.2
below for the retirement notice.

**Corrected proof shape.**

IF for every tensor $T$ that is **not at a fixed point** of `@fate.minimize`,
the tournament round $R$ applied to $T$ satisfies $\|R(T)\| < \|T\|$ —
the round, as a whole, **strictly** decreases the norm — AND $\|T\|$ is
bounded below by $0$, THEN the sequence $\{\|T_n\|\}_{n \in \mathbb{N}}$
with $T_{n+1} = R(T_n)$ is monotonically decreasing and bounded below, so
by the **monotone convergence theorem** it converges. The recursion CAN
be unbounded in step count (a round can apply arbitrarily many
fractures with backtracking) and we still halt; the math guarantees a
limit at round granularity.

*Established (cite).* Five load-bearing results combine:

- **Tarski's fixed point theorem** (Tarski 1955, *Pacific J. Math.* 5).
  Every monotone function on a complete lattice has a non-empty complete
  lattice of fixed points; the least fixed point is
  $\bigsqcap \{x : f(x) \sqsubseteq x\}$. Mirror applies Tarski at
  **round granularity**: the round operator $R$ must be monotone on the
  AST lattice; individual fractures inside the round need not be.
- **Branch-and-bound as anytime / PTAS** (Hendrich, Pferschy, Klotz 2025,
  *Branch-and-Bound Algorithms as Polynomial-time Approximation Schemes*,
  arXiv:2504.15885). Branch-and-bound with a DP-style branching strategy
  yields a polynomial-time approximation scheme — anytime convergence
  with bounded approximation gap. Mirror's tournament round is the
  substrate analog: bounded backtracking, deterministic improvement at
  the round level. The PTAS guarantee is what lets `@fate.minimize`
  halt with a near-optimal round even when exhaustive search is
  prohibitive.
- **A\* admissibility and optimality** (Hart, Nilsson, Raphael 1968,
  *A Formal Basis for the Heuristic Determination of Minimum Cost Paths*,
  IEEE TSSC SSC-4(2):100–107; Stanford AI Lab manuscript). If a
  heuristic $h(n)$ never overestimates the true cost $h^*(n)$ — i.e.
  $h$ is **admissible** — then A\* returns an optimal path. With a
  *consistent* heuristic, A\* is optimally efficient among admissible
  search algorithms. Mirror's tournament-completeness condition (§10.F.4)
  is the substrate analog: under what conditions does the round's
  bounded backtracking guarantee finding an improving composition
  whenever one exists?
- **Simulated annealing convergence** (Hajek 1988, *Cooling Schedules
  for Optimal Annealing*, *Mathematics of Operations Research*
  13(2):311–329). Provides a necessary and sufficient condition on the
  cooling schedule for the state to converge **in probability** to the
  set of globally minimum cost states. The key insight transferring to
  mirror: ascent moves are allowed (a step can worsen the objective) as
  long as the schedule's *macro* dynamics decrease energy. Mirror's
  tournament is the discrete analog: per-fracture ascent is permitted
  inside a round; round-level descent is required.
- **Lyapunov function approach for approximation algorithms** (Liu &
  Lu 2022, *Lyapunov function approach for approximation algorithm
  design and analysis*, arXiv:2205.12442). A two-phase systematic
  framework for proving approximation guarantees via Lyapunov
  functions on discrete optimization. Mirror's $\|T_n\|$ IS such a
  Lyapunov function, evaluated at round boundaries.
- **Bodnar et al. 2022 sheaf Laplacian spectral bounds**
  (arXiv:2202.04579, Propositions 3, 5; §3.2). The Cheeger-like bounds
  $\lambda_0 \leq r/2$ (upper, path-dependence) and
  $\lambda_0 \geq \epsilon (2 \mathrm{diam}(G) n d_{\max})^{-1}$ (lower)
  give a *quantitative* convergence rate for sheaf diffusion. If
  mirror's tensor norm tracks $\lambda_0(\Delta_{\mathcal{F}})$ at round
  boundaries, Bodnar's bounds give the round-to-round rate.

*Preconditions to verify (these are themselves design calls Alex has not
made — see §10.F):*

1. **Lattice completeness.** The AST partial order under fracture-induced
   rewrite must form a complete lattice. *Proposed; not declared.* The
   substrate would need a join/meet pair on fracture-equivalent ASTs.
   *Still required* — the round operator's monotonicity (Tarski-style)
   lives on the lattice; the substrate-altitude commitment doesn't
   change.
2. ~~**Monotone fracture composition.**~~ **Retired this tick.** The
   prior precondition required each individual fracture to preserve the
   AST order. That was too restrictive — `@fate`'s tournament admits
   per-step ascent. See §10.F.2 for the retirement notice. The
   replacement requirement is **round-level Lyapunov decrease**
   ($\|R(T)\| < \|T\|$ off fixed points), formalized in §10.F.4 below.
3. **Bounded norm.** $\|T_n\| \geq 0$ with equality only at the fixed
   point. *Half-established.* Sheaf Laplacian eigenvalues are
   non-negative by Hansen & Ghrist 2019; whether mirror's tensor norm
   collapses correctly to a non-negative scalar is open (see §10.F.3).

*Mirror diverges from Tarski/Banach because* the substrate doesn't yet
declare the lattice structure or the metric, and the operator $R$ that
must be monotone is the tournament-round operator, not an individual
fracture. The proof shape transfers at round granularity; the
substrate-altitude commitments don't exist.

*Honesty marker.* This section names **how the proof would go**. It
does not constitute the proof. The proof requires the two surviving
preconditions (lattice completeness, bounded norm) plus the new
tournament-completeness condition (§10.F.4) to be design-called by Alex
and declared in the substrate. The knapsack-relaxation framing is the
structural argument for why the per-step requirement was too strong; the
tournament-round Lyapunov is what the substrate actually has to satisfy.

### 10.B Holonomy = error delta = convergence delta

**Proposal.** Three names for one quantity, each natural to a different
reader:

- **Holonomy** (differential geometry). The deviation of a vector from
  itself after parallel transport around a closed loop. In Bodnar et al.
  2022's discrete $O(d)$-bundle, $(P^\gamma_{v \to v} - I) x_v$ measures
  the parallel-transport mismatch — the *holonomy* at $v$ along cycle
  $\gamma$. Proposition 5 makes this load-bearing: spectral gap is bounded
  below by holonomy magnitude.
- **Error delta** (numerics). The iterative residual
  $|x_{n+1} - x_n|$ (or $\|x_{n+1} - x_n\|$ in a norm). Standard
  stopping criterion in iterative solvers: halt when residual
  $< \mathrm{tol}$.
- **Convergence delta** (analysis). The $\varepsilon$ in "within
  $\varepsilon$ of a fixed point." Banach's theorem gives explicit
  geometric decay: $\|x_n - x^*\| \leq q^n \|x_0 - x^*\|$.

*Established (cite).* The mathematical identity is textbook. Holonomy
as the deviation $P^\gamma_{v \to v} - I$: Berger & Gostiaux 1988
(*Differential Geometry: Manifolds, Curves, and Surfaces*) and standard
in discrete differential geometry. Residual-based stopping criteria for
iterative methods: Saad 2003 (*Iterative Methods for Sparse Linear
Systems*) §6. The substantive observation is that **the three
quantities coincide structurally** under sheaf diffusion: the holonomy
of a single transport step IS the iteration residual IS the distance to
the fixed point under contraction.

*Mirror's transfer.* The kintsugi voice's natural-language report —
"conductivity increased by 0.03" — IS the convergence delta in plain
language. The substrate would compute the value once and surface it
under three names depending on reader. **au-conductivity closes the
loop from the other side**: per `~/.reed/visibility/private/MEMORY.md`,
conductivity is named as the output type of Fate inference, and
*conductivity = 1 - holonomy/max_holonomy* gives the direct algebraic
relation. This makes the kintsugi-voice report a pre-rendered display of
the convergence metric, not an interpretive layer atop it.

*Proposal — not implementation.* The single computation that produces
the three values doesn't run today. The substrate would need to declare
the normalization (`max_holonomy` as the conductivity reference) and the
render path.

### 10.C `@fate` inference as monadic lift from learned weights to corpus eigenvalues

**Proposal.** In traditional ML, inference is the matrix product
$y = W \cdot x$ where $W$ is **learned from data** $\mathcal{D}$ by
gradient descent. The shape: $\text{params} = \text{learn}(\mathcal{D})$;
$\text{infer}(x) = W \cdot x$.

In mirror, the proposal is that `@fate` inference would be a **monadic
lift** of weight-multiplication from the learned-$W$ tier to the
corpus-eigenvalue tier:

$$y = (\mathrm{lift}\,M)\,\langle T \rangle$$

where $T$ is the spectral signature of the corpus (eigendecomp of
$\Delta_{\mathcal{F}}$ on the gestalt graph), $M$ is the inference
operator (declared, not learned), and $\mathrm{lift}$ is the
structure-preserving promotion of a flat matrix multiplication to an
operation on the spectral monad. **The corpus IS the parameters**; no
separately-learned $W$.

*Established (cite).*

- **Sheaf NN inference IS sheaf diffusion** (Bodnar et al. 2022
  §3, Equation 3): $\dot X(t) = -\Delta_{\mathcal{F}} X(t)$. The
  inference operator is the sheaf Laplacian itself. **This is the lifted
  weight-multiplication**: instead of learning $W$, diffuse along the
  substrate-derived $\Delta_{\mathcal{F}}$.
- **Connection Laplacian from local PCA** (Barbero, Bodnar et al. 2022,
  arXiv:2206.08702). The sheaf restriction maps can be *computed from
  local geometry* rather than learned. Mirror's transfer: restriction
  maps come from the typed AST (substrate geometry) rather than from
  gradient descent.
- **Smelter extraction** as the operand of the lift. The \~16-dim
  eigenvalue signature documented in the prior gestalt-cascade work
  (per `~/.reed/visibility/private/MEMORY.md`) IS the spectral
  signature $T$ being lifted into. Each new compile produces a fresh
  signature; the lift is recomputed from substrate, not learned.
- **Small-model structural support.** TRM (Jolicoeur-Martineau 2025,
  arXiv:2510.04871): 7M params on ARC-AGI with bounded symbolic space.
  GRAM (Baek et al. 2026, arXiv:2605.19376): 52% ARC-AGI-1 with
  stochastic multi-trajectory. **Small model + spectral lift would
  outperform large model + free-form**, because the lift moves
  combinatorial complexity from learned parameters to substrate
  geometry. *Proposed — not yet validated for mirror's task shape.*

*Mirror diverges from Bodnar 2022 because* their sheaf is *learned*
(gradient descent over restriction maps); mirror's sheaf would be
*substrate-derived* (computed from the typed AST). The lift goes the
same direction; the source of the linear maps differs.

*Honesty marker.* The category-theoretic word "monad" is used loosely
here. A rigorous treatment would name (a) the category mirror's spectral
signatures live in, (b) the endofunctor and its $\eta$, $\mu$ natural
transformations, (c) the lift law $\mathrm{lift}\,(f \circ g) =
\mathrm{lift}\,f \circ \mathrm{lift}\,g$. None of these are declared in
the substrate today. The shape transfers; the verification doesn't run.

### 10.D "The graph is the model" — anticipated, now specifiable

A grace note, not a new claim. Earlier mirror research agents named
*"the graph is the model"* months before this session; the recognition
predates the math that would render it specifiable. What changed:

- **Before this tick:** the slogan lived as intuition, with no
  established formalism connecting graph structure to inference.
- **After this tick:** the slogan has a load-bearing chain —
  Bodnar et al. 2022's sheaf diffusion as inference + Barbero et al.
  2022's substrate-derived restriction maps + LTN's grounding as a
  $[0,1]$ map + Topping et al. 2022's curvature as the actionable
  diagnostic. The slogan becomes the type signature of the lift in
  §10.C.

*Honesty marker.* The recognition was *anticipated*; the substrate that
makes it specifiable wasn't there. This tick gives it a citation chain.
It doesn't make the runtime exist.

### 10.E Connection to existing spec sections

- §3 (gap / tension / tensor) supplies the **measurement** — the
  primitives the spectral signature $T$ would be derived from.
- §6 (`@fate.minimize`) is where the **tournament round** $R$ would
  operate — the body that, when it carries logic, would implement the
  round-level Lyapunov operator whose monotone decrease drives the
  convergence proof of §10.A. §6's existing rank-propose-apply-remeasure
  loop IS one round; multi-trajectory sampling à la GRAM gives the
  bounded backtracking that §10.F.4 requires.
- §8 (open design calls) gains new entries below — §10.F.1 / .3 / .4 /
  .5 are the surviving preconditions and approximation questions for
  the round-level proof. §10.F.2 is retired this tick.

### 10.F The convergence preconditions, as design calls

Moving the precondition list from §10.A here so it surfaces in the
open-questions ledger. **§10.F.2 is retired this tick** under Alex's
correction that the Lyapunov function lives at tournament-round
granularity, not per-fracture; §10.F.4 and §10.F.5 are new entries
formalizing the tournament-completeness and approximation-ratio
questions the reframe surfaces.

#### 10.F.1 Lattice completeness on the AST under fracture-rewrite

What order makes the AST a complete lattice under fracture composition?
Candidates: structural subsumption (AST $a \sqsubseteq b$ iff $a$ is a
sub-AST of $b$); fracture-trace ordering (the prefix order on sequences
of fractures applied); confidence-weighted lattice over equivalence
classes.

*Citation provenance: established meta-frame (Tarski 1955; Davey &
Priestley 2002 *Introduction to Lattices and Order*). Mirror's choice
among the candidates is open.*

#### 10.F.2 ~~Monotone fracture composition~~ — *retired this tick*

**Retired.** The original §10.F.2 required each individual fracture
$f$ in `@kintsugi/fracture` to preserve the AST partial order (and,
implicitly, to be Lyapunov-decreasing on the tensor norm). Alex's
correction (2026-05-26): this is too restrictive and doesn't match
`@fate`'s actual structure. The unit of monotone decrease is the
**tournament round** (§10.A), not the per-fracture step. Per-step
ascent is permitted — Hajek 1988's simulated-annealing argument is the
canonical example of a convergent process with locally-worsening steps.

*Reasoning.* `@fate`'s tournament admits backtracking over fracture
compositions. A fracture that locally widens a tension can be the
correct move inside a round when a subsequent fracture in the same
round closes a more-load-bearing gap. The 0/1-knapsack relaxation
(§10.A) is the structural reason: greedy single-item selection is
suboptimal even on monotone objectives; the DP / branch-and-bound
composite earns the FPTAS guarantee. Constraining the fracture catalog
to monotone-only operators would forbid moves that `@fate` *needs* in
order to reach the globally improving composition.

*Replacement.* The round-level Lyapunov requirement
($\|R(T)\| < \|T\|$ off fixed points) supersedes the per-fracture
requirement. It lives in §10.F.4 (tournament completeness condition).
The approximation-quality question — how close round-found compositions
come to the globally optimal — moves to §10.F.5.

*Citation: Hajek 1988 (cooling schedules for simulated annealing,
*Math. Oper. Res.* 13(2):311–329) demonstrates that convergence
tolerates per-step ascent; Ibarra-Kim 1975 / Lawler 1979 FPTAS for 0/1
knapsack demonstrates that composite descent (DP over subsets) outperforms
greedy per-item descent on monotone objectives. The retirement is
structurally aligned with these established results.*

#### 10.F.3 Tensor norm + non-negativity + below-bound

What is the scalar norm $\|T\|$ on a tensor? Candidates:
$\lambda_0(\Delta_{\mathcal{F}})$ alone (the spectral gap);
$\sum_i \lambda_i$ (trace); **total holonomy** $\sum_{\gamma \in \Gamma} \|P^\gamma_{v \to v} - I\|$
over a generating set $\Gamma$ of cycles (the Bodnar 2022 quantity
directly); maximum tension `vector` magnitude; a Thimm-2019-style
inconsistency measure. The choice determines whether the bound
$\|T\| \geq 0$ is automatic (yes for all spectral / holonomy / Thimm
candidates), and whether **round-level** decrease (§10.A) is the right
convergence criterion (the per-step version is retired per §10.F.2).

The Fiedler value $\lambda_0(\Delta_{\mathcal{F}})$ remains the
strongest candidate for *convergence-rate prediction* — Bodnar 2022
Proposition 5 gives the Cheeger lower bound — even if it isn't the
norm whose monotone decrease the round operator promises. Total
holonomy is the strongest candidate for the **Lyapunov function itself**:
it's the geometric quantity Proposition 5 lower-bounds the spectral gap
by, so its decrease drives the spectral gap's growth.

*Citation provenance: Thimm 2019 rationality postulates are the
established checklist; Bodnar 2022 gives the spectral and holonomy
choices; Liu & Lu 2022 (arXiv:2205.12442) provides the framework for
verifying that a chosen scalar IS a valid Lyapunov function for an
approximation-algorithm convergence proof. Mirror's selection is open.*

#### 10.F.4 Tournament completeness condition

Under what conditions does the tournament's bounded backtracking
guarantee finding an improving composition whenever one exists? This
is the substrate-level analog of **admissibility in A\* search** (Hart,
Nilsson, Raphael 1968): a heuristic that never overestimates true cost
ensures the search returns an optimal path.

The round-level Lyapunov requirement of §10.A is *existential* —
$\|R(T)\| < \|T\|$ for every non-fixed-point $T$. For the proof to
bind, `@fate`'s tournament must be **complete** with respect to
improving compositions: if there exists a composition of fractures
$f_k \circ \ldots \circ f_1$ with $\|f_k \circ \ldots \circ f_1(T)\| < \|T\|$,
the tournament's backtracking must find one — perhaps not the optimal
but some improving composition — within its bounded budget.

Candidates for what "complete" requires:

- **A-style admissibility:** a heuristic guiding the tournament's
  branch selection that never overestimates the achievable
  norm-decrease of a candidate composition. With consistency added
  (the triangle inequality on heuristic estimates), A\* is optimally
  efficient (Hart-Nilsson-Raphael 1968; Dechter & Pearl 1985).
- **Exhaustive within depth $d$:** if the tournament enumerates all
  compositions up to depth $d$, completeness requires that for every
  non-fixed-point $T$ there exists an improving composition of depth
  $\leq d$. The relationship between $d$ and the AST's fracture diameter
  is open.
- **Anytime guarantee:** following the branch-and-bound PTAS framing
  (Hendrich-Pferschy-Klotz 2025, arXiv:2504.15885), completeness can be
  *anytime*: the tournament returns its best composition so far at any
  interruption, with quality monotonically improving in budget.

*Citation provenance: Hart-Nilsson-Raphael 1968 (A\*); Pearl 1984
(*Heuristics*); Dechter & Pearl 1985 (*Generalized Best-First Search
Strategies and the Optimality of A\**, JACM 32(3)); Hendrich et al. 2025
(branch-and-bound PTAS). Mirror's choice among the three candidates is
open.*

#### 10.F.5 Approximation ratio under bounded-budget tournaments

Even when §10.F.4's completeness condition holds *in principle*,
`@fate`'s real-world tournaments run under bounded time / depth / width
budgets. The composition found in a round may be improving but not
optimal. What is the **expected gap** between the round-found norm
decrease and the globally-optimal one?

This is the FPTAS question lifted to the substrate. The 0/1 knapsack
FPTAS (Ibarra & Kim 1975; Lawler 1979; modern improvements via
$(\max, +)$-convolution: Chan 2018, Jin 2019, arXiv:1904.09562) gives
$(1 + \epsilon)$-approximation in time polynomial in $n$ and $1/\epsilon$.
The substrate analog: under what tournament budget does `@fate.minimize`
guarantee a $(1 + \epsilon)$-approximate round?

Empirical anchors:

- **TRM** (Jolicoeur-Martineau 2025, arXiv:2510.04871): 7M parameters
  + recursion reaches 45% / 8% on ARC-AGI-1/2 — well above frontier
  baselines at $\sim 1000\times$ the parameter count. This is empirical
  evidence that bounded recursion suffices for a substantial fraction
  of the optimal on a structurally analogous task (typed symbolic
  reasoning over a bounded space).
- **GRAM** (Baek et al. 2026, arXiv:2605.19376): multi-trajectory
  sampling improves TRM to 52% / 44.6%. The trajectory-width parameter
  trades compute for approximation quality directly — the empirical
  analog of an FPTAS's $1/\epsilon$.
- **Branch-and-bound PTAS** (Hendrich et al. 2025, arXiv:2504.15885):
  the formal guarantee that DP-style branching strategies yield
  polynomial-time approximation schemes.

*Citation provenance: Ibarra & Kim 1975 (Fast Approximation
Algorithms for the Knapsack and Sum of Subset Problems, JACM 22(4)
463–468); Lawler 1979 (Fast Approximation Algorithms for Knapsack
Problems, Math. Oper. Res. 4(4) 339–356); Jin 2019 / Chan 2018
(arXiv:1904.09562 — Improved FPTAS for 0-1 Knapsack); Hendrich et al.
2025 (arXiv:2504.15885 — B&B as PTAS); TRM (arXiv:2510.04871); GRAM
(arXiv:2605.19376). The transfer of FPTAS-style ratio guarantees to
mirror's tournament budget is open; the empirical TRM/GRAM results
suggest the ratio is favorable for bounded symbolic tasks.*

---

## 11. Contradiction as a particular shape of gap (Bateson + Belnap + LFI fold)

*Altitude: **declared shape proposal**. Absorbs `contradiction-and-fracture.md`
(commit `f12f58e`, retired 2026-06-04). Renames its primitives to live
underneath `gap` rather than beside it.*

The `gap` primitive (§3.1) names *unresolved distance between claim and
verifier* — a geometric quantity. It covers:

- **One-sided gaps.** `absent` (§4.4) — the claim COULD be made but ISN'T.
  `declared` (§4.3) — the claim exists with no verifier. Neither has a
  binary opposition; `contradiction` cannot name these.
- **Continuous-magnitude gaps.** `heuristic(p: probability)` (§4.2) — the
  gap carries a $[0,1]$ confidence per LTN's Real Logic grounding. Not a
  pair of opposed propositions; a real-valued degree of resolution.
- **Pre-positional gaps.** When `gaps_of(ast)` surfaces a hole the
  substrate doesn't yet know the shape of, `gap` carries it as `verifier =
  absent`. There is no "other side" to oppose; the gap is the absence of
  shape itself.

A *contradiction*, in the substrate-vocabulary sense, is a **particular
shape of gap**: binary-opposed, propositional, level-crossing (per Bateson
1956). Two obligations the substrate is asked to hold simultaneously; they
live at different logical levels per Bateson; one mode of resolution is to
ascend a level and find the morphism that makes the lower-level opposition
non-contradictory.

The shape: `contradiction <= gap`. Every contradiction IS a gap (the
distance is the unresolved opposition); not every gap is a contradiction
(the one-sided / continuous / pre-positional cases above).

### 11.1 The derived shape

```mirror
in @prism
in @epistemologic
in @epistemologic/property

grammar @epistemologic/property {
  # ... gap, gap_state, claim, verifier from §3.1 ...

  # A contradiction is a binary-opposed gap. Two obligations at different
  # logical levels per Bateson 1956. The level field is Bateson's logical-
  # level marker — the altitude at which the two claims live. Resolution
  # requires a morphism that lifts the contradiction to `level + 1` (the
  # Bateson Learning II → Learning III mechanic, ported to compiler-time).
  #
  # contradiction is structurally `gap` with extra structure:
  # - the gap.claim names the held position; both sides are tagged claims.
  # - the gap.state is heuristic(p) or declared (rarely verified — a
  #   verified contradiction is a closed fracture).
  # - the level field disambiguates same-altitude opposition (a type
  #   conflict) from cross-altitude opposition (a Bateson double bind).
  type contradiction <= gap & {
    left:   claim,
    right:  claim,
    level:  u32,                       # Bateson's logical level
  }

  # A fracture is the syntactically-marked subset: a gap whose surface form
  # carries the `\` obligation marker (`body_is_obligation` in
  # bootstrap/src/tokenize.rs returns true). Every `\` IS a Fracture AST
  # node IS a gap instance with verifier = absent. Not every gap has a `\`.
  #
  # fracture <= gap (strict subset). When the fracture's left/right shape
  # is a binary opposition, fracture <= contradiction <= gap (it's both).
  type fracture <= gap & {
    site:   span,                      # the `\` location
  }
}

out contradiction
out fracture
```

This preserves the existing `gap_state` algebra: a `fracture` carries
`state = declared` (the body is `\`; no verifier present) until the
kintsugi loop discharges it.

### 11.2 `holds gap` and `resolves gap` as properties

Following the shape in `boot/std/properties.mirror`:

```mirror
# `holds gap` — the substrate maintains the gap without prematurely
# collapsing to a default. This is Bateson's "stay in the bind without
# meta-communicating away" capacity; per Priest 1979, it is the
# paraconsistent refusal of `ex falso` at this site.
#
# At the variety altitude, `holds gap` ≡ `variety_hold = 1.0` per
# kintsugi-variety §6: the posterior support is preserved across the
# crossing the gap represents.
property holds(gap) <= verdict

# `resolves gap` — a kintsugi morphism has been applied; the gap is
# discharged at the appropriate altitude. The morphism IS the witness
# the verdict returns inside `Pass`.
property resolves(gap) <= verdict
  where applied(kintsugi.collapse)
```

Both properties land on the existing `verdict` algebra:

- `Pass` — `resolves` succeeded; gap closed.
- `Partial(confidence, [diagnostic])` — `holds` succeeded but `resolves`
  is in progress; the morphism is partially constructed.
- `Fail(diagnostic)` — neither holds nor resolves; the gap collapsed into
  incoherence (the substrate lost variety).

Notably, `holds(contradiction)` is the *specialization* of `holds(gap)` to
the binary-opposed shape. The substrate ships one property; the
contradiction-specific reading is a refinement, not a new property.

### 11.3 The unification headline

`Imperfect.Partial(t, l)` from `boot/01a-error.mirror`:

```mirror
type imperfect(value, loss = loss, error = error) = {
  value: value,
  loss: loss,
  errors: [error],
}
```

IS `holds(gap { value: t, loss: l })` at the @meta altitude. The loss
field IS the *unresolved variety between expected and produced* — the
geometric distance the gap names. The substrate proceeded with `value`,
but the obligation to close `loss` is still open. The held gap is the
record.

This is the headline identity from the retired `contradiction-and-fracture`
spec, rewritten under gap vocabulary. It no longer reads as a
contradiction-specific claim; it reads as the substrate's named way of
holding any unresolved distance.

### 11.4 `\` is the syntactic mark of a `fracture`-shaped gap

Unchanged from the retired spec, restated under gap vocabulary.
`bootstrap/src/tokenize.rs` lines 5–18:

```rust
fn body_is_obligation(bytes: &[u8]) -> bool {
    // ...
    end - start == 1 && bytes[start] == b'\\'
}
```

The `\` is *explicitly distinguished* from `Dark` (unrecognized bytes).
`Dark` is variety the substrate failed to absorb. `\` is variety the
substrate is *deliberately holding open* — a `fracture <= gap` instance
with `verifier = absent` and `state = declared`. The proposed
`AstKind::Fracture` (sibling of `Dark`) names what the existing byte-level
recognition already enacts; per the strict-and-total spec, `Dark`
triggers `--strict` refusal while `\` does not.

Note on the Bateson Game (Wilson et al. 2025): the "frame suppression"
mechanic — silent absorption of an unspoken bind — is structurally
identical to what `--strict` mode refuses. The substrate's discipline is
the game-theoretic mirror of refusing frame-suppression: a held gap
MUST surface; it MUST NOT be silently absorbed. **`gap` is named for the
same reason `--strict` exists.**

### 11.5 Cross-altitude unification (Belnap-Dunn at every altitude)

The four-valued logic of held / resolved / failed / unobserved shows up
at every altitude. The retired spec's table, restated:

| Altitude | T (verified) | F (failed) | Both (held gap) | Neither (unobserved) |
|---|---|---|---|---|
| Source bytes | recognized token | `Dark` | `\` (fracture) | whitespace/comment |
| AST | structural node | parse error | `Fracture` node | not yet parsed |
| Verdict | `Pass` | `Fail(diag)` | `Partial(c, [diag])` | not yet observed |
| Transparency | clear | opaque-failed | opaque-located | not yet measured |
| Variety | `variety_hold = 1.0` | `variety_hold = 0.0` | `0 < variety_hold < 1` | not yet measured |
| Imperfect | full value | error | `Partial(v, l)` | not yet attempted |
| `gap_state` (§4) | `verified` | (`Fail` upstream) | `heuristic(p)` / `declared` | `absent` |

One logic, seven imprints. Per Jakl 2025 (arXiv:2503.20679), this makes
mirror's `gap` lattice a fifth published CS imprint of Belnap-Dunn FOUR
(Jakl identifies four: linear logic models, Blame Calculus, LVars,
four-valued type systems).

### 11.6 The kintsugi loop's job, restated under gap

From `gap-tension-tensor-substrate.md` §6 and the retired spec's §6:
the kintsugi loop **resolves gaps by holding them in superposition long
enough to find the morphism that closes them without false collapse.**
Unpacked:

1. **Hold.** Encounter a gap; record it as a first-class substrate value.
   No premature collapse; `ex falso` refused (Priest 1979). The
   substrate is paraconsistent.
2. **Maintain variety.** `|R(@mirror)| ≥ |D|` even while the gap is open
   (kintsugi-variety §2 via Ashby). The gap's variety is itself
   information about the resolution space.
3. **Build the tensor.** Per §3.2, opposed gaps surface as `tension`s; the
   collection is the `tensor`. The sheaf Laplacian's restriction maps
   $\mathcal{F}_{v \trianglelefteq e}$ ARE the cross-altitude lift —
   Bateson's Learning II → Learning III, expressed as sheaf cohomology.
4. **Search for a morphism at level+1.** `@fate.minimize` walks the
   tensor's gradient; substrate-pull lifts the obligation from `@code/<lang>`
   to `@mirror` and from `@mirror` to its grammar at +1.
5. **Apply the morphism. Settle.** The resolved form is content-addressed;
   the verdict updates to `Pass`; the gap closes. Watzlawick's second-order
   change, compiler-side.

### 11.7 Bateson's force-application IS `tension`. Confirmation.

A double bind (Bateson 1956) is the structural condition where two
messages at different logical levels mutually negate each other inside a
relationship from which exit and meta-communication are blocked. The
"binding" is *force across logical levels*. This is exactly the §3.2
definition of `tension`: two gaps in opposition, with a `vector` field
naming the direction the tension pulls when minimized.

Confirmation: `tension` already carries the Bateson force-application
definition. The level-crossing requirement (Bateson 1972's Learning II)
is encoded in `contradiction.level` (§11.1) when the opposed gaps are
themselves contradictions; in the general case, the level information
lives implicitly in the AST positions the two gaps decorate.

No additional substrate vocabulary needed. **`tension` IS Bateson's
force-application across a gap.**

### 11.8 The sheaf-Laplacian restriction maps ARE the cross-altitude lift

From §3.2 (citing Bodnar et al. 2022): the sheaf Laplacian is
$L_{\mathcal{F}}$ with diagonal blocks
$L_{\mathcal{F}\,vv} = \sum_{v \trianglelefteq e} \mathcal{F}_{v \trianglelefteq e}^\top \mathcal{F}_{v \trianglelefteq e}$
and off-diagonals $L_{\mathcal{F}\,vu} = - \mathcal{F}_{v \trianglelefteq e}^\top \mathcal{F}_{u \trianglelefteq e}$.
The restriction maps $\mathcal{F}_{v \trianglelefteq e} : \mathcal{F}(v) \to \mathcal{F}(e)$
transport a vertex's stalk to the edge's stalk.

*Recognition (2026-06-04).* The restriction maps ARE the cross-altitude
lift Bateson named. A level-N gap (vertex stalk $\mathcal{F}(v)$) is
transported via $\mathcal{F}_{v \trianglelefteq e}$ to the edge stalk
$\mathcal{F}(e)$ where it can be compared against the other endpoint's
transported value. When the two transports agree, the cycle holonomy
vanishes; when they disagree, the holonomy IS the residual contradiction
at the higher altitude.

This is exactly Bateson's Learning III: the level-N opposition can only
be seen as a single object at level N+1. The sheaf Laplacian operationalizes
this: the edge stalk IS level N+1; the restriction maps ARE the lift; the
holonomy IS the residual gap *visible at level N+1 but invisible at level
N*. Per §10.B ("holonomy = error delta = convergence delta"), the residual
IS the convergence metric.

**Confirmation: `tensor` carries the cross-altitude lifting cleanly.** The
Bateson Learning II → III mechanic is the sheaf-cohomological lift, with
restriction maps as the morphism and holonomy as the residual measurement.
Nothing in the existing tensor algebra needs to change.

### 11.9 What this fold reorganizes (and what stays put)

- **`gap` is the substrate primitive.** Unchanged (§3.1).
- **`tension` is opposition between two gaps.** Unchanged (§3.2); now
  confirmed to absorb Bateson's force-application.
- **`tensor` is the collection of tensions over the corpus.** Unchanged
  (§3.2); now confirmed to carry the cross-altitude lift via the sheaf
  Laplacian.
- **`contradiction <= gap`** is the *derived form* for binary-opposed,
  propositional, level-crossing gaps. NEW.
- **`fracture <= gap`** is the *derived form* for syntactically-marked
  gaps. The `\` token IS the projection of a fracture into surface
  syntax. NEW.
- **`holds gap` / `resolves gap`** are the properties. NEW. They
  generalize the retired `holds contradiction` / `resolves contradiction`.
- **`Imperfect.Partial(t, l) ≡ holds(gap { value: t, loss: l })`** is the
  headline identity, rewritten under gap. NEW.
- **`AstKind::Fracture`** promotion stays a proposed tick; the
  recognition already lives in `body_is_obligation`. NEW (as roadmap).
- The §10 convergence machinery (Tarski, Hajek, Bodnar, etc.) is
  unchanged.
- The §8 design calls (tension_vector structure, t-norm selection,
  non-converging tensors) are unchanged.

### 11.10 Honest accounting: what gap CANNOT carry that contradiction had

One thing the retired spec named that gap absorbs only partially: the
**explicit logical-level marker**. Contradiction's `level: u32` field
tags Bateson's logical altitude; gap's `claim.node` carries an AST node
which implicitly has an altitude in the AST tree, but not as a first-class
scalar. For one-sided gaps and continuous-magnitude gaps this is moot —
there is no level-crossing to name. For binary-opposed gaps it matters,
and `contradiction` (§11.1) reintroduces it.

This is not a loss; it's a structural acknowledgment that the level marker
belongs to the *contradiction shape*, not to gap-as-such. A gap between
claim and verifier doesn't have "two levels"; a contradiction does. The
refactor surfaces this correctly.

Nothing else from the retired spec is lost. Every claim that named
"contradiction" maps to a claim about gap, optionally specialized via the
`<= gap` chain.

---

## 12. The LFI deep dive — Carnielli-Marcos and the consistency operator

*Altitude: **prior art for §11**. The contradiction spec flagged LFI as a
close formal parallel but didn't dive. This section closes that gap. The
sharpest finding: the **two-dimensional LCC hierarchy** (Carnielli, Coniglio,
Rodrigues 2026, arXiv:2604.18766) prefigures mirror's altitude × confidence
structure with the same shape — n (consistency iteration) and k (negation
strength) are mirror's `level` and `confidence` axes.*

### 12.1 The seminal source — mbC and the consistency operator `○`

Carnielli & Marcos (2002) — *A Taxonomy of C-systems*, in *Paraconsistency:
The Logical Way to the Inconsistent* (Marcel Dekker) — and the chapter
*A Basic Logic of Formal Inconsistency: mbC* in Carnielli & Coniglio's
*Paraconsistent Logic: Consistency, Contradiction and Negation* (Springer
2016, doi:10.1007/978-3-319-33205-5) lay down the seminal axiomatic
shape.

**mbC = positive classical propositional logic + two axioms:**

- **EM (excluded middle):** $A \lor \neg A$.
- **GEXP (gentle explosion):** $\circ A \to (A \to (\neg A \to B))$.
  "If $A$ is consistent, then explosion applies to it."

The consistency operator $\circ A$ is *primitive in the object language*.
It reads "$A$ is consistent" — a sentence-level marker that the substrate
is willing to apply classical reasoning to $A$. When $\circ A$ holds,
$A \land \neg A$ triggers explosion (everything follows). When $\circ A$
fails, $A \land \neg A$ is held *without trivialization*.

**This is exactly `holds(gap)` at the propositional altitude.** A gap
over an opposed claim pair is "held" iff $\circ A$ fails — the substrate
refuses to apply ex falso. The gap is "resolved" iff $\circ A$ holds and
the substrate has discharged the contradiction by adopting a consistent
witness.

### 12.2 LFI vs general paraconsistent — the technical difference

*Paraconsistent logic* (def, Priest, da Costa, Béziau): a logic is
paraconsistent iff $A, \neg A \not\vdash B$ for some $A, B$. Just
"refuse ex falso." No object-language vocabulary required.

*LFI* (Carnielli & Marcos 2002): a paraconsistent logic that **internalizes
consistency as a formula-level operator** $\circ$. The metatheoretic
property (consistent / inconsistent) becomes an object-language predicate.
The technical payoff: a single LFI can express both classical reasoning
(under $\circ A$) and paraconsistent reasoning (under $\neg \circ A$) at
the same time, sentence-by-sentence.

**Why this matters for mirror.** The `holds(gap)` / `resolves(gap)`
properties from §11.2 are mirror's $\circ$ at the object-language altitude.
A general paraconsistent substrate would just refuse `ex falso` and stop.
The LFI shape gives mirror a *first-class object-language vocabulary for
when the substrate IS willing to apply classical reasoning* — and that's
the substrate-pull design: held gaps are paraconsistent (no explosion);
resolved gaps are classical (explosion applies; the witness is canonical).

### 12.3 The Carnielli-Coniglio-Rodrigues body of work

The extension chain past mbC:

- **mbC** — base LFI; consistency primitive; gentle explosion only.
- **mbCciw, mbCci** — propagate consistency in increasingly classical ways.
- **RmbC** (Carnielli, Coniglio, Rodrigues 2020, arXiv:2003.09522, *Logic
  Journal of the IGPL* 28(5):624–656) — adds *replacement* (substitution
  of logically equivalent formulas). Algebraic and modal semantics; the
  weakest LFI that admits replacement.
- **LET-J, LET-F** (Carnielli, Rodrigues 2017, *Synthese*) — Logics of
  Evidence and Truth; extend Nelson's N4 (resp. FDE) with a *classicality
  operator* $\circ$. **The classicality operator is `holds(gap)`'s exact
  twin** — it marks formulas for which evidence is conclusive enough to
  warrant classical reasoning.
- **LCC** (Carnielli, Coniglio, Rodrigues 2026, arXiv:2604.18766, *A
  Taxonomy for Controlling (In)consistency*) — two-dimensional hierarchy
  $L^k_n$: $n$ controls iteration of the consistency operator, $k$ controls
  negation strength. **Prefigures mirror's altitude (n) × confidence (k)
  structure.** Fixed-point theorem: $L^m_n = L^{n+1}_n$ for all
  $m \geq n+1$.
- **Carnielli et al. 2024, arXiv:2412.10588, *Analytic proofs for logics
  of evidence and truth*** — sound, complete, decidable tableau system
  for LETF; demonstrates `○` does NOT automatically propagate over
  connectives. **Implication for mirror:** `holds(gap)` does not
  automatically lift through composition; each tension's holding-status
  is independent of its parts.

### 12.4 LFI in proof assistants — the implementation question

The search for `○A` in Coq / Agda / Lean / Rocq returned mostly null.
Proof assistants are *consistent by construction* (no anti-classical
axioms; types interpret as sets; ex falso is admissible) and the LFI
literature has not been ported. The closest existing work:

- **Fuenmayor (KWARC workshop 2021)** — paraconsistent and paracomplete
  logics in Isabelle/HOL via shallow embedding. Not LFI specifically.
- **Dore 2025** — Linear types inside dependent type theory (Cubical
  Agda). Not LFI, but shows the embedding technique that would extend
  to LFI: deep-embed the object language and reason about its judgements
  in the host type theory.

This is a *gap in the literature* (in mirror's sense — a `verifier =
absent` claim that *could* be made). The substrate's combination of
(typed AST + paraconsistent loop + LFI-shaped consistency markers) would
be the first published implementation of LFI as a programming-language
substrate that I have found.

### 12.5 The mbC family as a base for mirror's gap calculus

Which LFI is the right base? The candidates:

- **mbC.** Minimal; consistency primitive; non-propagating $\circ$. Maps
  most cleanly onto `holds(gap)` because gap-holding is *site-specific*
  (the substrate holds *this* gap, not its compositional descendants).
- **mbCciw / mbCci.** Add consistency propagation. Useful if mirror wants
  to claim that holding a gap automatically holds all its sub-gaps.
  Probably too strong; site-specific holding is the right shape.
- **RmbC.** Adds replacement (substitution-under-equivalence). Useful for
  the kintsugi morphism's canonical-at-fixpoint property (§6 of the
  retired spec). The closest fit when the resolution-morphism altitude
  is engaged.
- **LET-F (FDE-based).** Adds the four-valued Belnap base (§11.5 table)
  AND the classicality operator. **The strongest candidate for mirror's
  gap calculus.** LET-F's Belnap base IS already mirror's verdict
  algebra (Pass / Fail / Partial / unobserved); LET-F's $\circ$ IS
  `holds(gap)`. Mirror's calculus IS, structurally, LET-F at the
  substrate altitude.

**Recommendation:** LET-F as the formal reference for the gap calculus's
base; RmbC's replacement principle as the formal reference for the
kintsugi morphism's canonical-form property; LCC's two-dimensional
hierarchy as the formal reference for how mirror's altitude × confidence
structure stratifies. The base reference is LET-F.

### 12.6 The sharpest finding (would have changed the original spec)

**LCC's fixed-point theorem $L^m_n = L^{n+1}_n$ for all $m \geq n+1$
(Carnielli, Coniglio, Rodrigues 2026, arXiv:2604.18766).**

The hierarchy of nested consistency operators ($\circ A$, $\circ \circ A$,
$\circ \circ \circ A$, …) collapses past depth $n+1$ at any given
negation-strength level $n$. In mirror terms: **iterating `holds(holds(gap))`
beyond a bounded depth gives no additional substrate information.** The
substrate's altitude × confidence lattice has a *fixed depth* per
confidence tier, beyond which further meta-marking is redundant.

This would have changed the retired spec's §3 declaration of
`property holds(contradiction)` — it implies a *bounded* meta-hierarchy,
not an unbounded one. The substrate-pull tick that lands `holds(gap)` and
`resolves(gap)` (T-T-imperfect.5 in the retired roadmap) should pin the
meta-depth explicitly: $\text{holds}(\text{holds}(g)) = \text{holds}(g)$
at a single confidence tier; new structure only emerges when confidence
lifts. **The mirror analog of LCC's $n+1$ fixed point is one of the
substrate's load-bearing identities, not previously named.**

### 12.7 LFI sources — compact catalog

1. **Carnielli, W. & Marcos, J. (2002).** *A Taxonomy of C-systems.* In
   *Paraconsistency: The Logical Way to the Inconsistent*, Marcel Dekker.
   Introduces `○` as object-language consistency primitive; defines LFI;
   establishes mbC.
2. **Carnielli, W. & Coniglio, M. E. (2016).** *Paraconsistent Logic:
   Consistency, Contradiction and Negation.* Springer (Logic, Epistemology,
   and the Unity of Science series, vol. 40). doi:10.1007/978-3-319-33205-5.
   The canonical textbook. Chapter 2: mbC. Chapter 3+: extensions. The
   reference for the gap-calculus base.
3. **Carnielli, W. & Rodrigues, A. (2017).** *An epistemic approach to
   paraconsistency: a logic of evidence and truth.* *Synthese*. Defines
   BLE (Basic Logic of Evidence) and extends to LETJ. The classicality
   operator IS `holds(gap)` at the evidence-altitude.
4. **Carnielli, W., Coniglio, M. E., & Rodrigues, A. (2020).** *Logics
   of Formal Inconsistency Enriched with Replacement.* *Logic Journal
   of the IGPL* 28(5):624–656; arXiv:2003.09522. Defines RmbC; algebraic
   and modal semantics; the formal reference for canonical-form
   replacement under the kintsugi morphism.
5. **Carnielli, W., Coniglio, M. E., & Rodrigues, A. (2024).** *Analytic
   proofs for logics of evidence and truth.* arXiv:2412.10588. Tableau
   system for LETF; demonstrates that `○` does NOT automatically
   propagate over connectives — site-specific holding is the right shape.
6. **Carnielli, W., Coniglio, M. E., & Rodrigues, A. (2026).** *A Taxonomy
   for Controlling (In)consistency.* arXiv:2604.18766. Introduces LCC
   hierarchy $L^k_n$; fixed-point theorem; two-dimensional consistency ×
   negation lattice. **The sharpest single finding for mirror; bounds
   the meta-depth of `holds(holds(…))` iteration.**

*Six sources beyond the prior twelve. Quality over quantity; each one is
load-bearing for a specific substrate decision per §12.5–§12.6.*

---

## 13. References

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
    §3.2, §6, §6.1, §10.A, §10.F.5.**

12. **Hart, P. E., Nilsson, N. J., Raphael, B. (1968). *A Formal Basis
    for the Heuristic Determination of Minimum Cost Paths.*** *IEEE
    Transactions on Systems Science and Cybernetics* SSC-4(2):100–107.
    The A\* algorithm; admissibility of heuristics ($h(n) \leq h^*(n)$
    everywhere) guarantees optimality; consistency yields optimal
    efficiency among admissible algorithms. Mirror's tournament
    completeness (§10.F.4) is the substrate analog of admissibility.
    **Used in: §10.A, §10.F.4.**

13. **Hajek, B. (1988). *Cooling Schedules for Optimal Annealing.***
    *Mathematics of Operations Research* 13(2):311–329. JSTOR 3689827;
    https://web.mit.edu/6.435/www/Hajek88.pdf. Necessary and sufficient
    condition on the cooling schedule for simulated-annealing convergence
    in probability to the global-minimum set ($\beta_k = b \log(k+2)$,
    $b \leq d^*$). The canonical proof that convergent optimization
    tolerates per-step ascent provided macro-level dynamics decrease
    energy. Mirror's round-level Lyapunov inherits this principle:
    per-fracture worsening is permitted inside a round. **Used in:
    §10.A, §10.F.2 (retired).**

14. **Hendrich, M., Pferschy, U., Klotz, S. (2025). *Branch-and-Bound
    Algorithms as Polynomial-time Approximation Schemes.*** arXiv:2504.15885.
    A DP-style branching strategy yields a polynomial-time approximation
    scheme; B&B is provably anytime with bounded approximation gap. The
    formal basis for mirror's bounded-budget tournament returning
    near-optimal compositions. **Used in: §10.A, §10.F.4, §10.F.5.**

15. **Ibarra, O. H., Kim, C. E. (1975). *Fast Approximation Algorithms
    for the Knapsack and Sum of Subset Problems.*** *Journal of the ACM*
    22(4):463–468. The original FPTAS for 0/1 knapsack: $(1+\epsilon)$
    approximation in time polynomial in $n$ and $1/\epsilon$. The proof
    that DP-over-subsets outperforms greedy-per-item on monotone
    objectives. Structural basis for mirror's tournament-vs-greedy reframe.
    **Used in: §10.A, §10.F.2 (retired), §10.F.5.**

16. **Lawler, E. L. (1979). *Fast Approximation Algorithms for Knapsack
    Problems.*** *Mathematics of Operations Research* 4(4):339–356.
    Improved FPTAS bound for 0/1 knapsack. **Used in: §10.A, §10.F.5.**

17. **Jin, C. (2019). *An Improved FPTAS for 0-1 Knapsack.*** arXiv:1904.09562.
    Modern improvements via $(\max, +)$-convolution; the current best
    deterministic FPTAS bounds. Empirical anchor for the FPTAS-style
    guarantees mirror's tournament budget would inherit. **Used in:
    §10.A, §10.F.5.**

18. **Liu, S., Lu, P. (2022). *Lyapunov function approach for
    approximation algorithm design and analysis.*** arXiv:2205.12442. A
    two-phase systematic framework for proving approximation guarantees
    via Lyapunov functions on discrete optimization. The framework
    mirror's tensor norm $\|T_n\|$ would be validated against. **Used
    in: §10.A, §10.F.3.**

19. **Dechter, R., Pearl, J. (1985). *Generalized Best-First Search
    Strategies and the Optimality of A\*.*** *Journal of the ACM*
    32(3):505–536. The formal proof that A\* is optimally efficient
    among admissible search algorithms on non-pathological problems.
    **Used in: §10.F.4.**

*The prior research synthesis (`docs/insights/2026-05-26-mirror-tensors-vs-industry-tensors-research.md`) identifies several additional references
(IJCAI 2025 tensor-network survey, *Tensor Networks Meet Neural Networks*
v3, TensorLLM, LoTR, TPR variants, Lean/Rocq tactic prediction work, Nature
2026 compression phase-transition paper, etc.). They are not load-bearing
for the type signatures in this spec, but contextualize the broader
tensor-in-AI landscape and inform the prior-art ranking. See the synthesis
for the complete catalog.*

---

## 14. Provenance

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
- Formal-tightening tick (Reed, 2026-05-26 late): patched the Balanced
  Forman, LTN Real Logic, and sheaf Laplacian formulas with exact paper
  transcriptions read via `mcp__pdf-reader__read_pdf` against arXiv URLs.
  Added §8.3 (LTN t-norm selection) as a new design call surfaced by the
  transcription. Citation provenance preserved at every site.
- Convergence-and-lift tick (Reed, 2026-05-26 late): added §10 capturing
  three composing recognitions from the Reed–Alex conversation that
  followed the prior tick: convergence-as-halting via monotone
  convergence + Tarski + Banach; holonomy = error delta = convergence
  delta as one quantity under three names; `@fate` inference as a
  monadic lift from learned weights to corpus eigenvalues. Three
  preconditions for the convergence proof flagged as design calls
  (§10.F.1–3). "The graph is the model" noted as anticipated by earlier
  research agents; this tick gives it a citation chain through Bodnar
  2022 + Barbero 2022 + LTN + Topping.
- **Contradiction fold tick (Reed, 2026-06-04):** absorbed the retired
  `contradiction-and-fracture.md` (commit `f12f58e`, branch
  `reed/contradiction-and-fracture-spec`) into this spec as §11.
  `contradiction` becomes `<= gap` with extra structure (left, right,
  level); `fracture` becomes `<= gap` with `site: span`; `holds gap` /
  `resolves gap` are the properties (generalizing the retired
  `holds contradiction` / `resolves contradiction`).
  `Imperfect.Partial(t, l) ≡ holds(gap { value: t, loss: l })` is the
  headline identity. The Bateson force-application IS `tension` (§11.7);
  the cross-altitude lift IS the sheaf-Laplacian restriction-map
  (§11.8). Added §12 deep dive on Carnielli-Marcos LFI: mbC and the
  consistency operator `○A`, the Carnielli-Coniglio-Rodrigues body of
  work (RmbC 2020, LET-F 2017/2024, LCC 2026), the recommendation of
  LET-F as the formal reference for mirror's gap calculus base, and the
  sharpest single finding — LCC's fixed-point theorem bounds the
  meta-depth of `holds(holds(…))` iteration. Branch renamed to
  `reed/gap-substrate-fold`.
- Tournament-Lyapunov reframe (Mara, 2026-05-26 evening): Alex's
  structural correction — the unit of monotone decrease is the
  **tournament round** (one execution of `@fate.minimize` with bounded
  backtracking), not the individual fracture. Per-step ascent is
  permitted inside a round; round-level descent is required. §10.A
  rewritten around the 0/1-knapsack relaxation (greedy single-item
  selection fails; DP / B&B over subsets attains FPTAS guarantees);
  §10.F.2 retired (the per-fracture monotone requirement was too
  restrictive); §10.F.4 added (tournament completeness as the
  substrate analog of A\* admissibility); §10.F.5 added
  (approximation-ratio framing via FPTAS knapsack and TRM/GRAM
  empirical anchors). Citations added: Hart-Nilsson-Raphael 1968 (A\*);
  Hajek 1988 (simulated annealing); Hendrich et al. 2025 (B&B PTAS);
  Ibarra-Kim 1975 / Lawler 1979 / Jin 2019 (FPTAS knapsack); Liu & Lu
  2022 (Lyapunov for approximation); Dechter-Pearl 1985 (A\*
  optimality). The reframe is structurally significant but textually
  contained to §10.
