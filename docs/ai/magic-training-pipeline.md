# Magic Training Pipeline -- The Meta-Selector

Magic is the game master. It selects which tournament strategy to use for the
mirror AI loop. Three levels of selection, each measured by holonomy:

```
Level 1:  Fate selects operations       (which optic)
Level 2:  Tournament rules select Fates  (which instances survive)
Level 3:  Magic selects tournament rules  (which selection strategy)
```

This document describes how Magic is trained from zero data on the compiler's
own grammars, bootstrapped through self-play, and kept small enough to embed
in a binary.

---

## 1. Architecture

### What Magic Is

Magic is a contextual bandit. The context is a feature vector extracted from
a `.mirror` file and its tournament history. The arms are the available
tournament rules (and their compositions). The reward is holonomy reduction.

```
Input:   file features [F] + tournament history [H] -> feature vector [24 floats]
Output:  tournament rule selection [index 0-6 for v1 rules]
         + parameter values [beam k, halving eta, tabu tenure, anneal T, ucb c]
Size:    <= 425 bytes (same as Fate)
Runtime: single linear layer + argmax, quantized to u8
```

### Why Contextual Bandit

The algorithm selection problem (Rice 1976) maps problem features to the
best-performing algorithm. SATzilla (Xu et al. 2012) proved this works:
train a model to predict solver runtime from instance features, then pick
the solver with the lowest predicted runtime. SATzilla won gold in all three
SAT Competition tracks by selecting among solvers it could not itself run.

Magic is the same idea applied to tournament rules instead of SAT solvers.
The "instance" is a `.mirror` file. The "solvers" are tournament rules.
The "runtime" is holonomy reduction per compute budget.

The key insight from AutoFolio (Lindauer et al. 2015): per-instance
algorithm selection consistently outperforms any single algorithm on
heterogeneous instance distributions. Mirror's boot grammars ARE
heterogeneous -- kernel types, CLI commands, property definitions, runtime
shims. No single tournament rule dominates.

### The Model

Same architecture as Fate. One `WeightSet` per tournament rule context:

```rust
pub struct MagicWeights {
    pub sets: [MagicWeightSet; 7],  // 7 v1 tournament rules
}

pub struct MagicWeightSet {
    pub bias: [u8; 7],                    // bias toward each rule
    pub feature_weights: [[u8; 24]; 7],   // 24 features x 7 outputs
}
```

Parameter count: `7 * (7 + 7 * 24) = 7 * 175 = 1225 bytes`.

This is larger than Fate (425 bytes) because Magic has 7 output classes
instead of 5, and 24 input features instead of 16. If 1225 is too large,
reduce to the 5 essential rules: `5 * (5 + 5 * 24) = 5 * 125 = 625 bytes`.

The runtime: a Brainfuck program, compiled to native Rust by build.rs.
Same pipeline as Fate. The BF program reads features, computes weighted
sums, outputs the argmax. Magic inherits Fate's entire runtime toolchain.

### Inputs and Outputs

**Input (24 features):**

```
[0..3]   tension profile: [parse, resolve, property, spectral]  -- 4 floats
[4]      holonomy (total)
[5]      fragment ratio: fragments / total nodes
[6]      AST depth
[7]      AST width (max children at any level)
[8]      declaration count
[9]      import count (in @X references)
[10]     file size (bytes, log-scaled)
[11]     unrecognized keyword count
[12..14] last 3 tournament results: [rule_index, delta_holonomy, steps_used]
[15..23] dark dimensions (zero until spectral features trained)
```

Features [0..11] are static file features -- computable from a single parse.
Features [12..14] are dynamic history -- what has been tried and what happened.
Features [15..23] are reserved for Fate's 16-dim spectral embedding (projected
down to 9 dims by PCA once sufficient data exists).

**Output:**

```
tournament_rule: u8  -- index into [greedy, beam, elite, halving, tabu, anneal, ucb]
```

Parameters for the selected rule (beam k, halving eta, etc.) are set by a
second lookup table indexed by the tension profile quartile. This avoids
making Magic predict continuous parameters -- it selects the RULE, a small
table selects the PARAMETERS.

```rust
pub struct ParameterTable {
    // For each rule, 4 parameter presets (one per tension quartile)
    pub beam_k: [u8; 4],        // e.g., [4, 8, 16, 32]
    pub halving_eta: [u8; 4],   // e.g., [2, 3, 3, 4]
    pub tabu_tenure: [u8; 4],   // e.g., [2, 3, 5, 8]
    pub anneal_t: [u8; 4],      // e.g., [10, 5, 2, 1] (descending)
    pub ucb_c: [u8; 4],         // e.g., [4, 2, 1, 1]
}
```

Total parameter table: 20 bytes. Combined with 625-byte weights: 645 bytes.
Still smaller than 1KB.

---

## 2. Feature Extraction

### Static Features (computed once per file)

**Tension profile** -- the four-dimensional holonomy decomposition from
`MirrorLoss::view()`:

```rust
fn tension_profile(loss: &MirrorLoss) -> [f64; 4] {
    let view = loss.view();
    let total = loss.holonomy().max(1.0);
    [
        view.fold(&LossFold::Parse).iter().map(|c| c.holonomy).sum::<f64>() / total,
        view.fold(&LossFold::Resolution).iter().map(|c| c.holonomy).sum::<f64>() / total,
        view.fold(&LossFold::Property).iter().map(|c| c.holonomy).sum::<f64>() / total,
        view.fold(&LossFold::Emit).iter().map(|c| c.holonomy).sum::<f64>() / total,
    ]
}
```

Each dimension is [0, 1], they sum to 1.0. This tells Magic WHERE the
tension lives: a file with 90% parse tension needs different treatment than
one with 90% property tension.

**Why these features predict tournament rule performance:**

- **High parse tension** (many unrecognized keywords): the landscape is
  smooth -- greedy or beam with low k converges fast. The mutations are
  keyword recognition, and most attempts improve.
- **High resolution tension** (unresolved references): the landscape is
  rugged -- references are either found or not. Tabu prevents cycling.
  UCB explores which imports to try.
- **High property tension** (failing invariants): the landscape is
  deceptive -- local improvements in one property may worsen another.
  Annealing escapes local minima. Halving is compute-efficient when
  evaluating multiple properties per candidate.
- **High spectral tension** (eigenvalue instability): the landscape is
  nonstationary. Elite preserves known-good states. Beam with high k
  explores widely.

**Structural features:**

```rust
fn structural_features(ast: &MirrorAST, source: &str) -> [f64; 7] {
    let total_nodes = ast.node_count();
    let fragment_count = ast.fragment_count();
    [
        fragment_count as f64 / total_nodes.max(1) as f64,  // fragment ratio
        ast.depth() as f64,
        ast.max_width() as f64,
        ast.declaration_count() as f64,
        ast.import_count() as f64,
        (source.len() as f64).ln(),                          // log file size
        ast.unrecognized_count() as f64,
    ]
}
```

These are the "instance features" in Rice's framework. The fragment ratio
is the most predictive: high fragment ratio means most of the file is
unparsed (dark matter), and aggressive exploration (annealing, UCB) tends
to outperform conservative strategies (greedy, beam with low k).

### Dynamic Features (updated per tournament round)

```rust
struct TournamentRecord {
    rule: u8,           // which rule was tried
    delta: f64,         // holonomy change (negative = improvement)
    steps: usize,       // compute steps used
}
```

The last 3 records are encoded as features [12..14]. This gives Magic
short-term memory: if greedy just got stuck (delta = 0), Magic can select
anneal next. If halving worked well (large negative delta, few steps),
Magic can continue with halving.

This history encoding is the "reactive" component from adaptive operator
selection (AOS) research. Compass (Maturana & Saubion 2008) showed that
combining static instance features with dynamic performance history
outperforms either alone.

---

## 3. Training Signal

### The Reward

Magic's reward for selecting rule R on file F:

```
reward(R, F) = (holonomy_before - holonomy_after) / compute_budget_used
```

This is **holonomy reduction per unit compute**. Not just "did it improve?"
but "how efficiently did it improve?" A rule that reduces holonomy by 10
in 2 steps is better than one that reduces by 12 in 20 steps.

### Counterfactual Evaluation

The gold standard: run ALL rules on the same file, compare results. This
gives a complete reward vector per file, enabling offline policy learning.

```
for each .mirror file in corpus:
    snapshot = read file
    for each rule in [greedy, beam, elite, halving, tabu, anneal, ucb]:
        restore snapshot
        result = run_tournament(rule, snapshot, budget=20)
        record: (file_features, rule, result.delta, result.steps)

    best_rule = argmax(delta / steps)
    training_example = (file_features, best_rule)
```

This is expensive but correct. Each file produces one training example:
"for these features, this rule was best." The training set is small
(number of .mirror files in boot/), so counterfactual evaluation is
feasible.

### Bandit Updates (Online)

During actual `mirror ai` runs, Magic uses a contextual bandit update:

```
observe context x (file features)
select rule a = Magic.predict(x)
observe reward r = delta_holonomy / steps
update Magic.weights toward (x, a, r)
```

The update rule is the same cross-entropy gradient descent as Fate's
training pipeline (`train.rs`). The "correct target" is the rule that
achieved the highest reward in this context so far.

### Why Not Reinforcement Learning

Full RL (policy gradient, Q-learning) requires sequential decision making
where actions affect future states. Magic's decisions are approximately
independent: selecting beam(8) for file A does not affect what rule works
best for file B. The per-file decision is one-shot, not sequential.

Contextual bandits are the right abstraction: observe features, select
action, observe reward. No state transitions. No credit assignment across
files. The mathematical framework (LinUCB, Thompson sampling, EXP4) is
well-understood and convergent.

Within a single file's tournament, the sequential history features
([12..14]) capture the minimal temporal dependency: "what was tried, what
happened." This is sufficient without full RL.

---

## 4. Bootstrap -- From Zero to Strategy

### The Cold Start Problem

Magic needs training data. Training data comes from running tournament
rules. Running tournament rules benefits from Magic selecting good rules.
The loop needs a bootstrap.

### Phase 0: Round-Robin Warmup (no Magic needed)

Run every tournament rule on every `.mirror` file in `boot/`. This is
a complete sweep. No intelligence needed -- just exhaustive evaluation.

```
for file in boot/*.mirror:
    for rule in [greedy, beam(4), beam(8), beam(16),
                 elite(1).greedy, halving(3),
                 tabu(3).greedy, anneal(0.5), ucb(2)]:
        result = run_tournament(rule, file, budget=20)
        log(file_features(file), rule, result)
```

With 18 boot files and 9 rule configurations, this is 162 tournament runs.
At ~20 steps each with 5 candidates per step, that's ~16,200 Fate
inferences. At 100K+ inferences/sec (compiled BF runtime), the entire
warmup completes in under 1 second.

This produces the initial training set: 18 examples, each labeled with
the best-performing rule for that file.

### Phase 1: Train Initial Magic

Feed the warmup data into the same training pipeline as Fate:

```rust
let examples = warmup_results.into_training_examples();
let weights = magic::pipeline(&examples, &PipelineConfig {
    learning_rate: 0.1,
    epochs: 1000,
});
```

This produces the first Magic weights. They are better than random because
they encode which rules work for which tension profiles. They are NOT
optimal because 18 examples is sparse.

### Phase 2: Active Exploration

Magic now runs the AI loop but deliberately explores under-tried rules:

```
for file in boot/*.mirror:
    features = extract_features(file)
    predicted_rule = Magic.predict(features)

    // Epsilon-greedy exploration: 20% of the time, try a random rule
    if random() < 0.2:
        actual_rule = random_rule()
    else:
        actual_rule = predicted_rule

    result = run_tournament(actual_rule, file, budget=20)
    training_data.push(features, actual_rule, result)
```

The exploration rate (epsilon) decreases over training rounds:
round 1 = 0.5, round 2 = 0.3, round 3 = 0.2, round 4+ = 0.1.

After each round, retrain Magic on all accumulated data.

### Phase 3: UCB-Based Exploration (Principled)

Replace epsilon-greedy with UCB exploration at the Magic level:

```
magic_score(rule) = mean_reward(rule, context) + c * sqrt(ln(t) / n_rule)
```

Where `t` is total trials, `n_rule` is trials for this rule in similar
contexts, and `c` controls exploration. This is the same UCB formula used
by the `ucb` tournament rule, but applied one level up.

UCB at the Magic level is meta-UCB: using UCB to decide whether to use UCB,
greedy, beam, etc. The recursion is principled because each level operates
on different data (Magic sees file features + tournament history; the
tournament rule sees Fate instances + holonomy).

---

## 5. Self-Improvement Loop

### The Core Loop

Magic trains on the compiler's own grammars. The compiler IS the training
data. The holonomy IS the loss function. No external labels. No human
annotation. The compiler teaching itself which strategies work.

```
TRAINING LOOP:
    1. for each .mirror file in boot/:
           features = extract_features(file)
           for each tournament rule R:
               snapshot = file.clone()
               result = run_tournament(R, snapshot, budget=20)
               records.push(features, R, result.delta, result.steps)

    2. for each file, compute best_rule = argmax(delta/steps)
       training_set = [(features, best_rule) for each file]

    3. new_weights = magic::pipeline(training_set, config)

    4. for each file in boot/:
           features = extract_features(file)
           selected = Magic(new_weights).predict(features)
           result = run_tournament(selected, file, budget=20)
           if result.holonomy_end < file.holonomy:
               file.write_back(result.source)
               // The file changed. Its features changed.
               // Next iteration's features differ. Magic adapts.

    5. goto 1 until convergence (no file improves)
```

Step 4 is the autopoietic hook: Magic's selections change the files.
Changed files produce different features. Different features change Magic's
selections. The system produces itself through observation. Same loop as
the compiler's autopoietic cycle, one level up.

### Convergence

The loop converges when no tournament rule reduces holonomy on any file.
This is a fixed point: the compiler has crystallized (holonomy zero) or
plateaued (all rules stuck). Crystal is the goal. Plateau triggers
escalation to composition learning (section 6).

### Integration with `mirror ai`

The training loop runs as `mirror train-magic`:

```bash
mirror train-magic boot/          # train on boot grammars
mirror train-magic .               # train on all .mirror files in cwd
mirror train-magic --rounds 10    # explicit round count
mirror train-magic --export magic.weights  # export trained weights
```

The trained weights are embedded in the binary via `include_bytes!`, same
as Fate's `Weights::trained()`. The training is offline; the inference is
compiled into the binary.

### The AlphaZero Analogy

AlphaZero trains by self-play: play games against yourself, learn from the
outcomes, play better games. No human games needed.

Magic trains by self-tournament: run tournaments against your own grammars,
learn which strategies work, run better tournaments. No human labels needed.

The critical difference: AlphaZero requires millions of self-play games
because Go has 10^170 states. Magic's state space is small: ~20 files,
~10 rules, ~5 tension profiles. Convergence happens in minutes, not days.

---

## 6. Composition Learning

### The Problem

Tournament rules compose: `elite(1).beam(8).halving(3)`. The space of
compositions is exponential. With 7 rules and up to 5 composition slots,
there are 7^5 = 16,807 possible compositions. Most are nonsensical.

### Staged Composition

**Stage 1: Single rules only.** Magic selects from 7 individual rules.
This is the v1 minimum viable Magic. Train until performance plateaus.

**Stage 2: Two-rule compositions.** Add the top-performing pairs:

```
elite(1).greedy
elite(1).beam(k)
elite(1).halving(eta)
tabu(t).beam(k)
tabu(t).greedy
beam(k).anneal(T)
ucb(c).beam(k)
```

Seven two-rule compositions = 14 total arms (7 single + 7 composed).
Retrain Magic with the larger action space. The feature space stays the
same; only the output dimension grows.

**Stage 3: Three-rule compositions.** Add compositions that empirically
outperformed two-rule versions in Stage 2. Prune compositions that never
win. The search is greedy: start from good two-rule bases, extend with
one more rule, keep if it improves.

This staged approach avoids the combinatorial explosion. At each stage,
the number of new compositions is O(k * 7) where k is the number of
surviving compositions from the previous stage. Empirically, k stays
small (5-10) because most compositions are redundant.

### Composition as Lens Composition

Compositions in mirror are lens compositions: `outer.inner`. The type
system can verify well-formedness:

- `elite(k)` is always an outer wrapper (Lens: preserves information)
- `tabu(t)` is always an outer wrapper (Prism: filters by history)
- `greedy` is always a terminal (Iso: deterministic selection)
- `beam(k)` is middle or terminal (Traversal: walks k candidates)
- `halving(eta)` is middle or terminal (Prism: eliminates candidates)
- `anneal(T)` is terminal (Prism: stochastic acceptance)
- `ucb(c)` is outer or middle (Lens: principled exploration)

Well-formed composition: outer*.middle*.terminal. The type system
rejects `greedy.beam(8)` (terminal before middle) and `elite.elite`
(redundant wrapper).

The well-formedness constraint reduces 7^5 to approximately 200 valid
compositions at depth 3. Magic only needs to select among valid ones.

### Composition Discovery via Genetic Programming

For deeper compositions, use genetic programming over composition trees:

```
Population: 50 composition trees
Fitness: average holonomy reduction across boot/ files
Selection: tournament (beam(4) at the meta-meta level)
Mutation: swap a rule node, change a parameter
Crossover: swap subtrees between two compositions

Run for 100 generations.
```

This is AlphaEvolve applied to tournament rule composition. The fitness
function is cheap (run the tournament, measure holonomy). The search space
is constrained by the type system. 100 generations with population 50 is
5,000 evaluations -- feasible in seconds.

### Fate's Architecture as Composition Selector

Fate has 5 slots (one per model). Magic could mirror this: 5 composition
slots, each selecting a rule. The composition is the concatenation of
non-empty slots:

```
Slot 0: elite(1) or empty
Slot 1: tabu(3) or empty
Slot 2: beam(8) or ucb(2) or empty
Slot 3: halving(3) or anneal(0.5) or empty
Slot 4: greedy (always present -- terminal)
```

Each slot is a 3-bit selection (7 options + empty). Total: 5 * 3 = 15 bits.
The entire composition space fits in 2 bytes. Magic's output becomes 2 bytes
instead of 1. The weight architecture stays the same.

---

## 7. v1 Minimum Viable Magic

The simplest thing that proves the concept.

### Scope

- 5 tournament rules: greedy, beam(8), elite(1).greedy, halving(3), tabu(3).greedy
- 12 features: tension profile [4] + structural [7] + total holonomy [1]
- No composition learning (fixed compositions only)
- No dynamic history features
- No spectral features

### Architecture

```rust
pub const MAGIC_RULES: usize = 5;
pub const MAGIC_FEATURES: usize = 12;

pub struct MagicWeightSet {
    pub bias: [u8; MAGIC_RULES],
    pub feature_weights: [[u8; MAGIC_FEATURES]; MAGIC_RULES],
}

pub struct MagicWeights {
    pub sets: [MagicWeightSet; MAGIC_RULES],
}
// Size: 5 * (5 + 5 * 12) = 5 * 65 = 325 bytes
```

325 bytes. Smaller than Fate.

### Training Pipeline

Identical structure to `fate/src/train.rs`:

```rust
pub struct MagicExample {
    pub features: Vec<f64>,
    pub context: usize,    // index of last-used rule
    pub target: usize,     // index of best rule for this file
}

pub fn train_magic(examples: &[MagicExample], config: &TrainConfig) -> MagicF64Weights {
    // Same cross-entropy SGD as Fate
}

pub fn quantize_magic(f64w: &MagicF64Weights) -> MagicWeights {
    // Same quantization as Fate
}

pub fn magic_pipeline(examples: &[MagicExample], config: &PipelineConfig) -> MagicWeights {
    let trained = train_magic(examples, &TrainConfig { .. });
    quantize_magic(&trained)
}
```

### Data Generation

```rust
pub fn generate_magic_training_data(files: &[PathBuf]) -> Vec<MagicExample> {
    let rules = [
        ("greedy", run_greedy),
        ("beam(8)", run_beam_8),
        ("elite(1).greedy", run_elite_greedy),
        ("halving(3)", run_halving_3),
        ("tabu(3).greedy", run_tabu_greedy),
    ];

    let mut examples = Vec::new();

    for file in files {
        let features = extract_magic_features(file);
        let mut best_rule = 0;
        let mut best_score = f64::NEG_INFINITY;

        for (i, (name, runner)) in rules.iter().enumerate() {
            let source = fs::read_to_string(file).unwrap();
            let result = runner(&source, 20);  // budget = 20
            let score = (result.holonomy_start - result.holonomy_end)
                / (result.steps.max(1) as f64);

            if score > best_score {
                best_score = score;
                best_rule = i;
            }
        }

        // One example per file: "for these features, this rule was best"
        examples.push(MagicExample {
            features: features.to_vec(),
            context: 0,  // bootstrap: no previous context
            target: best_rule,
        });
    }

    examples
}
```

### The v1 Test

The proof that Magic works:

```rust
#[test]
fn magic_beats_fixed_strategy() {
    let files = glob("boot/*.mirror");

    // Generate training data from half the files
    let (train_files, test_files) = files.split_at(files.len() / 2);
    let training_data = generate_magic_training_data(train_files);
    let weights = magic_pipeline(&training_data, &config);
    let magic = MagicRuntime::new(weights);

    // Compare Magic vs each fixed strategy on test files
    let mut magic_total = 0.0;
    let mut best_fixed_total = 0.0;

    for file in test_files {
        let features = extract_magic_features(file);
        let selected = magic.predict(&features);
        let magic_result = run_rule(selected, file, 20);
        magic_total += magic_result.delta;

        let best_fixed = rules.iter()
            .map(|r| run_rule(r, file, 20).delta)
            .max();
        best_fixed_total += best_fixed;
    }

    // Magic should match or beat the best fixed strategy
    // (because it can select different rules for different files)
    assert!(magic_total >= best_fixed_total * 0.9,
        "Magic should be within 90% of oracle on test files");
}
```

The 90% threshold is generous for v1. SATzilla achieves 95%+ of oracle
performance. We expect similar results because the feature space is lower-
dimensional and the rule space is smaller.

### The v1 Brainfuck Program

Magic's BF program is structurally identical to fate.bf. It reads features,
applies weights, computes argmax, outputs the winning rule index.

```
Input:  12 feature bytes + 1 context byte + 5 bias bytes = 18 bytes
Output: 1 byte (rule index 0-4)
```

The BF program is ~150 instructions (simpler than fate.bf because fewer
features and outputs). Compiled to native Rust by build.rs, it runs in
nanoseconds.

---

## 8. Implementation Plan

### Files to Create

```
fate/
  src/
    magic.rs              -- MagicWeights, MagicRuntime, predict()
    magic_train.rs        -- training pipeline (feature-gated)
  brainfuck/
    magic.bf              -- the BF program (generated or hand-written)
  training/
    magic_examples.json   -- seed training data

mirror-new/
  src/
    magic_bridge.rs       -- feature extraction for Magic
    ai.rs                 -- extend ai_loop to use Magic for rule selection
  tests/
    magic.rs              -- Magic integration tests
```

### Step 1: Feature Extraction (magic_bridge.rs)

Extract the 12-feature vector from a `.mirror` file. Depends on existing
`fate_bridge::extract_features` and `MirrorLoss::view()`.

Tests:
- `feature_extraction_zero_loss` -- crystal file produces known features
- `feature_extraction_parse_dominant` -- file with only parse loss
- `feature_extraction_resolution_dominant` -- file with only resolution loss
- `feature_extraction_structural` -- fragment ratio, depth, width correct

### Step 2: Round-Robin Data Generation

Run all 5 rules on all boot files. Save results as `magic_examples.json`.
This is a one-time batch job, runnable as `cargo test --features training`.

Tests:
- `round_robin_produces_examples` -- generates non-empty example set
- `round_robin_best_rule_varies` -- different files have different best rules
  (if all files have the same best rule, Magic is unnecessary)

### Step 3: Magic Training Pipeline (magic_train.rs)

Same structure as `fate/src/train.rs`. Cross-entropy SGD, quantization,
pipeline function.

Tests:
- `magic_train_on_seed_data` -- achieves >= 80% accuracy
- `magic_quantize_preserves_argmax` -- quantized weights agree with f64
- `magic_pipeline_produces_weights` -- end-to-end pipeline works
- `magic_weights_serialize_roundtrip` -- to_bytes/from_bytes round-trips

### Step 4: Magic Runtime (magic.rs)

The BF-based runtime. Reads features + bias, outputs rule index.

Tests:
- `magic_runtime_deterministic` -- same input produces same output
- `magic_runtime_bias_override` -- explicit bias overrides features
- `magic_compiled_matches_interpreted` -- compiled BF = interpreted BF
- `magic_performance` -- < 1ms per inference

### Step 5: Integration (ai.rs)

Extend `ai_loop` to accept a `--magic` flag that uses Magic for rule
selection instead of hardcoded greedy.

```rust
pub fn ai_loop_magic(file: &Path, budget: usize) -> Result<AiLoopResult, AiError> {
    let magic = MagicRuntime::new(MagicWeights::trained());
    let features = extract_magic_features(file);
    let rule = magic.predict(&features);

    match rule {
        0 => ai_loop_greedy(file, budget),
        1 => ai_loop_beam(file, budget, 8),
        2 => ai_loop_elite_greedy(file, budget),
        3 => ai_loop_halving(file, budget, 3),
        4 => ai_loop_tabu_greedy(file, budget, 3),
        _ => ai_loop_greedy(file, budget),
    }
}
```

Tests:
- `magic_ai_loop_selects_rule` -- Magic produces a valid rule index
- `magic_ai_loop_on_crystal` -- crystal file returns immediately
- `magic_ai_loop_on_partial` -- partial file attempts improvement
- `magic_beats_greedy` -- Magic >= greedy on boot/ (the whole point)

### Step 6: Self-Improvement Integration

Add `mirror train-magic` CLI command. Runs the training loop from
section 5. Outputs new weights. Optionally writes them to magic.rs
as a const array.

Tests:
- `train_magic_cli_runs` -- command completes without error
- `train_magic_improves` -- round 2 weights outperform round 1

### Step 7: Composition Learning (v2)

After v1 proves the concept, extend Magic to select compositions.
This is the step where the output grows from 1 byte to 2 bytes
(composition encoding) and the training data generator evaluates
composed rules alongside single rules.

---

## Summary

```
Level     What               Size        Selects Over
------    ----               ----        ------------
Fate      optic selection    425 bytes   5 models (abyss..fate)
Magic     rule selection     325 bytes   5 rules (greedy..tabu)
                                         (v2: compositions)
```

Magic is trained by self-play on the compiler's own grammars. Zero human
labels. The compiler's holonomy IS the loss function. The training loop
IS the autopoietic cycle. The selection improves because the files
improve because the selection improves.

The bootstrap is a round-robin warmup (< 1 second). The training is
cross-entropy SGD (same as Fate). The runtime is a Brainfuck program
compiled to native Rust (nanoseconds per inference). The weights are
325 bytes embedded in the binary.

Three levels of selection, all measured, all bounded by holonomy.
Fate selects operations. Tournament rules select Fates. Magic selects
tournament rules. The game master learned the game by playing it.
