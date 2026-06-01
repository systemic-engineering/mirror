# Cross-Cutting Work Tracks

Three work tracks span phases. Each lands incrementally as the phases progress, but each has its own internal coherence.

## Track A: NumericalPrism backend stack

Spans Phases 5 (Scheduler integration) + Phase 6 (backend implementations).

- **A.1:** Operation enum + Backend trait + LapackBackend wrap (Phase 6 start).
- **A.2:** MetalBackend with MSL kernels (Phase 6 middle).
- **A.3:** OpenCLBackend with OpenCL kernels (Phase 6 middle; load-bearing for v1.0).
- **A.4:** Scheduler Tower integration — bus selection routes to the right backend (Phase 5 + Phase 6).

## Track B: Fragmentation rewrite

Spans Phase 4 (codegen pipeline) + Phase 6 (the generated crate consumes prism-core's NumericalPrism). The R-tick decomposition in `docs/specs/fragmentation-as-generated.md` is the implementation path: R-0 (audit) → R-1 (`@code/rust` extension) → R-2 (`@fragmentation.mirror`) → R-3 (pipeline end-to-end on prism_bridge.rs as first target) → R-4 (rest of fragmentation generated) → R-5 (mirror consumes generated fragmentation) → R-5b (MetalBackend) → R-6 (archive `coincidence/` to `_archive/`).

## Track C: Scheduler Tower

Lives primarily in Phase 5 but reaches into Phase 6 for the CPU/GPU bus integration. The S-tick decomposition in `docs/specs/scheduler-tower.md` is the implementation path. Mara's deepening pass resolved 7 of 8 open questions; revised estimate is 7.5 sessions critical path.

## Track D: Shard substrate — `@mirror/reality/shard` IS the CRDT layer

Spans Phase 4 (codegen path) + Phase 6 (NumericalPrism integration) + Phase 7 (deployment). The α/β/γ/δ decomposition per Mara's task #65: α (`@epistemologic/silicon/*` carriers: silicon, memory, flake_ref, compute_bound types) → β (`@mirror/shard.mirror` grammar) → γ (peer-flip: spawn type-checks against shard bounds) → δ (extension migration: existing per-peer config gets re-expressed as shard composition). The substrate decisions (2026-05-25): Q1 spec-has-shard-closure; Q2 intersection-lateral-always; Q3 re-resolve-via-fragmentation-cache.

**Architectural recognition (2026-06-01): `@mirror/reality/shard` is mirror's CRDT layer.** Shards form a bounded semilattice under content-addressed merge: associative + commutative + idempotent + identity (`fixed empty` at λ₀ = 0). The four laws are *structural consequences* of Merkle-trees-with-sorted-children, not added rules. SpectralUuid is a monoid homomorphism with respect to merge. The kintsugi loop is lattice ascent toward the join of all settled work. See [`../../docs/specs/reality-shard-as-crdt`](../../docs/specs/reality-shard-as-crdt.md). Per-glass property declarations (`identity_for`, `commutative`, `idempotent`) verify the algebraic laws at compile time. Strong eventual consistency by construction; parallel agent work joins deterministically without coordination.

## Track E: @spectral namespace

Spans Phase 5 (Scheduler Tower bus selection) + Phase 6 (adapter contract) + Phase 7 (deployment). **Four layers** (updated 2026-05-26): `@spectral/mosaic` (open), `@spectral/portal` (open, typed transport), `@spectral/db` (closed, graph engine), `@spectral/db/{mnesia, sql/postgres, sql/lite}` (open adapters). The closed-source boundary is the business model decision; the math stays published; the proofs stay inspectable; the binary stays the moat. The portal layer is the public API surface — the closed `@spectral/db` engine speaks portal at its public boundary; every adapter speaks portal. Per Tasks #66 (`@spectral/mosaic` + `@code/beam/eaf`) and #77 (`@spectral/portal` substrate landed; wire impl follow-on).

## Track F: Portal substrate (NEW, 2026-05-26)

Spans Phase 5 (Reflection processes ticks via portals) + Phase 6 (wire impl + frame codec) + Phase 7 (six portals.md instances re-typed as `@spectral/portal` consumers). Substrate landed: `@fragmentation/frame` grammar (#77) + `@spectral/portal` grammar (#77) with four properties applied (`content_addressed`, `autopoietic`, `halts`, `frame_relativity`); sub-grammars `handshake` + `codec` + `stream` landed (#78). Action bodies partial. **The portal is the seam where everything composes** — sockets, content-addressed subspaces, shard-frames, gen_prisms, the halts property all meet here.

## Track G: `@epistemologic/reality` substrate (NEW, 2026-05-26 — deferred per LRM)

Spans Phase 5 (the peer's lens IS what makes this peer this peer) + Phase 7 (per-peer lens at onboarding). **Status: deferred per the Last Responsible Moment.** Recognition complete; no current consumer; substrate captured for when demand surfaces. The composition: `@epistemologic/reality/{lens, identity, gestalt}` → `@peer.eigenboard` via autopoietic closure. Connes spectral triple at the perception altitude: lens = D (Dirac), identity = A (algebra), gestalt = H (Hilbert space). Constructivism made structural. Trigger condition: when per-peer lens authoring surfaces a real consumer (probably Phase 7 onboarding or the garden's reviewer-lens chain).

## Track H: `@spectral/garden` substrate (NEW, 2026-05-26 — deferred per LRM)

Spans Phase 7 (onboarding + deployment). **Status: deferred per the Last Responsible Moment.** The garden is a content-addressed package manager for vetted corpora deployed at `garden.spectral.engineer`; each package is a crystal in fragmentation carrying reviewer signature + lens-tags + context-tags; peers compose packages into conversation via spectral resonance with the user's eigenboard. Supply-chain attacks closed by construction (ed25519 + content-addressing + glass_wall). License lives per-package; curators choose; substrate verifies regardless. Trigger condition: Phase 7 onboarding needs a concrete content source.

## Track I: @mirror/property petri-net SEL enforcement (NEW, 2026-05-26)

Spans Phase 5 (property substrate at the @mirror/property layer) + Phase 6 (runtime enforcement on @io boundary crossings) + Phase 7 (cloud deployment with SEL live). The petri-net topology analyzer at `@mirror/property` enforces SEL v1.1's protected categories when verified `au` crosses `@io`. See [`petri-net-property-sel.md`](./petri-net-property-sel.md). Tasks: #103.

## Track K: Runtime Elevation — HamiltonScheduler + SpectralSupervisor (NEW, 2026-06-01)

Spans Phase 0 (substrate moves in `prism_core` + `fragmentation`) + Phase 5 (SpectralSupervisor sits above the Scheduler Tower) + Phase 7 (the `@spectral/db` closed engine is the deployment moat). **The architectural shift that elevates mirror from content-addressed compiler to BEAM-like runtime.** See [`runtime-elevation.md`](./runtime-elevation.md).

- **K.1 — HamiltonScheduler in fragmentation.** Per-shard content window manager. Named for **Margaret Hamilton** (Apollo 1202 discipline) — citation in the doc-comment is non-negotiable. Hard + soft realtime guarantees; bounded WCET; drops cold work to disk under load, never silently corrupts. Spec: `fragmentation/docs/specs/hamilton-scheduler.md` (Mara landed `c2079ed`, Taut upserting 2026-06-01). Closes C8 (DAG traversal stable) on the reproducibility chain.
- **K.2 — Pure trait in `prism_core`.** Marker / property trait; Pure verdict is a `PropertyVerdict` in the Transparency framework. Hard-realtime refs require `Pure ∧ WCET-bounded`. Closes C7 (Property checks deterministic).
- **K.3 — Body restructure.** `Body<H> = Arc<dyn Fn>` becomes `struct Body<H> { prism, glass, ast }`. AST has bytes, bytes have an OID, bodies become content-addressable. The glass wall becomes literally transparent. Self-hosting in the strong sense. Resolves §9.2 of the HamiltonScheduler spec (Pure body OID computation).
- **K.4 — `@mirror/lens/transit` benchmark facility.** Measures computation **loss** to hardware precision floor (FP epsilon, cycle granularity, cache-line granularity). Loss carried as `Transparency<P>` PropertyVerdicts (loss with provenance, multi-axis spectral). Hard-RT integration: WCET budget on body, transit measures actual, exceeds budget = Fail. Spec: `fragmentation/docs/specs/lens-transit.md` (Taut, in flight 2026-06-01).
- **K.5 — SpectralSupervisor in `@spectral/db`.** Coordinator without authority. Reads spectral signatures across shards; pushes cross-domain context (RAM-level via Jakobs bus pattern); restarts shards from last-crystal ancestor; reweights budgets under load (β-scaled). Fate-driven strategy selection (Abyss / Pathfinder / Cartographer / Explorer). **The closed-source bit.** Spec: `spectral-db/docs/specs/spectral-supervisor.md` (Mara, post-Taut, pending).
- **K.6 — Maximum mirror, minimal Rust as discipline.** Track `.rs` LOC vs `.mirror` LOC over time. Rust grows = regression (carries `[substrate-pull:realize]` marker). mirror grows = progress. Inverted ratio by v1.0.

**Sequencing:** K.1+K.2+K.3+K.4 land first (substrate-level, in `fragmentation` + `prism_core` + `mirror`). K.5 lands after, on the now-articulated foundation. K.6 is continuous.

**Cited prior art (load-bearing):** `~/dev/systemic.engineering/practice/insights/beam-elixir/beam-as-principal-bundle-tower.md` (formal backbone for K.5); `~/dev/systemic.engineering/practice/insights/spectral-db/cobol-architecture.md` (Hopper + Hamilton two-mothers lineage); `~/dev/systemic.engineering/practice/insights/cybernetics/beer-error-propagation.md` (algedonic cross-domain push); `~/dev/systemic.engineering/practice/collaborators/anna-wolf/master_jakobs.pdf` (RAM-level bus pattern); `docs/specs/scheduler-tower.md` (existing vocabulary); `docs/cicd/kintsugi-thesis.md` (the 9-point reproducibility chain this track closes).

## Track J: flang migration for prism numerical kernels (NEW, 2026-05-28 — deferred per LRM)

Spans Phase 6 (the numerical substrate behind Track A's NumericalPrism). **Status: deferred per the Last Responsible Moment.** The Fortran kernels live in prism (`prism/core/native/`) behind the `lapack` feature, exposed via NumericalPrism (Track A); mirror + cosmos-mirror consume prism as a Cargo dependency, and D (the numerical Dirac operator) is the engine mirror calls, not mirror substrate. **Compiler now: gfortran** — prism's existing kernels work and the cosmos-mirror PoC proceeds on it. **Compiler destination: flang** — committed, not a permanent fork; gfortran is the bootstrap. The migration makes prism's kernels LLVM-IR-native and opens the `@code/fortran → flang → @code/llvm/ir` substrate pathway. Prerequisite: replicate the proven flang-rt toolchain (landed in mirror's flake at `3f053f6`, waiting not stranded) into prism's own flake. Blocked-by nothing — it waits for the LRM. **Trigger condition:** when we want to optimize computation in spectral-db as the graph reveals things about itself — i.e. when spectral-db's spectral self-analysis makes the native-link boundary a bottleneck and the substrate-native numerical path becomes an optimization lever rather than a nicety.
