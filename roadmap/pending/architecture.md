# The Architecture

```
┌────────────────────────────────────────────────┐
│  Application layer (Phase 7)                       │
│  spectral.engineer deployment;                     │
│  @spectral/garden (open, vetted corpus dist.);     │
│  @spectral/portal (open, typed transport);         │
│  spectral-db distribution; user-facing CLI         │
├─────────────────────────────────────────────────┤
│  Loaded grammars (Phase 2–4)                       │
│  @mirror/glass; @fragmentation; @code/rust;        │
│  @nl/markdown; @data/markdown; @code/llvm/ir;      │
│  @kintsugi; @fate; @peer/{reflection,...};         │
│  @epistemologic/reality/{lens, identity, ...}      │
├─────────────────────────────────────────────────┤
│  Scheduler Tower (Phase 5 — temporal)               │
│  gen_prism with demand contracts;                  │
│  Bundle Tower + KMS-shaped backpressure            │
├─────────────────────────────────────────────────┤
│  Bundle Tower (Phase 0 — geometric)                 │
│  Fiber → Connection → Gauge → Transport → Closure  │
├─────────────────────────────────────────────────┤
│  Prism algebra (Phase 0)                            │
│  Prism trait; the five operations                  │
├─────────────────────────────────────────────────┤
│  NumericalPrism backends (Phase 6)                  │
│  LapackBackend (CPU, today);                       │
│  MetalBackend (Apple GPU, dev-zero-cost);          │
│  OpenCLBackend (cloud, non-optional for v1.0)      │
├─────────────────────────────────────────────────┤
│  fragmentation (Phase 6 + Phase 4)                  │
│  DAG VCS substrate; SpectralCoordinate;            │
│  generated from @fragmentation + @code/rust        │
├─────────────────────────────────────────────────┤
│  @io kernel (Phase 6 — minimal Rust surface)        │
│  syscalls (fs / net / process / time);             │
│  LAPACK Fortran FFI; SHA-1 (git interop only)      │
└─────────────────────────────────────────────────┘
```

The stack reads top-down as user-facing-to-substrate, or bottom-up as substrate-to-user-facing. Phase ordering is roughly bottom-up: substrate first (the @io kernel + fragmentation + NumericalPrism), then the algebra (Prism + Bundle Tower), then the temporal layer (Scheduler Tower), then the loaded grammars (parser/resolver/emitter self-descriptions), then the application layer (Phase 7's spectral-db + spectral.engineer).

## Cited prior art (the lineage)

All specs and architectural decisions cite this corpus inline:

- **`~/dev/systemic.engineering/practice/insights/spectral-db/dirac-operator-on-graphs.md`** — the unifying operator (D = d + d*); Connes distance; spectral action; KMS states.
- **`~/dev/systemic.engineering/practice/insights/spectral-db/turing-eigenvalue-thread.md`** — Turing 1952's Laplacian eigenvalue selection mechanism; the 74-year lineage to mirror.
- **`~/dev/systemic.engineering/practice/insights/coincidence/void-dual-geometry.md`** — the eight Narcissus-Splinter dualities; λ₀ = 0 as the void axis; the origin of the SpectralCoordinate manifold.
- **`~/dev/systemic.engineering/practice/insights/coincidence/quantum-graph-theory.md`** — witnessed computation; MBQC; fragmentation as the computational ground.
- **`~/dev/systemic.engineering/practice/insights/coincidence/heterogeneous-numerical-prism.md`** — NumericalPrism + backend abstraction; Anna Jakobs's thesis as architectural reference.
- **`docs/insights/2026-05-24-backpressure-as-modular-flow.md`** — Scheduler Tower as discrete modular flow; gen_prism IS a Stage.
- **`~/dev/systemic.engineering/practice/collaborators/anna-wolf/master_jakobs.pdf`** — Anna Jakobs (2012), *Integration von OpenGL-Visualisierungstechniken in GPU-Anwendungen.* FH Aachen / FZJ. The shared-memory bus pattern.
- **Connes, A. (1994).** *Noncommutative Geometry.* The spectral triple framework.
- **Turing, A.M. (1952).** *The Chemical Basis of Morphogenesis.* The discrete Laplacian on graphs; eigenvalue selection.
