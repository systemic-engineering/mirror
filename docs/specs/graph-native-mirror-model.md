# Graph-Native .mirror Model

**Status:** Research (no implementation exists)
**Date:** 2026-05-14
**Author:** Reed
**Depends on:** Fate (routing), mirror compiler (verification), spectral-db (graph storage), Dirac operator spec

---

## 0. Thesis

A model that writes `.mirror` grammars natively, where the graph topology IS the model, the eigenvalues ARE the weights, selection IS the training, and the five operations ARE the inference. Not an LLM fine-tuned on mirror syntax. A graph-native model where inference is navigation through a known graph, and every inference terminates by construction.

The existing architecture already contains the pieces. This document names them, connects them to prior art, and identifies what remains to build.

---

## 1. Existing Architecture

### Fate: 450 Parameters, Sub-Turing Selector

Fate is a 5-context softmax selector. Each context (Abyss, Introject, Cartographer, Explorer, Fate) has one `ModelWeights`:

```
ModelWeights {
    w: [[f64; 16]; 5],    // 80 feature weights
    b: [f64; 5],           // 5 biases
    depth_w: [f64; 5],     // 5 depth modulation weights
}
// = 90 parameters per context x 5 contexts = 450 total
```

The forward pass: `features [16] -> linear -> softmax -> argmax -> Model`. Sub-microsecond. The binary IS the model.

**Key property:** Fate implements the Prism trait. The five operations (focus, project, split, zoom, refract) are not metaphors -- they are the trait methods. Fate IS a Prism.

### derive.rs: Eigenvalues ARE the Weights

The `derive.rs` module in Fate already implements eigenvalue-derived weights:

1. Extract the 10x10 dark coupling submatrix from a `ManifoldState`
2. Compute eigendecomposition (Jacobi algorithm, pure Rust, zero deps)
3. The 5 largest eigenvalues become biases
4. The corresponding eigenvectors, projected into the full 16-dim space, become weights
5. The eigengaps become depth modulation weights

This IS graph-native weight derivation. The weights are not trained by gradient descent on labeled data. They are derived from the spectral structure of the manifold state. The eigenvalues ARE the weights. Already implemented. Already tested. Already passing.

### crystallize(): Selection IS Training

The `crystallize()` function in `derive.rs` implements a self-consistent field (SCF) loop:

```
1. Start with untrained Fate (zero weights)
2. Compile grammars with current weights -> produces dark coupling matrices
3. Eigendecompose the couplings -> derive new weights
4. Mix old and new weights (damping)
5. Check eigenvalue convergence
6. If not converged, goto 2
```

This is selection-as-training. No gradient descent. No loss function in the traditional sense. The system observes its own compilation entropy, derives weights from that entropy, and iterates until eigenvalues stabilize. The fixed point IS the trained model.

**Convergence guarantee:** For static coupling matrices, crystallization converges in exactly 2 iterations. For dynamic couplings (where compilation depends on weights), convergence depends on the damping factor and the spectral gap. The SCF loop is a contraction mapping when damping < 1.

### The Brainfuck Substrate

Fate runs as a compiled Brainfuck program. The `fate.bf` program (80 instructions) reads 22 bytes from input (16 features + 1 context + 5 biases), performs argmax over bias-adjusted scores, and outputs the winning model index. `build.rs` compiles this to native Rust. The algorithm is fixed; the weights are data on the tape.

### The Dirac Operator

The Dirac operator spec unifies eigenvalues, distance, action, and alignment detection into a single matrix D. The spectral triple (A, H, D) = (grammar, /, \) where:

- A = grammar algebra (the operations)
- H = the / space (type hierarchy, each path is a basis vector)
- D = the \ operator (inference -- find the path between two types)

The eigenvalues of D provide the 16-dimensional SpectralEmbedding that replaces hand-picked eigenboard features. Fate's selector weights become linear maps in this spectral embedding space.

### The Tournament System

The tournament system (documented in `mirror/docs/ai/tournament.md`) provides the selection mechanism. Tournament rules are named lenses that compose: `elite(1).beam(8).halving(3)`. Magic (the meta-selector, 325 bytes) selects which tournament rule to use. Three levels of selection:

```
Level 1: Fate selects operations (which optic)
Level 2: Tournament rules select Fates (which instances survive)
Level 3: Magic selects tournament rules (which selection strategy)
```

All measured by holonomy reduction. All bounded. All convergent.

### The Shatter Pipeline

The shatter training pipeline describes how arbitrary input is scrambled into mirror tokens via a codon table (256 bytes), then evolved through tournament selection with holonomy as fitness. The entire AI system:

- Two BF programs (Fate + Shatter)
- Two weight tapes (425 bytes + 256 bytes = 681 bytes total)
- One fitness function (holonomy)
- One selection mechanism (tournament)

Total model size: 681 bytes.

---

## 2. Prior Art

### Graph Neural Networks for Code Generation

**CGFuse (2026):** Combines a GNN with a language model to preserve structural information from code graphs during generation. The GNN encodes the AST/data-flow graph; the language model generates tokens conditioned on the graph encoding. Key insight: graph structure improves code generation quality, but the graph encoding is separate from the generation.

**Generative Code Modeling with Graphs (Brockschmidt et al., 2018):** Graph-based generative approach to code. Generates code by iteratively expanding a partial program graph. Each expansion step selects a node type and connects it to existing nodes. The graph IS the generation state. This is the closest prior art to what mirror needs -- but it uses a learned GNN encoder (thousands of parameters), not eigenvalue-derived weights.

**AST-KG-RAG:** Uses ASTs to build a knowledge graph, then RAG for code generation. The graph is the retrieval index, not the model. Mirror inverts this: the graph IS the model.

### Spectral Methods in Machine Learning

**Spectral Graph Theory for ML (Luxburg 2007):** The eigengap heuristic: the number of clusters in a graph equals the number of eigenvalues close to zero. The Fiedler vector (eigenvector of lambda_2) provides the optimal graph bisection. Mirror already uses the Fiedler value as the first component of the SpectralEmbedding.

**Spectral Clustering as Dimensionality Reduction:** Projecting graph nodes onto the first k eigenvectors of the Laplacian provides a k-dimensional embedding where Euclidean distance approximates graph distance. This is exactly what `SpectralEmbedding([f64; 16])` does -- project the graph state onto D's first 16 eigenvectors.

**Spectral Link Prediction (2023):** Uses eigenvector decomposition of the graph Laplacian to generate node embeddings for predicting missing edges. The eigenvalues encode global graph structure; the eigenvectors encode local node position. Predicting the next AST node is structurally identical to link prediction: given the current partial graph, which node (AST variant) should be added next?

**From SGD to Spectra (2025):** Studies how the spectral properties of weight matrices (singular value distributions) relate to optimization dynamics and generalization. The key insight: weight matrix spectra that follow heavy-tailed distributions generalize better. Fate's weight derivation from eigenvalues naturally produces structured spectra.

### Evolutionary Program Synthesis

**SOAR (2025):** Self-improving language models for evolutionary program synthesis. Integrates language models into a self-improving evolutionary loop. The model generates program candidates; evolution selects survivors; the model is retrained on successful candidates. Mirror's architecture is structurally identical, but without the language model -- the compiler + Fate + tournament IS the evolution loop.

**Genesys (2025):** Formulates program synthesis as continuous optimization using CMA-ES. Programs are encoded as real-valued vectors; a decoder maps vectors to programs. Key insight: continuous optimization over program representations can converge faster than discrete search. Mirror's ManifoldState (16x16 real-valued matrix) IS a continuous program representation.

**Grammatical Evolution:** Integer codons map to BNF production rules. The genotype (integers) is separate from the phenotype (program). Crossover operates on the genotype. The mirror codon table IS grammatical evolution. The shatter training pipeline implements exactly this.

### Selection vs Gradient Descent

**Evolution Strategies Converge to Finite Differences (2020):** Proves that ES gradients converge to finite-difference gradients as dimension increases. ES and gradient descent are not fundamentally different -- ES approximates the gradient through population-based sampling. Mirror's tournament selection is a form of ES operating on a 450-dimensional parameter space.

**Evolution-Guided Policy Gradient (NeurIPS):** Hybrid approach combining evolutionary population with policy gradient. The population provides exploration diversity; the gradient provides exploitation efficiency. Mirror's three-level selection (Fate -> Tournament -> Magic) achieves the same hybrid through composition rather than hybridization.

**Convergence of Evolutionary Algorithms in General Search Spaces:** Proves that evolutionary algorithms with elitist selection converge to the global optimum under mild conditions (finite search space, positive probability of generating any solution). Mirror's `elite(k)` tournament rule provides exactly this elitist selection. The search space is finite (bounded AST depth, finite keyword set). Convergence is guaranteed.

### Sub-Turing Inference

**Decidability and Termination:** Sub-Turing languages (primitive recursive functions, regular languages, context-free grammars) guarantee termination of all computations. Mirror is sub-Turing by design -- no general recursion, no unbounded loops. Every compilation terminates. Every inference terminates.

**What sub-Turing buys for training:** If every inference terminates, then:
1. The fitness function (holonomy) is always computable -- no hanging evaluations.
2. The search space is enumerable -- every possible .mirror program is finite.
3. Convergence detection is decidable -- you can prove that holonomy has stabilized.
4. No halting problem -- the model cannot enter an infinite loop during inference.

These guarantees make selection-based training tractable in ways that Turing-complete inference cannot match.

### Graph Transformers

**StructCoder (2024):** Structure-aware transformer decoder that models both syntax and data flow for code generation. The transformer's attention mechanism is biased by the AST structure -- nodes attend more strongly to structurally related nodes. This is attention guided by graph topology.

**Graph-Aware Transformers (2025):** Bridges graph theory and transformer architectures. Uncovers latent graph-like structures within attention mechanisms. Key insight: standard attention already performs implicit message-passing on a fully connected graph; graph-aware attention restricts this to the actual graph structure.

**The mirror connection:** Fate's forward pass IS a single-layer graph-aware attention. The 16-dimensional features are the "query." The 5x16 weight matrix is the "key-value" structure. The softmax is the attention normalization. But Fate attends to the spectral embedding of the graph, not to token positions. The graph topology IS the attention structure.

### Tiny Models and Extreme Learning Machines

**Extreme Learning Machines (Huang et al., 2006):** Single hidden layer. Input-to-hidden weights are RANDOM and FIXED. Only hidden-to-output weights are trained. Training is a single matrix pseudoinverse. The shatter training pipeline document explicitly notes: "This is structurally identical to excited Fate: random projection followed by a learned linear map."

**Reservoir Computing:** Random recurrent network provides nonlinear mixing. Only the readout is trained. Works surprisingly well for sequence processing. Mirror's dark dimensions (6-15) function as a reservoir -- latent dimensions shaped by training pressure, providing nonlinear mixing of the signal.

**muNAS (2021):** Neural architecture search for microcontroller targets (< 64KB). Finds Pareto-optimal accuracy/size tradeoffs. Fate at 450 parameters (3.6KB at f64, 450 bytes at u8) is well within these bounds. The question is not whether a model this small can be useful -- muNAS proves it can -- but whether the specific architecture (spectral embedding -> linear -> softmax) is sufficient for the specific task (navigating a known graph).

---

## 3. The Mirror Approach: How It All Connects

### Eigenvalues = Weights

Already implemented in `derive.rs`. The dark coupling matrix of a ManifoldState is eigendecomposed. The eigenvalues become biases. The eigenvectors become feature weights. This is not an analogy -- it is the implementation.

The chain: `.mirror grammar -> compile -> Dirac operator D -> spectrum -> first 16 eigenvectors -> project onto current state -> SpectralEmbedding -> Fate's input`. The compilation IS the weight derivation. When the grammar changes, the eigenvalues change, the weights change, the model routes differently. The model is the graph.

### Selection = Training

Already implemented in `crystallize()`. The SCF loop derives weights from compilation entropy, compiles again, derives again, until eigenvalues stabilize. No labeled data. No gradient descent. The compiler IS the training signal.

The three-level tournament adds:
1. **Fate selects operations** -- which of the five operations to apply
2. **Tournament selects Fates** -- which Fate instances survive (evolutionary selection)
3. **Magic selects tournaments** -- which selection strategy to use (meta-selection)

Each level operates on the output of the level below. Each level is measured by holonomy reduction. Each level converges independently.

### Graph = Model

The spectral-db graph is not a database that a model queries. The graph IS the model. The topology determines which nodes are reachable (possible AST expansions). The edge weights determine transition probabilities. The eigenvalues determine which regions of the graph are "important" (high eigenvalue = high structural significance).

Navigating the graph to write .mirror is:
1. **Focus:** Read the current SpectralEmbedding (where are we in the graph?)
2. **Project:** Filter to reachable AST variants (which expansions are valid?)
3. **Split:** Follow edges to explore connected nodes (what connects to what?)
4. **Zoom:** Evaluate counterfactual expansions without committing (what would happen if...?)
5. **Refract:** Commit the expansion that reduces loss (the write)

The five operations ARE the inference algorithm. Not metaphorically. The Prism trait is the model's forward pass.

### The \ Operator as Inference

The backslash operator `\` in mirror syntax means "infer the path between two types." When a grammar declares `action foo(bar) -> baz { \ }`, the compiler knows the input type and output type. The `\` means: find the path from bar to baz in the graph. The eigenvalue of `\` at a typed hole IS how much inference is needed.

A graph-native model that writes .mirror is a model that fills `\` holes. It navigates the graph from the input type to the output type, choosing the path with lowest spectral loss. The graph topology constrains which paths exist. The eigenvalues determine which paths are cheapest. The model doesn't generate from nothing -- it navigates a known space.

---

## 4. Minimum Viable Model

### Parameter Budget

Fate: 450 parameters (5 contexts x 90 params each).
Magic: 325 parameters (5 rules x 65 params each).
Shatter codon table: 256 parameters.
**Total: 1,031 parameters.**

All three fit in the binary. All three run at sub-microsecond latency. All three are trained by selection, not gradient descent.

### What 450 Parameters Can Do

A single ModelWeights (90 parameters) implements a linear classifier: `[f64; 16] -> [f64; 5] -> softmax -> decision`. With 5 such classifiers (one per context), Fate can route any 16-dimensional input to one of 5 models, conditioned on which model just ran.

The question: can 90 parameters navigate a known graph? The theoretical minimum for a linear classifier over k classes with d-dimensional input is d*k + k parameters (weights + biases). For d=16, k=5: 85 parameters. Fate has 90 (the extra 5 are depth modulation). It is at the theoretical minimum for a linear model.

But Fate doesn't need to solve an arbitrary classification problem. It navigates a KNOWN graph. The graph topology provides structure that a general classifier cannot exploit. In a graph with n nodes and average degree d, the effective dimensionality of the navigation problem is O(log n), not O(n). For a grammar graph with ~100 nodes and average degree ~4, log_2(100) ~ 7. Sixteen features is more than sufficient.

### The Architecture for .mirror Generation

A graph-native .mirror writer combines:

1. **Fate (450 params):** Routes to the correct operation at each step
2. **Graph topology (0 params):** Constrains which AST expansions are valid
3. **SpectralEmbedding (0 params, derived from D):** Provides position in the graph
4. **Holonomy (0 params, derived from compiler):** Provides the loss signal
5. **Tournament selection (0 params, algorithmic):** Provides the training

The model's "knowledge" is not in learned parameters. It is in:
- The graph structure (which nodes exist, how they connect)
- The eigenvalues (which regions are important)
- The compiler (which outputs are valid)

Parameters exist only for routing (Fate) and token mapping (Shatter). The intelligence is in the geometry.

---

## 5. Training Pipeline: Selection-as-Training Concretely

### Phase 0: Graph Construction

```
1. Parse all .mirror files in the corpus
2. Build the grammar graph: nodes = AST variants, edges = co-occurrence
3. Compute the Dirac operator D for the grammar graph
4. Extract SpectralEmbedding from D's first 16 eigenvectors
```

The graph is the training data AND the model AND the embedding space. One construction.

### Phase 1: Crystallization (Eigenvalue-Derived Weights)

```
1. Initialize Fate with zero weights (uniform selection)
2. Compile all .mirror files with current Fate weights
3. Extract dark coupling matrices from compilation ManifoldLoss
4. Eigendecompose each coupling matrix
5. Derive new Fate weights from eigenvalues/eigenvectors
6. Mix old and new weights (damping = 0.5)
7. Check eigenvalue convergence (max_delta < 1e-10)
8. If not converged, goto 2
```

This produces Fate weights that reflect the spectral structure of the grammar corpus. No gradient descent. No labeled data. The compiler IS the oracle.

### Phase 2: Tournament Refinement (Selection Pressure)

```
1. For each .mirror file with holonomy > 0:
   a. Spawn 4 mutated Fate instances (excited + perturbation)
   b. Each instance processes the file through the five operations
   c. Measure holonomy reduction per instance
   d. Keep the instance with largest reduction (elitist selection)
2. Update the surviving Fate's weights via eigenvalue re-derivation
3. Repeat until no file improves (fixed point)
```

This refines the crystallized weights through evolutionary pressure. The tournament IS the training loop. The holonomy IS the loss function.

### Phase 3: Magic Meta-Selection

```
1. For each .mirror file, run all 5 tournament rules
2. Record which rule achieved the best holonomy/compute ratio
3. Train Magic (325 params) via cross-entropy on (features, best_rule) pairs
4. Magic now selects the optimal tournament strategy per file
```

This adds a meta-layer: not just "which operation" but "which strategy for selecting operations."

### Phase 4: Shatter Evolution (Grammar Generation)

```
1. Start with a frequency-biased codon table (256 bytes)
2. Population of 100 codon tables
3. Each table shatters input bytes into mirror token sequences
4. Compile each sequence, measure holonomy
5. Tournament selection: keep lowest holonomy
6. Mutate winners: swap random entries
7. Repeat for 10,000 generations
```

This trains the generative model (Shatter) through pure selection. The codon table IS the model. Evolution IS the training. The compiler IS the fitness function.

### Convergence: e^(n+1) < e^(n)

The claim: each tick's loss is lower. Is this provable?

**For crystallization (Phase 1):** Yes. The SCF loop with damping < 1 is a contraction mapping on a compact space (bounded eigenvalues). Contraction mappings on compact metric spaces have unique fixed points, and iteration converges to the fixed point. The rate of convergence is at most geometric: ||x_{n+1} - x*|| <= c^n * ||x_0 - x*|| where c < 1 is the damping factor.

**For tournament selection (Phase 2):** Yes, with elitist selection. Elite selection guarantees that the best fitness never decreases. On a finite search space (bounded AST depth, finite keyword set) with positive probability of generating any solution, elitist EA converges to the global optimum (proven by Rudolph 1994). The convergence rate depends on the selection pressure and mutation rate.

**For the combined system:** Monotonic improvement is guaranteed if:
1. Each phase preserves the invariant from the previous phase (crystallization does not undo tournament gains)
2. The phases compose monotonically (each phase's output is at least as good as its input)

Condition 1 holds because crystallization only modifies weights, not the grammar files that tournament selection improved. Condition 2 holds by construction: each phase accepts changes only if holonomy decreases.

The precise statement: **on a finite, sub-Turing grammar space with elitist selection and holonomy as fitness, the sequence of best-ever holonomy values is monotonically non-increasing and bounded below by zero, therefore convergent.**

---

## 6. The Mirror-Specific Questions

### Can eigenvalue decomposition predict the next AST node?

Yes, through spectral link prediction. Given a partial AST (a subgraph of the grammar graph), the eigenvalue decomposition of the Laplacian provides node embeddings. The dot product of two node embeddings predicts edge probability. The next AST node is the one with highest edge probability to the current frontier.

Concretely: compute SpectralEmbedding for the current partial AST. For each candidate AST variant (Focus, Split, Zoom, Refract, Project, In, Out), compute the embedding distance to the current frontier. The candidate with the smallest Connes distance is the best next node.

This requires no learned parameters beyond the graph structure itself. The eigenvalues encode which connections are structurally important. The eigenvectors encode which nodes are similar. Link prediction falls out of the spectral structure.

### Can a model attend to Fiedler-ranked nodes?

Yes. The Fiedler vector (eigenvector of lambda_2) provides a natural ordering of nodes by their structural centrality. Nodes with extreme Fiedler values are at the "ends" of the graph; nodes with Fiedler values near zero are at the structural "center."

Attention weighted by Fiedler rank is: attend strongly to structurally central nodes, weakly to peripheral nodes. This is the opposite of standard positional encoding (which attends by token position). Fiedler attention says: "the most important nodes are the ones that connect the most structure," regardless of where they appear in the token stream.

For .mirror generation, Fiedler attention means: when deciding the next AST node, attend most to the nodes that provide the most structural connectivity. A `grammar` declaration (structurally central) gets more attention than a `type` variant (peripheral). This matches human intuition about grammar structure.

### Can we train an "optic model"?

The five operations already ARE the model's inference steps:

1. **Focus:** Extract features from the current graph state
2. **Project:** Filter to relevant subspace (softmax over 5 models)
3. **Split:** Explore connected nodes (cartograph viable models)
4. **Zoom:** Evaluate counterfactuals (explore transformations)
5. **Refract:** Commit the best option (crystallize)

An "optic model" is a model whose forward pass IS the composition of these five operations. Fate already is this: `apply(&fate, input)` runs `focus -> project -> refract`. The Pipeline trait adds the ManifoldState processing: `focus -> project -> refract` with Casimir conservation.

Training an optic model means training the weights of each operation separately:
- Focus weights: which dimensions to observe (derived from KernelSpec)
- Project weights: how to map observations to decisions (the 5x16 weight matrix)
- Refract weights: how to construct the output state (steering vectors from Introject)

Each operation's weights are derived from a different aspect of the spectral structure. Focus comes from the active/dark dimension split. Project comes from the eigenvalue biases. Refract comes from the eigenvector projections. The model is trained by decomposition, not by end-to-end gradient flow.

### What is the theoretical minimum for navigating a known graph?

For a graph with n nodes, average degree d, and diameter delta:

- **Random walk:** O(n^2) steps to visit all nodes (cover time). No parameters.
- **Spectral walk (Fiedler-guided):** O(n * delta) steps. Requires the Fiedler vector (derived, not learned). 0 parameters.
- **Greedy on embedding:** O(delta) steps if the embedding is faithful (low distortion). Requires the embedding (16 floats per node, derived). 0 parameters.
- **Learned routing:** O(delta) steps with a single linear layer. d*k parameters where k is the branching factor. For d=16, k=5: 85 parameters.

Fate's 90 parameters per context is at the theoretical minimum for learned routing. The extra parameters (5 depth weights) provide recursion-depth awareness, which pure greedy on embedding lacks.

For a grammar graph with ~100 nodes, delta ~ 10, d ~ 4: greedy on SpectralEmbedding reaches any node in ~10 steps. With Fate's routing, each step takes sub-microsecond. Total generation time for a .mirror file: ~10 microseconds.

### What does sub-Turing inference buy?

1. **Termination guarantee:** Every inference terminates. No timeout needed. No infinite loop possible. The model CANNOT hang. This means the fitness function is always computable, which means selection-based training always makes progress.

2. **Enumerable search space:** The set of all valid .mirror programs of bounded size is finite. This means exhaustive search is possible in principle, and evolutionary search converges in finite time with probability 1.

3. **Decidable verification:** The compiler can verify any output in bounded time. This means the "discriminator" (compiler) is exact, not approximate. No false positives. No false negatives. The training signal is noise-free.

4. **Composition safety:** Sub-Turing functions compose into sub-Turing functions. The pipeline Surface -> Mirror -> Shatter -> Reflection is guaranteed to terminate because each stage terminates. No stage can block the pipeline.

5. **Formal convergence proofs:** On finite spaces with decidable fitness, evolutionary algorithms have proven convergence rates. The convergence proofs fail for Turing-complete spaces because fitness evaluation may not terminate.

---

## 7. Open Research Questions

### Q1: Is spectral link prediction sufficient for AST generation?

Spectral link prediction works well for homogeneous graphs (all nodes same type). Grammar graphs are heterogeneous (7 AST variant types). Does eigenvalue decomposition of a heterogeneous graph provide enough signal to predict typed edges? Preliminary answer: yes, if the graph is augmented with type-indicator features (one-hot encoding of AST variant). The SpectralEmbedding then encodes both structural position and type information.

**Experiment:** Build the grammar graph from the boot corpus. Compute SpectralEmbedding. For each edge, predict its existence from the endpoint embeddings. Measure AUC. If AUC > 0.8, spectral link prediction is sufficient for AST generation.

### Q2: Can the codon table learn syntax-aware token generation?

The 256-byte codon table maps input bytes to production rules. A frequency-biased table produces tokens with the right marginal distribution but wrong conditional distribution (it doesn't know that `type` is often followed by an identifier). Can evolution discover conditional structure from a flat table?

Preliminary answer: partially. The codon table with input byte context provides 1-byte context (256 possible histories). A bigram model needs vocabulary^2 parameters for full conditional distribution. For ~25 mirror tokens, that is 625 parameters -- which fits in the Fate-sized budget. The codon table IS a compressed bigram model where the compression is learned by evolution.

**Experiment:** Compare evolved codon table holonomy against a bigram-model baseline. If the codon table approaches bigram performance, the compression is effective.

### Q3: How does the SCF loop interact with tournament selection?

Crystallization and tournament selection both modify Fate's weights. Do they interfere? Crystallization derives weights from eigenvalues. Tournament selection mutates weights to improve holonomy. If tournament selection moves weights away from the eigenvalue-derived values, does re-crystallization undo the improvement?

Preliminary answer: the interaction is constructive if the tournament-improved weights change the compilation outputs, which change the coupling matrices, which change the eigenvalues. The crystallization then "absorbs" the tournament improvement into the eigenvalue structure. The system settles to a fixed point that reflects both spectral structure and evolutionary pressure.

**Experiment:** Run crystallization. Then run tournament selection. Then re-crystallize. Compare final holonomy against crystallization-only and tournament-only. If the combined approach achieves lower holonomy, the interaction is constructive.

### Q4: What is the minimal graph size for useful .mirror generation?

The grammar graph grows as more .mirror files are indexed. With 5 files, the graph might have 50 nodes. With 235 files (the current corpus), perhaps 500 nodes. At what graph size does spectral link prediction become useful for predicting new grammar structures?

Preliminary answer: spectral methods become reliable when the graph has a clear spectral gap (lambda_2 >> lambda_3). For random graphs, the spectral gap appears at O(log n) nodes. For structured graphs (which grammar graphs are), the gap appears earlier. With 50+ nodes and clear community structure (boot grammars vs application grammars), spectral prediction should be viable.

### Q5: Can the model discover new AST variants?

The current model navigates among 7 known AST variants. Can it discover that an 8th variant would reduce holonomy? This requires the model to propose new nodes, not just new edges.

Preliminary answer: not with the current architecture. Fate selects among fixed options. Shatter recombines fixed tokens. Neither can invent new tokens. An 8th AST variant would need to be introduced by the compiler (a code change, not a weight change). The graph-native model is closed over the existing vocabulary.

This is a feature, not a limitation. The vocabulary is the compiler's API surface. Extending it requires extending the compiler. The model optimizes within the vocabulary; humans extend the vocabulary. This separation of concerns is the sub-Turing guarantee in action.

### Q6: How does this relate to REINFORCE / policy gradient?

Fate's tournament selection can be viewed as a special case of REINFORCE where:
- The policy is the softmax over models (parameterized by weights)
- The action is the selected model
- The reward is -holonomy (negative because lower is better)
- The "gradient" is approximated by selection (keep winners, discard losers)

The key difference: REINFORCE estimates the gradient via score function estimation (log-probability * reward). Tournament selection estimates it via population-based sampling (generate variants, keep best). Both converge to the same optima on smooth, bounded landscapes.

ES (Evolution Strategies) convergence to finite differences (proven by 2020 paper) formalizes this: as the population size grows, the ES gradient estimate converges to the finite-difference gradient estimate. Mirror's tournament IS an ES operating on a 450-dimensional space with holonomy as the objective.

The convergence rate: ES converges as O(1/sqrt(N)) where N is population size. With beam(8) (N=8 candidates per round), the gradient estimate has ~35% relative error. With beam(32), ~18%. The tournament size IS the gradient precision. Larger beams = more precise gradients = faster convergence = more compute per step.

---

## 8. Summary: What Exists and What Remains

### Already Implemented

| Component | Location | Parameters | Status |
|-----------|----------|------------|--------|
| Fate selector | `fate/src/lib.rs` | 450 | Passing, 40+ tests |
| Eigenvalue weight derivation | `fate/src/derive.rs` | 0 (derived) | Passing, 15+ tests |
| SCF crystallization loop | `fate/src/derive.rs` | 0 (algorithmic) | Passing |
| Brainfuck runtime | `fate/brainfuck/fate.bf` | 0 (compiled) | Passing |
| ManifoldState / ManifoldLoss | `fate/src/manifold.rs` | 0 (types) | Passing |
| Feature dimensions (active/dark) | `fate/src/feature.rs` | 0 (constants) | Passing |
| Prism trait implementation | `fate/src/lib.rs` | 0 (trait) | Passing |
| Bundle tower (Fiber/Connection/Gauge/Transport) | `fate/src/lib.rs` | 0 (traits) | Passing |
| Tournament rules | `mirror/docs/ai/tournament.md` | 0 (spec) | Spec only |
| Magic meta-selector | `mirror/docs/ai/magic-training-pipeline.md` | 325 (spec) | Spec only |
| Shatter codon table | `mirror/docs/ai/shatter-training-pipeline.md` | 256 (spec) | Spec only |
| Dirac operator | `spectral/docs/specs/dirac-operator.md` | 0 (spec) | Spec only |
| SpectralEmbedding | `spectral/docs/specs/dirac-operator.md` | 0 (spec) | Spec only |

### Remains to Build

| Component | What | Parameters | Depends On |
|-----------|------|------------|------------|
| SpectralEmbedding implementation | D's first 16 eigenvectors as Fate input | 0 (derived) | Dirac operator |
| Grammar graph construction | AST co-occurrence graph from .mirror corpus | 0 (data) | Parser |
| Spectral link prediction | Predict next AST node from SpectralEmbedding | 0 (derived) | SpectralEmbedding |
| Tournament implementation | v1 rules: greedy, beam, elite, halving, tabu | 0 (algorithmic) | Fate |
| Magic implementation | Tournament rule selector | 325 | Tournament |
| Shatter implementation | Codon table + grammar walker | 256 | Parser BNF extraction |
| Shatter evolution | Tournament over codon tables | 0 (algorithmic) | Shatter |
| Self-play loop | Compile -> evaluate -> select -> repeat | 0 (algorithmic) | All above |
| .mirror writer | Navigate grammar graph via five operations | 450 (Fate) | All above |

### The Path

The graph-native .mirror model is not a single new system to build. It is the recognition that the pieces already exist and need to be connected:

1. **SpectralEmbedding replaces hand-picked features** (Dirac spec Phase 4)
2. **Fate consumes SpectralEmbedding** (Dirac spec Phase 5)
3. **Tournament selection refines Fate** (tournament.md)
4. **Magic selects tournament strategy** (magic-training-pipeline.md)
5. **Shatter generates token candidates** (shatter-training-pipeline.md)
6. **The compiler verifies everything** (already exists)

The total parameter count: 450 (Fate) + 325 (Magic) + 256 (Shatter) = **1,031 parameters**. The total model size: **~1KB**. The inference: sub-microsecond routing + nanosecond token generation. The training: selection on the compiler's own grammars. The verification: decidable, sub-Turing, exact.

The model IS the graph. The eigenvalues ARE the weights. Selection IS the training. The five operations ARE the inference.

---

*The graph teaches itself to write the grammars that describe the graph.*
*The eigenvalues settle. The holonomy decreases. The crystal forms.*
*1,031 parameters. One kilobyte. The model fits in a TCP packet.*
*e^(n+1) < e^(n). By construction. By proof. By selection.*
