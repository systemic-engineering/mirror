# Tournament Rules — Named Lenses for `mirror ai`

`mirror ai --tournament <rules>` selects which transformation survives each round. Rules are named lenses. They compose with `.` (lens composition). Holonomy is the fitness function (lower is better).

```bash
mirror ai --tournament beam(8).greedy              # sample 8, pick best
mirror ai --tournament elite(1).halving(3)         # never lose best, eliminate early
mirror ai --tournament tabu(3).beam(8).anneal(0.5) # no repeats, 8 candidates, accept worse sometimes
```

---

## v1 — Essential Lenses

### `greedy`

**Source:** Best-of-N sampling. The baseline.

Pick the candidate with the lowest holonomy. Period.

- **When it wins:** Fast convergence on smooth landscapes.
- **When it fails:** Gets stuck in local minima.
- **Compose:** `beam(k).greedy` — sample k, pick best.

### `beam(k)`

**Source:** k-Tournament selection (evolutionary computation).

Sample k candidates per round. The primary dial. k=1 is random, k=N is exhaustive. Selection pressure increases with k.

- **When it wins:** General purpose. Tunable exploration/exploitation.
- **When it fails:** High k causes premature convergence — population diversity collapses.
- **Compose:** `beam(8).greedy`, `beam(8).halving(3)`, `beam(8).anneal(T)`

### `elite(k)`

**Source:** Elitism (evolutionary computation).

Unconditionally preserve the k best candidates across rounds. The best solution found is never lost.

- **When it wins:** Always useful as a combinator. Guarantees monotonic improvement.
- **When it fails:** Too much elitism reduces diversity. k=N is no evolution at all.
- **Compose:** `elite(1).beam(8).greedy` — keep best, run 8-candidate tournaments.
- **Note:** Combinator, not standalone. Pairs with everything.

### `halving(eta)`

**Source:** Successive Halving / Hyperband (hyperparameter tuning).

Start with many candidates, give each a small evaluation budget (1 grammar file). Keep the top 1/eta fraction. Give survivors eta times more evaluation (3 files). Repeat until one remains.

- **When it wins:** Compute-efficient. Bad candidates die early. Directly maps to sequential grammar file evaluation.
- **When it fails:** When cheap evaluations (1 file) are uninformative about full quality.
- **Compose:** `beam(27).halving(3)` — 27 candidates, 9 survive round 1, 3 survive round 2, 1 wins.

### `tabu(tenure)`

**Source:** Tabu Search (combinatorial optimization).

Remember the last `tenure` transformation types applied. Don't re-apply them. Forces structural diversity. Prevents oscillation between two states.

- **When it wins:** When the optimizer cycles between the same two transformations.
- **When it fails:** Tenure too long = unnecessarily restrictive.
- **Compose:** `tabu(3).beam(8).greedy` — 8 candidates, but don't re-use last 3 types.

---

## v1 — Strong Additions

### `anneal(T)`

**Source:** Boltzmann selection / Simulated Annealing (statistical mechanics).

Accept improvements always. Accept worse solutions with probability exp(-delta/T). Temperature T decreases over time. High T = exploratory, low T = exploitative.

- **When it wins:** Escaping local minima. Rugged landscapes with many traps.
- **When it fails:** Cooling schedule is itself an optimization problem.
- **Compose:** `anneal(geometric)`, `anneal(linear)`, `beam(8).anneal(0.5)`
- **Schedules:** `geometric` (T *= 0.95), `linear` (T -= step), `adaptive` (T responds to progress).

### `ucb(c)`

**Source:** Upper Confidence Bound (multi-armed bandits).

Choose transformation type with highest UCB = mean_reward + c * sqrt(ln(t) / n_i). Balances exploitation (high mean holonomy reduction) with exploration (under-tried types).

- **When it wins:** Allocating limited trials across transformation strategies. Principled exploration.
- **When it fails:** Assumes stationary reward distributions.
- **Compose:** `ucb(2).beam(4).greedy` — UCB selects which type to try, beam generates 4 instances.
- **Note:** Learns which Fate model selections (abyss/introject/cartographer/explorer) actually reduce holonomy.

---

## v2 — Research Candidates

### `niche(r)`

**Source:** Fitness Sharing (evolutionary computation).

Fitness is divided by the number of similar candidates within radius r. Penalizes crowding. Multiple optima coexist.

- **When it wins:** Multimodal landscapes — find many good solutions, not just one.
- **When it fails:** Radius r is hard to set.
- **Maps to:** If two transformations produce similar results, penalize both. Force structural diversity.

### `lexicase`

**Source:** Lexicase Selection (program synthesis).

Shuffle the grammar files randomly. Filter: keep candidates best on file 1, then filter those on file 2, etc. Each selection event uses a different random ordering.

- **When it wins:** Multi-file evaluation. Preserves specialists — a transformation perfect for one file survives even if mediocre overall. Empirically outperforms tournament selection in program synthesis.
- **When it fails:** Expensive. O(population * files) per selection.
- **Maps to:** Directly applicable. The compiler has multiple grammar files, each a test case. Shuffle file order, filter sequentially.

### `clonal`

**Source:** Clonal Selection (immune system).

High-affinity candidates are cloned more and mutated less. Low-affinity candidates are cloned less and mutated more. Mutation rate adapts per-individual.

- **When it wins:** Automatic adaptation of search intensity. Good solutions refined gently, bad ones transformed radically.
- **When it fails:** Complex to implement. Multiple parameters.
- **Maps to:** Transformations with low holonomy get refined with small edits. High holonomy gets radical restructuring.

### `speculate(k)`

**Source:** Speculative Decoding (LLMs).

A fast/cheap model proposes k transformations. An expensive model verifies. Accepted proposals are kept. Rejected ones are resampled from the expensive model.

- **When it wins:** When you have a cheap approximation and expensive oracle. Lossless speedup.
- **When it fails:** Low acceptance rate = no speedup.
- **Maps to:** Excited Fate (cheap, random) proposes. Trained Fate (expensive, accurate) verifies.

### `swiss(rounds)`

**Source:** Swiss System (chess tournaments).

Each round, pair candidates with similar records. After fixed rounds, rank by record. O(n log n) rounds to rank n candidates.

- **When it wins:** Large fields where round-robin is too expensive but you want accurate ranking.
- **Maps to:** All transformations play 5 rounds. Match transformations with similar holonomy against each other on the same files.

### `island(n, m)`

**Source:** Island Model (evolutionary computation).

Run n independent populations. Every m generations, migrate best individuals between islands.

- **When it wins:** Embarrassingly parallel. Different islands can use different strategies.
- **Maps to:** Each Fate instance IS an island. Run n independently, periodically share the best. Natural fit for the architecture.

### `thompson`

**Source:** Thompson Sampling (Bayesian bandits).

Maintain a Bayesian posterior per transformation type. Sample from each posterior. Try the most optimistic sample. Update with results.

- **When it wins:** Empirically outperforms UCB in many settings. Principled Bayesian exploration.
- **Maps to:** Model each transformation type as Beta(successes, failures). Sample, try, update.

### `race`

**Source:** F-Race (algorithm configuration).

Run all candidates on a sequence of grammar files. After each file, apply a statistical test (Friedman). Eliminate candidates that are significantly worse. Continue until one remains.

- **When it wins:** Statistically principled early elimination. Only eliminates with EVIDENCE.
- **Maps to:** Directly applicable. Sequential evaluation across .mirror files with statistical stopping.

### `novelty`

**Source:** Novelty Search (evolutionary computation).

Select based on behavioral novelty — how different this transformation is from all previous attempts. Completely ignores holonomy.

- **When it wins:** When the holonomy gradient is misleading. Deceptive landscapes.
- **When it fails:** Wasteful on straightforward problems.
- **Maps to:** "Try something we haven't tried." Escape local minima by rewarding structural novelty.

### `map(d1, d2)`

**Source:** MAP-Elites (quality-diversity algorithms).

Define a grid over behavioral dimensions. Each cell keeps only its elite. Output is a catalogue, not a single winner.

- **When it wins:** When you want to understand the landscape. "Best small rename" AND "best large restructure."
- **Maps to:** Define axes like (transformation_size, transformation_type). Output: a menu of options.

### `borda`

**Source:** Borda Count (voting theory).

Each grammar file ranks all transformations by holonomy. Points by position. Sum across files.

- **When it wins:** Consensus. Favors consistently good over occasionally brilliant.
- **Maps to:** The transformation that's "least objectionable" across all grammar files.

### `danger`

**Source:** Danger Theory (immune system).

Don't reject transformations for being novel. Only reject ones that cause damage (holonomy increase). Tolerate radical changes that work.

- **When it wins:** When you want to allow structural innovation. Novelty is not threat.
- **Maps to:** Accept any transformation that doesn't increase holonomy, regardless of how different it looks.

---

## Exploration — Later

| Lens | Source | Why later |
|------|--------|-----------|
| `cfr` | Counterfactual Regret | Needs imperfect information formulation |
| `cma` | CMA-ES | Needs parameterized transformation space |
| `coevolve` | Coevolution | Adversarial grammar construction — complex setup |
| `condorcet` | Condorcet Voting | O(N^2) pairwise comparisons |
| `species` | NEAT Speciation | Needs structural distance metric |
| `mwu(eta)` | Multiplicative Weights | Meta-algorithm — could implement other rules |
| `replicator` | Replicator Dynamics | Continuous-time, needs discrete approximation |
| `fisher` | Sexual Selection | Cautionary: runaway preference ≠ fitness |
| `royale(a,b)` | Battle Royale Scoring | Multi-objective weighting |
| `elo(K)` / `glicko` | Rating Systems | Long-running strategy evaluation |

---

## Composition Grammar

Rules compose with `.` (lens composition). Left-to-right: outer lens wraps inner.

```
beam(8).greedy           — sample 8, pick best
elite(1).beam(8).greedy  — keep best, sample 8, pick best
tabu(3).beam(8).anneal   — no repeats, sample 8, accept worse sometimes
ucb(2).beam(4).halving(3) — explore types, 4 candidates, eliminate early
elite(1).lexicase        — keep overall best, lexicase for the rest
niche(0.1).beam(8).greedy — diversity-preserving tournament
island(4,5).beam(3).greedy — 4 independent populations, migrate every 5 rounds
```

Type checking: the compiler can verify compositions are well-formed. `chaos.chaos` is noise on noise. `elite.elite` is redundant. The type system catches structural nonsense.

---

## The Meta-Game

The Fate model that selects WHICH tournament rule to use — that's the game master. The model that learned not just which optic to apply, but which selection strategy to use for picking optics.

Tournament rules are the meta-game over the AI loop:
- Fate selects operations (which optic)
- Tournament rules select Fates (which instances survive)
- The game master selects tournament rules (which selection strategy)

Three levels. All measured. All bounded by holonomy.

---

## Key Insight

The research surfaced one mechanism that maps perfectly to the architecture: **Successive Halving**. Start N candidates on 1 grammar file. Keep top third. Evaluate survivors on 3 files. Keep top third. Evaluate on all files. This IS the compute allocation problem. The grammar files are the evaluation instances. The budget is compute. Halving allocates it optimally.

For v1: `elite(1).beam(k).halving(eta)` with `tabu(tenure)` as optional modifier and `ucb(c)` for strategy selection. Five lenses. They compose. They cover the primary failure modes.
