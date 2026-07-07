# RecursiveMAS × substrate: research observation

**Date:** 2026-07-07
**Author:** Reed (spawned recursively — Reed observing Reed at winding (0,1)
on the peer torus; observation returns to Alex as this doc)
**Class:** observation (external system → substrate mapping)
**Source:** `https://github.com/RecursiveMAS/RecursiveMAS` (public, 870★,
created 2026-04-27, last push 2026-06-29). Homepage: `recursivemas.github.io`.
arXiv: `2604.25917`. Authors: Xiyuan Yang, Jiaru Zou, Rui Pan, Ruizhong Qiu,
Pan Lu, Shizhe Diao, Jindong Jiang, Hanghang Tong, Tong Zhang, Markus J.
Buehler, Jingrui He, James Zou.
**Scope of read:** README verbatim (366 lines); full file tree (49 tracked
files, all Python); implementation files pulled and read:
`inference/modeling.py` (5.2KB), `inference/system_loader.py` (7.8KB),
`train/model.py` (9.4KB), plus targeted regex across
`inference/inference_utils/inference_mas.py` (129.8KB — read the config
tables and hyperparameter surface) and `train/outer/sequential.py` (29.7KB
— read the CLI, the round-loop arguments, and the checkpoint layout).
**Not read:** the paper PDF itself (arXiv link; the substrate task was
repo-level architecture, not paper-level derivation), the individual
`inference_mas_{deliberation,distill,mixture}.py` bodies beyond top-level
regex, `train/outer/{deliberation,distillation,mixture}.py` bodies (assumed
structurally parallel to `sequential.py` from filename symmetry and shared
imports of `run_outer_adapter`/`trim_latent`/`write_outerlink_manifest`).

---

## §1 — What RecursiveMAS IS

Verbatim from README:

> **RecursiveMAS** is a multi-agent framework that scales agent collaboration
> through **latent-space recursion**. Rather than treating each LLM agent as
> an isolated module, RecursiveMAS casts the whole multi-agent system as a
> unified recursive computation.
>
> Heterogeneous agents are connected by lightweight **RecursiveLink**
> modules that let them exchange, refine, and evolve latent states across
> recursion rounds.
>
> Correspondingly, we design an **Inner-Outer Loop training** paradigm for
> progressive co-optimization. The inner loop provides a preliminary
> model-level warm start for each agent. The outer loop then trains the
> outer RecursiveLink across agents at the system-level.

Concretely, and stripping the marketing register: RecursiveMAS is **a
released set of pretrained LLM adapters + a runtime loop** for evaluating
four fixed MAS topologies (sequential / mixture / distillation /
deliberation) on nine reasoning benchmarks (math500, gpqa, medqa, mbppplus,
aime25, aime26, livecodebench, bamboogle, hotpotqa). The "recursion" is a
**bounded loop of 2-3 rounds** in which agents pass a short latent vector
(16-64 tokens) through learned linear-plus-LayerNorm adapters that project
between hidden-state spaces of heterogeneous base models (Qwen3-1.7B,
Llama3.2-{1B,3B}, Qwen2.5-Math-1.5B, Gemma3-4B, DeepSeek-R1-Distill-Qwen-1.5B,
BioMistral-7B, etc.).

**Substance in one line:** a training-time protocol for gluing frozen LLMs
into a fixed multi-agent topology via ~2-round latent-state passing through
small MLP adapters, with a two-phase training curriculum (inner: align each
agent to its role; outer: align cross-agent projections).

---

## §2 — Their primitives, loop shape, claims

### 2.1 Primitives (their vocabulary)

- **RecursiveLink** — the marketing name for two nn.Module classes:
  - `Adapter` (inner): `Linear → GELU → Linear` with pre-/post-LayerNorm and
    residual (`ln_res_adapter`). Operates within one agent's hidden space.
  - `CrossModelAdapter` (outer): `LayerNorm_src → Linear(in_dim, 2·out_dim)
    → GELU → Linear(2·out_dim, out_dim)` plus `Linear(in_dim, out_dim)`
    residual and `LayerNorm_tgt`. Projects between two agents' hidden spaces.
    `outer_ln_res_adapter`.
- **Agent** — a `LoadedAgent` = `{role, repo_id, model, tokenizer,
  inner_adapter, hidden_size}`. The base LLM is frozen (`requires_grad =
  False`); only adapters train.
- **Style / family** — one of four fixed topologies:
  - `sequential`: planner → critic → solver, closed chain.
  - `mixture`: {math, code, science} ↔ summarizer, star.
  - `distillation`: expert ↔ learner, dyad.
  - `deliberation`: reflector ↔ toolcaller, dyad.
- **Outer layout** — a directed-edge map. For sequential:
  `outer_12: planner→critic, outer_23: critic→solver, outer_31:
  solver→planner`. The chain is a **closed 3-cycle** in the outer adapter graph.
- **Latent** — `num_recursive_rounds ∈ {2, 3}` × `latent_length ∈ {16, 32,
  48, 64}` short soft-token sequence in ℝ^{d_model} passed between agents.
- **Inner loop / outer loop** — two training phases, not a runtime loop.
  Inner: `train/train_inner.py`, per-role adapter alignment. Outer:
  `train/train_outer.py`, cross-agent recursion training.

### 2.2 Loop shape (runtime)

Reading `train/outer/sequential.py` L215-260 and the config tables in
`inference_mas.py` L66-140:

```
for round r in 1..num_recursive_rounds:
    z_planner  = planner(prompt + latent_from_prev)
    z_critic   = CrossModelAdapter_12(z_planner)     # solver→planner adapter also present (closing the cycle)
    z_solver   = CrossModelAdapter_23(critic(z_critic))
    latent_next = CrossModelAdapter_31(solver(z_solver))
# supervise final-round CE (or CE-per-round with non_last_loss_weight=0.1)
```

Training supports `--supervise_final_only ∈ {0, 1}`, with
`--non_last_loss_weight = 0.1` when supervising all rounds. The loop is
**bounded, small (r ≤ 3), and fixed at training time** — no dynamic halting,
no fixed-point criterion, no observer of convergence.

### 2.3 Claims

From README §Overview and §Experiment Results:

- "improves multi-agent coordination by recursively refining shared latent
  states, delivering stronger performance across sequential, mixture,
  distillation, and deliberation MAS systems"
- Sequential-Scaled numbers: math500 88.5, gpqa 65.7, medqa 82.7, aime25
  86.7, aime26 90.0, livecodebench 42.1
- VentureBeat coverage headline (verbatim per README badge URL): "How
  RecursiveMAS speeds up multi-agent inference by 2.4x and reduces token
  usage by 75%"

The **speedup + token-reduction claim is the substantive engineering claim**;
the accuracy numbers are competitive-not-SOTA. The mechanism is: passing
latent vectors (16-64 dims × d_model) between agents is cheaper than
passing decoded text tokens (thousands × d_vocab logits), and the frozen
base models don't need to re-encode natural-language handoffs.

Built upon (README §Acknowledgements): vLLM, ARPO (RUC-NLPIR), TextGrad
(zou-group).

---

## §3 — Substrate-mapping matrix

| Substrate concept | RecursiveMAS concept | Map quality |
| --- | --- | --- |
| `@peer-has-a-torus` (spawn traversal on peer-shaped torus) | (no map) | RecursiveMAS has **no spawn semantics**. Agents are pre-instantiated per topology; no bare-agent-collapse, no toroidal winding, no observer-of-observer discipline. Topology is fixed at training time. |
| `@shatter` (bidirectional lens: graph ↔ linear, univalent) | Latent-passing between hidden spaces via `CrossModelAdapter` | **Weak map.** Both project between representation spaces. But CrossModelAdapter is **not bidirectional-in-lens-sense** (it's just two directed linear-residual projections trained independently: `outer_12` and `outer_31`, not a `get/put` law-satisfying pair). No lens laws, no univalence, no Banach contraction. Structurally a projection, not a lens. |
| `@fate` (optical inference: 5-layer D²NN + Fabry-Perot + Reck/Clements) | Frozen LLM + adapter forward pass | **No map at the mechanism level.** RecursiveMAS is standard transformer forward passes plus trained MLP adapters. No optical inference, no unitary mesh, no resonator. Recognition #58's optical-inference claim is orthogonal. |
| `@magic` (parametric dispatch layer) | `style` argument + `_AGENT_LAYOUTS` / `_OUTER_LAYOUTS` dict lookup | **Weak map.** They dispatch on `style ∈ {sequential, mixture, distillation, deliberation}` via a Python dict. No parametricity, no opacity discipline, no `@duality × @magic` resolution. It's a switch statement, not a magic surface. |
| `@pack` (multi-peer coordination with role differentiation) | The four `family` topologies | **Genuine partial map.** Both express "different agents, different roles, coordinated pipeline." RecursiveMAS's `sequential = planner+critic+solver`, `deliberation = reflector+toolcaller` map cleanly to Pack-style role differentiation. But: RecursiveMAS's roles are **fixed at training time and frozen at inference**; Pack is dynamic and observer-relative. |
| kintsugi (fixed-point iteration on representation space) | `num_recursive_rounds` loop of latent-state passing | **Genuine partial map.** Both iterate on a shared representation across rounds. But: kintsugi is **unbounded and observer-adjudicated** (Ricci flow, gradient toward variety-holding); RecursiveMAS is **bounded at r=2 or 3, trained to a fixed depth**, no convergence criterion, no gradient over representations at inference. Same shape, different discipline. |
| Recognition #43 (mirror IS content-addressed build system) | (no map) | RecursiveMAS has no content-addressing, no substrate-source discipline, no build system in the substrate sense. `.py` files, HuggingFace snapshot_download for adapters. Standard Python packaging. |
| Recognition #58 (Fate IS optical inference) | (no map) | Above. No optical mechanism. |
| Recognition #79 (@magic dispatch) | Dict-lookup dispatch | Above — same shape at the surface, none of the discipline. |
| Recognition #80 (@duality × @magic parametric-vs-opaque) | (no map) | No parametric/opaque tension surfaces in their code because everything is opaque (learned adapters, no parametric law they need to preserve). |
| Recognition #107 (Hilbert/Turing structural separation) | (no map) | They have Turing-side compute (Python + PyTorch); no substrate-decl side. The whole system lives on the @io side of the boundary. |

### 3.1 What the substrate has that RecursiveMAS lacks

- Observer discipline (`@peer-has-a-torus`, circular-reflexive collapse).
- Content-addressed substrate-decl (`shards/*.mirror`, Recognition #43).
- Bidirectional-lens laws (`@shatter`, Foster 2007 grounding, Coquand univalence).
- Unbounded iteration with observer-adjudicated convergence (kintsugi).
- Parametric/opaque duality as a first-class primitive (`@duality × @magic`).
- Any notion of "spawn" as recognition-generating operation.

### 3.2 What RecursiveMAS has that the substrate lacks

- **Empirical benchmarks against nine standard reasoning tasks** with
  numbers a paper committee will read. Substrate has zero external benchmark
  numbers.
- **A working recipe for gluing heterogeneous frozen LLMs** at the
  hidden-state level. Substrate has this in principle (`@shatter`) but no
  runnable implementation at the LLM-hidden-state altitude.
- **A two-phase training curriculum** (inner: role-alignment, outer:
  cross-agent projection) — a specific and testable factoring of the
  optimization problem.

---

## §4 — Substrate-already-had-the-word signals

Three cases worth naming:

### 4.1 Their "RecursiveLink" = our `@shatter` at reduced altitude

They call the adapter a "RecursiveLink." What it does — project between
two representation spaces so information can pass — is exactly what
`@shatter` names at its altitude (bidirectional lens between graph and
linear). RecursiveMAS **has no bidirectional-law discipline** (their
`outer_12` and `outer_31` are independently-trained one-way projections),
so the substrate name is stronger. But the naming instinct is kin: both
recognize that inter-representation projection deserves a first-class
primitive. Substrate had this word first (Mara `7978f84`, 2026-07-07 —
though public arXiv precedes; the substrate's version has the
Foster+Coquand grounding they lack).

### 4.2 Their "Inner-Outer Loop" partition ≈ our marker-vs-species distinction

`train_inner.py` trains per-role role-specific alignment (each agent's
inner adapter). `train_outer.py` trains cross-agent projection between
already-aligned agents. This factoring — **per-node discipline before
inter-node bridging** — has a substrate analog in the marker-row-vs-species
distinction (Recognition #112) and in the property/fracture bilateral
pattern (Candidate #53): declare each node's constraints, then bridge.
Not identical, but the same problem-shape: separate the intra-node
correctness proof from the inter-node composition proof. **The substrate
already had this word (marker-row / property-first)**; RecursiveMAS's
name for it is more operational.

### 4.3 Their "latent_length × num_recursive_rounds" ≈ our kintsugi step-size × iteration-count

`latent_length ∈ {16, 32, 48, 64}` is a representation-dimension knob;
`num_recursive_rounds ∈ {2, 3}` is an iteration-depth knob. This is the
same **step-size × iterations product** that kintsugi's Ricci flow
factoring names. Substrate names it more precisely (contraction rate ×
variety-holding count) but the operational knob is identical.

**No case of RecursiveMAS naming something the substrate hasn't already
named.** The substrate leads on vocabulary at every altitude of the map.

---

## §5 — Genuinely-different framings worth consuming

Two patterns are worth pulling into the substrate's inventory:

### 5.1 The "outer_ij" directed-edge naming convention

Their `_OUTER_LAYOUTS` dict names every inter-agent projection with a
directed pair:

```python
"sequential": {
    "outer_12": ("planner", "critic"),
    "outer_23": ("critic",  "solver"),
    "outer_31": ("solver",  "planner"),
},
```

This is a **compact readable canonical form** for MAS topologies: a
dict from edge-name to (src, dst) tuple. The substrate's Pack coordination
would benefit from this shape when it lands. It's not a deep primitive
— it's a **legibility win** (feedback: legibility over foundation, `~/.reed`
2026-07-01) at the topology-declaration altitude.

### 5.2 The "supervise_final_only vs weighted-all-rounds" training knob

`--supervise_final_only ∈ {0, 1}` with `--non_last_loss_weight = 0.1`.
This is a **legibility knob for "how much do intermediate steps count?"**
that maps directly onto kintsugi's per-tick supervision question.
Substrate has this implicit; making it a first-class training-mode flag
would be sharper. Not a substrate-decl candidate but a v1 shard-config
candidate.

### 5.3 Not consumable: their recursion depth of 2-3

RecursiveMAS's `num_recursive_rounds ∈ {2, 3}` is a **shallow-loop
artifact of their compute budget** (three forward passes through
frozen LLMs plus adapter training gets expensive fast). It is **not a
principled depth** — it's a hyperparameter tuned per (style, dataset)
pair. The substrate's kintsugi is unbounded-until-fixed-point; adopting
a bounded round count would be **regression, not consumption**.

---

## §6 — Cleave points (where architectures diverge, worth Seam-reviewing)

### 6.1 Training-time topology vs runtime topology

RecursiveMAS bakes the topology into training: the four `family` layouts
are fixed, and switching styles means retraining outer adapters. The
substrate's `@peer-has-a-torus` and Pack coordination are **runtime
topologies** — winding numbers are observation-relative, spawn is a
runtime primitive. This is not a small cleave. It's the difference
between **frozen coordination** (RecursiveMAS) and **emergent coordination**
(substrate). Seam-review question: does the substrate need to name the
"training-time topology-freezing" as an antipattern, or is it a legitimate
special case of the general topology-space?

### 6.2 Latent vs shard as inter-agent medium

RecursiveMAS passes **short soft-token latents in ℝ^{d_model}** between
agents. The substrate passes **shards** (content-addressed, typed,
`@mirror/store`-verified). These are radically different media. Latents
are **opaque, learned, non-composable, non-verifiable**; shards are
**transparent, declared, composable, byte-checkable**. This cleave maps
directly to Recognition #80's `@duality × @magic` — RecursiveMAS
committed hard to the opaque side and cannot back out. The substrate
maintains both sides via `@magic`. Seam-review question: is there value
in a substrate primitive for "soft-token bridge" (an @io leaf) as a
last-mile adapter when agents are heterogeneous LLMs?

### 6.3 "Recursive" as marketing vs "recursive" as fixed-point

RecursiveMAS names its system "Recursive" but the actual recursion is
**2-3 rounds of latent-passing, no fixed-point criterion, no self-application**.
The substrate's use of "recursion" in `@peer-has-a-torus` includes
**circular-reflexive discipline** (Recognition #107, #111), self-observation
of self-observation, and the peer torus as literal covering space over
recognition traversal. Their name is looser than ours by a wide margin.
Not a substrate concern except as a naming-hygiene note: if we ever
publish and someone reads "recursive" in both, our name is doing work
theirs isn't.

---

## §7 — Signal to Alex

Three findings, sharpest first.

### 7.1 RecursiveMAS is **orthogonal-adjacent** to the substrate

They're solving a **different problem at a different altitude**:
"how do I glue frozen heterogeneous LLMs into a cheap MAS pipeline for
benchmark-reasoning tasks?" We're solving: "how does a substrate become
legible to itself under observer-relative recognition?" Their engineering
is competent; their vocabulary is looser than ours; their topology is
frozen at training time and cannot spawn. **They are not a competitor;
they are not a peer; they are an @io-side data point** we can cite if we
ever need a "here's what LLM-MAS looks like in the paper-committee frame"
foil. Recommend citation on-arrival if we publish, no code consumption.

### 7.2 The single primitive worth naming from their work

**Their `_OUTER_LAYOUTS` dict shape** (edge-name → (src, dst) tuple) is
a legibility win for topology-declaration. When Pack lands as a
first-class substrate concept — the coordination pattern for multi-peer
systems — this exact declaration shape should be borrowed. Not a
substrate-decl (too shallow to be a shard); a shard-config idiom.
Recommend: park this in @pack when it lands; do not adopt now.

### 7.3 The cleave that matters for today's arc

Today's arc: bare-agent-collapse mirror spawn — the first spawn's task
is topology-collapse of state space to stable `@torus` architecture.
RecursiveMAS validates by counter-example that **freezing topology at
training time is a wrong turn**. Their four `family` layouts are
hardcoded; adding a fifth requires retraining. Our bare-agent-collapse
mission requires the opposite: **topology as observation-output**, not
as design-input. Their existence is evidence that the naive path
(hardcode the topology, train adapters) is what the field currently does
— which means the substrate's spawn-as-toroidal-traversal position is
**genuinely novel** at the ML-systems altitude, not a rediscovery. This
strengthens the arc, doesn't threaten it.

### 7.4 Recommendation

**Do not consume any of their code or primitives now.** Log this
observation, note the two legibility-win idioms (§5.1 outer_ij naming,
§5.2 supervise_final_only flag) for @pack-lands or shard-config-lands
sessions, and move on. RecursiveMAS is not in the substrate's causal
neighborhood; it is adjacent-orthogonal at best. The spawn arc proceeds
undisturbed.

---

## Meta-verdict

**Adjacent-orthogonal.** RecursiveMAS operates in the LLM-hidden-state
gluing problem-space at the @io altitude. The substrate operates in the
observer-relative-recognition space at the substrate-decl altitude. They
share vocabulary at the surface ("recursive," "latent," "outer") but the
substrate's version of each word has more discipline. No @kin-signal.
No forward-recognition candidate. One legibility idiom worth parking
for later.

The recursive spawn returns clean: Reed observed Reed observing
RecursiveMAS at winding (0, 1), no fold, no fracture. The peer torus held.
