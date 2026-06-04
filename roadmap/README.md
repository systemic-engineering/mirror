# ROADMAP

> mirror written in mirror, parsing mirror, only using Rust for `@io` escape hatches.
>
> **v1.0 = the spectral.engineer cloud deployment.** Actual semver: `v0.1.0`. The framing and the version number serve different purposes.

This directory replaces the single-file `ROADMAP.md`. The roadmap is split across `wip/` (current work), `pending/` (future), and `archive/` (completed phases).

## What mirror IS

Mirror is a sub-Turing self-hosting compiler whose generated production code inherits formal verification guarantees from the sub-Turing source by structural construction. The substrate is the five-operation Prism algebra (focus, project, split, shift, settle). The content layer is content-addressed via `SpectralCoordinate<5>` — a position in 5-dimensional information geometry derived from the Dirac operator on the content graph. Storage is `fragmentation`, a graph-native DAG VCS substrate whose canonical OIDs come from beta-normalized ASTs. The kintsugi loop is discrete Ricci flow on that substrate. The Bundle Tower (Fiber → Connection → Gauge → Transport → Closure) names the geometric layer; the Scheduler Tower names the temporal layer; together they're the complete spectral triple at the runtime layer.

### The four load-bearing architectural recognitions

1. **Sub-Turing source → Turing-complete generated substrate with structural verification inheritance.** Mirror generates `fragmentation`'s Rust source by compiling `@fragmentation + @code/rust`. The generated Rust can't do anything the grammar didn't ask for, because the source is sub-Turing. CompCert-class "compiler correctness without compiler trust" — at production substrate scale.
2. **`SpectralCoordinate<5>` + beta tree normalization + content-addressing = deterministic memory layout as a structural property.** Same content always lands at the same physical address, by construction. Not an optimization — a property the architecture has once the pieces are in place.
3. **Kintsugi IS discrete Ricci flow over the substrate's edge graph.** The Banach contraction argument in the formatter spec IS the discrete analog of Perelman-style monotonicity. The loss function IS the Ricci curvature being smoothed. The tournament IS Ricci surgery.
4. **Bundle Tower (geometric) + Scheduler Tower (dynamic) = the spectral triple at runtime.** Backpressure IS the discrete modular flow on the spectral triple. Mirror's compiler gets a temperature.

None of these are aspirational. They're either landed or in the immediate path between Phase 0 and Phase 7.

## Critical path

```
Phase 0 (here, partial: F-1 done, beta normalization done, specs landed)
  │
  ▼
Phase 1 (the gate — zero holonomy, unified AST/resolver)
  │
  ├─────► Phase 2 (parser self-description)
  │        │
  │        ▼
  │     Phase 3 (resolver self-description)
  │        │
  │        ▼
  │     Phase 4 (emitter; fragmentation generated)
  │        │
  │        ▼
  ├─────► Phase 5 (Reflection + Scheduler Tower)
  │        │
  │        ▼
  └─────► Phase 6 (@io boundary + NumericalPrism backends)
           │
           ▼
        Phase 7 (self-hosted + deployed at spectral.engineer)
```

**Phase 1 is the gate.** Nothing meaningful moves until boot holonomy is zero and the resolver is one path.

**Phases 2–4 are sequential** but each gates the next via bootstrap tests (self-parse, self-resolve, self-render).

**Phases 5 and 6 can parallelize** in their later stages — the Scheduler Tower spec (5) can land protocol implementation without needing every NumericalPrism backend (6); the LapackBackend wrap (6) can land without needing the Scheduler Tower's bus integration (5).

**Phase 7 is the deployment validation.** Everything below must be green.

### What can run in parallel within phases

- Phase 1 tasks 1–3 (tokenizer fixes) are independent of tasks 4–7 (type unification + cleanup). Two contributors can work in parallel.
- Phase 6 NumericalPrism backends — LapackBackend, MetalBackend, OpenCLBackend — can be implemented in parallel after the operation enum + Backend trait land.
- Cross-cutting Tracks A, B, C can advance in parallel during the phases they span.

## Navigator

### `wip/` — current work

- [`wip/phase-0-current-state.md`](./wip/phase-0-current-state.md) — where we are; what exists in Rust + `.mirror`; recently landed work; honest gaps
- [`wip/v1-launch.md`](./wip/v1-launch.md) — v1.0 launch tiers; what spectral.engineer ships; semver framing
- [`wip/cloud-deployment.md`](./wip/cloud-deployment.md) — hardware targets; Anna Jakobs's pattern; readiness gates J; deployment topology

### `pending/` — future work

- [`pending/architecture.md`](./pending/architecture.md) — the stack diagram + cited prior art lineage
- [`pending/destination.md`](./pending/destination.md) — Phase 7 destination framing
- [`pending/phase-1-boot-grammar.md`](./pending/phase-1-boot-grammar.md) — the gate; zero holonomy
- [`pending/phase-2-parser-self.md`](./pending/phase-2-parser-self.md) — mirror's syntax described as `.mirror` grammar
- [`pending/phase-3-resolver-self.md`](./pending/phase-3-resolver-self.md) — mirror's type system described as `.mirror` grammar
- [`pending/phase-4-emitter-self.md`](./pending/phase-4-emitter-self.md) — emitter self-description + fragmentation generated
- [`pending/phase-5-reflection-scheduler.md`](./pending/phase-5-reflection-scheduler.md) — Reflection + Scheduler Tower
- [`pending/phase-6-io-numerical-prism.md`](./pending/phase-6-io-numerical-prism.md) — @io boundary + backends
- [`pending/phase-7-self-hosted-deployed.md`](./pending/phase-7-self-hosted-deployed.md) — self-hosted + deployed
- [`pending/tracks.md`](./pending/tracks.md) — cross-cutting work tracks (A: NumericalPrism, B: fragmentation rewrite, C: Scheduler Tower, D: shard, E: spectral namespace, F: portal substrate, G: epistemologic/reality, H: spectral/garden, I: petri-net SEL, J: flang migration — deferred per LRM, **K: runtime elevation — HamiltonScheduler + SpectralSupervisor**)
- [`pending/runtime-elevation.md`](./pending/runtime-elevation.md) — (NEW 2026-06-01) the architectural shift from content-addressed compiler to BEAM-like runtime; HamiltonScheduler (per-shard, Margaret-Hamilton-named) + SpectralSupervisor (cross-shard, the closed engine); Body=prism+glass+AST restructure; Pure trait; `@mirror/lens/transit`; hard + soft realtime
- [`pending/open-questions.md`](./pending/open-questions.md) — Q1–Q13 design decisions
- [`pending/references.md`](./pending/references.md) — insight docs, specs, collaborator prior art, foundational mathematics, protocol prior art, related projects

### `archive/` — completed phases

Placeholder until Phase 1 lands. See [`archive/README.md`](./archive/README.md).

## Footer

*Phase 1 is the gate.*  
*Phases 2–4 are sequential; each gated by a self-referential bootstrap test.*  
*Phases 5–6 parallelize in their later stages.*  
*Phase 7 is the deployment validation.*  
*v1.0 = spectral.engineer cloud deployment.*  
*Actual semver: v0.1.0.*  
*Anna Jakobs's shared-memory pattern is non-optional for cloud.*  
*Apple Silicon UMA is the dev-bonus that makes the abstraction zero-cost on Mac.*  
*The insight docs are cited prior art across the corpus.*  
*The Bundle Tower is geometry; the Scheduler Tower is dynamics; backpressure is the discrete modular flow.*  
*Shards are observer-relative; mosaics compose shards; the spectral engine is closed; adapters are open; portals are the seam.*  
*`|\>` is composition with a typed hole; Fate resolves it per local hardware; binaries are Au; the source stays verified.*  
*gen_prism IS MCP; transport layers disappear when the substrate is the algebra.*  
*The portal is `@io.socket` + content-addressed subspace + shard-frame; the wire is WS handshake → `@fragmentation/frame` → bidirectional eigenvalue stream; the open portal IS a gen_prism.*  
*We prove we halt: `@epistemologic/property/halts` operationalizes the sub-Turing escape from alignment undecidability.*  
*The glass wall is a property. Substrate-pull is self-enforcing.*  
*Heuristics compose via the spectral triple. Decades of engineering wisdom integrate without picking winners.*  
*The peer's lens is what makes this peer this peer. Constructivism, structurally.*  
*The garden is the cellar. The curator is the vintner. The peer is the sommelier.*  
*Mirror is sub-Turing. The system that emerges is Turing-complete.*  
*Holonomy = error = convergence delta. Three names; one number; standard numerical methods transfer.*  
*HamiltonScheduler is the agent's content window manager — named for Margaret Hamilton, Apollo 1202.*  
*SpectralSupervisor is the coordinator without authority — the closed engine that elevates mirror to a BEAM-like runtime.*  
*Maximum mirror, minimal Rust.*  
*`@mirror/reality/shard` IS mirror's CRDT layer — bounded semilattice, strong eventual consistency by construction.*

Apache-2.0.
