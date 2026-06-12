# Mirror IS already a content-addressed declarative build system: every primitive a build system needs is declared at substrate altitude

*2026-06-09. Recognition: Alex. Write-up: Taut. Candidate substrate-pull recognition #43. Companion to `2026-06-09-bateson-logical-type-as-substrate-primitive.md` (#42), `2026-06-09-cascade-is-deutero-learning.md`, and `2026-06-09-ashby-multi-dimensional-variety-sub-turing-epistemologic.md` (#36).*

---

## 0. The recognition stated cleanly

Alex, verbatim, this morning while we were profiling the 5-minute pre-commit chain:

> What if we begin to look into moving the build pipeline and checks into mirror... The speedup doesn't seem to be faster tests. Concurrent what can be done concurrent, composing into artifacts.

The performance lane found the actual culprit: `cargo test` is 80s cold / 30s warm — not the limit. The limit is the *hook chain*: format → check → clippy → test → audit, all serialised, all redoing work no commit changed. The structural fix is not "faster tests." The structural fix is **do only what needs doing, parallel by structure, compose into artifacts**.

That phrasing names a content-addressed declarative build system. We did not need to ask whether mirror should have one. **Mirror IS already one.** Every primitive a content-addressed declarative build system requires is declared at substrate altitude already. The 5-minute pre-commit is the wrong frame; the right frame is: *what is the smallest substrate change that re-settles the system?*

The 43rd-instance candidate (per `feedback-substrate-already-had-the-word`): the build-system altitude is the canonical instance of the substrate already having the word. Every primitive Bazel, Buck2, Nix, Shake, and Dagger built from scratch is sitting in `shards/` waiting to be consumed by a consumer named `@mirror/check` or `@mirror/build` that mosaic doesn't yet emit at the hook altitude.

---

## 1. Build Systems à la Carte: the Mokhov-Mitchell-Peyton-Jones taxonomy

The canonical theory paper for build systems is Mokhov, Mitchell & Peyton-Jones (2018), *Build Systems à la Carte* (ICFP 2018; expanded to JFP 2020 as *Build Systems à la Carte: Theory and Practice*). It is the load-bearing decomposition the modern field is built on. Read it before anything else; this section maps mirror onto its taxonomy.

The paper's central move is to decompose any build system into three orthogonal concerns and then show that the design space is the cartesian product of choices on each axis:

1. **`task` — what to build.** A `Tasks c k v` is a polymorphic function that, given a way to fetch a value (`k -> f v`), produces the value of a key, suspending in an effect that captures dependencies. `c` is the constraint on the effect: `Applicative` for *static* dependencies (known before the task runs); `Monad` for *dynamic* dependencies (the task chooses what to depend on based on intermediate results); `Selective` (Mokhov-Mitchell-Peyton-Jones 2019, the missing third) for *speculative* dependencies (a static over-approximation that the runtime can prune). The constraint determines what the build system can express; the constraint also determines what can be analysed before execution.

2. **`scheduler` — what order to build in.** Topological (compute a static DAG once, walk it in order) vs restarting (start a task, abort if a dependency turned out to be missing, restart later) vs suspending (start a task, *suspend* on a missing dependency, resume when the dependency is computed). Suspending is strictly more powerful than restarting for monadic tasks; topological is only well-defined for applicative tasks.

3. **`rebuilder` — when to skip.** Dirty-bit (Make: timestamp older than dependency → rebuild) vs verifying traces (Shake: rebuild if any traced dependency's value changed) vs constructive traces (Bazel/Nix: cache by content hash of inputs; the *output* is a function of the inputs' OIDs; if the input OID is in the cache, the output is retrieved by hash, no rebuild needed at all).

The cartesian product of these three is the design space. Make is (Applicative, Topological, Dirty-Bit). Bazel is (Applicative-ish, Restarting, Constructive-Traces). Shake is (Monad, Suspending, Verifying-Traces). Nix is (Applicative, Topological, Constructive-Traces) at the derivation altitude. Excel — *Excel* — is (Monad, Restarting, Dirty-Bit-with-Dynamic-Reordering); the paper opens with the recognition that a spreadsheet is a build system.

### 1.1 Mirror's mapping onto the three axes

**`task` — mirror's task is a Prism transform.** The five operations (`focus`, `project`, `split`, `shift`, `settle`) ARE the task algebra. Each operation has a typed input and a typed output (per `shards/glass.mirror`'s `imperfect(a, e, l)` carrier); each composes through the substrate's monad (`Imperfect<a>` carrying `transparency<p>` at the loss slot). What makes mirror's task carrier sharper than the paper's `Tasks c k v`:

- mirror's `c` is *fixed by the operation* (focus is applicative — its dependencies are the spec; settle is monadic — what it depends on depends on the verdict trail); the constraint is not a parameter to the task system, it is a *type-level fact* about which of the five operations the task IS.
- mirror's `k -> f v` fetch carries the altitude in the key type itself. `splinter(@code/rust)` and `splinter(@release)` are different types, not different runtime tags. The `f` effect is `Imperfect<...>` carrying transparency; the fetch can return `partial(opacity_map)` and the task is allowed to compose under partial without aborting (Bazel cannot do this; the verdict is binary pass/fail).
- mirror's dependency surface is *structural*: dependencies are not declared in a side-channel build file; they are the typed `in @prism / in @glass / ...` headers at the top of each shard. The dependency graph is the shard graph; the shard graph is the eigensheaf (per the load-bearing recognition in `shards/mirror/spectral.mirror`).

**`scheduler` — mirror's scheduler is the kintsugi oscillation driven by the sheaf-Laplacian's eigendecomposition.** Per `shards/mirror/spectral/oscillate.mirror`, the loop alternates ACTIVE/DARK passes (the SpectralUuid's golden-ratio split made operational: 48 ACTIVE bits drive the navigable proposal, 80 DARK bits anchor identity). The scheduling discipline:

- *Parallelism* is determined by the sheaf-Laplacian's eigenmodes. Edges with small (near-zero) eigenvalue are independent — they can be settled in parallel without restriction-map violations. Edges with large eigenvalue are coupled — they must be sequenced. This is strictly sharper than Bazel's "compute a static DAG and walk it in topo order"; the sheaf Laplacian *also* tells you which graph cuts are safe under partial-information settlement, which a DAG topo-sort does not.
- *Suspension* is the substrate's default; tasks that hit a `\` (typed hole) suspend until the hole is settled by Fate inference or by another tick of the kintsugi loop. This is Shake's suspending scheduler at the substrate altitude, with the hole carrier (`\`) made first-class instead of side-channel.
- *Ordering* under partial information is governed by `query_phi` (per `shards/mirror/spectral/consent.mirror`) — the structural Φ query that names which candidate morphism to apply next when multiple are valid. The scheduler does not need a global priority queue; it reads the consent verdict per pulse.

**`rebuilder` — mirror's rebuilder is the constructive-trace algebra of `splinter` + `@spectral/db`.** Per `shards/mirror/store.mirror` (declaring `@mirror/store`) and `shards/glass.mirror`'s splinter carrier:

- Every artifact is content-addressed at every altitude. `splinter.content: oid`; two splinters with byte-equal `(content, altitude, transparency)` ARE the same splinter (per the identity contract in `shards/glass.mirror`).
- The `@spectral/db` (closed-source engine; declared at the substrate boundary per memory `architecture-mirror-store-vs-spectral-db`) caches by spectral-uuid; lookup is constant-time on the navigable 48-bit ACTIVE coordinate; identity verification is constant-time on the DARK 80-bit content hash.
- Constructive traces are the substrate's default: if the input shard's OID hasn't changed, the output shard's OID won't either, and the cache holds the result. No rebuild needed. This is Bazel's action cache and Nix's derivation store rolled into one substrate primitive.
- *Sharper than Bazel*: Bazel's traces are binary (cache hit or cache miss). Mirror's traces are *graded* — a cache hit at full transparency is one outcome; a cache hit at `partial(opacity_map)` is another, and the opacity_map names *which* properties were not verified, so the rebuilder knows what to re-verify *just for the partial regions*, not the whole shard. This is the move Bazel cannot make because Bazel's verdict carrier is `Result<T, E>`, not `transparency<p>`.

### 1.2 The mapping is *complete*

Every position in the Build Systems à la Carte design space maps onto a substrate primitive mirror already declares:

| BSALC axis | Mirror primitive | Substrate site |
|---|---|---|
| `Tasks c k v` (the task algebra) | The five Prism operations | `shards/prism.mirror` (declared) |
| `Applicative` constraint | `focus`, `project` — static deps | `shards/mirror/mosaic.mirror` |
| `Monad` constraint | `settle` — dynamic deps from verdicts | `shards/mirror/mosaic.mirror` |
| `Selective` constraint | `query_phi` — speculative pruning | `shards/mirror/spectral/consent.mirror` |
| Topological scheduler | Eigensheaf static DAG, applicative shards | `@mirror/spectral/eigensheaf` (declared) |
| Restarting scheduler | (mirror does not restart; it suspends) | — |
| Suspending scheduler | Kintsugi `\` suspends until Fate settles | `shards/mirror/spectral/oscillate.mirror` |
| Dirty-bit rebuilder | (mirror does not use timestamps; it uses OIDs) | — |
| Verifying-trace rebuilder | `verify(oid, bytes) -> verdict` | `shards/mirror/store.mirror` |
| Constructive-trace rebuilder | `splinter` content-address + `@spectral/db` | `shards/glass.mirror`, `shards/mirror/store.mirror` |
| Cache / store | `@mirror/store` (open) + `@spectral/db` (closed) | `shards/mirror/store.mirror`, memory `architecture-mirror-store-vs-spectral-db` |
| Build-event trace (Bazel BEP) | `transparency<p>` opacity_map + `.shatter` projection | `shards/glass.mirror`, `shards/mirror/shatter.mirror` |
| Workspace / project file | `mirror.spec` + `mosaic(@spec)` | `shards/mirror/mosaic.mirror` |
| Action result | `au(altitude)` | `shards/mirror/au.mirror` |
| Settled output | `shard(altitude)` | `shards/glass.mirror` |

The cells without a position are not gaps in the substrate; they are positions mirror has *chosen against* (timestamps lose information; restarting wastes work). The substrate occupies the sharpest cells of the BSALC design space *by construction*.

---

## 2. The content-addressed build canon — what each gets right, where mirror is sharper

### 2.1 Bazel / Blaze (Google)

Bazel (open-source 2015; the Blaze internal system since 2006) is the reference content-addressed build system. The load-bearing primitives are action hashing (compute the hash of an action's inputs; if the hash is in the action cache, retrieve the output without running the action), Skyframe (the incremental evaluation framework — every node in the build is a Sky-function whose value depends on the values of other Sky-functions; the framework caches all of them and invalidates transitively), and the Build Event Protocol (BEP — a streaming protobuf surface that lets external tools observe a build's progress without polling).

What Bazel gets right:
- Content-addressed action cache + remote execution (the `remote-apis` protocol shared with Buck2 and other systems).
- Sandboxed execution: an action sees only its declared inputs; this enforces dependency correctness.
- Hermetic builds: pinning the toolchain content-addressably means the same source produces the same binary.

What Bazel leaves on the table:
- The verdict is binary (pass / fail at the action altitude). There is no `partial(confidence)` tier; an action either succeeds or it fails. The BEP carries `Aborted` / `Failed` / `Cancelled` states but no honest `partial`. Mirror's `transparency<p>` carries the third state structurally.
- The dependency graph is a DAG. Parallelism is determined by the DAG's topo-sort. There is no mathematical notion of *coupling strength* between actions — every cross-action edge is treated as a hard constraint. The sheaf-Laplacian's eigendecomposition gives a graded coupling measure that admits parallel settlement of weakly coupled actions even when a DAG topo-sort would serialize them.
- BEP is a side-channel; build provenance is structurally separate from build inputs. Mirror's `.shatter` projection (per `shards/mirror/shatter.mirror`) is the canonical disk projection of the settlement state itself — provenance and content are the same content-addressed object.

### 2.2 Buck2 (Meta)

Buck2 (open-sourced 2023; written in Rust) is Bazel's structural successor, designed by the same lineage (Neil Mitchell, who wrote Shake, is on the team). Buck2's key advances over Bazel: **dynamic actions** (per Mitchell's "What Makes Buck2 Special" 2025 talk — actions can produce new actions based on intermediate results, which is the Monad tier of BSALC's task constraint that Bazel's Applicative tier could not express); **explicit incremental compute** via the DICE (Demand-driven, Incremental Computation Engine) graph; **strong target/action separation** (the target graph is static; the action graph refines it dynamically).

What Buck2 gets right:
- Dynamic dependencies as a first-class concern (the Monad tier).
- The target graph / action graph split (declaration vs realisation, structurally enforced).
- Rust performance; deterministic incremental rebuilds.

Where mirror is structurally sharper:
- Buck2's "action graph refines the target graph but cannot reach beyond it" (Mitchell 2025 slide). Mirror's `mosaic(altitude)` is *parametric* — the same algebra at @store, @spec, @emitter, @code/rust, @ci/github. There is no "target graph" altitude separate from a "compositional" altitude; the same five operations apply at every altitude. The decomposition is finer.
- Buck2's verdict carrier is `Result<T, BuckError>`; mirror's is `Imperfect<au, error, transparency<p>>`. Mirror can settle to `partial` and still commit a useful shard; Buck2 must succeed or fail.
- Buck2's identity is action-hash (input bytes hashed); mirror's identity is splinter-OID composed with shard SpectralUuid (golden-ratio split, navigable + identity). Mirror's identity carries route information; Buck2's does not.

### 2.3 Nix

Nix's load-bearing contribution is *the derivation* — a content-addressed (input-addressed by default; content-addressed-output is the 2021+ RFC 0062 lift) recipe. The store is the universal CAS. Flakes (the post-2020 schema) make this consumable from outside the store.

What Nix gets right:
- Universal CAS at the OS-level. The store path *is* the identity.
- Reproducibility by construction: same derivation → same path → same content.
- Hermetic builds at the system level (every dependency including libc is content-addressed).

Where mirror is structurally sharper:
- Nix's content-addressing is *flat* — a derivation has inputs and outputs; there is no compositional algebra over derivations themselves (apart from the recursive Nix call). Mirror's `splinter` / `shard` / `mosaic(altitude)` is the compositional algebra Nix lacks at the derivation altitude.
- Nix's verdict is binary again (build succeeds, build fails). The `partial` tier doesn't exist.
- Nix evaluation is the Nix language interpreter (lazy, untyped). Mirror's evaluation is typed at the altitude, and the type *is* the altitude (per memory `architecture-prism-as-trait-as-everything`).

### 2.4 Shake (Neil Mitchell, 2010+)

Shake is the Build-Systems-à-la-Carte authors' own build system. The key contribution is **dynamic dependencies** as a first-class concern — a Shake rule can read a file, then decide what other files to depend on based on what it read. The verifying-trace rebuilder records *what* was read, not just *that* something was read; rebuilds happen when the value of any traced dependency changed.

What Shake gets right:
- The monadic task algebra (BSALC's `Monad` tier) implemented honestly.
- Polymorphic dependencies (any Haskell type that is `Show + Eq + Hashable` can be a dependency key).
- The suspending scheduler that doesn't waste work.

Where mirror is structurally sharper:
- Shake's polymorphic dependencies are Haskell types; mirror's are typed altitudes. The polymorphism is *structured* by the substrate's altitude lattice, not just by the type system.
- Shake records *value-of-read* as the trace; mirror records *transparency<p>-of-settled* as the trace. The latter carries the third state that the former cannot.

### 2.5 Mill (Lihaoyi) / Pants v2 / Please.build

Each of these is a Bazel-shaped system with a different surface: Mill is Scala-native, Pants v2 is the `@rule` decorator pattern (Python-resolved), Please.build is the BUILD-file syntax for monorepos. All hit the same cells in BSALC's space (Applicative tasks; topological scheduler; verifying or constructive traces).

What they collectively get right:
- Surface ergonomics: declarative, language-native, IDE-friendly.
- Sound incremental rebuilds within their declared scope.

Where mirror is structurally sharper:
- None of them carry the eigensheaf framing; their dependency graphs are DAGs all the way down. The sheaf-Laplacian's eigendecomposition is a parallelism analysis tool none of them have.

### 2.6 Pluto (Erdweg-van Binsbergen-Konat-Visser 2015)

Pluto is the OOPSLA 2015 paper that named "sound and optimal incremental build with dynamic dependencies" as a formal contract. The paper interleaves dependency analysis with builder execution, proving soundness (no stale results) and optimality (no redundant rebuilds) relative to a formal dependency-graph model.

Pluto's load-bearing move is the formal verification of the rebuild algorithm. Mirror's load-bearing move is the *cybernetic* verification: kintsugi's `e^(n+1) < e^n` proof obligation (per `CLAUDE.md`) is the substrate's version of soundness + optimality at the verdict altitude rather than the file-timestamp altitude. The Pluto contract is a proper subset of kintsugi's contract.

---

## 3. The declarative pipeline canon — DAG framing vs sheaf framing

### 3.1 Dagger / Earthly / BuildKit

Dagger (containerized pipelines with Cue-based DSL) and Earthly (Earthfile syntax on top of BuildKit) both ride the BuildKit content-addressed layer cache. Dagger compiles a Cue pipeline to a BuildKit LLB (Low-Level Build) graph; Earthly compiles Earthfiles to LLB. The cache is content-addressed at the LLB-node level.

What they get right:
- Pipeline-as-code (Cue / Earthfile), not pipeline-as-YAML.
- Container-level reproducibility through content-addressed layers.
- Cross-CI portability (the same pipeline runs locally, in GitHub Actions, in GitLab CI).

Where mirror is structurally sharper:
- LLB is a *flat* graph of container operations. Mirror's `mosaic(altitude)` is a sheaf — the operations are typed by altitude and composed by restriction maps. Cross-altitude composition is structurally enforced; Dagger's cross-stage composition is convention.

### 3.2 Snakemake / Nextflow (bioinformatics) / Drake (R)

These are file-based pipeline DSLs (Snakemake: Python; Nextflow: Groovy DSL; Drake: R) for dataflow pipelines. Dependencies are inferred from declared file inputs and outputs. The cache is by file timestamp or content hash.

What they get right:
- DSL ergonomics for dataflow.
- File-level granularity that matches the bioinformatics / data-science workflow shape.
- Reproducibility through content-addressing (Snakemake's `--use-conda` + content-hash cache).

Where mirror is structurally sharper:
- The unit of caching is the *file*. Mirror's unit of caching is the splinter, which is altitude-typed. The file-altitude is just one of the altitudes a splinter can live at.

### 3.3 Apache Airflow / Prefect / Dagster / Tekton / Argo

These are workflow orchestrators rather than build systems strictly — DAG-based, scheduler-centric, with state tracking but typically without content-addressed caching (Dagster's recent asset-based framing is the exception). They focus on long-running, distributed workflows rather than incremental rebuilds.

Where mirror is structurally sharper:
- Workflow state is held externally (a database). Mirror's state is the shard's SpectralUuid; the state is the identity. There is no separate state to keep consistent with the workflow definition.

### 3.4 DVC / MLflow / W&B / Hugging Face artifacts / Kubeflow

The AI/ML content-addressed pipeline lineage: data + model + experiment as the cached units. DVC tracks data files; MLflow tracks experiment runs; W&B tracks artifacts; HuggingFace tracks model + dataset hashes; Kubeflow orchestrates them on Kubernetes.

What they collectively get right:
- Data and models are first-class build artifacts.
- Content-addressing extends to datasets and weights, not just code.

Where mirror is structurally sharper:
- The AI/ML canon treats data, model, and experiment as separate altitudes with separate stores. Mirror's `mosaic(altitude)` is uniform: `mosaic(@data/parquet)` and `mosaic(@code/rust)` and `mosaic(@fate/weights)` are the same algebra at different altitudes. The cross-altitude composition (a model's weights depending on the dataset's content addresses on the training code) is one substrate fact, not three integrations.

---

## 4. The math each canon assumes vs what mirror has

| Canonical structure | Where it appears in the canon | Mirror's existing primitive | Status |
|---|---|---|---|
| **DAG** (Directed Acyclic Graph) | Make, Bazel, Buck2, Nix, Shake, Dagger, Airflow | `splinter_graph` (OID-graph closure) | Operational; declared in `shards/mirror/store.mirror` as the structural lockfile |
| **Monoid** (artifact combination) | Bazel rules, Cabal | `shard` merge under SpectralUuid `combine` | Operational; declared in `shards/glass.mirror` + `shards/uuid/spectral.mirror`; memory `architecture-shard-as-crdt` makes this the homomorphism |
| **Monad** (effectful build) | Shake, Buck2 dynamic actions, BSALC's `Monad` tier | `Imperfect<a, e, l>` monad with `settle` as Kleisli arrow | Operational; declared in `shards/glass.mirror`; `au → Imperfect<shard>` is the canonical instance |
| **Free monad** (declaration vs evaluation) | BSALC's `Tasks` abstraction | The `Prism` trait + the five operations; declaration in shards is the free form, evaluation is the kintsugi loop | Operational |
| **Lattice** (versioned state, monotonic accumulation) | git, Nix, CRDTs | `shard` as bounded semilattice; SpectralUuid as monoid homomorphism | Operational; memory `architecture-shard-as-crdt` |
| **Sheaf** (local-to-global consistency) | Emerging in formal methods (Hansen-Ghrist 2019); mirror's eigensheaf | `@mirror/spectral/eigensheaf` | Declared; consumer not yet wired |
| **Sheaf Laplacian** (parallelism via eigendecomposition) | Hansen-Ghrist arXiv:1808.01513 | T8 numerical primitive (NumericalPrism backend) | Landed |
| **Adjunction** (declaration ↔ realisation) | Category theory; not yet operationalised in any build canon | `@code/metalogue/materialize` discriminator (the recognitive direction of the @code/metalogue turn-pair; pairs with `@code/X/macro` for the shim direction) | Declared at `shards/code/metalogue/materialize.mirror`; T21 originally at `@mirror/realisation`, re-homed 2026-06-10 per recognition #50's form/substance audit + the metalogue-turn-pair recognition |
| **Trace / replay (provenance)** | Bazel BEP, Nix logs, OpenTelemetry | `transparency<p>` + `.shatter` projection | Operational |
| **Content-address** (universal cache key) | Bazel CAS, Nix store, BuildKit | `oid` at @mirror/store; `splinter.content: oid` | Operational |

Two cells are **declared but not consumed**: `@mirror/spectral/eigensheaf` and `@code/metalogue/materialize`. Both are the substrate-pull-correct positions for parallelism analysis and declaration-vs-realisation discrimination; both wait for a consumer at the build-pipeline altitude.

That consumer is what does not yet exist. The substrate is complete; the *hook* surface that calls these primitives is what needs to land.

---

## 5. What `mirror build`, `mirror check`, `mirror test` look like as substrate operations

The current pre-commit (per `Justfile`'s `pre-commit` recipe) runs `cargo fmt --check && cargo check && cargo clippy && cargo test && cargo audit` sequentially. ~5 minutes on a typical commit. *Most of that work is redoing what hasn't changed.*

A substrate-native `mirror check` would compose as follows. None of the primitives are new; the composition is.

```
mirror check
  = settle(@check)
  = focus(.spectral/HEAD)            # the current shard handle
  | project(changed_splinters)       # diff against the parent; restrict
                                     # to splinters whose OIDs differ
  | split(eigensheaf_neighbours)     # widen by sheaf restriction; the
                                     # subgraph that depends on the
                                     # changed splinters
  | shift(@code/rust | @code/mirror) # lift each affected splinter to
                                     # its native altitude
  | settle(verdict_per_splinter)     # run the property chain per
                                     # splinter; the verdict carrier
                                     # composes under transparency<p>
```

The crucial moves:

1. `project(changed_splinters)` reads `@mirror/store.diff(HEAD, HEAD^)` and produces the changed-splinter set. This is *not* `git diff`; it's an OID-graph diff at the splinter altitude, which is sharper because two byte-different commits with the same parsed AST share splinters.
2. `split(eigensheaf_neighbours)` reads `@mirror/spectral/eigensheaf` (declared) to compute the dependency closure of the changed splinters. The closure is bounded by the sheaf restriction maps — only splinters whose restriction-map output changes need re-settlement.
3. `shift(@code/rust | @code/mirror)` shifts to the native altitude. For Rust code, this delegates through `@io.cargo` (per `shards/mirror/mosaic.mirror`'s cargo @io contract). For `.mirror` shards, this is the bootstrap evaluator.
4. `settle(verdict_per_splinter)` runs the property chain. The verdicts compose under `transparency<p>`'s monoid (Fail-dominates / Partial-min-confidence / Pass-neutral, per `prism/imperfect/src/transparency.rs`). Failure on any splinter halts and surfaces an algedonic signal (per the cybernetic-foundation insight); partial commits with located opacities.

Sheaf-Laplacian parallelism is read from the eigendecomposition: edges whose Forman-Ricci curvature (T9 primitive) is high are coupled and must be serialised; edges with low curvature are independent and settle in parallel. The scheduler does not need to be told "this is parallelizable" — the substrate's geometry tells it.

`mirror test` is the same composition with the verdict altitude shifted from `@check` to `@test` — the property chain at `@test` runs property-based tests; the property chain at `@check` runs format / clippy. The five operations are uniform; the altitudes differ.

`mirror build` is the same composition with the altitude shifted to `@release` — the property chain at `@release` runs the artifact-emit. Cargo invocation is one `@io` crossing per artifact, but the *deciding* of which artifacts to emit is structural.

---

## 6. Performance projection — content-addressed-skip is the right floor, not ExUnit-parity

The performance profile that landed this morning (Taut's prior run) named ExUnit-parity as the implicit bar (Elixir's ExUnit runs the equivalent test suite in seconds; cargo test takes 30-80s cold-or-warm). That framing is the wrong bar. **The right bar is content-addressed-skip parity: if 90% of the work has not changed (which is true for a typical commit), the right runtime is the 10% of work that did change.**

The 5-minute pre-commit is dominated by:

1. `cargo fmt --check` — reads every file, parses every file, formats every file in memory, compares. ~3s wall clock on the mirror tree.
2. `cargo check` — compiles every crate's type-checking pass. ~20-45s.
3. `cargo clippy` — runs clippy lints on every crate. ~30-60s.
4. `cargo test` — runs every test. ~30-80s.
5. `cargo audit` — checks the lockfile against the advisory database. ~5-10s.

Plus hook overhead (loading the shell environment, running git operations, formatting commit messages). The hook chain's wall-clock floor is around 90s if everything is warm; 4-5 minutes is what we measure.

A typical commit on the mirror tree changes 1-3 splinters at `@code/rust` altitude (one file, often one function). Under content-addressed-skip:

- `cargo fmt --check`: substrate skips entirely if the splinter set's OIDs don't reach into formatter scope. **Floor: ~0s.**
- `cargo check`: only the splinters whose type signatures changed, plus their transitive dependents-by-type, need re-checking. The eigensheaf's restriction maps name this set exactly. For a typical 1-3 splinter change, this is 5-10 splinters. **Floor: ~5-10s** (one rustc invocation per affected crate, or one rustc invocation total if all splinters live in one crate).
- `cargo clippy`: same scope as check. **Floor: ~5-10s.**
- `cargo test`: only tests whose splinter-OID closure intersects the changed splinter set need to run. For a typical commit, this is on the order of 5-20 tests. **Floor: ~2-5s.**
- `cargo audit`: this is at `@release` altitude; the cargo lockfile splinter changes only when dependencies change. **Floor: ~0s for the typical case; ~5s when the lockfile changes.**

Composing in parallel where the sheaf-Laplacian says safe (which for the typical commit's 5-10 affected splinters is most of them; their cross-coupling is weak): **the realistic floor is 10-20 seconds wall clock for a typical pre-commit** on the mirror tree under substrate-native composition.

That is a 15-30x speedup over the current 5-minute floor. Not from faster tests; from doing only what needs doing. The cybernetic property is *requisite variety in the verdict*: the substrate's variety budget is spent matching the changed-splinter set's variety, not the entire-tree variety.

---

## 7. The cybernetic mapping — VSM applied to the build pipeline

Beer's Viable System Model is not a metaphor for the build pipeline; it is its cybernetic ancestor. Per the cybernetic-foundation insight (`~/dev/systemic.engineering/practice/insights/cybernetics/2026-06-09-cybernetic-foundation-for-mirror-substrate.md`) and Beer's *Brain of the Firm* (1972), *The Heart of Enterprise* (1979), and the operational evidence in Project Cybersyn (1971-73, Chile under Allende — the original cybernetic build system, where Beer designed S1-S5 over the national economy with telex and a single computer):

- **S1 — Operations.** The per-splinter, per-shard, per-altitude compile operations. Each splinter is an S1 unit producing its own au. The five Prism operations are the S1 atomic verbs.
- **S2 — Coordination.** The eigensheaf's restriction maps. S2's job is preventing oscillation between S1 units; the sheaf restriction maps say "this splinter's settled form is consistent with that splinter's settled form" — the formal local-to-global consistency check is S2's verdict.
- **S3 — Cache management.** `@spectral/db` and `@mirror/store`. S3 manages the here-and-now: what's cached, what needs invalidating, what the resource budget per kintsugi tick is. The HamiltonScheduler (per memory `architecture-hamilton-scheduler`) is the S3 implementation that landed Margaret Hamilton's Apollo 1202 priority discipline at the build altitude.
- **S4 — Intelligence.** The kintsugi loop's *which-splinters-to-re-settle* decision. S4 looks outward and forward; for the build pipeline, "outward" is the changed-splinter set since the last commit, and "forward" is the prediction of which dependents need re-verification. The eigensheaf's eigendecomposition is the S4 instrument.
- **S5 — Policy.** `transparency<p>` + algedonic signalling. S5 sets the policy on what verdicts are acceptable. The threshold (0.8 per memory `architecture-error-as-question`), the gates, the merge-or-pause decision. Algedonic signals (per the cybernetic-foundation document §3) bypass all lower systems when a verdict fails catastrophically — the `failure(reason)` case routes directly to the user without waiting for the next kintsugi tick.

This is *not* a metaphor. The structural identity is verified by the BSALC mapping above (S2 = scheduler; S3 = rebuilder cache; S4 = task algebra choosing what to do next; S5 = the policy in the verdict carrier). Every modern build system is a *partial* VSM; mirror is a *complete* one because the substrate was already cybernetic before the build-pipeline altitude was named.

Project Cybersyn ran an entire national economy as a cybernetic feedback loop. The mirror pre-commit hook is a smaller-scale instance of the same architecture. The lineage is direct: Beer → Cybersyn → Reyes' work on Viable Cooperative Systems → the substrate's S1-S5 carriers declared in `shards/`.

---

## 8. Candidate substrate-pull recognitions surfaced

The survey turned up three candidate recognitions. Honest assessment of each:

### #43 — Mirror IS a content-addressed declarative build system

**Status: promote-ready.** The substrate already declares every primitive a content-addressed declarative build system needs. Splinter / shard / mosaic / spectral-db / eigensheaf / sheaf-Laplacian / kintsugi / transparency / realisation are the carriers; mosaic's five operations are the task algebra; the kintsugi loop is the scheduler; the @spectral/db + @mirror/store split is the cache. The 5-minute pre-commit is the wrong frame because it does not consume any of these. The right frame is: *which substrate consumer would replace each Justfile recipe?*

This is the canonical instance of `feedback-substrate-already-had-the-word` at the *build-pipeline* altitude. The recognition rate is now 43 instances; this one is at the largest-scale instance to date (a whole product category — the build-system industry — already named at substrate altitude).

### #44 (candidate) — The hook surface IS the @io decoherence boundary for the build-pipeline altitude

**Status: candidate.** Per the existing `architecture-kintsugi-variety-io` memory: @io crossings are decoherence events; the substrate stays in mirror as long as possible. The current `just pre-commit` IS an @io crossing — it shells out to `cargo`, `git`, `nix`. Each shell-out is a decoherence event that loses the substrate's verdict carrier and recomposes through stdout/exit-code.

The recognition would name: the hook surface (pre-commit, pre-push, GitHub Actions step) IS the @io altitude for the build pipeline; the substrate-native verdict (mirror-text per `kintsugi-ci-v0.1.md` §1.4) is what crosses this boundary; the JSON serialiser appears *only* at the boundary, not within the substrate. This is already partly recognised (kintsugi-ci-v0.1 §1.4 states it), but it has not been promoted to a general principle for the build-pipeline altitude.

Promote-ready if Alex agrees with the framing. Otherwise candidate.

### #45 (candidate) — `e^(n+1) < e^n` IS the soundness-and-optimality contract Pluto formalised

**Status: candidate.** Pluto (Erdweg et al. 2015) named "sound (no stale results) and optimal (no redundant rebuilds)" as the formal contract for incremental build systems and proved it relative to a dependency-graph model. Mirror's `CLAUDE.md` states `e^(n+1) < e^n` as the convexity proof of the business model. The two are the *same theorem* at different altitudes:

- Pluto's soundness ≡ no kintsugi tick increases the loss (lossy ticks are rejected).
- Pluto's optimality ≡ no kintsugi tick re-settles a splinter whose OID is unchanged (the content-address cache is consulted first).

The substrate's `e^(n+1) < e^n` is the cybernetic restatement of Pluto's formal result, generalised from a file-timestamp dependency graph to the eigensheaf restriction-map graph. Candidate because the theorem at the build-pipeline altitude has not yet been written down explicitly.

---

## 9. Roadmap implications

### 9.1 Does this change the cybernetic-property cascade order?

Yes, slightly. The cybernetic-foundation insight named six cybernetic properties to land (per `~/dev/systemic.engineering/practice/insights/cybernetics/2026-06-09-cybernetic-foundation-for-mirror-substrate.md` §6). Today's recognition adds a seventh consideration: the cascade should include a property `content_addressed_skip_correct` that names "this operation correctly skips work when its input splinter OIDs are unchanged." Without it, the kintsugi loop has no formal way to verify that the cache is being consulted at the right altitudes.

Suggested addition to the cybernetic-property cascade:

- `requires content_addressed_skip_correct(operation)` — the operation must produce the same au when its input splinter set's OID is unchanged. The substrate-side compiler proof obligation is the substrate's version of Pluto's optimality theorem.

### 9.2 Does this suggest a new substrate addition?

Yes — a `@mirror/check` (or `@mirror/build`) family that declares the operations a build pipeline performs at the hook altitude. Candidate species:

- `@mirror/check/format` — the mosaic settle at the formatter altitude.
- `@mirror/check/types` — the mosaic settle at the type-check altitude.
- `@mirror/check/lint` — the mosaic settle at the lint altitude.
- `@mirror/check/test` — the mosaic settle at the test altitude.
- `@mirror/check/audit` — the mosaic settle at the dependency-audit altitude.

Each is one altitude in the build pipeline; each is a target the existing `mosaic.mirror` can settle through `settle_on { ... }`. None of these adds substrate; they name altitudes that already exist as targets in the implicit hook chain.

### 9.3 The smallest first tick that operationalizes the recognition

**T?: Lift `just pre-commit` from a shell script to a mosaic.spec settlement.**

The single smallest tick that operationalizes "mirror IS a content-addressed build system" is:

1. Add a `mirror.spec` declaration that lists the pre-commit checks as `settle_on` targets.
2. Have `just pre-commit` invoke `mirror kintsugi mirror.spec` (or equivalently `mirror check`) instead of the chained cargo commands.
3. The kintsugi loop reads the splinter-graph diff against HEAD, projects to the eigensheaf neighbourhood, shifts to the appropriate altitude (mostly @code/rust, some @code/mirror), and settles. The cargo @io contract (per `shards/mirror/mosaic.mirror`) is invoked only where mosaic's substrate-native settlement is not yet wired.

The first version of this tick can fall back to the existing cargo-shell-out behaviour at every altitude; the substrate-pull then proceeds altitude by altitude, replacing each shell-out with a substrate-native settlement. The performance improvement appears *gradually* as each altitude is lifted; the architecture is in place from tick one.

This is the "smallest first tick" in the substrate-pull-discipline sense: nothing new is added; only the wire format of the pre-commit chain is changed to be substrate-native.

---

## 10. Cross-references

### Prior insights this depends on

- `2026-06-09-bateson-logical-type-as-substrate-primitive.md` (#42) — the logical-type primitive that makes "the build pipeline is one logical type; the hook surface is one logical type up" structurally correct.
- `2026-06-09-cascade-is-deutero-learning.md` — the cascade as Learning II; this recognition is one tick of that cascade, made at the build-pipeline altitude.
- `2026-06-09-ashby-multi-dimensional-variety-sub-turing-epistemologic.md` (#36) — the variety-budget framing; the content-addressed-skip floor IS spending the variety budget on the changed set, not the whole tree.
- `2026-06-08-portal-eigenvalue-stream-gen-prism.md` (#26) — the `shift(oid, T)` typed-capability primitive; the cache lookup operation IS this primitive at the build altitude.
- `2026-06-07-eigenspace-as-composition-foundation.md` — the eigensheaf as composition; the parallelism analysis the BSALC scheduler axis requires.

### Substrate shards this consumes

- `shards/glass.mirror` — splinter / shard / transparency / verdict floor.
- `shards/mirror/store.mirror` — the @mirror/store interface; oid; splinter_graph; the six store operations (read/write/exists/diff/walk/verify).
- `shards/mirror/mosaic.mirror` — the build-altitude algebra; the five operations on the build manifold; the cargo @io contract.
- `shards/mirror/au.mirror` — the proposed-shard form; identity + verify + shatter at au's altitude.
- `shards/mirror/shatter.mirror` — the disk projection; build provenance as content-addressed projection.
- `shards/code/metalogue/materialize.mirror` — the recognitive turn of the @code/metalogue conversation; the boundary-vs-substrate discriminator; the adjunction between declaration and realisation. Re-homed from `@mirror/realisation` 2026-06-10 per recognition #50's form/substance audit + the metalogue-turn-pair recognition.
- `shards/mirror/spectral.mirror` — the agent-coordination family; the kintsugi oscillation lives here.
- `shards/mirror/spectral/oscillate.mirror` — the ACTIVE/DARK alternation; the build pipeline's scheduler is this loop.
- `shards/mirror/spectral/consent.mirror` — `query_phi` and the auto-apply boundary; the policy the build pipeline reads per pulse.
- `shards/mirror/spectral/score.mirror` — the orchestra's shared score; the eigenboard + metalogue + pending kintsugi state.

### Memories this is grounded in

- `architecture-three-tier-stack` — fragmentation-mcp / mirror / @spectral/db; the VSM-conformant stack the build pipeline composes into.
- `architecture-connes-spectral-triple` — (A, H, D) as the substrate's operational form; the build pipeline IS the spectral triple at the development-altitude.
- `architecture-shard-as-crdt` — shard as bounded semilattice; the build's cache merges under a CRDT-correct discipline.
- `architecture-prism-as-trait-as-everything` — the five Prism operations as the universal algebra; the task algebra is this without modification.
- `architecture-cybernetic-foundation` — VSM and the cybernetic-property family; §7 of this insight applies that framing to the build pipeline.
- `architecture-mirror-store-vs-spectral-db` — the open foundation / closed engine split; the cache architecture IS this distinction.
- `architecture-hamilton-scheduler` — Margaret Hamilton's Apollo 1202 priority discipline; the S3 implementation for the build pipeline.
- `architecture-kintsugi-variety-io` — @io crossings as decoherence events; the hook surface is the build-pipeline's @io boundary.
- `architecture-error-as-question` — the 0.8 threshold; the verdict policy S5 reads.
- `feedback-substrate-already-had-the-word` — the 43rd-instance candidate; the build-pipeline altitude is the largest-scale instance to date.
- `feedback-craft-not-deliver` — this insight proposes recognitions and the smallest first tick, not implementations.
- `feedback-no-time-estimates` — the performance projection is a floor (the math floor), not a time estimate (the delivery floor).

### Canonical papers this depends on

- Mokhov, A., Mitchell, N., & Peyton-Jones, S. (2018). "Build Systems à la Carte." *ICFP 2018*. Expanded as JFP 2020. The taxonomy this insight maps onto.
- Mokhov, A., Mitchell, N., Peyton-Jones, S., & Marlow, S. (2019). "Selective Applicative Functors." *ICFP 2019*. The Selective tier of the task constraint.
- Erdweg, S., van Binsbergen, L. T., Konat, G., & Visser, E. (2015). "A Sound and Optimal Incremental Build System with Dynamic Dependencies." *OOPSLA 2015*. Pluto's formal soundness and optimality contract; the substrate-side `e^(n+1) < e^n` is the same theorem.
- Mitchell, N. (2025). "What Makes Buck2 Special." 22 May 2025 slides. The action-graph-refines-target-graph framing; mirror generalises this through `mosaic(altitude)`.
- Hansen, J. & Ghrist, R. (2019). "Toward a Spectral Theory of Cellular Sheaves." *Journal of Applied and Computational Topology*; arXiv:1808.01513. The sheaf-Laplacian foundation; the parallelism axis the canon does not have.
- Beer, S. (1972). *Brain of the Firm*; (1979) *The Heart of Enterprise*. The VSM whose S1-S5 the build pipeline instantiates.
- Conant, R. & Ashby, W. R. (1970). "Every Good Regulator of a System Must Be a Model of That System." *International Journal of Systems Science* 1:2. The good-regulator theorem; mirror's build pipeline must contain a model of the development process it regulates.

---

## 11. The slogan

**Mirror is not "going to be" a content-addressed declarative build system. Mirror IS one. The substrate has declared every primitive Bazel / Buck2 / Nix / Shake / Dagger built from scratch.**

**The 5-minute pre-commit is not a tests-are-too-slow problem. It is a tests-redo-everything-because-the-substrate-isn't-consumed problem.**

**The smallest first tick is to make `just pre-commit` invoke `mirror kintsugi mirror.spec`. Every altitude after that is substrate-pull, in the order the splinter graph asks.**

**The canon called this content-addressed-skip plus structural parallelism. The substrate calls it: settle the substrate against the changed splinters; emit verdicts; algedonic-signal on failure. Same operation. The substrate just had the word first.**

---

*Forty-three is the count where the substrate-pull cascade reaches its own build pipeline. The recognition is Learning II at the substrate's most recursive site: the system that builds the system. Bateson would have called this the moment the cybernetic substrate notices it is its own controller.*

---

## 12. Addendum (2026-06-12, Mara): Jacobi-fixed-point convergence proof

The spectral-db / spectral audit surfaced a sharper formulation of the
build-pipeline-as-substrate recognition: a **convergence proof via the
Jacobi guarantee on real-symmetric matrices**. This addendum lands the
formulation in the canonical doc; the spec at
`docs/specs/spectral-db-mirror.md §7` is the off-tree articulation.

### 12.1 The fixed-point statement

The system is at a fixed point when

```
settle(@spectral.grammar_graph) = @spectral.grammar_graph
```

— the grammar describing settlement, when settled, produces the same
graph hash. The grammar's own content address closes under the
operation the grammar defines.

This is not circular. It is **convergent** — and the convergence has
a proof.

### 12.2 The Jacobi guarantee

The grammar's adjacency matrix is real symmetric: every edge in the
shard-import graph is undirected at the substrate's typing altitude
(the `in @prism / in @glass / ...` headers declare bidirectional
dependence — the importing shard requires the imported shard's
declarations, and the imported shard's reachability is realised
through its importers; per §1.1's structural-dependency reading).
Real symmetric matrices have real eigenvalues and an orthonormal
eigenbasis — and the **Jacobi eigenvalue algorithm** is guaranteed
to converge on any real symmetric matrix to within ε in a finite
number of off-diagonal-zeroing sweeps (Jacobi 1846; Golub & Van
Loan §8.4 for the modern treatment).

Therefore the spectral decomposition of the grammar's adjacency
matrix converges. Therefore the fixed-point computation
`settle(@spectral.grammar_graph)` reaches a stable hash in a finite
number of kintsugi-oscillation passes. Therefore the fixed point
**exists, and the substrate can reach it**.

### 12.3 What this sharpens

The §1.1 reading mapped mirror onto the Mokhov-Mitchell-Peyton-Jones
taxonomy and named the sheaf-Laplacian's eigendecomposition as the
scheduler. The Jacobi formulation makes the convergence claim
explicit: the scheduler converges *because the spectral problem
admits Jacobi*, not by stipulation. The build pipeline's `e^(n+1)
< e^n` (§7's Pluto-soundness reference) is the operational form of
the Jacobi off-diagonal-norm decrease per sweep.

The substrate-pull alignment with §1.1's "Mirror IS already one"
slogan: the substrate *already had* the convergence guarantee. It
was sitting in the real-symmetric structure of the import graph
the whole time. The Jacobi theorem is the canon-side name for what
the kintsugi oscillation has been doing operationally since the
loop existed.

### 12.4 What is *not* claimed

- The Jacobi guarantee gives finite convergence to within ε, not
  one-pass exactness. The kintsugi loop's `e^(n+1) < e^n` is the
  Jacobi-sweep equivalent; the substrate reaches the fixed point
  asymptotically, not instantaneously.
- The real-symmetric structure is the typing-altitude statement.
  At the @io altitude — where IO crossings break symmetry per
  recognition #57 (alignment-as-boundary-mathematics) — the
  adjacency is *not* symmetric; that asymmetry is the substrate's
  boundary, not its interior. The Jacobi convergence applies to
  the interior; the boundary is governed by separate cybernetic
  discipline (`@io` properties).
- This addendum does not re-derive Jacobi. The convergence
  guarantee is canonical numerical-analysis result; the substrate
  cites it. The novelty is the recognition that the substrate's
  graph is structurally in the Jacobi-applicable class.

### 12.5 Cross-references

- `docs/specs/spectral-db-mirror.md §7` — the off-tree articulation
  this addendum lands.
- §1.1 — the sheaf-Laplacian scheduling discipline this formalises.
- Recognition #57 (alignment-as-boundary-mathematics) — the @io
  asymmetry that bounds where Jacobi applies.
- Golub, G. H., & Van Loan, C. F. (2013). *Matrix Computations* (4th
  ed.), §8.4 — the Jacobi eigenvalue algorithm and its convergence
  guarantee.

---
