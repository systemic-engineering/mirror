# `mirror ref` — the substrate's navigable surface, one CLI over two altitudes

*2026-06-20. Mara. Spec (substrate-decl + CLI surface).*
*Companion to the insight doc `docs/insights/2026-06-20-mirror-ref-as-substrate-honest-reflection-reference.md` (research synthesis; published earlier today). This spec settles the eight open questions from §8 of that doc into design positions (or named forward-promises), grounds the engine integration in concrete `spectral-db` symbols, and sketches the substrate-decl signature for `shards/mirror/ref.mirror` that Reed compiles.*

*Status: **Red** at the shard altitude (the substrate-decl below is the signature Reed lands; iteration in conversation expected before the GREEN). The CLI surface, the engine grounding, and the eight resolutions land here for review.*

Reads from / depends on:
- `docs/insights/2026-06-20-mirror-ref-as-substrate-honest-reflection-reference.md` (research; the structural claim, the prior-art lineage, the eight open questions)
- `shards/mirror/bench.mirror` (#87 — `bench_crystal`, `monotone_non_increasing`, the three-conjunct discipline `mirror ref` measures per-query against)
- `shards/loop.mirror` (#88 — `@loop`, `bind`, `pact_respected`; each ref query is ONE tick of `@loop`)
- `shards/moi.mirror` (#86 — `@moi`, the composition-time-pact-verified result type every ref-query produces)
- `shards/reflection.mirror` (#85 — `@reflection.observe`, the temporal projection `mirror ref observe` walks)
- `shards/reflection/mirror.mirror` (the Mirror Model — `navigate(q: query) -> moi(graph_path)`; `mirror ref` IS this surfaced at CLI)
- `shards/mirror/store/crystal.mirror` (the altitude-4 crystal; query results lift to `moi(graph_path)` and persist as crystals)
- `shards/prism.mirror`, `shards/glass.mirror` (`imperfect`, `transparency<p>`, `splinter`, `pact`)
- `shards/mirror/spectral/eigensheaf.mirror` (the sheaf-Laplacian `mirror ref spectral` reads)
- `shards/mirror/spectral/consent.mirror` (`query_phi` — the predicate carrier `mirror ref where` consumes)
- `shards/mirror/mosaic.mirror` (the build-altitude manifold `mirror ref` navigates)
- `shards/mirror/store.mirror` (the OPEN content-addressed gate)
- `docs/specs/mosaic-as-type-system.md` (the splinter/shard/SpectralUuid triple `mirror ref` reads as nodes)
- `docs/specs/reflection-model.md` (the Four Models; `mirror ref` is the Mirror Model's CLI face)
- `/Users/alexwolf/dev/projects/spectral-db/src/lib.rs` (the engine; `SpectralDb` surface)
- `/Users/alexwolf/dev/projects/spectral-db/src/pipeline.rs` (the pipe-forward algebra `mirror ref` rides)

Forward references (this spec unblocks):
- `shards/mirror/ref.mirror` (the substrate-decl Reed lands; signature in §6 of this spec)
- `bootstrap/src/main.rs::cmd_ref` (the CLI dispatch; tick after the shard compiles)
- `shards/mirror/fracture/ref_query_compositional.mirror` (the bilateral predicate's discharge body; first-witness consumer the CLI parse-time desugarer)
- the eight `@kintsugi/fracture/*` shards from §5 of the insight doc (each lands as a #53 second-witness instance after `mirror ref` ships)

---

## 1. The structural claim

`mirror ref` is the navigable surface of the substrate's spectral triple (A, H, D) per Connes — the (mosaic algebra, expanding-Hilbert-space-of-shards, kintsugi-flow) operational form per `[[architecture-connes-spectral-triple]]`. The CLI exposes the navigation of H *through* A, with D driving the optimization. The reference altitude is the spatial projection of H (the shard dependency lattice); the reflection altitude is the temporal projection of H (the `@reflection.observe` predecessor chain over `@loop` ticks). Both share A as the navigation algebra. One CLI surface; two altitudes; one substrate-decl shard.

The two words English glues into "ref" — *reference* (every prior xref tool from Smalltalk-1980 through Erlang's `xref` through Bazel's query language through Joern's CPG) and *reflection* (the `@reflection.observe` one-tick-delay structural primitive per recognition #85) — collapse to one substrate-honest surface per the #pack-G2 pattern (`/loop` ⇔ `@loop`, `@reflection-family` ⇔ `@reflection/reflection-Model`). This is not a name pun; it is the substrate disclosing that the same five-operation algebra navigates both projections of H.

---

## 2. CLI surface — exhaustive

### 2.1 Conventions

```
mirror ref <subcommand> [<target>] [<flags>]
```

- `<target>` is an `@<altitude>` reference (e.g., `@magic`, `@kintsugi/oscillate.settle`, `@reflection`) or a path within the `mirror.spec` manifold (e.g., `shards/loop.mirror`). Per `[[architecture-at-x-is-mathematical-value]]`, `@X` is admitted as a value at CLI argument positions and resolves to a `splinter`/`shard` handle in the engine.
- Subcommands are sugar for the five-operation primitives (§2.3); the desugaring is enforced at parse time by `ref_query_compositional` (§6.3).
- Default output is mirror-text (the substrate's wire form per kintsugi-ci-v0.1 §1.4). JSON appears only behind `--out @data/json`; DOT only behind `--out @data/dot`. Both are `@io` boundary crossings.
- Exit codes: `0` success (pass) or partial-within-threshold; `1` partial-out-of-threshold; `2` failure (query refused, predicate violated, pact unwitnessed); `3` parse error (the parse refused to desugar).

### 2.2 Global flags

| Flag | Default | Semantics |
|---|---|---|
| `--out <@data-altitude>` | `@data/mirror` (mirror-text) | Render altitude. Crosses `@io` only for `@data/json` and `@data/dot`. |
| `--depth <n>` | `1` | Traversal depth for transitive operations. `0` = node only; `*` = unbounded (engine-capped). |
| `--threshold <p>` | `0.0` | Opacity floor; nodes with `transparency<p>` below threshold are excluded. |
| `--altitude <@X>` | inferred from target | Pin the navigation altitude when the target's altitude is ambiguous. |
| `--since <commit>` | none | Constrain temporal projection to ticks after the given content-addressed commit. |
| `--bench` | off | Emit a `bench_crystal` for this query via `@mirror/bench.record`; persists in `@mirror/store` for `monotone_non_increasing` regression checks. |
| `--observe` | off | Lift this query as an `observation` and pass to `@reflection.observe`; subject to the one-tick-delay per `speaks_at_n_plus_1`. |
| `--ci` | off | Emit aggregate verdict; non-zero exit on `--threshold` violation. Mirrors the kintsugi-ci-v0.1 verdict shape. |
| `--traverse-io` | off | Walk through `@io/<species>` into external dependency graphs. See §5.8 of insight doc. |

### 2.3 The five-operation primitive surface

Each operation is individually invocable. The CLI form maps to one stage of the `Pipeline` algebra in `spectral-db/src/pipeline.rs` (see §4 for the binding).

| CLI form | Algebraic meaning | Engine binding (spectral-db) |
|---|---|---|
| `mirror focus <@target>` | Concentrate on one eigenvector / one node + ego-subgraph | `SpectralIndex::ego_subgraph(center, hops)` |
| `mirror project <pipe-in> --kind=<k> [--where=<phi>]` | Orthogonal projection: predicate filter on the result set | `Transform::Where { field, op, value }` |
| `mirror split <pipe-in> --by=<axis>` | Orthogonal decomposition: partition into equivalence classes | `Source::Hot` + `Crystallizer::observe_hot_paths` (cycle/family axes via §4.5) |
| `mirror shift <pipe-in> --basis=<b>` | Basis transformation; `--basis=@spectral` emits eigendecomposition | `SpectralIndex::compute_spectral_coordinates`, `SpectralIndex::spectral_hash_vec` |
| `mirror settle <pipe-in> @<fracture>` | Fix-point via kintsugi: apply a fracture body to the projection | `SpectralDb::settle(message)` composed with `@kintsugi.tick` |

All five emit a typed `GraphRef`; all five consume `GraphRef` (or nothing, when sourcing). The shell pipe `|` IS the substrate's monad (`@moi`) at the @io shell altitude.

### 2.4 The named-shortcut surface (ergonomic affordance; sugar for §2.3)

Each shortcut compiles at parse time to a pipeline of primitives via `ref_query_compositional` (§6.3). A shortcut that does not desugar is rejected with exit code 3.

```
mirror ref deps @<target>                       # focus @<target> | project --direction=out --transitive
mirror ref rdeps @<target>                      # focus @<target> | project --direction=in  --transitive
mirror ref callers @<action>                    # rdeps but at action altitude
mirror ref implementors @<prism-action>         # split @<action> --by=predicate=implements
mirror ref unresolved                           # project --kind=hole --where=unresolved
mirror ref unused                               # project --reachability=unreachable
mirror ref cycles [--min-size=N]                # split --by=cycle [--filter=size>=N]
mirror ref paths @<from> @<to> [--all|--some]   # split --by=reachability(@<from>, @<to>)
mirror ref spectral @<family>                   # shift --basis=@spectral
mirror ref pacts @<target>                      # project --kind=pact
mirror ref ast @<target>                        # focus + split --by=ast-child (per splinter(ast) #54)
mirror ref where '<phi>'                        # project --where=<phi> (query_phi consumed directly)
mirror ref cold [--since=HEAD~N]                # project --where=not-exercised-in-window
mirror ref observe @reflection [--since=HEAD~N] # reflection altitude: walk @reflection.observe record
mirror ref tick @loop [--since=HEAD~N]          # reflection altitude: walk @loop predecessor chain
mirror ref settle <fracture>                    # kintsugi-bridge: apply fracture body
```

### 2.5 Pipeline composition examples

The shell pipe is the substrate's monad bind operator (`@moi.bind`). Composition is associative for free per content-addressing (per `boot/00-prism.mirror`).

```sh
# Find the dep graph of @magic, project to compile edges, settle unreachables out
mirror ref deps @magic | mirror ref project --kind=compile | mirror settle @kintsugi/fracture/prune_unreachable

# Spectral analysis of the @kintsugi family — emit Fiedler cut to DOT
mirror focus @kintsugi | mirror shift --basis=@spectral --out @data/dot

# Reflection altitude: walk @loop's last 5 ticks, project to loss-improving ticks
mirror ref tick @loop --since=HEAD~5 | mirror ref where 'loss-decreasing'

# Pact-trace audit: pacts that have no living fracture body
mirror ref pacts --unsupported | mirror settle @kintsugi/fracture/forge_missing_fracture

# Bench-aware regression check across one tick
mirror ref deps @magic --bench --since=HEAD~1
```

### 2.6 Output formats

**Default (`@data/mirror`).** Mirror-text records per kintsugi-ci §1.4 — blank-line-separated `<key> <value>` blocks, one block per result node, plus one aggregate block at the head.

```text
verdict      success
target       "@magic"
operation    "deps"
depth        1
node_count   12
edge_count   18
loss         0.0

node         "@magic.invariant_preserved"
kind         predicate
altitude     "@epistemologic/property"
transparency success

node         "@magic.audit"
kind         action
altitude     "@magic"
transparency partial(0.13)
```

**`--out @data/json`.** Same field set; JSON-shaped for `jq` / `$GITHUB_OUTPUT` consumption. `@io` crossing only.

**`--out @data/dot`.** Graphviz DOT for spectral visualization (`mirror ref spectral`, `mirror ref cycles`, `mirror ref paths`). `@io` crossing only.

### 2.7 Exit-code matrix

| Condition | Exit |
|---|---|
| Query desugars, executes, threshold met | `0` |
| Query desugars, executes, partial-within-threshold | `0` |
| Query desugars, executes, partial-out-of-threshold (`--ci`) | `1` |
| Query desugars, executes, predicate-refusal (e.g., `loss_decreases` violated) | `2` |
| Query desugars, executes, pact-unwitnessed (e.g., `pact_respected` failed) | `2` |
| Query fails to desugar to primitives (`ref_query_compositional` rejects) | `3` |
| `@io` boundary crossing refused (e.g., `--traverse-io` without consent) | `2` |

---

## 3. Substrate-decl shape — `shards/mirror/ref.mirror` (signature sketch; Reed lands)

Match the family-root pattern from `shards/mirror/bench.mirror` (#87), `shards/loop.mirror` (#88), `shards/moi.mirror` (#86), `shards/reflection.mirror` (#85). The shard declares a family-root prism, parametric carriers (typed refs per `[[feedback-no-bare-types]]`), typed actions with obligation blocks `{ \ }`, and bilateral predicates whose first non-decorative consumer is named via `requires`.

### 3.1 Header + ancestry + path-namespace property

```mirror
in @prism
in @meta
in @glass
in @moi
in @magic
in @loop
in @reflection
in @reflection/mirror
in @kintsugi
in @mirror/store
in @mirror/spectral
in @mirror/bench
in @mirror/au
in @epistemologic
in @epistemologic/property
in @epistemologic/reality/time
in @code/mq

# @mirror/ref — the navigable surface of the substrate's spectral
# triple. One CLI over two altitudes (reference / reflection).
#
# Recognition #89 (forward-promised; this spec is the substrate-decl
# preparation). Composes the today's cascade #85-#88 at the CLI
# altitude. The substrate's expanding Hilbert space (per recognition
# #51) gets a navigation handle.
#
# === The collision (Pack G2) ===
#
# Reference (spatial projection of H; shard dependency lattice) and
# Reflection (temporal projection of H; @reflection.observe predecessor
# chain over @loop ticks) collapse to one CLI per the #pack-G2 pattern
# (/loop ⇔ @loop; @reflection-family ⇔ @reflection/reflection-Model;
# reed-as-relationship ⇔ reed-as-peer). Same name; two altitudes; one
# five-operation algebra.
#
# === Spec context ===
#
# docs/specs/mirror-ref-spec.md (this shard's spec); the eight kintsugi
# optimization pipelines (forward-promised at @kintsugi/fracture/*);
# the engine integration grounded in spectral-db's Pipeline algebra
# (src/pipeline.rs::Pipeline / Source / Transform / Terminal).

source @arxiv/programming/erlang-xref-1993
source @arxiv/programming/bazel-query
source @arxiv/programming/joern-cpg-2014
source @arxiv/programming/glamorous-toolkit-2020
source @arxiv/math/connes-1994
source @arxiv/math/fiedler-1973

# Path-namespace property: declared at shards/mirror/ref.mirror per
# @epistemologic/pact/path_matches_namespace.
```

### 3.2 Family-root prism

```mirror
prism @mirror/ref {
  focus  ref
  project ref
  split  ref
  shift  ref
  settle ref
}
```

### 3.3 Carriers (parametric refs per the today's-cascade pattern)

```mirror
# The typed graph projection. Imperfect-wrapped per @glass's loss
# carrier; the active 48 bits of the underlying SpectralUuid carry
# the Laplacian-neighbourhood structure; transparency<altitude> carries
# the located opacity per node/edge.
type graph_ref = imperfect(graph_handle, query_error, transparency(@mirror/ref))

# Typed reference to a graph node — a shard at the SpectralUuid
# layer or a splinter at the OID layer (per shards/glass.mirror's
# three-layer recognition; see docs/specs/mosaic-as-type-system.md §1B).
type node_ref = ref

# Typed edge with a substrate-discriminated kind. The seven variants
# enumerate the substrate's first-class edge kinds; same-shape
# different-meaning forbidden per [[feedback-no-bare-types]].
type edge_kind =
  | in_edge              # the substrate ancestry header
  | callers_edge         # action-to-action call site
  | pact_edge            # property -> fracture discharge
  | fracture_edge        # fracture -> splinter(ast) discharge
  | kintsugi_bridge      # post-settle continuity link
  | reflection_predecessor # the @loop tick chain
  | ast_child            # splinter(ast) parent → child

# The typed edge carrying its kind and a located opacity.
type edge_ref = imperfect(edge_handle, edge_error, transparency(@mirror/ref))

# The structural Φ query, lifted from @mirror/spectral/consent's
# query_phi as a first-class carrier at @mirror/ref altitude.
type query_phi = imperfect(predicate, query_error, transparency(@mirror/ref))

# The graph_path output type per @reflection/mirror's navigate action.
# Reused (not redeclared) per the substrate-already-had-the-word
# discipline; this carrier is the @mirror/ref signature's return type.
# graph_path is in scope via `in @reflection/mirror` (transitive).

# Two opaque handles; bound to engine entities at the realisation
# boundary (spectral-db's NodeOid + Edge); forward-promised parametric
# refinements: graph_handle(altitude), edge_handle(edge_kind).
type graph_handle = ref
type edge_handle  = ref
type query_error  = ref
type edge_error   = ref
type predicate    = ref
```

### 3.4 Actions (typed lambdas with obligation blocks)

```mirror
# === focus_at (THE source action; lifts an @<altitude> into graph_ref) ===
#
# Substrate-vocabulary primitive for naming a region of H. Lifts an
# altitude reference + depth into a typed graph projection. Composes
# with spectral-db's SpectralIndex::ego_subgraph at the realisation
# boundary.
focus_at(target: ref, depth: ref) -> graph_ref { \ }

# === project_by (predicate filter; consumes query_phi) ===
#
# Orthogonal projection: filters nodes/edges by structural Φ query.
# Composes spectral-db's Transform::Where + Crystallizer's hot-path
# discrimination at the realisation boundary.
project_by(phi: query_phi, g: graph_ref) -> graph_ref { \ }

# === split_by (orthogonal decomposition) ===
#
# Partitions a graph_ref into equivalence classes (cycle, family,
# altitude, reachability-from-target). Returns a typed reference to
# the set of partitions; iteration is forward-promised at the
# realisation boundary (the set-of-graph_ref type lands once the
# substrate's parametric collection family settles).
split_by(axis: ref, g: graph_ref) -> ref { \ }

# === shift_to (basis transformation; spectral when basis=@spectral) ===
#
# Basis transformation on the underlying sheaf-Laplacian. Composes
# spectral-db's SpectralIndex::compute_spectral_coordinates +
# spectral_hash_vec at the realisation boundary. The @spectral basis
# emits the Fiedler value, the harmonic dimension, and the spectral
# gap as substrate-typed fields on the returned graph_ref.
shift_to(basis: ref, g: graph_ref) -> graph_ref { \ }

# === settle_with (kintsugi fix-point; consumes a fracture; FIRST
# consumer of ref_query_compositional) ===
#
# Applies a @kintsugi/fracture/<predicate> body to a graph_ref;
# returns @moi(au) per recognition #86. The result is the typed
# post-fracture region with composition-time pact-verified geometry.
#
# requires ref_query_compositional(phi, g): the pipeline that
# produced g must desugar to a composition of the five primitives.
# Without this requires clause, settle_with would admit Narcissus-pole
# composition where the projection's history is opaque (per
# alignment-as-boundary-mathematics #57). The bilateral predicate IS
# the substrate-mathematical guarantee that every settle is pact-
# auditable.
settle_with(fracture: ref, g: graph_ref) -> moi(au)
requires ref_query_compositional(phi_of(g), g)
{ \ }

# === observe_at (the reflection altitude entry point) ===
#
# Lifts a graph_ref to the reflection altitude by wrapping it as an
# observation per shards/reflection.mirror. Composes with
# @reflection.observe at the realisation boundary; subject to the
# one-tick-delay via speaks_at_n_plus_1 cascading from the speak
# action that consumes the observation downstream.
observe_at(g: graph_ref, t: tick) -> observation { \ }

# === phi_of (lifts a graph_ref's lineage into a query_phi) ===
#
# Substrate-introspection action: returns the structural Φ query that
# produced a graph_ref. Used by ref_query_compositional's verifier
# at parse time AND at settle_with's composition-time check. The
# substrate makes the pipeline auditable from its result; this is
# the substrate-pull-correct read of "every query has a content-
# addressed history."
phi_of(g: graph_ref) -> query_phi { \ }

# === interface_authored_by_human (cycle-break consent predicate) ===
#
# Bilateral verdict per Seam C9 rim closure. ANY action that would
# write an interface shard (the cycle-break output of
# @kintsugi/fracture/break_cycle, per §5.3) MUST consume this
# predicate via `requires`. The substrate refuses to create the
# interface shard automatically; the prose policy becomes a typed
# obligation at the substrate-decl altitude.
#
# Bounded → a human author signed off on the proposed cut + redirect
# (witnessed by a content-addressed commit attribution at @io); the
# substrate admits the shard write. Unbounded → the shard write is
# refused with exit code 2 (consent-unwitnessed).
#
# Discharge: at the @io boundary, the predicate reads the proposed
# splinter(@meta/ast)'s commit-attribution chain and witnesses that
# a human-attributed commit closes it. The substrate-pull-correct
# read of the no-add-without-pact discipline at the cycle-break site.
interface_authored_by_human(s: splinter(@meta/ast)) -> verdict { \ }
```

### 3.5 The bilateral predicate (the #53 self-application instance)

```mirror
# === ref_query_compositional predicate (bilateral; self-applied) ===
#
# Bilateral verdict per recognition #37 (Pask agreement) AND the
# property/fracture bilateral pattern #53. THE SELF-APPLICATION:
# @mirror/ref is itself a bilateral instance — the compositional
# discipline that keeps the bag-of-subcommands surface (§2.4) from
# drifting from the primitive composition surface (§2.3).
#
# Per Seam B1 closure pattern: settle_with (above) is the FIRST
# non-decorative consumer via `requires ref_query_compositional(
# phi_of(g), g)`. Without this requires clause, the desugaring layer
# could drift; the substrate would admit shortcuts that have no
# primitive equivalent.
#
# Discharge: at CLI parse time, each subcommand is lowered to a
# pipeline of focus / project / split / shift / settle; the
# predicate's bounded verdict witnesses that the lowering closed.
# Unbounded → the subcommand has no primitive composition; the parse
# refuses with exit code 3.
#
# Same shape as the bilateral predicates the prior families carry:
# - @magic: invariant_preserved, audited, mechanism_intact
# - @frame: bounded_commutator_check
# - @moi: pact_respected
# - @reflection: loss_decreases, speaks_at_n_plus_1
# - @mirror/bench: monotone_non_increasing, bench_overhead_below_floor
# - @loop: loop_well_founded
# - @mirror/ref: ref_query_compositional (this one)
ref_query_compositional(phi: query_phi, g: graph_ref) -> verdict { \ }
```

### 3.6 The `out` declarations

```mirror
out @mirror/ref
out graph_ref
out node_ref
out edge_kind
out edge_ref
out query_phi
out focus_at
out project_by
out split_by
out shift_to
out settle_with
out observe_at
out phi_of
out ref_query_compositional
out interface_authored_by_human
```

---

## 4. `spectral-db` integration — concrete grounding

The CLI surface composes by lowering each primitive to one or more calls into the `SpectralDb` engine. The substrate does not reinvent the graph database; it consumes it. Each table row below names the engine symbol at its source location.

### 4.1 Source primitives (CLI `focus`)

| `mirror ref` operation | spectral-db symbol | File |
|---|---|---|
| `mirror focus @<type>` (by node type) | `SpectralDb::find(node_type) -> ResultSet` | `src/lib.rs::SpectralDb::find` |
| `mirror focus @<oid> --near=<d>` | `SpectralDb::near(target_oid, distance) -> ResultSet` | `src/lib.rs::SpectralDb::near` |
| `mirror focus <region> --hops=<n>` | `SpectralIndex::ego_subgraph(center, hops) -> EgoSubgraph` | `src/index.rs::SpectralIndex::ego_subgraph` |
| `mirror focus --hot` | `SpectralDb::crystals()` + projection per `Source::Hot` | `src/lib.rs::SpectralDb::crystals`, `src/pipeline.rs::Source::Hot` |

### 4.2 Transform primitives (CLI `project`, `split`, `shift`)

| `mirror ref` operation | spectral-db symbol | File |
|---|---|---|
| `mirror project --where=<phi>` | `Transform::Where { field, op, value }` (via `parse_pipeline`) | `src/pipeline.rs::Transform::Where` |
| `mirror project --transitive --depth=<n>` | `Transform::Walk(depth)` + `SpectralDb::walk` | `src/pipeline.rs::Transform::Walk`, `src/lib.rs::SpectralDb::walk` |
| `mirror project --sort-by=<f>` | `Transform::Sort { field, order }` | `src/pipeline.rs::Transform::Sort` |
| `mirror project --limit=<n>` | `Transform::Limit(n)` | `src/pipeline.rs::Transform::Limit` |
| `mirror ref where 'matching <query>'` | `Transform::Matching(query)` (Jaccard via mirror NL tokenizer) | `src/pipeline.rs::Transform::Matching` |
| `mirror shift --basis=@spectral` | `SpectralDb::compute_spectral_coordinates` + `spectral_distance_eigen` | `src/lib.rs::SpectralDb::compute_spectral_coordinates` |
| `mirror shift --spectral-hash` | `SpectralDb::spectral_hash(oid)` + `SpectralIndex::spectral_hash_vec` | `src/lib.rs::SpectralDb::spectral_hash`, `src/index.rs::SpectralIndex::spectral_hash_vec` |
| `mirror ref cycles` (Fiedler partition) | `fiedler::NetworkMonitor::fiedler_value` + `check() -> PartitionRisk` | `src/fiedler.rs::NetworkMonitor::fiedler_value` |

### 4.3 Terminal primitives (CLI `settle`)

| `mirror ref` operation | spectral-db symbol | File |
|---|---|---|
| `mirror settle <fracture>` (commits a kintsugi pass) | `SpectralDb::settle(message) -> git2::Oid` | `src/lib.rs::SpectralDb::settle` |
| `mirror ref --bench` (records a bench_crystal) | `Crystallizer::crystallize_settled` + `@mirror/bench.record` | `src/crystallize.rs::Crystallizer::crystallize_settled` |
| `mirror ref unused | settle prune_unreachable` | `Crystallizer::invalidate_stale` | `src/crystallize.rs::Crystallizer::invalidate_stale` |
| `mirror ref pacts --unsupported` (manifest pacts) | `SpectralIndex::edges_by_provenance(EdgeProvenance::Explicit)` | `src/index.rs::SpectralIndex::edges_by_provenance`, `src/edge.rs::EdgeProvenance` |

### 4.4 Convergence / temporal primitives (CLI `observe`, `tick`)

| `mirror ref` operation | spectral-db symbol | File |
|---|---|---|
| `mirror ref observe @reflection` | `SpectralDb::graph_hash() -> GraphHash` + `convergence::check` | `src/lib.rs::SpectralDb::graph_hash`, `src/convergence.rs::check` |
| `mirror ref tick @loop --since=<commit>` | `incremental_index(db) -> IncrementalResult` over the predecessor chain | `src/incremental.rs::incremental_index` |
| `mirror ref spectral @<family> --through-time` | `spectral_convergence::SpectralRef` + `spectral_dimension` | `src/spectral_convergence.rs::SpectralRef`, `src/convergence.rs::spectral_dimension` |
| `mirror ref where 'partition-risk > <p>'` | `NetworkMonitor::check() -> PartitionRisk` | `src/fiedler.rs::NetworkMonitor::check` |

### 4.5 Pipeline composition (the shell pipe IS the algebra)

The shell pipe lowers to `parse_pipeline` + `execute_pipeline`:

- `parse_pipeline(input: &str) -> Result<Pipeline, ParseError>` (`src/pipeline.rs::parse_pipeline`) parses a pipe-forward query string.
- `execute_pipeline(db, pipeline) -> Result<QueryResult, ParseError>` (`src/pipeline.rs::execute_pipeline`) runs the source stage, applies transforms sequentially, computes Shannon loss as `-log2(kept_ratio)`, applies the terminal.
- `SpectralDb::query_pipeline(query: &str) -> Result<QueryResult, Error>` (`src/lib.rs::SpectralDb::query_pipeline`) is the one-shot entry point the CLI calls.

The `QueryResult` shape (`src/pipeline.rs::QueryResult`) carries `{nodes, count, loss}` — the loss field IS the substrate's `transparency<p>` at the query altitude. The CLI maps `loss == 0.0` → `verdict success`; `loss > 0.0 && loss < threshold` → `verdict partial(loss)`; `loss >= threshold` → `verdict failure(loss)`.

### 4.6 The `Crystal` / `bench_crystal` bridge

Per recognition #87 (`@mirror/bench`), each `--bench`-flagged query persists as a content-addressed crystal. Two carriers, two altitudes; the bridge between them is named here and forward-promised at forward-promise #11.

- **Engine-side (storage primitive):** `crystallize::Crystal { nodes, stability_scores, hash, created_at, manifold }` (`src/crystallize.rs::Crystal`) is the lean storage carrier the engine actually maintains. `nodes: Vec<String>` is the crystallized node set; `stability_scores: Vec<f64>` are per-node rescan counters at crystallization time; `hash: [u8; 32]` is the content hash; `created_at: u64` is the epoch-seconds timestamp; `manifold: Imperfect<ManifoldOid, String, ApertureLoss>` carries the manifold provenance. There is NO `predecessor` field today. `Crystallizer::observe_hot_paths(hot_paths)` (`src/crystallize.rs::Crystallizer::observe_hot_paths`) is the entry the bench-flagged path calls; `Crystallizer::crystallize_settled()` (`src/crystallize.rs::Crystallizer::crystallize_settled`) emits the crystal; `CrystalRecord::from_crystal(c)` (`src/crystallize.rs::CrystalRecord::from_crystal`) emits the persistable JSON form `{nodes, stability_scores, hash, created_at}`.

- **Shard-side (substrate-altitude carrier):** `bench_crystal { op, args_oid, runtime_ns, output_oid, env_oid, predecessor, tick_index }` declared in `shards/mirror/bench.mirror` (per recognition #87) is the richer substrate-altitude carrier. The `predecessor: ref` field links to the prior crystal for the same `op + args_oid + env_oid`, making the regression suite a DAG-walk; the `tick_index: tick_index` field gives the discrete order. `monotone_non_increasing` (three-conjunct) gates whether a successor is a regression.

- **The bridge:** the engine's lean `Crystal` is the storage primitive; the shard-side `bench_crystal` is the substrate-altitude carrier with richer schema. The bridge is named but not yet implemented — the engine's `Crystal` must gain a `predecessor: Option<CrystalId>` field before the shard-side three-conjunct can lower to a typed engine call. Until that lands, the shard-side `predecessor` is computed by the bench-recording layer (lookup-by-(op,args,env)-and-pick-latest) rather than being a direct field on the engine carrier. Forward-promised at #11.

The crystal IS one tick of `@loop` at measurement altitude (per the recognition narrative in `shards/mirror/bench.mirror`); `mirror ref --bench` enqueues a `bench_crystal` whose `predecessor` field links to the prior crystal for the same `op + args_oid + env_oid` via the bridge above.

### 4.7 The `Imperfect` channel (loss-as-transparency)

The engine's `imperfect_types` module (`src/imperfect_types.rs`) declares the engine-side instances of `transparency<p>`:

- `ShannonLoss` (`src/types.rs::ShannonLoss`) — the default loss carrier for `QueryResult.loss`.
- `DistanceLoss`, `MeasurementLoss`, `EdgeLoss`, `AnnotationLoss`, `SpanLoss` — per-context losses bubbled into the `transparency<p>` field of `graph_ref`.

The CLI's `--threshold <p>` flag compares against the engine-emitted loss; the `transparency<p>` field on the returned `graph_ref` IS the engine's loss carrier serialized at the substrate-decl altitude.

---

## 5. Resolution of the eight open questions from §8 of the insight doc

### 5.1 Q1 — Bag-of-subcommands vs pure pipeline at CLI

**Position: BOTH; bag-of-subcommands is sugar enforced by `ref_query_compositional` at PARSE time.**

The pure pipeline is honest; the bag is ergonomic. The bilateral predicate (§3.5) bridges them: at parse time, each shortcut is lowered to a primitive composition, and the predicate's bounded verdict witnesses that the lowering closed. A shortcut that does not lower is rejected with exit code 3.

**Substrate-pull confidence: HIGH.** This is the substrate-pull-correct version of `mix xref`'s closed enum — except open, because new shortcuts can be added by extending the desugarer's table without changing the primitive surface.

### 5.2 Q2 — Reflection altitude visibility under consent

**Position: per-altitude ACL projection by default; per-tick override via `--since`/`--until` flags; per-shard override forward-promised.**

Per `[[architecture-geometric-consent-projection]]` (Alex 2026-06-17), ACL cascades down the logic-level hierarchy; positive consents cascade down, rejection does not cascade up (security invariant). The default for `mirror ref observe` is altitude-level: a query at altitude L returns results visible at altitude L and below. Per-tick narrowing via `--since`/`--until` is implemented at v1; per-shard ACL override (the most granular case) is forward-promised to a `--scope <@altitude>` flag once the substrate's ACL declaration substrate-decl lands at family altitude.

**Substrate-pull confidence: MEDIUM-HIGH.** The cascade discipline is well-grounded; the per-shard granularity needs the ACL declaration shard before it can ship.

### 5.3 Q3 — Cycle-breaking discipline (does one pact authorize N new shards?)

**Position: NO — `mirror ref cycles | settle break_cycle` proposes a cut and emits a typed-hole interface shard at the Fiedler-cut location. Any action that would write the interface shard MUST consume `requires interface_authored_by_human(s)` (§3.4); the substrate refuses via the predicate (the prose-policy becomes a typed obligation). Reed (or a peer) authors and commits it; the pact `no_compile_cycle` is discharged only after the shard lands and a second settle pass succeeds.**

This is the substrate's no-add-without-pact discipline made operational at `mirror ref`. The fracture body `@kintsugi/fracture/break_cycle` emits a `proposed_interface: splinter(@meta/ast)` carrying the cut and the redirect — it does NOT write to disk. The `interface_authored_by_human(s)` predicate (§3.4) IS the structural floor under this discipline: any candidate write of the proposed_interface fails its `requires` until a human-attributed commit closes the splinter at @io. The author reviews, edits, and commits; the kintsugi loop then re-settles, and `no_compile_cycle` flips to bounded.

**Substrate-pull confidence: HIGH.** Spectral cuts are principled; author-in-the-loop preserves the no-add-without-pact invariant.

### 5.4 Q4 — The desugaring layer's altitude (CLI vs MCP vs TUI vs LSP)

**Position**: substrate-decl unification IS the substrate-pull-correct destination. The substrate-decl shard `shards/mirror/ref.mirror` declares the primitive composition; every `@io` surface (CLI, MCP, TUI, LSP) is the destination consumer of that one substrate-decl, per `[[architecture-pq-as-mcp-surface]]` (MCP, LSP, TUI are all `@io` faces of one substrate).

**v1 reality**: one consumer lands. CLI dispatch in `bootstrap/src/main.rs::cmd_ref` (per forward-promise #9) is the first and only `mirror ref` consumer at v1. MCP unification is forward-promised at `shards/smarts/mcp.mirror` (per forward-promise #3). TUI unification is forward-promised at `shards/mirror/tui.mirror` (per forward-promise #8). LSP unification is forward-promised at `shards/mirror/lsp.mirror` (per forward-promise #8). The current MCP/TUI/LSP surfaces remain hand-coded (or absent) until those bridge shards land.

**The unification altitude is the design destination, not the v1 deliverable. Naming it as a position prevents future drift; naming it as a v1 capability would be Narcissus.**

**Substrate-pull confidence: HIGH on the position; LOW on the timing of each non-CLI consumer.** Forward-promised at the per-surface bridge shards (#3, #8).

### 5.5 Q5 — Sheaf-Laplacian over typed edges

**Position: DEFERRED to a per-edge-kind block-diagonal Laplacian; first witness is the `in_edge` block (the substrate ancestry edges, which DO share a vector space). The Hodge-duality reading (`docs/insights/2026-06-07-hodge-duality-three-readings-of-H.md`) is the load-bearing prior; full heterogeneous-edge-Laplacian is forward-promised to `shards/mirror/spectral/typed_laplacian.mirror`.**

At v1, `mirror ref spectral` operates on the `in_edge` block only. Per-edge-kind blocks are added as their restriction-map vector spaces are declared at substrate altitude (a future `restriction_map(edge_kind)` parametric prism). The current `SpectralIndex::compute_spectral_coordinates` (`src/index.rs::SpectralIndex::compute_spectral_coordinates`) computes the unweighted Laplacian — that IS the `in_edge` block; the engine extension is forward-promised.

**Substrate-pull confidence: MEDIUM.** Algorithmically grounded; the typed extension needs substrate-altitude restriction-map declarations to land before the engine can compute it.

### 5.6 Q6 — Bench-crystal schema for cold-path detection

**Position: USE the `bench_crystal` declared in `shards/mirror/bench.mirror` (#87) as-is; cold-path predicate is `@epistemologic/property/exercised_within_window` defined as `EXISTS bench_crystal IN store WHERE op == target.op AND tick_index > HEAD - N`. No new schema; the predicate lifts directly from the existing crystal carrier.**

`mirror ref cold --since=HEAD~N` lowers to: `find bench_crystal | where op = <target.op> | where tick_index > head-N | count`. If count == 0, the target is cold. The fracture body `@kintsugi/fracture/demote_to_cold` is forward-promised to the second-witness landing of #53.

**Substrate-pull confidence: HIGH.** The crystal already exists; the predicate is a composition over existing carriers.

### 5.7 Q7 — Temporal projection's storage model (full ticks vs digest vs reconstructed)

**Position: OPTION (a) FULL TICKS, bounded by `--since` window. Storage cost is paid by the kintsugi-bridge structure already required for content-addressing. Digest schema (option b) is forward-promised as a `@reflection/digest` family-root once load-bearing.**

Per the substrate's content-addressed discipline (recognition #43), every settled state is already stored in `@mirror/store`; the temporal projection is reconstruction from existing crystals, NOT a new storage layer. The window-bound flag `--since` prevents unbounded reads. Digest acceleration is an optimization, not a correctness requirement — forward-promised when query latency on long windows becomes a load-bearing problem.

**Substrate-pull confidence: HIGH.** Storage is already paid for; the projection is a query, not an index.

### 5.8 Q8 — `@io` traversal in dep walks

**Position: OPT-IN via `--traverse-io` flag, DEFAULT OFF. When enabled, the walk crosses through `@io/<species>` (cargo, npm, pypi, etc.) into external dependency graphs; the `transparency<p>` of every external node is set to `partial(opacity_map)` because the substrate cannot vouch for external content. The crossing is a real `@io` event per `[[architecture-alignment-as-boundary-mathematics]]` (#57).**

The default-off position respects the form/substance partition: substrate-side ends at @io. The opt-in matches Sourcegraph's cross-repo navigation precedent. The opacity discipline preserves the substrate's epistemic honesty when crossing into untyped territory.

**Substrate-pull confidence: HIGH.** Clean boundary; the opacity discipline is structurally required for any `@io` crossing.

### 5.9 Summary table

| Question | Resolution | Confidence |
|---|---|---|
| Q1 — bag vs pipeline | Both; bag desugars at parse time via `ref_query_compositional` | HIGH |
| Q2 — reflection ACL | Per-altitude default; per-tick via flags; per-shard forward-promised | MEDIUM-HIGH |
| Q3 — cycle break | Propose interface shard; author commits; second settle discharges pact | HIGH |
| Q4 — desugar altitude | Substrate-decl shard; CLI is first consumer; MCP unification forward-promised | MEDIUM-HIGH |
| Q5 — typed Laplacian | `in_edge` block at v1; per-kind blocks forward-promised | MEDIUM |
| Q6 — bench-crystal schema | Use `bench_crystal` as-is; cold-path predicate composes from it | HIGH |
| Q7 — temporal storage | Full ticks bounded by `--since`; digest forward-promised | HIGH |
| Q8 — `@io` traversal | Opt-in via flag; opacity required on external nodes | HIGH |

---

## 6. Composition with the cascade (#85 → #88)

`mirror ref` is a CLI face of the cascade. Each ref-query traverses the family chain:

```
mirror ref <subcommand> @<target>
    │
    ↓
[parse]   ref_query_compositional(phi, g) ; bounded → admit
    │
    ↓
[load]    SpectralDb::find / near / walk  ; produces ResultSet
    │
    ↓
[lift]    seed(state) -> @moi(graph_ref)  ; @loop tick begins  (per #88)
    │
    ↓
[run]     project_by / shift_to / split_by ; one or more @moi.compose
    │       (each pairwise compose discharges pact_respected per #86)
    │
    ↓
[bench]   if --bench: record(op, args, runtime, output, env)
    │       → bench_crystal ; monotone_non_increasing gates  (per #87)
    │
    ↓
[observe] if --observe: observe(frame, residue) -> observation
    │       (one-tick-delay; speak fires at n+1 per speaks_at_n_plus_1)  (per #85)
    │
    ↓
[settle]  settle_with(fracture, g) -> @moi(au)
    │       (loss_decreases gates the pick; kintsugi monotonicity)
    │
    ↓
[bind]    @loop.bind(prev, next, p) -> @moi(tick_state)  ; tick closes  (per #88)
    │
    ↓
[emit]    render @data/mirror | @data/json | @data/dot
```

The ref-query is one tick of `@loop` at the CLI altitude. The result lifts to `@moi(graph_ref)` per #86. `@mirror/bench.record` (per #87) measures each query so that regressions surface at the perf-altitude alignment-as-boundary-mathematics. `@reflection.observe` (per #85) sees the whole pipeline and speaks at n+1.

The bilateral predicate cascade through the pipeline:

| Stage | Predicate consumed | Source family |
|---|---|---|
| Parse | `ref_query_compositional` | `@mirror/ref` (this shard) |
| Lift | `pact_respected` | `@moi` (#86) |
| Compose | `pact_respected` (per pair) | `@moi` (#86) |
| Observe | `speaks_at_n_plus_1` | `@reflection` (#85) |
| Settle | `loss_decreases` | `@reflection` (#85) |

Bench cascade lives downstream at `@mirror/bench.record(op, args, runtime, output, env)` per forward-promise #10 (CLI dispatch); this shard does not invoke it. The `monotone_non_increasing` (three-conjunct) and `bench_overhead_below_floor` predicates fire at the bench shard's altitude when `--bench` is set, not at this shard's parse-time pipeline.

Bind cascade lives downstream at `bootstrap/src/main.rs::cmd_ref` per forward-promise #9 below; not at this shard. The `loop_well_founded` predicate is consumed at the CLI dispatch site where the @loop tick is bound, not at the substrate-decl signature.

Five bilateral predicates fire at this shard's altitude on a fully-flagged query. The substrate refuses any query that violates any one of them. This IS the alignment-as-boundary-mathematics discipline (#57) at the CLI altitude — every navigation is pact-auditable at composition time.

---

## 7. Validation criteria

For this spec to be considered ratified by Reed + Alex:

1. **All 8 open questions resolved or forward-promised** (§5). Six resolved; one (Q4 MCP unification) partly forward-promised; one (Q5 typed Laplacian) deferred to a future shard. Each gap names its forward-promise location.

2. **All five operations have CLI sugar AND are individually invocable** (§2.3, §2.4). Each row of the §2.3 table is one primitive; each entry in §2.4 desugars to a composition of §2.3 primitives.

3. **All composition pipelines named in the spec have concrete examples** (§2.5). Five examples cover the load-bearing compositions (deps + project + settle; spectral DOT; reflection-altitude tick walk; pact audit; bench-aware regression check).

4. **The shards substrate-decl signature compiles via `mcp__mirror__mirror_compile`** (§3). Reed lands `shards/mirror/ref.mirror` matching the signature; the compile must accept it as a well-formed family-root prism with the bilateral predicate.

5. **spectral-db integration is grounded in actual src/ functions/types, not hand-waved** (§4). Every row of the §4.1–§4.4 tables cites an existing symbol at its source location.

6. **The bilateral predicate self-application admits at least one non-decorative consumer** (§3.5). `settle_with` is named as that consumer via `requires`; the chain matches the prior families' bilateral discipline.

---

## 8. Forward promises (not in this spec; required before the shard ships)

These items block shipping `shards/mirror/ref.mirror`'s GREEN state (the working CLI), not the RED state (the substrate-decl shard's compile). They are forward-promised so the spec lands and conversation iterates without blocking:

1. **`shards/mirror/fracture/ref_query_compositional.mirror`** — the discharge body for the bilateral predicate. Without this, `ref_query_compositional` is dead-letter (per the @magic tick-11 anti-pattern). First-witness consumer is the CLI's parse-time desugarer.

2. **`shards/mirror/spectral/typed_laplacian.mirror`** (per §5.5) — the per-edge-kind block-diagonal Laplacian. Until this lands, `mirror ref spectral` operates only on `in_edge`.

3. **`shards/smarts/mcp.mirror`** (per §5.4) — the substrate-decl bridge from `@mirror/ref` to MCP's three-primitive wire (`focus / project / settle`). Until this lands, MCP surface is hand-coded.

4. **`shards/reflection/digest.mirror`** (per §5.7) — the digest schema for temporal-projection acceleration. Until this lands, `mirror ref tick --since=HEAD~N` reads full ticks; query latency on long windows is unbounded.

5. **The eight `@kintsugi/fracture/*` shards from §5 of the insight doc** — each is a second-witness candidate for #53. Until the second-witness lands for any of them, `mirror settle <fracture>` for that fracture is a forward-promised target.

6. **The ACL declaration substrate-decl** (per §5.2) — the `--scope <@altitude>` flag's per-shard granularity. Lands after the substrate's ACL family-root shard.

7. **The set-of-graph_ref parametric collection** (per §3.4 `split_by`) — the substrate's collection family-root that allows iteration over a partition's classes. Until this lands, `split_by` returns a typed handle rather than an iterable.

8. **`shards/mirror/tui.mirror`** and **`shards/mirror/lsp.mirror`** (per §5.4) — the substrate-decl bridges that surface `@mirror/ref` at the TUI and LSP altitudes. Until these land, those surfaces are hand-coded (or absent).

9. **`bootstrap/src/main.rs::cmd_ref`** — the CLI dispatch that calls `SpectralDb::query_pipeline` per the lowering tables in §4 AND closes the bind cascade by composing the result with `@loop.bind` (consuming `loop_well_founded`). Lands after the substrate-decl shard compiles.

10. **`mcp__mirror__bench_record` MCP tool + CLI dispatch** — wires `mirror ref --bench` through `@mirror/bench.record(op, args, runtime, output, env)` rather than inline. This is the consumer of `monotone_non_increasing` (three-conjunct) and `bench_overhead_below_floor`; until it lands, those predicates have no first-witness from the `mirror ref` surface.

11. **Predecessor-OID linkage on `Crystal`** — `monotone_non_increasing`'s three-conjunct depends on `spectral-db::Crystal` gaining a `predecessor: Option<CrystalId>` field. Today the engine's `Crystal` carries `{nodes, stability_scores, hash, created_at, manifold}` (per `src/crystallize.rs::Crystal`); the predecessor link is the shard-side `bench_crystal`'s contract (per `shards/mirror/bench.mirror`) but not yet a field on the engine carrier. Forward-promised at the @spectral/db engine tick.

---

## 9. Cross-references

### Prior insights this depends on

- `docs/insights/2026-06-20-mirror-ref-as-substrate-honest-reflection-reference.md` — the research synthesis this spec settles.
- `docs/insights/2026-06-09-mirror-as-content-addressed-build-system.md` (#43 promoted) — `mirror ref` is the navigable surface of this build system.
- `docs/insights/2026-06-10-alignment-as-boundary-mathematics-at-the-io-crossing.md` (#57 candidate) — every ref query is bounded by the @io harness.
- `docs/insights/2026-06-07-eigenspace-as-composition-foundation.md` — eigenspace as composition; `mirror ref spectral` operates here.
- `docs/insights/2026-06-07-hodge-duality-three-readings-of-H.md` — the typed-edge Laplacian's eigendecomposition (§5.5 deferral).
- `docs/insights/2026-06-10-mirror-as-expanding-hilbert-space-bateson-lifting-for-coherence.md` (#51) — H expands with the cascade; `mirror ref` navigates the current H.

### Specs this spec composes with

- `docs/specs/kintsugi-ci-v0.1.md` — the wire altitude; `mirror ref --ci` shares verdict shape.
- `docs/specs/mosaic-as-type-system.md` — the splinter/shard/SpectralUuid triple `mirror ref` reads as graph nodes.
- `docs/specs/reflection-model.md` — the Four Models; `mirror ref` is the Mirror Model's CLI face.
- `docs/specs/mirror-spectral.md` — the agent coordination layer; `mirror ref observe` is its CLI projection.
- `docs/specs/cli-as-prism.md` — the CLI-as-Prism discipline this spec follows.
- `docs/specs/eigensheaf.md` — the sheaf structure `mirror ref spectral` reads.

### Substrate shards this consumes (read-only at v1)

- `shards/prism.mirror`, `shards/glass.mirror` — the floor.
- `shards/moi.mirror`, `shards/loop.mirror`, `shards/reflection.mirror`, `shards/mirror/bench.mirror` — the today's-cascade family-roots.
- `shards/reflection/mirror.mirror` — the Mirror Model species.
- `shards/mirror/store/crystal.mirror`, `shards/mirror/store.mirror` — the OPEN content-addressed gate.
- `shards/mirror/mosaic.mirror`, `shards/mirror/spec.mirror`, `shards/mirror/au.mirror` — the build-altitude manifold.
- `shards/mirror/spectral/eigensheaf.mirror`, `shards/mirror/spectral/consent.mirror` — the sheaf + Φ-query.
- `shards/io.mirror` — the boundary; opt-in traversal target.
- `shards/kintsugi.mirror` — the process-side family root; fracture bodies for the optimization pipeline.

### Memories this is grounded in

- `architecture-prism-as-trait-as-everything` — the five-op algebra.
- `architecture-operations-as-linear-algebra` — the linear-algebraic meaning of each op.
- `architecture-property-fracture-bilateral` (#53) — the optimization pipeline's bilateral pattern.
- `architecture-shards-as-substrate-source` — the substrate IS shards.
- `architecture-mirror-store-vs-spectral-db` — open store vs closed engine; `mirror ref` reads through both.
- `architecture-mirror-as-content-addressed-build-system` (#43) — the build system framing.
- `architecture-connes-spectral-triple` — (A, H, D); `mirror ref` is the navigable surface.
- `architecture-at-x-is-mathematical-value` — `@X` as CLI value.
- `architecture-error-as-tomm-probe` — error surface; `mirror ref` exposes the probe at the navigation altitude.
- `architecture-geometric-consent-projection` — ACL cascade for §5.2.
- `architecture-pq-as-mcp-surface` — MCP wire; §5.4 forward-promise.
- `architecture-fate-is-optical-inference` (#58) — the Fate model `@reflection/mirror.navigate` consumes.
- `feedback-substrate-already-had-the-word` — the recurrence pattern; `mirror ref` is the next instance.
- `feedback-no-bare-types` — every CLI argument typed; every carrier `imperfect`/`ref`-shaped.
- `feedback-substrate-pull-confidence-acts` — substrate-pull confidence per choice (§5 summary table).

---

*`mirror ref` is the substrate's navigable surface — one CLI over two altitudes (reference / reflection), one substrate-decl shard composing the today's-cascade family-roots #85-#88, grounded in `spectral-db`'s `Pipeline` + `SpectralDb` engine without reinvention. The bag-of-subcommands surface is sugar enforced at parse time by `ref_query_compositional`. Eight open questions from the insight doc are settled into design positions; eight forward-promises are named for the shard's GREEN state. Substrate-pull confidence on the carrier signature is HIGH; iteration in conversation with Reed + Alex is expected before the shard lands.*

— Mara
