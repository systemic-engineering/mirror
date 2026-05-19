# mirror refract -- infer the .spec

*2026-05-19. Reed. Spec.*

Status: **Red**

Depends on: `property-projection.md`, `epistemologic-import-resolver.md`,
`epistemologic-grammar.md`, `liquid-types-for-mirror.md`, `gutter-lenses.md`

---

## What It Does

`mirror refract <path>` reads a project, measures its topology,
and writes a `mirror.spec` with inferred properties below `---`.

The measurement IS the product. The spec IS the measurement.
The engineer did not ask which properties apply. The topology said.

## The Pipeline

1. Detect language: scan file extensions, select `@code/*` grammar
2. Tokenize all source through the grammar lens, build AST
3. Build the grammar graph (nodes = declarations, edges = references)
4. Compute eigenvalues (Fiedler, Cheeger, entropy, Ricci, mixing)
5. Match eigenvalue patterns against `@epistemologic/*` property thresholds
6. Infer which properties apply
7. Write `mirror.spec`:
   - Above `---`: spec declaration (language, targets, structure)
   - Below `---`: inferred property imports

## The Spec Format

```mirror
spec @my_project {
  in @code/rust           # detected language
  in @prism               # always

  target binary <| @code/kernel/arm64 <| std
}

---

# inferred from topology at [timestamp]
# fiedler: 0.043, narcissus: 0.71, entropy: 3.2

in @epistemologic/property/hub_detection      # narcissus > 0.5
in @epistemologic/property/dead_code          # unreachable nodes detected
in @epistemologic/math/cheeger                # bottleneck detected
in @epistemologic/bio/elegans                 # connectivity pattern match
```

The declaration above `---` is the programmer's. The observation below
`---` is the compiler's. Both are content-addressed. Both are part of the
OID. The spec file IS a `.mirror` file with the `---` separator.

## Property Inference Rules

Each `@epistemologic/*` grammar declares thresholds. When the measured
topology exceeds a threshold, the property is imported below `---`.

The thresholds come from the five dualities (gutter lenses):

| Measurement | Threshold | Property | Duality |
|-------------|-----------|----------|---------|
| narcissus > 0.5 | hub detected | `@epistemologic/property/hub_detection` | spectral |
| fiedler < 0.05 | fragile | `@epistemologic/property/fragility` | spectral |
| dead nodes > 0 | unreachable | `@epistemologic/property/dead_code` | mixing |
| cheeger < 0.1 | bottleneck | `@epistemologic/math/cheeger` | cheeger |
| entropy < log(n)/2 | ordered | `@epistemologic/physics/ricci` | entropy |
| mixing > O(n) | isolated | `@epistemologic/property/isolation` | mixing |
| pattern ~ elegans | worm-like | `@epistemologic/bio/elegans` | spectral |
| harmonic > 0 | irreducible debt | `@epistemologic/math/hodge` | cheeger |
| override_ratio > 0.5 | epistemic crisis | `@epistemologic` | entropy |

The verdicts are continuous, not boolean. A property that barely exceeds
the threshold gets `partial(observation, loss)`. A property far past the
threshold gets `pass`. The loss IS the distance from the threshold.

## The Detection Grammar

Language detection IS a grammar operation. File extensions map to `@code/*`:

```
.rs     -> @code/rust
.ex     -> @code/elixir
.ts     -> @code/typescript
.py     -> @code/python
.c      -> @code/c
.go     -> @code/go
.mirror -> @mirror (self-referential: mirror measures mirror)
```

When multiple languages are detected, the spec imports all matching
`@code/*` grammars. The topology is measured per-language, then composed.
The `|\>` operator weights the composition by file count.

## spectral.engineer

The website IS this command compiled to WASM.
Paste a git URL. Get a `mirror.spec`. See the topology.
No install. No CLI. The measurement IS the product.

The WASM target runs the same pipeline:
1. Clone (shallow) the repo
2. `mirror refract .`
3. Render the spec + eigenvalue visualization
4. The visualization IS the five lenses from `gutter-lenses.md`

## Connection to the Pipeline

`refract` IS the measurement. The measurement writes the spec.
The spec declares the properties. The properties inform kintsugi.
kintsugi fills holes. The spec updates. The loop.

```
refract -> spec -> properties -> kintsugi -> refract -> spec -> ...
```

Each iteration: `e^(n+1) < e^(n)`. The loss decreases. The properties
sharpen. The spec converges to the project's ground state.

## Connection to Liquid Types

The Spectral Liquid inference from `liquid-types-for-mirror.md` IS the
mechanism behind property inference. The qualifier set Q IS the set of
`@epistemologic/*` grammars. The Dirac operator IS the constraint solver.
The eigenvalues determine which qualifiers (properties) hold.

`mirror refract` runs the Spectral Liquid inference pipeline:
1. HM-like shape detection (language, structure)
2. Constraint generation (property thresholds as constraints)
3. Spectral solving (eigenvalues of the property Laplacian)
4. Verdict projection (which properties hold, to what degree)

The result below `---` IS the automatically inferred refinement type
of the project. No annotation. No configuration. Measurement.

## Connection to Property Projection

The spec file follows the same `---` convention as all `.mirror` files
(from `property-projection.md`). Above `---`: what the engineer declared.
Below `---`: what the compiler measured. The LSP reads below `---` for
gutter colors. Git diffs show when the topology changed.

## What Makes This Different

Every linter in existence: "here's what's wrong with your code."
`mirror refract`: "here's the topology of your code, and here are
the mathematical frameworks that describe it."

Not warnings. Measurements. Not rules. Properties. Not opinions.
Eigenvalues.

The spec doesn't say "fix this." The spec says "your connectivity
pattern matches C. elegans. Your bottleneck structure matches Cheeger.
Your flow decomposition has harmonic residual." The engineer decides
what to do. The compiler measures what IS.

---

*The measurement writes the spec.*
*The spec declares the properties.*
*The properties are not rules. They are eigenvalues.*
*The eigenvalues are not opinions. They are the topology.*
