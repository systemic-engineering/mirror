# Shatter Training Pipeline — Architecture Document

Shatter takes arbitrary input and scrambles it into mirror syntax tokens.
Not translation. Not understanding. Shattering. The loop:

```
Input (anything) -> Shatter (scramble into mirror tokens) -> Mutate (apply to target)
  -> Fate (measure holonomy) -> Tournament (keep improvements) -> Repeat
```

This document synthesizes research across evolutionary computation, program
synthesis, mutation testing, adversarial generation, small model design,
biological encoding, self-improving systems, and minimal computation into a
concrete training pipeline.

---

## 1. Token Scrambling / Recombination Techniques

### What we found

**Grammatical Evolution (GE)** is the closest match to what Shatter does.
GE uses a BNF grammar to constrain the mapping from integer sequences
(genotypes) to syntactically valid programs (phenotypes). An integer sequence
is read codon-by-codon; each codon selects a production rule from the grammar.
Wrapping handles sequences longer than the grammar needs. PonyGE2 is the
reference implementation.

**Grammar-guided fuzzing** (AFL++ Grammar-Mutator, Grammarinator) generates
structured inputs from ANTLR grammars. The fuzzer produces valid syntax by
walking the grammar and making random choices at each production rule. This is
exactly what Shatter's "scramble into mirror tokens" does, except Shatter
doesn't care about validity — it cares about holonomy reduction.

**Byte-Pair Encoding (BPE)** in reverse: BPE builds a vocabulary by merging
frequent byte pairs. The reverse — decomposing text into subword units — gives
us a way to take arbitrary input and break it into pieces that can be
recombined. BPE gives us the DECOMPOSITION step; the grammar gives us the
RECOMBINATION step.

**Markov chain n-gram models** over token sequences are the simplest
possible "learned scrambler." Given a corpus of .mirror files, an n-gram
model learns P(next_token | last_n_tokens). Generation is: sample from
the distribution. This is the baseline Shatter — zero-parameter, pure
frequency-based token recombination.

**Music recombination via genetic algorithms** (Majumder 2019, Tokui &
Iba 2000): MIDI files are decomposed into motifs (short sequences), then
recombined by crossover operators. Fitness is harmonic compatibility. The
direct analogy: .mirror files are decomposed into AST subtrees, recombined
by crossover, fitness is holonomy.

### How it maps

Mirror grammar IS a BNF grammar. The vocabulary is fixed:

```
Keywords:  type, grammar, action, property, in, out, focus, project,
           split, shift, settle, template, abstract, recover, rescue,
           io, fold, requires, invariant, ensures
Structural: { } ( ) | = < . @ , :
Literals:  identifiers, grammar refs (@X), strings
```

The mapping from arbitrary bytes to mirror tokens is a codon table:
every N bytes map to one production rule choice. The grammar constrains
what's syntactically possible. The holonomy fitness function decides what
survives.

### Implementation approach

1. Define the mirror grammar as BNF (we already have the parser — extract
   the grammar rules from `parse.rs`)
2. Build a codon table: map byte pairs (0-65535) to grammar production choices
3. To shatter input: read it as a byte stream, consume byte pairs, map each
   to a production rule, emit the corresponding mirror token
4. The codon table IS the model. Training = adjusting which byte pairs map
   to which production rules

### What to try first

**N-gram baseline:** Build a trigram model over token sequences from existing
.mirror files. Generate 1000 random .mirror snippets. Feed each through the
compiler. Measure holonomy distribution. This tells us the base rate — how
often does random-but-frequency-weighted token generation produce low-holonomy
output? This is the number to beat.

---

## 2. Mutation Testing (Inverted)

### What we found

**Mutation operators** from Pitest, Stryker, and cargo-mutants:
- Arithmetic: replace `+` with `-`, `*` with `/`
- Conditional: negate conditions, replace `<` with `<=`
- Statement deletion: remove a statement entirely
- Return value mutation: change return values
- Constant replacement: change literals
- Method call deletion: remove a call

**Higher-order mutation** combines multiple first-order mutations. A
higher-order mutant might swap an operator AND delete a statement. Most
higher-order mutants are equivalent to a first-order mutant, but some
create genuinely new behavior.

**The equivalent mutant problem:** Some mutations produce identical behavior.
Detection is undecidable in general but tractable for restricted languages.
Mirror grammar is sub-Turing — we can detect equivalent mutations by
comparing OIDs (same OID = equivalent).

**Selective mutation:** Not all mutation operators are equally informative.
Research shows ~5 operators cover 95% of mutation detection ability.

### How it maps

Traditional mutation testing introduces bugs to test tests. We INVERT it:
introduce mutations to IMPROVE code. The survivor is the mutation that
REDUCES holonomy. Same operators, opposite selection pressure.

Mirror-specific mutation operators:
- **Type variant swap:** `type color = red | blue` -> `type color = blue | red`
- **Grammar ref redirect:** `in @mirror` -> `in @property`
- **Action body shuffle:** reorder statements within an action
- **Property strengthen:** `partial(0.97)` target -> `pass` target
- **Import addition:** add `in @X` for an unresolved reference
- **Fragment crystallization:** replace `Fragment(raw)` with parsed equivalent
- **Declaration deletion:** remove a type/action/property
- **Subtree swap:** exchange two AST subtrees

The key insight: OID comparison makes equivalent mutant detection FREE.
`mutant.oid == original.oid` means the mutation was equivalent. Skip it.
No computation wasted.

### Implementation approach

1. Define mutation operators on `MirrorAST` nodes
2. Each operator takes a node and returns a mutated node
3. Mutation is random but TYPE-AWARE: only apply operators to nodes of the
   right type (don't try to swap variants on an action node)
4. Measure holonomy before and after. Keep if holonomy decreased.
5. Higher-order: compose 2-3 operators per mutation

### What to try first

**Single-operator sweep:** Implement the 8 operators above. For each
.mirror file in the test corpus, apply every operator to every applicable
node. Measure holonomy change. Build a histogram: which operators most
often reduce holonomy? This tells us which operators to prioritize.

---

## 3. Evolutionary Program Synthesis

### What we found

**PushGP** uses a stack-based language (Push) designed for genetic
programming. Key features: multiple typed stacks, implicit type handling,
programs are flat lists of instructions. Crossover = list splice. Mutation
= random instruction insertion/deletion/replacement. The flat representation
avoids the "bloat" problem of tree GP.

**Grammatical Evolution (GE)** maps integer codons to BNF production rules.
The genotype is a flat integer array. The phenotype is a program. Crossover
operates on the integer array (genotype), not the program (phenotype). This
separation of genotype/phenotype is critical — it means mutation operators
don't need to understand the target language.

**Cartesian Genetic Programming (CGP)** represents programs as directed
acyclic graphs encoded as integer lists. Each node has a function, inputs
(indices of previous nodes), and outputs. Mutation = change a function or
reconnect an input. CGP uses mutation-only (no crossover) and typically
evolves with (1+4) strategy (1 parent, 4 children, keep best).

**Gene Expression Programming (GEP)** (Ferreira 2001) separates the genome
(a fixed-length string) from the expression tree it encodes. Each gene has
a head (functions and terminals) and a tail (terminals only). This guarantees
every random genome encodes a valid expression tree. No repair operators
needed.

**AlphaEvolve/OpenEvolve** (2025): LLM-based evolutionary coding agent
using MAP-Elites. Maintains a population database organized by behavioral
features. LLMs generate code mutations (as diffs). Cascaded evaluation
filters candidates cheaply before expensive evaluation. Island model for
diversity.

### How it maps

The MirrorAST IS the phenotype. We need a genotype representation that:
1. Is flat (for easy crossover/mutation)
2. Maps deterministically to valid-ish MirrorAST
3. Supports partial validity (Fragment nodes absorb unparseable regions)

**GE is the natural fit.** The mirror grammar is our BNF. The genotype is
a byte array. Each byte selects a production rule. The mapping produces
MirrorAST. Invalid regions become Fragment nodes (loss = 1.0). The compiler
handles the rest.

The CGP insight applies differently: CGP's mutation-only (1+4) strategy
maps directly to our tournament. Spawn 4 mutations of the best parent.
Keep the best child. No crossover needed for v1.

### Implementation approach

1. Extract mirror grammar rules as a numbered list of productions
2. Build the GE mapper: `[u8] -> MirrorAST`
3. Population: N byte arrays (genotypes)
4. Each generation:
   a. Map genotypes to phenotypes (MirrorAST)
   b. Compile each phenotype
   c. Measure holonomy
   d. Tournament selection: keep lowest holonomy
   e. Mutate winners: flip random bytes in the genotype
5. Crossover (optional): single-point crossover on byte arrays

### What to try first

**CGP-style (1+4):** Start with a random byte array. Generate 4 mutations
(flip 1-3 random bytes each). Map all 5 to MirrorAST via GE. Compile all 5.
Keep the one with lowest holonomy. Repeat. Track holonomy over generations.
Does it decrease? How many generations to reach holonomy < 1.0?

---

## 4. Adversarial / Generative Approaches

### What we found

**Discrete diffusion models** (DiffuSeq, FS-DFM): Apply diffusion
(noise-then-denoise) to discrete token sequences. Start with pure noise
tokens. Iteratively denoise toward valid sequences. The denoising model
learns the target distribution. Recent work (2025-2026) shows these can
match autoregressive models for text.

**CodeRL** (Salesforce, NeurIPS 2022): Treats a pretrained language model
as a stochastic policy. Token predictions are actions. Unit test results
are rewards. REINFORCE algorithm updates the policy to generate code that
passes tests. Key insight: the reward signal is BINARY (pass/fail) but
applied at the TOKEN level via credit assignment.

**Program GANs** (Trabucco et al.): Generator produces program trees.
Discriminator checks syntactic validity and behavioral correctness. The
generator learns to produce syntactically valid programs. Key limitation:
mode collapse — the generator finds one working program and stops exploring.

**VAEs with discrete latent spaces** (Discrete VAE, VQ-VAE): Encode
discrete sequences into a learned latent space. Decode back. The latent
space supports interpolation — "between" two programs is a meaningful
point. VQ-VAE uses a codebook (finite set of latent vectors), which maps
naturally to our vocabulary.

### How it maps

We already have the architecture:
- **Generator** = Shatter (scramble into mirror tokens)
- **Discriminator** = Mirror compiler (measures holonomy)
- **Reward signal** = Holonomy reduction (continuous, not binary)
- **Policy** = Fate (selects which optic/operation to apply)

The key advantage we have over CodeRL: our reward signal is CONTINUOUS
(holonomy is a real number) and CHEAP (compiler runs at millions of
ops/sec, no test execution needed). CodeRL needs to run tests. We just
measure information loss.

The diffusion model insight: START from noise, iteratively refine. This
is exactly what Shatter does. Start from random token soup. Apply Fate.
Holonomy decreases. Apply again. Eventually: crystal.

The VQ-VAE codebook maps directly to our codon table: a finite set of
"code vectors" (production rules) that discrete inputs index into.

### Implementation approach

For v1, we don't need neural networks. The compiler IS the discriminator.
Fate IS the policy. The loop IS the training. But the research suggests
two specific techniques to incorporate:

1. **Credit assignment:** When a mutation reduces holonomy, WHICH tokens
   contributed? The MirrorLoss breakdown (parse, resolution, properties,
   emit) tells us which phase improved. Map that back to the tokens that
   changed in that phase.

2. **Iterative refinement schedule:** Start with aggressive mutation
   (high temperature, many random tokens). As holonomy decreases, reduce
   mutation rate (lower temperature, smaller changes). This is simulated
   annealing applied to the generation process, not just the selection.

### What to try first

**Random generation with compiler feedback:** Generate 10,000 random
token sequences (uniform over mirror vocabulary). Compile each. Histogram
the holonomy distribution. Then: take the top 1% (lowest holonomy), mutate
each 100 times, compile, keep improvements. How many generations until
any sequence reaches holonomy 0?

---

## 5. Small Model Training Techniques

### What we found

**Knowledge distillation:** Train a small "student" model to mimic a large
"teacher" model. The student learns the teacher's output distribution, not
just the hard labels. TinyBERT achieves 96.8% of BERT-base at 7.5x smaller.

**Extreme quantization:** Binary Neural Networks (1-bit weights, +1/-1
only). BitNet achieves competitive performance with 1-bit weights. The key:
you need more epochs (5-10x) but inference is pure integer arithmetic.
Multiplications become additions/subtractions.

**Lottery Ticket Hypothesis** (Frankle & Carlin 2018): Dense networks
contain sparse subnetworks that can match the full network's accuracy when
trained in isolation. Iterative magnitude pruning finds these "winning
tickets." Networks can be pruned to 10% of original size.

**Extreme Learning Machines (ELM):** Single hidden layer. Input-to-hidden
weights are RANDOM and FIXED. Only hidden-to-output weights are trained.
Training is a single matrix pseudoinverse — no gradient descent, no
backpropagation. Training takes milliseconds. This is structurally identical
to excited Fate: random projection followed by a learned linear map.

**Reservoir Computing / Echo State Networks:** A random recurrent network
(the "reservoir") transforms input into a high-dimensional space. Only the
readout layer is trained. The reservoir provides nonlinear mixing for free.
Works surprisingly well for time series and sequence processing.

**muNAS (Constrained Neural Architecture Search):** Searches for tiny
neural architectures that fit within microcontroller constraints (< 64KB).
Accurately captures resource requirements and finds Pareto-optimal
accuracy/size tradeoffs.

### How it maps

Fate is 425 bytes. Five weight sets of u8 values. 5 biases + 5x16 feature
weights per set. Quantized from f64 during training. This is ALREADY an
extreme learning machine — the architecture is fixed, only weights change.

Shatter needs to be similarly small. The question: can 425 bytes (or fewer)
encode a useful token scrambling function?

**Yes, by the ELM argument.** An ELM with random input weights and a
trained readout can approximate any continuous function given enough hidden
nodes. We don't even need "enough" — we need a function that's BETTER THAN
RANDOM at producing low-holonomy output. The bar is low.

**The reservoir computing connection:** If Shatter reads input as a byte
stream, a small recurrent state (even 16 bytes) provides nonlinear mixing.
The "reservoir" is the interaction between the input bytes and the internal
state. The "readout" maps the state to a production rule choice.

Concrete model sizes:
- **Codon table:** 256 entries x 1 byte each = 256 bytes. Maps each input
  byte to a production rule. Zero parameters to train — optimize by
  evolutionary search over the table.
- **Tiny ELM:** 16-byte state + 256-byte readout = 272 bytes. State is
  updated by XOR/rotate with input bytes. Readout maps state to production.
- **Fate-sized Shatter:** 425 bytes. Same architecture as Fate but mapping
  input features to production rule choices instead of model selections.

### Implementation approach

1. Start with the codon table (256 bytes, zero training needed)
2. Optimize the codon table by evolutionary search:
   a. Population of 100 random codon tables
   b. Each table shatters a corpus of non-mirror input
   c. Fitness = average holonomy of produced .mirror output
   d. Tournament selection + mutation (swap entries)
3. Graduate to tiny ELM if the codon table plateaus
4. The training loop IS the tournament. No separate training phase.

### What to try first

**256-byte codon table evolution:** Generate 100 random codon tables
(each: 256 bytes mapping input bytes to production rule indices). Feed
the same input text through each table. Compile the outputs. Keep the
10 tables with lowest average holonomy. Mutate (swap 5 random entries).
Repeat for 1000 generations. Plot holonomy over time.

---

## 6. The Genotype-Phenotype Mapping

### What we found

**Biological codon table:** 64 codons (triplets of 4 nucleotides) map to
20 amino acids + stop signals. The mapping is REDUNDANT: most amino acids
have 2-6 codons. This redundancy is a feature — synonymous mutations (change
the codon but not the amino acid) provide robustness. The codon table is
UNIVERSAL across almost all life.

**Codon optimization** in synthetic biology: when expressing a gene in a
new organism, you pick the codons that organism prefers (codon usage bias).
Same protein, different DNA. CodonTransformer (2M+ downloads) and
CodonMPNN use deep learning for this. The key: the mapping is many-to-one,
and the choice among synonyms matters for expression efficiency.

**Developmental encoding** in evolutionary computation: instead of mapping
genotype directly to phenotype, the genotype encodes a PROCESS that grows
the phenotype. Indirect encodings like HyperNEAT use a Compositional
Pattern Producing Network (CPPN) to generate neural network weights from
spatial coordinates. Small CPPN = large network with regular structure.

**HyperNEAT:** A CPPN takes (x1, y1, x2, y2) as input and outputs the
weight of the connection from neuron at (x1,y1) to neuron at (x2,y2).
The CPPN is small; the network it generates is large. Patterns like
symmetry, repetition, and variation emerge naturally from the CPPN's
activation functions (sin, gaussian, sigmoid, linear).

**Grammatical encoding** (from GE): the genotype is integers. Each
integer selects a production rule by `codon % num_rules`. Wrapping:
when the codon stream is exhausted, wrap around to the beginning.
This gives every integer sequence a valid phenotype.

### How it maps

The mirror codon table:

```
Input byte(s)  ->  Production rule index  ->  Mirror token
0x00-0xFF         0-N (N = number of rules)   type, grammar, action, ...
```

The biological analogy is precise:
- **Codons** = byte pairs from input
- **Amino acids** = mirror grammar tokens
- **Protein** = MirrorAST
- **Folding** = compilation (parse -> resolve -> emit)
- **Fitness** = holonomy (lower = better folded)
- **Synonymous mutations** = different byte pairs mapping to the same token
  (redundancy = robustness)

The redundancy structure matters. If mirror has ~25 production rules,
then 256/25 ~ 10 different byte values map to each rule. Which bytes map
to which rules determines the "codon usage bias" — the probability that
random input produces each token. We want this bias to MATCH the
frequency distribution of tokens in well-formed .mirror files.

**Developmental encoding for v2:** Instead of byte -> token directly, the
genotype encodes a small program (like a CPPN) that maps bytes to tokens.
The program can learn patterns: "after `type`, the next token is likely an
identifier." This is the n-gram model reified as a developmental process.

### Implementation approach

1. **Measure token frequencies** in existing .mirror files. This gives us
   the target distribution.
2. **Build the codon table** with redundancy proportional to token frequency.
   Common tokens (type, identifier, |, =) get more byte values. Rare tokens
   (abstract, template, rescue) get fewer.
3. **Frequency-biased codon table:** If `type` is 15% of tokens in .mirror
   files, then ~38 of the 256 byte values should map to `type`.
4. **Evolve the table:** The frequency-biased table is the starting point.
   Evolution fine-tunes which specific byte values map to which tokens.

### What to try first

**Token frequency analysis:** Parse all .mirror files in the test corpus.
Count token frequencies. Build a codon table with redundancy proportional
to frequency. Compare: random codon table vs frequency-biased codon table.
Feed the same input through both. Which produces lower average holonomy?
The gap tells us how much the codon table structure matters.

---

## 7. Self-Improving Systems

### What we found

**Godel machines** (Schmidhuber): A self-modifying universal problem solver.
Makes provably optimal self-improvements: only modifies its own code when it
can PROVE the modification will improve its expected future performance.
Theoretical guarantee: if an improvement exists, the Godel machine will
find it. Practical limitation: the proofs are intractable for complex systems.

**PowerPlay** (Schmidhuber 2011): Continually invents new problems AND
solves them. The system searches for (task, solver_modification) pairs
where the modification solves the new task WITHOUT breaking solutions to
previous tasks. Monotonically increasing capability. No forgetting.

**AlphaGo Zero self-play:** Starts with random play. Plays against itself.
Uses the game outcome as the training signal. No human data. Key insight:
the training data is GENERATED BY THE CURRENT MODEL. Each iteration:
play games -> train on game outcomes -> new model plays better games.
Convergence: the model improves monotonically because self-play provides
an ever-improving curriculum.

**Iterated Amplification** (Christiano): Recursively decompose hard
problems into easier subproblems. A weak model solves easy subproblems.
A "meta-model" decomposes hard problems and aggregates sub-solutions.
Training alternates: train the model to match the (expensive) decomposition
process, then use the trained model as the base for the next level.

**ICLR 2026 Workshop on Recursive Self-Improvement** confirms this is now
an active research area: LLM agents rewriting their own codebases,
scientific discovery pipelines with continual fine-tuning.

### How it maps

The autopoietic loop in the shatter-spec IS self-play:

```
compile source.mirror -> output.shatter (carries Fate gen N)
  -> mirror ai --train -> output.shatter (carries Fate gen N+1)
  -> mirror ai --train -> output.shatter (carries Fate gen N+2)
  -> ... -> converged (holonomy ~ 0)
```

The compilation IS the training data. The MirrorLoss IS the gradient.
The property verdicts ARE the reward signal. This is AlphaGo Zero for
code: the model plays against itself (compiles -> evaluates -> improves).

PowerPlay maps to the Shatter loop: each generation invents new
transformations (the "problem") and tests whether they improve holonomy
(the "solution"). Transformations that improve holonomy without breaking
existing crystals survive. Monotonically increasing capability.

**Convergence guarantees:** The loop converges when holonomy reaches zero
(the fixed point). For a given .mirror file, holonomy is bounded below by
zero and each accepted mutation reduces it. Monotone bounded sequences
converge. The guarantee is: if the crystal exists, the loop will find it.
If no crystal exists (the input is fundamentally unrepresentable in mirror
syntax), holonomy asymptotes to some positive value.

### Implementation approach

1. The loop already exists conceptually. Build it:
   ```
   fn self_play(source: &str, generations: usize) -> ShatterResult {
       let mut best = shatter(source);  // initial scramble
       for _ in 0..generations {
           let mutations = spawn_mutations(&best, 4);
           let scored: Vec<_> = mutations.into_iter()
               .map(|m| (compile_and_measure(&m), m))
               .collect();
           let winner = scored.into_iter()
               .min_by(|a, b| a.0.holonomy.partial_cmp(&b.0.holonomy).unwrap())
               .unwrap();
           if winner.0.holonomy < best_holonomy {
               best = winner.1;
           }
       }
       best
   }
   ```
2. PowerPlay extension: maintain a REPERTOIRE of solved problems (files
   that reached crystal). New mutations must not increase holonomy on ANY
   solved file. This prevents catastrophic forgetting.

### What to try first

**Self-play convergence test:** Take one .mirror file. Shatter it
(scramble). Run the self-play loop for 10,000 generations. Plot holonomy
over time. Does it converge? How fast? What's the final holonomy? Try
with different initial shatterings. Is convergence robust to starting
point?

---

## 8. Brainfuck as Computation Substrate

### What we found

**Evolved BF programs** (brainfuck-evolved, BrainfuckIntern): Genetic
algorithms successfully evolve BF programs that produce target output
strings. The technique: population of random BF programs, fitness =
edit distance between program output and target string, selection +
mutation + crossover. "Hello, world!" programs emerge in ~10,000
generations.

**BF as compilation target** (BrainSTARK): BF is used as a simple
instruction set architecture for STARK proof systems. The simplicity
of BF makes formal verification tractable.

**Computational expressiveness:** BF is Turing-complete with 8
instructions. Any computable function can be expressed. The question
is not "can it?" but "how large is the program?" BF programs are
typically 10-100x larger than equivalent programs in other languages.

**BF with fixed weights:** Fate already runs as BF (`fate.bf`). The
program is fixed; the data tape encodes the weights. The BF program
implements the forward pass: read features from tape, multiply by
weights, find argmax.

### How it maps

Shatter-as-BF would be:
- **BF program:** Fixed. Implements the codon table lookup and state
  machine.
- **Data tape:** The weights/codon table entries. These are what evolve.
- **Input:** The byte stream to shatter.
- **Output:** Mirror token indices.

The BF program for Shatter is simpler than Fate's BF program because
the core operation is just a table lookup:

```
read input byte -> index into codon table -> output production rule
```

In BF this is:
1. Read byte to cell 0
2. Use cell 0 as offset into a 256-entry table stored on the tape
3. Output the value at that offset

The wrinkle: BF doesn't have random access. Indexing requires walking
the tape. For a 256-entry table, this means up to 256 moves per byte.
At Fate's 2M decisions/sec, this is still fast enough for practical use
(~8,000 bytes/sec).

**The deeper insight:** If both Fate and Shatter run as BF programs
with data-tape weights, the entire AI system is:
- Two BF programs (Fate + Shatter)
- Two weight tapes (425 bytes + 256 bytes = 681 bytes total)
- One fitness function (holonomy)
- One selection mechanism (tournament)

Total model size: 681 bytes. The entire AI fits in a single TCP packet.

### Implementation approach

1. Write the BF program for codon table lookup (fixed, ~200 instructions)
2. The data tape IS the codon table (256 bytes)
3. Evolve the data tape using the same tournament mechanism as Fate
4. Alternatively: compile the codon table to BF directly (each entry
   becomes a hardcoded output value)

### What to try first

**BF codon table prototype:** Write a BF program that reads one input
byte, looks it up in a 256-byte table on the tape, and outputs the
result. Verify it produces correct output for all 256 possible inputs.
Measure execution speed. If > 1000 lookups/sec, it's viable.

---

## Synthesis: The MAP-Elites Connection

The research surfaced one framework that unifies everything: **MAP-Elites**
(quality-diversity). Instead of searching for a single best solution,
MAP-Elites maintains a grid of solutions organized by behavioral
features. Each cell keeps only its elite (highest fitness for that
feature combination).

For Shatter, the grid axes could be:
- **x-axis:** Output length (short fragments vs long programs)
- **y-axis:** Token diversity (few unique tokens vs many)

Each cell contains the codon table that produces the lowest-holonomy
output for that (length, diversity) combination. This gives us a
CATALOGUE of Shatters, not just one. Different inputs might benefit
from different Shatter strategies.

This connects to the tournament doc's `map(d1, d2)` lens.

---

## Recommended Pipeline

### Step 1: Token frequency analysis

Parse all .mirror files. Count token frequencies. Build the target
distribution that a "good" Shatter should approximate.

**Output:** `token_frequencies.json` — map from token to frequency.

### Step 2: Build the codon table

256 entries. Each maps an input byte to a production rule index.
Initial table: frequency-biased (common tokens get more byte values).

**Output:** `CodonTable` struct — `[u8; 256]` mapping bytes to rules.

### Step 3: Build the GE mapper

Takes a codon table and an input byte stream. Produces MirrorAST by
walking the mirror grammar, using each input byte to select production
rules via the codon table.

**Output:** `fn shatter(table: &CodonTable, input: &[u8]) -> MirrorAST`

### Step 4: Define mutation operators

Eight MirrorAST mutation operators (type variant swap, grammar ref
redirect, action body shuffle, property strengthen, import addition,
fragment crystallization, declaration deletion, subtree swap).

**Output:** `fn mutate(ast: &MirrorAST, op: MutationOp) -> MirrorAST`

### Step 5: Build the evolution loop

Population of codon tables. Each generation: shatter input, compile,
measure holonomy, tournament selection, mutate winning tables.

```rust
fn evolve_shatter(
    input: &[u8],
    target_files: &[&str],  // .mirror files to improve
    generations: usize,
    population_size: usize,
) -> CodonTable {
    let mut population = random_codon_tables(population_size);

    for gen in 0..generations {
        let scored: Vec<_> = population.iter()
            .map(|table| {
                let ast = shatter(table, input);
                let applied = apply_to_targets(&ast, target_files);
                let holonomy = measure_total_holonomy(&applied);
                (holonomy, table.clone())
            })
            .collect();

        population = tournament_select_and_mutate(scored);
    }

    population.into_iter()
        .min_by(|a, b| a.holonomy.partial_cmp(&b.holonomy).unwrap())
        .unwrap()
}
```

**Output:** Trained `CodonTable` that produces low-holonomy output.

### Step 6: Self-play refinement

Once evolution plateaus, switch to self-play: the best codon table
shatters its own output, and the result is re-shattered. The fixed
point is the crystal.

### Step 7: Fate integration

Connect Shatter to Fate. Fate decides WHICH codon table to use (from
the MAP-Elites catalogue). Shatter produces candidates. Fate measures
and selects. The tournament runs the outer loop.

### Step 8: BF compilation

Compile the winning codon table to a BF program. Bake it into the
binary alongside Fate's BF. Total AI: two BF programs, 681 bytes.

---

## v1 Minimum Viable Shatter

The absolute simplest thing that could work:

### Architecture

```
[256-byte codon table] + [mirror grammar BNF] + [compiler] = Shatter v1
```

### Components

1. **Codon table:** `[u8; 256]` — maps input bytes to production rule
   indices. Initial: frequency-biased from corpus analysis.

2. **Grammar walker:** Given a production rule index, emit the
   corresponding mirror token and advance the grammar state.

3. **Compilation + measurement:** Feed the emitted tokens through the
   existing mirror compiler. Measure holonomy.

4. **Tournament:** (1+4) strategy. One parent codon table. Four children
   (each with 3-5 random byte swaps). Keep the child with lowest holonomy.

### Implementation

```rust
/// A 256-byte codon table. The entire Shatter model.
pub struct CodonTable {
    pub entries: [u8; 256],
}

impl CodonTable {
    /// Create a frequency-biased table from token frequencies.
    pub fn from_frequencies(freqs: &[(usize, f64)]) -> Self { ... }

    /// Shatter: map input bytes to mirror token indices.
    pub fn shatter(&self, input: &[u8]) -> Vec<u8> {
        input.iter().map(|b| self.entries[*b as usize]).collect()
    }

    /// Mutate: swap n random entries.
    pub fn mutate(&self, n: usize, rng: &mut impl Rng) -> Self {
        let mut new = self.clone();
        for _ in 0..n {
            let i = rng.gen_range(0..256);
            let j = rng.gen_range(0..256);
            new.entries.swap(i, j);
        }
        new
    }
}

/// One evolution step. Returns the best of parent + 4 children.
pub fn evolve_step(
    parent: &CodonTable,
    input: &[u8],
    grammar: &MirrorGrammar,
    rng: &mut impl Rng,
) -> (CodonTable, f64) {
    let parent_holonomy = evaluate(parent, input, grammar);

    let mut best = (parent.clone(), parent_holonomy);
    for _ in 0..4 {
        let child = parent.mutate(3, rng);
        let h = evaluate(&child, input, grammar);
        if h < best.1 {
            best = (child, h);
        }
    }
    best
}

/// Evaluate a codon table: shatter input, walk grammar, compile, measure.
fn evaluate(
    table: &CodonTable,
    input: &[u8],
    grammar: &MirrorGrammar,
) -> f64 {
    let token_indices = table.shatter(input);
    let source = grammar.walk(&token_indices);
    let (_, loss) = extract_features(&source);
    loss.holonomy()
}
```

### What proves the concept

If, after 10,000 generations of (1+4) evolution, the best codon table
produces .mirror output with holonomy significantly lower than a random
codon table, the concept is proven. The crystal can grow from noise.

### Success criteria

- **Baseline:** Random codon table average holonomy (probably ~8-10, most
  output is Fragment nodes)
- **Target:** Evolved codon table holonomy < 2.0 after 10,000 generations
- **Crystal:** Holonomy = 0.0 (the output is valid .mirror that compiles
  clean)

### Timeline

- Day 1: Token frequency analysis + codon table struct
- Day 2: Grammar walker (BNF extraction from parser)
- Day 3: Evolution loop + (1+4) tournament
- Day 4: Run experiments, measure convergence
- Day 5: Integrate with Fate bridge, connect to `mirror ai`

### What it gives us

Even v1 MVP proves:
1. Arbitrary input CAN be transformed into mirror syntax
2. Evolutionary search over a 256-byte model CAN reduce holonomy
3. The compilation-as-fitness-function loop works
4. The crystal-from-noise thesis is testable

If it works, everything else (MAP-Elites catalogue, BF compilation,
self-play refinement, developmental encoding) is optimization of a
proven concept.

---

## Research Sources

### Token Scrambling / Recombination
- Grammatical Evolution: Ryan, Collins, O'Neill (2001). BNF grammar + integer codons.
- PonyGE2: reference GE implementation (github.com/PonyGE/PonyGE2)
- AFL++ Grammar-Mutator: grammar-guided fuzzing (github.com/AFLplusplus/Grammar-Mutator)
- Grammarinator: ANTLR grammar-based fuzzer (grammarinator.readthedocs.io)
- BPE: Sennrich et al. (2016). Subword tokenization for NMT.

### Mutation Testing
- Pitest: Java mutation testing (pitest.org). 30+ mutation operators.
- Stryker: JS/C#/Scala mutation testing (stryker-mutator.io).
- cargo-mutants: Rust mutation testing (mutants.rs).
- Higher-order mutation: Jia & Harman (2009).

### Evolutionary Program Synthesis
- PushGP: Spector et al. (2005). Stack-based GP.
- CGP: Miller (2011). Graph-encoded programs, mutation-only evolution.
- GEP: Ferreira (2001). Fixed-length genomes, expression trees.
- AlphaEvolve/OpenEvolve: Google DeepMind (2025). LLM + MAP-Elites.
- ADATE: Olsson (1995). Incremental program transformations.
- DeepCoder: Balog et al. (2017). Neural-guided program synthesis.

### Adversarial / Generative
- DiffuSeq: Gong et al. (ICLR 2023). Discrete diffusion for text.
- CodeRL: Le et al. (NeurIPS 2022). RL for program synthesis.
- Discrete VAE: tutorial at arxiv.org/abs/2505.10344.

### Small Models
- Lottery Ticket Hypothesis: Frankle & Carlin (2018). Sparse trainable subnetworks.
- Extreme Learning Machines: Huang et al. (2006). Random hidden, trained readout.
- Reservoir Computing: Jaeger (2001). Echo state networks.
- muNAS: Liberis et al. (2021). NAS for microcontrollers (< 64KB).
- Binary Neural Networks: 1-bit weights (arxiv.org/abs/2509.07025).

### Genotype-Phenotype Mapping
- Biological codon table: 64 codons -> 20 amino acids. Redundancy = robustness.
- HyperNEAT: Stanley et al. (2009). CPPN indirect encoding.
- CodonTransformer: ML-based codon optimization (github.com/Adibvafa/CodonTransformer).
- Developmental encoding: Mouret (2024). Meta-learning evolvable mappings.

### Self-Improving Systems
- Godel Machines: Schmidhuber (2003). Provably optimal self-improvement.
- PowerPlay: Schmidhuber (2011). Self-invented problems + monotonic capability growth.
- AlphaGo Zero: Silver et al. (2017). Self-play from scratch.
- Iterated Amplification: Christiano et al. Recursive decomposition.

### Minimal Computation
- brainfuck-evolved: GA-evolved BF programs (github.com/kurtjd/brainfuck-evolved).
- BrainSTARK: BF as verification substrate (aszepieniec.github.io/stark-brainfuck).

### Quality-Diversity
- MAP-Elites: Mouret & Clune (2015). Quality-diversity algorithm.
- Lexicase Selection: Spector et al. Specialist-preserving selection.
