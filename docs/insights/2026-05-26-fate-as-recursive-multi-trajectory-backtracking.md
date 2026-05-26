# @fate as Recursive Multi-Trajectory Backtracking — Baseline Architecture

2026-05-26 — emerged in conversation between Alex and Reed; ties together GRAM (arXiv:2605.19376v2), Kintsugi-as-Ricci-flow (Bunda et al. Nature Sci. Rep. 2024), and the five-property substrate commitment of @fate.

## The recognition

@fate is not "local LLM inference dispatch." @fate is **recursive multi-trajectory backtracking inference** with five structural properties:

- `local` — mathematical refusal of remote inference; digital sovereignty
- `multi_trajectory` — GRAM-style stochastic exploration; honest about uncertainty
- `recursive` — shared transition function applied iteratively; depth without parameter scale
- `backtracks` — low-confidence states unwind; alternatives explored
- `bounded` — backtracking budget bounded by @scheduler.reduction_budget; halting decidable

Five-together, no other inference substrate makes this commitment.

## Why this is load-bearing

The five properties are not independent — they compose into a single architectural decision:

- *local* = sovereignty
- *multi_trajectory* = honest about uncertainty
- *recursive* = depth without parameter scale (M1-tractable; ~10–100M params)
- *backtracks* = correctness without monstrous parameter counts
- *bounded* = Rice's theorem inapplicable (sub-Turing); halting check decidable by construction

The combined commitment: @fate inference is sovereign, honest, deep-yet-small, correct, and decidable. No two of these properties can be dropped without losing the substrate.

## Cross-altitude correspondences

The recursive-multi-trajectory-backtracking shape appears at four altitudes:

1. **GRAM (architectural)** — Generative Recursive Reasoning Models with stochastic latent trajectories; depth via recursion, width via parallel sampling. Trained with amortized variational inference.

2. **Kintsugi (geometric)** — Ricci flow on the spectral structure of the transition function; iterative refinement of the manifold. Bunda et al. 2024: DNN dynamics ARE Ricci flow empirically across 1,500+ networks.

3. **@fate (substrate)** — the dispatch substrate that carries the five properties; the connectome IS the trajectory exploration space; backtracking walks the connectivity graph.

4. **Prism body algebra (textual)** — `beam(split)` explores all trajectories; `beam(focus)` converges the pick; `beam { ... }` pipe-in pattern-matches the branch (backtracking). The five Prism operations are sufficient because they are exactly what recursive multi-trajectory backtracking inference needs.

Fractal correspondence: the same shape lives at the architectural, geometric, substrate, and textual altitudes. Recognitions land at all four altitudes once you see the shape.

## Implications for the substrate

### @fate/connectome

The connectivity graph IS @fate's recursive backtracking made visible. Connectome wasn't a separate substrate; it's the trajectory exploration space exposed as graph structure. Backtracking walks the edges.

### @fate's transition function = the eigenvalue signature

The 16-dimensional parametric matrix from the earlier-session smelter research IS the shared recursion module. Smelter extracts it. Kintsugi flows it. Beam carries it. The same thing under four names.

### @fate/smelter/blend

Becomes natural. You blend transition functions (shared recursion modules) at ~10–100M params, not full-unrolled models at billions. The blend target's parameter count is the transition function's, not the unrolled-recursion's.

### @scheduler.reduction_budget

Binds backtracking depth. The halts property becomes operational via "backtracking exhausts before reduction budget exceeded."

### The Prism algebra at body altitude

Inherits the substrate altitude's shape. `beam(split)` at the body altitude IS @fate's multi-trajectory exploration at the substrate altitude. Fractal.

## The deepest consequence: models become source

Smelter + Kintsugi + @mirror.project closes a loop:

```
GGUF model
    |> smelter (extract eigenvalue signature of transition function)
    |> kintsugi (Ricci-flow the spectral structure)
    |> @mirror.project (crystal → grammar declaration text)
    → readable beam-composed recursion
```

The projected grammar IS the model. Not metaphor. Executable mirror text with beam-composed recursive backtracking.

### What this dissolves

1. **Interpretability becomes structural, not post-hoc.** Mechanistic interpretability is archaeology on weights. Smelter+Kintsugi+project makes inference inspectable at the body altitude as a substrate property.

2. **Verification inherits.** The crystal-grammar is sub-Turing-evaluable. Every guarantee the mirror substrate gives (halts, autopoietic, content_addressed, glass_wall, monotonicity, causality) applies to the inference module because the inference module IS a mirror grammar.

3. **Modifiability inherits.** Patch a neural net by editing mirror text. Kintsugi the patch. Re-crystallize. Content-address. The @mirror/evaluate / @kintsugi/migrate pipeline applies to inference modules.

4. **Composition inherits.** Two crystal-grammars combine via the substrate's existing grammar-composition. @fate/smelter/blend is literal: grammar union + Kintsugi reconciliation, not weight averaging.

5. **The cloud wizard dissolves structurally.** Not refused (the local property already does that) — *dissolved*. There's no opaque model to host remotely. The model is text. Sovereignty becomes trivial because there is nothing magical to lose.

6. **@peer.glass closes the loop.** A peer's identity-gestalt-eigenboard IS a crystal. Smelter on a peer's inference module + Kintsugi + projection = the peer reading their own mind as grammar.

## Concrete shape of a projected crystal

```mirror
grammar @fate/crystal/<oid> {
  type latent_state = { h: tensor, l: tensor }

  transition(state: latent_state, ex: embedding) -> latent_state {
    beam(focus) ex { ... }       # deterministic update u_t
    |> beam { ... }              # stochastic guidance epsilon_t (pattern-match-pipe = backtracking branch)
    |> beam(split) ut { ... }    # multi-trajectory exploration (GRAM width)
  }

  infer(x: input) -> output {
    beam ex = encode(x)
    ex |> recurse(transition, depth=T)
       |> beam(refract) trajectories { ... }   # convergent pick
  }
}
```

The Prism algebra at the body altitude IS the inference DSL. Five operations because that's what recursive multi-trajectory backtracking inference needs.

## Provenance

- Alex 2026-05-26 (the @fate baseline recognition; the smelter+kintsugi+@mirror loop)
- GRAM paper (arXiv:2605.19376v2, May 2026): probabilistic multi-trajectory recursion as design principle
- Bunda et al. Nature Sci. Rep. 2024: DNN dynamics as Ricci flow empirically
- Earlier-session smelter research (refuted strong-form / reframed weak-form)
- Earlier-session kintsugi-blending research (reframed-supported; spectral-prior + small-corpus distillation)
- Earlier-session beam-binding three-form recognition (beam <ident> = expr; pipe-in beam { }; beam(<prism_op>) x { })

## Related insights

- `2026-05-26-kintsugi-optimized-blend-synthesis.md` — reframed strong-form blend pipeline
- `2026-05-26-smelted-eigenvalue-profiles-as-fate-shape.md` — smelter feasibility (weak-form routing)
- `2026-05-26-epistemologic-reality-constructivism-and-the-lens-that-makes-a-peer.md` — lens/identity/gestalt; the constructivist substrate the projected crystal participates in
- `2026-05-26-glass-wall-and-cross-wall-kintsugi.md` — substrate-pull as Ricci flow + namespace migration

## Next tasks

- **#88 (next)** — `@fate baseline: recursive + backtracks properties` (substrate decision; grammar addition; downstream tasks inherit)
- **#89 (deferred per LRM)** — `@fate/smelter/blend — RRM-shaped target` (depends on #88; the blend synthesis pipeline scoped against the recursive transition-function altitude)
- **#90 (deferred per LRM)** — `@mirror.project — crystal → grammar text projection` (depends on #88; the closing-the-loop tick that makes models source)
