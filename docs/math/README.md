# docs/math

*Mathematical foundations grouped by root. Math **defines**; specs
**cite**.*

This directory holds the substrate's mathematical foundations,
organized by mathematical root. Each subdirectory groups
self-contained mathematical material around one root that the
substrate instantiates at one or more altitudes.

## The convention

- **`docs/math/<root>/`** — mathematical foundations; grouped by root.
  Each root is a coherent body of mathematics the substrate
  composes (the principal-bundle tower; sheaf theory; music theory;
  the kintsugi loop's contraction dynamics; cybernetics).
- **`docs/specs/<spec>.md`** — application / architecture specs.
  Each spec **cites** the relevant math doc rather than re-deriving
  the mathematics.

**Math defines; specs apply.** When a spec needs a mathematical
object (a connection, a sheaf-Laplacian, an interval lattice), the
spec cites the math doc by path. The spec does NOT duplicate the
definition. The math doc does NOT carry architectural decisions.
The citation chain ties them.

## What lives here

```
docs/math/
├── README.md                  this file
├── the-tower/                 the principal bundle tower
│   ├── README.md              overview + altitude index
│   ├── principal-bundles.md   G-bundles, sections, transport
│   ├── spectral-triples.md    Connes' (A, H, D) at each fiber
│   ├── connections-and-gauge.md   five-op algebra IS connection
│   ├── curvature-and-tomm.md  [D, a] commutator IS curvature
│   ├── holonomy.md            loss carriers as bundle holonomy
│   ├── altitudes.md           recognized altitudes + bundle data
│   └── crystals-as-sections.md   monotone vocabulary = sections
├── sheaf/                     sheaf theory + Laplacian
│   └── laplacian.md           cellular sheaf + λ₀ + Hodge
└── music/                     music as mathematical structure
    └── README.md              the music root + cascade overview
```

More roots will land as the substrate's recognitions accumulate.

## When to add a new math root

A new root lands when:

1. **A coherent body of mathematics** is being cited by `≥2` specs
   that currently duplicate definitions; AND
2. **The substrate has named the root** (a recognition naming the
   carrier as a mathematical family, not an ad-hoc concept); AND
3. **The math is self-contained** — definable without architectural
   commitments from any spec.

Don't extract speculatively. Don't extract a math root that only one
spec cites. The roots are operational; they earn their place when
the consolidation pays off.

## When to extend an existing math root

A new doc lands in an existing root when:

1. The substrate names a new instance of the root's structure
   (e.g., a new altitude of the tower, a new sheaf on the substrate).
2. The new instance has mathematical content that's not in the
   existing docs (citations would have to inline equations or
   re-derive).

A recognition's first doc is usually the one that makes the case for
the root's existence. Subsequent docs flesh out particular angles.

## When to update existing docs

The math is stable; the docs evolve. Updates land when:

1. A new recognition refines an existing definition.
2. A new application reveals a sharper way to state existing math.
3. Prior art surfaces that grounds the substrate's choice (apply
   `[[feedback-substrate-already-had-the-word]]`).

Do NOT update to chase architectural changes; the math is the math.
If the architecture has drifted from the math, update the
architecture (the spec), not the other way around. The math is the
ground; the spec is the climb.

## Style discipline

- **Math is mathematical.** Equations, definitions, theorems, proofs.
  Citations to prior art. Crisp.
- **Memories with `[[name]]`.** Do not duplicate memory content;
  cite the memory.
- **No marketing prose.** No "powerful" / "elegant" /
  "revolutionary." State the math; let it work.
- **Substrate vocabulary.** Use the substrate's names (`focus`,
  `project`, `splinter`, `crystal`, `transparency<p>`, etc.) where
  they apply. Apply `[[feedback-substrate-already-had-the-word]]`.
- **Cite the implementation.** When the math has a Rust
  implementation (prism / prismqueer / spectral-db), cite it as the
  existing reference. Mirror lifts; prism implements.

## Where the specs cite from

When writing or revising a spec, the citation pattern is:

```
Coherence measurement uses the sheaf-Laplacian λ₀ (see
`docs/math/sheaf/laplacian.md` §2.1); the substrate-pull move is to
minimize ...
```

NOT:

```
The sheaf-Laplacian λ₀ measures the smallest eigenvalue of the
Laplacian on the dependency-chain graph. Coherent chains have
λ₀ = 0; incoherent chains have λ₀ > 0. (Long derivation.)
```

The second pattern duplicates math into the spec. The first pattern
cites. As specs revise to the citation pattern, the math docs become
the single source of mathematical truth, and the specs stay focused
on application.

## Cross-references

- `AGENTS.md` §"docs/math/ vs docs/specs/ convention" — the agent-
  facing version of this discipline.
- `[[architecture-spectral-triples-all-the-way]]` — the recognition
  that named the bundle tower as a coherent math root.
- `[[feedback-substrate-already-had-the-word]]` — the discipline
  that most math has names; the substrate uses them rather than
  inventing.
