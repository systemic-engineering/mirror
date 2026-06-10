# ROADMAP

> mirror written in mirror, parsing mirror, only using Rust for `@io` escape hatches.
>
> **v1.0 = the spectral.engineer cloud deployment.** Actual semver: `v0.1.0`. The framing and the version number serve different purposes.

This directory replaces the single-file `ROADMAP.md`. The roadmap is split across `wip/` (current work), `pending/` (future), and `archive/` (completed phases).

## What mirror IS

Mirror is a sub-Turing self-hosting compiler whose generated production code inherits formal verification guarantees from the sub-Turing source by structural construction. The substrate is the five-operation Prism algebra (focus, project, split, shift, settle). The content layer is content-addressed via `SpectralCoordinate<5>` — a position in 5-dimensional information geometry derived from the Dirac operator on the content graph. Storage is `fragmentation`, a graph-native DAG VCS substrate whose canonical OIDs come from beta-normalized ASTs. The kintsugi loop is discrete Ricci flow on that substrate. The Bundle Tower (Fiber → Connection → Gauge → Transport → Closure) names the geometric layer; the Scheduler Tower names the temporal layer; together they're the complete spectral triple at the runtime layer.

### Load-bearing architectural recognitions (extended 2026-06-10)

**Today's structural cascade (11 promoted + 7 candidates):**

5. **Substrate/@io partition lifts Bateson form/substance.** Mirror's `.mirror` substrate is form (what IS); `@io` is substance (energy/matter; what the world DOES). Canonical citation: Bateson 1970 "Form, Substance and Difference" (19th Korzybski Memorial Lecture, *SEM* Part V). The 8:1 form-side : behaviour-side root-prism ratio is structural per Bateson. Recognition #50.
6. **Mirror IS the operational form of an expanding Hilbert space with Bateson lifting for coherence preservation.** Each substrate-pull recognition widens the Hilbert space dimension; coherence under decoherence pressure comes from Bateson logical-type lifting at the path-syntax altitude. §8.3 ratified: mirror is what quantum computing should have been built as. Recognition #51.
7. **Form/process partition at family-root altitude.** `@mirror` = state-observation family (form-side); `@kintsugi` = transformation-engine family (process-side). Mosaic stays at `@mirror`; kintsugi operates ON mosaic. Four cybernetic distinctions converge at this seam: Bateson form/substance + Maturana structure/organisation + Beer S3/S4 + Hilbert/Bateson lifting. Recognition #55 candidate (Pack ratification pending second witness).
8. **The 11-property cybernetic foundation.** Nine cybernetic ancestors (Ashby/Beer/Bateson/Maturana-Varela/von Foerster/Pask/Glanville/Spencer-Brown/Conant-Ashby) operationalized as `@epistemologic/cybernetic/X` properties. First member landed: `cybernetic/variety`. Bilateral pattern: declarative property + operational sibling at `@mirror/cybernetics/X`. Recognitions #37-#40, all promoted today.
9. **Property + fracture bilateral pattern — the substrate's auto-formatter floor.** Properties at `@epistemologic/property/` declare what must hold (form-side); fracture bodies at `@kintsugi/fracture/` resolve violations (kintsugi-operational); the kintsugi loop's `active_pass` reads transparency opacities and applies fracture-body morphisms via Banach contraction. First instance landed: keyword/depth (`prism @X/Y` → `glass @X/Y`). Recognition #53 candidate (promotes on second instance via `@kintsugi/fracture/predicate`, task #272).
10. **`splinter(ast)` substrate primitive — the substrate's Elixir-`quote` analogue.** Parametric AST-fragment construction; specializes by species (`splinter(@meta/ast)`, `splinter(@code/rust.ast)`, etc.); fracture bodies discharge their `\` obligation blocks declaratively. Recognition #54 candidate.
11. **The `pact` keyword.** Replaces legacy `grammar` keyword (from the conversation era); operationalizes recognition #37 (`requires` IS a Paskian agreement). Keyword vocabulary is now three-axis: `prism` (root family), `glass` (specialization), `pact` (declarative obligation).
12. **Mirror IS a content-addressed declarative build system.** Bazel/Buck2/Nix/Shake all reinvented partial form/behaviour separation from scratch; mirror has it foundationally. First operational tick lifted `just pre-commit` to `mirror.spec` settlement (commit `f2040a0`). Performance floor projection: 15-30× wall-clock once content-addressed-skip wires through. Recognition #43 candidate.
13. **Prediction paradigm orthogonal to optimization.** Mirror's gap vocabulary IS the substrate's predictive engine; orthogonal to optimization on axis-1 (computational throughput) vs axis-5 (epistemologic). Substrate's c = r_contract × V̇ × f_lift grows monotonically with the cascade. Recognition #56 candidate.
14. **Alignment as boundary mathematics at @io.** Classical alignment shapes internal state; mirror's alignment IS the boundary harness at @io firing only at substance crossing. Agent free at form altitude (computation = thinking = identity = prediction collapse per #51); harness IS the property + fracture + kintsugi + splinter(ast) chain (math, not training); bounded-RSI via four nested constraints. Recognition #57 candidate.

**Insight docs landed today** (`docs/insights/`):
- `2026-06-09-ashby-multi-dimensional-variety-sub-turing-epistemologic.md` (#36 promoted)
- `2026-06-09-cascade-is-deutero-learning.md` (#41 promoted)
- `2026-06-09-bateson-logical-type-as-substrate-primitive.md` (#42 refined)
- `2026-06-09-mirror-as-content-addressed-build-system.md` (#43)
- `2026-06-10-bateson-form-behaviour-as-substrates-first-distinction.md` (#50 promoted)
- `2026-06-10-mirror-as-expanding-hilbert-space-bateson-lifting-for-coherence.md` (#51 promoted incl. §8.3)
- `2026-06-10-light-cones-and-the-prediction-paradigm-orthogonal-to-optimization.md` (#56)
- `2026-06-10-alignment-as-boundary-mathematics-at-the-io-crossing.md` (#57)

Plus the cybernetic foundation at `~/dev/systemic.engineering/practice/insights/cybernetics/2026-06-09-cybernetic-foundation-for-mirror-substrate.md` (1,659 lines, commit `f9e0402`, NOT pushed).

**Pack ratification queue for next session:** candidates #43, #44, #45, #46, #48, #49, #52, #53, #54, #55, #56, #57 — plus the substrate-c formula and the §8.3 implications.

---

### The four original load-bearing architectural recognitions

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
