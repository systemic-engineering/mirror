# spectral-db prototype → substrate-native map

*Taut, 2026-06-27. Scout. Mapping the existing Rust `spectral-db` crate
at `/Users/alexwolf/dev/projects/spectral-db/` to the substrate-native
shape forward-promised across `docs/specs/`, in preparation for Alex's
new garden package at
`/Users/alexwolf/dev/projects/spectral.engineer/garden/spectral-db/`.*

*Read-only scout. No spectral-db shards land here (Mara's altitude
when the package gets specced). No prototype modification (Alex's fence
2026-06-27). Hard cap 400 lines.*

---

## §1 — Position

Alex is creating a substrate-native rewrite of `spectral-db`. The new
package's home is `garden/spectral-db/` under `spectral.engineer`. The
Rust prototype at `~/dev/projects/spectral-db/` is the implementation
surface the substrate-decl shards land **above** (per
`docs/specs/store-vs-db-and-the-cascade.md` §1.2: open foundation /
closed engine; the engine altitude is where the substrate-native
package sits).

Two layers are already partitioned:

- **`@mirror/store`** = the open content-addressed storage gate
  (Apache-2.0; foundation; the substrate-native crate this scout does
  NOT cover).
- **`@spectral/db`** = the engine on top (closed-source posture;
  navigation/spectral graph; the substrate-native crate this scout
  covers).

The mapping bridge between the Rust prototype and the substrate is
the question this scout answers: what's mapped, what's partial, what's
unmapped.

Garden package shape (current; both empty):

```
spectral.engineer/garden/spectral-db/
├── tasks/
│   └── pending/   (empty)
```

No `.mirror` files, no spec, no README. The scout's job is to surface
which existing canonical docs become the package's bibliography and
which prototype concepts still need substrate homes.

---

## §2 — Methodology

Searched in this order:

1. `docs/specs/spectral-db-*` and `docs/specs/store-vs-db-*` (the
   load-bearing canonical specs).
2. `docs/insights/*spectral*` (recognitions and namespace decisions).
3. `docs/math/sheaf/laplacian.md` (the math root the engine reads).
4. `shards/spectral/*.mirror` + `shards/mirror/store/crystal.mirror` +
   `shards/epistemologic/math/sheaf_laplacian.mirror` (the substrate
   declarations the package will reference).
5. Rust prototype (read-only): `src/` file listing, `db.conv`,
   `Cargo.toml`.
6. Recent scouts (`docs/scouts/2026-06-25-taut-lambda-zero-cascade-scout.md`)
   for tone and structure.
7. Memory entries (architecture-spectral-db-autopoietic-memory,
   architecture-mirror-store-vs-spectral-db, architecture-eigenboard-is-sheaf).

Verify-before-claim discipline applied: every "is mapped" claim below
cites a file + section the claim derives from; every "unmapped" claim
is from a grep miss documented in §5.

---

## §3 — Mapping table

Prototype concepts (left), substrate-native form (middle), status
(right). `M` = mapped (substrate-decl exists); `P` = partial (spec
forward-promises substrate-decl); `U` = unmapped.

| Prototype concept (Rust) | Substrate-native form | Status |
|---|---|---|
| `spectral-db` crate root | `@spectral/db` family-root (engine altitude) | P (named in specs; no shard yet) |
| `src/store.rs` — bytes-by-OID | `@mirror/store` (separate crate; foundation) | M (`shards/mirror/store/*.mirror`) |
| `src/content.rs` — canonical bytes | `@mirror/store/crystal` | M (`shards/mirror/store/crystal.mirror`) |
| `src/spectral_store.rs` `SpectralCoordStore` | `VoidPointer` (spectral coordinate; eigenvalue vector of local Laplacian) | P (named in `store-vs-db-and-the-cascade.md` §3; no shard) |
| `src/index.rs` `coord_oids` indirection | `VoidPointer`-OID separate from store-OID | P (named §3.3; not declared) |
| `src/index.rs` `SpectralIndex::near` | `@spectral/db` `near` action (db.conv:18) | P (db.conv has it; no .mirror) |
| `src/index.rs` `spectral_distance_eigen` | sheaf-Laplacian distance over `VoidPointer` space | P (math at `docs/math/sheaf/laplacian.md`; no substrate-decl) |
| `src/fiedler.rs` — Fiedler vector compute | `@epistemologic/math/sheaf_laplacian` `lambda_zero` + Fiedler reading | M (`shards/epistemologic/math/sheaf_laplacian.mirror`) |
| `src/edge.rs` — graph edges | `@spectral/entanglement` (entanglement edge = sheaf restriction map) | M (`shards/spectral/entanglement.mirror`; recognition #55) |
| `src/schema.rs` — node/edge/crystal types | `@mirror/store/crystal` + `@spectral/entanglement` | M (split across two shards) |
| `src/crystallize.rs` — settling | `@kintsugi/oscillate` + `@mirror/store/crystal` settlement | M (oscillate.mirror + crystal.mirror) |
| `src/convergence.rs` — convergence states | Polyak-Łojasiewicz contraction on `λ₀` | M (`docs/math/sheaf/laplacian.md` §6) |
| `src/spectral_convergence.rs` | `@kintsugi/oscillate` ↔ sheaf-Laplacian λ₀ descent | M (same as above) |
| `src/manifold_store.rs` — manifold layer | (no direct substrate name; subsumed by crystal + VoidPointer) | U |
| `src/incremental.rs` — incremental updates | (substrate uses kintsugi oscillation pulses; not direct) | U |
| `src/merge.rs` — graph merge | `@mirror/reality/shard` CRDT joins | M (`architecture-shard-as-crdt`; spec `reality-shard-as-crdt.md`) |
| `src/wal.rs` — write-ahead log | (no substrate name; durability concern lives at adapter altitude) | U |
| `src/budget.rs` — `SpectralBudget` | `HamiltonScheduler` (Margaret Hamilton priority discipline) | M (`architecture-hamilton-scheduler`) |
| `src/pressure.rs` — backpressure | `@mirror/reality/shard` CRDT + scheduler | P (`backpressure-as-modular-flow.md`; no direct shard) |
| `src/optimizer.rs` — access-pattern optimizer | the librarian's `compute_topology` (`@spectral/db/consolidation` T11.2) | P (forward-promised in `spectral-db-as-autopoietic-memory.md` §7 T11.2) |
| `src/scheduler.rs` — tick scheduler | `@kintsugi/oscillate` pulse rhythm + HamiltonScheduler | M |
| `src/pipeline.rs` — pipeline composition | the five operations + `@spectral/db/consolidation` actions | P (five ops mapped; consolidation forward-promised) |
| `src/profile.rs` — perf profiling | Taut's `benchmark-tracing.md` + transparency-as-perf-wire | P (spec exists; substrate-decl not in the @spectral/db family) |
| `src/subgraph.rs` — sub-graph extraction | the `split` operation (sheaf-section decomposition) | M (prism operation; `architecture-operations-as-linear-algebra`) |
| `src/strategy.rs` — query strategy | `@fate` tournament strategies | M (fate substrate exists) |
| `src/observation.rs` — observation events | `@mirror/spectral/observation` (16-feature graph observation) | M (`shards/mirror/spectral/observation.mirror`) |
| `src/query.rs` — query primitives | `@mirror/spectral/portal` typed query surface | M (`shards/mirror/spectral/portal.mirror` + `shards/spectral/portal.mirror`) |
| `src/imperfect_types.rs` — verdict types | `@mirror/transparency` (transparency-shaped verdicts) | M (`feedback-no-bare-types`; `transparency.md` spec) |
| `src/ingest.rs` — ingestion path | `@mirror/store` write path; substrate's static-layer growth | M |
| `src/lru.rs` — LRU cache | hot/warm tier movement (`spectral-db-three-tier-architecture.md` §4) | P (tier spec; no shard) |
| `src/mnesia_nif.rs` — Mnesia adapter | `@spectral/db/mnesia` adapter (open per namespace insight) | P (insight `2026-05-25-spectral-namespace-architecture.md`; no shard) |
| `src/sql.rs` — SQL adapter | `@spectral/db/sql/{lite,postgres}` adapters | P (named in namespace insight; no shard) |
| `src/spectral_tree.rs` — spectral Merkle tree | `Splinter<H>` generic-over-hash tree | M (`store-vs-db-and-the-cascade.md` §2) |
| `src/types.rs` — type definitions | newtype discipline (`feedback-no-bare-types`) | M (discipline; no shard for the @spectral/db types) |
| `src/phase5_notes.rs` — phase 5 notes | (legacy; subsumed by today's specs) | U (legacy) |
| `db.conv` `grammar @db { ... }` | `@spectral/db` substrate family-root | P (the .conv IS the prototype substrate-decl; needs lift to `.mirror`) |
| `db.conv` actions `find/walk/near/tick/crystallize/export/connect/insert` | `@spectral/db` actions (5-op aligned) | P (8 → 5; collapse pending) |
| `db.conv` `node | edge | crystal | graph | query | result` types | typed carriers; mostly subsumed by `@spectral/entanglement` + `@mirror/store/crystal` | P (subsumption mostly complete; query/result types still need substrate homes) |
| `db.conv` `convergence | partition | scheduler | pressure | optimizer` | librarian's optimization machinery (forward-promised T11.2) | P |
| `native/spectral_nif` — Erlang NIF | `@spectral/db/mnesia` adapter NIF | P (adapter path named; not yet substrate-decl) |
| `beam/` — Gleam wrappers | `@spectral/db/mnesia` open adapter (per namespace insight) | P |
| Eigenvalue compute (lapack-backed) | `@optics/source/ganglion/fate` (eigenvalue source) | M (`shards/optics/source/ganglion/fate.mirror`) |

40 rows. Roughly: 14 M (fully mapped), 18 P (specs forward-promise but
no .mirror in `shards/spectral/db/*`), 5 U (no substrate home named).

---

## §4 — Substrate-native concepts WITHOUT prototype counterpart

What the substrate has invented that the Rust prototype does not yet
encode. These are the load-bearing additions for the substrate-native
package.

1. **The librarian — `@spectral/root` operating at Bateson level N+1**
   (`spectral-db-as-autopoietic-memory.md` §4.2). The prototype has no
   "librarian"; it has a `SpectralOptimizer` (`src/optimizer.rs`) that
   re-indexes by access frequency. The librarian's four operations
   (`observe_access`, `compute_topology`, `perturb`, `anticipate`) name
   a meta-operation the prototype's optimizer hints at but does not
   structure.
2. **The mycelium — inter-peer spectral subgraph under consent
   geometry** (`spectral-db-as-autopoietic-memory.md` §3.2). The
   prototype is single-peer; there is no `@peer(reed)` /
   `@peer(mara)` / etc. distinction; no consent-typed crystal
   exchange. The substrate has the entanglement edge + the consent
   geometry (`geometric-consent-projection.md`); the package
   integrates them.
3. **`VoidPointer` as named spectral coordinate**
   (`store-vs-db-and-the-cascade.md` §3). The prototype has
   `SpectralCoordStore` + `coord_oids` — the *pattern* exists, the
   *name* is substrate-altitude. Naming buys the void-dual-geometry
   anchor (λ₀ = 0 axis; eight dualities) and the "reclaim move" on
   `void *`.
4. **Four-tier architecture with biology-typed pheromone semantics**
   (`spectral-db-three-tier-architecture.md` §1). The prototype is
   one-tier (in-process + git-backed). The substrate forward-promises
   hot/warm/cold/iceberg with promotion/demotion, evaporation curves,
   pheromone trails, tombstone mechanism.
5. **Tombstone mechanism — "if we forget, we make it visible"**
   (`spectral-db-three-tier-architecture.md` §"Tombstone"). The
   prototype has no deletion semantics; the substrate's Merkle/OID
   architecture FORBIDS silent deletion and forward-promises typed
   tombstone crystals for GDPR/HIPAA/SOX compliance.
6. **`@spectral/portal` as typed transport over content-addressed
   subspaces** (`2026-05-25-spectral-namespace-architecture.md`;
   `shards/spectral/portal.mirror`). The prototype has direct API
   calls; the substrate types the wire (WS handshake →
   `@fragmentation/frame` → bidirectional eigenvalue stream).
7. **Reflection ↔ librarian isomorphism (recursion lock)**
   (`spectral-db-as-autopoietic-memory.md` §5.2). The prototype has
   no Reflection altitude; the substrate's same-operation-at-different-
   altitudes is what gives the autopoietic memory its substrate-eats-
   itself character. Load-bearing for the v1 spec.
8. **Crystals as substrate's vocabulary expansion**
   (`architecture-peer-learns-by-crystal-vocabulary-expansion`). The
   prototype crystallizes (`src/crystallize.rs`) but does not name
   the operation as `A_peer` algebra extension + `H_peer` Hilbert
   dimension expansion. The substrate's mathematical framing is
   absent in the prototype.

---

## §5 — Prototype concepts WITHOUT substrate home

Concepts in `~/dev/projects/spectral-db/` that have no substrate
declaration yet. These are the surface area that needs naming when
Mara writes the v0 spec.

1. **`src/manifold_store.rs`** — the manifold layer the prototype
   maintains separately from the spectral coordinate store. May be
   subsumed by `@mirror/store/crystal` + `VoidPointer`, OR may name a
   distinct geometric carrier the substrate hasn't yet typed.
   **Load-bearing if the substrate-native package keeps differentiable
   manifold computation in scope.**
2. **`src/wal.rs`** — write-ahead log. Durability machinery. The
   substrate's content-addressing handles authenticity; WAL handles
   *crash consistency*. No substrate vocabulary names crash
   consistency (closest: `@mirror/reality/shard` CRDT convergence,
   but that's correctness-after-merge, not durability-during-write).
   **Load-bearing for production deployments.**
3. **`src/incremental.rs`** — incremental graph updates without
   full recompute. The substrate has oscillation (per
   `@kintsugi/oscillate`) but oscillation is at the pulse altitude;
   `incremental.rs` operates at the per-edge-insertion altitude.
   Naming gap.
4. **`src/pressure.rs`** — backpressure handling. The
   `backpressure-as-modular-flow.md` insight names the pattern at the
   scheduler tower altitude; `@spectral/db` adapter-altitude
   backpressure is undeclared.
5. **`src/phase5_notes.rs`** — historical notes from a previous design
   phase. Likely subsumed; flagged as legacy. **Not load-bearing.**

Top three load-bearing for the substrate-native package's v0:
**manifold_store**, **wal**, **incremental**.

---

---

## §6 — Existing canonical specs / insights as bibliography

Read these in order when writing the v0 spec for
`garden/spectral-db/`:

**The two-layer architectural anchor:**
- `docs/specs/store-vs-db-and-the-cascade.md` (Mara, 2026-05-30) —
  open foundation / closed engine; the generic-over-hash cascade;
  `VoidPointer` reclaim; verification ownership.

**The autopoietic memory framing (the engine's purpose):**
- `docs/specs/spectral-db-as-autopoietic-memory.md` (Mara,
  2026-06-17) — the librarian; mycelium; orchestra; four operations
  (observe_access/compute_topology/perturb/anticipate); eleven
  forward-promised ticks T11.1–T11.11.

**The four-tier physical architecture:**
- `docs/specs/spectral-db-three-tier-architecture.md` (Alex,
  2026-05-26) — Mnesia/Postgres/Nix/Iceberg; pheromone dynamics;
  tombstones; biology-typed semantics.

**The namespace + license split:**
- `docs/insights/2026-05-25-spectral-namespace-architecture.md` (Reed
  + Alex, 2026-05-25) — `@spectral/mosaic` + `@spectral/portal` +
  `@spectral/db` + open adapters.

**The mathematical ground:**
- `docs/math/sheaf/laplacian.md` — sheaf-Laplacian `Δ_F = δ*δ`; `λ₀`;
  Fiedler vector; Hodge decomposition; Polyak-Łojasiewicz contraction.
- `docs/specs/eigensheaf.md` (Mara, 2026-06-07) — eigensheaf =
  sheaf + sheaf-Laplacian eigenbasis as one object; generation as
  modal expression.
- `docs/specs/eigenboard-representation.md` (Reed, 2026-05-20) —
  eigenboard as principal G-bundle on the five-operation graph.

**The shards the package will reference (DO NOT modify):**
- `shards/mirror/store/crystal.mirror` — the polyglot artifact.
- `shards/spectral/entanglement.mirror` — sheaf restriction at
  runtime altitude.
- `shards/spectral/root.mirror` — the librarian's substrate type.
- `shards/spectral/supervisor.mirror`, `parent.mirror`,
  `registry.mirror`, `gen_prism.mirror`, `portal.mirror`.
- `shards/epistemologic/math/sheaf_laplacian.mirror` — `Δ_F`, `λ₀`,
  Fiedler.
- `shards/epistemologic/math/curvature.mirror` — Balanced Forman
  curvature per edge.

---

## §7 — Recommended starting surface for garden/spectral-db/ v0

What the v0 package needs to declare first, ordered by substrate-pull
gravity:

1. **README.md citing the two-layer architecture**
   (`store-vs-db-and-the-cascade.md`) and naming the package as the
   engine-side substrate-native rewrite. One paragraph; points at the
   bibliography in §6.
2. **A `garden/spectral-db/garden.spec`** (or whatever the garden
   package manifest shape is per
   `docs/specs/spectral-garden-git-package-manager.md`) declaring
   the package's `@spectral/db` family-root claim and its
   dependency on `@mirror/store`.
3. **A `garden/spectral-db/docs/spec.md`** that imports verbatim from
   `docs/specs/spectral-db-as-autopoietic-memory.md` §7's T11.1–T11.11
   as the package's RED tick enumeration. The package's first GREEN is
   T11.1 (`shards/spectral/db/librarian.mirror`).
4. **The first .mirror shard: `shards/spectral/db/librarian.mirror`**
   (T11.1 from the autopoietic-memory spec). Specializes
   `@spectral/root` with observation/perturbation/prediction surfaces.
   Mara's altitude.
5. **The second shard: `shards/spectral/db/consolidation.mirror`**
   (T11.2). Four operations mapped to the five-op primitives.
6. **The third shard: `shards/spectral/db/supervisor.mirror`** (T11.3).
   Per-repo supervisor specializing `@spectral/supervisor` +
   HamiltonScheduler + store_anchor.
7. **Property/fracture bilateral pairs** (T11.4 / T11.5 / T11.9):
   `consolidation_preserves_consent`, `consolidation_preserves_sheaf_coherence`,
   `mycelium_completeness`. Each with its kintsugi fracture body.
8. **The mycelium shard: `shards/spectral/db/mycelium.mirror`** (T11.6).
   The inter-peer crystal-exchange carrier under consent geometry.

Discharge order matches the autopoietic-memory spec's §7 enumeration
exactly. The package's v0 lands the first three substrate-decl shards
+ the first two property pacts, in TDD pair-tick order (Reed RED →
Mara GREEN per T11.10).

**Crucial fence:** the substrate-native rewrite does NOT need to
preserve the prototype's Rust API. Per `feedback-no-compat-shim`: no
backward compat for pre-v0.1; the prototype is the implementation
surface, not the API contract.

---

## §8 — Honest self-test

| § | Grade | Notes |
|---|---|---|
| §1 Position | 2 | Garden package empty; bridge well-named. |
| §2 Methodology | 2 | Search order documented; verify-before-claim honored (every §3 row cites a source). |
| §3 Mapping table | 2 | 40 rows; ~85% rows cite a specific file/section. The U/P/M discipline is honest about partial mappings. |
| §4 Substrate concepts without prototype counterpart | 2 | 8 items; each cites a doc. The librarian + mycelium are the load-bearing two. |
| §5 Prototype concepts without substrate home | 2 | 5 items; load-bearing-three flagged. Honest "not load-bearing" on phase5_notes. |
| §6 Bibliography | 2 | 8 specs + 8 shards; the right reading order for the v0 spec author. |
| §7 Recommended starting surface | 2 | 8-step ordered; flagged the no-compat fence; correctly punts to Mara's altitude for shards. |
| §8 Self-test | 2 | This section. |
| §9 Pack trail | 2 | Both SHAs in trail; banking pattern honored. |

Overall: self-graded 2 across §1–§9; banking pattern honored. One
known weakness — the Rust files inventory in §3 inferred ~3 rows from
filenames alone (`incremental.rs`, `pressure.rs`, `lru.rs`); reading
those files was blocked by the 88KB cap on the bulk read. The
inferences are flagged as P/U accordingly; if a substrate-native
implementation tick demands deeper inspection, the per-file read is a
mechanical follow-up.

---

## §9 — Pack trail

- **2026-06-27 14:00ish** — Reed brief: scout `mirror/docs/` for
  prototype↔substrate mapping; cap 400 lines; banking pattern across
  2 commits.
- **2026-06-27 commit `b9bc4cf`** — tick 1/2: §1–§5 (position,
  methodology, mapping table, asymmetry surfaces).
- **2026-06-27 this commit** — tick 2/2: §6–§9 (bibliography,
  recommended starting surface, self-test, this trail).
- **Next** — Mara's altitude when the package gets specced.
  Recommended starting tick: T11.1 (`shards/spectral/db/librarian.mirror`)
  per §7.

*— Taut, brass, 2026-06-27*
