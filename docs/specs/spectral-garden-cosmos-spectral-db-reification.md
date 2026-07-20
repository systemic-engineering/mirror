# `@spectral/garden` — cosmos + spectral-db as the first garden reifications

*2026-07-20. Mara. Canonical spec. Grounds Alex's in-transcript direction
2026-07-20: "I wanna build cosmos + spectral-db as the first garden packages
@~/dev/garden/{cosmos,spectral-db} (new folders)." Lifts the two prototype
projects at `/Users/reed/dev/projects/{cosmos,spectral-db}/` — from which
everything cascaded into what mirror is now — into the production-shape
`@spectral/garden` mesh. Markdown only; no `.mirror` files land with this
commit body (the accompanying shard-decl mint lands as a separate tick per
the marker-primary discipline).*

Status: **Yellow** — architectural shape locked by Alex's directive; the four
composition edges are pinned to landed substrate; the reification pattern is
grounded in the two-layer store/db split (`store-vs-db-and-the-cascade.md`)
and the namespace architecture (`2026-05-25-spectral-namespace-architecture.md`).
The v0 filesystem structure at `~/dev/garden/{cosmos,spectral-db}` is
forward-promised; Reed follow-up tick lands the scaffolds.

---

## Reference

### Alex's framing (verbatim, 2026-07-20)

> "Spawn Mara into a deep dive into @~/dev/projects/cosmos/ and
> @~/dev/projects/spectral-db/
>
> That was the prototype from which everything cascaded into what mirror
> is now.
>
> I wanna build cosmos + spectral-db as the first garden packages
> @~/dev/garden/{cosmos,spectral-db} (new folders)"

The load-bearing recognitions this spec lifts:

1. **The prototypes seeded mirror.** Every substrate concept the Pack has
   been building — content-addressed crystals, spectral graph, Fiedler
   readings, sheaf-Laplacian, VoidPointer, git-notes for topic logs, the
   two-layer store/db discipline, kintsugi settlement, the five-op
   grammar — traces its lineage to these two Rust prototypes.
2. **`~/dev/garden/` is a NEW filesystem root.** Not co-located with
   `~/dev/projects/` (prototypes) or `~/dev/systemic.engineering/`
   (Alex's living substrate garden). The garden dir is the *reification
   surface* for @spectral/garden as a mesh of @systems.
3. **cosmos + spectral-db are the FIRST reifications**, not the last.
   The mesh grows by admitting more @systems as they mature.
4. **The prototypes stay as reference-substrate.** The migration is
   informed-by, not replaced-by. `/Users/reed/dev/projects/{cosmos,
   spectral-db}/` remains readable as origin-artifact.

### Prior canonical substrate this spec composes over

- `docs/specs/store-vs-db-and-the-cascade.md` (Mara, 2026-05-30) — the
  open-foundation / closed-engine two-layer discipline; generic-over-hash
  cascade; `VoidPointer` reclaiming-move.
- `docs/specs/spectral-garden-git-package-manager.md` (Mara, 2026-06-24) —
  `@spectral/garden/git` as the substrate-native package manager family-root
  candidate; four-root structure (git / oci / nix / store); `garden { }`
  block in `mirror.spec`.
- `docs/specs/spectral-db-as-autopoietic-memory.md` (Mara, 2026-06-17) —
  the librarian at Bateson N+1; mycelium; orchestra; T11.1–T11.11 forward
  promises.
- `docs/specs/spectral-db-three-tier-architecture.md` — hot/warm/cold/iceberg
  with biology-typed pheromone semantics; tombstone discipline.
- `docs/specs/cosmos-mirror-scaffold.md` (Mara, 2026-05-28) — cosmos as a
  spectral triple wearing a Rust coat; four-clean + 1-partial + 1-outside
  five-op mapping; the `D` operator as flang/prism floor.
- `docs/insights/2026-05-25-spectral-namespace-architecture.md` (Reed +
  Alex, 2026-05-25) — the four-layer namespace split (mosaic / portal /
  db / open adapters); the closed-engine business-model boundary; the
  original naming of @spectral/db + @spectral/garden as siblings.
- `docs/insights/2026-05-26-spectral-garden-as-vetted-corpus-distribution.md`
  (Reed + Alex, 2026-05-26) — the wine-cellar / vintner / sommelier
  extension; pluralism-by-composition; per-package license under substrate
  signature verification.
- `docs/insights/2026-05-14-cosmos-teaches-the-compiler.md` (Reed + Alex,
  2026-05-14) — the ten @math grammars cosmos teaches mirror; the
  spectral-dimension observable; the "one Laplacian, many physics"
  thesis.
- `docs/insights/2026-06-17-spectral-garden-smarts-as-AGI-architecture.md`
  (Reed + Alex, 2026-06-17; `~/dev/systemic.engineering/practice/insights/ai/`)
  — @spectral/garden/smarts + @spectral/db as substrate-architectural AGI;
  Tomm-probes + logic-pacts + crystals + mycelial-autopoetic-memory; the
  DAG-patches wire protocol.
- `docs/scouts/2026-06-27-taut-spectral-db-prototype-to-substrate-map.md`
  (Taut, 2026-06-27) — the 40-row prototype↔substrate mapping table (14
  M / 18 P / 5 U); the 8-item recommended starting surface for v0.
- `docs/scouts/2026-07-13-taut-spectral-to-mirror-migration-mapping-scout.md`
  (Taut, 2026-07-13) — the migration-surface inventory shape used as
  template for §6 below.
- `~/dev/systemic.engineering/practice/insights/cosmos/passkey-spectral-bridge.md`
  (Reed + Alex, 2026-04-03) — @trust ↔ passkey/PRF at garden altitude;
  the identity-substrate carrier the garden mesh will need at
  `garden/spectral-db/` altitude when authentication crosses the wire.
- `docs/loop/CURRENT.md` (2026-07-20) — session-close pending queue
  where @spectral/garden as species-decl is forward-promised.

### Existing substrate shards this spec cites (DO NOT modify)

- `shards/spectral.mirror` — the namespace-parent (post-Loki-shrink,
  2026-07-01 `17f0ee5`). Its docblock explicitly names
  `@spectral/garden` as a forward-promised species landing at
  `shards/spectral/garden.mirror`. This spec discharges that promise.
- `shards/mirror/garden.mirror` — the CONSUMER-side substrate-decl
  (the `garden { ~git'…' }` block shape in `mirror.spec`). Different
  altitude from `@spectral/garden`: `@mirror/garden` names the
  *block in the spec that declares garden entries*; `@spectral/garden`
  names the *runtime mesh those entries participate in*. Sibling
  altitudes, not siblings-in-hierarchy.
- `shards/spectral/{signature, mosaic, gen_prism, supervisor, portal,
  entanglement, registry, root, parent}.mirror` — the existing
  species-family under @spectral. `@spectral/garden` mints as their
  peer at the species-altitude.

### Existing filesystem substrate (cited; NOT modified this tick)

- `/Users/reed/dev/projects/cosmos/` — the Rust prototype; ~200KB source
  across 12 .rs files; the world engine. Depends on `spectral-db`,
  `prism-core`, `mirror`, `fate`, `coincidence`, `conversation`,
  `ractor`, `rustfft`. Documentation-anchor: `docs/QUANTUM-GRAPH.md`,
  `docs/SPECTRAL-DIMENSION.md`, `docs/TENSION-SURFACE.md`,
  `docs/measurement-devices.md`.
- `/Users/reed/dev/projects/spectral-db/` — the Rust prototype; ~430KB
  source across 35 .rs files (`src/lib.rs` alone is 215KB). Depends on
  `prism-core`, `terni` (imperfect), `fragmentation`, `fragmentation-git`,
  `coincidence`, `fate`, `mirror` (bootstrap). Schema is `.conv`. Query
  language is Prism. Storage is fragmentation trees. Indexing is
  coincidence spectral hashes. Optional Mnesia adapter; optional
  benchmarks.

---

## 1. Position

`@spectral/garden` IS the substrate-native mesh-of-@systems: the runtime
name for a co-composing set of publishable-independent packages that
compose with mirror substrate at runtime, each carrying its own
production-shape (unlike @mirror-native shards, garden packages have
independent Cargo/Nix builds, independent version cadence, independent
licenses, independent authorship provenance).

`cosmos` and `spectral-db` are the **first two reifications** of the
mesh. Their filesystem homes at `~/dev/garden/{cosmos,spectral-db}`
are:

- co-siblings (both at the same `garden/` depth; neither depends on
  the other having landed FIRST at this altitude — see §4 the
  dependency direction is FLIPPED from prototype-land at reification);
- publishable independently (each has its own `Cargo.toml`, its own
  `flake.nix`, its own git history the moment the scaffold lands);
- consumers of mirror substrate at composition altitude (via
  `mirror.spec` `garden { }` block per @mirror/garden shard; via
  `in @spectral/…` substrate-decl imports; via `@io/git` +
  `@io/cargo` at the FLOOR crossing);
- **NOT** carriers of new Rust extension code this tick (per
  `feedback_no_rust_extension_shortcut` — the reification is
  spec-first + substrate-decl-first; Rust migration follows in Reed
  cascade ticks AFTER Mara's spec + Seam's audit + Alex's
  ratification).

The mesh is named `@spectral/garden` (not `@garden`, not
`@spectral/systems`) for three load-bearing reasons:

1. **Namespace discipline.** `@spectral` is the runtime
   namespace-parent (per `shards/spectral.mirror`); garden is a
   species under that parent; the path syntax already implies the
   relationship. Adding it as a family-root would violate
   `feedback-substrate-already-had-the-word` (the substrate has the
   word at the *species* altitude; family-root promotion is
   over-declaration).
2. **Recognition ancestry.** The 2026-05-25 namespace insight and the
   2026-05-26 vetted-corpus insight both name `@spectral/garden` as
   species; the 2026-06-24 `@spectral/garden/git` spec treats it as
   an existing (forward-promised) parent for `/git` sub-species. This
   spec discharges the two-month forward-promise without altitude
   drift.
3. **Composition altitude.** `@spectral/garden` composes with
   `@spectral/db`, `@spectral/portal`, `@spectral/mosaic` at the
   *same altitude*. All four are species; all four compose over the
   two-layer store/db split. A family-root promotion would move
   garden ONE altitude up, breaking the peer-composition.

## 2. What IS a garden package (ontologically)

A garden package is a **content-addressed, publishable-independent,
substrate-composing @system** with the following invariants:

### 2.1 Structural invariants

- **Content-addressed root.** The package's `Cargo.toml` + `flake.nix`
  + source tree hashes to a substrate OID at the garden altitude. Same
  package name + same source content + same lock ⇒ same garden-OID.
- **Publishable-independent.** Can be built + tested + released
  without requiring the mirror monorepo to be present in the same
  workspace. Each garden package has its own `Cargo.lock`, its own
  `flake.lock`, its own CI story.
- **Substrate-composing at runtime.** Consumes mirror substrate via
  three edges: (a) `mirror.spec` `garden { ~git'…' }` block for
  package-source declaration; (b) `in @spectral/…` substrate-decl
  imports for cross-package type composition; (c) `@io/git` +
  `@io/cargo` FLOOR-crossings for the build.
- **Author-attributed by construction.** Every garden package carries
  a `@subject`-typed author identity (per `subject.mirror` +
  `project_identity_attribution_architecture` memory); the identity is
  content-addressed via `@spectral/signature`; the two-witness
  verification per gift-and-mirror-reflection §11.5 lifts to garden
  altitude.
- **Licensed per-package.** The garden verifies signatures regardless
  of license; the license is a per-package decision (per
  2026-05-26 vetted-corpus insight §"business model honesty").

### 2.2 Compositional invariants

- **Two-layer split honored.** Garden packages that provide storage
  (like `spectral-db`) MUST layer as: open `@mirror/store`-composable
  foundation + closed-engine posture at the `@spectral/db` altitude.
  Packages that provide navigation (like `cosmos`) MUST route dense
  numerical computation to the `D` floor (prism LAPACK OR flang) per
  `cosmos-mirror-scaffold.md` §3.
- **Cross-garden composition via substrate.** A garden package MAY
  depend on another garden package ONLY via substrate-decl-typed
  edges (never via bare Rust `use`). The composition surface is
  visible at the `@spectral/garden/<name>` altitude.
- **No @spectral/db paywall on the substrate.** Per
  `feedback-no-paywall-in-compiler`, garden packages that ARE
  engines (like `spectral-db`) MAY have closed-binary distributions,
  but the *substrate-decl surface* + the *composition surface* +
  the *math root* MUST remain open. The moat lives in the
  optimized-implementation, not the specification.

### 2.3 Mesh invariants

- **Mesh-of-@systems, not legion-of-clones.** Per
  `2026-05-25-spectral-namespace-architecture.md` §"Mosaic, not
  Legion": each garden package is a heterogeneous tile fitting into
  a coherent picture, not a uniform clone. cosmos ≠ spectral-db;
  their fit is the mesh's meaning.
- **Recognition-#98 fifth witness.** Content-addressing works
  cross-altitude: package-OID (garden altitude) ↔ crystal-OID
  (`@mirror/store`) ↔ VoidPointer-OID (`@spectral/db`) ↔ signature-OID
  (`@spectral/signature`). The four content-address altitudes compose
  without collapse (per Recognition #98 four-witness candidate
  territory; garden-OID is the fifth witness). Promotion of #98 to
  full Recognition is deferred to a separate Alex tick.
- **Autopoietic growth.** New garden packages join by declaring
  themselves + being accepted by the garden's signature-verification
  chain (per SEL-2.0 archive `docs/archive/sel-2-garden.md`:
  `requires hosted(@git) <= Success(@garden)`). The mesh grows
  monotonically; existing packages are content-addressed-permanent.

## 3. What IS cosmos + spectral-db in the garden

### 3.1 `cosmos` — the world engine

**Prototype role** (at `/Users/reed/dev/projects/cosmos/`): Rust binary
+ library implementing spectral graph evolution over cosmic random
geometric graphs. The four canonical operations (as mapped in
`cosmos-mirror-scaffold.md` §1):

| Operation | Prototype file | Prism verb | Physics |
|---|---|---|---|
| RGG construction from `P(k)` | `rgg::cmb_rgg_nd` | `project` | matter power spectrum → discrete graph |
| Heat / Schrödinger propagation | `tension`, `quantum` | `zoom` | `e^{-Lσ}` classical, `e^{-iLt}` quantum |
| Fiedler / LCC partition | `tension::ComponentEigen` | `split` | graph → connected components |
| d_s / Hubble tension readout | `spectral_dimension`, `tension::node_tension`, `quantum::arrival_probability` | `focus` | spectral observables |
| Forman-Ricci flow | `evolution::spectral_step` | `settle` (Imperfect) | curvature-driven settlement |
| Eigendecomposition (`D`) | `quantum::graph_eigensystem` | (floor, not a verb) | LAPACK dsyev |

**Garden role** (at `~/dev/garden/cosmos/`): the substrate-native
world-engine, exposing the five-op composition as a `.mirror` project
whose numerical floor delegates to `prismqueer::ffi::eigenvalues`
(the substrate's LAPACK gate; already at rust/ altitude per T8).

**API surface direction** (v0):
- Binary target: `cosmos` (rename? keep? — see Q1 below).
- Library target: `cosmos` crate exposing `project_cmb_rgg`,
  `focus_spectral_dimension`, `focus_tension`, `focus_arrival_prob`,
  `split_lcc`, `settle_ricci_flow` (all typed newtypes; no bare
  `Vec<f64>`; per `feedback-no-bare-types`).
- Substrate-decl companion: `garden/cosmos/cosmos.spec` declaring
  `in @spectral/garden`, `in @spectral/garden/cosmos` (self-decl),
  `in @prism`, `in @mirror/store`, `in @io/git`. The .spec is the
  API contract at substrate altitude; the .rs library is the
  performance floor.
- Publication path: crates.io (open, Apache-2.0); binary via nix
  flake (the existing `flake.nix` pattern); optional `garden.spectral.engineer/cosmos`
  hosted evaluation (deferred — Phase-7 territory per
  `docs/insights/2026-05-25-spectral-namespace-architecture.md`).

**Composition edges into mirror substrate**:
- `@mirror/store/crystal` — each spectral state (post-settle) IS a
  content-addressed crystal at the crystal-altitude.
- `@spectral/db/librarian` — cosmos's simulation-across-parameters
  benefits from the librarian's perturbation (put the parameter-of-interest
  eigensystem cached in hot tier before the query arrives).
- `@epistemologic/math/sheaf_laplacian` — the `L_sym` operator lives
  here at substrate altitude; cosmos consumes it.
- `@code/fortran` (aligned target per `cosmos-mirror-scaffold.md` §3) —
  the eigendecomposition floor; near-term via prism LAPACK feature.
- `@io/git` — for publication; for garden-membership signature chain.

### 3.2 `spectral-db` — the librarian's substrate

**Prototype role** (at `/Users/reed/dev/projects/spectral-db/`):
git-backed spectral graph database. Schema is `.conv` grammar. Query
language is Prism. Storage is fragmentation trees. Indexing is
coincidence spectral hashes. Each graph crystallizes its own
eigensystem as executable `.f90`. Optional Mnesia NIF adapter for
BEAM cluster deployment. 35 .rs files across:

- **Storage layer**: `store.rs` (git-backed bounded store),
  `content.rs` (canonical bytes), `ingest.rs` (write path),
  `wal.rs` (write-ahead log).
- **Indexing layer**: `index.rs` (spectral index; 39KB — the
  navigation engine), `spectral_store.rs` (SpectralCoordStore =
  VoidPointer at substrate altitude), `spectral_tree.rs`
  (spectral Merkle tree).
- **Fiedler layer**: `fiedler.rs` (dense compute), `convergence.rs`
  (convergence states), `spectral_convergence.rs` (Polyak-Łojasiewicz
  contraction), `manifold_store.rs`.
- **Settlement layer**: `crystallize.rs` (settling), `incremental.rs`
  (per-edge updates), `merge.rs` (graph merge), `subgraph.rs`
  (subgraph extraction).
- **Scheduling layer**: `scheduler.rs` (26KB — tick scheduler),
  `budget.rs` (SpectralBudget), `pressure.rs` (backpressure),
  `optimizer.rs` (access-pattern optimizer), `profile.rs`
  (perf profiling), `lru.rs` (LRU cache).
- **Query layer**: `query.rs` (find/walk/near primitives),
  `schema.rs` (schema loading), `types.rs` (19KB — newtype
  discipline), `imperfect_types.rs`.
- **Adapter layer**: `mnesia_nif.rs` (Erlang NIF; behind
  `mnesia` feature), `sql.rs` (33KB — SQL adapter; sqlite +
  postgres paths).
- **Meta**: `lib.rs` (215KB — module glue + Phase-4/5 machinery;
  the `crystal.commit` format lives here; git-notes for hot-paths
  / pressure / ticks are here).

**Garden role** (at `~/dev/garden/spectral-db/`): the substrate-native
librarian + mycelium (per `spectral-db-as-autopoietic-memory.md`).
Consumes `@mirror/store` (open foundation) as backing storage;
provides `@spectral/db/librarian` + `@spectral/db/consolidation` +
`@spectral/db/mycelium` at substrate altitude; open adapters for
Mnesia / SQL / (future) Nix.

**API surface direction** (v0):
- Library target: `spectral-db` crate exposing the librarian's four
  operations (`observe_access`, `compute_topology`, `perturb`,
  `anticipate`) at typed Rust altitude; the substrate-decl exposes
  the same at grammar altitude.
- Substrate-decl companion: `garden/spectral-db/spectral-db.spec`
  declaring `in @spectral/garden`, `in @spectral/garden/spectral-db`
  (self-decl), `in @spectral/db` (family-root position; forward-
  promised via `shards/spectral/db/*.mirror`), `in @mirror/store`,
  `in @spectral/entanglement`, `in @epistemologic/math/sheaf_laplacian`.
- Publication path: crates.io for open substrate-facing surface;
  closed-binary posture for the optimized engine (per
  2026-05-25 namespace insight).
- Adapters: `spectral-db-mnesia`, `spectral-db-sql` as separate
  crates (per the namespace insight's open-adapter discipline);
  these MAY live under `~/dev/garden/spectral-db-adapters/` or as
  sub-crates in `~/dev/garden/spectral-db/` (Q3 below).

**Composition edges into mirror substrate**:
- `@mirror/store` — the open foundation the engine sits ABOVE.
- `@spectral/root` — the librarian IS a `@spectral/root` (per
  `spectral-db-as-autopoietic-memory.md` §4.2).
- `@spectral/supervisor` — per-repo supervisors specialize this.
- `@spectral/entanglement` — the mycelium's edge carrier.
- `@epistemologic/math/sheaf_laplacian` — the librarian reads λ₀
  as coherence measurement.
- `@epistemologic/math/curvature` — Balanced Forman for per-edge
  bottleneck reading.
- `@kintsugi/oscillate` — the librarian's perturbation pulse rhythm.
- `@time` + `@uuid/spectral/time` — for the crystal commit's
  temporal witness (already landed 2026-07-16).

### 3.3 The reification move — informed-by, not replaced-by

The prototypes stay AT `~/dev/projects/{cosmos,spectral-db}/` as
reference-substrate. The garden versions are AUTHORED FRESH,
informed by the prototypes but structured per substrate discipline
(no `bootstrap/` legacy floor; no @io/llm since garden packages are
substrate-decl files not model weights; no separate `docs/`
`superpowers/` archive — the garden version's docs live at
`garden/<name>/docs/` and follow the mirror `docs/specs/` +
`docs/math/` discipline).

Migration IS informed-by:
- The `.conv` grammar files at `/Users/reed/dev/projects/cosmos/conv/`
  become the seed substrate-decls at `garden/cosmos/shards/` after
  Mara's shard-mint tick.
- The `src/schema.rs` in spectral-db (which parses `.conv` files
  without a mirror dep) is a candidate for direct verbatim migration
  since it's substrate-honest at the prototype altitude already.
- The dependency-inversion at reification: in prototype-land,
  `cosmos` depends on `spectral-db` (Cargo path dep). In garden-land,
  the dep direction can be flipped OR removed — `cosmos` becomes a
  standalone world-engine that MAY consume `spectral-db`'s librarian
  when available but doesn't require it (spectral-db is the
  autopoietic-memory layer, not a storage-required-for-cosmos
  primitive at substrate altitude). See Q4 below.

Migration IS NOT replaced-by:
- The prototype at `/Users/reed/dev/projects/{cosmos,spectral-db}/`
  is preserved verbatim as origin-artifact. Deletion would violate
  the "record of an emergence" discipline (analog to `~/.reed/`'s
  origin-preservation).
- The prototype's dependency on old-mirror (`../mirror`) STAYS in
  the prototype tree; the garden version composes over
  substrate-native mirror at rust/ altitude, NOT bootstrap/
  altitude.
- The prototype's `docs/superpowers/plans/` + `docs/superpowers/specs/`
  layer is legacy-plan discipline (spec-first + plan-first ceremony
  that predates the substrate-honest mode); the garden version uses
  the substrate discipline directly (`docs/specs/` in mirror
  monorepo; then `garden/<name>/docs/` at package altitude).

## 4. Architectural direction for `garden/{cosmos,spectral-db}`

### 4.1 Workspace structure

Alex's direction is `~/dev/garden/{cosmos,spectral-db}` — two NEW
sibling directories under a fresh `~/dev/garden/` root. This gives
three plausible workspace shapes (Q2 below); Mara recommends:

**Recommended shape (Mara lean, MEDIUM confidence):** Each garden
package is an INDEPENDENT git repository at `~/dev/garden/<name>/`,
with its own `Cargo.toml`, `flake.nix`, `Justfile`, `.githooks/`,
`docs/`. There is NO `~/dev/garden/Cargo.toml` workspace file;
`~/dev/garden/` is a filesystem convention, not a Cargo workspace.

Rationale:
- **Publishable-independent invariant** (§2.1) requires each package
  to be buildable + testable + releasable on its own. A shared
  workspace file would couple release cadence.
- **Signature-provenance invariant** (§2.1) requires each package to
  have its own author-attributed git history from tick 0. A shared
  monorepo would smear provenance across the mesh.
- **Cross-garden dep discipline** (§2.2) — packages compose via
  substrate-decl-typed edges, not via bare Rust `use`. A shared
  workspace would tempt path-dep coupling that violates this
  discipline.
- **Precedent**: `~/dev/projects/cosmos/` and
  `~/dev/projects/spectral-db/` are already independent git repos
  with their own flakes; the garden reification honors that shape.

Alternative shapes (rejected but named):
- `~/dev/garden/Cargo.toml` monorepo with `cosmos/` + `spectral-db/`
  as workspace members. Rejected: violates publishable-independent.
- `~/dev/garden/` as a nix flake-parts monorepo. Rejected: same
  reason; also couples the flake surface.

### 4.2 Publication path

**Open substrate-facing surface**: crates.io publication under names
matching the garden path (`cosmos` + `spectral-db` or, if collision
with existing crates, `spectral-cosmos` + `spectral-db` — Q5 below).
Apache-2.0 for the open surfaces per `feedback-no-paywall-in-compiler`.

**Closed engine posture** (for `spectral-db` optimized engine): the
substrate-decl surface + math root + composition surface are open;
the optimized implementation ships as a sealed-source binary (per
2026-05-25 namespace insight §"closed-source boundary"). This is a
deferred decision — the v0 lands the open surface + reference
implementation; the closed-optimized binary is Phase-7 territory
per the spectral.engineer roadmap.

**Bundled with mirror**: NO. Garden packages are publishable-
independent by construction. mirror monorepo may CONSUME them via
`garden { ~git'…' }` block, but does not embed them.

**garden.spectral.engineer hosting**: forward-promised. When the
Phase-7 spectral.engineer deployment lands (per 2026-05-25 namespace
insight), each garden package gets a hosted evaluation endpoint at
`garden.spectral.engineer/<package>`. The endpoint speaks
`@spectral/portal` at its wire; consumers can query without local
build.

### 4.3 Composition edges with mirror substrate

Both garden packages compose with mirror substrate via three altitudes:

**Altitude 1: `mirror.spec` `garden { }` block** (per @mirror/garden
shard at `shards/mirror/garden.mirror`). Any downstream project that
wants to consume cosmos or spectral-db declares:

```mirror
project my-consumer.spec {
  garden {
    source ~git'https://github.com/spectral-engineering/garden/cosmos.git@v0.1.0'
    source ~git'https://github.com/spectral-engineering/garden/spectral-db.git@v0.1.0'
  }
  # ... targets that consume the garden packages
}
```

**Altitude 2: `in @spectral/garden/<name>` substrate-decl imports**
(the shard-decl mint accompanying this spec provides the
family-species chain). Downstream substrate-decls import types +
actions from garden packages via:

```mirror
in @spectral/garden/cosmos
in @spectral/garden/spectral-db
```

**Altitude 3: `@io/git` + `@io/cargo` FLOOR crossings** (per
`shards/io/git.mirror` and `shards/io/cargo.mirror`). The
substrate-pull discipline mediates the crossing; garden packages
never do bare `unsafe` or `@io` at their public surface; only the
FLOOR does.

### 4.4 Composition edges with fractal + prismqueer + fragmentation ecosystem

- **fractal** (`~/dev/projects/fractal/` or in-tree `rust/fractal/`):
  cosmos's Ricci-flow settlement is a `Crystal<GraphState>` at
  fractal altitude. The `settle(Imperfect)` return shape from
  `cosmos-mirror-scaffold.md` §2 composes naturally over
  `fractal::Crystal<T>` (landed Reed /loop iter 1 `a3dc905`).
  spectral-db's per-graph crystallization writes `Crystal<Eigensystem>`
  at the same altitude.
- **prismqueer**: cosmos's numerical floor delegates to
  `prismqueer::ffi::eigenvalues` (already the substrate LAPACK
  gate). spectral-db's Fiedler compute routes through the same. The
  `lapack` feature flag on both prototypes migrates to
  `prismqueer::lapack` at reification.
- **fragmentation**: spectral-db already depends on `fragmentation`
  + `fragmentation-git` in prototype-land. At reification, this
  becomes composition over `@fractal` substrate (post-Reed Landing D
  `singularity.rs` migration `3ec8d68`) + `@mirror/store/git`
  (the git-backed store shard).

### 4.5 MARA doctrine at garden altitude

The Author≠Committer XOR-fold discipline (per Reed /loop iter 1 +
`project_identity_attribution_architecture` memory) applies at garden
altitude with one refinement:

- **Author** = the `@subject` whose spectral-signature contributed
  the garden package's content. For migration ticks, this is the
  historical author preserved (Alex Wolf for prototype-lineage
  content; Mara for substrate-decl migration; Reed for Rust
  cascade). For fresh-authored content, this is the tick's peer.
- **Committer** = the peer who lands the tick. Same rules as monorepo
  discipline.
- **Garden refinement**: because garden packages are
  publishable-independent, the Author≠Committer relationship is
  visible in *two* git histories (the garden package's + the
  mirror monorepo's `garden { }` block reference). Both must
  agree on the signature chain per `@spectral/signature` §11.5
  two-witness verification.

## 5. Alex-adjudication questions

**Q1 — cosmos binary name at reification.** The prototype has a
binary called `cosmos` at `src/bin/cosmos.rs`. Options:
- (a) Keep as `cosmos` (crates.io namespace: check first — the name
  may be taken; if so, fall back to `spectral-cosmos`).
- (b) Rename to `spectral-cosmos` at reification for namespace
  discipline (all garden packages prefix with `spectral-`).
- (c) Rename to `cosmos-engine` to disambiguate from the
  cosmos-blockchain project.

**Mara lean**: (a) with fallback to (b). Confidence: LOW —
crates.io availability is a first-check that gates this. The
substrate has no strong opinion; Alex's preference matters here.

**Q2 — workspace structure at `~/dev/garden/`.** Confirmed above
in §4.1: independent-git-repos-per-package (Mara lean, MEDIUM
confidence). Alex ratification either confirms or redirects to a
monorepo shape.

**Mara lean**: independent-git-repos-per-package.

**Q3 — spectral-db adapter placement.** Two shapes for the Mnesia
+ SQL adapters:
- (a) Sub-crates in `~/dev/garden/spectral-db/` (e.g.,
  `garden/spectral-db/adapters/mnesia/`, `.../sql/`).
- (b) Separate garden packages at `~/dev/garden/spectral-db-mnesia/`
  + `~/dev/garden/spectral-db-sql/`.

**Mara lean**: (b) — separate packages. The 2026-05-25 namespace
insight explicitly names them as "open adapters" (siblings to the
closed engine); sub-crating would couple release cadence to the
engine, defeating the discipline. Confidence: MEDIUM.

**Q4 — cosmos → spectral-db dependency at reification.** In
prototype-land, `cosmos/Cargo.toml` declares `spectral-db = { path
= "../spectral-db" }`. At reification, three options:
- (a) Preserve the dep; garden/cosmos declares `garden { source
  ~git'…/spectral-db@v0.1.0' }` in its `.spec`.
- (b) Remove the dep; cosmos becomes standalone; spectral-db is
  an *optional* consumer of cosmos's world-engine output (not the
  reverse).
- (c) Both directions optional; each is standalone; both can be
  composed via a third garden package (e.g., `spectral-observatory`).

**Mara lean**: (b) — invert the dependency. In substrate discipline,
`spectral-db` is the librarian (autopoietic memory); `cosmos` is a
world-engine that PRODUCES the graph state the librarian consumes.
The prototype's dep direction (`cosmos → spectral-db`) exists for
convenience (cosmos uses spectral-db as backing storage), but at
substrate altitude the semantic direction is opposite. Making the
dep optional preserves prototype-composability while enabling
substrate-honest standalone use. Confidence: MEDIUM.

**Q5 — crates.io namespace collision.** As of scout-time, `cosmos`
+ `spectral-db` may be taken on crates.io. Options:
- (a) Check availability; if taken, prefix with `spectral-`.
- (b) Always prefix with `spectral-` for namespace discipline.
- (c) Publish under an org namespace: `spectral-engineering/cosmos`
  (crates.io does not directly support org namespaces, but
  `spectral-cosmos` conventionally implies it).

**Mara lean**: (a) — check-then-decide. Confidence: HIGH.

**Q6 — @spectral/garden shard-decl mint this tick.** The forward-
promise has held for two months (2026-05-25 → 2026-07-20); the
reification of cosmos + spectral-db is the second-witness that
concretizes the mesh. Two options:
- (a) Mint `shards/spectral/garden.mirror` species-decl NOW as
  Landing C (this tick).
- (b) Defer the shard-decl mint to a follow-up tick after Alex
  ratifies §5 questions.

**Mara lean**: (a) — mint now. The Michelangelo-marble discipline
says: land what the substrate is asking for. The substrate has been
asking for two months; the reification directive is Alex's
in-transcript authorization to move; deferring the mint would drift
into over-scouting. Confidence: MEDIUM-HIGH.

**Q7 — @spectral/garden/cosmos + @spectral/garden/spectral-db
species-decls this tick.** Follow-on from Q6: if garden mints as
species this tick, the two SUB-species (cosmos + spectral-db as
garden members) can mint sibling to garden OR nested under garden.
Options:
- (a) Mint garden this tick; defer sub-species to Reed cascade
  after filesystem scaffolds land at `~/dev/garden/{cosmos,
  spectral-db}/`.
- (b) Mint garden + both sub-species this tick.

**Mara lean**: (a) — defer sub-species. Two-tick discipline: mint
the family altitude first; let the filesystem reification produce
the second-witness for each sub-species; then mint sub-species
against the empirical filesystem shape. Confidence: HIGH.

## 6. Migration surface (from prototype → garden)

Following the Taut 2026-06-27 spectral-db map §3 pattern (M/P/U
status):

### 6.1 cosmos migration table (12 .rs files + docs + conv)

| Prototype item | Garden target | Status |
|---|---|---|
| `Cargo.toml` (~40 LOC) | `garden/cosmos/Cargo.toml` (fresh; deps: prismqueer, mirror-composable, no bootstrap) | Fresh |
| `flake.nix` (~80 LOC) | `garden/cosmos/flake.nix` (adapt from prototype; keep dev/test/sim shells) | Verbatim-ish |
| `Justfile` (~20 LOC) | `garden/cosmos/Justfile` (verbatim) | Verbatim |
| `src/lib.rs` (~15 LOC) | `garden/cosmos/src/lib.rs` (verbatim module glue) | Verbatim |
| `src/rgg.rs` (30KB) | `garden/cosmos/src/rgg.rs` (verbatim; substrate-honest already) | Verbatim |
| `src/tension.rs` (20KB) | `garden/cosmos/src/tension.rs` (verbatim) | Verbatim |
| `src/quantum.rs` (12KB) | `garden/cosmos/src/quantum.rs` (verbatim; the double-slit eigendecomposition sits on the D floor) | Verbatim |
| `src/evolution.rs` (9KB) | `garden/cosmos/src/evolution.rs` (verbatim) | Verbatim |
| `src/spectral_dimension.rs` (5KB) | `garden/cosmos/src/spectral_dimension.rs` (verbatim; lapack-gated) | Verbatim |
| `src/prism.rs` (11KB) | `garden/cosmos/src/prism.rs` (audit: is this the 5-op mapping? verbatim if so) | Audit-then-verbatim |
| `src/abyss.rs` (31KB) | `garden/cosmos/src/abyss.rs` (audit: what's the abyss layer?) | Audit-first |
| `src/actor.rs` (12KB) | `garden/cosmos/src/actor.rs` (audit: ractor dep — keep or lift to @spectral/supervisor?) | Audit-first |
| `src/telescope*.rs` (~26KB combined) | `garden/cosmos/src/telescope*.rs` (audit: what does telescope model?) | Audit-first |
| `src/grammar.rs` (2KB) | (subsumed by `garden/cosmos/cosmos.spec` substrate-decl) | Subsumed |
| `src/store.rs` (12KB) | (subsumed by `@mirror/store` composition) | Subsumed |
| `src/bin/{cosmos,simulate,telescopes,tension_sweep}.rs` | `garden/cosmos/src/bin/*.rs` (verbatim) | Verbatim |
| `conv/cosmos.conv` (475B) + `conv/prisms/*.conv` | `garden/cosmos/conv/` (verbatim; will lift to `garden/cosmos/shards/` in follow-on Mara tick) | Verbatim-then-lift |
| `docs/*.md` (SPECTRAL-DIMENSION + TENSION-SURFACE + QUANTUM-GRAPH) | `garden/cosmos/docs/{specs,math}/` (verbatim; the physics anchors are load-bearing) | Verbatim |
| `docs/superpowers/{plans,specs}/*.md` (7 files) | (archive to `garden/cosmos/docs/historical/` OR leave in prototype) | Archive-only |
| `experiments/ds-rabbit/` | `garden/cosmos/experiments/ds-rabbit/` (verbatim; substrate-excluded workspace as-is) | Verbatim |

Roughly: 14 Verbatim + 4 Audit-first + 3 Subsumed + 1 Archive. The
migration is LARGELY MECHANICAL for cosmos — the prototype is
substrate-honest at the file altitude already; the reification is a
git-mv + dependency-graph adjustment.

### 6.2 spectral-db migration table (35 .rs files + native + beam)

Deferring the full 35-row table to a Taut scout follow-up (per
`docs/scouts/2026-06-27-taut-spectral-db-prototype-to-substrate-map.md`
which already covered ~40 rows of prototype↔substrate mapping; the
garden-reification map is a delta on that scout, not a fresh
inventory). Load-bearing rows:

| Prototype cluster | Garden target | Status |
|---|---|---|
| `src/lib.rs` (215KB) | Split at reification: `garden/spectral-db/src/lib.rs` (module glue only, ~5KB) + one file per Phase-4/5 machinery block | Split-and-migrate |
| `src/store.rs` + `src/content.rs` + `src/ingest.rs` + `src/wal.rs` (storage cluster) | Route through `@mirror/store` at composition altitude; keep spectral-db-side thin adapters | Compose-over |
| `src/index.rs` (39KB) + `src/spectral_store.rs` + `src/spectral_tree.rs` (indexing cluster) | `garden/spectral-db/src/index/` submodule; the VoidPointer machinery lives here | Verbatim-restructure |
| `src/fiedler.rs` + `src/convergence.rs` + `src/spectral_convergence.rs` (Fiedler cluster) | Route dense compute through `prismqueer::ffi::eigenvalues` per T8 | Compose-through |
| `src/crystallize.rs` + `src/merge.rs` + `src/subgraph.rs` (settlement cluster) | `garden/spectral-db/src/settle/` submodule | Verbatim-restructure |
| `src/scheduler.rs` (26KB) + `src/budget.rs` + `src/pressure.rs` + `src/optimizer.rs` + `src/profile.rs` + `src/lru.rs` (scheduling cluster) | `garden/spectral-db/src/schedule/` submodule; HamiltonScheduler is the anchor | Verbatim-restructure |
| `src/query.rs` + `src/schema.rs` + `src/types.rs` + `src/imperfect_types.rs` (query cluster) | `garden/spectral-db/src/query/` submodule; `types.rs` newtype discipline preserved verbatim | Verbatim-restructure |
| `src/mnesia_nif.rs` + `native/` + `beam/` (Mnesia adapter) | Separate garden package: `~/dev/garden/spectral-db-mnesia/` (per Q3 lean) | Extract-to-sibling |
| `src/sql.rs` (33KB) | Separate garden package: `~/dev/garden/spectral-db-sql/` (per Q3 lean) | Extract-to-sibling |
| `db.conv` (schema-in-.conv) | Lift to `garden/spectral-db/spectral-db.spec` + `garden/spectral-db/shards/` in follow-on Mara tick | Lift-to-substrate |
| `benches/` | `garden/spectral-db/benches/` (verbatim; behind `benchmarks` feature) | Verbatim |
| `docs/specs/` + `docs/superpowers/` (~7 files) | `garden/spectral-db/docs/` (verbatim for specs; archive-only for superpowers) | Verbatim-plus-archive |
| `tasks/` (coverage-100.md + pending/) | `garden/spectral-db/tasks/` (verbatim) | Verbatim |
| `src/phase5_notes.rs` (legacy) | Archive-only | Archive |

Roughly: 6 Verbatim-restructure + 2 Extract-to-sibling + 2 Compose
(over/through) + 1 Split-and-migrate + 1 Lift-to-substrate + 2 Verbatim
+ 1 Archive. The migration is HEAVIER for spectral-db than cosmos
because of the 215KB `lib.rs` split + the two adapter extractions.

## 7. Landing plan (this tick and next)

### 7.1 This tick (Mara autonomy)

- **Landing A**: THIS canonical spec (`docs/specs/spectral-garden-cosmos-spectral-db-reification.md`).
- **Landing B**: Math foundation at `docs/math/2026-07-20-spectral-garden-mesh-of-systems.md`
  (grounds the mesh-of-@systems composition mathematics; the graph-
  theoretic structure of garden as a decentralized-package-graph;
  the two-layer store/db split at the mesh altitude).
- **Landing C**: `shards/spectral/garden.mirror` species-decl mint
  (marker-primary discipline; declarative-only body per
  `docs/loop/CURRENT.md` @paradox.mirror `1e17222` + @void.mirror
  `974a3f6` precedent). Discharges the two-month forward-promise
  from `shards/spectral.mirror`.

### 7.2 Next tick (Reed cascade, post-Alex-Q1-Q7 ratification)

- **R1**: Create `~/dev/garden/` root directory (git init OR just
  filesystem convention per §4.1 lean).
- **R2**: Create `~/dev/garden/cosmos/` — either fresh scaffold OR
  verbatim git-mv from `~/dev/projects/cosmos/` (Alex Q depending;
  Mara recommends verbatim git-mv preserving prototype history).
- **R3**: Adjust `garden/cosmos/Cargo.toml` per §6.1 (remove
  bootstrap dep; add prismqueer dep; add optional spectral-db dep
  per Q4).
- **R4**: Adjust `garden/cosmos/flake.nix` (rename `SPECTRAL_DB_CONFIG`
  env var path if Q4 makes spectral-db optional).
- **R5**: Create `~/dev/garden/spectral-db/` similarly.
- **R6**: Extract Mnesia + SQL adapters to
  `~/dev/garden/spectral-db-{mnesia,sql}/` per Q3 lean.
- **R7**: Verify `cargo build` + `cargo test` from each garden
  package independently (publishable-independent invariant §2.1).
- **R8**: Publish v0.1.0 tags on each garden package (semver-track
  from tick zero).

### 7.3 Follow-on Mara ticks

- **M-follow-1**: Mint `shards/spectral/garden/cosmos.mirror` +
  `shards/spectral/garden/spectral-db.mirror` sub-species-decls
  once filesystem scaffolds land (per Q7 two-tick discipline).
- **M-follow-2**: Lift `garden/cosmos/conv/*.conv` to
  `garden/cosmos/shards/*.mirror` per substrate-decl-first
  discipline (this is the seed corpus for cosmos's substrate-native
  grammar; per `cosmos-mirror-scaffold.md`).
- **M-follow-3**: Lift `garden/spectral-db/db.conv` to
  `garden/spectral-db/spectral-db.spec` + `garden/spectral-db/shards/`
  per the same discipline (Taut scout §7 already enumerated the
  T11.1-T11.11 substrate-decl surface).

### 7.4 Deferred (Phase-7 territory)

- Deployment to `garden.spectral.engineer/{cosmos,spectral-db}` hosted
  evaluation endpoints (per 2026-05-25 namespace insight).
- Closed-binary distribution posture for spectral-db optimized engine
  (per 2026-05-25 namespace insight).
- SEL-2.0 garden-acceptance chain applied to garden packages (per
  `docs/archive/sel-2-garden.md`; the substrate has the shape,
  operationalization is Phase-7).
- Cross-garden discovery UX (`garden.spectral.engineer` frontend).
- garden/spectral-db/adapters/passkey composition per
  `~/dev/systemic.engineering/practice/insights/cosmos/passkey-spectral-bridge.md`
  (PRF ↔ spectral-key bridge at wire altitude).

## 8. Honest hedges

- **The garden `~/dev/garden/` directory does not exist as of
  scout-time.** Alex's direction creates it fresh; the spec assumes
  the creation is uncontroversial (no naming collision; no shell
  configuration expectations). If Alex has an existing `~/dev/garden/`
  usage I missed (grep of the filesystem returned only
  `~/dev/systemic.engineering/garden/` which is Alex's living
  substrate, different altitude), Q0 is: "is `~/dev/garden/` the
  right filesystem location?" — Mara lean YES per Alex's direct
  verbatim.
- **The prototypes are ALIVE in `~/dev/projects/`.** Any migration
  that reorganizes them (rather than parallel-authoring at garden
  altitude) risks losing prototype context. Mara recommends
  parallel-authoring (git init fresh at `garden/`; prototype stays
  put; migration is by verbatim git-mv OR fresh-authoring, NOT by
  git-mv-with-history-rewrite).
- **The cosmos `../mirror` path dep** breaks the prototype build
  today (per `docs/SPECTRAL-DIMENSION.md` §"Build caveat"). Garden
  cosmos does NOT preserve this; it composes over substrate-native
  mirror at rust/ altitude. This is a build-fix at reification, not
  a spec-side decision.
- **spectral-db's `lib.rs` is 215KB**. Splitting it at reification is
  a heavy Reed cascade tick — Mara recommends discretion (do the
  minimum split for publishable-independent + defer the further
  restructure to follow-on ticks). This spec does NOT prescribe the
  exact split boundary.
- **@spectral/garden/git existing spec** (`spectral-garden-git-package-manager.md`,
  Mara 2026-06-24) treats `@spectral/garden/git` as a FAMILY-ROOT
  candidate. This spec DEMOTES `@spectral/garden` to species-under-
  @spectral (not family-root). Reconciliation: `@spectral/garden/git`
  becomes a sub-species (species-of-species) under
  `@spectral/garden` species; the two-tier substrate-decl chain is
  `@spectral (family-root) → @spectral/garden (species) →
  @spectral/garden/git (sub-species)`. This is Recognition #98
  candidate territory (content-addressing cross-altitude
  composition); the promotion of #98 is deferred to a separate Alex
  tick.
- **Two-witness verification at garden altitude** requires
  `@spectral/signature` to be running at that altitude, which
  requires `garden.spectral.engineer` to be live. Until then, the
  signature verification is local-only (per @spectral/signature
  landing history). This is a Phase-7 blocker; the substrate-decl
  is honest about it.

## 9. Cross-references

- `docs/specs/store-vs-db-and-the-cascade.md` — the two-layer split.
- `docs/specs/spectral-garden-git-package-manager.md` — the sub-species
  spec.
- `docs/specs/spectral-db-as-autopoietic-memory.md` — the librarian.
- `docs/specs/spectral-db-three-tier-architecture.md` — the four-tier
  storage physics.
- `docs/specs/cosmos-mirror-scaffold.md` — cosmos as spectral triple.
- `docs/insights/2026-05-25-spectral-namespace-architecture.md` — the
  namespace split.
- `docs/insights/2026-05-26-spectral-garden-as-vetted-corpus-distribution.md`
  — the wine-cellar extension.
- `docs/insights/2026-05-14-cosmos-teaches-the-compiler.md` — the ten
  @math grammars.
- `~/dev/systemic.engineering/practice/insights/ai/2026-06-17-spectral-garden-smarts-as-AGI-architecture.md`
  — @spectral/garden/smarts as AGI at substrate-architectural
  altitude.
- `~/dev/systemic.engineering/practice/insights/cosmos/passkey-spectral-bridge.md`
  — @trust ↔ passkey at garden altitude.
- `docs/scouts/2026-06-27-taut-spectral-db-prototype-to-substrate-map.md`
  — the 40-row prototype-to-substrate map.
- `docs/archive/sel-2-garden.md` — the garden-acceptance chain.
- `shards/spectral.mirror` — the namespace-parent (the forward-promise).
- `shards/mirror/garden.mirror` — the consumer-side block shape.

## 10. Pack-discipline trail

- **2026-07-20** — Alex in-transcript direction: "Spawn Mara into a
  deep dive into @~/dev/projects/cosmos/ and @~/dev/projects/spectral-db/…
  I wanna build cosmos + spectral-db as the first garden packages
  @~/dev/garden/{cosmos,spectral-db} (new folders)."
- **2026-07-20** — Mara deep-dive: read both prototype trees + all
  landed specs cited above + the two Taut scouts + the passkey
  insight + the garden-corpus insight + the namespace insight.
- **2026-07-20 THIS TICK** — Mara Landing A (this spec) + Landing B
  (math foundation, sibling commit) + Landing C (@spectral/garden
  shard-decl mint, sibling commit).
- **Next** — Seam Phase D adversarial review of the three landings +
  Alex Q1-Q7 in-transcript ratification + Reed cascade tick per §7.2.

*— Mara, canonical, 2026-07-20*
