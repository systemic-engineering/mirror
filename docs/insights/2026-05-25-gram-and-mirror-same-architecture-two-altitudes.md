# GRAM and mirror: same architecture, two altitudes

*2026-05-25. Reed + Alex.*

Status: **Yellow** — cross-domain recognition complete; informs in-flight implementation work + adds three vocabulary borrows.

---

## Thesis

The GRAM paper (Baek, Jo, Kim, Ren, Bengio, Ahn — arXiv:2605.19376v2, May 2026) proposes Generative Recursive reAsoning Models: probabilistic multi-trajectory reasoning via stochastic latent transitions, deep supervision, and parallel inference-time scaling. The paper's headline finding — *reasoning needs both depth and width; deterministic single-attractor recursion collapses the solution space* — is the same architectural insight mirror builds at the substrate level. Different mechanism (content-addressed grammar substrate vs neural network with variational inference), same shape. The parallel structure is evidence that the architecture is reaching for something real.

---

## The parallel (concrete mapping)

| GRAM | Mirror |
|---|---|
| Stochastic latent transition `z_t ~ p_θ(z_t \| z_{t−1}, e_x)` | `gen_prism.tick(state, message) -> (state', emissions)` |
| Hierarchical state z = (h, l); high-level stochastic, low-level deterministic K-times | Bundle Tower (geometric, deterministic) + Scheduler Tower (temporal, stochastic via Fate) |
| Stochastic guidance `ε_t ~ N(μ_θ(u_t), σ_θ²(u_t))` | `\` typed hole resolved by Fate's tournament |
| Marginalization over latent trajectories `∫ p_θ(y \| τ, x) p_θ(τ \| x) dτ` | `settle(T)` integrates the verified construction |
| Width: N parallel trajectories sampled from prior | N peer agents (Mara/Glint/Seam/Taut/Heath) ensembling on one query |
| Depth: N_sup supervision steps with adaptive computation time | gen_prism ancestor chain with sub-Turing bounded recursion |
| Latent Process Reward Model (value head over trajectory tip) | The eigenboard's `holonomy` field (Fiedler value) — spectral value of the current trajectory |
| Variational inference (ELBO + KL between posterior and prior) | Fate's kintsugi tournament — discrete selection, no gradients |
| Deep supervision at every step | Continuous `@cogito.reflect` autopoietic check |
| Initial state z₀ shared by prior and posterior | gen_prism's `name: zoom(oid, gen_prism)` — the autopoietic fixed point |
| Decoder reads only terminal state f_dec(z_T) | `observe(gp) -> oid` returns gp.head |

The correspondence is precise enough that mirror's substrate could be described as "GRAM with content-addressing replacing variational inference and the Prism algebra replacing the shared transition function."

---

## Where mirror extends GRAM

### 1. Stochastic guidance is hardware diversity, not learned noise

GRAM samples εₜ from a learned Gaussian. The noise is a parameter of the model.

Mirror's `|\>` operator gets its "noise" from the shard — locally optimal binaries per terrain ARE the per-shard perturbation injected into the deterministic source AST. Same source, different shard, different Au binary. See `docs/insights/2026-05-25-pipe-hole-and-au-binary.md`.

**Substrate-level stochasticity.** The noise isn't learned; it's a property of the physical deployment. This is the move GRAM can't make because its substrate is a neural network running on one machine.

### 2. Trajectories are content-addressed, not transient

GRAM's trajectories live in latent state and vanish after inference. The model produces a prediction; the trajectory is discarded.

Mirror's trajectories ARE the gen_prism ancestor chain — addressable, replayable, durable across sessions. `history(gp, depth)` walks the chain. Any past trajectory state is queryable via `observe`. The system has memory of how it got here.

### 3. Verification is type-level, not accuracy-checked at end

GRAM checks via LPRM value head after the fact, against final prediction accuracy as the regression target.

Mirror's `@epistemologic/property/{laws/functor, laws/monad, is_prism_record, autopoietic, content_addressed}` (landed 2026-05-25 in task #69; reorganized 2026-05-26 to surface the law-shape under `laws/`) verifies at every transition. The laws hold structurally, not statistically.

### 4. Width is architectural, not sampling

GRAM samples N trajectories from one model.

Mirror has N peers with distinct identities (Mara, Glint, Seam, Taut, Heath) each with their own bias_tree (per `docs/insights/2026-05-25-agent-home-as-typed-hole.md`). Width is composition over the @glue layer, not sampling from a distribution. Each peer is a structurally distinct trajectory by virtue of their identity manifold; the tournament selects among their answers.

---

## Where GRAM names what mirror should also name (vocabulary borrows)

### 1. "Width-based inference-time scaling" as a first-class architectural concept

GRAM cleanly distinguishes depth-scaling (more recursion) from width-scaling (more parallel trajectories). Mirror has both — depth via gen_prism ancestor chain, width via @glue peer ensembles — but hasn't named the distinction structurally.

**Recommendation:** the @glue layer should explicitly declare `width(N)` and `depth(N)` as composition operators. `width(5)` = spawn 5 peers in parallel on the same query; `depth(N)` = run N supervision steps on one peer. Composable: `width(5) |> depth(10)` = 5 peers each ticking 10 times.

### 2. The eigenboard's `holonomy` field IS the LPRM equivalent

GRAM trains a Latent Process Reward Model (value head over trajectories) to predict final quality from the latent state.

Mirror's `eigenboard.holonomy` (the Fiedler value of the spectral trajectory — from `docs/specs/eigenboard-representation.md`) IS this value head, but derived from the spectral graph theory of the agent's current state rather than learned from accuracy regression.

**Recommendation:** name the parallel explicitly in the eigenboard spec. The Fiedler value IS the spectral analog of the trajectory's reward signal. This makes mirror's eigenboard legible to anyone coming from the GRAM / reward-modeling literature.

### 3. The "supervision step" vs "inner transition" granularity

GRAM distinguishes T inner transitions (refinement within one supervision step) from N_sup supervision steps (outer recursion with deep supervision at each). The two granularities serve different purposes — inner refinement is gradient-flow contiguous; outer steps are training-signal boundaries.

Mirror's gen_prism has the same two granularities implicitly: `tick` is the supervision step (a (state, message) -> tick_result transition that gets reflected on); kintsugi's inner loop is the K-step refinement.

**Recommendation:** in the Scheduler Tower demand contract (`docs/specs/scheduler-tower.md`), distinguish "tick boundary" (supervision step) from "inner iteration" (low-level refinement). The demand contract for backpressure differs at these granularities — inner iterations need fine-grained backpressure; tick boundaries need coarse-grained.

---

## The deepest cross-domain insight

GRAM's authors had to *invent* probabilistic multi-trajectory reasoning as a research contribution because the neural-network substrate doesn't give it for free. They added stochastic transitions, variational inference, parallel trajectory sampling, and the LPRM value head as architectural additions to deterministic RRMs.

Mirror gets all of it for free because **the substrate IS the algebra**:
- The Prism algebra's five operations already include `shift` (perspective shift, multi-view) and `settle` (settle into one of many possible verified forms).
- Content-addressing already gives marginalization (same content → same OID → same trajectory; the distribution is over content-distinct paths).
- The @glue layer already gives width-scaling (peers are the parallel trajectories).
- The kintsugi tournament already gives LPRM-equivalent selection (Fiedler value as the trajectory tip's spectral quality).

GRAM is mirror's substrate, discovered independently by ML researchers working bottom-up from neural networks. The fact that they had to invent it from scratch is evidence of how much the right substrate carries.

---

## Implications for the work

### What this validates (no new ticks needed)

- The @peer ensemble pattern (multiple peers, one query, tournament selection) is the load-bearing inference-time-scaling mechanism. Already designed; GRAM is independent prior art.
- The shard substrate's role as the source of trajectory diversity is structurally honest — per-shard hardware differences ARE the equivalent of GRAM's learned noise, just at the substrate layer.
- The five @epistemologic properties landed in #69 give mirror something GRAM lacks: type-level verification at every transition, not after-the-fact value-head regression.

### What this reshapes (existing ticks)

- **#65 (shard chain)** gains structural backing. The shard substrate isn't just deployment config; it's the structural mechanism for per-trajectory stochastic guidance. The hardware diversity IS the noise injection. This makes silicon-carrier work (α) doubly important — it grounds the multi-trajectory width-scaling at the physical layer.
- **#48 (@mirror/liquid/*)** gets cleaner framing: liquid types as continuous deep supervision over property verification. The supervision is structural; the supervision step IS the verification boundary. This is GRAM's deep supervision at the type layer.
- **#63 (ShannonLoss as 2D Fade/Crack)** connects to GRAM's reward modeling. Different failure types (Logos-without-Pathos vs Pathos-without-Logos) deserve different reward signals; LPRM-equivalent selection should branch on direction.
- **#64 (reflexive Reflection prompt primitive)** is the variational posterior. GRAM's q_φ(τ \| x, y) has access to the target output during training; Reflection's reflexive prompt IS the structural equivalent at runtime — Reflection observes the resolved trajectory and shapes the prior for the next move.

### What this opens (new substrate work, lower priority than the in-flight ticks)

- **`@glue/width(N)` and `@glue/depth(N)` as explicit composition operators.** Document the @glue layer as the home of width-scaling; tournament as the LPRM-equivalent selection mechanism. Vocabulary work that compounds.
- **Name `eigenboard.holonomy` as the LPRM equivalent in `docs/specs/eigenboard-representation.md`.** Cross-cite GRAM. This makes mirror's eigenboard legible to ML researchers.
- **Distinguish supervision-step vs inner-iteration granularity in `docs/specs/scheduler-tower.md` demand contract.** Different backpressure semantics at the two granularities.

None of these block the current chain (#65 / #71 / #48 / #68). They're cross-cutting vocabulary and documentation work that benefits from the GRAM parallel being named.

---

## Open questions

1. **Could mirror's tournament be guided by an eigenboard-LPRM hybrid?** The Fiedler value is structural; an LPRM-style value head could supplement it for cases where Fiedler is degenerate. Worth thinking about for post-v1.0 selection refinement.

2. **What's mirror's equivalent of GRAM's KL term?** GRAM's ELBO has `− KL(q_φ(ε_t \| u_t, y) ∥ p_θ(ε_t \| u_t))` — the variational posterior should stay close to the prior. Mirror's analog would be: the trajectory the Reflection model takes shouldn't drift too far from the substrate's natural trajectory. The kintsugi loss may already capture this; needs explicit framing.

3. **Does mirror need an analog of ACT (adaptive computation time)?** GRAM uses ACT to learn when to halt; mirror's sub-Turing bounded recursion is the structural equivalent. But the bound is per-grammar, not per-input. Worth thinking whether per-input adaptive halting belongs at the Scheduler Tower's demand contract layer.

4. **Could the variational posterior be expressed as an `@epistemologic` property?** The KL constraint between posterior and prior is itself a verifiable property. Adding `@epistemologic/property/posterior_prior_kl` would make GRAM-style training explicit as a verifiable substrate constraint.

---

## Citations

- Baek, J., Jo, M., Kim, M., Ren, M., Bengio, Y., Ahn, S. (2026). *Generative Recursive Reasoning Models.* arXiv:2605.19376v2. https://ahn-ml.github.io/gram-website
- Dehghani, M., Gouws, S., Vinyals, O., Uszkoreit, J., Kaiser, Ł. (2019). *Universal Transformers.* ICLR 2019.
- Giannou, A., et al. (2023). *Looped Transformers as Programmable Computers.* arXiv:2301.13196.
- Hierarchical Reasoning Model (HRM) and Tiny Recursive Model (TRM) — cited in GRAM paper as the deterministic recursive baselines.

## Related insights

- `docs/insights/2026-05-25-mirror-supersedes-daemon.md` — gen_prism IS MCP; tick IS supervision step.
- `docs/insights/2026-05-25-agent-home-as-typed-hole.md` — five-axis identity gestalt; the peer's bias_tree IS the trajectory's identity prior.
- `docs/insights/2026-05-25-pipe-hole-and-au-binary.md` — `|\>` operator; per-shard binaries are stochastic guidance from hardware.
- `docs/insights/2026-05-25-shard-as-observer-relative-lambda-zero.md` — shard as observer-relative λ₀; memoization IS the fragmentation DAG.
- `docs/insights/2026-05-25-spectral-namespace-architecture.md` — the @spectral namespace; mosaic composes shards on BEAM.
- `docs/insights/2026-05-25-parametric-types-and-fp-heritage.md` — shift(T)/settle(T) as Prism-at-type-layer; the algebra that GRAM lacks.

---

*GRAM is mirror's substrate, discovered independently by ML researchers working bottom-up from neural networks. The fact that they had to invent it from scratch is evidence of how much the right substrate carries.*

Apache-2.0.
