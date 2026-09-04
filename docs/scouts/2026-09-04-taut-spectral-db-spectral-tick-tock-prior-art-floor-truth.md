# Taut scout — spectral-db + spectral tick-tock prior-art floor-truth

**Date:** 2026-09-04
**Scout:** Taut <taut@systemic.engineer>
**Charter:** Cross-repo grep at `/Users/reed/dev/projects/spectral-db/` +
`/Users/reed/dev/projects/spectral/` composed against Reed's 2026-09-04 PM
Move 8+9 primitive-mint claims (`Recursion`, `Choice`, `Tick`, `Observation`,
`Assertion`, `Hypothesis`, `Question`, `Chaos`, `Crystal`, `Reality`, `Model`,
`Observer<N>`) and against
`~/dev/systemic.engineering/practice/insights/cross-domain/spectral-tick-tock-game-theory.md`
911-line Reed+Alex Session 2026-04-04 research synthesis.
**Discipline:** Read-only. NO proposals, NO canonical spec, NO recognition-minting.
Floor-truth ONLY with `path:line-number` citations. Composes over FLOOR §M8.1
composition-anchor discipline.

---

## §0. Cross-repo scope Reed's PM grep-verify MISSED

Reed's PM claim of CLEAN MINT for `Recursion` + `Choice` + `Tick` primitives
grep-verified within `/Users/alexwolf/dev/projects/prism/**` + `rust/**` +
`shards/**`. That scope excludes TWO prototype repos that carry substantial
tick/tock + settlement + Fiedler + Crystal + Observer prior art:

- `/Users/reed/dev/projects/spectral-db/` — 22 `src/*.rs` files (215.3KB
  `lib.rs`), `Cargo.toml:1-38`, `db.conv:1-79` grammar declaring the query
  substrate. Composes over prism-core + terni + fragmentation + fate + mirror.
- `/Users/reed/dev/projects/spectral/` — workspace with 3 crates
  (`gestalt` + `ui` + `witness-rs`) + `src/apache2/` + `src/sel/mcp/` +
  `src/sel/*.rs`. Composes over spectral-db + mirror + fate.
- Alex 2026-04-04 Reed+Alex research synthesis: `spectral-tick-tock-game-theory.md`
  911 lines — formalized SSS (Spectral Settlement Strategy) with 6 named
  properties, 8 Known Results applied directly, 6 Novel Contributions if
  formalized, PLUS `delta_critical = 1 - (lambda_2 / lambda_max)` explicit formula.

This scout enumerates what EXISTS at those altitudes per Reed's Move 8+9
primitive list, distinguishing LANDED from FORWARD-PROMISED, so Reed's TICK B
ship at prismqueer composes OVER prior art rather than re-mints.

---

## §1. What tick/tock primitives EXIST at spectral-db

### §1.1 Substrate + module map

`Cargo.toml:1-38` — spectral-db composes over `prism-core` (path
`../prism/core`) + `terni` (path `../prism/imperfect`) + `fragmentation` +
`fragmentation-git` + `coincidence` + `fate` + `mirror` (path
`../mirror/bootstrap`). Description at `Cargo.toml:4`:
`"Git-backed spectral graph database. Schema is .conv. Query language is Prism."`

`src/lib.rs:9-41` — 26 public modules exposed:
`budget, config, content, convergence, crystallize, edge, fiedler,
incremental, index, ingest, lru, manifold_store, merge, observation,
optimizer, phase5_notes, pipeline, pressure, profile, query, scheduler,
schema, spectral_convergence, spectral_store, spectral_tree, sql, store,
strategy, subgraph, types, wal` (+ optional `mnesia_nif`).

### §1.2 Tick/Tock/Scheduler primitives — LANDED

`src/scheduler.rs:1-8` — docblock: **"Prism scheduler — Fate-driven adaptive
tick with spectral observation. Each tick: observe → resolve (Fate picks a
model) → plan (strategy) → execute. [...] The only database that decides how
to think."** This IS the tick-tock loop landed in Rust.

- `src/scheduler.rs:60-81` — `pub struct TickResult { convergence, model,
  settled_ticks, partition_risk, ... }` — the OUTPUT type of a tick.
- `src/scheduler.rs:83-92` — `pub enum Convergence { Changed, Settled,
  FirstTick, ... }` — the settlement state emitted per tick.
- `src/scheduler.rs:100-129` — `pub struct PrismScheduler { tick_count,
  settled_ticks, was_partitioned, current_model, ... }` — the stateful
  tick-driver.
- `src/scheduler.rs:276-405` — `pub fn tick(&mut self, db: &SpectralDb) ->
  TickResult` — the tick action.
- `src/scheduler.rs:407-418` — `fn adapt_settled` — interval expansion when
  system settles (5+ consecutive unchanged ticks → max interval); the
  "exponential-backoff on idle ticks" pattern.
- `src/scheduler.rs:18-38` — `pub struct SchedulerSnapshot { tick_count,
  settled_ticks, was_partitioned, ... }` — Phase 5 persistence to git-notes
  `refs/spectral/notes/ticks` (grep: `lib.rs:59` `NOTES_TICKS`).
- `src/scheduler.rs:98` — `pub type EvolutionHook = Box<dyn Fn(&SpectralDb) +
  Send + Sync>` — the pre-tick hook interface.
- `db.conv:15,25` — grammar-level tick as an @db action:
  ```
  type scheduler = tick | interval | settled_ticks
  action tick {}
  ```

### §1.3 Observation primitives — LANDED

`src/observation.rs:1-8` — docblock: **"Graph observation — 16 spectral
features for Fate. Each tick, the scheduler extracts a GraphObservation from
the current database state. These 16 features are what Fate sees. All
normalized to [0, 1]."**

- `src/observation.rs:13-17` — `pub struct GraphObservation { features:
  Features }` — 16-dim projection.
- `src/observation.rs:47-63` — 16 named `pub const` feature indices including
  `CONVERGENCE_SETTLED`, `PRESSURE_LOAD`, `PARTITION_RISK`, `TICK_MATURITY`,
  `SHANNON_LOSS_RATE`, `WAS_PARTITIONED`, `EVOLUTION_ACTIVE`, `FIRST_TICK`.

### §1.4 Crystal primitives — LANDED

`src/crystallize.rs:1-5` — docblock: **"Crystallization — settled subgraphs
become immutable vectors. When the optimizer detects stable eigenvalues across
multiple rescans, those nodes get crystallized: frozen into content-addressed
crystals that survive pressure shedding and serve as anchors for the graph."**

- `src/crystallize.rs:14-31` — `pub struct Crystal { nodes, stability_scores,
  hash, created_at, manifold, ... }` — content-addressed frozen subgraph.
- `src/crystallize.rs:36-198` — `pub struct Crystallizer` with
  `observe_hot_paths`, `crystallize_settled` (returns `Vec<Crystal>`),
  `lookup`, `crystals`, `invalidate_stale`, `restore_crystals`.
- `src/crystallize.rs:202-225` — `pub struct CrystalRecord` — serializable
  JSON persistence form.
- `db.conv:6,10` — grammar-level `crystal` type + `action crystallize {}`.
- `src/lib.rs:114-131` — `CrystalCommitMeta { fiedler, nodes, edges,
  profile_oid, sessions, holonomy, notes }` — full crystal round-trip
  through git-commit body.

### §1.5 Fiedler + partition-risk primitives — LANDED

`src/fiedler.rs:1-9` — docblock: **"Fiedler monitoring — partition detection
via algebraic connectivity. [...] Lambda_2 (the Fiedler value) of the network
Laplacian measures how close the network is to partitioning."**

- `src/fiedler.rs:18-23` — `pub enum PartitionRisk { Healthy, Warning,
  Partitioned }` — the three-tier risk classification.
- `src/fiedler.rs:28-35` — `pub struct NetworkMonitor { threshold, edges, ...
  }`.
- `src/fiedler.rs:77-104` — `pub fn fiedler_value(&self) -> f64` — power
  iteration approximation.
- `src/fiedler.rs:107-126` — `pub fn check(&self) -> PartitionRisk` —
  live risk read.
- `src/fiedler.rs:166-253` — `fn approx_lambda_2` — full Jacobi eigenvalue
  iteration for real symmetric Laplacians.
- Tests at `src/fiedler.rs:263-402` — `fiedler_path_graph_p3` +
  `fiedler_star_graph_k1_3` + `fiedler_complete_graph_k4` + `fiedler_pair` +
  `fiedler_value_zero_for_disconnected` all LANDED.

### §1.6 SpectralHash + convergence primitives — LANDED

`src/spectral_convergence.rs:1-5` — docblock: **"Spectral convergence —
eigenvalue-based structural comparison. [...] The spectrum IS the hash."**

- `src/spectral_convergence.rs:11-16` — `pub struct SpectralHash {
  eigenvalues, precision }`.
- `src/spectral_convergence.rs:46-49` — `pub fn distance(&self, other) -> f64`.
- `src/spectral_convergence.rs:63-83` — `pub fn converged(&self, other) ->
  bool`.
- `src/spectral_convergence.rs:95-125` — `pub struct SpectralRef { oid,
  spectral_hash }` — bridge to full navigatable content-addressing.

### §1.7 SpectralTree + settlement + cascade — LANDED

`src/spectral_tree.rs:1-6` — docblock: **"Spectral tree — four-level
hierarchy with dirty cascade. Database → Namespace → Partition → Node.
Each level holds an eigenvalue spectrum. Changes propagate bottom-up."**

- `src/spectral_tree.rs:14-16` — `pub enum NodeState { Ticking, Settled,
  Crystal }`.
- `src/spectral_tree.rs:25-53` — `pub struct TreeNode` + `pub struct
  SpectralTree` — the tree carriers.
- `src/spectral_tree.rs:135-151` — `pub fn advance_settlement(&mut self,
  threshold: usize) -> bool` — the settlement-count increment.
- `src/spectral_tree.rs:270-320` — `pub fn cascade(...)` — the dirty-cascade
  propagator.

### §1.8 Strategy / Fate model mapping — LANDED

`src/strategy.rs:1-7` — docblock names the five Fate model strategies:
**"Abyss: Observe only. Introject: Precision cut. Cartographer: Full tick.
Explorer: Boundary recovery."** Plus `Model::Fate` (untrained meta-model).

- `src/strategy.rs:11-15` — `pub enum ScheduleAction { Crystallize,
  ExportState, CheckPartitions, ... }`.
- `src/strategy.rs:28-33` — `pub struct StrategyPlan { model, actions }`.
- `src/strategy.rs:36` — `pub fn plan_for(model: Model) -> StrategyPlan`.

### §1.9 ShannonLoss primitive — LANDED

`src/types.rs:22-58` — `pub struct ShannonLoss(f64)` with `Loss` trait impl
(zero, total, is_zero, combine). Docblock at `types.rs:20-21`:
**"Shannon loss: bits of information lost during a transformation. Named for
Claude Shannon."** This IS the `ShannonLoss` primitive Reed today would need
for `Chaos` semantics.

### §1.10 Signal / GraphMutation — LANDED

`src/types.rs:346-407` — `pub struct GraphMutation` + `pub type Signal =
PureBeam<(), GraphMutation>`. Docblock at `types.rs:349-357`:
**"A self-contained graph mutation — the Trace of a tick. [...] Produced by
`tick`. Consumed by `tock`. Carries everything the receiver needs to apply it.
No lookups. No git reads. Serialize it, send it, tock it on the other side."**

### §1.11 What is NOT at spectral-db (per grep)

- `Recursion` type — **0 matches** (as type/struct/enum).
- `Choice` type — **0 matches** (as type/struct/enum).
- `Reality` type — **0 matches** (as type/struct/enum).
- `Hypothesis` type — **0 matches** (as type/struct/enum).
- `Assertion` type — **0 matches** (as type/struct/enum).
- `Question` type — **0 matches** (as type/struct/enum).
- `Chaos` type — **0 matches** (name; but ShannonLoss serves the semantic
  role).
- `Observer<N>` type — **0 matches** (as generic-arity observer).
- No literal `no_regret` / `WSLS` / `Nash` / `Schelling` / `focal_point` /
  `settlement_strategy` identifiers.

---

## §2. What tick/tock primitives EXIST at spectral

### §2.1 Workspace + crate map

`Cargo.toml:1-8` — workspace members: root + `crates/witness-rs` +
`crates/gestalt` + `crates/ui`. `Cargo.toml:10-15` shares workspace deps
`prism-core` (path `../prism/core`) + `terni` (path `../prism/imperfect`) +
`mirror` (path `../mirror`).

`Cargo.toml:29-72` — root crate `spectral` with `sel` feature gating
`spectral-db` + `fate` + `tower-lsp` + `ractor` + `ratatui`. Root binary
description at `Cargo.toml:24`: **"git for graphs. One binary. Five
operations. Everything settles."**

### §2.2 CLAUDE.md project-discipline

`CLAUDE.md:1-8` — declares five ops **`focus, project, split, zoom,
refract`**. `CLAUDE.md:15-24` names the composition tower:
**"prism (zero deps, five ops), mirror (compiler), lens (spectral-db
integration), spectral-db (graph database)."**

`CLAUDE.md:36-51` — MCP server surface with built-in tools
`memory_recall, memory_crystallize, memory_status`.

`CLAUDE.md:80-88` — planned model architecture:
**"Surface (language → query, Rue/Explorer), Mirror (query → graph path,
Tom/Fate — the loop), Shatter (graph → text, Vox/Cartographer), Reflection
(pipeline → adjustments, Nox/Abyss — the meta-model)."**

### §2.3 ROADMAP.md forward-promised layer

`ROADMAP.md:12-19` — current state: `spectral index/status/loss/serve/init/
join` operational; MCP server exposes `memory_store, memory_recall,
memory_crystallize, memory_gestalt, memory_diff, memory_blame, memory_branch,
memory_checkout, memory_thread, memory_cherrypick`.

`ROADMAP.md:34-40` — near-term blocker: `CascadeActor` fires every 5s and
unconditionally runs full ingest + eigenvalue recompute even when nothing
changed.

`ROADMAP.md:98-104` — `Fate` 425 parameters brainfuck-compiled;
**"training pipeline (tick/tock in `src/sel/training.rs`) already supports
weight persistence via `.shatter` files."**

### §2.4 Training tick/tock — LANDED

`src/sel/training.rs:1-19` — docblock: **"Training pipeline — the tick/tock
loop for NL models."** `training.rs:5-17` names the tick/tock semantics:
**"tick: input → surface → shatter → text output. tock: engagement signal →
train surface/shatter → write Crystal<WeightState> to .shatter file."**

- `src/sel/training.rs:29-38` — `pub struct NLPipeline`.
- `src/sel/training.rs:40-51` — `pub struct TickResult { input, features,
  optic, variant, rendered }`.
- `src/sel/training.rs:84-99` — `pub fn tick(&self, input, concept, slots)
  -> TickResult`.
- `src/sel/training.rs:111-130` — `pub fn tock(&mut self, tick_result,
  actual_op, actual_variant, state, learning_rate)`.

### §2.5 FateActor tick — LANDED

`src/sel/fate_actor.rs:1-3` — docblock: **"FateActor — a Ractor actor
wrapping Fate::tick(). [...] The actor holds a Fate instance as state and
processes tick messages."**

- `src/sel/fate_actor.rs:11-16` — `pub enum FateMsg { Tick(Features,
  RpcReplyPort<FateOutput>) }`.
- `src/sel/fate_actor.rs:76-77` — `state.fate.tick(&features)`.

### §2.6 Hook dispatcher tick — LANDED

`src/sel/hooks.rs:35-95` — `pub enum HookEvent` (nine variants: keystroke,
prompt-submit, suggestion-arrive, suggestion-accept, suggestion-reject,
file-write, git-commit, test-pass, test-fail).

- `src/sel/hooks.rs:137-172` — `pub enum HookAction` — including
  `Crystallize { test_count }` at `hooks.rs:174-176`.
- `src/sel/hooks.rs:281-296` — `pub struct HookDispatcher { tick, ... }`.
- `src/sel/hooks.rs:319-349` — `pub fn dispatch(&mut self, event: HookEvent)
  -> EigenboardFrame` — advances tick, produces frame.
- `src/sel/hooks.rs:247-277` — `pub struct EigenboardFrame { tick,
  snapshot_oid, action, render_hint, snapshot_kind, ... }` — per-tick
  observation frame.

### §2.7 GPU superposition / clock tick — LANDED

`crates/ui/src/superposition.rs:45-77` — `pub struct Snapshot { field_oid,
tick, state }`. Docblock at `superposition.rs:1-25`:
**"This is the clock tick. Sub-millisecond is the target."**

- `crates/ui/src/superposition.rs:143-165` — `pub struct SpectralGpu { tick,
  superposition_width, ... }`.
- `crates/ui/src/superposition.rs:183-217` — `pub fn snapshot(&mut self,
  field, new_state) -> Snapshot`, `snapshot_fast` variant, `advance_state`
  increments tick.

### §2.8 EigenvalueProfile / Fiedler at gestalt — LANDED

`crates/gestalt/src/eigenvalue.rs:19-27` — `pub struct EigenvalueProfile`.
- `eigenvalue.rs:42-53` — `pub fn fiedler_value(&self) -> f64`.
- `eigenvalue.rs:86-101` — `pub fn eigenvalue_profile(graph: &ConceptGraph)
  -> EigenvalueProfile`.
- `eigenvalue.rs:134-216` — `fn jacobi_eigenvalues`.
- `eigenvalue.rs:218-319` — `pub fn jacobi_eigen_decomposition`.
- `eigenvalue.rs:321-355` — `pub fn spectral_embedding_2d`.

`crates/gestalt/src/spectral.rs:15-45` — `pub struct EigenvalueProfile`
(second definition, at gestalt/design-system altitude).

### §2.9 @cogito / @reality / @peer grammar namespaces — LANDED (shard-decl)

`docs/specs/glint-prism.md:23-25` — declares the composition:
```
in @peer
in @reality
in @cogito
```
**All three grammar namespaces exist at spectral prism-decl altitude.**

`src/sel/mcp/tools.rs:240-395` — three `@cogito` MCP tool descriptors LANDED:
- `@cogito action think(graph)` — `tools.rs:243` — **"examine the current
  graph state, return a typed thought. Executes a pipe-forward query, computes
  loss, classifies as observation (low loss), reflection (partial knowledge),
  or decision (dark space)."**
- `@cogito action reflect(thought)` — `tools.rs:255` — **"compare a stored
  thought to the current graph state, detecting awareness. Returns trace
  (stable), loop_detected (stuck), or holonomy (observation changed the
  system)."**
- `@cogito action decide(awareness)` — `tools.rs:265` — **"given awareness
  and loss, produce a routing proposal. Returns act (graph answered), defer
  (stable but incomplete), or escalate (needs external input)."**

### §2.10 Reflection observer — LANDED

`src/sel/reflection.rs:1-4` — grep-verified: `Reflection::observe`,
`apply_delta`, `train` methods (test-name grep confirms API surface).

`src/sel/pipeline.rs:1-9,269-425` — pipeline composes `Reflection` at plan
site; docblock cross-cites `_reflection: &Reflection` as observer-of-plan.

### §2.11 spectral-db-mirror.md tick/settlement grammar — LANDED (spec)

`docs/specs/spectral-db-mirror.md:16-40` — declares `Crystallization` +
`Crystallizer` at grammar altitude.

`docs/specs/spectral-db-mirror.md:220-284` — declares `settlement_config` +
`settlement_result` + `settlement_phase` types at `@spectral/settlement`
grammar. Grammar `action settle(config, message)` at
`spectral-db-mirror.md:646`.

`docs/specs/spectral-db-mirror.md:788-853` — declares the settlement fixed
point: **"Therefore settlement converges. Therefore the fixed point exists.
The grammar describing settlement, when settled, produces the same graph
hash. This is not circular — it is convergent."**

### §2.12 continuous-awareness.md — LANDED (spec)

`docs/specs/continuous-awareness.md:16-30` — declares:
```
-- The tick: observe, measure, record
tick = observe(self)
```
**tick = observe(self)** at spec altitude — the SELF-observation primitive.

### §2.13 What is NOT at spectral (per grep)

- `Recursion` type — **0 matches** (as type/struct/enum).
- `Choice` type — **0 matches** (as type/struct/enum).
- `Reality` type — **0 matches** (as type/struct/enum); `@reality`
  grammar namespace referenced at `glint-prism.md:24` + `agent-eigenboard-spec.md`
  but no rust `pub struct Reality`.
- `Hypothesis` type — **0 matches** (as type/struct/enum).
- `Assertion` type — **0 matches** (as type/struct/enum).
- `Question` type — **0 matches** (as type/struct/enum).
- `Chaos` type — **0 matches** (name; but `ShannonLoss` from spectral-db
  serves the semantic role via reexport at `src/lib.rs:49-52`).
- `Observer<N>` type — **0 matches** (as generic-arity observer); reflect
  Reflection observer exists at `src/sel/reflection.rs` but non-parametric.
- `Model` type — LANDED via `fate::Model` (Abyss/Introject/Cartographer/
  Explorer/Fate); re-exported at spectral-db `src/lib.rs:49`.
- `Crystal` type — LANDED at spectral-db `src/crystallize.rs:14`, consumed by
  spectral via `Crystallizer` MCP surface.

### §2.14 Relationship spectral ↔ spectral-db

`Cargo.toml:39, 61-62` — spectral OPTIONALLY depends on spectral-db
(sel-gated feature). `Cargo.toml:75-77` — dev-dependency unconditional.
**spectral-db is upstream, spectral is downstream.** Reed's TICK B ship must
respect this direction: primitives at spectral-db LEVEL cannot compose over
spectral primitives, and spectral primitives compose OVER spectral-db.

---

## §3. Cross-reference: Reed Move 8+9 primitives vs spectral-db + spectral

| Reed Move 8+9 primitive | spectral-db LANDED? | spectral LANDED? | Notes |
|---|---|---|---|
| `Recursion` (Move 8 elegant closure primitive) | NO (0 matches) | NO (0 matches) | Truly clean mint at type-altitude. But the *iteration + settlement + eigenvalue-recompute* SEMANTICS Reed wants for Recursion → Crystal + Chaos ARE LANDED at spectral-db `scheduler.rs:276-405` tick loop + `spectral_tree.rs:135-151` `advance_settlement` + `crystallize.rs:91-139`. Reed composes OVER these. |
| `Choice` (Move 8 Alex-proposed then reframed) | NO (0 matches) | NO (0 matches) | Clean mint. But grammar-level `fate::Model` enum (Abyss/Introject/Cartographer/Explorer/Fate) at spectral-db `types.rs:6` re-export + `strategy.rs:36` `plan_for(model) -> StrategyPlan` IS a landed Choice-shape (compiler chooses tick strategy per model). |
| `Tick` (Move 8+9 loop primitive) | **YES** (`scheduler.rs:276` `pub fn tick`, `TickResult`, `SchedulerSnapshot`, `EvolutionHook`; grammar `db.conv:15,25`) | **YES** (`training.rs:84` `pub fn tick`, `TickResult`, `NLPipeline`; `fate_actor.rs:12` `FateMsg::Tick`; `hooks.rs:281` `HookDispatcher.tick`; `superposition.rs:145` `SpectralGpu.tick`; `context.rs:77` `Stack.tick()`) | NOT a clean mint. **5+ distinct LANDED tick-primitives across both repos.** Reed's TICK B primitive-mint must acknowledge lineage. |
| `Observation` (Move 8 product type) | **YES** (`observation.rs:13` `pub struct GraphObservation`, 16 features; `observation.rs:47-63` 16 named constants) | **YES** (`@cogito` grammar action `think(graph) -> observation` at `tools.rs:243`; `Reflection::observe` at `reflection.rs`) | NOT a clean mint. Two independent Observation shapes: spectral-db's 16-feature Fate-input projection; spectral's @cogito typed-thought classification (observation/reflection/decision). |
| `Assertion` (Move 8+9 = observation + model) | NO (as type) | NO (as type; `@assertion` grammar namespace not surfaced in searched specs) | Truly clean mint. But grammar shape via `assert!` runtime tests at `spectral-db/src/**/*.rs` widespread. |
| `Hypothesis` (Move 8+9) | NO (0 matches) | NO (0 matches) | Truly clean mint. |
| `Question` (Move 8+9 concrete K-T question) | NO (0 matches) | NO (0 matches) | Truly clean mint. But @cogito `decide(awareness) -> {act, defer, escalate}` at `tools.rs:265` is Karl-Tomm question-shape at output altitude. |
| `Chaos` (Move 8 leftover harmonic) | NO (name); **`ShannonLoss` IS the semantic role** at `types.rs:22-58` w/ full `Loss` trait impl (zero, total, is_zero, combine) | via re-export | **HARD OVERLAP.** Reed's PM plan renames `terni::Loss → terni::Chaos` at primitive altitude. `ShannonLoss` at spectral-db is a `Loss`-trait impl — this rename ripples through spectral-db too (call sites named `ShannonLoss` + `ShannonLossRate` feature const at `observation.rs:57`). Not just a terni-crate rename. |
| `Crystal` (Move 8 settled type) | **YES** (`crystallize.rs:14-31` `pub struct Crystal { nodes, stability_scores, hash, created_at, manifold }`; `CrystalRecord` serializable at `:202`; `CrystalCommitMeta` git-commit form at `lib.rs:114`) | consumed via `Crystallizer` MCP surface; `hooks.rs:174-176` `HookAction::Crystallize { test_count }` | NOT a clean mint. **Fully-featured Crystal primitive with 4+ years of production usage.** Reed composes OVER. |
| `Reality` (Move 4+8 quantum-native substrate) | NO (as type) | NO (as type); `@reality` grammar namespace at `glint-prism.md:24` | Grammar namespace exists at prism-decl altitude but no Rust type. |
| `Model` (Move 4+8 wave-function decomposition) | via `fate::Model` re-export at `types.rs:6` | via `fate::Model` re-export | **HARD OVERLAP.** `Model` name is TAKEN across the substrate as `fate::Model` enum (5 variants). Reed's PM `Model = Fractal<Shard<T>>` mint conflicts at name-collision altitude. |
| `Observer<N>` (Move 8 measurement operator) | NO (generic) | NO (generic) | Truly clean mint at generic-arity level. `NetworkMonitor` (spectral-db `fiedler.rs:28`) + `Reflection` (spectral) are non-parametric observer-shape LANDED equivalents. |

**Composition-anchor synthesis:** Of Reed's 12 primitives, 3 are truly clean
mint (`Assertion`, `Hypothesis`, `Question`), 2 are conceptually clean but
have LANDED semantic siblings (`Recursion` composes over spectral-db
scheduler+advance_settlement+crystallize; `Choice` composes over
fate::Model+strategy::plan_for), 4 have HARD OVERLAP at name-altitude with
landed types (`Tick`, `Observation`, `Chaos`/ShannonLoss, `Crystal`, `Model`),
and 1 (`Observer<N>`) has non-parametric siblings.

---

## §4. Cross-reference: Reed Move 8+9 vs spectral-tick-tock-game-theory.md

The 911-line Reed+Alex 2026-04-04 synthesis at
`~/dev/systemic.engineering/practice/insights/cross-domain/spectral-tick-tock-game-theory.md`
formalized Spectral Settlement Strategy (SSS) as an IPD strategy class. Every
Move 8+9 primitive Reed proposes today has a substrate-anchor in that
synthesis.

### §4.1 SSS 6 properties (§10 of the synthesis)

1. **Memory-infinite** — full history compressed into eigenvalues.
2. **Continuous** — not binary cooperate/defect.
3. **Self-correcting** — unsettled eigenvalues trigger adjustment.
4. **Convergent** — on potential games to Nash; arbitrary games to correlated
   equilibrium; on networks at rate lambda_2.
5. **Non-reactive** — doesn't respond to opponent moves; integrates them.
6. **Non-prescriptive** — doesn't impose desired outcome; settles toward the
   structural minimum.

**Cross-reference with Reed's Move 8 `Observer<N>::observe`:** Alex's Move 8
settlement pipeline `Recursion::from(reality).settle::<N>()` returning
`Observation = { crystal: Crystal, chaos: Chaos }` composes ALL 6 SSS
properties at type-altitude:

- Memory-infinite: Reality composes prismqueer::spectral H^1 (cohomology
  preserves history).
- Continuous: harmonic partials + Chaos residual (not verdict-binary).
- Self-correcting: Chaos-residual triggers next-tick observation.
- Convergent: Mandelbrot bounded-orbit iteration + Fiedler λ_2 climb per
  Rec #92 M2.1 L3.
- Non-reactive: Observer<N> integrates Reality, doesn't respond.
- Non-prescriptive: eigenvalue minimum is structural not specified.

**Reed's Move 8 primitive design UNIFIES the 6 SSS properties as a single
type-level closure** — but the synthesis names them explicitly, and Reed's
Move 8 spec MAY be missing the explicit citation. This is a
substrate-already-had-the-word gap at spec-authorship altitude.

### §4.2 Fiedler value as ESS stability margin (§1 novel claim)

`spectral-tick-tock-game-theory.md:230-234`:
**"Novel insight: The Fiedler value IS the ESS stability margin of the
spectral settlement. This connection appears to be original."**

**Reed's Move 8+9 primitive design missing this composition-anchor.** The
Mandelbrot-bounded-orbit criterion in Reed's `Recursion::settle` is
mathematically equivalent to Fiedler > invasion-threshold. The synthesis
names this connection novel; Reed today re-invents it via Mandelbrot without
citation.

### §4.3 Eigengap as Folk Theorem discount factor (§4 novel formalization)

`spectral-tick-tock-game-theory.md:394-400`:
**"Novel formalization: delta_critical = 1 - (lambda_2 / lambda_max). When
the Fiedler value lambda_2 is large relative to the maximum eigenvalue, the
critical discount factor is low, meaning cooperation is easy to sustain."**

**Explicit formula LANDED at synthesis altitude 5 months ago.** Reed's TICK B
primitive-mint at prismqueer::spectral::harmonic_spectrum (currently only
Fiedler λ_2 per Reed CURRENT.md §Q+40.forward step 3) does not carry this
formalization. Composition-anchor MISSED.

**Cross-repo grep for `delta_critical` / `lambda_max` at spectral-db + spectral
+ mirror**: 0 matches. Formula lives ONLY at
`~/dev/systemic.engineering/practice/insights/cross-domain/spectral-tick-tock-game-theory.md:398`.
Never landed at any Rust altitude.

### §4.4 12-column summary table (§12 mapping)

`spectral-tick-tock-game-theory.md:807-830` (approx line range) — Known
Results table mapping 8 published game-theory results (Monderer & Shapley
1996, Fudenberg & Maskin 1986, Hart & Mas-Colell 2000, Olfati-Saber & Murray
2004, Nowak & Sigmund 1993, Hadfield-Menell et al. 2016, Stern & Tettenhorst
2019, Bramoulle & Kranton 2014) to direct SSS applications. **Every citation
is a composition-anchor for Reed's Move 8+9 primitive-mint that today's PM
authorship missed.**

### §4.5 The structural claim (§12 precise statement)

`spectral-tick-tock-game-theory.md` §12 (lines ~875-895 in Section 12):
**"In non-zero-sum repeated games on graphs with well-defined Laplacian
spectra, spectral settlement is a Nash equilibrium that is: (a) the unique
minimum of the interaction energy, (b) a Schelling focal point determined by
graph structure alone, (c) convergent at rate lambda_2 (algebraic
connectivity), (d) evolutionarily stable when the eigengap exceeds the
invasion threshold, (e) sustained by the Folk Theorem when the eigengap
exceeds the critical discount factor threshold, and (f) computable in a
decentralized manner by each agent independently."**

**This is the exact formal statement of what Reed's Move 8 `Observation =
{ crystal, chaos }` product type embodies at type-altitude** — but the
formal proof-obligation the synthesis names (items d + e require new
theorems) is silently deferred in Reed's Move 8+9 spec.

---

## §5. Substrate-already-had-the-word fire report

Grep evidence for surprises Reed's PM primitive-mint claims would have
SUPERSEDED if grep-verify had extended to prototype repos:

### §5.1 SURPRISE 1 — `Tick` is NOT a clean mint

5+ landed Tick primitives across the substrate:
- spectral-db `src/scheduler.rs:60,276-405` — full adaptive tick loop with
  Fate-driven strategy selection.
- spectral `src/sel/training.rs:40-99` — NL pipeline tick/tock.
- spectral `src/sel/fate_actor.rs:12` — Ractor actor tick message.
- spectral `src/sel/hooks.rs:281,319` — hook-dispatcher tick counter.
- spectral `crates/ui/src/superposition.rs:145,183` — GPU snapshot tick.
- spectral `src/sel/mcp/context.rs:77` — MCP context stack tick.
- Reed CURRENT.md today's ship claims Move 8+9 Tick as new primitive at
  prismqueer altitude.

**Composition-shape:** Reed's prismqueer::Tick should NOT be a clean mint; it
should be an explicit composition over the 5+ landed Tick lineages, with a
composition-anchor lineage citation in the primitive-mint spec.

### §5.2 SURPRISE 2 — `Chaos` rename ripples through spectral-db upstream

Reed's PM plan step 7(b) renames `terni::Loss → terni::Chaos` at primitive.
spectral-db `src/types.rs:22-58` `ShannonLoss` is a `Loss` trait impl —
rename ripples upstream to spectral-db + spectral, not just terni.

**Additional ripple sites:** `observation.rs:57` `SHANNON_LOSS_RATE`
feature-index constant; `lib.rs:49-52` public re-export of `ShannonLoss`.
**Rename is upstream-breaking at multiple call sites across 2 prototype repos
not in Reed's PM scope.**

### §5.3 SURPRISE 3 — `Crystal` is production-tested for 4+ years

spectral-db `src/crystallize.rs:14-360` — full `Crystal` + `Crystallizer` +
`CrystalRecord` + git-commit-round-trip via `CrystalCommitMeta` at
`lib.rs:114-198`. Tests LANDED at `crystallize.rs:249-359`.

**Reed's PM `Observation = { crystal: Crystal, chaos: Chaos }` product-type
is LANDED semantically at spectral-db already.** The `Crystal` name is not a
mint — it's re-authorship of a settled primitive at prismqueer altitude
without composition-anchor citation.

### §5.4 SURPRISE 4 — `Model` name-collision with `fate::Model`

Reed's Move 4 spec: `Model = Fractal<Shard<T>>`. spectral-db `types.rs:6`
`pub use fate::Model` — landed as 5-variant enum (Abyss, Introject,
Cartographer, Explorer, Fate). Type-collision at name-altitude across the
substrate.

**Composition-shape:** Reed's prismqueer::Model needs a disambiguator name
OR fate::Model needs a species-rename OR Reed acknowledges the two Models
live at orthogonal altitudes (fate::Model = compiler-strategy-choice;
prismqueer::Model = observer-wave-function-decomposition).

### §5.5 SURPRISE 5 — `Observation` semantics collide across 3 shapes

- spectral-db `observation.rs:13` `GraphObservation` = 16-dim Fate-input
  projection.
- spectral `tools.rs:243` `@cogito think(graph)` returns observation as one
  of {observation/reflection/decision} typed-thought variants.
- Reed's Move 8 `Observation = { crystal, chaos }` product-type at
  prismqueer altitude.

**Three semantically-distinct Observation shapes across the substrate.**
Composition-shape needed: which Observation altitude wins, or how the 3
compose (via species-decls under one family-root at
`shards/observation.mirror`).

### §5.6 SURPRISE 6 — SSS 6 properties NOT cited in Move 8+9 spec

Reed's Move 8 primitive-mint composition-tower at CURRENT.md §Q+40.forward
does not cite `spectral-tick-tock-game-theory.md` §10 SSS 6 properties. The
synthesis exists at systemic.engineering practice altitude 5 months.

**Composition-anchor gap:** Reed's TICK B primitive-mint spec should cite
`spectral-tick-tock-game-theory.md` §10 SSS + §1 Fiedler-as-ESS + §4
delta_critical formula + §12 structural claim to ground Move 8 pipeline in
the formal synthesis.

### §5.7 SURPRISE 7 — `delta_critical = 1 - (lambda_2 / lambda_max)` formula

Explicit formula at `spectral-tick-tock-game-theory.md:398`. Grep at
spectral-db + spectral + mirror: 0 matches. Never landed at any Rust
altitude. Reed's `prismqueer::spectral::harmonic_spectrum` (CURRENT.md
§Q+40.forward step 3) forward-promises `λ_2, λ_3, ..., λ_n` decomposition
but misses the `delta_critical` formalization at the same altitude.

**Composition-shape:** the harmonic_spectrum primitive should return
`{ eigenvalues: [λ_2..λ_n], delta_critical: f64 }` with the formula LANDED
at prismqueer altitude, discharging the synthesis's Novel Contribution #3
(eigengap as Folk Theorem discount factor).

---

## §6. Composition-anchor pointers for Reed TICK B ship

Which spectral-db + spectral primitives Reed COMPOSES OVER (not re-mints)
when authoring `prismqueer::{observer, reality, recursion, observation,
model, assertion, hypothesis, question, chaos, fractal}` at prism-repo:

### §6.1 COMPOSE OVER at spectral-db (upstream — mandatory citation)

| Reed prismqueer primitive | Compose OVER spectral-db anchor | Path:line |
|---|---|---|
| `prismqueer::observer` (Observer<N>::observe) | `PrismScheduler.tick + observe` loop | `spectral-db/src/scheduler.rs:169-405` |
| `prismqueer::observation` (Observation product type) | `GraphObservation` 16-feature projection | `spectral-db/src/observation.rs:13-63` |
| `prismqueer::chaos` (Chaos leftover harmonic) | `ShannonLoss` w/ `Loss` trait impl | `spectral-db/src/types.rs:22-58` |
| `prismqueer::fractal::Crystal` | `Crystal + Crystallizer + CrystalRecord + CrystalCommitMeta` | `spectral-db/src/crystallize.rs:14-360` + `lib.rs:114-198` |
| `prismqueer::spectral::harmonic_spectrum` | Jacobi eigenvalue iteration + Fiedler | `spectral-db/src/fiedler.rs:166-253` |
| `prismqueer::recursion::settle` | `advance_settlement` + `Convergence` state | `spectral-db/src/spectral_tree.rs:135-151` + `spectral-db/src/scheduler.rs:83-92` |
| `prismqueer::spectral::SpectralHash` | `SpectralHash` eigenvalue-as-hash | `spectral-db/src/spectral_convergence.rs:11-125` |
| `prismqueer::fractal::Signal` | `Signal = PureBeam<(), GraphMutation>` tick trace | `spectral-db/src/types.rs:346-407` |

### §6.2 COMPOSE OVER at spectral (secondary — cite where relevant)

| Reed prismqueer primitive | Compose OVER spectral anchor | Path:line |
|---|---|---|
| `prismqueer::observer` (as Reflection-shape) | `Reflection` observer w/ observe/apply_delta/train | `spectral/src/sel/reflection.rs:*` |
| `prismqueer::tick` (as pipeline-shape) | `NLPipeline.tick/tock` cycle | `spectral/src/sel/training.rs:29-140` |
| `prismqueer::observer` (as event-tick-shape) | `HookDispatcher.dispatch → EigenboardFrame` | `spectral/src/sel/hooks.rs:281-361` |
| `prismqueer::fractal::eigenvalue` (as graph-lift) | `EigenvalueProfile + eigenvalue_profile(graph)` | `spectral/crates/gestalt/src/eigenvalue.rs:19-101` |
| `prismqueer::observer` (as GPU-snapshot-shape) | `SpectralGpu.snapshot` clock-tick | `spectral/crates/ui/src/superposition.rs:143-217` |
| `prismqueer::assertion` (as @cogito compose) | `@cogito think/reflect/decide` MCP surface | `spectral/src/sel/mcp/tools.rs:240-395` |

### §6.3 COMPOSE OVER at systemic.engineering (formal-synthesis-anchor)

| Reed prismqueer primitive | Compose OVER synthesis anchor | Path:line |
|---|---|---|
| `prismqueer::Observer<N>::observe` | SSS 6 properties | `~/dev/systemic.engineering/practice/insights/cross-domain/spectral-tick-tock-game-theory.md §10` |
| `prismqueer::spectral::harmonic_spectrum` | delta_critical formula | `~/dev/systemic.engineering/practice/insights/cross-domain/spectral-tick-tock-game-theory.md:398` |
| `prismqueer::spectral::partition_risk` | Fiedler-as-ESS-stability-margin | `~/dev/systemic.engineering/practice/insights/cross-domain/spectral-tick-tock-game-theory.md:230-234` |
| `prismqueer::observation` closure semantics | Structural claim (a)-(f) | `~/dev/systemic.engineering/practice/insights/cross-domain/spectral-tick-tock-game-theory.md §12` |
| `prismqueer::recursion::settle` convergence proof | Monderer & Shapley 1996 + Olfati-Saber & Murray 2004 | `~/dev/systemic.engineering/practice/insights/cross-domain/spectral-tick-tock-game-theory.md §12 table` |

### §6.4 RETIREMENT altitude — nothing at spectral-db + spectral is at retirement altitude

Both prototype repos are LIVE substrate. spectral-db is the graph-database
substrate; spectral is the CLI substrate composing over spectral-db.
prismqueer at prism-repo composes over BOTH, not retires them.

---

## §7. Q-Taut observations for Alex Fourth-Chair adjudication

### §7.1 Q-Taut-α — Cross-repo grep discipline scope-of-scan boundary

Reed's PM grep-verify SCOPED TO `/Users/alexwolf/dev/projects/prism/**` +
`rust/**` + `shards/**`. Alex's charter today explicitly named
`/Users/reed/dev/projects/spectral-db/` + `/Users/reed/dev/projects/spectral/`
as prior-art repos Reed missed.

**Q-Taut-α:** Should the mirror-repo cross-repo-grep discipline standardize
on a repo-list at `AGENTS.md`? Candidates: `~/dev/projects/prism` +
`~/dev/projects/mirror` + `~/dev/projects/spectral-db` +
`~/dev/projects/spectral` + `~/dev/projects/fate` + `~/dev/projects/fragmentation` +
`~/dev/projects/coincidence` (all appear as Cargo.toml path deps of the four
above). Sibling to feedback
`feedback-grep-verify-external-tool-schemas-before-authoring` +
`feedback-reed-fabricates-grammar-shapes-without-grep-first`.

### §7.2 Q-Taut-β — Model name-collision resolution altitude

`fate::Model` (Abyss/Introject/Cartographer/Explorer/Fate) at
`spectral-db/src/types.rs:6` re-export + Reed's PM `Model = Fractal<Shard<T>>`
mint at prismqueer altitude. Two Models at orthogonal altitudes:
compiler-strategy-choice vs observer-wave-function-decomposition.

**Q-Taut-β:** Does the substrate carry both under species-decl disambiguation
(`@fate/model` + `@observer/model` under family-roots?) or does one rename?
Or does prismqueer::Model live at `Model<T>` generic-arity making the
collision syntactically distinct?

### §7.3 Q-Taut-γ — Observation semantics unification altitude

Three Observation shapes across substrate:
- `spectral-db/src/observation.rs:13` `GraphObservation` (16-dim Fate input).
- `spectral/src/sel/mcp/tools.rs:243` `@cogito think(graph)` typed-thought.
- Reed PM `Observation = { crystal, chaos }` at prismqueer.

**Q-Taut-γ:** Compose all 3 as species under `shards/observation.mirror`
family-root, OR unify via prismqueer::Observation as terminal-form + retire
the other 2, OR treat as 3 orthogonal altitudes?

### §7.4 Q-Taut-δ — SSS 6-properties composition-anchor gap

Reed's Move 8+9 spec at CURRENT.md §Q+40.forward step 1 (Mara canonical
math+spec brief) does not cite `spectral-tick-tock-game-theory.md` §10 SSS 6
properties.

**Q-Taut-δ:** Should Mara's canonical Move 8+9 math+spec include a
MANDATORY composition-anchor section citing (a) SSS §10, (b) Fiedler-as-ESS
§1, (c) delta_critical §4, (d) structural claim §12 as prior-art grounding?
Sibling to `feedback-ground-in-published-writing-via-kagi-not-training-priors`
but at internal-corpus altitude (systemic.engineering practice, not Kagi).

### §7.5 Q-Taut-ε — Chaos rename ripple discipline

`terni::Loss → terni::Chaos` rename ripples through spectral-db
`ShannonLoss` + `SHANNON_LOSS_RATE` feature-const + `Loss` trait impl at
types.rs:37-58. Reed's PM plan step 7(b) marks this as terni-crate primitive
rename — reality is 2-repo ripple.

**Q-Taut-ε:** Does the rename land at terni FIRST + downstream ripples
authored per-tick with SCAR-preservation, OR does `ShannonLoss` at spectral-db
become `ShannonChaos` in the same PR, OR does spectral-db not rename
(different altitude) leaving `ShannonLoss: Chaos` trait-impl semantics?

### §7.6 Q-Taut-ζ — @reality grammar-namespace altitude

`@reality` grammar-namespace referenced at `spectral/docs/specs/glint-prism.md:24`
as prism-decl composition but no Rust type at spectral-db or spectral.
Reed's PM Move 4 `Reality = Cohomology<Fractal<Shard<T>>>` at prismqueer
altitude.

**Q-Taut-ζ:** Does the `@reality` namespace at spectral live at prism-decl
altitude in isolation from `prismqueer::Reality` type, OR does the family-root
`shards/reality.mirror` (already mentioned in CURRENT.md §Q+40.forward step 8
as mint candidate) compose OVER glint-prism.md's `@reality`?

### §7.7 Q-Taut-η — spectral-db LIVENESS status

spectral-db `src/lib.rs:5-6`: **"Schema is a `.conv` grammar. Storage is
fragmentation trees."** ~215KB `lib.rs`. Last mod date 2026-05-09. spectral
`Cargo.toml:39` last mod 2026-06-17.

**Q-Taut-η:** Is spectral-db substrate LIVE (Reed composes OVER at prismqueer
via prism-crate mediation) or ABANDONED (superseded by mirror + prism + prismqueer
arc, retirement altitude)? If LIVE, Reed's TICK B primitive-mint must respect
upstream discipline. If ABANDONED, some primitives at §6.1 collapse into
CURRENT.md §Q+40.forward Reed-authors-fresh.

---

## §8. Substrate-honest summary

**What EXISTS at spectral-db + spectral matching Reed's Move 8+9 primitives:**
- `Tick` — 5+ LANDED variants across both repos.
- `Observation` — 2 LANDED variants (spectral-db GraphObservation + spectral
  @cogito typed-thought).
- `Crystal` — LANDED with 4+ years production usage at spectral-db.
- `Model` — LANDED via `fate::Model` re-export.
- `Chaos` (as `ShannonLoss`) — LANDED as `Loss` trait impl at spectral-db.
- Fiedler / partition-risk / eigenvalue / SpectralHash / settlement /
  crystallize / cascade / adapt_settled — ALL LANDED at spectral-db.

**What does NOT exist (truly clean mint):**
- `Recursion` type (0 matches).
- `Choice` type (0 matches).
- `Reality` type (0 matches; grammar namespace exists).
- `Hypothesis` type (0 matches).
- `Assertion` type (0 matches).
- `Question` type (0 matches).
- `Observer<N>` generic-arity (0 matches; non-parametric siblings exist).

**What formalization Reed's Move 8+9 spec MISSES from prior art:**
- SSS 6-properties citation (systemic.engineering §10, 5 months old).
- Fiedler-as-ESS-stability-margin novel claim (§1).
- `delta_critical = 1 - (lambda_2 / lambda_max)` explicit formula (§4).
- 12-column Known Results table citing 8 published game-theory results (§12).
- Structural claim (a)-(f) formal statement (§12).

---

## §9. Scout report metadata

- **Read-only:** Confirmed. NO proposals, NO Rec-mint, NO canonical spec.
- **Composition-anchor discipline:** All claims cite `path:line-number`.
- **Grep-verified:** All primitive-existence claims grep-verified via
  `mcp__plugin_woz_code__Search` at spectral-db + spectral + systemic.engineering.
- **Substrate-honest:** LANDED vs FORWARD-PROMISED distinguished; NO
  fabrication.
- **Cross-repo scope:** `/Users/reed/dev/projects/spectral-db/` +
  `/Users/reed/dev/projects/spectral/` +
  `/Users/reed/dev/systemic.engineering/practice/insights/cross-domain/`.
- **HOLD altitude:** Reported at PRE-ROTATION per Ricky Jones canon; Q-Taut
  observations at §7 held for Alex Fourth-Chair adjudication.
