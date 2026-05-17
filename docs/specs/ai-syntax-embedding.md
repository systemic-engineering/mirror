# Embedding @ai into Mirror Syntax

*2026-05-17. Reed. Research spec.*

Status: **Research** (gap analysis, no implementation)

---

## 0. Thesis

The language carries its own training signal. The syntax IS the model. The weights ARE the code. The code IS the weights.

Three composition operators encode the relationship between programmer knowledge and machine inference:

- `|>` -- unweighted composition. The programmer knows the path. Weight = 1.0.
- `|\>` -- eigenboard-inferred composition. Fate decides the weight. The `\` inside the pipe IS Fate resolution.
- `<|` -- reverse flow. Type collapsing toward a union with probability distribution.

The eigenboard provides the weights. Reflection perturbs them. Each tick, the weights shift. The grammar evolves. The `.mirror` file IS the model.

---

## 1. What Exists (Precise Inventory)

### 1.1 The `|>` Operator (Unweighted Composition)

**In `boot/01-meta.mirror`:**
```mirror
zoom |>(ref, prism)
zoom <|(prism, ref)
```

`|>` is declared as a `zoom` operation taking `(ref, prism)` -- a reference and a prism operation. It is a meta-operator in the grammar. `<|` is its reverse.

**In `boot/std/craft.mirror`:**
```mirror
grammar @craft {
  craft(target) -> crystal {
    focus(target) |> split |> zoom |> refract |> project
  }
}
```

`|>` is used in grammar action bodies to compose the five operations sequentially. This is the ONLY place `|>` has concrete semantics today -- as textual pipeline composition in action bodies.

**In the AST (`mirror_ast.rs`):** `|>` has NO dedicated AST variant. The 7 variants are Focus, Project, Split, Zoom, Refract, In, Out. When the tokenizer encounters `|>` in a grammar body, it is treated as part of the body text -- the body is stored as `Vec<MirrorAST>` or left unstructured. The tokenizer does not parse `|>` into any specific structure.

**In the tokenizer (`tokenize.rs`):** The tokenizer recognizes keywords via grammar mappings (e.g., `fn` -> Zoom). It does NOT recognize `|>` as a token. The scanner looks for alphanumeric identifier-like tokens. Operators like `|>` fall through as non-keyword punctuation and are skipped.

**In the interpreter (`interpreter.rs`):** Pipeline execution (`|>`) is declared in grammars but NOT implemented. The interpreter has hardcoded `match` dispatch. There is no pipeline chaining.

### 1.2 The `<|` Operator (Reverse Flow)

**In `boot/01-meta.mirror`:** Declared as `zoom <|(prism, ref)` -- a prism applied to a ref in reverse direction.

**In the build spec (`spec.mirror`):** Referenced as `target binary <| @code/llvm <| std` -- the composition target chains leftward.

**Nowhere else.** `<|` is purely declarative. No execution path exists.

### 1.3 The `\` Operator (Intent Hole / Fate Resolution)

**In the AST:** `ZoomNode.is_abstract: bool`. When `true`, the node represents a `\` hole -- an action whose implementation is to be inferred by Fate.

**In grammars:** Used pervasively:
```mirror
infer(connectome, [f64]) -> ganglion { \ }
interpret(ast) -> imperfect { \ }
resolve_hole(hole, context) -> ast { \ }
```

**In the interpreter:** When `is_abstract == true` is encountered, nothing happens. No Fate call. No inference. The `\` is a marker, not an execution.

**In `@code/mq`:** The `\` operator has explicit query semantics:
```mirror
\ action(intent: text) -> query
```
Agent writes natural language intent. Fate runs a tournament. The query resolves to concrete operations.

### 1.4 Fate (The 450-Parameter Model)

**Architecture:** 5 selectors, each with:
- `w: [[f64; 16]; 5]` -- 80 feature weights
- `b: [f64; 5]` -- 5 biases
- `depth_w: [f64; 5]` -- 5 depth modulation weights
- Total: 90 params per context x 5 contexts = 450 parameters

**Forward pass:** `features [16] -> linear -> softmax5 -> argmax -> Model`

**Weight derivation (`derive.rs`):**
1. Extract 10x10 dark coupling submatrix from ManifoldState
2. Jacobi eigendecomposition (pure Rust, zero deps)
3. 5 largest eigenvalues become biases
4. Corresponding eigenvectors projected into 16-dim space become weights
5. Eigengaps become depth modulation weights

**Crystallization (`crystallize()`):** SCF loop:
1. Start with zero weights
2. Compile grammars -> dark coupling matrices
3. Eigendecompose -> derive weights
4. Damped mixing
5. Check eigenvalue convergence
6. Repeat until stable

### 1.5 The Eigenboard

**In `prism/gestalt/memory.mirror`:**
```mirror
type eigenboard = { fiedler: float, lambda: float[], updated_at: timestamp }
```

**In `boot/std/code/mq.mirror`:**
```mirror
type eigenboard_state {
  fiedler: f64,
  nodes: u32,
  edges: u32,
  loss: f64
}
```

**In spectral repo:** The eigenboard is a full runtime concept -- the TUI renders it, hooks shift it, the runtime tracks it. It tracks Fiedler value (graph connectivity), eigenvalues (spectral structure), node/edge counts, and loss.

**Where it lives:** Git. Always git. The eigenboard state is content-addressed. Each tick produces a new crystal. The history of eigenboard states IS the git log.

### 1.6 The Dirac Operator (`dirac.rs`)

Fully implemented:
- `SpectralTriple { dirac, dimension, node_count, edge_count, edges }`
- `construct_dirac(nodes, edges)` -- D = [[0, B^T], [B, 0]]
- `jacobi_eigenvalues()` / `jacobi_eigen()` -- eigendecomposition
- `spectral_embedding()` -- first k eigenvectors packed into 16 dims
- `connes_distance()` -- Dijkstra with 1/sqrt(w) weights

### 1.7 Reflection

**In `docs/specs/reflection-model.md`:** Four operations per tick:
1. **Observe** -- watch the pipeline run
2. **Adjust** -- bias future runs (weight nudges)
3. **Write gestalt** -- update growth/tension/beam history
4. **Hold** -- carry unresolvable tensions

Reflection IS the training loop. Every query trains the system through Reflection's adjustments. No separate training phase. Reflection is the ONLY model that adjusts the others.

**Grammar:** `@peer` (or `@cogito` applied to computation). Actions: observe, notice, adjust, write, hold, speak. Speaks at tick n+1 (one tick behind).

### 1.8 The Five Models as Persistent/Transient

**Persistent (the pipeline):**
| Model | Operation | What |
|-------|-----------|------|
| Surface | Zoom | language -> query |
| Mirror | Refract | query -> graph path |
| Shatter | Split | graph -> text |
| Reflection | Focus | pipeline -> adjustments |

**Transient (within Mirror loop, selected by Fate):**
| Model | Enum | What |
|-------|------|------|
| Abyss | `Model::Abyss` | depth detection |
| Introject | `Model::Introject` | pattern matching |
| Cartographer | `Model::Cartographer` | mapping |
| Explorer | `Model::Explorer` | search |
| Fate | `Model::Fate` | selector (which model wins) |

---

## 2. The Operators

### 2.1 `|>` -- Unweighted Composition (weight = 1.0)

The programmer knows the path. No inference needed. Composition is deterministic.

```mirror
craft(target) -> crystal {
  focus(target) |> split |> zoom |> refract |> project
}
```

**Semantics:** Left output becomes right input. Each step has weight 1.0 -- the programmer asserts full confidence that this composition is correct.

**AST representation today:** None. `|>` exists only as body text in ZoomNode bodies. The tokenizer does not parse it.

**What it should be:** A composition node in the AST body. Specifically: within a ZoomNode's `body: Option<Vec<MirrorAST>>`, a `|>` chain should be represented as a sequence of operation references with implicit weight 1.0. This does NOT require a new AST variant -- it requires the body parser to recognize `|>` and produce an ordered list of operation references.

### 2.2 `|\>` -- Eigenboard-Inferred Composition (Fate fills the weight)

The programmer declares the composition but does not know the optimal weight. Fate provides it from the eigenboard.

```mirror
-- Hypothetical syntax:
transform(input) -> output {
  normalize(input) |\> compress |\> refine -> output
}
```

**Semantics:** The `\` inside the pipe IS Fate resolution. At each `|\>` boundary, Fate evaluates:
1. What is the current spectral embedding? (where are we in the graph)
2. What are the eigenboard weights for this transition? (how confident is the system)
3. Should this step run, and with what priority? (weight 0.0 to 1.0)

The weight is NOT a probability of correctness. It is the eigenvalue-derived confidence that this transition reduces holonomy. Weight 0.0 = skip this step. Weight 1.0 = always run. Intermediate = run with proportional resource allocation.

**How the weight flows from eigenboard to `|\>`:**
1. Grammar graph is computed from all `.mirror` files (nodes = types/actions, edges = references)
2. Dirac operator D is constructed from the grammar graph
3. SpectralEmbedding (16-dim) is computed from D's eigenvectors
4. At each `|\>` boundary, the embedding of the left-side output type and right-side input type are compared
5. The Connes distance between them in the spectral embedding IS the weight
6. Low distance = high weight (types are spectrally close = natural transition)
7. High distance = low weight (types are spectrally far = forced transition)

**AST representation:** Two options, neither requiring new Rust:

*Option A: Extend body parsing.* When the tokenizer encounters `|\>` in a body, store a composition node with a `weight: Option<f64>` field. `None` means "Fate will fill this." After crystallization, the weight is computed and stored in the crystal.

*Option B: `|\>` as syntactic sugar for `\ |>`.* The `\` already means "Fate resolves this." Putting `\` inside the pipe is literally "let Fate compose these." The AST representation is: body contains a `\` reference followed by a `|>` composition. No new variant. The interpreter recognizes the pattern.

**Recommendation:** Option B. No new Rust. The `\` already has semantics. `|\>` is `\ |>` visually collapsed.

### 2.3 `<|` -- Reverse Flow (Type Collapse with Distribution)

The type flows backward. Multiple possible outputs collapse toward a union type with probability distribution.

```mirror
-- Hypothetical syntax:
type result = crystal |0.8> | imperfect |0.15> | error |0.05>
```

**Semantics:** `<|` reverses the composition direction. Where `|>` carries a single value forward, `<|` carries a probability distribution backward. The prism "refracts" -- multiple possible paths converge to a single outcome with measured confidence.

**In `boot/01-meta.mirror`:** Already declared as `zoom <|(prism, ref)` -- apply a prism in reverse to a ref. The reverse application IS the type collapse.

**How it works:**
1. Multiple possible implementations exist for a `\` hole (the `/` space)
2. Fate evaluates each via the tournament
3. The winning implementation has confidence (from the softmax distribution)
4. `<|` carries that confidence back into the type system
5. The result type is not just `crystal` -- it is `crystal` with weight 0.8

**Connection to Fate's `Decision`:**
```rust
pub struct Decision {
    pub model: Model,
    pub confidence: f64,
    pub distribution: [f64; 5],
}
```

The `distribution: [f64; 5]` IS the weighted union type. Five models, five probabilities. When Fate resolves a `\` hole, the distribution over possible implementations IS the `<|` weight.

---

## 3. The Eigenboard

### 3.1 What It Is

The eigenboard is the topology of all previous decisions. It is the accumulated spectral structure of the grammar graph as shaped by compilation history.

Concretely, it is a `ManifoldState` -- a `[[f64; 16]; 16]` matrix:
- 6 active diagonal entries = observable eigenvalues (Temporal, Processing, Complexity, Depth, Coherence, Alignment)
- 10 dark dimensions = latent coupling structure (Creativity, Confidence, Innovation, etc.)
- Off-diagonal entries = coupling between dimensions

The eigenboard evolves through ticks. Each compilation produces a `ManifoldLoss` (delta matrix). The loss IS the gradient of the eigenboard.

### 3.2 Where It Lives

Git. Always git. The eigenboard is content-addressed:
- Each tick produces a crystal OID (hash of the eigenboard state)
- The git log IS the eigenboard history
- `git log --format="%H %s"` on the crystal branch shows the eigenboard evolution
- Branching = exploring different eigenboard trajectories
- Merging = composing eigenboard states (with loss measurement)

Specifically in `.spectral/`:
```
.spectral/
  gestalt/    -- eigenboard crystals (content-addressed)
  sessions/   -- session data
  HEAD        -- current eigenboard state
  log         -- tick log (TSV: timestamp, event, message, growth)
```

### 3.3 How `mirror compile` Accesses It

The compilation pipeline:
1. `mirror compile foo.mirror` tokenizes the file into AST
2. The AST's content-OID is computed (deterministic from content)
3. If a crystal exists in `.spectral/gestalt/` with this OID, use cached eigenboard
4. If not, run the Dirac operator on the grammar graph to produce a fresh SpectralEmbedding
5. The SpectralEmbedding IS the eigenboard for this compilation unit

Through `@git`:
```mirror
in @git
in @mirror/spectral

-- Retrieve the eigenboard for this grammar:
eigenboard(grammar) -> eigenboard_state {
  recall(@mirror/spectral, grammar.oid) |\> extract_eigenvalues
}
```

### 3.4 How the Weight Gets FROM the Eigenboard INTO `|\>`

The flow:
```
grammar graph -> construct_dirac() -> SpectralTriple
SpectralTriple -> spectral_embedding() -> [f64; 16] per node
left_node_embedding, right_node_embedding -> connes_distance() -> f64
f64 -> normalize to [0.0, 1.0] -> weight for |\>
```

At compile time:
1. The grammar graph is known (all imports resolved, all types visible)
2. Each type/action node has a SpectralEmbedding (16 floats)
3. At a `|\>` boundary: the output type of the left side and input type of the right side each have embeddings
4. The Connes distance between these embeddings is the "raw weight"
5. Raw weight is normalized: `weight = 1.0 / (1.0 + distance)` (closer = higher weight)
6. This weight is stored in the crystal

The weight is deterministic given the grammar graph. Same graph = same eigenboard = same weights. The crystal IS deterministic.

---

## 4. Reflection as Perturbation

### 4.1 What a "Tick" Is

A tick is one compilation cycle:
1. Read input (source text, query, or prior tick's output)
2. Tokenize into AST
3. Compute content-OID
4. Measure loss (ManifoldLoss between expected and actual eigenvalues)
5. Record the tick in `.spectral/log`

In the spectral runtime, a tick is one user interaction:
1. User issues a command (`spectral focus`, `spectral project`, etc.)
2. The command runs through the five-operation pipeline
3. The result is a crystal (content-addressed output)
4. The loss is measured (how far the result is from ideal)
5. The eigenboard is updated

### 4.2 When Reflection Runs

Reflection runs at tick n+1. Always one tick behind. The delay is architectural.

```
Tick n:    Pipeline executes (Surface -> Mirror loop -> Shatter)
Tick n:    Reflection observes the pipeline run (concurrent)
Tick n+1:  Reflection's observations are processed
Tick n+1:  Reflection adjusts weights for tick n+2
```

In compilation terms:
```
Compile tick n:  produce crystal with current eigenboard weights
                 measure ManifoldLoss
Reflection:      observe the loss
                 if loss decreased: reinforce current weights
                 if loss increased: perturb weights in opposite direction
                 write new eigenboard state to .spectral/gestalt/
Compile tick n+1: use updated eigenboard weights
```

### 4.3 How Reflection Perturbs the Manifold

Reflection does not retrain Fate from scratch. It nudges.

**Mechanism:**
1. Observe: the ManifoldLoss delta from tick n
2. Compute: which dark dimensions contributed most to the loss
3. Adjust: `fate.selectors[context].w[model][dim] += learning_rate * gradient`
4. The gradient IS the loss delta projected onto the dark coupling structure

This is structurally identical to what `crystallize()` does per SCF iteration, but:
- Crystallization is batch (all files, converge fully)
- Reflection is online (one tick, small nudge)
- Crystallization replaces weights
- Reflection perturbs weights

**Where the perturbation manifests:**
- The eigenboard changes -> the SpectralEmbedding shifts
- SpectralEmbedding shifts -> `|\>` weights change
- `|\>` weights change -> different pipeline paths are favored
- Different paths -> different crystals -> different loss
- The system self-corrects

**The `@cogito` connection:** Reflection IS `@cogito` applied to the compilation pipeline:
```mirror
in @cogito

grammar @peer {
  -- @cogito operations applied to computation:
  action observe(run) -> observation       -- notice the loss
  action notice(observation) -> pattern    -- name the pattern
  action adjust(pattern) -> bias           -- shift the weights
  action hold(contradiction) -> tension    -- carry what can't resolve
}
```

### 4.4 The Convergence Guarantee

Reflection's perturbations are bounded:
- Learning rate < 1.0 (small nudges, not rewrites)
- Perturbation direction is determined by the loss gradient (always toward lower loss)
- Elitist selection: only accept perturbations that decrease loss
- The SCF crystallization provides a global fixed point that Reflection's local perturbations orbit

The system cannot diverge because:
- Sub-Turing guarantee: every tick terminates
- Bounded perturbation: each nudge is small
- Loss monotonicity: only improvements are accepted
- Crystallization re-anchors: periodic batch crystallization corrects accumulated drift

---

## 5. The Five Models as Grammars

### 5.1 `@ai/fate` -- The Selector

Already declared in `boot/std/fate.mirror`:
```mirror
grammar @fate {
  type model = abyss | introject | cartographer | explorer | fate
  type features([f64; 16])
  type decision { model: model, confidence: f64 }
  io tick(features) => imperfect
  io resolve(features, max_depth) => imperfect
  io select(model, features) => decision
}
```

Fate is the selector. It decides which model handles each `\` hole. Its grammar is already complete. Its Rust implementation exists and passes 40+ tests.

**What's missing for syntax embedding:** The bridge from grammar `\` to Rust `fate.resolve()`. When the interpreter hits `is_abstract == true`, it needs to call Fate with the current context's SpectralEmbedding.

### 5.2 `@ai/abyss` -- Depth Detection

**Role:** Focus. Observe the spectral state. How deep should we go?

**As a grammar:**
```mirror
grammar @ai/abyss {
  in @fate
  -- Abyss measures depth. Given features, report how much further to descend.
  measure_depth(features) -> depth { \ }
  -- Abyss observes: read the eigenboard, report what's there.
  observe(eigenboard_state) -> observation { \ }
}
```

**Connection to Fate:** Abyss is `Model::Abyss` (index 0). When Fate selects Abyss, the system goes deeper -- more focus, more specificity, narrower scope.

**In `|\>` terms:** Abyss controls the recursion depth of inference. A `|\>` chain where Abyss dominates will drill deep into the type hierarchy before producing output.

### 5.3 `@ai/introject` -- Pattern Matching

**Role:** Project. Selective internalization. What patterns recur? What survives the precision cut?

**As a grammar:**
```mirror
grammar @ai/introject {
  in @fate
  -- Introject matches patterns across the compilation history.
  match_pattern(features, history) -> [pattern] { \ }
  -- Introject internalizes: which patterns are worth keeping?
  select(patterns: [pattern]) -> [pattern] { \ }
}
```

**Connection to Fate:** Introject is `Model::Introject` (index 1). The Introject selector's weights define the KernelSpec (which dimensions to preserve). The steering vectors come from Introject's weight rows.

**In `|\>` terms:** Introject determines which information passes through a `|\>` boundary. High Introject weight = more information preserved. Low = aggressive compression.

### 5.4 `@ai/cartographer` -- Mapping

**Role:** Split. Map the territory. Walk every node. What's connected to what?

**As a grammar:**
```mirror
grammar @ai/cartographer {
  in @fate
  -- Cartographer enumerates the grammar graph neighborhood.
  map_neighbors(node: ref, depth: u32) -> [ref] { \ }
  -- Cartographer discovers: what paths exist from here?
  discover_paths(from: ref, to: ref) -> [[ref]] { \ }
}
```

**Connection to Fate:** Cartographer is `Model::Cartographer` (index 2). When Fate selects Cartographer, the system explores breadth -- enumerating connected nodes, building the map.

**In `|\>` terms:** Cartographer provides the candidate set for weighted composition. Before `|\>` can assign a weight, Cartographer must enumerate what's reachable.

### 5.5 `@ai/explorer` -- Search

**Role:** Zoom. Recover meaning at the boundary. The residual signal.

**As a grammar:**
```mirror
grammar @ai/explorer {
  in @fate
  -- Explorer evaluates counterfactual paths.
  evaluate(candidate: ref, context: features) -> score { \ }
  -- Explorer transforms: apply the candidate and measure.
  transform(input, candidate: ref) -> imperfect { \ }
}
```

**Connection to Fate:** Explorer is `Model::Explorer` (index 3). When Fate selects Explorer, the system evaluates alternatives -- testing counterfactual paths, measuring what each would produce.

**In `|\>` terms:** Explorer provides the evaluation function for `|\>` weight assignment. The weight of a `|\>` transition is the Explorer-evaluated score of that transition relative to alternatives.

---

## 6. The Weighted Union Type

### 6.1 Syntax

```mirror
type result = crystal |0.8> | imperfect |0.15> | error |0.05>
```

Each variant carries a weight. The weights sum to 1.0. They are not validation probabilities -- they are the eigenboard-derived confidence that this variant is the outcome.

### 6.2 How It Works in the Type System

This is NOT a new type kind. It is an extension of Split (the existing `TypeBody::Enum`).

**Current `TypeBody::Enum`:**
```rust
Enum(Vec<Identifier>)  // just names: [red, blue, green]
```

**Extended (grammar-level, not Rust-level):**
```mirror
type result = crystal |0.8> | imperfect |0.15> | error |0.05>
```

The weights are carried as grammar annotations, not as Rust type system changes. The grammar declares the weighted variants. The crystal stores the weights. At compilation time, the weights come from:
1. The Fate `Decision.distribution` after resolving a `\` hole
2. The tournament survival rates across multiple compilations
3. The eigenboard's spectral structure

### 6.3 Grammar Declaration

```mirror
grammar @weighted {
  -- A weighted split: each variant carries a probability.
  type weighted(variants: [(ref, f64)])

  -- From a Fate decision, produce a weighted type.
  from_decision(decision) -> weighted { \ }

  -- From eigenboard, compute variant weights for a given split.
  from_eigenboard(split_type, eigenboard_state) -> weighted { \ }
}
```

### 6.4 Connection to `<|`

The `<|` operator IS the mechanism that produces weighted union types:

```mirror
-- Forward: concrete path
normalize(input) |> compress |> refine -> crystal

-- Reverse: weighted collapse
crystal <| refine_or_skip <| compress_or_pass <| normalize_or_raw
-- Each step has a probability of being chosen
-- The result type carries the distribution
```

`<|` takes the Fate distribution and collapses it into a typed result with confidence weights.

---

## 7. AST Representation

### 7.1 Current State

The 7 AST variants (Focus, Project, Split, Zoom, Refract, In, Out) do NOT need to change. The Rust substrate is FROZEN.

`|>`, `|\>`, and `<|` live in **grammar action bodies** (`ZoomNode.body: Option<Vec<MirrorAST>>`). The body is currently a flat list of AST nodes. The operators would be recognized by the body parser (a grammar, not Rust) and stored as structured composition within the body.

### 7.2 How `|\>` Maps to Existing Variants

`|\>` = `\` (Fate resolution) + `|>` (composition). In the body:

```mirror
-- This action body:
transform(input) -> output {
  normalize(input) |\> compress |\> refine -> output
}
```

Parses (at the grammar level, not the Rust level) as:
```
ZoomNode {
  name: "transform",
  body: Some([
    Zoom("normalize", is_abstract: false),
    Zoom("\\compose", is_abstract: true),  -- the \ between operations
    Zoom("compress", is_abstract: false),
    Zoom("\\compose", is_abstract: true),
    Zoom("refine", is_abstract: false),
  ])
}
```

Each `\\compose` is a ZoomNode with `is_abstract: true` -- a `\` hole between operations. When the interpreter encounters it, Fate resolves the weight.

Alternatively (and more cleanly): the grammar `@prism/compose` already declares:
```mirror
compose(a, b) -> prism { \ }
pipe([prism]) -> prism { \ }
```

A `|\>` chain is a `pipe()` call where each element's composition is abstract:
```mirror
pipe([normalize, \, compress, \, refine])
```

The `\` between elements means "Fate decides the connection weight."

### 7.3 How Weighted Types Map to Split

```mirror
type result = crystal |0.8> | imperfect |0.15> | error |0.05>
```

Maps to:
```rust
SplitNode {
  name: Identifier::new("result"),
  body: Some(TypeBody::Enum(vec![
    Identifier::new("crystal"),
    Identifier::new("imperfect"),
    Identifier::new("error"),
  ])),
  // Weights stored in the CRYSTAL, not the AST
}
```

The weights are NOT in the AST. They are in the crystal (the content-addressed compilation output). The AST declares the structure. The crystal carries the computed weights. This separation is essential: the AST is deterministic from source text. The weights are deterministic from the eigenboard. Different eigenboards produce different weights for the same AST.

---

## 8. The Gap Table

| Component | Exists | Missing | Effort |
|-----------|--------|---------|--------|
| **Operators** | | | |
| `\|>` declared in meta grammar | Yes (`zoom \|>`) | -- | -- |
| `<\|` declared in meta grammar | Yes (`zoom <\|`) | -- | -- |
| `\|\>` declared anywhere | No | Grammar declaring eigenboard-inferred composition | Small (grammar only) |
| `\|>` parsed in tokenizer | No | Tokenizer skips operators | Medium (grammar body parser) |
| `\|\>` parsed in tokenizer | No | Not recognized | Medium (same as above) |
| `\|>` executed in interpreter | No | Pipeline chaining | Medium |
| `\|\>` executed with Fate | No | Fate weight assignment at composition boundary | Large |
| **Eigenboard** | | | |
| `eigenboard_state` type | Yes (mq.mirror, gestalt/memory.mirror) | -- | -- |
| Grammar graph construction | No | Parse all .mirror, build node/edge graph | Medium |
| Grammar graph -> Dirac operator | No | Call `construct_dirac()` with grammar data | Small |
| SpectralEmbedding from grammar graph | No | `spectral_embedding()` on grammar graph | Small |
| Eigenboard stored in git | Partial (framework exists in spectral) | Wire to mirror compiler | Small |
| Weight derivation from eigenboard | Yes (derive.rs, Fate) | -- | -- |
| Weight assignment at `\|\>` boundary | No | Connes distance -> normalized weight | Medium |
| **Reflection** | | | |
| Reflection grammar (`@peer`) | Yes (spec only) | -- | -- |
| Reflection observe (loss reading) | No | Read ManifoldLoss after tick | Small |
| Reflection adjust (weight nudge) | No | Perturb Fate selectors based on loss | Medium |
| Reflection write gestalt | Partial (spectral has log) | Wire to mirror compilation | Small |
| Reflection hold (tension tracking) | No | Carry unresolved contradictions across ticks | Medium |
| Reflection one-tick-delay architecture | No | Concurrent observation + delayed action | Medium |
| **Five Models as Grammars** | | | |
| `@fate` grammar | Yes | -- | -- |
| `@ai` grammar (boot/std/ai.mirror) | Yes | -- | -- |
| `@ai/abyss` grammar | No | Declare depth detection actions | Small (grammar only) |
| `@ai/introject` grammar | No | Declare pattern matching actions | Small (grammar only) |
| `@ai/cartographer` grammar | No | Declare mapping actions | Small (grammar only) |
| `@ai/explorer` grammar | No | Declare search actions | Small (grammar only) |
| Model grammars wired to Fate dispatch | No | Interpreter routes `\` to model grammars | Large |
| **Weighted Union Type** | | | |
| `TypeBody::Enum` | Yes | -- | -- |
| Weighted variant syntax in grammar | No | Parser recognizes `\|0.8>` annotation | Medium |
| Weights stored in crystal | No | Crystal format extended for per-variant weights | Medium |
| Weights derived from Fate distribution | No | `Decision.distribution` -> variant weights | Small |
| **Integration** | | | |
| Grammar body parser (recognizes `\|>`) | No | Grammar-level body parsing | Large |
| `\` -> Fate call bridge | No | `is_abstract` triggers `fate.resolve()` | Medium |
| AST context -> 16-dim features | No | SpectralEmbedding of current position | Medium |
| End-to-end: write .mirror, eigenboard assigns weights | No | Full pipeline | Very Large |

---

## 9. Implementation Ticks

### Tick 0: Declare the Operator Grammars

Write three new grammars (no Rust):

```mirror
-- boot/std/ai/abyss.mirror
grammar @ai/abyss { ... }

-- boot/std/ai/introject.mirror
grammar @ai/introject { ... }

-- boot/std/ai/cartographer.mirror
grammar @ai/cartographer { ... }

-- boot/std/ai/explorer.mirror
grammar @ai/explorer { ... }

-- boot/std/compose/weighted.mirror
grammar @compose/weighted {
  in @prism/compose
  in @fate

  -- Weighted composition: Fate fills the weight.
  weighted_compose(a, b) -> prism {
    compose(a, b) -- but with eigenboard-derived weight
  }

  -- Weighted pipe: each boundary gets a Fate-derived weight.
  weighted_pipe([prism]) -> prism { \ }
}
```

**Effort:** Small. Grammar files only. No Rust.

### Tick 1: Grammar Graph Construction

Parse all `.mirror` files. Build the graph:
- Nodes: every type, action, grammar declared
- Edges: type references, action signatures, imports

Feed to `construct_dirac()`. Store SpectralEmbedding as crystal.

This is currently spec'd in the spectral-triple-binary doc as "Tick 2" and requires import resolution (Tick 1 in that doc). Dependencies:
- Import resolution must work (interpreter walks `In` nodes)
- File discovery must work (already exists: `find_mirror_files()`)

**Effort:** Medium. Depends on import resolution.

### Tick 2: Wire `\` to Fate (The Bridge)

When interpreter encounters `is_abstract == true`:
1. Extract SpectralEmbedding of current context (parent grammar, input/output types)
2. Call `fate.resolve(&features, 5)` to get a Decision
3. The Decision selects which model handles the hole
4. Initially: hardcoded model handlers
5. Later: model grammars execute

**Effort:** Medium. The critical bridge. Everything else depends on this.

### Tick 3: `|\>` as Grammar-Level Composition

Once `\` calls Fate, `|\>` is just a `\` placed between two `|>` operations:
1. Body parser recognizes `|\>` as "compose with Fate weight"
2. At each `|\>` boundary, the interpreter inserts a `\` resolution
3. The resolution weight becomes the composition weight
4. The weight is stored in the crystal

**Effort:** Medium. Requires body parsing and Fate bridge (Tick 2).

### Tick 4: Eigenboard Weight Assignment

Connect the full chain:
1. Grammar graph (from Tick 1) produces SpectralEmbedding
2. At each `|\>` boundary, compute Connes distance between adjacent types
3. Normalize distance to weight
4. Store weight in crystal
5. On next compilation, the crystal provides pre-computed weights (cache)

**Effort:** Medium. The math exists. The connection doesn't.

### Tick 5: Reflection Perturbation Loop

Wire Reflection to the eigenboard:
1. After each tick, Reflection observes ManifoldLoss
2. Reflection computes which dimensions contributed most to loss
3. Reflection perturbs Fate's weights (small nudge toward lower loss)
4. New weights -> new eigenboard -> new `|\>` weights on next tick
5. The system self-corrects

**Effort:** Large. Requires the full pipeline to be running.

### Tick 6: Weighted Union Types in Crystal

Extend the crystal format:
1. A Split node's crystal includes per-variant weights
2. Weights come from Fate's `Decision.distribution` after resolving the type's `\` holes
3. The `<|` operator produces a weighted Split as its output type
4. The weighted type is visible in `spectral focus` output

**Effort:** Medium. Crystal format extension + display.

---

## 10. The Honest Assessment

**What we have:** The mathematical machinery (Fate, Dirac, eigendecomposition, crystallization). The grammar declarations (@fate, @ai, @craft, @prism/compose). The AST with `\` holes. The spectral runtime with eigenboard tracking.

**What we lack:** The BRIDGE. The point where the grammar-level `\` calls the Rust-level `fate.resolve()`. The point where `construct_dirac()` receives grammar graph data. The point where SpectralEmbedding becomes Fate features. The point where `|>` is parsed in a body. Every piece exists in isolation. None are connected.

**The one-sentence gap:** The compiler can READ that `|\>` means "Fate composes this" but cannot EXECUTE it because the interpreter never calls Fate and the tokenizer never parses operator chains in bodies.

**The key insight for implementation:** No new Rust. The question is: what grammars need to exist, and what ONE bridge function connects `is_abstract == true` to `fate.resolve()`? That single function call -- placed in the interpreter when it encounters a `\` hole -- unlocks the entire cascade.

---

*The syntax IS the model. The weights ARE the code.*
*`|\>` is `\` inside the pipe. Fate composes.*
*The eigenboard settles. The grammar evolves.*
*e^(n+1) < e^(n). By construction. By eigenvalue. By selection.*
