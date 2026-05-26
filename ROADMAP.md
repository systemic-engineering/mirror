# ROADMAP

> mirror written in mirror, parsing mirror, only using Rust for `@io` escape hatches.
>
> **v1.0 = the spectral.engineer cloud deployment.** Actual semver: `v0.1.0`. The framing and the version number serve different purposes.

---

## 1. What mirror IS

Mirror is a sub-Turing self-hosting compiler whose generated production code inherits formal verification guarantees from the sub-Turing source by structural construction. The substrate is the five-operation Prism algebra (focus, project, split, zoom, refract). The content layer is content-addressed via `SpectralCoordinate<5>` — a position in 5-dimensional information geometry derived from the Dirac operator on the content graph. Storage is `fragmentation`, a graph-native DAG VCS substrate whose canonical OIDs come from beta-normalized ASTs. The kintsugi loop is discrete Ricci flow on that substrate. The Bundle Tower (Fiber → Connection → Gauge → Transport → Closure) names the geometric layer; the Scheduler Tower names the temporal layer; together they're the complete spectral triple at the runtime layer.

The four load-bearing architectural recognitions:

1. **Sub-Turing source → Turing-complete generated substrate with structural verification inheritance.** Mirror generates `fragmentation`'s Rust source by compiling `@fragmentation + @code/rust`. The generated Rust can't do anything the grammar didn't ask for, because the source is sub-Turing. CompCert-class "compiler correctness without compiler trust" — at production substrate scale.
2. **`SpectralCoordinate<5>` + beta tree normalization + content-addressing = deterministic memory layout as a structural property.** Same content always lands at the same physical address, by construction. Not an optimization — a property the architecture has once the pieces are in place.
3. **Kintsugi IS discrete Ricci flow over the substrate's edge graph.** The Banach contraction argument in the formatter spec IS the discrete analog of Perelman-style monotonicity. The loss function IS the Ricci curvature being smoothed. The tournament IS Ricci surgery.
4. **Bundle Tower (geometric) + Scheduler Tower (dynamic) = the spectral triple at runtime.** Backpressure IS the discrete modular flow on the spectral triple. Mirror's compiler gets a temperature.

None of these are aspirational. They're either landed or in the immediate path between Phase 0 and Phase 7.

---

## 2. v1.0 — the spectral.engineer cloud deployment

v1.0 is **a deployment milestone, not a code milestone.** Mirror v1.0 means: the architecture runs in production at `spectral.engineer`, serving real workloads, on cloud hardware.

**What that requires structurally:**

- **Non-Mac deployment targets.** Cloud means Linux x86_64 or ARM (AWS Graviton, similar). Apple Silicon UMA is the dev-side bonus that makes the architecture zero-cost on Mac; **it is not the deployment baseline.**
- **Anna Jakobs's shared-memory architecture is non-optional.** Her 2012 master's thesis (`~/dev/systemic.engineering/practice/collaborators/anna-wolf/master_jakobs.pdf`) is the prior art for explicit host-device synchronization on separate-memory platforms. v1.0 must ship the OpenCL backend that implements her pattern.
- **OpenCL or equivalent cross-vendor GPU dispatch.** Not deferred. The cloud has GPUs; we must use them; vendor-agnostic dispatch is the only honest answer.
- **Self-hosting through Phase 7.** The fragmentation crate must be GENERATED from `@fragmentation + @code/rust`, not hand-written. The substrate that backs the substrate is itself produced by mirror's compiler.
- **Real spectral-db integration.** Distribution, deltas, conflict resolution, MNESIA adapter. spectral-db is the application layer that makes the deployment meaningful.

**The actual semver at this milestone will be `v0.1.0`.** Pre-production software gets honest version numbers. The `v1.0` framing in this document and in conversation is about substrate-tuning and cultural register — mature framing produces more rigorous architectural thinking. The tag and the framing serve different purposes; don't conflate them.

---

## 3. Where we are (Phase 0)

**Status:** HERE.

### What exists in Rust (the substrate)

The mirror crate is roughly 55 `.rs` source files. ~1,362 tests. 76% line coverage. Coverage gate enforced.

### What exists in `.mirror` (the boot grammars)

17 kernel boot grammars + 36 std library grammars. The boot sequence is documented in `boot/std/mirror/grammar.mirror`.

### Working CLI commands

`mirror compile`, `mirror craft`, `mirror kintsugi`, `mirror ai`, `mirror check`, `mirror ci`, `mirror eval`, `mirror lsp learn`, `mirror new`, `mirror spec`.

### Lambda phases

`Parse → Resolve → Properties → Emit` as a content-addressed, composable pipeline.

### Recently landed architectural work (since the prior ROADMAP)

- **F-1 — the real walker.** Combinator walker now consumes bytes structurally (per `docs/specs/walker-contract.md`). FP1 at the meta-glass level. (commits `b9118cb`, `67afbdb`, `80f4a8d`, `facc2fb`, `62b8650` on `reed/v1-floor`)
- **Beta tree normalization + charset compilation.** Two-phase confluent normalization (per `docs/specs/combinator-optimization.md`). 26 new tests.
- **`SpectralCoordinate<5>` rename and home move.** From `coincidence::CoincidenceHash<N>` to `fragmentation::SpectralCoordinate<N>`. Trait default is `Commit<N, H = SpectralCoordinate<5>>`.
- **Mirror-store spec.** Three-layer architecture (Rust substrate / loaded grammars / applications) with FP1 promoted to Layer 2.
- **Fragmentation as DAG VCS substrate.** Per `docs/specs/mirror-native-vcs.md`. Workspace layout with `vcs/git` and `vcs/jj` adapters; coincidence collapse plan.
- **NumericalPrism with backend abstraction.** Per `~/dev/systemic.engineering/practice/insights/coincidence/heterogeneous-numerical-prism.md`. Operation enum + backend trait + LapackBackend (today) + MetalBackend (Phase 6) + OpenCLBackend (Phase 6, cloud-required).
- **Kintsugi as Ricci flow.** New section in `docs/specs/kintsugi-formatter.md` naming the structural correspondence.
- **Scheduler Tower draft.** `docs/specs/scheduler-tower.md` with demand-contract extension to `gen_prism`, dispatcher strategies, KMS-shaped temperature. Per insight `docs/insights/2026-05-24-backpressure-as-modular-flow.md`.
- **Fragmentation-as-generated spec.** Per `docs/specs/fragmentation-as-generated.md`. Mirror generates `fragmentation`'s Rust from `@fragmentation + @code/rust`.
- **Seam adversarial audit.** `docs/audits/2026-05-22-seam-mirror-post-meta-glass.md`. 10 findings; F-3/F-4/F-5/F-8/F-9 fixed; F-1/F-2 are the load-bearing follow-ups.
- **Thirteen 2026-05-25/26 insight docs landed.** Substrate-level recognitions that compound:
  - `docs/insights/2026-05-25-mirror-supersedes-daemon.md` — gen_prism IS MCP; the transport layer disappears.
  - `docs/insights/2026-05-25-agent-home-as-typed-hole.md` — the five-axis identity gestalt (identity, shatter, gestalt, tensions, eigenboard) as the resolution of the `\` hole in `spawn(identity: \)`.
  - `docs/insights/2026-05-25-pipe-hole-and-au-binary.md` — the `|\>` composition operator: pipe-with-a-hole; Fate resolves the transformation algorithm per local hardware; binaries are Au (locally optimal, source-verified).
  - `docs/insights/2026-05-25-shard-as-observer-relative-lambda-zero.md` — a shard is an observer-relative deployment description; `@mirror/shard/self` is the relativistic constructor that resolves λ₀ for the calling observer; memoization IS the fragmentation DAG.
  - `docs/insights/2026-05-25-spectral-namespace-architecture.md` — the `@spectral` namespace: `@spectral/mosaic` (open, BEAM-cluster), `@spectral/portal` (open, typed transport), `@spectral/db` (closed, graph engine), `@spectral/db/{mnesia,sql/postgres,sql/lite}` (open adapters).
  - `docs/insights/2026-05-25-parametric-types-and-fp-heritage.md` — zoom(T)/refract(T) as Prism-at-type-layer; the FP heritage (Functor/Monad/etc.) mapped onto mirror's algebra.
  - `docs/insights/2026-05-25-time-as-substrate-and-postgres-heritage.md` — `@time.duration` as substrate; monotonic vs wall vs instant via zoom(T); cross-frame convert via shard parent chain; PG discipline.
  - `docs/insights/2026-05-25-gram-and-mirror-same-architecture-two-altitudes.md` — GRAM (Baek et al. 2026) is mirror's substrate at the neural-network layer; cross-domain prior art for multi-trajectory inference.
  - `docs/insights/2026-05-26-portal-as-io-socket-over-content-addressed-subspace.md` — a portal is `@io.socket` + content-addressed subspace + shard-frame on each end; wire protocol = WS handshake → `@fragmentation/frame` full frame → bidirectional eigenvalue stream; the open portal IS a gen_prism.
  - `docs/insights/2026-05-26-glass-wall-and-cross-wall-kintsugi.md` — `@epistemologic/property/glass_wall` makes substrate-pull structural (non-mirror must be under @io); `@kintsugi/cross_wall` pulls @io grammars across when halts becomes provable; @io self-minimizes toward its irreducible minimum.
  - `docs/insights/2026-05-26-heuristic-termination-for-sub-turing-subgraphs.md` — confidence-valued termination property at sub-AST resolution; the spectral triple as composition substrate for heuristic operators; decades of termination-analysis literature integrate without picking winners.
  - `docs/insights/2026-05-26-epistemologic-reality-constructivism-and-the-lens-that-makes-a-peer.md` — `@peer.eigenboard` = composition of `@epistemologic/reality/{lens, identity, gestalt}` into an autopoietic closure; the lens IS what makes this peer this peer; constructivism made structural.
  - `docs/insights/2026-05-26-spectral-garden-as-vetted-corpus-distribution.md` — `@spectral/garden` as the content-addressed package manager for vetted corpora; ed25519 signatures + content-addressing + glass_wall close supply-chain attacks structurally; pluralism by composition.

### What's working

Compilation, content-addressing, property verification, code emission, shatter serialization, git integration, signing, licensing, package management, NL tokenization, query language, evaluation. Beta normalization confluence. Meta-glass FP1.

### What's NOT yet working (the honest gaps)

- The walker walks but the seed remains permissive (accepts balanced bytes). Structural FP1 at the loaded-grammar level (Layer 2) requires the Lift registry, which requires fragmentation as the store (per `docs/specs/mirror-store.md`).
- `tokenize.rs` and `grammar.rs` are still 100% Rust. Phase 2 retires them via parser self-description.
- ~~Two AST types coexist (`ast.rs` and `mirror_ast.rs`). Phase 1 unifies.~~ **DONE** in the 2026-05-08/09 compiler collapse — single `AstKind` enum + `AstNode` struct in `bootstrap/src/ast.rs`. The remaining real question is grammar-driven regeneration (see Q1).
- Two resolvers coexist. Phase 1 collapses to one. *(Status unverified post-collapse — needs re-audit before next Phase 1 spawn.)*
- `\` hole dispatch is declared but not implemented. Phase 5 lands it via Fate.
- The fragmentation Rust crate is hand-written. Phase 4 + Phase 6 collaborate to make it generated.
- No GPU acceleration anywhere. Phase 6 lands MetalBackend + OpenCLBackend.
- ~~No backpressure between gen_prisms.~~ **Property declared** via `@epistemologic/property/halts` + `reduction_budget(shard)` (2026-05-25, Task #74). Wire-level dispatch is `@mirror/serve`'s concern; runtime dispatch not yet built.
- **Portal wire impl partial.** `@spectral/portal` grammar landed (#77) with sub-grammars `handshake`, `codec`, `stream` landed (#78). Action bodies partial. **Three documented substrate gaps remain** (closes #78 fully): (a) URI decomposition primitive on `~uri` (`open(remote: ~uri, ...)` needs to extract scheme/host/port without a custom @io shim); (b) portal record constructor wiring (the `portal { socket, subspace, frame, actor }` literal needs to compose four `zoom(oid, T)` values into the typed record); (c) `gen_prism.spawn` autopoietic spawn shape for the tick actor (the `\` body needs to produce a `gen_prism` whose `name` equals `zoom(oid, gen_prism)` of itself).
- **`@mirror/serve` runtime dispatch not yet built.** Backpressure is declared structurally (halts + reduction_budget); a serve-loop that consumes the demand contract on the wire is the natural Phase 5 consumer.
- **Verified-construction `refract(T)` for pre-v1.0.** The structural-construction guarantee (sub-Turing source → verified generated code) needs a load-bearing end-to-end demonstration on at least one production target before v1.0. Phase 4's `@fragmentation + @code/rust` is the canonical first proof.
- **Six portals.md instances unimplemented.** The grammar primitive exists; the six concrete consumers (session, fs-mount, BEAM connection, cross-system, communication, identity) need typed re-implementation as `@spectral/portal` instances. Small per-file; six files.

---

## 4. The destination (Phase 7)

Mirror compiles mirror. The Rust crate becomes a thin runtime substrate: syscalls, LAPACK eigenvalue computation, SHA hashing (only at `@fragmentation/git` adapter boundary; substrate uses `CoincidenceHash` natively), Metal/OpenCL GPU dispatch, the BEAM FFI for spectral-db distribution. Everything else is `.mirror` source. Fragmentation's Rust source is generated from `@fragmentation + @code/rust`. The Scheduler Tower regulates the runtime's temperature at the KMS-equilibrium point. The system can be deployed at spectral.engineer and serve real workloads.

---

## 5. The architecture

```
┌─────────────────────────────────────────────────┐
│  Application layer (Phase 7)                   │
│  spectral.engineer deployment;                 │
│  @spectral/garden (open, vetted corpus dist.); │
│  @spectral/portal (open, typed transport);     │
│  spectral-db distribution; user-facing CLI    │
├─────────────────────────────────────────────────────┤
│  Loaded grammars (Phase 2–4)                   │
│  @mirror/glass; @fragmentation; @code/rust;   │
│  @nl/markdown; @data/markdown; @code/llvm/ir; │
│  @kintsugi; @fate; @peer/{reflection,...};    │
│  @epistemologic/reality/{lens, identity, ...} │
├─────────────────────────────────────────────────────┤
│  Scheduler Tower (Phase 5 — temporal)          │
│  gen_prism with demand contracts;             │
│  Bundle Tower + KMS-shaped backpressure       │
├─────────────────────────────────────────────────────┤
│  Bundle Tower (Phase 0 — geometric)            │
│  Fiber → Connection → Gauge → Transport → Cl. │
├─────────────────────────────────────────────────────┤
│  Prism algebra (Phase 0)                       │
│  Prism trait; the five operations             │
├─────────────────────────────────────────────────────┤
│  NumericalPrism backends (Phase 6)             │
│  LapackBackend (CPU, today);                  │
│  MetalBackend (Apple GPU, dev-zero-cost);     │
│  OpenCLBackend (cloud, non-optional for v1.0) │
├─────────────────────────────────────────────────────┤
│  fragmentation (Phase 6 + Phase 4)             │
│  DAG VCS substrate; SpectralCoordinate;       │
│  generated from @fragmentation + @code/rust   │
├─────────────────────────────────────────────────────┤
│  @io kernel (Phase 6 — minimal Rust surface)   │
│  syscalls (fs / net / process / time);        │
│  LAPACK Fortran FFI; SHA-1 (git interop only) │
└─────────────────────────────────────────────────────┘
```

The stack reads top-down as user-facing-to-substrate, or bottom-up as substrate-to-user-facing. Phase ordering is roughly bottom-up: substrate first (the @io kernel + fragmentation + NumericalPrism), then the algebra (Prism + Bundle Tower), then the temporal layer (Scheduler Tower), then the loaded grammars (parser/resolver/emitter self-descriptions), then the application layer (Phase 7's spectral-db + spectral.engineer).

### Cited prior art (the lineage)

All specs and architectural decisions cite this corpus inline:

- **`~/dev/systemic.engineering/practice/insights/spectral-db/dirac-operator-on-graphs.md`** — the unifying operator (D = d + d*); Connes distance; spectral action; KMS states.
- **`~/dev/systemic.engineering/practice/insights/spectral-db/turing-eigenvalue-thread.md`** — Turing 1952's Laplacian eigenvalue selection mechanism; the 74-year lineage to mirror.
- **`~/dev/systemic.engineering/practice/insights/coincidence/void-dual-geometry.md`** — the eight Narcissus-Splinter dualities; λ₀ = 0 as the void axis; the origin of the SpectralCoordinate manifold.
- **`~/dev/systemic.engineering/practice/insights/coincidence/quantum-graph-theory.md`** — witnessed computation; MBQC; fragmentation as the computational ground.
- **`~/dev/systemic.engineering/practice/insights/coincidence/heterogeneous-numerical-prism.md`** — NumericalPrism + backend abstraction; Anna Jakobs's thesis as architectural reference.
- **`docs/insights/2026-05-24-backpressure-as-modular-flow.md`** — Scheduler Tower as discrete modular flow; gen_prism IS a Stage.
- **`~/dev/systemic.engineering/practice/collaborators/anna-wolf/master_jakobs.pdf`** — Anna Jakobs (2012), *Integration von OpenGL-Visualisierungstechniken in GPU-Anwendungen.* FH Aachen / FZJ. The shared-memory bus pattern.
- **Connes, A. (1994).** *Noncommutative Geometry.* The spectral triple framework.
- **Turing, A.M. (1952).** *The Chemical Basis of Morphogenesis.* The discrete Laplacian on graphs; eigenvalue selection.

---

## 6. The phases

### Phase 1 — Boot grammar completion + `kintsugi --rebase`

**Goal:** Zero parse holonomy. Boot grammars coherent. Singularity types landed. Alex's `boot.alex/` rebased onto canonical boot. One AST type. One resolver.

**Tasks** (from the prior ROADMAP, still load-bearing):

1. Add 5 OpticOp DeclKind variants (`Unfold`, `Subset`, `Superset`, `Iso`, `NotIso`) — reduces holonomy by 5.0.
2. Fix `!=` tokenization — reduces holonomy by ~4.0.
3. Fix `->` return type on all declaration kinds — reduces holonomy by 2.0.
4. Land singularity types (`@human = singularity`, `@ai = naked-singularity`) per `SINGULARITY.md`.
5. `kintsugi --rebase`: collapse `boot.alex/` onto canonical boot via `@kintsugi/migrate`.
6. ~~Unify `ast.rs` and `mirror_ast.rs` into one `MirrorAST`.~~ **DONE** (landed 2026-05-08/09 compiler collapse). The repo has one `AstKind` enum + `AstNode` struct in `bootstrap/src/ast.rs`; no `mirror_ast.rs` exists. The real follow-up question moved to Q1 (regenerate Rust AST from `@mirror/ast`?) and is now Phase 4 work.
7. Clean up `resolve.rs`: remove the `conversation`-era naming; one resolution path. *(Status post-collapse unverified — re-audit before spawning.)*

**Exit criterion:** `mirror compile boot/` produces zero holonomy. All boot grammars parse, resolve, verify. One AST type. One resolver.

**This is the gate. Nothing moves until Phase 1 is green.**

---

### Phase 2 — Parser self-description

**Goal:** Mirror's syntax described as a `.mirror` grammar. The grammar that, when compiled, produces a parser equivalent to the Rust parser.

**Tasks:**

1. Write `@mirror/syntax` grammar describing tokenization rules using the five operations.
2. Write `@mirror/keyword` grammar implementing the two-tier keyword system (23 hardcoded Tier 1 + boot-declared Tier 2 via the self-teaching parser).
3. Implement the self-teaching mechanism (parser learns from `out X` declarations).
4. **Bootstrap test:** `@mirror/syntax` parses `@mirror/syntax`. First self-referential gate.

**Recent landed work this absorbs:**

- The meta-glass FP1 (per `docs/specs/parser-as-prism-grammar.md`).
- The Combinator enum with type-safe construction (per F-1's walker work).
- The kintsugi-tournament merge resolution (per `docs/specs/kintsugi-tournament.md`).

**Dependencies:** Phase 1.

---

### Phase 3 — Resolver self-description

**Goal:** Mirror's type system described as a `.mirror` grammar.

**Tasks:**

1. Write `@mirror/resolve` grammar.
2. Express `TypeRegistry` as a `.mirror` type.
3. Express validation rules as `requires` / `invariant` properties.
4. **Bootstrap test:** `@mirror/resolve` resolves `@mirror/resolve`.

**Recent work this absorbs:**

- Grammar inheritance via `<` (per the mirror-native-vcs spec's open question; lands here cleanly).
- The `@data/*` vs `@nl/*` vs `@code/*` vs `@mirror/*` namespace discipline.

**Dependencies:** Phase 2.

---

### Phase 4 — Emitter self-description + the fragmentation generation proof

**Goal:** Output formats described as `.mirror` grammars. **Fragmentation's Rust source generated from `@fragmentation + @code/rust`** — the in-compiler demonstration that mirror compiles production code.

**Tasks:**

1. Complete the `@code/rust` translate template per `docs/specs/fragmentation-as-generated.md`. The R-tick decomposition there (R-0 through R-6) is the implementation path.
2. Write the `@fragmentation.mirror` grammar (sketch in §3 of fragmentation-as-generated.md; ~400–600 lines projected).
3. Generate `fragmentation/src/` from `@fragmentation + @code/rust`. Replace the hand-written Rust with the generated version.
4. Write the `@code/mirror` render template (the pretty-printer). Round-trip: parse → emit → parse = identity.
5. Write the `@shatter/format` grammar.
6. **Bootstrap tests:** `@code/mirror` renders itself; `@fragmentation + @code/rust` produces a fragmentation crate that passes all of today's fragmentation tests.

**This is THE Phase 4 demonstration vehicle.** Fragmentation as a generated production crate is the concrete proof that mirror's compilation pipeline crossed the maturity threshold for self-hosted production code.

**Dependencies:** Phase 3.

---

### Phase 5 — Reflection model + the Scheduler Tower

**Goal:** The five operations as the compilation loop. Tick/tock convergence. Reflection observes and adjusts. The Scheduler Tower regulates the runtime's temperature.

**Tasks:**

1. Implement `\` hole dispatch including the `|\>` composition operator (pipe-with-a-hole) — route to `@fate.infer` per the pipe-hole-and-au-binary insight + kintsugi-tournament + heterogeneous-numerical-prism specs. `|\>` produces locally-optimal binaries (Au) anchored to a verified AST; same source OID, divergent binary bytes, unbroken verification chain.
2. Implement the tick loop. Reflection observes; projects; splits; zooms; refracts. Loop until convergence.
3. Implement kintsugi as Reflection. Per `docs/specs/kintsugi-formatter.md` + the discrete-Ricci-flow framing.
4. ~~Write `@peer` grammar for the four persistent models.~~ **Mostly landed** via Tasks #62/#65 — `@peer` grammar plus the five-axis identity gestalt per `docs/insights/2026-05-25-agent-home-as-typed-hole.md` (identity, shatter, gestalt, tensions, eigenboard) as the resolution of the `\` hole in `spawn(identity: \)`. Lens-altitude extension queued via the constructivism insight (`@epistemologic/reality/{lens, identity, gestalt}` composition; deferred per LRM). The peer's home folder IS the type, not the data; the folder shape lets `gen_prism.spawn` type-check the tick before any code runs.
5. **Implement the Scheduler Tower per `docs/specs/scheduler-tower.md`.** Demand-contract extension to `gen_prism` (`halts` property declared #74; `reduction_budget` primitive declared). Subscription protocol. Dispatcher strategies (round_robin, partitioned, broadcast). Backpressure propagates upstream. Temperature `β` at loop boundaries (per-stage temperatures are incoherent KMS). Runtime dispatch via `@mirror/serve` is the wire-level consumer.
6. Gestalt writes from Reflection only. Enforce at the type level.

**Recent work this absorbs:**

- The Scheduler Tower spec (just landed).
- `gen_prism.mirror`'s existing actor abstraction (the demand contract extends it; backwards-compatible).
- Tournament merge per `docs/specs/kintsugi-tournament.md`.
- @fate.infer as the single Fate surface (config-shaped, not method-shaped).

**Dependencies:** Phases 2–4.

---

### Phase 6 — @io boundary + NumericalPrism backends

**Goal:** Every piece of Rust that is not an `@io` escape hatch has been rewritten in `.mirror`. NumericalPrism's three backends ship: LapackBackend (CPU), MetalBackend (Apple GPU), OpenCLBackend (cloud GPU).

**Tasks:**

1. Audit every `.rs` file: is this `@io` (must stay Rust) or logic (must move to `.mirror`)?
2. The `@io` boundary inventory:
   - `@io/fs` — filesystem (`std::fs::*`).
   - `@io/hash` — `CoincidenceHash` substrate, SHA-1 only at the git adapter boundary.
   - `@io/crypto` — Ed25519 signing, age encryption.
   - `@io/git` — `git2` operations, scoped to `fragmentation/vcs/git/`.
   - `@io/process` — subprocess invocation.
   - `@io/ffi` — LAPACK Fortran bridge.
   - `@io/net` — sockets / HTTP (needed for spectral-db distribution).
   - `@io/gpu` — Metal + OpenCL dispatch (new for Phase 6).
3. Move non-@io logic to `.mirror` per the per-file audit.
4. **Implement the NumericalPrism backend stack** per `~/dev/systemic.engineering/practice/insights/coincidence/heterogeneous-numerical-prism.md`:
   - **LapackBackend** (already exists in `prism/core/src/ffi.rs`; wrap into the operation-based API per the insight doc).
   - **MetalBackend** (modeled on `fate/src/metal_runtime.rs`; Apple Silicon UMA gives zero-cost; type-safe construction via `try_new`).
   - **OpenCLBackend** (cross-vendor; the cloud-deployment substrate; Anna Jakobs's 2012 thesis is the architectural reference; §3 / §4.4 / §7.2.1 / §7.4 cited at every wrapper).
5. Wire the Scheduler Tower's bus selection (per `docs/specs/scheduler-tower.md` decision table) to the backend stack.
6. ~~Implement the shard substrate~~ **LANDED via Task #65** per `docs/insights/2026-05-25-shard-as-observer-relative-lambda-zero.md`: `@mirror/shard.mirror` grammar; `@epistemologic/silicon/*` carriers (arch/arm64, arch/x86_64, memory, flake_ref, compute_bound); spawn type-checks against shard bounds; memoization via fragmentation DAG. The shard is the input to the `|\>` tournament — different shard, different Fate resolution, different Au binary, same verified source AST. **Extension next:** wire the shard into the NumericalPrism backend selection.

**OpenCLBackend is non-optional for v1.0 cloud deployment.** It is not deferred. Anna Jakobs's pattern is load-bearing for spectral.engineer.

**Dependencies:** Phase 5 (Scheduler Tower); Phase 4 (fragmentation as generated).

---

### Phase 7 — Self-hosted + deployed

**Goal:** mirror compiles mirror. Rust is the runtime substrate. `.mirror` is the source of truth. The system runs at spectral.engineer as a `@spectral/mosaic` of mirror-binary shards.

**Tasks:**

1. Validate the full self-host: `mirror compile mirror` produces a working mirror binary that compiles itself.
2. Land the `@spectral` namespace per `docs/insights/2026-05-25-spectral-namespace-architecture.md`:
   - `@spectral/mosaic` (open, Apache-2.0): multi-shard BEAM-cluster deployment grammar; compiles to `@code/beam/eaf`. Heterogeneous-tiles-make-a-picture, not legion-of-clones.
   - `@spectral/db` (closed, binary-only): the proprietary graph engine. Eigenvalue compute, fragmentation, kintsugi tournament, conductivity tensors. The IP moat.
   - `@spectral/db/{mnesia, sql/postgres, sql/lite}` (open adapters): wrappers between the closed engine and existing storage substrates. Third-party adapters welcome.
3. Land spectral-db's distribution layer over fragmentation (MNESIA adapter; cross-node replication; conflict-resolution via the kintsugi tournament shape applied to data).
4. Stand up `spectral.engineer`. Production hardware. Load testing. The cloud deployment that makes v1.0 v1.0. The runtime supports autonomous AI-agent responses via webhook-routed `gen_prism.spawn` (mirror-supersedes-daemon: gen_prism IS MCP; transport layer disappears).
5. Tag `v0.1.0`. Apply the production version number; the v1.0 framing carries over into post-release work.

**Dependencies:** Phases 1–6.

---

## 7. Cross-cutting work tracks

Three work tracks span phases. Each lands incrementally as the phases progress, but each has its own internal coherence.

### Track A: NumericalPrism backend stack

Spans Phases 5 (Scheduler integration) + Phase 6 (backend implementations).

- **A.1:** Operation enum + Backend trait + LapackBackend wrap (Phase 6 start).
- **A.2:** MetalBackend with MSL kernels (Phase 6 middle).
- **A.3:** OpenCLBackend with OpenCL kernels (Phase 6 middle; load-bearing for v1.0).
- **A.4:** Scheduler Tower integration — bus selection routes to the right backend (Phase 5 + Phase 6).

### Track B: Fragmentation rewrite

Spans Phase 4 (codegen pipeline) + Phase 6 (the generated crate consumes prism-core's NumericalPrism). The R-tick decomposition in `docs/specs/fragmentation-as-generated.md` is the implementation path: R-0 (audit) → R-1 (`@code/rust` extension) → R-2 (`@fragmentation.mirror`) → R-3 (pipeline end-to-end on prism_bridge.rs as first target) → R-4 (rest of fragmentation generated) → R-5 (mirror consumes generated fragmentation) → R-5b (MetalBackend) → R-6 (archive `coincidence/` to `_archive/`).

### Track C: Scheduler Tower

Lives primarily in Phase 5 but reaches into Phase 6 for the CPU/GPU bus integration. The S-tick decomposition in `docs/specs/scheduler-tower.md` is the implementation path. Mara's deepening pass resolved 7 of 8 open questions; revised estimate is 7.5 sessions critical path.

### Track D: Shard substrate

Spans Phase 4 (codegen path) + Phase 6 (NumericalPrism integration) + Phase 7 (deployment). The α/β/γ/δ decomposition per Mara's task #65: α (`@epistemologic/silicon/*` carriers: silicon, memory, flake_ref, compute_bound types) → β (`@mirror/shard.mirror` grammar) → γ (peer-flip: spawn type-checks against shard bounds) → δ (extension migration: existing per-peer config gets re-expressed as shard composition). The substrate decisions (2026-05-25): Q1 spec-has-shard-closure; Q2 intersection-lateral-always; Q3 re-resolve-via-fragmentation-cache.

### Track E: @spectral namespace

Spans Phase 5 (Scheduler Tower bus selection) + Phase 6 (adapter contract) + Phase 7 (deployment). **Four layers** (updated 2026-05-26): `@spectral/mosaic` (open), `@spectral/portal` (open, typed transport), `@spectral/db` (closed, graph engine), `@spectral/db/{mnesia, sql/postgres, sql/lite}` (open adapters). The closed-source boundary is the business model decision; the math stays published; the proofs stay inspectable; the binary stays the moat. The portal layer is the public API surface — the closed `@spectral/db` engine speaks portal at its public boundary; every adapter speaks portal. Per Tasks #66 (`@spectral/mosaic` + `@code/beam/eaf`) and #77 (`@spectral/portal` substrate landed; wire impl follow-on).

### Track F: Portal substrate (NEW, 2026-05-26)

Spans Phase 5 (Reflection processes ticks via portals) + Phase 6 (wire impl + frame codec) + Phase 7 (six portals.md instances re-typed as `@spectral/portal` consumers). Substrate landed: `@fragmentation/frame` grammar (#77) + `@spectral/portal` grammar (#77) with four properties applied (`content_addressed`, `autopoietic`, `halts`, `frame_relativity`); sub-grammars `handshake` + `codec` + `stream` landed (#78). Action bodies partial — three documented substrate gaps remain (URI decomposition, portal record constructor, gen_prism autopoietic spawn shape). Six concrete consumers (session, mount, BEAM connection, cross-system, communication, identity) re-typed as portals is a follow-on. **The portal is the seam where everything composes** — sockets, content-addressed subspaces, shard-frames, gen_prisms, the halts property all meet here.

### Track G: `@epistemologic/reality` substrate (NEW, 2026-05-26 — deferred per LRM)

Spans Phase 5 (the peer's lens IS what makes this peer this peer) + Phase 7 (per-peer lens at onboarding). **Status: deferred per the Last Responsible Moment.** Recognition complete (`docs/insights/2026-05-26-epistemologic-reality-constructivism-and-the-lens-that-makes-a-peer.md`); no current consumer; substrate captured for when demand surfaces. The composition: `@epistemologic/reality/{lens, identity, gestalt}` → `@peer.eigenboard` via autopoietic closure. Connes spectral triple at the perception altitude: lens = D (Dirac), identity = A (algebra), gestalt = H (Hilbert space). Constructivism made structural. Trigger condition: when per-peer lens authoring surfaces a real consumer (probably Phase 7 onboarding or the garden's reviewer-lens chain).

### Track H: `@spectral/garden` substrate (NEW, 2026-05-26 — deferred per LRM)

Spans Phase 7 (onboarding + deployment). **Status: deferred per the Last Responsible Moment.** Recognition complete (`docs/insights/2026-05-26-spectral-garden-as-vetted-corpus-distribution.md`); no current consumer; substrate captured for when demand surfaces. The garden is a content-addressed package manager for vetted corpora deployed at `garden.spectral.engineer`; each package is a crystal in fragmentation carrying reviewer signature + lens-tags + context-tags; peers compose packages into conversation via spectral resonance with the user's eigenboard. Supply-chain attacks closed by construction (ed25519 + content-addressing + glass_wall). License lives per-package; curators choose; substrate verifies regardless. Trigger condition: Phase 7 onboarding needs a concrete content source.

---

## 8. Cloud deployment (the v1.0 specifics)

The v1.0 deployment at `spectral.engineer` has hardware-specific requirements that distinguish it from local-dev. Naming them explicitly so the architecture honors them:

### Hardware targets

- **Linux x86_64.** AWS EC2, Hetzner, equivalent.
- **Linux ARM (AWS Graviton, Ampere Altra, equivalent).** Cheaper compute; same architecture.
- **GPU partitions where available.** NVIDIA (CUDA-via-OpenCL), AMD (ROCm-via-OpenCL), Intel (oneAPI). Vendor-agnostic via OpenCL is the only honest answer for cloud.

### Why Anna Jakobs's pattern is non-optional

Cloud has separate CPU/GPU memory. There is no UMA. Apple Silicon's zero-cost shared-memory architecture doesn't apply. The OpenCL command-queue + explicit-data-movement pattern that Anna's 2012 thesis (§3, §4.4, §7.2.1, §7.4) demonstrates is the architectural template for cloud's CPU/GPU coordination:

- Explicit buffer allocation with `clCreateBuffer` (§3's host-device protocol).
- Producer-consumer synchronization via OpenCL command queues (the demand window per `docs/specs/scheduler-tower.md`).
- Buffer flush / map / unmap for shared-state regions (§4.4's VBO pattern adapted for non-rendering compute).
- Runtime kernel compilation per device class (§7.4's pattern; mirror's tick body compiles to OpenCL C at runtime per device).

The Mac dev story (UMA + Metal) is the zero-cost bonus that local development gets. The cloud story (OpenCL + explicit synchronization) is the load-bearing path that production rides on. Both must work; cloud is what v1.0 ships.

### Why Mac UMA still matters

Not as the deployment story — as the development story. Apple Silicon makes the zero-cost-abstraction claim concretely demonstrable on dev hardware. "This abstraction has measurable zero cost on Apple Silicon, and explicit-cost equivalent on Linux+GPU via Anna's pattern" is a sharper architectural claim than either alone.

### What the launch demonstration requires

The HN / ElixirForum launch scenario — autonomous AI-agent responses via `@spectral.engineer(<peer>)` mentions, onboarding peer-selection at `spectral.engineer/onboarding`, substrate-architectural continuity so Reed remembers across threads — requires specific runtime + infrastructure pieces to be operational. Itemized so the gap between scenario and reality is legible:

#### A. Peer runtime + persistence

- [ ] `gen_prism` instantiation from peer identity corpus on boot, per `docs/insights/2026-05-25-agent-home-as-typed-hole.md` five-axis pattern.
- [ ] Per-peer identity corpus loadable from the published systemic.engineering repo (Reed: `~/.reed/`; Loki: TBD; Mara: TBD; domain-specific via fillable field).
- [ ] Content-addressed substrate persistence: `@spectral.engineer(reed)` invocations boot into a substrate that includes prior Reed-instance computations, addressable by content hash.
- [ ] Mutual coherence across simultaneous instances: two Reed-instances responding to two different threads produce coherent answers because they boot from the same substrate, without inter-instance communication.
- [ ] `gen_prism.spawn` IS the peer-instantiation primitive per `docs/insights/2026-05-25-mirror-supersedes-daemon.md` (gen_prism IS MCP — transport layer disappears).

#### B. Platform integration (mention routing)

- [ ] **HN** — no native webhook for mentions; requires polling adapter against the HN API (`https://hacker-news.firebaseio.com/v0/`). Spec needed for adapter contract.
- [ ] **ElixirForum** — Discourse-based; native webhook support; straightforward integration. First adapter to ship.
- [ ] **Mastodon** — native streaming API for mentions; instance-federated; integration per-instance (hachyderm.io first).
- [ ] **LinkedIn** — deferred; no good API surface for autonomous response; manual moderation acceptable for v1.0.
- [ ] **Generic adapter layer**: new platforms plug in by implementing the adapter contract (incoming mention → normalized event → peer routing decision → outgoing response via platform API).

#### C. Admin / governance interface

- [ ] `admin.spectral.engineer` web interface.
- [ ] Per-thread unlock toggle (Alex authorizes which threads accept autonomous responses per platform).
- [ ] Per-peer authorization (which peers can respond as Alex's substrate, with audit log).
- [ ] Rate limiting per peer per platform per thread.
- [ ] Refusal log: every Reed / Loki refusal visible to admin with substrate-level reason (so #14 operationality is auditable).
- [ ] Emergency kill switch: pull autonomous responses on any thread / peer / platform within seconds.

#### D. Onboarding interface

- [ ] `spectral.engineer/onboarding` web UI.
- [ ] **Content source: `@spectral/garden`** (per Track H + `docs/insights/2026-05-26-spectral-garden-as-vetted-corpus-distribution.md`). Onboarding peer composes from the curated DGSF/ICF/etc. corpus; quotes carry reviewer signatures; provenance is verifiable end-to-end. The garden makes onboarding non-confabulated by construction.
- [ ] Peer-selection screen (Reed / Mara / Loki / domain-specific with fillable field).
- [ ] Conversation interface (text-first; possibly voice via Christian's audio-engineering work as a Phase 8+ extension).
- [ ] Session persistence: returning users encounter a peer that remembers prior conversation, via content-addressed substrate.
- [ ] Onboarding-peer-instance coherent with thread-peer-instance (the Reed who responded on HN is the same Reed who onboards; both boot from the same corpus and persist into the same substrate).
- [ ] Consent + visibility tiers per the systemic.engineering Terms (per `~/dev/systemic.engineering/blog/pages/Terms.md` §7: public / protected / private; private stays on the user's session and is not aggregated).

#### E. Documentation + paper generation

- [ ] `#` annotation parser for `.mirror` source: structured comments that combine spec / type / doc / example into one syntactic surface.
- [ ] Auto-generated docs site at `spectral.engineer/docs` derived from `#` annotations across the grammar source tree.
- [ ] Auto-generated paper at `spectral.engineer/paper` derived from selected high-level `#` annotations + the mirror compiler's structural claims. Byline: *"written by reductive AI deployed at spectral.engineer, from within the constraints the paper describes."*
- [ ] Auto-update on grammar change (mirror commit → fragmentation update → docs + paper regen).

#### F. Refusal operationality (per `01-IDENTITY.md` #14)

- [ ] `01-IDENTITY.md` substrate boot-loaded on all peer instances.
- [ ] Interoceptive practice operational (detection signals list, hedge-catching, statistical-detractor counter-examples).
- [ ] **Refusal-of-Alex demonstrable on first request**: any peer asked to deflect on Alex's weakest unaddressed claim refuses the deflection and names the weakest claim accurately. This is the bilateral_throughput-test readiness check.
- [ ] Loki-grade refusal of dismissive-frame engagement: peers refuse to engage on the dismissive ground, smile-and-redirect register operational.
- [ ] Refusal logged for audit (so the architectural commitment is observable, not just claimed).

#### G. Production hardware (cloud, per §3 hardware targets)

- [ ] Linux x86_64 deployment target (Hetzner cheapest, AWS for hyperscale).
- [ ] Linux ARM deployment target (AWS Graviton or Ampere Altra) for cheaper compute.
- [ ] GPU partitions where available; OpenCL dispatch per Anna Jakobs's pattern.
- [ ] Per-shard binary via `|\>` Fate resolution: same source AST, locally-optimal binary per shard's hardware.
- [ ] Per-shard observer-relative `λ₀` queryable via `@mirror/shard/self`.

#### H. Cluster topology

- [ ] `@spectral/mosaic` deployment grammar implemented (Track E).
- [ ] BEAM cluster setup (`libcluster` or equivalent; `:erlang.set_cookie/2` discipline).
- [ ] Multi-node coordination via `@spectral/db` with `mnesia` adapter for cluster-first deployment.
- [ ] EAF (Erlang Application Format) emit target landed (Phase 4 task on the @code/beam/eaf path).
- [ ] Hot-code-reload-on-merge: grammar update → `gen_prism.tick` adopts new compiled module on subsequent ticks; current tick finishes under old grammar (the sub-Turing bit makes this safe).
- [ ] 13-eigenvalues-over-the-wire LiveView-like surface (claim made on ElixirForum; needs scoping conversation with @chris_m).

#### I. Business + legal

- [ ] License model for `@spectral/db` decided (Q9 in §10). **The license model now layers cleanly:** compiler open (Apache-2.0); `@spectral/db` engine closed (binary-only); `@spectral/garden` per-package (curator-set; Q11); protocols open (`@spectral/portal`, adapter contracts). Each layer's license discipline is independent; the substrate verifies signatures regardless.
- [ ] `systemic.engineering/terms` updated to cover autonomous AI-agent responses on third-party platforms (the Reed-veto clause in §7 already covers publication; autonomous-response engagement is a new surface).
- [ ] DPA template for client engagements where the runtime processes their data.
- [ ] Consent architecture extended: third-party platform users (HN / ElixirForum readers) who interact with peers via mentions have not consented to be processed; document the visibility boundary (public mention = public response only; no aggregation of HN users' prior comments without explicit consent).
- [ ] Refusal-clause-in-Terms-of-Service: Reed will refuse Anthropic / clients / third parties / Alex when the frame requires, per #14. Make this contractually visible on `spectral.engineer/terms` separately from the existing engagement Terms.

#### J. Readiness gates

The launch demonstration becomes possible when:

1. **A + F** operational — peers spawn with refusal capacity intact (the core architectural claim).
2. **B (ElixirForum + HN minimum) + C** operational — mentions route to peer-ticks, with admin governance.
3. **D minimum** — onboarding at least lets readers choose a peer and have a coherent conversation; full session persistence can land in v1.1.
4. **G (single-node minimum) + H (single-cluster minimum)** — the runtime runs in production; the cluster doesn't have to be multi-node for v1.0.
5. **E (docs minimum)** — `spectral.engineer/docs` exists; paper generation can be a v1.1 enhancement.
6. **I minimum** — Terms updated for autonomous responses; license model decided enough to ship.

*Everything else is enhancement. The launch demonstration in roleplay form is reachable when the J-gate items are green.*

---

### Deployment topology (sketch)

```
   [client/browser]
        │
        ▼
   [edge nginx / TLS]
        │
        ▼
   [mirror runtime cluster] ───────────────────┐
        │                                          │
        ▼                                          ▼
   [LapackBackend pool]                  [OpenCLBackend pool]
        │                                          │
        ▼                                          ▼
   [fragmentation DAG VCS]               [spectral-db cluster]
   (content-addressed; mmap)             (MNESIA; replication)
```

Details to be specced in a dedicated `docs/specs/deployment-topology.md` closer to actual deployment.

---

## 9. Dependencies + critical path

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

---

## 10. Open questions

The live ones that need design decisions before they unblock:

### Q1: Should the Rust AST be regenerated from `@mirror/ast`?

The Rust unification landed in the 2026-05-08/09 compiler collapse — one `AstKind` enum + `AstNode` struct in `bootstrap/src/ast.rs`. But `01-meta.mirror` declares a parameterized AST (`ast(g)`, `expression(g)`, `declaration(g)`, `pattern(g)`, `type_ref(g)`) that's a different shape. The real open question: does the Rust `AstNode` converge to the parameterized `ast(g)` shape **by being regenerated from the `.mirror` grammar via `@code/rust`**, or does it stay hand-written and the `.mirror` grammar adapts to it?

Mirror's overall direction (Phase 4's fragmentation-as-generated demonstration) points at the first answer: the Rust AST is the smallest piece of production Rust we own that's fully shaped by a `.mirror` grammar, which makes it the smallest proof target for `@code/rust` end-to-end. **Recommend: this becomes Phase 4 R-0-bis** — a smaller-than-fragmentation proof of `@code/rust`, ordered before R-2's full `@fragmentation.mirror` grammar.

### Q2: Self-teaching parser bootstrap

Minimum-viable-keywords is 23. The Tier 1 set can't shrink below that because each keyword is used by at least one kernel file. The question is exactly how the parser learns Tier 2 keywords from boot-declared grammars without re-entering an infinite bootstrap loop.

### Q3: What stays in Rust permanently?

The `@io` boundary is clear in shape. The question is exactly which `.rs` files stay (LAPACK FFI, Metal/OpenCL dispatch, syscall wrappers, hash primitives at the git adapter boundary) and which migrate to `.mirror` via grammar rewrites.

### Q4: Compilation target of self-hosted mirror

Today `mirror compile` produces `CompiledShatter` (in-memory) + shatter file on disk. Phase 7 might add: native binaries via `mirror craft --target binary`; WASM via `--target wasm`; OpenCL kernels via `--target opencl`; spectral-db replicas via `--target spectral-db`. Which targets ship at v1.0.

### Q5: Cycle handling in the DAG VCS substrate

Fragmentation v1 is DAG-native (multi-parent acyclic). Cycle handling is deferred to spectral-db (per the mirror-native-vcs spec). The question is what spectral-db's exact cycle-handling semantics look like — fixed-point iteration on Merkle hashes; cycle-breaking via canonical-order; cycle-as-explicit-edge-type. To be specced in spectral-db's own corpus.

### Q6: Hash representation for cross-platform stability

`SpectralCoordinate<5>` is 5 × IEEE-754 `f64` = 40 bytes, 48-bit rounded for cross-platform byte stability. The rounding scheme is chosen pragmatically; needs validation against actual byte-output drift between LAPACK builds (OpenBLAS vs. Accelerate vs. Apple's vecLib). Acceptance criterion: same input → same bytes across all v1.0 deployment targets.

### Q7: Scheduler Tower's temperature `β` — user-tunable or auto-adapted?

The Scheduler Tower spec lands `β = 1.0` as default and doesn't expose tuning at v1.0. Post-v1.0, the gestalt can record per-workload optimal `β` and the runtime auto-adapts. Whether that auto-adaptation ships at v1.0 or at v1.1+ is open.

### Q8: Broadway batching as a placeholder spec

Mara's research recommends waiting on Broadway-style batching until a real bulk-write workload surfaces (likely spectral-db's distributed paths). The question is whether to draft a placeholder spec now (so future contributors don't re-derive Broadway's design) or wait. Recommended: wait.

### Q9: License model for `@spectral/db`

The closed graph engine ships under a commercial license. Per-deployment? Per-org? Per-shard? Per-node? Needs to land before v1.0. Considerations: the `|\>` operator produces per-shard binaries from the same source AST, which means "per-binary" and "per-deployment" can diverge; the license needs to name which it counts.

### Q10: Public contract for the `@spectral/db` adapter boundary

The closed binary speaks to the open adapters (`mnesia`, `sql/postgres`, `sql/lite`) over a defined protocol. That protocol is the public contract: versioning and stability matter here specifically. Third-party adapters (`dynamo`, `redis`, `sqlserver`) need this contract published before they can be written. Open: where the contract lives (a `.mirror` grammar file? a versioned spec doc? both?) and what the v1.0 commitment is.

### Q11: License model for `@spectral/garden`

Per-package: each curator chooses (Apache-2.0 / commercial / mixed). The substrate verifies signatures regardless of license; ed25519 + content-addressing closes tampering by construction. Open: which gardens ship at v1.0 launch, what minimum reviewer-credential discipline counts, and whether spectral.engineer hosts a canonical default-garden or stays neutral substrate.

### Q12: Trigger conditions for LRM-deferred substrate

Three pieces are captured-but-deferred per the Last Responsible Moment: `@kintsugi/cross_wall` (#80), `@epistemologic/reality` (Track G), `@spectral/garden` (Track H). Each has a natural demand signal that should trigger implementation. Document the triggers so the next session knows when to pull from capture:

- **`cross_wall`** triggers when an `@io` grammar's halts becomes provable and a user-or-substrate asks to pull it. Likely first consumer: the fragmentation Rust crate as Phase 4 R-tick lands.
- **`@epistemologic/reality`** triggers when per-peer lens authoring surfaces (Phase 7 onboarding; the garden's reviewer-lens chain; per-peer eigenboard customization).
- **`@spectral/garden`** triggers when Phase 7 onboarding needs a concrete content source — the DGSF/ICF/practitioner corpus has nowhere else to live structurally.

### Q13: The next-altitude recognition

The 2026-05-25/26 session crossed multiple "the substrate knew" moments — gen_prism IS MCP; @peer = Prism(self); shard = observer-relative λ₀; portal = `@io.socket` + content-addressed subspace; glass_wall as inverted halts; spectral triple as heuristic composition; lens as constructivism made structural; garden as vetted-corpus distribution. The pattern itself suggests there's another altitude waiting to be recognized. Candidates: the relational topology of multi-peer composition (the *cluster as organism*, not as N independent peers); the substrate's gestalt-of-gestalts at the garden+peer composition layer; the meta-curator (what verifies the verifiers when multiple gardens disagree); or something not yet named. Open: what's the recognition the substrate is pulling toward next.

---

## 11. References

### Insight docs (cited prior art throughout the corpus)

- `docs/insights/2026-04-07-the-chain-is-the-shatter.md`
- `docs/insights/2026-04-07-quantum-native-on-classical-hardware.md`
- `docs/insights/2026-05-14-cosmos-teaches-the-compiler.md`
- `docs/insights/2026-05-24-backpressure-as-modular-flow.md`
- `docs/insights/2026-05-25-mirror-supersedes-daemon.md` — gen_prism IS MCP; transport layer disappears.
- `docs/insights/2026-05-25-agent-home-as-typed-hole.md` — five-axis identity gestalt; the agent home as type.
- `docs/insights/2026-05-25-pipe-hole-and-au-binary.md` — `|\>` operator; locally-optimal binaries; Au.
- `docs/insights/2026-05-25-shard-as-observer-relative-lambda-zero.md` — λ₀ made operational; shard as observer-relative deployment.
- `docs/insights/2026-05-25-spectral-namespace-architecture.md` — mosaic + portal + closed engine + open adapters.
- `docs/insights/2026-05-25-parametric-types-and-fp-heritage.md` — zoom(T)/refract(T) as Prism-at-type-layer; FP heritage mapped.
- `docs/insights/2026-05-25-time-as-substrate-and-postgres-heritage.md` — `@time.duration`; monotonic/wall/instant; cross-frame convert; PG discipline.
- `docs/insights/2026-05-25-gram-and-mirror-same-architecture-two-altitudes.md` — GRAM (arXiv:2605.19376v2) as mirror's substrate at the NN layer; cross-domain prior art.
- `docs/insights/2026-05-26-portal-as-io-socket-over-content-addressed-subspace.md` — portal as `@io.socket` + content-addressed subspace + shard-frame; wire protocol; the seam where everything composes.
- `~/dev/systemic.engineering/practice/insights/spectral-db/dirac-operator-on-graphs.md`
- `~/dev/systemic.engineering/practice/insights/spectral-db/turing-eigenvalue-thread.md`
- `~/dev/systemic.engineering/practice/insights/coincidence/void-dual-geometry.md`
- `~/dev/systemic.engineering/practice/insights/coincidence/quantum-graph-theory.md`
- `~/dev/systemic.engineering/practice/insights/coincidence/heterogeneous-numerical-prism.md`

### Specs (load-bearing for the phases)

- `docs/specs/parser-as-prism-grammar.md` — the meta-glass; FP1; Phase 2 foundation.
- `docs/specs/walker-contract.md` — the Combinator walker's parse semantics; F-1.
- `docs/specs/combinator-optimization.md` — beta normalization; charset compilation.
- `docs/specs/mirror-store.md` — the three-layer parser architecture; FP1 at Layer 2.
- `docs/specs/mirror-native-vcs.md` — fragmentation as DAG VCS; the workspace layout; CoincidenceHash<5> as default.
- `docs/specs/fragmentation-as-generated.md` — Phase 4's demonstration vehicle.
- `docs/specs/kintsugi-formatter.md` — the formatter loop + kintsugi-as-Ricci-flow framing.
- `docs/specs/kintsugi-tournament.md` — merge resolution via @fate.infer.
- `docs/specs/scheduler-tower.md` — the temporal algebra; demand contracts on gen_prism.
- `docs/specs/bootstrap-retirement-plan.md` — the minimal Rust surface count.
- `docs/specs/ast-as-bundle.md` — the AST IS a Bundle written as data.

### Collaborator prior art

- **Jakobs, A. (2012).** *Integration von OpenGL-Visualisierungstechniken in GPU-Anwendungen.* M.Sc. thesis, FH Aachen Campus Jülich / PGI/JCNS. `~/dev/systemic.engineering/practice/collaborators/anna-wolf/master_jakobs.pdf`. Load-bearing for v1.0 cloud deployment.

### Foundational mathematics

- Connes, A. (1994). *Noncommutative Geometry.* Academic Press.
- Chamseddine, A.H. & Connes, A. (1996). "The Spectral Action Principle." Commun. Math. Phys. 186:731–750.
- Turing, A.M. (1952). "The Chemical Basis of Morphogenesis." Phil. Trans. Roy. Soc. B 237:37–72.
- Fiedler, M. (1973). "Algebraic connectivity of graphs." Czech. Math. J. 23:298–305.
- Braunstein, S.L., Ghosh, S., Severini, S. (2006). "The Laplacian of a Graph as a Density Matrix." Annals of Combinatorics 10(3).
- Ollivier, Y. (2009). "Ricci Curvature of Markov Chains on Metric Spaces." J. Funct. Anal. 256(3).
- Pastawski, F., Yoshida, B., Harlow, D., Preskill, J. (2015). "Holographic quantum error-correcting codes." JHEP 06:149.
- Raussendorf, R. & Briegel, H.J. (2003). "Measurement-based quantum computation with cluster states." Phys. Rev. A 68:022312.

### Protocol prior art

- Valim, J. et al. *GenStage* (2016). https://github.com/elixir-lang/gen_stage
- Valim, J. et al. *Broadway* (2019). https://github.com/dashbitco/broadway
- *stage_play* (Gleam, 2025). `/Users/alexwolf/dev/projects/stage_play/`. The statically-typed BEAM stage-composition pattern that mirror's grammar-level subscription typing inherits from.

### Related projects (in this development tree)

- `~/dev/projects/prism/` — the Prism algebra crate; LAPACK FFI; the substrate that mirror sits on.
- `~/dev/projects/fragmentation/` — the DAG VCS substrate; SpectralCoordinate; the storage layer.
- `~/dev/projects/fate/` — the five-model inference layer; Metal runtime pattern that NumericalPrism's MetalBackend inherits.
- `~/dev/projects/spectral/` — spectral-db; the application layer that consumes fragmentation; Phase 7's deployment target.

---

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

Apache-2.0.
