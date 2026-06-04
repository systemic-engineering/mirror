# mirror — Roadmap

*This directory captures milestone notes from the project's BEAM/Gleam/conversation era (March–May 2026). The canonical roadmap is now [`mirror/roadmap/`](../../roadmap/) (with `wip/`, `pending/`, `archive/`). The milestone files here (00–12) remain as historical context; the substrate moved past most of what they describe during the June 2026 substrate-pull arc.*

---

## Where we are (2026-06-04)

The substrate IS the operational form of Connes' (A, H, D) spectral triple:
- **A** — the five operations (`focus`, `project`, `split`, `shift`, `settle`)
- **H** — the [Void](../insights/void-dual-geometry.md) (graph quantum information manifold; λ₀ = 0 ground state)
- **D** — the kintsugi flow (Dirac operator; eⁿ⁺¹ ≤ eⁿ monotone descent; c-theorem on graph Laplacians)

The stack:

```
mirror          — the compiler; shards → graph → emit          (Rust + .mirror substrate)
prism           — the algebra (A); five operations as a trait    (Rust crate)
fragmentation   — the store; content-addressed substrate (no deps; Splinter IS the lockfile)
coincidence     — numerics; Fortran via LAPACK for eigenvalue work
```

## v0.1

**Kintsugi becomes the build system for the repo. The language owns the build process. cargo is the @io delegation that shrinks as mirror grows over phases.**

The v0.1.0 surface:

```yaml
- uses: systemic-engineering/mirror/actions/kintsugi@v0.1
  with:
    spec: ./mirror.spec
```

A `mirror.spec` declares the project manifold (sources, legacy floors, targets, settle_on conditions). `mirror kintsugi ./mirror.spec` runs [mosaic](../specs/mosaic.md) as a glass within `@mirror`. Mosaic settles each target to `au` at its altitude (binary at `@code/rust` via cargo; action at `@ci/github` via YAML; release at `@release` via github-release). The settled output projects optionally to `.shatter` on disk; the [fragmentation store](../specs/mirror-store.md) is canonical.

v0.1 is the kintsugi-CI release. v1.0 is the spectral.engineer cloud deployment (per [`../../roadmap/wip/v1-launch.md`](../../roadmap/wip/v1-launch.md)).

## Status

The canonical status table is at [`../../roadmap/README.md`](../../roadmap/README.md). Headline:

- Substrate-pull rename arc landed (`zoom → shift`, `refract → settle`) across docs + code + boot grammar in 4 repos
- Gap fold landed (`contradiction ≤ gap`; LFI / Bateson grounded; tension/tensor absorbed)
- Three new spec foundations landed: `transparency.md` (replaces MirrorLoss), `mosaic.md` (build system), `mirror-spec-schema.md` (the manifold)
- v0.1 scaffolding shipped: `actions/kintsugi/action.yml`, `.github/workflows/{kintsugi,release}.yml`, Justfile local-parity recipe
- T11.7 cut pending workflow validation; T11.8 post-release

## Canonical specs (current)

- **Substrate floor**: [`prism-floor-and-the-grammar-rename.md`](../specs/prism-floor-and-the-grammar-rename.md), [`bootstrap-retirement-plan.md`](../specs/bootstrap-retirement-plan.md), [`mirror-store.md`](../specs/mirror-store.md)
- **Glass wall**: [`transparency.md`](../specs/transparency.md), [`properties-on-glass.md`](../specs/properties-on-glass.md), [`gap-tension-tensor-substrate.md`](../specs/gap-tension-tensor-substrate.md)
- **Build system**: [`mosaic.md`](../specs/mosaic.md), [`mirror-spec-schema.md`](../specs/mirror-spec-schema.md)
- **v0.1 release**: [`kintsugi-ci-v0.1.md`](../specs/kintsugi-ci-v0.1.md), [`../../roadmap/wip/kintsugi-ci-release-v0.1.md`](../../roadmap/wip/kintsugi-ci-release-v0.1.md)
- **Shatter format**: [`../shatter-spec.md`](../shatter-spec.md)

## Historical milestone files (BEAM/Gleam/conversation era)

| # | Milestone | Status |
|---|-----------|--------|
| [00](00-root.md) | Root definition (pre-Connes-triple) | Historical |
| [01](01-architecture.md) | BEAM/Gleam compiler architecture | Historical |
| [02](02-compilation-chain.md) | Traced compilation chain | Historical (folded into shatter-spec.md) |
| [03](03-shipping.md) | Shipping via fragmentation | Partially active (v0.1 uses GitHub Actions, not Nix) |
| [04](04-fortran-bridge.md) | Fortran bridge for numerics | Active (LAPACKPrism + coincidence-core) |
| [05](05-kandddinsky.md) | KanDDDinsky — October 2026 | Planned (post-v0.1) |
| [06](06-model-checker.md) | Model checker properties | Active (sub-Turing verification via @epistemologic) |
| [07](07-projection.md) | Projection: properties as plans | Active (transparency<p> + properties-on-glass) |
| [08](08-oid-native-model.md) | OID-native model | Active (Splinter as content-addressed floor) |
| [09](09-licensing.md) | SEL licensing | Active (per Track I; type sel = io + au) |
| [10](10-inference-physics.md) | Inference physics | Active (\ as Dirac kernel; Fate tournament) |
| [11](11-ca-cogito.md) | CA cogito (constructor automaton) | Active (Reflection model; @cogito) |
| [12](12-coherence-benchmark.md) | Coherence Benchmark — post-release | Planned (post-v0.1) |

The "Historical" entries describe pre-substrate-pull architecture; the "Active" entries map to current canonical specs (linked above). The conversation/compiler-actor framing in 01–02 is superseded by `@mirror` as the compiler surface and the shards/ substrate floor.
