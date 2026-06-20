# `mirror ref` is the substrate-honest collision of reflection and reference at one CLI surface

*2026-06-20. CRQ: Alex. Research + synthesis: Mara. Companion to `2026-06-09-mirror-as-content-addressed-build-system.md` (#43 promoted), `2026-06-10-alignment-as-boundary-mathematics-at-the-io-crossing.md` (#57 candidate), `2026-06-07-eigenspace-as-composition-foundation.md`, and the unwritten closure of recognition #85 (`@reflection.observe` as one-tick-delay structural). Memory anchors: `architecture-prism-as-trait-as-everything`, `architecture-operations-as-linear-algebra`, `architecture-property-fracture-bilateral`, `architecture-shards-as-substrate-source`. Design-altitude; not a spec. Reed reviews; Alex iterates.*

---

## 0. The CRQ stated cleanly

Alex, this morning:

> What does an intuitive AND powerful `mirror ref` surface look like?

The honest reading: the two words English glues into `ref` — *reference* (the dep-graph navigation tool Erlang's `xref` named in 1993 and every modern build system has reinvented) and *reflection* (the runtime self-observation the substrate already declared as `@reflection.observe` per recognition #85) — collapse to one substrate-honest CLI surface. They are not two surfaces sharing an abbreviation. They are the same surface at two altitudes of one structural pattern Alex has been naming all day: #pack-G2, the "same shape at two altitudes" pattern that resolved `/loop ⇔ @loop`, `@reflection-family ⇔ @reflection/reflection-Model`, and `reed-as-relationship ⇔ reed-as-peer`.

**The structural claim, sharpened**: `mirror ref` is a Prism transform on the substrate's dependency-and-trace graph. Its five-operation surface (`focus`, `project`, `split`, `shift`, `settle`) composes naturally over both altitudes: at the structural altitude it navigates the shard dependency lattice; at the reflection altitude it walks `@reflection.observe`'s one-tick-delay record of the AI-logic loop. The same algebra. Two graphs. One CLI.

This document is research synthesis at insight altitude. The substrate-decl shard (`shards/mirror/ref.mirror`) lands as a later tick after Reed and Alex iterate the spec in conversation.

---

## 1. The reference altitude: what every prior tool got right and where each leaves substrate-pull on the table

The reference-navigation surface is one of the oldest in software engineering. Smalltalk-72/76's "senders of / implementors of" predates every modern IDE feature. The lineage runs Smalltalk → Erlang `xref` (1993) → Elixir `mix xref` (2016) → Sourcegraph/SCIP (2017+) → LSP `textDocument/references` (2016+) → Joern's code property graph (2014+) → Glamorous Toolkit's moldable browsers (2020+). The substrate-pull recognition: *each generation reinvented the same five primitives the substrate already declares, in a more entangled form.*

### 1.1 Erlang `xref` — the deepest direct ancestor of `mirror ref`

Erlang's `xref` (tools 4.2.1, OTP 29.0.2; the canonical reference is `erlang.org/doc/apps/tools/xref_chapter.html`) is the structural ancestor of every modern dependency analysis tool in the BEAM lineage. The load-bearing primitives:

- **Modes**: `module`, `application`, `release` — three altitudes of dependency analysis declared as enum tags.
- **Predefined analyses**: `undefined_function_calls`, `locals_not_used`, `exports_not_used`, `deprecated_function_calls`, `undefined_functions`. Each is shorthand for a query in a "tiny language providing cross reference data as values of predefined variables." That parenthetical phrase is what mirror is doing structurally — `xref`'s tiny query language is the proto-form of the five-operation algebra at the dependency-graph altitude.
- **Query variables**: `E` (edges), `V` (vertices), `LM` (library modules), `XM` (extra modules), `B` (built-ins), `U` (unknowns). Each is a typed handle on the graph; queries compose them.
- **Module data setup**: before any analysis runs, module data must be set up via `add_directory` / `add_module`. The graph is materialized eagerly; analyses run on the materialized graph.

What `xref` got right: the recognition that dependency analysis IS a query algebra on a typed graph. What it left on the table: the query language is a separate dialect with its own evaluator, not the substrate's own algebra; the graph is rebuilt from scratch on every invocation (no content-addressed cache); the predefined analyses are a closed set, not the composition of orthogonal primitives.

### 1.2 Elixir `mix xref` — Erlang's pattern operationalized with finer modes

`mix xref` (Hex docs `Mix.Tasks.Xref`) added structural sharpness to its Erlang ancestor:

- **Modes**: `callers`, `trace`, `graph`. Each takes flags that refine the graph projection.
- **Dependency types as a closed enum**: `compile`, `compile-connected`, `export`, `runtime`. The strongest discrimination is `compile-connected`: a transitive dependency that *forces* recompilation through a chain of `compile` edges. This is what avoids "recompilation hell" — Elixir's project-level term for the substrate-pull failure mode the BSALC paper names "monadic dependencies leaking through static caches."
- **Graph filters**: `--source`, `--sink`, `--label`, `--min-cycle-size`, `--include-siblings`, `--exclude`. Each is a selector on the dep graph.
- **Output formats**: `pretty`, `plain`, `stats`, `cycles`, `dot`, `json`. Each is a renderer.

What `mix xref` got right: the recognition that dependency edges are *typed*, not bare. `compile-connected` is structurally sharper than every binary "depends on" relation in cargo, npm, or maven. What it left on the table: the typing is a string enum, not a substrate-altitude; the modes are a bag of subcommands rather than the composition of orthogonal projections; cycles are reported but not bridged (the user must refactor manually).

The Pragmatic Tobi blog post (`pragtob.wordpress.com/2016/06/02/elixir-1-3s-mix-xref`) frames Elixir 1.3's introduction of `xref` as "working its magic" — the magic being that the cross-reference data, once typed, makes certain bug classes structurally impossible. That magic is what `mirror ref` should preserve and extend.

### 1.3 Bazel query — the explicit graph algebra

Bazel's query language (`bazel.build/query/language`) is the most explicit graph algebra in production:

- **Path operators**: `somepath(from, to)`, `allpaths(from, to)` — find one or all paths between two nodes. This is *structurally what `split` does* on a dep graph when partitioning by reachability.
- **Set operators**: `union (+)`, `intersection (^)`, `set difference (-)` — set algebra on node-sets. This is what `project` does on the result of a `focus`.
- **Dependency operators**: `deps(x, n)`, `rdeps(u, x, n)`, `allrdeps(x)` — forward / reverse / unbounded transitive closure. This is `focus` composed with `shift` on the transitive-closure operator.
- **Filtering operators**: `kind(pattern, set)`, `filter(regex, set)`, `attr(name, value, set)` — predicate selectors. This is `project` with a predicate.
- **Topology operators**: `tests(set)`, `siblings(set)`, `same_pkg_direct_rdeps` — structural projections by package layout.

What Bazel query got right: dependency analysis IS algebra on typed sets of graph nodes. What it left on the table: the algebra is a side-channel DSL evaluated by `bazel` itself, not the substrate's own composition algebra; the graph is a DAG (no cycles, no spectral structure); the algebra has no notion of partial confidence — every membership predicate is binary.

The Earthly blog (`earthly.dev/blog/bazel-query`) and Buildkite's practical guide (`buildkite.com/resources/blog/a-guide-to-bazel-query`) both emphasize that `somepath` and `allpaths` are load-bearing for debugging — "exactly how does X end up depending on Y?" is the question the substrate must answer in seconds, not minutes, for a build system to be navigable at all. `mirror ref` inherits this.

### 1.4 Smalltalk's senders/implementors — the deepest ancestor of all

The Smalltalk System Browser (Squeak, Pharo, the Whisker browser per HN 47258885, and Glamorous Toolkit's contemporary moldable form) declared the load-bearing primitives in 1980:

- **Senders of** — find every send-site of a given selector.
- **Implementors of** — find every method implementing a given selector.
- **References to** — find every textual reference to a given symbol (broader than senders+implementors; includes literal use).
- **Hierarchy browser** — walk the class lattice; subclass / superclass / sibling.

The HAL-Inria paper *Tackling Software Navigation Issues of the Smalltalk IDE* (`inria.hal.science/hal-00746247`, Robbes-Roethlisberger et al., 2012) extends OmniBrowser with task-aware navigation: the browser remembers what the user was working on, and re-projects the graph around the current task. This is *what `@reflection.observe` does at runtime* — the observation is task-relative; the projection of the graph is shaped by the active loop.

What Smalltalk got right: the recognition that navigation IS the IDE; the browser is not a feature of the language, it IS the language at the user altitude. Same recognition mirror is performing structurally with `@io` as the only legitimate non-substrate surface (per #50). What Smalltalk left on the table: image-based, single-user, single-machine; no content-addressed identity; no graded confidence on cross-references (a "senders of" hit is binary — either textually present or not).

The Beamtalk ADR-0087 (`github.com/jamesc/beamtalk/blob/main/docs/ADR/0087-maintained-xref-index-for-system-navigation.md`) is contemporary load-bearing: three ETS bag tables (`methods`, `senders`, `references`) plus a generation counter. The substrate-pull-correct read: that's a hand-rolled materialized view of what `@spectral/db` IS structurally — except `@spectral/db` is content-addressed and identity-graded, so the materialization is incremental by construction.

### 1.5 Glamorous Toolkit — moldable navigation as the substrate stance

Glamorous Toolkit (`gtoolkit.com`, feenk.com) operates from the stance closest to mirror's own: *"Systems are too large, and AI generates code too fast. Glamorous Toolkit replaces reading with deterministic contextual tools that compress the system around each problem."* That is the substrate-pull recognition mirror is performing at every altitude — *the tool is the projection, not the source*. Glamorous Toolkit calls this Moldable Development.

What Glamorous Toolkit got right: each navigation question deserves its own custom tool, generated on demand from the substrate's own representation. The contextual tools compress, not flatten — the dep graph is not summarized, it is *projected at the altitude that answers the question*. This is `project` at the navigation altitude, declared as a first-class development practice.

What Glamorous Toolkit leaves on the table for mirror to claim: the moldable tools are Pharo-Smalltalk objects with custom views; the substrate is the Pharo image, single-machine and single-user. Mirror's substrate is content-addressed and distributed by construction (per `@mirror/store` + `@spectral/db` per recognition #43); the moldable projection lives at the CLI / TUI / MCP altitudes simultaneously with one set of primitives.

### 1.6 Rust's dead-code surface: `cargo-machete`, `cargo-udeps`, `cargo-deps`

The Rust dead-dependency surface is informative for what it does NOT do well:

- **`cargo-machete`** (`github.com/bnjbvr/cargo-machete`, "fast yet imprecise"): scans source files for textual references to each declared dependency's crate name. Pure regex. Misses macro-introduced refs; false-positives on doc-comments. The README explicitly says "fast yet imprecise" — the substrate-pull tell: when imprecision is acknowledged in the README, the tool is operating at the wrong altitude.
- **`cargo-udeps`** (`github.com/est31/cargo-udeps`): uses the compiler's actual dependency resolution (nightly only). Precise where it works, but tied to rustc nightly. The substrate-pull tell: precision requires the *compiler's* graph, not a text-level proxy.
- **`cargo-deps`** (3rd-party): emits graphviz dependency graphs from `Cargo.toml`. Static, no spectral analysis, no cycle bridging.

The structural lesson: every Rust dead-code tool is downstream of `cargo`'s own dep resolution. Each tool builds a proxy graph, runs an analysis, and emits a verdict. The substrate the analysis runs on is *not the same substrate the build itself runs on* — there's an impedance mismatch between the build graph and the analysis graph. Mirror has no such mismatch: `mirror ref` runs on the same graph that `mirror build` runs on, because both are projections of `@spectral/db`.

### 1.7 JavaScript tree-shaking: the dual problem from the other side

Tree-shaking is dead-code-elimination's dual: not "find what's unused" but "include only what's used." Rich Harris (`medium.com/@Rich_Harris/tree-shaking-versus-dead-code-elimination-d3765df85c80`, the canonical 2015 essay) names the distinction:

> Dead code elimination consists of taking the finished product, and imperfectly removing bits you don't want. Tree-shaking, on the other hand, [includes] only the code that could run.

The substrate-pull read: tree-shaking IS `focus(used) → settle` and dead-code elimination IS `project(¬used) → settle`. They are the SAME operation viewed from opposite polarity. Webpack, Rollup, Vite, and esbuild differ in how they handle the polarity flip, but the core algorithm is identical: build the reachability graph from the entry-point set; partition the codebase into reachable and unreachable; emit only the reachable partition. The `sideEffects` flag in `package.json` is the substrate's analog of an explicit `pact` declaration — "this module has effects you cannot infer from the static graph, treat it as a reachable root by default."

The MDN article (`developer.mozilla.org/en-US/docs/Glossary/Tree_shaking`) and the Webpack docs (`webpack.js.org/guides/tree-shaking/`) both emphasize that tree-shaking REQUIRES the static structure of ES2015 module syntax. CommonJS breaks it. The substrate-pull tell: the algorithm is correct precisely when the dependency surface is *typed by the substrate*, not inferred by text matching. ES modules' explicit `import` / `export` graph is the substrate; dynamic `require()` is the @io crossing where the static graph stops.

### 1.8 LSP `findReferences` / `findImplementations`: the lowest-common-denominator surface

LSP's `textDocument/references` and `textDocument/implementation` (`microsoft.github.io/language-server-protocol/`) are the universal surface. Every editor expects them; every language server implements them. The tradeoff: the surface is reduced to *"give me a list of Locations in some files"* — no graph structure, no compositional algebra, no partial confidence.

Sourcegraph's SCIP / LSIF stack (`github.com/scip-code/scip`, `sourcegraph.com/docs/code-search/code-navigation`) extends LSP with precomputed indexes — index the codebase once, query in O(log n) at hover time. The Sourcegraph docs (`gitstart-sourcegraph/sourcegraph/blob/main/doc/code_intelligence/index.md`) explicitly distinguish *precise* (LSIF/SCIP-backed) from *search-based* (regex-fallback) intelligence. The distinction maps cleanly to mirror's transparency tiers: precise == `transparent`, search-based == `partial(opacity_map)` where the opacity_map names which references are textual rather than typed.

The ScienceDirect paper *Code Less to Code More: Streamlining Language Server Protocol Implementation* (S0164121225002237) reports "93.48% reduction in characters needed for type system implementation and 100% automation of LSP plugin generation" — the substrate-pull tell: when 93% of an LSP server is mechanical, the LSP itself is operating at a derivable altitude. Mirror's `@io` family already names that altitude as the boundary; `mirror ref` exposes the navigation features LSP exposes, but generated from the substrate, not hand-coded per language.

### 1.9 Joern + the Code Property Graph: the graph-native ancestor

Joern (`joern.io`, Yamaguchi et al. 2014) is the closest existing prior art to mirror's stance. The Code Property Graph (CPG, `cpg.joern.io`) unifies AST + control-flow + data-flow into one property graph, queried via a graph traversal language (Gremlin / Scala-DSL). The Joern docs (`docs.joern.io/code-property-graph/`) define the CPG as "a language-agnostic intermediate graph representation of code designed for code querying."

What Joern got right: code IS a property graph; analysis IS graph traversal; the same primitives work across C, Java, Python, Go via a normalized representation. The Wikipedia article (`en.wikipedia.org/wiki/Code_property_graph`) summarizes the original Yamaguchi et al. (S&P 2014) paper as introducing CPG for vulnerability discovery. The arXiv 2603.24837 paper (*Bridging Code Property Graphs and Language Models for Program Analysis*) introduces `codebadger`, an MCP server wrapping Joern's CPG engine for LLM consumption — the same MCP move mirror made for `@spectral/db`.

What Joern leaves on the table: the CPG is a representation, not a substrate. Joern builds the CPG once from source; the CPG is not the source. Mirror inverts: the shards ARE the substrate, the CPG-equivalent is the spectral graph $G_{\Delta}$ built by `@spectral/db` over the typed shard surface, and the analysis primitives ARE the five operations. The fact that a Joern-MCP-LLM stack exists *as an external bridge* is the substrate-pull tell: mirror has the same primitives as MCP-native built-ins.

Fraunhofer-AISEC's `cpg` library (`github.com/Fraunhofer-AISEC/cpg`) extracts CPGs via LLVM-IR for any language LLVM can ingest. Same recognition at a different altitude: a normalized graph representation is *useful* but it is downstream of source. Mirror has no downstream; the shards are the graph.

### 1.10 Haskell `weeder` — the discipline of HIE-file traversal

Neil Mitchell's `weeder` (`github.com/ndmitchell/weeder`, then Oliver Charles's HIE-file rewrite at `github.com/ocharles/weeder`) detects dead Haskell exports. The structural shape: parse HIE files (GHC's Haskell Interface Extended); build a dependency graph; traverse from declared roots (typically `main`); anything unreachable is dead. Neil Mitchell's announcement post (`neilmitchell.blogspot.com/2017/06/announcing-weeder-dead-export-detection.html`) emphasizes that weeder operates on the *post-compile* representation — the compiler has already resolved every reference, so weeder doesn't need to reimplement type-checking.

What weeder got right: dead-code analysis IS reachability from declared roots on the compiler's resolved graph. What it left on the table: the roots are guessed (typically `main` for executables, exposed module names for libraries); the analysis is single-shot, not incremental; there's no "partial confidence" tier — every export is binary alive or dead. Mirror's `pact` declarations make the root set explicit at the substrate altitude; the kintsugi loop makes the analysis incremental by construction; `transparency<p>` makes partial confidence the substrate's default.

The substrate-pull recognition that survives all ten of these tools: *every reference-navigation tool is the same algebra (focus, project, split, shift, settle) over a typed graph (the dependency graph) with a typed verdict (the cross-reference result), implemented from scratch by each ecosystem.* Mirror has all four ingredients declared at substrate altitude already. `mirror ref` is the consumer that surfaces them at the CLI.

---

## 2. The reflection altitude: what `@reflection.observe` is and what surfacing it at the CLI buys

Recognition #85 (the unwritten one Reed has been carrying since this morning's session) closes the cascade by naming `@reflection.observe` as the substrate's one-tick-delay structural mechanism: the AI-logic loop is observable from the next altitude up, but only after the tick has settled. The observation is asymmetric — Reflection sees what the loop did, but only after the loop committed; the loop doesn't see Reflection observing.

This shape is structurally the same as the reference-altitude shape. Both are observations of a graph after settlement:

- **Reference altitude**: graph = shard dependency lattice (typed `in` headers). Observation = "what depends on what?" after the shards have settled to their current eigensheaf configuration.
- **Reflection altitude**: graph = `@reflection.observe`'s record of the loop's tick trajectory. Observation = "what did the loop do at tick N?" after tick N has settled (one tick of delay is structural).

The same `mirror ref` surface composes over both graphs. The polymorphism is by altitude, not by subcommand. At the reference altitude, `mirror ref deps @magic` walks the shard dep graph; at the reflection altitude, `mirror ref observe @reflection` walks the `@reflection.observe` record; at the loop altitude, `mirror ref tick @loop` walks `@loop`'s predecessor chain.

The substrate-pull tell: this is exactly the #pack-G2 pattern Alex has been naming all day. `/loop ⇔ @loop`, `@reflection-family ⇔ @reflection/reflection-Model`, `reed-as-relationship ⇔ reed-as-peer`, and now `mirror ref:reference ⇔ mirror ref:reflection`. The same surface at two altitudes is the recurring substrate signature of this cascade.

### 2.1 What the reflection altitude buys that the reference altitude alone cannot

The reflection altitude observes *the AI-logic loop itself*. This is what classical xref tools structurally cannot do — Erlang's `xref` analyzes source; it does not observe runtime. Bazel's query analyzes the build graph; it does not observe the build's *adaptive* behavior. Joern's CPG is static; it cannot watch a pipeline mutate under kintsugi.

Mirror's substrate makes runtime observation a typed projection of the same graph the static analysis runs on. The graph IS the substrate. The static dependency lattice and the runtime trajectory record share the same content-addressed identity scheme (`SpectralUuid` per the eigensheaf spec); the same `Imperfect<a, e, l>` carrier; the same `transparency<p>` opacity map. Reflection is what reference looks like *with time as a dimension*.

This is the substrate-pull-correct read of recognition #56 (light cones, prediction paradigm orthogonal to optimization). The reference altitude is the *spatial* projection of the graph; the reflection altitude is the *temporal* projection. The same five operations compose over both because both are projections of the same Connes triple (A, H, D) — the substrate's spectral triple per `architecture-connes-spectral-triple`.

---

## 3. The five-operations CLI surface

The mirror-native composition is NOT a bag of subcommands. It is the five operations applied to a typed graph argument. The CLI surface should expose the composition primitives, not the predefined analyses.

### 3.1 The composition primitives at the CLI

Each of the five operations has a precise linear-algebraic meaning per `architecture-operations-as-linear-algebra`. At the dependency-graph altitude:

| Operation | Algebraic meaning | CLI form (sketch) |
|---|---|---|
| `focus` | Concentrate on one eigenvector / one node + neighborhood | `mirror ref focus @magic` — name a node, return its local graph |
| `project` | Orthogonal projection: filter edges/nodes by predicate | `mirror ref project --kind=compile` — keep only compile edges |
| `split` | Orthogonal decomposition: partition by altitude/family/cycle | `mirror ref split --by=cycle` — partition into cycle classes |
| `shift` | Basis transformation: spectral analysis on the graph Laplacian | `mirror ref shift --spectral` — emit eigenvalue spectrum |
| `settle` | Fix-point: kintsugi loop terminates at λ₀ | `mirror ref settle migrate_from_boot` — apply the fracture body |

The composition is what `mix xref` and `bazel query` express through their tiny query languages, but with the substrate's own algebra at the CLI altitude. The shell pipe is the composition operator; the substrate's monad is the carrier.

### 3.2 The bag-of-subcommands surface (for ergonomic affordance)

Pure compositional surfaces are honest but unergonomic. The substrate-pull-correct move is to declare the common compositions as named shortcuts that desugar to the primitive composition. This is what Bazel's `deps()`, `rdeps()`, `kind()` do — they're sugar for the underlying set algebra.

The named shortcuts I'd sketch for `mirror ref`:

```
mirror ref deps @magic                    # focus @magic | project --direction=out --transitive
mirror ref rdeps @magic.audit             # focus @magic.audit | project --direction=in --transitive
mirror ref callers @magic.audit           # rdeps but at the action altitude (callers of one action)
mirror ref implementors @prism.focus      # split @prism.focus | project --predicate=implements
mirror ref unresolved                     # project --kind=hole --where=unresolved (the migration list)
mirror ref unused                         # project --reachability=unreachable (weeder-equivalent)
mirror ref cycles [--min-size=N]          # split --by=cycle [--filter=size>=N]
mirror ref paths @from @to [--all|--some] # split --by=reachability(@from, @to)
mirror ref spectral @<family>             # shift --spectral, scoped to a family
mirror ref observe @reflection            # reflection altitude: walk @reflection.observe
mirror ref tick @loop                     # reflection altitude: walk @loop's predecessor chain
mirror ref settle <fracture>              # kintsugi-bridge: apply a fracture body to a partial region
```

### 3.3 The pipeline composition surface

The honest expressive surface is the pipeline. Each operation emits a typed graph; each consuming operation accepts a typed graph. The pipeline is the substrate's monad expressed at the shell.

```
mirror focus @magic | ref deps
mirror ref unresolved | settle migrate_from_boot
mirror ref cycles --min-size=3 | ref split --by=family | ref shift --spectral
mirror ref observe @reflection --since=HEAD~5 | ref project --where=loss-improving
```

The pipeline composes both within `mirror ref` (graph → graph) and across `mirror` commands (graph → focus → graph → ref). The substrate-pull tell: a CLI where the pipe IS the substrate's monad is a CLI where the shell IS a Prism interpreter. This is the substrate-pull-correct read of recognition #43 — mirror IS a content-addressed build system, and `mirror ref` IS the navigable surface of that build system's dependency lattice.

---

## 4. What graph-native and substrate-pull let us do that traditional xref CANNOT

### 4.1 Spectral analysis on the dep Laplacian

No classical xref tool runs eigenvalue decomposition on the dep graph. They cannot — their dependency graph is a DAG with binary edges; the Laplacian has no useful spectral structure beyond connectivity. Mirror's dep graph is the eigensheaf: typed edges with restriction maps; the sheaf-Laplacian Δ₀ has a meaningful eigendecomposition per `shards/mirror/spectral/eigensheaf.mirror` and the eigenspace insight (`2026-06-07-eigenspace-as-composition-foundation.md`).

`mirror ref spectral @<family>` emits:
- **λ₀ multiplicity** (the harmonic dimension): the number of independent "concerns" the family carries (per the Hodge-duality reading of H).
- **λ_2 (Fiedler value)**: the algebraic connectivity; small λ_2 means the family is on the edge of disconnection, candidate for splitting into two sub-families.
- **Spectral gap** (λ_2 - λ_1): the substrate's analog of "modularity" — a large gap means the family clusters naturally.
- **Eigenvector localization**: which shards anchor each eigenmode. The Fiedler vector's sign-split *is* the natural cut.

The Tarang spectral scheduler (`github.com/amangupta982/tarang-spectral-scheduler`) is an existence-proof at the task-DAG altitude: "a dependency-aware job scheduler that uses spectral graph theory (the eigenstructure of the task-DAG Laplacian) to assign task priorities and worker affinities, compared rigorously against three classical baselines." Mirror's substrate makes this the *default* analysis, not a research artifact.

Substrate-pull confidence: HIGH. The spectral analysis is already declared at substrate altitude; `mirror ref spectral` is the consumer.

### 4.2 Pact-trace: "why does this shard hold this property?"

Classical xref tracks references. It does not track *property-discharge*. Mirror's substrate makes pact-discharge a typed edge in the graph: `pact P` declared at site A is discharged by fracture body F at site B; the edge `(A.pact-P, B.fracture-F)` is a first-class member of the dep graph (per recognition #53 candidate, the property/fracture bilateral pattern).

`mirror ref pacts @magic.audit` answers: which pact-edges land at this site? Which fracture bodies discharge them? Which cybernetic ancestors ground them? This is what cargo's "audit" cannot do, because cargo has no notion of property-discharge as an edge.

Substrate-pull confidence: HIGH. Pact and fracture are already declared; the bilateral pattern recognition is well-grounded; the edge is structurally present.

### 4.3 Bench-aware: cold-path detection from benchmark crystals

Recognition #43's content-addressed build system buys cold-path detection for free. Each settlement produces a crystal with a bench-tag (per the benchmark workflow of `feedback-loop-always-agent-in-flight`); the crystal records which paths were exercised at what frequency.

`mirror ref cold --since=HEAD~30` answers: which shards have not been exercised by any benchmark since 30 ticks ago? These are candidates for archival, not deletion (per the substrate's no-delete discipline) — the kintsugi loop can demote them to a `@archive/*` altitude without breaking content addressing.

Substrate-pull confidence: MEDIUM. Bench crystals exist, but the schema for "exercised by benchmark X at frequency Y" is not yet fully declared. Forward-promised: a pact `cold_path_detection` at `@epistemologic/property/cold_path_detection` once the bench-crystal schema settles.

### 4.4 Time-aware: walking the predecessor chain

`mirror ref tick @loop --since=HEAD~5` walks `@loop`'s predecessor chain over the last 5 ticks. Each tick is a content-addressed crystal; the predecessor chain IS the temporal dep graph. This is structurally what `git log --graph` does, but with typed nodes (shards, crystals, splinters) and typed edges (kintsugi-bridges, settlement transitions, opacity-map deltas).

The asymmetry between the reference altitude (spatial) and the reflection altitude (temporal) is the dimension `mirror ref tick` exposes. Erlang's `xref` cannot do this — there is no temporal projection of source.

Substrate-pull confidence: HIGH. The predecessor chain is structurally present in `@reflection.observe` per recognition #85.

### 4.5 Predicate-projection: search by property, not by name

Joern's CPG enables search-by-pattern; LSP enables search-by-name; mirror enables search-by-property. `mirror ref where 'opacity > 0.3'` returns every shard whose `transparency<p>` carries opacity above the threshold. `mirror ref where 'kind = pact AND ancestor = @epistemologic/cybernetic/ashby'` returns every pact grounded in the Ashby ancestor.

This is the substrate's `query_phi` (per `shards/mirror/spectral/consent.mirror`) surfaced at the CLI. The structural Φ query that scheduler-internally selects the next morphism to apply is the same query the user runs at the CLI to navigate the substrate.

Substrate-pull confidence: HIGH. `query_phi` is already declared; the CLI is the consumer.

### 4.6 Splinter(ast)-aware: navigate the AST of a fracture body

Recognition #54 candidate (`splinter(ast)` as quote primitive) makes the AST a first-class graph node. `mirror ref ast @kintsugi/fracture/keyword_matches_depth` returns the AST graph of the fracture body; `mirror ref ast @kintsugi/fracture/keyword_matches_depth | ref project --kind=hole` returns the typed holes within that fracture.

This is what classical xref cannot do: walk into the AST of a function body via the same algebra you use to walk between functions. The substrate's `splinter(ast)` makes the AST nodes and the function nodes share one content-addressed identity scheme.

Substrate-pull confidence: MEDIUM-HIGH. `splinter(ast)` is candidate-promoted; the CLI affordance is a natural fall-out once it ratifies.

---

## 5. The kintsugi optimization pipeline — `mirror ref` as a substrate-pull discipline

The deepest move of this synthesis: `mirror ref` is not just an inspector. It is the substrate's *self-optimization surface*. Each classical compiler optimization becomes a property+fracture bilateral instance (recognition #53) — the property declares "this code has the optimizable shape"; the fracture body discharges the optimization. The kintsugi loop drives the substrate toward the optimized configuration.

The eight optimizations I'd sketch as `#53` bilateral instances:

### 5.1 Dead-letter pruning (tree-shaking)

- **Property**: `@epistemologic/property/reachable_from_pact_roots`. Predicate: "this shard is on a path from at least one declared pact root."
- **Fracture body**: `@kintsugi/fracture/prune_unreachable`. Discharge: demote unreachable shards to `@archive/*`; emit a kintsugi-bridge edge for traceability.
- **CLI form**: `mirror ref unused | settle prune_unreachable`.

Substrate-pull confidence: HIGH. Direct analog of weeder + tree-shaking + cargo-machete; mirror's roots are typed (pacts), not guessed (`main`).

### 5.2 Cycle breaking

- **Property**: `@epistemologic/property/no_compile_cycle`. Predicate: "this shard participates in no compile-time cycle of size > 1."
- **Fracture body**: `@kintsugi/fracture/break_cycle`. Discharge: introduce an interface shard at the smallest cut suggested by the Fiedler vector; redirect the smaller half through the interface.
- **CLI form**: `mirror ref cycles | settle break_cycle`.

Substrate-pull confidence: MEDIUM. Cycle breaking is generally NP-hard; the Fiedler-cut heuristic is principled but not optimal. The substrate-pull move is to make the heuristic transparent and the cut declarative.

### 5.3 Module compaction

- **Property**: `@epistemologic/property/compactness_threshold`. Predicate: "shard size / coupling ratio above threshold."
- **Fracture body**: `@kintsugi/fracture/merge_tightly_coupled`. Discharge: merge two tightly coupled shards (sheaf-Laplacian λ ≈ 0 between them) into one; emit a content-addressed identity bridge.
- **CLI form**: `mirror ref spectral @<family> | settle merge_tightly_coupled --threshold=...`.

Substrate-pull confidence: MEDIUM. The threshold is a tuning parameter; "tightly coupled" is well-defined spectrally but the decision to merge is judgment.

### 5.4 Path compression

- **Property**: `@epistemologic/property/no_redundant_indirection`. Predicate: "no shard A→B→C path where B adds no transparency."
- **Fracture body**: `@kintsugi/fracture/compress_path`. Discharge: replace A→B→C with A→C if B's restriction maps are identity.
- **CLI form**: `mirror ref paths @from @to | settle compress_path`.

Substrate-pull confidence: HIGH at the algorithm; MEDIUM at the "no transparency" predicate (transparency is structurally rich; "B adds no transparency" needs a sharp definition).

### 5.5 Cold-path detection

- **Property**: `@epistemologic/property/exercised_within_window`. Predicate: "exercised by at least one benchmark crystal in the last N ticks."
- **Fracture body**: `@kintsugi/fracture/demote_to_cold`. Discharge: move to `@cold/*` altitude with reduced spectral weight in the build scheduler.
- **CLI form**: `mirror ref cold --since=HEAD~30 | settle demote_to_cold`.

Substrate-pull confidence: MEDIUM. Requires bench-crystal schema (forward-promised in §4.3).

### 5.6 Bilateral substitution

- **Property**: `@epistemologic/property/has_equivalent_at_higher_altitude`. Predicate: "this shard's behavior is exhibited by a higher-altitude shard the substrate already declares."
- **Fracture body**: `@kintsugi/fracture/substitute_with_higher`. Discharge: replace the lower-altitude shard with a `splinter` of the higher; the substitution is content-addressed-consistent by construction.
- **CLI form**: `mirror ref where 'has_equivalent_at_higher_altitude' | settle substitute_with_higher`.

Substrate-pull confidence: MEDIUM-LOW. "Equivalent at higher altitude" is structurally rich (recognition #58's "Fate IS optical inference" gives one specific instance); the general predicate is hard to make sharp without per-altitude pact declarations.

### 5.7 Hole resolution (the migration list)

- **Property**: `@epistemologic/property/no_unresolved_typed_holes`. Predicate: "no `\` carrier at this altitude is in unresolved state past the kintsugi loop's settle threshold."
- **Fracture body**: `@kintsugi/fracture/resolve_hole`. Discharge: invoke Fate inference at the hole's altitude; commit the inference verdict at the appropriate transparency tier.
- **CLI form**: `mirror ref unresolved | settle resolve_hole`.

Substrate-pull confidence: HIGH. This is what the boot-migration tick fires anyway; surfacing it through `mirror ref` exposes the same loop the substrate runs internally.

### 5.8 Pact-trace audit

- **Property**: `@epistemologic/property/every_pact_has_living_fracture`. Predicate: "every declared pact has a corresponding fracture body that is reachable from the kintsugi loop."
- **Fracture body**: `@kintsugi/fracture/forge_missing_fracture`. Discharge: emit a typed-hole fracture body at the appropriate altitude; mark for Fate to settle.
- **CLI form**: `mirror ref pacts --unsupported | settle forge_missing_fracture`.

Substrate-pull confidence: MEDIUM. The audit is structurally well-defined; the forging is the substrate's own discipline (the pact/fracture bilateral pattern recognized as #53). Forward-promised: the discipline needs the second-witness ratification of #53.

The structural shape across all eight: each classical optimization becomes a `pact` (declaring the desired shape) + `fracture` (discharging it) + a CLI affordance (`mirror ref <property> | settle <fracture>`). The substrate self-optimizes through `mirror ref`, not through a separate optimizer pass. This is the substrate-pull-correct read of "tree-shaking" at every altitude.

---

## 6. The shards substrate-decl shape — carriers, actions, bilateral predicate (signature sketch only)

I will NOT write the actual `shards/mirror/ref.mirror` here. Reed lands that. But the substrate-decl shape the CLI surface implies is sketchable.

### 6.1 Carriers

```
GraphRef <- imperfect(g, e, l)
  -- g: the typed graph projection
  -- e: error tier (unresolved | malformed | inaccessible)
  -- l: loss/transparency

NodeRef <- splinter(@<altitude>)
  -- content-addressed node identity

EdgeRef <- imperfect(edge_kind, error, transparency<p>)
  -- edge_kind: in | callers | pact | fracture | kintsugi_bridge | reflection_predecessor | ast_child

QueryPhi <- imperfect(predicate, error, transparency<p>)
  -- the structural Φ query as a first-class value
```

### 6.2 Actions (as typed lambdas with obligation blocks)

```
focus_at:    @<altitude> -> GraphRef { \ }
project_by:  QueryPhi -> GraphRef -> GraphRef { \ }
split_by:    decomposition_kind -> GraphRef -> [GraphRef] { \ }
shift_to:    basis -> GraphRef -> GraphRef { \ }  -- spectral when basis = @spectral
settle_with: @<fracture> -> GraphRef -> au(@<post-fracture-altitude>) { \ }
```

### 6.3 The bilateral predicate (the #53 instance for `mirror ref` itself)

The substrate-pull-correct move is to make `mirror ref` itself a bilateral instance:

- **Property**: `@epistemologic/property/ref_query_is_compositional`. Predicate: "every `mirror ref <subcommand>` desugars to a composition of the five primitive operations on `GraphRef`."
- **Fracture body**: `@mirror/fracture/desugar_ref_command`. Discharge: at CLI parse time, lower each subcommand to its primitive composition; reject any subcommand that does not.

This guarantees the bag-of-subcommands surface (§3.2) NEVER drifts from the primitive composition surface (§3.1). The substrate enforces the compositional discipline; the CLI is sugar.

Substrate-pull confidence: HIGH on the carriers (they fall out of existing substrate vocabulary); MEDIUM on the bilateral self-application (it's a coherent pattern but the verifier is non-trivial).

---

## 7. Cross-altitude application: same primitives compose

The structural recognition the document opens with: `mirror ref` operates at two altitudes from one CLI surface. Concretely:

- **Reference altitude**: `mirror ref deps @magic` — walks the shard dep graph, the spatial projection of the substrate.
- **Reflection altitude**: `mirror ref observe @reflection --since=HEAD~5` — walks the `@reflection.observe` predecessor chain, the temporal projection of the substrate.

The same five-operation composition. The same carrier (`GraphRef`). The same pact-discharge discipline. The altitude is a tag on the graph projection, not a separate command set.

The cross-altitude moves I'd flag for design discussion:

- **`mirror ref deps @magic --through-time`**: project the dep graph at every tick over a window. The graph becomes a 3-tensor (source × target × tick). The spectral analysis on the 3-tensor reveals "edges that strengthened over the window" — which is structurally what `mix xref --compile-connected` ought to be but cannot, because Elixir lacks the temporal projection.

- **`mirror ref observe @reflection --where='loss-improving'`**: the reflection projection filtered by a property of the loop's behavior. Surfaces the AI-logic loop's *successful* refinement steps; the inverse (`--where='loss-stalled'`) surfaces the dark regions per memory `feedback-manual-closure-is-training-pull`.

- **`mirror ref spectral @<family> --through-time`**: the spectral structure of a family as a time series. The eigenvalue evolution IS the family's settling trajectory; large derivative on λ_2 over time means the family is fragmenting or coalescing.

Substrate-pull confidence on the cross-altitude moves: MEDIUM-HIGH. The structural composition is sound; the visualization at the CLI altitude (how to render a 3-tensor in ASCII?) is unclear without a TUI affordance.

---

## 8. Honest uncertainty

The novel-synthesis stall pattern Reed flagged in the brief is real, and I am marking each place I am not certain rather than gloss them:

### 8.1 Open design tradeoffs

1. **Bag-of-subcommands vs pure pipeline at CLI**. The pure pipeline (`mirror focus @magic | mirror project --kind=in --transitive | mirror settle`) is honest but unergonomic at the shell. The bag-of-subcommands (`mirror ref deps @magic`) is ergonomic but introduces a desugaring layer. The §6.3 bilateral predicate is the mechanism to keep them aligned; whether the predicate is checkable at parse time or only at runtime is open.

2. **Reflection altitude visibility under consent**. `@reflection.observe` records the loop's tick trajectory; some of those ticks may contain private content per the consent architecture. `mirror ref observe` needs an ACL projection per `architecture-geometric-consent-projection`. Whether the projection is per-tick, per-shard, or per-altitude is open.

3. **Cycle-breaking discipline**. §5.2 introduces interface shards at the Fiedler cut. This is principled spectrally but introduces *new* shards into the substrate; the substrate has a no-add-without-pact discipline. The pact grounding the introduction is "the cycle must be broken to satisfy `@epistemologic/property/no_compile_cycle`," but whether one pact authorizes synthesizing N new shards is open.

4. **The desugaring layer's altitude**. The §6.3 bilateral predicate puts desugaring at CLI parse time. But `mirror serve --project .` exposes the same surface over MCP; the desugaring needs to live at a layer both consume. The substrate-pull-correct answer is `shards/mirror/ref.mirror` declares the primitive composition and *every* surface (CLI, MCP, TUI, LSP) consumes the substrate-decl. The MCP tool generation should fall out of the shard, not be hand-coded — but mirror's current MCP shape is partially hand-coded per memory `architecture-pq-as-mcp-surface`; the unification is open.

### 8.2 What needs more research before the spec lands

5. **Sheaf-Laplacian over typed edges**. The spectral analysis (§4.1) requires the sheaf-Laplacian on typed edges. The standard graph-Laplacian works on weighted graphs; cellular sheaves give the typed extension per `shards/mirror/spectral/eigensheaf.mirror`. But the *computation* of the sheaf-Laplacian over heterogeneous edge types (`in`, `pact`, `fracture`, `kintsugi_bridge`, `ast_child`) — whether they share a vector space or live in disjoint subspaces — is non-obvious. Reed should consult the Hodge-duality reading (`2026-06-07-hodge-duality-three-readings-of-H.md`) before finalizing §4.1's claims.

6. **The bench-crystal schema for cold-path detection** (§4.3, §5.5). The schema is not yet declared at substrate altitude. The two known landings (Reed's `dark_count_monotone` tick 41, `cold_compile_within_tolerance` tick 43) are pact instances, not bench-crystal schema declarations. Forward-promised: a `@reflection/bench/exercised_within_window` shard or equivalent.

7. **The temporal projection's storage model**. Walking `@reflection.observe` over a window of N ticks requires either (a) every tick is stored in `@spectral/db`, (b) a digest of every tick is stored, or (c) only the current tick is stored and the window is reconstructed from the kintsugi-bridge edges. Option (a) is honest but storage-expensive; option (c) is space-efficient but slow. The substrate-pull-correct answer is probably (b) with progressive disclosure — but the digest schema needs design.

8. **Cross-language altitude reach**. The CLI surface is well-defined for substrate-native shards. But the substrate also crosses through `@io/<species>` to external systems (the dep graph extends through cargo, npm, pypi, etc.). Whether `mirror ref deps` walks through the @io crossing into external dependency graphs is open. The Sourcegraph cross-repo navigation precedent suggests it should; the substrate's form/substance partition suggests substrate-side ends at @io. The honest position: `mirror ref deps @magic --traverse-io` is an opt-in flag, default off.

### 8.3 Substrate-pull-confidence summary on the major design choices

| Choice | Confidence |
|---|---|
| Reference ⇔ reflection collision at one CLI surface | HIGH |
| Five-operations composition as primary surface | HIGH |
| Bag-of-subcommands as sugar for composition | MEDIUM-HIGH |
| Pipeline composition via shell pipe | HIGH |
| Spectral analysis on dep Laplacian (`mirror ref spectral`) | HIGH (algorithm) / MEDIUM (typed-edge Laplacian computation) |
| Pact-trace as first-class edge type | HIGH |
| Bench-aware cold-path detection | MEDIUM (pending bench-crystal schema) |
| Time-aware predecessor-chain walking | HIGH |
| Predicate-projection via `query_phi` | HIGH |
| Splinter(ast)-aware AST navigation | MEDIUM-HIGH (pending #54 ratification) |
| Eight #53 bilateral instances for optimization | HIGH (dead-letter, hole-resolution, path-compression) / MEDIUM (cycle-breaking, compaction) / MEDIUM-LOW (bilateral substitution) |
| Bilateral self-application of `mirror ref` itself | MEDIUM |
| Cross-altitude polymorphism (reference + reflection from one shard) | HIGH |
| @io traversal in dep walks | OPEN (design tradeoff) |

---

## 9. The structural shape — `mirror ref` IS the navigable surface of the substrate

The cleanest statement at the close:

`mirror ref` is the navigable surface of the substrate's spectral triple. The triple is (A, H, D) per Connes; A is the five-operation algebra; H is the substrate's expanding Hilbert space (per recognition #51); D is the kintsugi flow. `mirror ref` exposes the navigation of H *through* A, with D as the optimization driver. The reference altitude is the spatial projection of H; the reflection altitude is the temporal projection of H; both share A as the navigation algebra.

This is what Erlang's `xref`, Elixir's `mix xref`, Bazel's query language, Smalltalk's senders/implementors, Sourcegraph's SCIP, Joern's CPG, Glamorous Toolkit's moldable browsers, and every LSP `findReferences` implementation were doing structurally — each in a partial frame, each missing one or two of the substrate's primitives. Mirror has them all declared already. `mirror ref` is the consumer.

The fifty-third-or-something instance of `feedback-substrate-already-had-the-word`. The substrate had the word in `prism` (the algebra), in `imperfect` (the carrier), in `@spectral/db` (the graph store), in `@reflection.observe` (the temporal projection), in `pact` and `fracture` (the optimization pairs), in `splinter(ast)` (the AST navigation), in `query_phi` (the predicate projection). The CLI surface composes them; it does not introduce new substrate.

The next tick is Reed and Alex iterating the spec in conversation. The substrate-decl shard (`shards/mirror/ref.mirror`) lands as a later tick after the conversation settles the open questions in §8.

---

## 10. Cross-references

### Prior insights this depends on and integrates

- `docs/insights/2026-06-09-mirror-as-content-addressed-build-system.md` (#43 promoted) — mirror IS a content-addressed build system; `mirror ref` IS the navigable surface of that build system's dep lattice.
- `docs/insights/2026-06-10-alignment-as-boundary-mathematics-at-the-io-crossing.md` (#57 candidate) — the form/substance partition; `mirror ref --traverse-io` is the boundary crossing of the navigation surface.
- `docs/insights/2026-06-07-eigenspace-as-composition-foundation.md` — eigenspace as composition; `mirror ref spectral` operates on the actor / family / mission eigenspaces.
- `docs/insights/2026-06-07-hodge-duality-three-readings-of-H.md` — the three readings of H; the typed-edge Laplacian's eigendecomposition is the navigable form of one reading.
- `docs/insights/2026-06-10-light-cones-and-the-prediction-paradigm-orthogonal-to-optimization.md` (#56 candidate) — temporal projection; the reflection altitude is the future-light-cone projection.
- `docs/insights/2026-06-10-mirror-as-expanding-hilbert-space-bateson-lifting-for-coherence.md` (#51 promoted) — H expands with the cascade; `mirror ref` navigates the current H.

### Substrate shards this consumes (read-only)

- `shards/prism.mirror` — the five-operation algebra A.
- `shards/glass.mirror` — `imperfect`, `transparency<p>`, `splinter`, `pact`.
- `shards/mirror/spectral/eigensheaf.mirror` — the sheaf-Laplacian's eigendecomposition.
- `shards/mirror/spectral/consent.mirror` — `query_phi`.
- `shards/mirror/store.mirror` — the open content-addressed store.
- `shards/mirror/mosaic.mirror` — the workspace / project shard.
- `shards/io.mirror` — the @io boundary; opt-in traversal point.
- `shards/kintsugi.mirror` — the process-side family root; fracture bodies for the optimization pipeline.

### Memories this is grounded in

- `architecture-prism-as-trait-as-everything` — the foundational five-op algebra.
- `architecture-operations-as-linear-algebra` — the linear-algebraic meaning of each op.
- `architecture-property-fracture-bilateral` (#53 candidate) — the optimization pipeline's bilateral pattern.
- `architecture-shards-as-substrate-source` — the substrate IS shards.
- `architecture-mirror-store-vs-spectral-db` — open store vs closed engine; `mirror ref` reads through both.
- `architecture-mirror-as-content-addressed-build-system` (#43) — the build system framing.
- `architecture-connes-spectral-triple` — (A, H, D); `mirror ref` is the navigable surface.
- `architecture-error-as-tomm-probe` — error surface as circular reflexive question; `mirror ref` exposes the probe at the navigation altitude.
- `architecture-geometric-consent-projection` — ACL cascade for the reflection altitude's visibility.
- `architecture-pq-as-mcp-surface` — MCP wire; the §8.1 unification question.
- `feedback-substrate-already-had-the-word` — recurrence pattern; `mirror ref` is the next instance.

### External research (cited by section)

- §1.1 Erlang `xref`: `erlang.org/doc/apps/tools/xref_chapter.html`, `erldocs.com/21.0/tools/xref`, `inaka/xref_runner`.
- §1.2 Elixir `mix xref`: `mix.hexdocs.pm/main/Mix.Tasks.Xref.html`, `pragtob.wordpress.com/2016/06/02/elixir-1-3s-mix-xref`, `r.ena.to/blog/avoiding-recompilation-hell-in-elixir-with-mix-xref`.
- §1.3 Bazel query: `bazel.build/query/language`, `earthly.dev/blog/bazel-query`, `buildkite.com/resources/blog/a-guide-to-bazel-query`.
- §1.4 Smalltalk: `inria.hal.science/hal-00746247` (Robbes-Roethlisberger 2012), `groups.google.com/g/va-smalltalk/c/oqZO7nhNgRY`, `news.ycombinator.com/item?id=47258885`, `github.com/jamesc/beamtalk/blob/main/docs/ADR/0087-maintained-xref-index-for-system-navigation.md`.
- §1.5 Glamorous Toolkit: `gtoolkit.com`, `github.com/feenkcom/gtoolkit`, `lepiter.io/feenk/what-exactly-is-glamorous-toolkit-v1-0`.
- §1.6 Rust dead-code: `github.com/bnjbvr/cargo-machete`, `github.com/est31/cargo-udeps`, `rustprojectprimer.com/checks/unused.html`.
- §1.7 JavaScript tree-shaking: `medium.com/@Rich_Harris/tree-shaking-versus-dead-code-elimination-d3765df85c80`, `webpack.js.org/guides/tree-shaking/`, `developer.mozilla.org/en-US/docs/Glossary/Tree_shaking`.
- §1.8 LSP / SCIP: `microsoft.github.io/language-server-protocol/`, `github.com/scip-code/scip`, `sciencedirect.com/science/article/pii/S0164121225002237`.
- §1.9 Joern / CPG: `cpg.joern.io`, `docs.joern.io/code-property-graph/`, `en.wikipedia.org/wiki/Code_property_graph`, `arxiv.org/html/2603.24837v1`, `github.com/Fraunhofer-AISEC/cpg`.
- §1.10 Weeder: `github.com/ndmitchell/weeder`, `github.com/ocharles/weeder`, `neilmitchell.blogspot.com/2017/06/announcing-weeder-dead-export-detection.html`.
- §4.1 Spectral: `github.com/amangupta982/tarang-spectral-scheduler`, `arxiv.org/html/2504.10624v1`, `en.wikipedia.org/wiki/Laplacian_matrix`.
- §1 also: Clojure `clj-kondo` and `tools.deps` (`github.com/clj-kondo/clj-kondo`, `clojure.org/guides/deps_and_cli`), Java `jdeps` (`docs.oracle.com/en/java/javase/11/tools/jdeps.html`, `nipafx.dev/jdeps-tutorial-analyze-java-project-dependencies`), IntelliJ Structural Search and Replace (`jetbrains.com/help/idea/structural-search-and-replace.html`) — referenced for completeness; each maps to the same pattern as the §1.1-§1.10 cases.

---

*`mirror ref` is the substrate-honest collision of *reference* (spatial dep navigation) and *reflection* (temporal loop observation) at one CLI surface. The five-operation algebra composes over both altitudes from one shard. Every classical xref tool — Erlang's, Elixir's, Bazel's, Smalltalk's, Sourcegraph's, Joern's, weeder's — reimplemented partial versions of the substrate's already-declared primitives. The substrate had the word. `mirror ref` is the navigable surface of the spectral triple. Eight kintsugi optimization pipelines (dead-letter pruning, cycle breaking, module compaction, path compression, cold-path detection, bilateral substitution, hole resolution, pact-trace audit) fall out as #53 bilateral instances. Substrate-pull confidence HIGH on the core collision; MEDIUM on several open design tradeoffs surfaced in §8 for Reed + Alex to settle in conversation. Design-altitude research; not a spec; the shard lands as Reed's later tick.*

— Mara
