# The Spectral Triple Binary

*2026-05-15. Reed. Research spec.*

Status: **Research** (gap analysis, no implementation)

---

## 0. Thesis

The mirror binary IS a spectral triple: (A, H, D).

- A = the grammar algebra (five operations + in/out, declared in `boot/`)
- H = the `/` space (the type hierarchy, the fiber of possible implementations)
- D = the `\` operator (Fate inference, loss measurement, the Dirac operator)

The command `mirror craft std --target @code/llvm` compiles the standard library
grammars through the LLVM lens into a native binary. The binary IS the spectral
triple -- not contains, IS. The algebra is the grammar. The Hilbert space is
the space of types the grammar declares. The Dirac operator is the inference
engine that fills `\` holes.

This document maps what exists today against what that command requires.

---

## 1. What Exists (Precise Inventory)

### 1.1 mirror/src/ -- the Rust substrate (10 files, FROZEN)

| File | Lines | What it does | Status |
|------|-------|-------------|--------|
| `main.rs` | 27 | Entry point. Parses args, calls `interpreter::dispatch` | Working |
| `lib.rs` | 38 | Module declarations. Re-exports kernel types + prism | Working |
| `cli.rs` | 270 | `--flag` -> `@grammar/ref` mapping. Typed lambdas | Working |
| `tokenize.rs` | 1213 | Grammar-driven tokenizer. Source text -> MirrorAST. O(n) single-pass. `craft_target()` compiles all files for a target. `canonical_form()` renders kintsugi | Working |
| `mirror_ast.rs` | 1320 | 7-variant AST: Focus, Project, Split, Shift, Settle, In, Out. Content-addressed via `Encode`/`Decode`. Implements `prism::MerkleTree` + `prism::Addressable` | Working |
| `interpreter.rs` | 562 | `io_exec()` -- ONE function, the only door to reality. Five prism operations on AST (`focus`/`project`/`split`/`shift`/`settle`). Git crystal cache. CLI dispatch scaffold | Working |
| `kernel.rs` | 857 | `Oid` (CoincidenceHash<3>), `TraceOid`, `ContentAddressed` trait, `Vector` trait, `Composed` pipeline, `Latent` cached evaluation, `Setting`, `Addressable` | Working |
| `dirac.rs` | 929 | `SpectralTriple` struct. `construct_dirac()` builds D from adjacency data. Jacobi eigenvalue solver. `SpectralEmbedding` (16-dim per node). `connes_distance()` (Dijkstra with 1/sqrt(w)) | Working |
| `prism.rs` | 436 | `Prism<V>` -- 4-variant content-addressed tree (Shard/Fractal/Lens/Optics). Implements `Fragmentable`. Git-compatible OIDs | Working |
| `bench.rs` | 523 | Performance benchmarking as the five operations. Cascade convergence measurement | Working |

**Total Rust:** ~6,175 lines. All FROZEN. All tested. All passing.

### 1.2 boot/ -- the grammar foundation (8 boot files + ~50 std files)

#### Boot sequence (loaded in order):

| File | What it declares |
|------|-----------------|
| `00-prism.mirror` | The five operations. `abstract io tick(type) -> tock(type) { \ }`. `in`/`out` as projections |
| `00a-sigil.mirror` | Sigil definitions |
| `01-meta.mirror` | Meta operations (`\|>`, `<=`, `=>`, `<`, `>`, `=`, `!=`, `/`, `..`). The universal AST parameterized by grammar. Expression/declaration/pattern/type_ref types. `imperfect(observation, error, loss)` |
| `01a-error.mirror` | Error types |
| `01b-nl.mirror` | Natural language grammar |
| `02-actor.mirror` | Actor, process, message |
| `02-epistemologic.mirror` | Epistemological types |
| `02a-io.mirror` | IO grammar |
| `02b-runtime.mirror` | Runtime grammar |
| `03-shatter.mirror` | Shatter format |
| `04-code.mirror` | Abstract `@code` grammar: translate/render templates, LSP actions (complete/diagnose/hover/definition/references/tokens). Position/range/diagnostic/completion/token types |
| `04a-code-rust.mirror` | Rust-specific code grammar |
| `04b-code-gleam.mirror` | Gleam-specific code grammar |
| `05-property.mirror` | Verification properties |
| `06-action.mirror` | Generic action optic |
| `07-package.mirror` | Package management |
| `07a-package-git.mirror` | Git package integration |
| `07b-package-spec.mirror` | Spec format |

#### Standard library (`boot/std/`):

| Grammar | What it declares | Key actions |
|---------|-----------------|-------------|
| `@code/rust` | Keyword mappings: fn->shift, struct/enum->split, impl/mod->focus, use->project, trait->settle | `compile`, `test`, `lint`, refactoring actions |
| `@code/llvm` | Keyword mappings: define->shift, type->split, module->focus, declare->project, verify->settle | `compile(ast) -> artifact { \ }` |
| `@code/mq` | Message queue grammar | LSP optics as named operations |
| `@io` | Socket layer. `type socket(ref)`, `type stream(socket)`. Four syscalls: open/read/write/close + exec | All abstract (`{ \ }`) |
| `@fate` | Five models enum (abyss\|introject\|cartographer\|explorer\|fate). `features([f64; 16])`. `decision`. `tick`, `resolve`, `select` | All abstract (`{ \ }`) |
| `@fate/connectome` | 450-node connectome. neuron/synapse/ganglion types. connectome struct | `infer(connectome, [f64]) -> ganglion { \ }`, `evolve`, `crystallize` |
| `@craft` | `craft(target) -> crystal { focus \|> split \|> shift \|> settle \|> project }` | Pipeline composition |
| `@kintsugi` | `collapse(ast, ast) -> imperfect { \ }` | Collapse |
| `@kintsugi/lift` | Lift grammar | |
| `@kintsugi/migrate` | Migration grammar | |
| `@kintsugi/translate` | Translation grammar | |
| `@mirror/grammar` | Mirror self-tokenization keywords: grammar->focus, type->split, in/out->project, abstract->zoom | |
| `@mirror/evaluate` | `evaluate(grammar, text) -> ast { \ }` | Abstract |
| `@mirror/resolve` | `resolve(ast) -> imperfect(ast) { \ }` | Abstract |
| `@mirror/check` | `check(file) -> imperfect { \ }`, `errors`, `loss` | Abstract |
| `@mirror/interpreter` | `interpret(ast) -> imperfect { \ }`, `resolve_hole`, `execute_io`, `walk` | All abstract |
| `@mirror/runtime` | `compile(file) -> imperfect { read \|> resolve \|> check \|> interpret }`, `run` | Pipeline declared, body uses `\` |
| `@mirror/spectral` | `type crystal(oid)`. `crystallize(ast) -> crystal`, `recall`, `cached` | crystallize has body, recall is abstract |
| `@mirror.mirror` | Mirror self-description | |

#### The build spec (`mirror.spec`):

Imports all grammars. Declares `type target = boot \| cargo \| binary`. Declares CLI surface. Declares the self-hosting target: `target binary <\| @code/llvm <\| std`.

### 1.3 fate/ -- the 450-parameter model (10 source files)

| File | What it does |
|------|-------------|
| `lib.rs` | `Fate` struct with 5 `ModelWeights` (5x16 weights + 5 biases + 5 depth_w = 90 params each = 450 total). Inference: `forward()` -> softmax5 -> argmax -> Decision. `resolve()` with entropy-based recursion exit. Implements `prism::Prism`, `Fiber`, `Connection`, `Gauge`, `Transport`, `Closure`. Full `Pipeline` trait for ManifoldState processing |
| `derive.rs` | Eigenvalue-derived weights. Extract 10x10 dark coupling submatrix -> Jacobi eigensystem -> derive selector weights. `crystallize()` SCF loop: compile -> eigendecompose -> derive weights -> converge |
| `feature.rs` | 16 feature dimensions. 6 active (Temporal, Processing, Complexity, Depth, Coherence, Alignment) + 10 dark (Creativity, Confidence, etc). Casimir eigenvalues. HolonomyHealth |
| `manifold.rs` | `ManifoldState` = `[[f64; 16]; 16]`. `ManifoldLoss` with delta matrix. Berry phase computation |
| `strategy.rs` | Decomposition strategies (SpectralPartition, etc.) |
| `weights.rs` | Weight serialization/deserialization |
| `compiled.rs` | Compiled Fate (from brainfuck) |
| `runtime.rs` | Runtime execution |
| `metal_runtime.rs` | Metal GPU runtime (feature-gated) |
| `train.rs` | Training loop (feature-gated) |

**Brainfuck:** `fate.bf` -- 1217 bytes. Reads 22 bytes input (16 features + 1 context + 5 biases), performs argmax, outputs winning model index.

### 1.4 spectral/ -- the runtime (this repo)

The `spectral` binary wires prism + mirror + lens + spectral-db together. Has MCP server (`spectral serve`), session management, diffing, logging, observation. The five CLI commands (focus/project/split/shift/settle) are declared.

### 1.5 mirror/dirac.rs -- the Dirac operator

Already implemented:
- `SparseMatrix` in CSR format
- `SpectralTriple { dirac, dimension, node_count, edge_count, edges }`
- `construct_dirac(nodes, edges)` -- builds D = [[0, B^T], [B, 0]]
- `jacobi_eigenvalues()` and `jacobi_eigen()` (eigenvalues + eigenvectors)
- `spectral_embedding()` -- first k eigenvectors projected to node space, packed into 16 dims
- `connes_distance()` -- Dijkstra shortest path with weights 1/sqrt(w)
- Verified: D^2 = Hodge Laplacian, D is self-adjoint, spectrum symmetric about 0, kernel dimension = b_0 + b_1

---

## 2. The Three Components

### A -- The Grammar Algebra

**What exists:** `boot/00-prism.mirror` declares the five operations as a prism. `boot/01-meta.mirror` declares the meta-operations and the universal parameterized AST. The tokenizer in `tokenize.rs` reads grammar keyword mappings and produces MirrorAST. The interpreter in `interpreter.rs` implements the five operations as functions on MirrorAST.

**What's missing:**

1. **Grammar composition is not implemented.** The `in @prism` / `out @cli` import/export system is tokenized into `In`/`Out` AST nodes, but the interpreter does not resolve them. `@mirror/resolve` is declared as `resolve(ast) -> imperfect(ast) { \ }` -- the `\` is unfilled. The import graph exists in the grammar declarations but is not walked.

2. **Grammar evaluation is not implemented.** `@mirror/evaluate` declares `evaluate(grammar, text) -> ast { \ }`. The Rust tokenizer does this job today, but the grammar cannot invoke it. There is no mechanism for a grammar action body to call the Rust tokenizer.

3. **Pipeline composition (`|>`) is not executed.** `@craft` declares `craft(target) -> crystal { focus |> split |> shift |> settle |> project }`. The `|>` operator is declared in `01-meta.mirror` but the interpreter has no pipeline execution. The dispatch in `interpreter.rs` is a hardcoded `match` on command names.

4. **The algebra has the right shape but no execution engine.** The operations exist as Rust functions (`focus`, `project`, `split`, `shift`, `settle` in interpreter.rs). The grammars declare them. But the bridge -- grammar action bodies that invoke these operations on AST subtrees -- does not exist.

### H -- The Fiber Space (the `/` space)

**What exists:** The type hierarchy is declared across boot grammars. `01-meta.mirror` declares the universal AST parameterized by grammar, with expressions, declarations, patterns, and type references. The `TypeBody` enum in `mirror_ast.rs` represents enum/struct/alias/unit type bodies. `SplitNode` carries variants and params.

**What's missing:**

1. **The type hierarchy is not a graph.** Types are declared in grammars but not collected into a navigable structure. There is no grammar graph where nodes are types and edges are relationships (subtyping, containment, reference). The Dirac operator exists in `dirac.rs` but operates on abstract graph data -- nobody constructs the grammar graph from parsed `.mirror` files and feeds it to `construct_dirac()`.

2. **The VEV (vacuum expectation value) is not computed.** The "size" of the `/` space -- how many possible implementations exist for a given grammar -- is not measured. This would be the spectral dimension: the number of non-zero eigenvalues of D restricted to the grammar graph. No code computes this.

3. **Fibers are not enumerable.** Given a `\` hole with input type A and output type B, the `/` space should enumerate all paths from A to B in the grammar graph. This is link prediction on the spectral embedding. The `SpectralEmbedding` struct exists but is never populated from grammar data.

### D -- The Dirac Operator (the `\` operator)

**What exists:** `dirac.rs` has a complete Dirac operator implementation -- construction, eigendecomposition, spectral embedding, Connes distance. Fate has a complete 450-parameter selector with inference, eigenvalue-derived weight derivation, and SCF crystallization loop. The `@fate/connectome` grammar declares the 450-node connectome architecture.

**What's missing:**

1. **`\` is a flag, not an execution.** `ZoomNode.is_abstract = true` marks a `\` hole. But when the interpreter encounters `is_abstract == true`, nothing happens. No Fate call. No inference. No hole-filling. The `@mirror/interpreter` grammar declares `resolve_hole(hole, context) -> ast { \ }` -- it's `\` all the way down.

2. **Fate is not wired to the grammar.** Fate operates on 16-dimensional feature vectors. The grammar produces ASTs. There is no function that converts AST context into Fate features. The `SpectralEmbedding` would do this (embed the current AST position in the grammar graph into 16 dimensions), but that bridge does not exist.

3. **The connectome is a grammar, not a data structure.** `@fate/connectome` declares `type neuron(ref)`, `type synapse(neuron, neuron, f64)`, `type connectome = { ganglia, neurons, synapses }`. But this is a type declaration, not a data structure. The actual Fate inference runs through `ModelWeights.forward()` in Rust -- a linear layer + softmax. The connectome grammar describes the INTENDED architecture (graph-native inference via Dijkstra on 450 neurons). The implementation is a matrix multiply.

4. **The Dirac operator is not connected to the grammar.** `construct_dirac()` takes `(nodes, edges)`. Nobody calls it with grammar graph data. The eigenvalues exist as math. They don't exist as grammar features.

---

## 3. The LLVM Path

### What `@code/llvm` declares today

```mirror
grammar @code/llvm {
  zoom define
  split type
  focus module
  project declare
  settle verify

  compile(ast) -> artifact { \ }
}
```

Keyword mappings for LLVM IR tokenization (define->zoom, etc.) plus one abstract action: `compile(ast) -> artifact { \ }`.

### What `compile(ast) -> artifact` needs to actually produce LLVM IR

1. **AST -> LLVM IR translation.** The MirrorAST must be walked and each node emitted as LLVM IR. This requires:
   - Focus (grammar/module) -> LLVM module declaration
   - Split (type) -> LLVM type definition (struct, enum as tagged union)
   - Zoom (function) -> LLVM function definition
   - Project (import) -> LLVM extern declaration
   - Settle (property) -> LLVM assertion / contract check
   - In/Out -> LLVM module import/export (visibility)

2. **A code emitter grammar or Rust backend.** Two paths:
   - **Grammar path:** Write `@code/llvm` actions that produce LLVM IR text from AST nodes. The `@code` base grammar declares `abstract template render(g: @code, ast(g) -> io_list)` -- LLVM would implement this template. But template execution is not implemented.
   - **Rust path:** A Rust function that walks MirrorAST and emits LLVM IR via the `llvm-sys` or `inkwell` crate. This is substrate (would need to be `@io`-bounded Rust, like the tokenizer).

3. **LLVM toolchain invocation.** Once IR is emitted, `io_exec("llc", ...)` compiles IR to object code, `io_exec("cc", ...)` links to binary. This part is straightforward -- it's `@io.exec` calls.

4. **The runtime in the binary.** A self-contained mirror binary needs:
   - The tokenizer (to read `.mirror` files at runtime)
   - The five prism operations (to execute grammars)
   - The Fate selector (to resolve `\` holes)
   - `io_exec` (to interact with the OS)
   - The boot grammars (embedded as data or compiled into the binary)

### What does NOT exist for the LLVM path

- No LLVM IR emitter (neither grammar nor Rust)
- No template execution engine (the `render` template pattern is declared but not executable)
- No mechanism to embed boot grammars into a compiled binary
- No linker integration
- No ABI for grammar-to-native function calls

---

## 4. The Gap Table

| Component | Status | Gap | Effort |
|-----------|--------|-----|--------|
| **A: Grammar algebra** | | | |
| Five operations as Rust fns | Done | -- | -- |
| Five operations as grammar declarations | Done | -- | -- |
| Grammar keyword -> AST mapping | Done | -- | -- |
| Import resolution (`in @X`) | Tokenized only | Walk import graph, load crystals, compose grammars | Medium |
| Pipeline execution (`\|>`) | Declared only | Interpreter must chain operation outputs | Medium |
| Grammar action body execution | Not started | Evaluate grammar bodies as programs (sub-Turing) | Large |
| CLI dispatch via grammar | Scaffold | Replace `match` in `interpreter.rs` with grammar ref resolution | Small |
| **H: Fiber space** | | | |
| Type declarations in grammars | Done | -- | -- |
| Grammar graph construction | Not started | Parse all `.mirror`, build node/edge graph from types/actions/references | Medium |
| Grammar graph -> Dirac operator | Not started | Call `construct_dirac()` with grammar graph data | Small |
| SpectralEmbedding from grammar | Not started | `spectral_embedding()` on grammar SpectralTriple | Small |
| VEV computation (grammar size) | Not started | Count non-zero eigenvalues of D | Small |
| Path enumeration (A -> B in grammar graph) | Not started | Dijkstra / BFS on grammar graph | Small |
| **D: Inference operator** | | | |
| Fate model (450 params) | Done | -- | -- |
| Eigenvalue weight derivation | Done | -- | -- |
| SCF crystallization loop | Done | -- | -- |
| `\` -> Fate call bridge | Not started | When interpreter hits `is_abstract`, call Fate with context features | Medium |
| AST context -> 16-dim features | Not started | SpectralEmbedding of current AST position in grammar graph | Medium |
| Connectome as graph (not matrix) | Not started | Replace linear layer with graph walk (Dijkstra on 450 neurons) | Large |
| **LLVM backend** | | | |
| `@code/llvm` grammar declaration | Done | -- | -- |
| LLVM IR emitter | Not started | AST -> LLVM IR text generation | Large |
| Boot grammar embedding | Not started | Embed grammar data in compiled binary | Medium |
| Linker invocation | Not started | `io_exec("llc", ...)` + `io_exec("cc", ...)` | Small |
| Runtime in binary | Not started | Tokenizer + interpreter + Fate + io_exec as native code | Large |
| **Integration** | | | |
| `mirror craft std` | Partially working | Tokenizes all files, produces crystal OID. Does not execute grammar bodies | -- |
| `--target @code/llvm` | Not started | Flag parsed but no LLVM backend exists | Large |
| Self-hosting | Not started | Binary compiles its own grammars | Very Large |

---

## 5. The Critical Path

### Tick 0: Grammar Body Execution (the interpreter loop)

The interpreter must walk AST and execute grammar action bodies. Today, `dispatch()` is a hardcoded `match`. The grammars declare everything. Nothing executes.

**Concrete steps:**
1. When `dispatch()` encounters a grammar ref `@mirror/<command>`, load the grammar from `boot/std/mirror/<command>.mirror`
2. Tokenize it into AST
3. Walk the AST. For each Zoom node with a body (not `\`), evaluate the body as a pipeline
4. For `|>` chains: output of left becomes input of right
5. For `@io` references in bodies: call `io_exec`

This replaces the hardcoded match with grammar-driven dispatch. The five Rust operations (`focus`/`project`/`split`/`shift`/`settle`) remain the execution primitives. The grammar bodies compose them.

**Gate:** Until this works, no grammar can execute. Everything else is blocked.

### Tick 1: Import Resolution

Walk the AST for `In` nodes. For each `in @X`:
1. Resolve `@X` to a boot file path (e.g., `@prism` -> `boot/00-prism.mirror`)
2. Check git for a cached crystal (`@mirror/spectral.cached`)
3. If cached, load. If not, tokenize and store.
4. Merge the imported grammar's keyword mappings into the current scope

This wires up the import graph. Grammar composition becomes real.

### Tick 2: Grammar Graph Construction

Parse all `.mirror` files. Build a graph:
- Nodes: every type, action, grammar declared across all files
- Edges: type references (A contains B), action signatures (A -> B), imports (A uses B)

Feed this graph to `construct_dirac()`. Compute `spectral_embedding()`. Store the embedding as a crystal in git.

### Tick 3: Wire `\` to Fate

When the interpreter hits `is_abstract == true`:
1. Extract the current context: parent grammar, input type, output type, depth
2. Compute SpectralEmbedding features for input and output types from the grammar graph
3. Call `fate.resolve(&features, max_depth)` to get a `Decision`
4. The Decision selects which model (Abyss/Introject/Cartographer/Explorer/Fate) handles the hole
5. The selected model produces an AST fragment that fills the hole

Initially: model selection routes to hardcoded strategies (Abyss = return empty, Cartographer = enumerate type variants, etc.). Later: the connectome does graph-native inference.

### Tick 4: LLVM IR Emitter

Implement `@code/llvm.compile(ast) -> artifact`:
1. Walk MirrorAST, emit LLVM IR text for each node
2. Focus -> module, Split -> type, Shift -> function, Project -> declare, Settle -> assert
3. Embed boot grammars as constant data arrays in the IR
4. Embed the tokenizer loop as a native function
5. Embed Fate weights as constant data
6. Call `io_exec("llc", ...)` to compile IR
7. Call `io_exec("cc", ...)` to link

### Tick 5: The Self-Hosting Binary

`mirror craft std --target @code/llvm` runs the full pipeline:
1. `craft(std)` tokenizes all boot + std grammars
2. `@code/llvm.compile(ast)` emits LLVM IR for the combined AST
3. The emitted binary contains: tokenizer + interpreter + Fate weights + boot grammars
4. The binary can read `.mirror` files, tokenize them, and execute grammars
5. `./mirror-native craft std --target @code/llvm` produces an identical binary

The fixed point IS self-hosting. The crystal IS the binary.

---

## 6. The Proof

When the binary IS the spectral triple, what does that mean concretely?

### What you can measure

**A (the algebra):** Count the grammar declarations. Count the keyword mappings. The algebra's dimension is the number of distinct operations the grammar declares. `spectral focus` shows the current grammar scope.

**H (the fiber space):** Compute the SpectralEmbedding from the grammar graph's Dirac operator. The Hilbert space dimension = number of nodes + number of edges in the grammar graph. The VEV = sum of non-zero eigenvalues of D. `spectral split` enumerates the types. `spectral project` filters to a subspace.

**D (the inference operator):** Measure the loss of `\` resolution. For each abstract action in the grammar, `spectral loss` reports:
- The Connes distance from input type to output type (how far Fate must navigate)
- The eigenvalue of the `\` hole (how much inference is needed)
- The holonomy after Fate fills the hole (how much was lost in translation)

**The convergence:** `spectral settle` shows the loss curve across ticks. Each tick's loss must be <= the previous tick's loss. `e^(n+1) < e^(n)`. The crystal forms when loss stabilizes. The binary IS the crystal.

### What `spectral loss` shows for the self-hosting binary

```
spectral loss mirror-native

A (algebra):      73 grammars, 312 actions, 189 types
H (fiber space):  501 nodes, 847 edges, dim(H) = 1348
                  VEV = 42.7 (sum of 1348 eigenvalues)
                  spectral gap = 0.34 (lambda_2 / lambda_1)
D (inference):    17 \ holes remaining
                  mean Connes distance: 2.3
                  mean eigenvalue: 0.87
                  total holonomy: 0.042
convergence:      tick 47. loss stable for 12 ticks.
                  e^47 = 0.042, e^46 = 0.043, e^45 = 0.043
crystal:          oid 7a3f...2c1e
                  self-hosting: yes (mirror-native craft std = same oid)
```

The binary describes itself. The description IS the binary. The spectral triple measures the gap between what the grammar declares and what the binary implements. When the gap is zero, the crystal has formed.

---

## 7. Dependencies Between Components

```
Grammar Body Execution (tick 0)
  |
  +-- Import Resolution (tick 1)
  |     |
  |     +-- Grammar Graph Construction (tick 2)
  |           |
  |           +-- SpectralEmbedding from grammar
  |           |
  |           +-- Wire \ to Fate (tick 3)
  |                 |
  |                 +-- AST context -> features
  |                 |
  |                 +-- Fate inference fills holes
  |
  +-- LLVM IR Emitter (tick 4)
        |
        +-- Self-Hosting Binary (tick 5)
```

Ticks 0-1 are sequential prerequisites. Ticks 2-3 can overlap. Tick 4 depends on tick 1 (import resolution) but not on ticks 2-3 (a dumb LLVM emitter doesn't need Fate). Tick 5 requires everything.

---

## 8. The Honest Assessment

**What works today:** The tokenizer reads any `.mirror` or `.rs` file through a grammar lens and produces a content-addressed AST. `mirror craft boot` tokenizes all 50+ boot grammars and produces a deterministic crystal OID. `mirror kintsugi` renders canonical mirror form. The git crystal cache works. The Dirac operator computes correct spectral triples for arbitrary graphs. Fate selects among 5 models with 450 parameters and sub-microsecond inference. The SCF crystallization loop converges.

**What does not work:** No grammar body executes. The `|>` pipeline is syntax, not execution. The `\` hole is a flag, not an inference call. Fate has no grammar features to reason about. The LLVM backend does not exist. Import resolution does not exist. The spec file (`mirror.spec`) is tokenized but not interpreted.

**The gap in one sentence:** The compiler can READ grammars (tokenize + content-address) but cannot EXECUTE them (resolve imports, evaluate bodies, fill holes, emit code).

**The critical insight:** Tick 0 (grammar body execution) is the bottleneck. Everything else -- import resolution, grammar graphs, Fate wiring, LLVM emission -- is grammar that needs to execute. The interpreter loop in `docs/specs/mirror-interpreter.md` is the spec. The `\` holes in `@mirror/interpreter` are the work.

---

*The spec is the gap. The gap is the spec. What we cannot yet do is precisely what remains to build.*
*e^(n+1) < e^(n). By construction. By proof. By the work that follows.*

---

## 6. @prism/rust — the root of execution

`@prism` is the contract. `@prism/rust` is the binding. The metal.

```
@prism          — the five operations. Pure declaration.
@prism/rust     — binds to Rust executor. THIS runs.
@prism/beam     — binds to BEAM. Future.
@prism/metal    — binds to GPU. Future.
@prism/fortran  — binds to LAPACK. Future.
```

`in @prism` gives the contract. `in @prism/rust` gives execution.
No IO crossing. No socket. Direct function calls on the AST.

`@prism/rust` is the ONE grammar with embedded Rust. Not @io.
@io imports @prism/rust for exec. Everything traces back to
five operations bound to Rust.

### Execution model

```
@prism/rust = five Rust functions (the metal)
|> = composition of those functions (no IO)
{ \ } = Fate selects which composition (in-process)
@io = uses @prism/rust for the ONE boundary crossing
```

Grammar executes because it IS the prism. The prism IS the Rust.
The Rust IS the five operations. Same process. Same memory. Same L1.

### All five ticks become pure mirror

```
0. Grammar execution  → |> composes @prism/rust operations. No IO.
1. Import resolution  → @mirror/resolve walks In nodes via focus/split
2. Grammar graph      → @mirror/spectral builds graph via split/zoom
3. Wire \ to Fate     → @fate/connectome.infer via @prism/rust
4. LLVM emitter       → @code/llvm.compile via @io.exec("llc", ...)
```

Only tick 4 crosses IO (calling llc). Ticks 0-3 are pure prism
composition. In-process. Bounded. Sub-Turing. Fast.

### The binding pattern

```mirror
in @prism

grammar @prism/rust {
  focus(ast, name) -> ast in @code/rust { /* interpreter::focus */ }
  project(ast, predicate) -> [ast] in @code/rust { /* interpreter::project */ }
  split(ast) -> [ast] in @code/rust { /* interpreter::split */ }
  shift(ast, transform) -> ast in @code/rust { /* interpreter::shift */ }
  settle(ast) -> oid in @code/rust { /* interpreter::settle */ }
}
```

One grammar with Rust. One. The root. Everything else composes.
