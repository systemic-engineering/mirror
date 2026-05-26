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

## Track D: Shard substrate

Spans Phase 4 (codegen path) + Phase 6 (NumericalPrism integration) + Phase 7 (deployment). The α/β/γ/δ decomposition per Mara's task #65: α (`@epistemologic/silicon/*` carriers: silicon, memory, flake_ref, compute_bound types) → β (`@mirror/shard.mirror` grammar) → γ (peer-flip: spawn type-checks against shard bounds) → δ (extension migration: existing per-peer config gets re-expressed as shard composition). The substrate decisions (2026-05-25): Q1 spec-has-shard-closure; Q2 intersection-lateral-always; Q3 re-resolve-via-fragmentation-cache.

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
