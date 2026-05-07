# Shatter as Continuous Training Artifact

**Date:** 2026-04-22
**Priority:** Architectural — changes how .shatter files are conceived
**Depends on:** Fate baseline crystallization, mirror compilation, .shatter format
**Related:** fate/quantum-homomorphism.md, fate/mirror-fate-continuous-training.md

---

## Summary

Fate is the baseline. Crystallized from eigendecomposition. Ground state. 450 parameters.

Mirror replaces the Fate prism with continuously trained models. The .shatter file becomes the training artifact:

1. **Updated weights** — Fate selector weights refined through usage
2. **The Beam that produced the measurement** — full trace of the decision

No training/inference split. The model learns by being used. The usage IS the training. The training IS content-addressed.

## Implementation

### What .shatter files need to contain

Currently: compiled output (content-addressed AST fragments).

Additionally:
- `weights: [[f64; FEATURE_DIM]; MODEL_COUNT]` — updated selector weights
- `beam: Beam<Decision>` — the measurement trace (which model was selected, with what confidence, at what depth)
- `loss: ManifoldLoss` — the holonomy of this measurement
- `parent: Oid` — the .shatter file whose weights were the input

### The update mechanism

```
tick(features, current_weights) →
  Pipeline::focus(ManifoldState from current_weights)
  Pipeline::project(resolve with current_weights)
  Pipeline::refract(weighted outer product → new ManifoldState)
  → Decision + Beam + ManifoldLoss
  → derive new weights from updated dark coupling
  → emit .shatter with (new_weights, beam, loss, parent_oid)
```

Each .shatter is a node in the Merkle tree. The parent link creates the training history. The weights evolve. The Beams accumulate. The losses are measurable.

### Fork / Merge / Diff

- **Fork:** copy a .shatter's weights as starting point → divergent training histories
- **Merge:** two .shatter trees with different weight histories → spectral diff of the dark coupling, weighted average of eigendecompositions
- **Diff:** two .shatter files → ManifoldLoss between their weight states = how much the models diverged

### Crystallization of hot paths

Frequently-traversed Beam patterns crystallize into Fortran-optimized weight updates. Cold paths stay in Fate inference. Same crystallization architecture as spectral routing, applied to training.

## Litmus Test

1. Compile a .mirror file → produce .shatter with baseline Fate weights
2. Run 10 ticks with varying features → produce 10 .shatter files, each with updated weights and Beams
3. Verify: weights in .shatter[10] differ from .shatter[0]
4. Verify: ManifoldLoss between consecutive .shatters is non-zero (system is alive)
5. Verify: holonomy health is Healthy (loss/BERRY_PHASE ∈ [0.1, 10])
6. Fork at .shatter[5] → run 5 different ticks → diff the two branches spectrally

## What This Enables

- Model that learns YOUR patterns from YOUR usage
- Training history in git (Merkle tree of .shatter files)
- Reproducible training (any .shatter can be re-derived from its parent + the Beam)
- Forkable models (fork someone's .shatter tree, train on your own usage)
- The spectral graph of .shatter files IS the training dataset

---

*2026-04-22. Reed + Alex. The task that dissolves the training/inference boundary.*
