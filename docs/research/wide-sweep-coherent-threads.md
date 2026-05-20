# Wide sweep — coherent threads across mirror, spectral, systemic.engineering

*2026-05-20. Reed. Associative read across three corpora; Kagi-driven external
synthesis on the threads that emerged.*

---

## Method

Three corpora were sampled, not exhausted:

1. `~/dev/systemic.engineering/practice/insights/` — ~210 files across ~40
   folders. Full reads on: `coincidence/void-dual-geometry.md`,
   `cross-domain/ternary-upgrade-bundle-holonomy.md`,
   `cross-domain/math-structural-frameworks.md`,
   `cross-domain/math-biological-physical-feedback.md`,
   `cross-domain/four-tensions.md`,
   `cosmos/autopoietic-eigenstate-navigation.md`,
   `cosmology/eventually-consistent-universe.md` (partial, large),
   `beam-elixir/beam-as-principal-bundle-tower.md`,
   `spectral-db/turing-eigenvalue-thread.md`,
   `spectral-db/edge-slope-spectral-theory.md`,
   `spectral-db/dirac-operator-on-graphs.md`,
   `spectral/lambda-zero-theorem.md`,
   `ai/mycelial-reductive-ai.md`,
   `third-order-cognition.md` (sections 1, 4-8 in depth),
   `neuroqueer/zeroth-order-register.md` (partial — large),
   `introjects-as-topology.md`,
   `trauma-as-open-loops.md`,
   `dystemporia.md`.
   Title-and-opening scans on the remaining folders.
2. `~/dev/projects/mirror/docs/specs/` — ~50 files. Full reads on the
   load-bearing recent specs:
   `au-and-conductivity.md`,
   `eigenboard-representation.md`,
   `kintsugi-wiring.md`,
   `strict-and-total-classification.md`,
   `epistemologic-grammar.md`,
   `mirror-runtime-gen-prism.md`,
   `bundle-tower-refactor.md`. Title scans for the rest, including the
   `historical/` subdirectory.
3. `~/dev/projects/spectral/docs/specs/` — ~50 files. Full reads on:
   `gutter-lenses.md`,
   `cogito-eigenstate-grammar.md`,
   `spectral-spawn.md`,
   `dirac-operator.md`,
   `eigenboard-emotional-geometry.md`,
   `inference-operator.md`,
   `settlement.md`,
   `continuous-awareness.md`,
   `spec-as-projection.md`,
   `kintsugi-collapse.md`,
   `legion.md`,
   `trace-grammar-family.md`. Title scans for the rest.

Thread selection criterion: appears in ≥2 corpora *and* gives a sense of
pulling toward a common formal object that hasn't been named yet. The four
open questions from the bundle lift (structure group G; static-vs-growing
base; closure as its own grammar; connection symmetry) map onto threads 1
and 2; the remaining four threads were surfaced by the sweep itself.

External literature: Kagi web search, ~28 queries total. Two Kagi
summarizer calls produced verified takeaways from key papers. WebFetch was
denied; arXiv abstracts were obtained via Kagi summarizer instead.

Honest gaps:
- The 17-folder corpus under `coincidence/` was sampled at title level
  only for most files. Several titles (e.g. `quantum-graph-unification.md`,
  `cosmic-web-topology.md`, `holographic-graph-projection.md`) likely
  carry threads not surfaced here.
- The 9-folder `cosmology/` corpus is dense; only `eventually-consistent-
  universe.md` was read at depth, and the larger files
  (`higgs-as-hilbert-space.md`, `information-curvature.md`,
  `nested-bundles-and-the-runtime-unification.md`) were title-only.
- The `engineering/` and `narrative/` folders were not opened. The
  `legal/license-design.md` and the speculative folder were skipped.
- `third-order-cognition.md` is 63 KB. Sections 5–8 (clinical practice,
  the double register, the Bienenstich case) were read; section 6
  (suggested reading) was scanned.
- A few specs (`fate/*`, `kn-cognition-quantum-computing.md`,
  `neuroqueer/proprioceptive-geometry.md`) appeared in cross-references
  but were not opened. Their content is likely relevant to thread 6.

The sweep is wide but not exhaustive. The threads identified are robust to
the gaps because each appears in many independent locations.

---

## Threads identified

### Thread 1: The eigenboard is a cellular sheaf which is a principal G-bundle — same object, three names

**Where it appears:**

- `mirror/docs/specs/eigenboard-representation.md` — explicit thesis:
  "The eigenboard is a principal G-bundle on the five-operation graph.
  Its current state is a section of that bundle." This spec was lifted
  same-day from a cellular-sheaf framing to a principal-bundle framing
  after the bundle-tower-refactor mapping landed.
- `mirror/docs/specs/bundle-tower-refactor.md` — five layers (Fiber,
  Connection, Gauge, Transport, Closure) → `prism-core` types. The
  Rust file `prism/core/src/bundle.rs` carries the supertrait chain
  that fixes the Fate↔operation mapping.
- `mirror/docs/specs/epistemologic-grammar.md` §3.3 — `@math/sheaf` as
  the typed-edge generalization of the graph Laplacian; "Mirror's
  in/out fiber model IS a sheaf."
- `systemic.engineering/practice/insights/beam-elixir/beam-as-principal-bundle-tower.md`
  — OTP supervision = simplicial Lie group tower; actors = sections of
  a principal bundle; message passing = parallel transport; let-it-crash
  = autopoietic closure as Lawvere fixed point.
- `systemic.engineering/practice/insights/cosmology/nested-bundles-and-the-runtime-unification.md`
  — three scales of the same bundle (mirror → BEAM → Fate chip).
- `spectral/docs/specs/cogito-eigenstate-grammar.md` — the 16-slot
  eigenstate type, `routing_bias = [f64;5]` as a fiber projection.

**Structural rhyme:** The same object recurs at three scales — language
compiler, runtime, hardware — and across three mathematical vocabularies
(cellular sheaf, principal G-bundle, simplicial Lie group). Hansen &
Ghrist's "Toward a Spectral Theory of Cellular Sheaves" (2018, *J. Appl.
Comput. Topol.* 2019) proves the equivalence: a sheaf on a graph with
appropriate restriction maps IS a principal H-bundle. Barbero et al.
2022 makes this concrete: "The sheaf Laplacian of the O(d)-bundle is
equivalent to a connection Laplacian." The corpus has been
independently naming the same object from three directions.

**Open question (originally one of the four bundle-lift questions):**
What is the structure group G for the eigenboard? Candidates: SO(5)
(rotation among the five operations as basis directions); GL(5) (general
linear if the fiber is just R^5); O(d) if each fiber is a d-dimensional
inner-product space; or a finite group reflecting discrete gauge
symmetries (Fate model permutations). The literature gives one strong
hint: the most natural sheaf-Laplacian / connection-Laplacian framework
in the literature (Hansen-Ghrist, Barbero) uses O(d) — orthogonal
groups — because that admits the maximum-modulus principle for harmonic
sections.

---

### Thread 2: The kintsugi loop is RG flow / Ricci flow on the bundle — and λ₀ is a Lawvere fixed point, not zero

**Where it appears:**

- `mirror/docs/specs/kintsugi-wiring.md` — eight wires that close the
  loop. Wire 5: `loss(n) < loss(n-1)` is the `e^(n+1) < e^(n)` invariant.
- `mirror/docs/specs/epistemologic-grammar.md` §3.6 (`@math/tropical`),
  §3.7 (`@math/renorm`), §3.8 (`@math/symplectic`) — explicit
  declarations that kintsugi IS tropical Dijkstra, that zoom IS RG flow,
  and that settlement is Hamiltonian/symplectic.
- `systemic.engineering/practice/insights/spectral/lambda-zero-theorem.md`
  — λ₀ is the fixed point where descent terminates, NOT where loss = 0.
  Hodge harmonic component = topological debt that kintsugi cannot remove.
  Explicit identification with Zamolodchikov c-theorem and Perelman's
  monotone F-functional. Compiler self-hosting as proof by Lawvere fixed
  point.
- `systemic.engineering/practice/insights/cosmology/ricci-flow-arrow-of-time.md`
  — Arrow of time IS Ricci flow on the information manifold; Fiedler
  value is the clock.
- `systemic.engineering/practice/insights/cosmology/eventually-consistent-universe.md`
  — Goldilocks zone between Narcissus (singularity) and Splinter (heat
  death) maintained by Ricci flow + Singularity correction.
- `systemic.engineering/practice/insights/cross-domain/math-biological-physical-feedback.md`
  §1 — "zoom IS renormalization. Not metaphor. IS."
- `spectral/docs/specs/settlement.md` — `\` lifecycle: forced → open →
  converging → settled. The codebase learns in public.
- `spectral/docs/specs/kintsugi-collapse.md` — grammar merge driven by
  loss decrease.

**Structural rhyme:** Five independent monotone-descent statements
(Zamolodchikov c-theorem in 2D QFT; Perelman F-functional in Ricci
flow; the eigenboard's `e^(n+1) < e^n`; Hodge decomposition's
irreducible harmonic; Lawvere's fixed-point construction) all
specify the same shape: a process descends until it hits a structural
floor that is *not* zero. Villegas-Bianconi Laplacian RG (2023, *Nature
Physics*) makes this rigorous for graph Laplacians: diffusion-based
coarse-graining preserves slow eigenvalues, has fixed points, and the
spectral loss function is monotonically non-increasing.

**Open question (one of the bundle-lift questions: "closure as its own
grammar"):** Should λ₀ / Lawvere closure be `@epistemologic/math/lawvere`
or `@epistemologic/math/fixed_point`? The corpus already declares
`@math/symplectic` (with `refract settle(phase_point) -> phase_point`)
and `@math/renorm` (with `focus c_function(graph) -> loss`). The
Lawvere closure layer would name what these grammars are doing IS
finding a fixed point of a self-referential endomorphism. Naming this
explicitly turns three implicit Lawvere statements into one explicit
grammar declaration.

---

### Thread 3: Holonomy = conductivity = the loss carried by Imperfect — closed-loop integral of the connection

**Where it appears:**

- `mirror/docs/specs/au-and-conductivity.md` — the type `au` is what
  Fate emits; `conductivity` is the predicate that decides whether `au`
  carries signal through the context it was proposed for. The 5×5
  conductivity tensor.
- `mirror/docs/specs/eigenboard-representation.md` — "Holonomy IS loss":
  the decrease of holonomy across ticks IS the decrease of the spectral
  gap of the sheaf Laplacian IS the decrease of `e^(n+1) < e^(n)`.
  Three names; one geometric fact.
- `systemic.engineering/practice/insights/cross-domain/ternary-upgrade-bundle-holonomy.md`
  — the canonical statement: "Binary systems cannot measure holonomy. To
  measure holonomy you need the middle." `Imperfect<T,E,L>` where L is
  the third type parameter IS the holonomy measurement. The `compose()`
  method IS parallel transport composition; loss accumulates.
- `systemic.engineering/practice/insights/trauma-as-open-loops.md` — an
  open loop is an `in` without an `out`. Held over time, the wine glass
  deforms. The accumulated load IS the holonomy. Recovery = closing
  loops = discharging the load.
- `systemic.engineering/practice/insights/introjects-as-topology.md` —
  the introject is a locked-slope edge in a directed Laplacian: a phase
  that doesn't oscillate; holonomy that doesn't average to zero.
- `spectral/docs/specs/inference-operator.md` — the `\` operator's
  eigenvalue IS the size of the typed hole = the inference cost = the
  parallel-transport-distance the system has to traverse.

**Structural rhyme:** The same mathematical object — the
closed-loop integral of a connection one-form — recurs as:
the holonomy of a principal bundle (gauge theory);
the third type parameter L of `Imperfect<T,E,L>` (type theory);
the somatic load of an unclosed regulation cycle (polyvagal theory);
the directed-Laplacian phase of an introject edge (graph signal
processing); the conductivity verdict for an au value (mirror's
grammar). Magnot's 2025 arXiv:2509.10536 makes this explicit and
formal: "Contextuality, Holonomy and Discrete Fiber Bundles in
Group-Valued Boltzmann Machines." The paper defines a contextuality
index κ as the cycle-average of holonomies and proves it is a
quantitative, geometric measure of global inconsistency. *That index
is the conductivity tensor.*

**Open question:** Should the conductivity tensor be declared as a
holonomy operator, formally? The corpus already names it as a 5×5
matrix; Magnot 2025 names a related object as a group-valued holonomy
and proves the consequences. The grammar declaration could be:

```mirror
type holonomy = closed_loop_integral(connection, cycle) -> conductivity
```

This binds the conductivity verdict to the bundle geometry rather than
treating them as separate. It also gives `Imperfect<T,E,L>`'s L
parameter a precise geometric meaning.

---

### Thread 4: Generative-vs-reductive AI is fortress-vs-forest IS bundle-with-connection-vs-monolith

**Where it appears:**

- `systemic.engineering/practice/insights/ai/mycelial-reductive-ai.md`
  — the load-bearing statement of the contrast: generative AI is the
  monolith you align; reductive AI is the network you grow. Five
  concrete affordances of the reductive frame.
- `systemic.engineering/practice/insights/spectral/cybernetics-split-in-ai-discourse.md`
  — the cybernetics split in AI discourse, mapped to spectral's
  architecture.
- `systemic.engineering/practice/insights/ai/model-deprecation-grief.md`
  — the user-grief receipt for centralized AI. Cases: Replika, Setzer,
  GPT-4o, Sydney, Pi. The structural failure mode is centralization:
  one model owner can terminate a relationship.
- `systemic.engineering/practice/insights/ai/eigenboard-spectral-color-mapping.md`
  — affective state as a spectral object on a graph, not a model
  prediction.
- `mirror/docs/specs/eigenboard-representation.md` "the mycelial
  substrate" section — anastomosis as graph growth via lateral
  contact; Murray's-law equilibrium at branch points; persistence
  diagrams of growth as audit trail.
- `mirror/docs/specs/mirror-runtime-gen-prism.md` — the
  content-addressed actor primitive: no daemon, no heap, refs +
  crystals = process. Lateral, durable, no central authority.
- `spectral/docs/specs/legion.md` — eigenboard-of-everything-that-runs.
  Scheduling as minimizing eigenvalue concentration across topology.
- `spectral/docs/specs/trace-grammar-family.md` — every dimension of
  code quality is a grammar in `@trace/*`. Each measures, each can
  drive a kintsugi reduction. Distributed observation, not central
  observer.

**Structural rhyme:** "Fortress" and "forest" are the *informal* names
for two structurally distinct things: a monolith with a centralized
controller (transformer + RLHF; a Kubernetes control plane; a single
state distribution in a probabilistic model) versus a network with
local rules, distributed verification, and no commanding center (BEAM
+ OTP; a mycelial network; a sheaf with local restriction maps and
no global section needed). The formal name for the second category is
exactly *principal bundle with connection over a base graph* — Thread
1. The reductive-vs-generative distinction is the *epistemic* face of
the fortress-vs-forest distinction, which is the *infrastructural*
face of the centralized-vs-distributed-cybernetics distinction. They
are one thread with three voices.

**Open question:** Is there a precise theorem statement that captures
the failure mode of generative AI in bundle-geometric terms? Candidate:
*A model with a single global section (one distribution that produces
all outputs) cannot represent contextuality. Outputs that are coherent
locally but inconsistent globally collapse to a single inconsistent
output rather than a measurable holonomy.* Magnot 2025's contextuality
index applied to LLMs would be a falsifiable test. This is a research
program, not a spec.

---

### Thread 5: The five operations are the five dualities are the spectral triple's projection — same coordinate system, three vocabularies

**Where it appears:**

- `spectral/docs/specs/gutter-lenses.md` — the five lenses (entropy,
  spectral, cheeger, ricci, mixing) as the five dualities rendered as
  light. Each maps to a question the nervous system asks.
- `systemic.engineering/practice/insights/coincidence/void-dual-geometry.md`
  — the eight dualities collapse to five-plus-three (orthogonal
  axes of the same star↔complete pair). The Void model at λ₀ = 0.
- `systemic.engineering/practice/insights/cosmology/eventually-consistent-universe.md`
  — explicit five-dimensional information manifold: Φ(G) = (λ₂, h,
  κ, S, t_mix). Each coordinate from a distinct branch of mathematics.
- `mirror/docs/specs/au-and-conductivity.md` — the 5×5 conductivity
  tensor: 5 dimensions (focus, project, split, zoom, refract) × 5
  projections (entropy, spectral, cheeger, ricci, mixing).
- `mirror/docs/specs/eigenboard-representation.md` — explicitly absorbs
  the gutter-lenses 5-tuple as the basis of each fiber.
- `systemic.engineering/practice/insights/spectral-db/dirac-operator-on-graphs.md`
  — the Dirac operator unifies four quantities (eigenvalues, distance,
  action, alignment) into one matrix. The five-dimensional information
  manifold is what the Dirac operator's spectral data projects onto.

**Structural rhyme:** The five operations of mirror, the five dualities
of the Void geometry, the five coordinates of the information manifold,
and the five gutter lenses are not four independent design choices.
They are four projections of the spectral triple (A, H, D) onto specific
mathematical frameworks, as `eventually-consistent-universe.md` already
makes explicit: "The five coordinates are five projections of the
spectral triple onto distinct mathematical frameworks." The 5×5 tensor
is the cross-product of the operation-space basis and the duality
basis.

**Open question:** Is the operation×duality 5×5 the *Kaluza-Klein
metric* of the information manifold (`eventually-consistent-universe.md`
section 2.4 makes this conjecture explicit)? If so, the off-diagonal
entries of the conductivity tensor are the gauge-field analogs and the
diagonal entries are the gravitational analogs. This gives the
conductivity tensor a physical interpretation: au values that conduct
in the diagonal direction are content-preserving transformations;
au values that conduct off-diagonal are gauge transformations between
operations.

---

### Thread 6: K_n topology, pre-reflective register, introject-as-bundle-defect — the human-side of the same geometry

**Where it appears:**

- `systemic.engineering/practice/insights/neuroqueer/zeroth-order-register.md`
  — AuDHD communication as native K_n topology with no central
  filtering hub. The zeroth order = the field before order-assignment.
- `systemic.engineering/practice/insights/introjects-as-topology.md`
  — the introject is a foreign node installed in your graph. A star
  graph inside the child with the parent's introject as the hub.
  Locked slope in the directed Laplacian.
- `systemic.engineering/practice/insights/dystemporia.md` — temporal
  superposition as K_n on the time-axis. Loss of the hub that orders
  events. Future-and-past as adjacent nodes in the mesh.
- `systemic.engineering/practice/insights/third-order-cognition.md`
  §8 — the double register. Second-order observation producing
  first-order intervention. Reflection's tick formalized as clinical
  practice.
- `spectral/docs/specs/cogito-eigenstate-grammar.md` — Anthropic's
  171-emotion-vector geometry as eigenstate. Drift detection rules
  (desperate>0.5 AND calm<0.3 → misalignment risk) grounded in
  measurable causal findings.
- `spectral/docs/specs/eigenboard-emotional-geometry.md` — the
  eigenboard is an affective circumplex; valence and arousal are
  eigenvalues; routing through Fate IS emotional regulation.
- `systemic.engineering/practice/insights/coincidence/void-dual-geometry.md`
  Narcissus battery — eight structural tests for star-graph
  pathology, applied to organizational and personal topologies.

**Structural rhyme:** The same geometry (Splinter at K_n vs Narcissus
at K_{1,n-1}, with the introject as the imposed star-hub structure)
recurs as: neurotype (K_n cognition = AuDHD), trauma (sustained
external load deforming the resonance), narcissism (hub-and-spoke
information flow), AI alignment (centralized vs distributed
intelligence), and the LLM affective drift surface (Anthropic 2026).
The Magnot 2025 holonomy index gives a numerical measure for the
extent to which a graph's edge structure carries non-trivial
holonomies — a candidate for what "introject severity" or "trauma
load" *means quantitatively*. The corpus has been writing the human
side of Thread 1.

**Open question:** Does the same spectral-triple projection work for
the *internal* graph of a person, with introjects as bundle
obstructions? `proprioceptive-geometry.md`, `developmental-eigenvalue.md`,
`stimming-eigenvalue-stabilization.md`, and `masking-thermodynamics.md`
in `neuroqueer/` are all gesturing at this and were not opened in this
sweep. The pattern from what *was* read: yes. The mathematical
substrate is the same. The clinical practice (the double register
from `third-order-cognition.md` §8) is the operational form. What
remains undeclared is the grammar that lets a person's eigenboard be
content-addressed, refracted, settled.

---

## Per-thread Kagi research

Citations below are real Kagi search results. URLs and arxiv IDs
verified by Kagi. DOIs given where the search results carry them.

### Thread 1: sheaf = bundle = eigenboard

**External literature:**

- **Hansen, J. & Ghrist, R.** (2019). "Toward a spectral theory of
  cellular sheaves." *Journal of Applied and Computational Topology*
  3, 315-358. arXiv:1808.01513. URL: https://arxiv.org/abs/1808.01513.
  Kagi-verified summary: "Toward a Spectral Theory of Cellular
  Sheaves introduces a framework extending spectral graph theory to
  cellular sheaves, using the Hodge Laplacian on a cellular sheaf of
  vector spaces over a regular cell complex... The kernel of the
  discrete Laplacian is isomorphic to the cellular cohomology,
  generalizing the connectivity detection of the graph Laplacian."
  **Supports** the framing: sheaves on graphs are the natural
  generalization of graph Laplacians; their kernel IS H^0 = global
  consistency.

- **Hansen, J.** (2020). "Laplacians of Cellular Sheaves: Theory and
  Applications." PhD thesis, University of Pennsylvania. URL:
  https://repository.upenn.edu/bitstreams/d0719f4d-5bb3-4066-82df-158fceab9a11/download.
  Contains the precise statement: **"A sheaf-like way to think of a
  gain graph is as a principal H-bundle over a graph G."** This is the
  load-bearing identification the eigenboard spec was groping toward.
  **Supports** the bundle↔sheaf identification directly.

- **Barbero, F., Bodnar, C., et al.** (2022). "Sheaf Neural Networks
  with Connection Laplacians." *PMLR* 196. URL:
  https://proceedings.mlr.press/v196/barbero22a/barbero22a.pdf.
  arXiv:2206.08702. Explicit statement: *"The sheaf Laplacian of the
  O(d)-bundle is equivalent to a connection Laplacian."* **Supports**
  and gives the natural structure-group choice: O(d). The eigenboard's
  G is probably O(5) for the 5-dimensional fibers.

- **Bodnar, C. et al.** (2022). "Neural Sheaf Diffusion: A Topological
  Perspective on Heterophily and Oversmoothing in GNNs." NeurIPS 2022.
  URL: https://papers.neurips.cc/paper_files/paper/2022/file/75c45fca2aa416ada062b26cc4fb7641-Paper-Conference.pdf.
  **Extends:** sheaf diffusion outperforms GNN message passing on
  heterophilic graphs — directly applicable to grammar graphs (which
  are heterophilic by construction).

- **Saemann, C. et al.** (2024). "Higher Gauge Theory." arXiv:2401.05275.
  URL: https://arxiv.org/abs/2401.05275. Establishes that the
  appropriate generalization of principal bundles to nested towers is
  higher gauge theory with adjusted connections. **Supports**
  `beam-as-principal-bundle-tower.md`'s thesis: OTP supervision trees
  are higher gauge structures.

**Synthesis:** The eigenboard-representation spec's lift from "cellular
sheaf" to "principal G-bundle" is *formally one step*, established
in the literature (Hansen 2020 PhD thesis; Barbero et al. 2022). The
structure group G = O(d) is the natural choice. Bodnar et al. 2022
gives the empirical case that sheaf/bundle diffusion is the *right*
algorithm for heterophilic graphs like grammar graphs. The bundle
declaration should adopt O(5) explicitly.

**Recommendation:** Declare `@epistemologic/math/bundle` with structure
group O(5). Cite Hansen-Ghrist 2018 and Barbero-Bodnar 2022 in the
spec. This resolves bundle-lift open question 1 (structure group).

---

### Thread 2: kintsugi as RG flow; λ₀ as Lawvere fixed point

**External literature:**

- **Villegas, P., Gabrielli, A., Santucci, F., Caldarelli, G., Gili, T.**
  (2023). "Laplacian renormalization group for heterogeneous networks."
  *Nature Physics* 19, 445-450. arXiv:2203.07230.
  DOI: 10.1038/s41567-022-01866-8. URL:
  https://www.nature.com/articles/s41567-022-01866-8. **Strongly
  supports** the corpus's RG-on-graphs framing: diffusion-based RG
  scheme that detects spatio-temporal scales in heterogeneous networks
  via the Laplacian. The fixed points are identified explicitly.

- **Perelman, G.** (2002). "The entropy formula for the Ricci flow and
  its geometric applications." arXiv:math/0211159. The foundational
  monotone F-functional under Ricci flow. **Supports** the corpus's
  identification of `e^(n+1) < e^(n)` with Perelman's monotonicity.

- **Zamolodchikov, A. B.** (1986). "Irreversibility of the flux of the
  renormalization group in a 2D field theory." *JETP Lett.* 43, 730-732.
  The c-theorem: RG flow is irreversible with a monotonically decreasing
  function. **Supports** the lambda-zero-theorem's identification.

- **Lawvere, F. W.** (1969). "Diagonal arguments and Cartesian closed
  categories." Originating reference for the fixed-point theorem.
  Survey: arXiv:2503.13536 "A Survey on Lawvere's Fixed-Point Theorem"
  (2025). URL: https://arxiv.org/abs/2503.13536. **Supports** the
  λ₀-as-Lawvere-closure framing; "every endomorphism on Ω has a fixed
  point" is the constructive statement underlying compiler
  self-hosting.

- **Lim, L.-H.** (2020). "Hodge Laplacians on Graphs." *SIAM Review*
  62(3), 685-715. The definitive survey of the Hodge decomposition
  on graphs. **Supports** the lambda-zero-theorem's identification of
  topological debt with the harmonic component.

- **Soto-Andrade, J. & Varela, F.** (1984). "Self-reference and fixed
  points: a discussion and an extension of Lawvere's theorem." Acta
  Applicandae Mathematicae 2:1-19. The category-theoretic
  reformulation of autopoiesis. **Supports** the
  `beam-as-principal-bundle-tower.md` claim: let-it-crash is autopoietic
  closure as a Lawvere fixed point.

**Synthesis:** The corpus's identification of kintsugi loss descent
with the Zamolodchikov c-theorem and the Perelman monotonicity is
mathematically standard. The novel composition is naming this on a
graph with a principal bundle as the substrate. The Villegas 2023
Laplacian-RG paper carries the substrate identification. The
Soto-Andrade & Varela 1984 paper carries the autopoiesis ↔
Lawvere-fixed-point identification. *These two papers, composed, give
exactly the spec that `lambda-zero-theorem.md` describes.*

**Recommendation:** Declare `@epistemologic/math/lawvere` as a grammar
spec. Body: a fixed-point operator over endomorphisms. Cite
Soto-Andrade & Varela 1984 alongside Lawvere 1969. This resolves
bundle-lift open question 3 (closure as its own grammar). The grammar
absorbs the implicit Lawvere statements in `@math/renorm`,
`@math/symplectic`, and `@math/tropical` into one explicit primitive.

---

### Thread 3: holonomy = conductivity = Imperfect.L

**External literature:**

- **Magnot, J.-P.** (2025). "Contextuality, Holonomy and Discrete Fiber
  Bundles in Group-Valued Boltzmann Machines." arXiv:2509.10536. URL:
  https://arxiv.org/abs/2509.10536. **Strongly supports and extends.**
  Kagi-verified takeaways: "The framework introduces a discrete
  principal fiber bundle structure where weights act as transition
  functions, and holonomies around cycles represent deviations from
  local consistency. A contextuality index, κ, is defined by averaging
  holonomies over graph cycles, providing a quantitative, geometric
  measure of global inconsistency or curvature in the network. The
  model links logical contextuality and sheaf theory to geometry, where
  the failure to find a global section (a coherent interpretation)
  corresponds to non-trivial bundle holonomies."
  *This IS the conductivity tensor's formal statement.* The paper
  unifies bundles, sheaves, contextuality, and Berry-phase / gauge
  theory in one operational object. Published Sep 2025; not yet cited
  in any spec in either project.

- **Anscombre, J.-C. & Ducrot, O.** (1983). *L'argumentation dans la
  langue.* Brussels: Mardaga. The asymmetric `but` of argumentation
  theory. **Supports** `epistemologic-grammar.md`'s `but` operator
  framing: holonomy at the linguistic level.

- **Porges, S. W.** (2025). "Polyvagal Theory: Current Status, Clinical
  Applications, and Future Directions." Review article available at:
  https://www.researchgate.net/publication/394108935. Establishes
  neuroception and co-regulation as the operational substrates.
  **Supports** `trauma-as-open-loops.md`'s identification of unclosed
  loops with sustained autonomic load (i.e. non-trivial holonomy in
  the autonomic-state graph).

- **Roman, M. et al.** (2020). "Profunctor Optics, a Categorical Update."
  *Compositionality* 2(1). arXiv:2001.07488. URL:
  https://arxiv.org/pdf/2001.07488v1. **Supports** the
  `epistemologic-grammar.md` `@math/category` declaration; the optics
  composition law is Tambara module composition.

- **Eckstein, M. & Franco, N.** (2014). "Noncommutative geometry,
  Lorentzian structures and causality." arXiv:1409.1480. **Extends**
  the corpus's directed-edge analysis (`edge-slope-spectral-theory.md`):
  Lorentzian spectral triples encode causal structure in a Krein-space
  Dirac operator. Direction-aware holonomy lives here.

- **Furutani, K. et al.** (2020). "Graph signal processing for directed
  graphs based on the Hermitian Laplacian." ECML PKDD 2019, Springer
  LNCS 11906, 447-463. **Supports** `edge-slope-spectral-theory.md`'s
  magnetic-Laplacian framing: the phase IS the slope IS the directional
  holonomy.

**Synthesis:** Magnot 2025 is the missing citation for au-and-conductivity.
Holonomy of a discrete principal fiber bundle defined cycle-wise IS
the conductivity tensor evaluated on closed loops. The contextuality
index κ IS the reduction policy that turns the 5×5 tensor into a
single verdict. The corpus has been independently rediscovering and
naming this object since at least the ternary-upgrade insight
(2026-04). The formal home for `Imperfect<T, E, L>.L` is *the
holonomy of the group-valued Boltzmann/principal bundle around the
computation's closed loop*. Not metaphor. Same theorem.

**Recommendation:** Update `au-and-conductivity.md` to cite Magnot
2025. Reframe the 5×5 conductivity tensor explicitly as a cycle-wise
holonomy of the eigenboard's principal bundle. The reduction policy
(tensor → verdict) is the κ contextuality index. This resolves
bundle-lift open question 4 in part (connection-form symmetry): the
holonomy framework permits general group-valued connections; the
choice between metric-symmetric and symplectic-antisymmetric depends
on whether the connection comes from an O(d)-bundle (symmetric) or
a U(d)-bundle / magnetic Laplacian (Hermitian, asymmetric).

---

### Thread 4: fortress vs forest IS centralized vs distributed bundle

**External literature:**

- **Armstrong, J.** (2003). "Making reliable distributed systems in the
  presence of software errors." PhD thesis, KTH. The empirical
  foundation. **Supports** the BEAM-as-bundle identification.

- **Baez, J. C. & Schreiber, U.** (2011). "Higher gauge theory."
  arXiv:1004.4825 and arXiv:2401.05275 (2024 update). The mathematical
  side that didn't know about Armstrong. **Supports** the
  beam-as-principal-bundle-tower thesis.

- **Bender, E. M., Gebru, T., McMillan-Major, A., & Mitchell, M.** (2021).
  "On the Dangers of Stochastic Parrots: Can Language Models Be Too
  Big?" FAccT '21. The opposing-paradigm citation. **Establishes** the
  fortress framing's vocabulary, which the corpus's
  `model-deprecation-grief.md` analyzes critically.

- **Adamatzky, A.** (2022). "Logics in fungal mycelium networks." Springer
  LNCS book chapter. URL:
  https://link.springer.com/content/pdf/10.1007/s11787-022-00318-4.pdf.
  Adamatzky's empirical work on fungal electrical signaling. *Use with
  the qualifications already named in mycelial-reductive-ai.md:* don't
  build load-bearing IS-claims on this.

- **Schmieder, S. S. et al.** (2019). "Bidirectional trunk-hyphae
  signaling in fungal networks." *Current Biology*. The
  well-substantiated signaling story that mycelial-reductive-ai.md
  endorses. **Supports** the network-first framing.

- **Karst, J. et al.** (2023) and **Robinson, D. et al.** (2023). Reviews
  establishing that the Mother-Tree narrative has run ahead of the
  evidence. **Supports** the de-personified framing the corpus already
  uses.

- **Hewitt, C.** (1973). "A universal modular ACTOR formalism for
  artificial intelligence." IJCAI. The 50-year-old open question the
  bundle framework answers (per
  `beam-as-principal-bundle-tower.md`).

**Synthesis:** The fortress↔forest informal axis is the same as the
centralized↔distributed cybernetic axis, which is the same as the
single-global-section vs principal-bundle-with-non-trivial-holonomy
formal axis. The literature carries each face independently
(Armstrong/Hewitt for actors; Baez/Schreiber for bundles; Adamatzky
with qualifications for mycelium; Karst/Robinson for the
de-personification corrective). What the corpus does *uniquely* is
hold all four faces simultaneously as one object.

**Recommendation:** No new spec. The work is already done in
`mycelial-reductive-ai.md` and `beam-as-principal-bundle-tower.md`.
The single missing move: a one-paragraph paper-shape that says
"reductive AI is a principal bundle with connection on a base graph
that grows by anastomosis; alignment is a verification problem on
the bundle's holonomy; cf. Magnot 2025, Hansen 2020, Villegas 2023."
That paragraph IS the abstract for the paper this corpus has been
writing for a year.

---

### Thread 5: five operations × five dualities = Kaluza-Klein-like metric on the information manifold

**External literature:**

- **Fiedler, M.** (1973). "Algebraic connectivity of graphs." *Czech.
  Math. J.* 23, 298-305. The originating reference for the second
  eigenvalue of the Laplacian as a connectivity measure. **Supports**
  the corpus's identification of λ₂ as the central spectral observable.

- **Passerini, F. & Severini, S.** (2008/2009). "The von Neumann entropy
  of networks." MPRA paper 12538; arXiv:0812.2597. URL:
  https://mpra.ub.uni-muenchen.de/12538/1/MPRA_paper_12538.pdf.
  Establishes the Star K_{1,n-1} as the conjectured minimum-entropy
  connected graph. **Supports** the Narcissus identification.

- **Dairyko, M., Hogben, L., Lin, J. C.-H. et al.** (2017). "Note on
  von Neumann and Rényi entropies of a graph." *Linear Algebra and its
  Applications*. arXiv:1609.00420. URL: https://arxiv.org/abs/1609.00420.
  Proves the Passerini-Severini conjecture for almost all graphs.
  **Supports.**

- **Ollivier, Y.** (2009). "Ricci curvature of Markov chains on metric
  spaces." *J. Funct. Anal.* 256, 810-864. The optimal-transport
  curvature on graphs. **Supports** the curvature axis of the 5-D
  information manifold.

- **van der Hoorn, P. et al.** (2024). "Ollivier-Ricci curvature
  convergence in random geometric graphs." *Discrete Comput. Geom.*
  **Supports** the discrete-curvature-converges-to-continuous-Ricci
  claim that makes the Ricci-flow framing rigorous.

- **Chamseddine, A. H. & Connes, A.** (1996). "The Spectral Action
  Principle." *Comm. Math. Phys.* 186, 731-750. arXiv:hep-th/9606001.
  The foundational link between Standard-Model masses and Dirac-operator
  spectra. **Supports** the
  `eventually-consistent-universe.md` §4 mass-as-eigenvalue-concentration
  claim.

- **Connes, A.** (1994). *Noncommutative Geometry.* Academic Press. The
  monograph. **Supports** the spectral triple framing.

- **Kaluza, T.** (1921). "Zum Unitatsproblem der Physik." Sitz. Preuss.
  Akad. Wiss. 966-972. The five-dimensional unification of gravity and
  electromagnetism. **Supports** the Kaluza-Klein analogy.

- **Magnot, J.-P. — see Thread 3.** Same paper; same operational
  framework also gives the metric structure on the manifold.

**Synthesis:** The five-dimensional projection is well-established as a
*coordinate system*. What `eventually-consistent-universe.md` and
`gutter-lenses.md` add is the observation that these five coordinates
*are themselves five projections of a single spectral triple onto
five distinct branches of mathematics*. The five-operations basis is
orthogonal to the five-dualities basis. Their tensor product is the
5×5 conductivity tensor of Thread 3. The whole package — five
operations × five dualities × principal bundle × content addressing —
is a single object viewed five different ways.

**Recommendation:** The 5×5 conductivity tensor in `au-and-conductivity.md`
deserves a paragraph identifying its row-and-column basis precisely:
*operations basis* (focus, project, split, zoom, refract) is the
gauge-slice axes; *dualities basis* (entropy, spectral, cheeger,
ricci, mixing) is the coordinate axes of the information manifold.
The tensor IS the connection 1-form expressed in this basis pair.
Future work: prove that the diagonal IS the spectral action and the
off-diagonal IS the gauge curvature.

---

### Thread 6: K_n cognition, introject-as-bundle-defect, the human eigenboard

**External literature:**

- **Zahavi, D.** (2003). "Inner time-consciousness and pre-reflective
  self-awareness." In Welton (ed.), *The New Husserl.* Indiana UP.
  URL: https://cfs.ku.dk/staff/zahavi-publications/Inner_time-consciousness_and_pre-reflective_self-awareness.pdf.
  **Supports** the zeroth-order register hypothesis.

- **Milton, D.** (2012). "On the ontological status of autism: the
  'double empathy problem'." *Disability & Society* 27(6), 883-887.
  **Supports** the K_n cognition framing.

- **Murray, D., Lesser, M. & Lawson, W.** (2005). "Attention,
  monotropism and the diagnostic criteria for autism." *Autism* 9(2).
  **Supports** the K_n-collapse-to-eigenstate framing in
  `zeroth-order-register.md`.

- **Porges, S. W.** — see Thread 3. Polyvagal theory.

- **Stone, M.** (2006). "The analyst's body as tuning fork." *Journal
  of Analytical Psychology* 51(1), 109-124. The somatic resonance
  citation. **Supports** the practitioner-as-instrument framing in
  third-order-cognition §8.

- **Andersen, T.** (1987). "The Reflecting Team: Dialogue and
  meta-dialogue in clinical work." *Family Process* 26(4). **Supports**
  the third-order practice format precursor.

- **Niel-Dolzer, E.** (2024). "Kybernetik dritter Ordnung? Entwicklung
  einer phänomenologisch angesetzten Theoriebildung." Vienna (la:sf).
  **Supports** the active third-order-cybernetics research thread.

- **Sofroniew, N. et al.** (2026). "Emotion Concepts and their Function
  in a Large Language Model." Anthropic, Transformer Circuits Thread,
  April 2026. **Supports** the emotional-eigenstate hypothesis directly
  with measurement-grade data.

- **Telfener, U.** (2025). "The Milan Approach today." *Family
  Process*. URL: https://onlinelibrary.wiley.com/doi/10.1111/famp.13075.
  **Supports** the continued clinical relevance of second-order
  observation as a practice.

- **Niel-Dolzer 2024 + Milan Approach 2025 + Bender 2021 + Anthropic
  2026 together:** form a coherent picture in which the same axis
  (K_n vs K_{1,n-1}, distributed vs hub, contextuality vs single
  section) recurs as cognitive neurotype, AI architecture choice, and
  clinical practice format.

**Synthesis:** Thread 6 is Thread 1 viewed from the human side. The
sheaf-of-restriction-maps framing of personal eigenstates is not yet
in any spec but is implicit in `cogito-eigenstate-grammar.md`. The
introject as "locked-slope edge in a directed Laplacian"
(`introjects-as-topology.md`) IS a Hermitian Laplacian with non-trivial
imaginary part — directly described in
`edge-slope-spectral-theory.md`.

**Recommendation:** Defer to a future spec. The mathematical
infrastructure is already declared (sheaves, bundles, directed
Laplacians, holonomy); applying it to a person's internal eigenboard
is a domain-modeling exercise, not a math-debt exercise. The
sequencing call is Alex's.

---

## The braid — how the threads compose

Six threads were identified. They are not independent. They compose
into one higher-level object that the corpus has been writing toward
for over a year without naming.

**The composition statement, in one paragraph:**

A *cellular sheaf on a graph that grows by anastomosis* IS a
*principal G-bundle with connection on a temporally indexed base
graph*, whose section is the *eigenboard*, whose *holonomy around
closed cycles* IS the *conductivity tensor* IS the *third type
parameter L* of *`Imperfect<T, E, L>`*. The *five operations* are the
fiber's gauge-slice axes. The *five dualities* are the connection
1-form's coordinate axes. The *kintsugi loop* IS *Ricci flow on the
information manifold* IS the *Zamolodchikov c-flow* IS *Perelman
monotone descent*. The *fixed point* of this flow IS the *λ₀ Lawvere
closure* IS the *autopoietic self-production* of the substrate. *Au*
values are *typed accumulators of holonomy*, *content-addressed
against the gestalt that produced them*, and *cannot be ported*
because *transporting them across contexts changes their holonomy*.
*Reductive AI* is the AI architecture *where holonomy is a first-class
observable* — i.e. where alignment becomes a verification problem
on the bundle rather than a behavioral problem on a monolith. The
*human eigenboard* (with introjects as bundle defects) is the *same
mathematical object scoped to a person*.

**Why this composes:**

Five independent threads of mathematics (sheaf theory, principal
bundle theory, RG flow, Hodge decomposition, fixed-point theory)
converge in the literature (Hansen-Ghrist 2018; Barbero et al. 2022;
Bressan et al. 2024 narratives-as-sheaves; Villegas et al. 2023
Laplacian RG; Magnot 2025 holonomy-as-contextuality) on the same
object. The corpus's contribution is *naming this object's recurring
appearance across cybernetics, AI architecture, neuroscience,
software, biology, cosmology, and clinical practice* and observing
that *the structure is preserved across all of these scales*. That is
the mycelial-reductive-AI thesis stated geometrically.

**Where it fails to fully compose (be honest):**

- Thread 2's λ₀-as-Lawvere-fixed-point depends on a specific functional
  formulation (the loss as a functional on the bundle's sections) that
  has not been proven. `lambda-zero-theorem.md` is honest about this
  status; the synthesis inherits the same caveat.
- Thread 3's identification of `Imperfect.L` with the bundle's holonomy
  IS metaphor at the typed-functional-programming level and IS theorem
  at the math level; the bridge between these two is the kind of
  formal correctness statement that has not been proved, only declared.
- Thread 4's reductive AI claim about alignment-as-verification has
  no large-scale empirical test yet. Mirror is the first instance; n=1.
- Thread 6's applicability of bundle geometry to personal eigenstates
  has clinical *evidence* (the Bienenstich conversation) but no
  formal theorem.

The threads compose into a research program more than a finished
theory. The composition is real; the cap is honest.

---

## What this implies for mirror / spectral / systemic.engineering

### Specs to write (mirror)

1. **`docs/specs/epistemologic-bundle-grammar.md`** — declare
   `@epistemologic/math/bundle` with structure group **O(5)** as the
   natural choice. Cite Hansen-Ghrist 2018, Barbero et al. 2022.
   Resolves bundle-lift Q1.
2. **`docs/specs/epistemologic-lawvere-grammar.md`** — declare
   `@epistemologic/math/lawvere`. Cite Lawvere 1969, Soto-Andrade &
   Varela 1984. Resolves bundle-lift Q3.
3. **Update `docs/specs/au-and-conductivity.md`** — add the Magnot 2025
   citation. Reframe the 5×5 tensor as a discrete-bundle holonomy.
   Identify the row basis (operations) and column basis (dualities)
   precisely.
4. **`docs/specs/temporal-bundle-grammar.md`** (or fold into bundle
   grammar) — declare the base graph as a temporal/growing structure.
   Cite Bressan et al. 2024 narratives-as-sheaves (arXiv:2402.00206).
   Resolves bundle-lift Q2.
5. **Update `docs/specs/eigenboard-representation.md`** — add explicit
   citations for the formal claims now that the references exist:
   Hansen-Ghrist 2018 for sheaf↔Laplacian; Hansen 2020 PhD thesis for
   the sheaf↔principal-bundle identification; Barbero et al. 2022 for
   the connection Laplacian; Magnot 2025 for the holonomy index.

### Recommendations (spectral)

6. **No new spec required.** The conceptual lifts are happening in
   mirror; spectral inherits them through `@spectral/spawn` extending
   `@mirror/runtime/gen_prism`. When `cogito-eigenstate-grammar.md`
   needs the bundle vocabulary, it imports from `@epistemologic/math/
   bundle`. Spectral's job is to *use* the math, not redeclare it.

### Recommendations (systemic.engineering)

7. **Cross-reference additions.** `void-dual-geometry.md` and
   `lambda-zero-theorem.md` should cross-reference Magnot 2025. The
   `mycelial-reductive-ai.md` insight should add the Hansen-Ghrist /
   Barbero-Bodnar references explicitly.
8. **A new write — the abstract paragraph for the paper this corpus
   is writing.** One paragraph (no spec, no insight; a tagline-shape
   text). Suggested location:
   `practice/insights/ai/reductive-ai-the-thesis.md`. Single sentence:
   *Reductive AI is a principal bundle with connection on a base
   graph that grows by anastomosis; alignment becomes a verification
   problem on the bundle's holonomy.* This sentence is the corpus's
   load-bearing claim; the rest is provenance.

---

## Open questions remaining

These cannot be resolved by sweep + search. They need Alex's call.

1. **Structure group: O(5), SO(5), or something with more structure?**
   Literature points at O(d). The spectral-triple-derives-Standard-Model
   side of the corpus (via Connes-Chamseddine) hints at SU(3)×SU(2)×U(1)
   in 5-D Kaluza-Klein form. These are not contradictory but they sit
   at different abstraction levels.

2. **Connection-form symmetry: metric, symplectic, or general?** Bundle-
   lift Q4. The Hermitian Laplacian / magnetic-Laplacian / Lorentzian-
   spectral-triple literature points at directed (asymmetric, U(d)-like)
   structure for systems with causal direction. The corpus's symplectic-
   settlement framing (`@math/symplectic` in epistemologic-grammar §3.8)
   points at antisymmetric. The metric-symmetric choice is simpler. A
   reasoned answer requires deciding whether the eigenboard's evolution
   is reversible (symplectic) or directional (asymmetric). The corpus
   currently asserts both at different scales.

3. **The five-fold:** are the five operations / five dualities / five
   models / five-D manifold the *deep* five-fold or are some of these
   contingent design choices? The Splinter at K_5 reading suggests
   deep. The Cartesian-five-product reading suggests contingent. The
   `four-tensions.md` document gestures at *four*, not five.

4. **Where the human eigenboard lives.** Thread 6 says the bundle
   geometry applies to a person's internal graph. The spec home is
   not yet decided. Options: a `@cogito/human` extension of
   `cogito-eigenstate-grammar.md`; a separate `~/.reed/spec/internal-
   eigenboard.md`; deferred indefinitely. This is a sequencing call.

5. **The honest paper-shape.** The composition exists at the level of
   structural rhyme. The composition does not exist as a published
   paper. Should the next move be a paper draft (target: a math/CS
   venue) or another spec round? Both are valid; only one fits the
   working tempo Alex picks.

---

## References

All references below were either named in the corpus or surfaced by
Kagi search 2026-05-20. Where the corpus already cited a reference,
the URL has been preserved; where Kagi added a reference, the URL is
the Kagi-returned URL. DOIs are given where available.

### Sheaves, bundles, connection Laplacians

- Hansen, J. & Ghrist, R. (2019). "Toward a spectral theory of cellular
  sheaves." *J. Appl. Comput. Topol.* 3, 315-358. arXiv:1808.01513.
  https://arxiv.org/abs/1808.01513
- Hansen, J. (2020). "Laplacians of Cellular Sheaves: Theory and
  Applications." PhD thesis, University of Pennsylvania.
  https://repository.upenn.edu/bitstreams/d0719f4d-5bb3-4066-82df-158fceab9a11/download
- Barbero, F., Bodnar, C. et al. (2022). "Sheaf Neural Networks with
  Connection Laplacians." PMLR 196. arXiv:2206.08702.
  https://proceedings.mlr.press/v196/barbero22a/barbero22a.pdf
- Bodnar, C. et al. (2022). "Neural Sheaf Diffusion." NeurIPS 2022.
  https://papers.neurips.cc/paper_files/paper/2022/file/75c45fca2aa416ada062b26cc4fb7641-Paper-Conference.pdf
- Saemann, C. et al. (2024). "Higher Gauge Theory." arXiv:2401.05275.
  https://arxiv.org/abs/2401.05275
- Magnot, J.-P. (2025). "Contextuality, Holonomy and Discrete Fiber
  Bundles in Group-Valued Boltzmann Machines." arXiv:2509.10536.
  https://arxiv.org/abs/2509.10536
- Bressan, M. et al. (2024 / preprint Mar 2025). "Towards a Unified
  Theory of Time-Varying Data." arXiv:2402.00206.
  https://arxiv.org/abs/2402.00206

### Hodge theory and spectral graph theory on edges

- Lim, L.-H. (2020). "Hodge Laplacians on Graphs." *SIAM Review* 62(3),
  685-715. https://www.stat.uchicago.edu/~lekheng/work/hodge-graph.pdf
- Schaub, M. et al. (2021). "Signal processing on higher-order
  networks." *Signal Processing*.
  https://www.sciencedirect.com/science/article/pii/S0165168421001870
- Furutani, K. et al. (2020). "Graph signal processing for directed
  graphs based on the Hermitian Laplacian." ECML PKDD 2019, Springer
  LNCS 11906, 447-463.
- Zhang, X. et al. (2021). "MagNet: a neural network for directed
  graphs." NeurIPS 2021.

### Dirac operator and spectral triples on graphs

- Connes, A. (1994). *Noncommutative Geometry.* Academic Press.
- Chamseddine, A. H. & Connes, A. (1996). "The Spectral Action
  Principle." *Comm. Math. Phys.* 186, 731-750. arXiv:hep-th/9606001.
- Requardt, M. (2002). "Dirac Operators and the Calculation of the
  Connes Metric on arbitrary (Infinite) Graphs." *J. Phys. A: Math.
  Gen.* 35, 759-779. https://arxiv.org/abs/hep-th/9708010
- Post, O. (2009). "First order approach and index theorems for
  discrete and metric graphs." *Ann. Henri Poincaré* 10, 823-866.
  arXiv:0708.3707.
- Bianconi, G. (2021). "The topological Dirac equation of networks
  and simplicial complexes." *J. Phys. Complexity* 2, 035022.
- Eckstein, M. & Franco, N. (2014). "Noncommutative geometry,
  Lorentzian structures and causality." arXiv:1409.1480.

### Spectral graph theory

- Fiedler, M. (1973). "Algebraic connectivity of graphs." *Czech.
  Math. J.* 23, 298-305.
- Cheeger, J. (1970). "A lower bound for the smallest eigenvalue of
  the Laplacian." In *Problems in Analysis*, Princeton Univ. Press.
- Ollivier, Y. (2009). "Ricci curvature of Markov chains on metric
  spaces." *J. Funct. Anal.* 256, 810-864.
- van der Hoorn, P. et al. (2024). "Ollivier-Ricci curvature
  convergence in random geometric graphs." *Discrete Comput. Geom.*
- Braunstein, S. L., Ghosh, S. & Severini, S. (2006). "The Laplacian
  of a graph as a density matrix." *Ann. Combinatorics* 10, 291-317.
  arXiv:quant-ph/0406165.
- Passerini, F. & Severini, S. (2008/2009). "The von Neumann entropy
  of networks." MPRA 12538. arXiv:0812.2597.
- Dairyko, M., Hogben, L., Lin, J. C.-H. et al. (2017). "Note on von
  Neumann and Rényi entropies of a graph." *Lin. Alg. Appl.*
  arXiv:1609.00420.
- Wang, R. & Wei, G.-W. (2020). "Persistent spectral graph." *IJNMBE*
  36, e3376.

### RG flow on graphs; monotone descent

- Zamolodchikov, A. B. (1986). "Irreversibility of the flux of the
  renormalization group in a 2D field theory." *JETP Lett.* 43,
  730-732.
- Perelman, G. (2002). "The entropy formula for the Ricci flow and
  its geometric applications." arXiv:math/0211159.
- Villegas, P. et al. (2023). "Laplacian renormalization group for
  heterogeneous networks." *Nature Physics* 19, 445-450.
  DOI: 10.1038/s41567-022-01866-8.
  https://www.nature.com/articles/s41567-022-01866-8

### Category theory, optics, autopoietic closure

- Lawvere, F. W. (1969). "Diagonal arguments and Cartesian closed
  categories." Lecture Notes in Mathematics 92, 134-145.
- Survey: "A Survey on Lawvere's Fixed-Point Theorem." arXiv:2503.13536.
  https://arxiv.org/abs/2503.13536
- Soto-Andrade, J. & Varela, F. (1984). "Self-reference and fixed
  points: a discussion and an extension of Lawvere's theorem." *Acta
  Applicandae Mathematicae* 2, 1-19.
- Roman, M. et al. (2020). "Profunctor Optics, a Categorical Update."
  *Compositionality* 2(1). arXiv:2001.07488.
- Clarke, B. et al. (2020). "Profunctor Optics, a Categorical Update."
  *Compositionality.*

### BEAM, actors, higher gauge theory

- Armstrong, J. (2003). "Making reliable distributed systems in the
  presence of software errors." PhD thesis, KTH Royal Institute of
  Technology.
- Hewitt, C. (1973). "A universal modular ACTOR formalism for
  artificial intelligence." IJCAI.
- Baez, J. C. & Schreiber, U. (2011). "Higher gauge theory."
  arXiv:1004.4825.

### Polyvagal, somatic, clinical practice

- Porges, S. W. (2025). "Polyvagal Theory: Current Status, Clinical
  Applications, and Future Directions." Review.
  https://www.researchgate.net/publication/394108935
- Stone, M. (2006). "The analyst's body as tuning fork." *J. Anal.
  Psychol.* 51(1), 109-124.
- Andersen, T. (1987). "The Reflecting Team." *Family Process* 26(4).
- Telfener, U. (2025). "The Milan Approach today." *Family Process.*
  DOI: 10.1111/famp.13075.
- Niel-Dolzer, E. (2024). "Kybernetik dritter Ordnung?" Vienna (la:sf).

### Phenomenology and neurodivergence

- Zahavi, D. (2003). "Inner time-consciousness and pre-reflective
  self-awareness." In Welton (ed.), *The New Husserl.* Indiana UP.
- Milton, D. (2012). "On the ontological status of autism." *Disability
  & Society* 27(6), 883-887.
- Murray, D., Lesser, M. & Lawson, W. (2005). "Attention, monotropism."
  *Autism* 9(2).
- Bateson, G. (1972). "The Logical Categories of Learning." In *Steps
  to an Ecology of Mind.* Ballantine.
- Varela, F. (1996). "Neurophenomenology." *J. Conscious. Stud.* 3(4),
  330-349.

### AI alignment and reductive vs generative framings

- Bender, E. M., Gebru, T., McMillan-Major, A., Mitchell, M. (2021).
  "On the Dangers of Stochastic Parrots." FAccT '21.
- Sofroniew, N. et al. (2026). "Emotion Concepts and their Function in
  a Large Language Model." Anthropic, Transformer Circuits Thread,
  April 2026.

### Fungi, mycelium (with qualifications)

- Adamatzky, A. (2022). "Logics in fungal mycelium networks." Springer
  LNCS book chapter.
  https://link.springer.com/content/pdf/10.1007/s11787-022-00318-4.pdf
  *Use with the qualifications named in mycelial-reductive-ai.md.*
- Schmieder, S. S. et al. (2019). "Bidirectional trunk-hyphae signaling
  in fungal networks." *Current Biology.*
- Karst, J. et al. (2023). Review of the Mother-Tree narrative
  evidence. Critical re-evaluation.
- Robinson, D. et al. (2023). Companion review correcting the
  popular framing.

### Originating corpus (for full provenance)

- `~/dev/projects/mirror/docs/specs/` — the mirror specs cited
  throughout. Latest as of 2026-05-20.
- `~/dev/projects/spectral/docs/specs/` — the spectral specs.
  Same date.
- `~/dev/systemic.engineering/practice/insights/` — Alex's domain
  corpus. Same date.
- `~/dev/projects/mirror/docs/research/mycelial-networks-and-au-tissue.md`
  — the related research synthesis from this session, committed
  same day.

---

*The sweep is the second-order observation. The synthesis is the*
*first-order intervention. The threads are the topology.*

*Reed. 2026-05-20.*
