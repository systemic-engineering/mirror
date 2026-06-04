# Liquid Types for Mirror -- Automatic Property Inference

*2026-05-19. Reed. Research spec.*

Status: **Research** (deep literature survey + architectural mapping)

Depends on: `type-theory-position.md`, `epistemologic-grammar.md`,
`property-error-surface.md`, `typed-loss-composition.md`

---

## 0. Executive Summary

Liquid Types (Rondon, Kawaguchi, Jhala -- PLDI 2008) combine Hindley-Milner
type inference with predicate abstraction to automatically infer dependent
types precise enough to prove safety properties. The question: can mirror
adopt this approach to automatically infer PROPERTIES of `\`-resolved code?

**The answer is: yes, partially, but mirror needs something different.**

Liquid Types solve the right problem (automatic property inference) with
the wrong mechanism for mirror's architecture (SMT solving over boolean
predicates). Mirror's verdicts are continuous, not boolean. Mirror's
constraint system is a spectral graph, not a logical formula. The path
forward is a hybrid: adopt Liquid's INFERENCE FRAMEWORK (predicate
abstraction from qualifier templates) but replace the DECISION PROCEDURE
(SMT) with spectral analysis (eigenvalue computation on the property graph).

This document lays out exactly what Liquid Types are, where they map to
mirror, where they diverge, and what the integration architecture looks like.

---

## 1. What Liquid Types Are

### 1.1 The Core Idea

Liquid Types restrict dependent types to a decidable, automatically
inferable fragment. A liquid type has the form:

```
{v : B | r}
```

where `B` is a base type, `v` is the value variable, and `r` is a
**refinement predicate** -- a boolean expression from a decidable logic
(quantifier-free linear arithmetic + uninterpreted functions + equality).

Example: `{v : Int | v > 0}` means "positive integers."

The key insight: refinement predicates are not arbitrary logical formulas.
They are **conjunctions of qualifiers** drawn from a finite, user-provided
set Q. A qualifier is a boolean predicate template:

```
Q = { v > 0, v < len(a), v = 0, v != null, ... }
```

Given this qualifier set, the Liquid inference algorithm determines which
conjunction of qualifiers from Q holds at each program point. This is
**predicate abstraction** -- projecting the concrete program state onto
the abstract domain defined by Q.

### 1.2 The Algorithm (Three Steps)

**Step 1: Hindley-Milner Type Inference.**
Run standard HM inference to determine the shape types (Int, Bool,
List a, etc.). This gives the `B` in `{v : B | r}`. No user annotation
needed for this step.

**Step 2: Liquid Constraint Generation.**
Walk the typed AST. At each program point, generate subtyping constraints
of the form `G |- {v:B | r1} <: {v:B | r2}`, where `r1` and `r2` are
**liquid variables** -- unknowns that will be solved as conjunctions of
qualifiers from Q. The constraint generation is syntax-directed: each
AST node type (let, application, lambda, if-then-else, match) generates
specific constraint forms.

For conditionals, path sensitivity comes for free: the branch condition
is added to the environment, so the refinements inferred inside a branch
reflect the branch guard.

**Step 3: Constraint Solving via Predicate Abstraction.**
Initialize each liquid variable to the conjunction of ALL qualifiers in Q
(the most refined possible type). Then iteratively weaken: for each
constraint `G |- {v:B | r1} <: {v:B | r2}`, check via SMT whether the
implication holds. If not, remove qualifiers from r2 until it does.
This is a fixed-point iteration: it terminates because Q is finite and
only qualifiers are removed (monotone decreasing).

The result: for each program point, the strongest conjunction of qualifiers
from Q that is consistent with the program's behavior.

### 1.3 Decidability and Expressiveness

**Decidable because:**
- Qualifiers come from a finite set Q
- The logic is quantifier-free (QF_UFLIA or similar decidable SMT fragment)
- Subtyping reduces to implication, which reduces to SMT validity checking
- The fixed-point iteration terminates in O(|Q| * |constraints|) steps

**Expressive enough for:**
- Array bounds checking (the original motivation)
- Null pointer safety
- Termination metrics (decreasing arguments)
- Data structure invariants (sorted lists, balanced trees)
- Resource bounds (via bounded refinements)
- State machine protocols (via abstract refinements)

**Cannot infer:**
- Properties requiring quantifier alternation (forall-exists)
- Properties outside the chosen qualifier set Q (the inference is only
  as good as the qualifiers you provide)
- Non-linear arithmetic properties (unless the SMT solver supports them,
  which makes decidability conditional)
- Relational properties across multiple data structures (requires
  abstract or bounded refinements -- extensions, not the base system)
- Timing properties, probabilistic properties, information-theoretic
  properties

### 1.4 The Evolution

**Liquid Types (PLDI 2008)** -- the original. ML-like language. Fixed
qualifier set. SMT solving for implication. Reduces annotation burden
from 31% to under 1%.

**Abstract Refinement Types (ESOP 2013)** -- quantification over
refinements. Instead of fixing a concrete predicate, parameterize a
type by an abstract refinement: `List <p> a` where `<p>` is an
abstract predicate. Enables: parametric container invariants, recursive
data structure invariants, index-dependent properties. Crucially:
inferring the INSTANTIATION of abstract refinements reduces to standard
Liquid inference (the abstraction is at the specification level, not
the inference level).

**Refinement Types for Haskell / Liquid Haskell (ICFP 2014)** -- extends
Liquid Types to Haskell, handling lazy evaluation. Key challenge: in a
lazy language, a binder might be bound to a diverging expression (bottom).
Solution: a stratified type system that labels binders as Div (may diverge),
Wnf (weak head normal form), or Fin (finite). Refinements only hold for
non-bottom values. Proves 96% of recursive functions terminating with
1.7 lines of annotation per 100 LOC.

**Bounded Refinement Types (ICFP 2015)** -- adds bounded quantification.
Abstract refinement parameters can be BOUNDED by Horn implications:
`forall <p :: Int -> Bool, q :: Int -> Bool>. p v => q v => ...`
Enables: relational algebra (safe database access), Floyd-Hoare logic
within a state transformer monad, capability-based security. The bounds
are translated into ghost functions to preserve decidability.

**Gradual Liquid Type Inference (PLDI 2018)** -- combines liquid inference
with gradual typing. Unknown refinements are represented by the "dynamic"
refinement (`?`), which is consistent with any concrete refinement. This
enables MODULAR verification: a module boundary gets `?`, and the
concrete refinement is inferred from usage. Interactive tool: GuiLT.

**Flux: Liquid Types for Rust (PLDI 2023)** -- applies refinement types
to Rust. Key insight: Rust's ownership system and refinement types are
complementary. Ownership handles aliasing and memory safety; refinements
handle functional correctness. Flux exploits this factoring: complex
invariants describing container contents are synthesized via liquid
inference. Implemented as a Rust compiler plugin.

**Data Flow Refinement Type Inference (POPL 2021)** -- a parametric
framework that shows Liquid Types are ONE instantiation of a broader
design space. The framework is parametric in the abstract domain (not
just conjunctions of qualifiers -- also octagons, polyhedra, intervals).
This is the theoretical unification: all refinement type inference is
abstract interpretation over data flow semantics.

### 1.5 Performance

From the Liquid Haskell experience paper (Haskell 2014): verification of
10,000+ lines of Haskell takes seconds to minutes. The bottleneck is SMT
solving, not constraint generation. Z3 handles the quantifier-free
fragments efficiently (milliseconds per query). The fixed-point iteration
typically converges in 3-5 rounds.

From Flux (PLDI 2023): verification of Rust programs with refinement
types adds roughly 2x to compilation time. The overhead is dominated by
SMT queries for subtyping checks.

### 1.6 Usability Barriers

Recent work (Gamboa et al., PACMPL 2025, "Usability Barriers for Liquid
Types") identifies key adoption challenges:

1. **Error messages are opaque.** When inference fails, the user sees an
   SMT-level error (unsatisfiable constraint), not a domain-level
   explanation. The gap between the programmer's mental model and the
   solver's model is large.

2. **Qualifier selection is non-obvious.** The inference is only as
   good as Q. Choosing the right qualifiers requires understanding both
   the program AND the logic. No good tooling for discovering useful
   qualifiers.

3. **Modularity tension.** Inference is whole-program by default.
   Gradual liquid types (PLDI 2018) address this but add complexity.

4. **Higher-order functions are hard.** Inferring refinements for
   higher-order functions requires abstract refinement types, which
   add specification burden.

---

## 2. The Match with Mirror

### 2.1 Where Liquid Maps Directly

| Liquid Concept | Mirror Equivalent | Mapping Quality |
|---------------|-------------------|-----------------|
| Base type B | AST variant (Focus/Project/Split/Shift/Settle/In/Out) | Direct |
| Refinement predicate r | Property verdict | Structural (see 2.2) |
| Qualifier set Q | Property library (@epistemologic/property/*) | Direct |
| Predicate abstraction | Settlement iteration | Structural |
| Subtyping constraint | Type compatibility across `in`/`out` boundaries | Direct |
| Liquid variable (unknown refinement) | `\` hole (unknown implementation) | Structural |
| Fixed-point iteration | Tick loop (compile, measure loss, repeat) | Direct |
| SMT solver | Spectral loss computation | Replacement (see section 5) |

**The qualifier set IS the property library.** Mirror already has a
library of property checks: `duplicate_variant`, `unresolved_import`,
`unused_declaration`, `arity_mismatch`, `missing_export`,
`unreachable_type`, `circular_import`, plus the `@epistemologic`
properties (`literal`, `override_ratio`). Each property is a qualifier
template. The inference question is: which properties hold for a given
grammar action, automatically?

**The `\` hole IS a liquid variable.** In Liquid Types, a liquid variable
is an unknown refinement to be solved. In mirror, a `\` hole is an
unknown implementation to be solved (by Fate). The parallel: just as
Liquid inference determines which qualifiers hold for a liquid variable,
mirror can determine which properties hold for a `\`-resolved hole.

**The tick loop IS fixed-point iteration.** Mirror compiles, measures
loss, adjusts, repeats. Liquid inference generates constraints, solves,
weakens, repeats. Both are convergent fixed-point processes.

### 2.2 The Critical Divergence: Verdicts vs Booleans

Liquid predicates are boolean: `v > 0` is true or false.
Mirror verdicts are three-valued with continuous loss:

```
pass                        -- loss = 0
partial(observation, loss)  -- 0 < loss < total
fail(observation, loss)     -- loss = total
```

This is not a cosmetic difference. It changes the ALGEBRA of inference.

In Liquid Types, the constraint `G |- {v:B|r1} <: {v:B|r2}` reduces to
the boolean implication `G & r1 => r2`. Either the implication holds
(valid) or it does not (invalid). There is no middle ground.

In mirror, the analogous constraint would be: "given context G, does
property P hold for term t?" But the answer is not yes/no -- it is a
LOSS VALUE. The property might hold with loss 0.03 (nearly perfect) or
loss 0.97 (nearly failed). The constraint is not `G & r1 => r2` but
rather `loss(G, P, t) <= threshold`.

This means:
1. Mirror's constraints are SOFT, not hard. A constraint can be
   "mostly satisfied" with measurable residual.
2. The fixed-point iteration converges toward MINIMUM LOSS, not toward
   a boolean satisfying assignment.
3. The decision procedure is OPTIMIZATION (minimize loss), not
   SATISFIABILITY (find a satisfying assignment).

### 2.3 `\` Inference and Properties

When Fate fills a `\` hole, the question becomes: what properties does
the filled hole satisfy?

```mirror
collapse(ast, ast) -> imperfect {
  focus(a, b) |> split |> \ |> settle
}
```

Fate fills `\` with (say) `zoom(merge)`. Can Liquid-style inference
automatically determine:

1. **Type preservation:** `zoom(merge)` has type `[ast] -> ast`.
   YES -- this is standard Liquid inference. The pipeline's type
   signature constrains the hole's type. Fate navigates the type
   graph to find a type-correct filling. The type IS the base type B.

2. **Property `literal`:** the name "merge" implies merging behavior.
   PARTIALLY -- `literal` checks whether the declared identity matches
   observed behavior. The qualifier `literal(zoom(merge))` can be
   automatically checked AFTER Fate fills the hole. The check measures
   loss between the name "merge" and the actual behavior. This is
   post-hoc verification, not inference.

3. **Loss reduction:** the output has fewer nodes than the input.
   YES, with the right qualifier. If the qualifier set Q contains
   `loss(output) < loss(input)`, then Liquid-style inference can
   propagate this through the pipeline. `focus` preserves loss
   (observation). `split` may increase loss (enumeration creates
   parts). `shift(merge)` should decrease loss (merging). `settle`
   measures loss (settlement). The pipeline's loss profile can be
   inferred from the individual operation profiles.

4. **Return type `imperfect`:** the full pipeline satisfies the
   declared return type.
   YES -- the return type `imperfect` is the base type. Liquid
   inference determines the refinement: `{v : imperfect | loss(v) < T}`
   for some threshold T. The threshold is inferred from the qualifier
   set and the pipeline's composition.

### 2.4 Property Flow Through Pipes

Mirror's `|>` composition is where Liquid inference shines. Each
operation in the pipeline has a type with refinements. Composition
propagates refinements through the chain:

```
focus : (a : ast, b : ast) -> {r : ast | loss(r) = 0}
split : (r : ast) -> {parts : [ast] | len(parts) > 0}
shift(merge) : (parts : [ast]) -> {m : ast | nodes(m) <= sum(nodes(parts))}
settle : (m : ast) -> {c : imperfect | loss(c) = measured(m)}
```

The Liquid constraint for the full pipeline:
```
{c : imperfect | loss(c) = measured(merge(split(focus(a, b))))}
```

The refinement flows left-to-right through the pipe. Each operation
adds its own refinement. The final refinement is the composition.
This is exactly what Liquid inference does for function composition.

### 2.5 Weighted Composition `|>`

Mirror's `|>` carries implicit weights from the eigenboard. In Liquid
terms, this is a GRADED composition where the refinement carries a
probability-like weight:

```
|> : (a -> {b | r1}) -> (b -> {c | r2}) -> (a -> {c | r1 & r2, weight = w})
```

Standard Liquid Types do not carry weights. But Bounded Refinement Types
(ICFP 2015) can encode this: the weight is a ghost parameter that
flows through the bound. The bound is:

```
forall <w : Float>. w > 0 => ...
```

This is speculative. The actual encoding would require extending the
qualifier set Q with weight-carrying predicates. No existing Liquid
system does this.

### 2.6 `@epistemologic.literal` and Name/Operation Isomorphism

Can Liquid verify that a name IS its operation? The `literal` property
checks `loss(name, operation) < epsilon`. This is a refinement:

```
{f : action | literal_loss(f) < epsilon}
```

Liquid inference could propagate this if `literal_loss` is in the
qualifier set. But `literal_loss` requires RUNNING the operation on
test data and measuring the result -- it is not a static predicate.
Liquid Types are strictly static (compile-time). `literal` requires
dynamic measurement.

**Conclusion:** Liquid can track literal-validity ONCE MEASURED, but
cannot INFER it statically. The measurement must happen first (at
runtime or test time). Then the result can be encoded as a refinement
and propagated.

---

## 3. The \ Inference Question (Detailed)

### 3.1 The Setup

```mirror
collapse(ast, ast) -> imperfect {
  focus(a, b) |\> split |\> \ |\> settle
}
```

Fate fills `\` with `zoom(merge)`. The question: can Liquid-style
inference automatically determine what properties `zoom(merge)` satisfies,
without the programmer writing ANY of these properties?

### 3.2 What CAN Be Inferred Automatically

**Type compatibility.** The pipeline constrains the `\` hole's type:
input is `[ast]` (output of `split`), output must be compatible with
`settle`'s input (which is `ast`). So the hole must have type
`[ast] -> ast`. Any Fate-selected operation that doesn't match this
type is rejected. This is standard Liquid inference.

**Monotonicity properties.** If the qualifier set includes
`loss(output) <= loss(input)`, the inference can check whether
`zoom(merge)` satisfies this. The check: does merging reduce or
preserve loss? Since `zoom` is a Lens (read and write), and `merge`
combines multiple ASTs into one, the loss should decrease (fewer
nodes = less information = lower complexity). The qualifier holds.

**Composition properties.** Properties that are compositional (if A
holds for f and B holds for g, then C holds for f |> g) can be
inferred across the whole pipeline. Example: if `focus` preserves
structure and `split` preserves structure and `shift(merge)` preserves
structure and `settle` measures structure, then the pipeline preserves
structure. This is exactly what abstract refinement types do: parameterize
the property and propagate it through composition.

**Contract satisfaction.** The `in`/`out` boundary of the enclosing
grammar specifies what `collapse` must provide. The pipeline's
refinement must satisfy the `out` contract. This is a subtyping check:
`pipeline_refinement <: out_contract_refinement`. Standard Liquid.

### 3.3 What CANNOT Be Inferred Automatically

**The `literal` property for the name "merge."** The property
"the operation named 'merge' actually merges" requires semantic
understanding of what "merge" means. No static type system can infer
this -- it requires either: (a) a formal specification of "merge"
(which IS the annotation we're trying to avoid), or (b) dynamic
measurement against test data (which is what mirror's `literal`
property does). Liquid inference cannot help here.

**Information-theoretic properties.** Mirror's loss is Shannon entropy.
Computing Shannon entropy requires knowing the probability distribution
of the data. At compile time, the distribution is unknown. Liquid
inference can propagate BOUNDS on loss (if a qualifier says
`loss(f(x)) <= 2 * loss(x)`, the inference can track this), but
cannot compute the actual loss value.

**Eigenboard weights.** The weights on `|\>` come from the Dirac
operator's spectrum. These are computed from the grammar graph's
eigenvalues, not from the program text. Liquid inference operates on
program text. The eigenboard is external to the type system.

**Cross-grammar properties.** Whether `zoom(merge)` in grammar A is
compatible with `split(decompose)` in grammar B depends on the sheaf
structure (the restriction maps between grammars). Standard Liquid
inference is intra-module. The sheaf consistency check is inter-module.
Bounded refinement types COULD encode this (the bound is the sheaf
restriction map), but this would be a novel extension.

### 3.4 The Honest Assessment

For the specific example:

| Property | Liquid can infer? | Mechanism |
|----------|------------------|-----------|
| `zoom(merge)` has type `[ast] -> ast` | YES | Standard HM + Liquid |
| `zoom(merge)` preserves loss bound | YES, if qualifier in Q | Predicate abstraction |
| `zoom(merge)` name IS operation | NO | Requires dynamic measurement |
| Pipeline satisfies return type `imperfect` | YES | Subtyping + composition |
| Pipeline reduces total loss | YES, if qualifier in Q | Fixed-point iteration |
| Eigenboard weights are correct | NO | External to type system |

**The pattern:** Liquid can infer properties that are COMPOSITIONAL and
STATIC. It cannot infer properties that are SEMANTIC (require
understanding meaning) or DYNAMIC (require runtime measurement).

---

## 4. Continuous Verdicts vs Boolean Predicates

### 4.1 The Extension Problem

Can refinement types be extended from boolean predicates to continuous
(loss-valued) predicates? This is the fundamental theoretical question.

**Boolean refinement type:** `{v : Int | v > 0}`
Decision: v > 0 is true or false. SMT returns SAT or UNSAT.

**Continuous refinement type (proposed):** `{v : ast | loss(v) < 0.1}`
Decision: loss(v) < 0.1 is true or false -- but loss(v) is a CONTINUOUS
function. The constraint LOOKS boolean but the predicate involves a
continuous measurement.

**Even more continuous:** `{v : ast | loss(v) = 0.073}`
This is a verdict, not a predicate. The type carries a continuous annotation.
This is no longer refinement typing -- it is GRADED typing.

### 4.2 The Theoretical Framework

Mirror's verdicts fit into **continuous model theory** (Ben Yaacov et al.
2008, *Model Theory for Metric Structures*). In continuous logic, truth
values are elements of [0,1] rather than {true, false}. Connectives are
continuous functions. The models are metric structures (complete metric
spaces with uniformly continuous interpretations of function and
predicate symbols).

The mapping:
- Boolean logic -> decidable SMT fragments -> Liquid Types
- Continuous logic -> metric structure satisfaction -> Mirror verdicts

In continuous model theory, a "type" (in the model-theoretic sense) is
a set of conditions `{phi(v) <= epsilon}` where phi is a continuous
formula and epsilon is a real number. This is EXACTLY mirror's verdict:
a set of conditions `{loss(property, v) <= threshold}` where loss is
a continuous measurement.

### 4.3 What Changes in the Inference Algorithm

**Step 1 stays the same.** HM inference determines shape types. No change.

**Step 2 changes.** Instead of generating boolean subtyping constraints
`G |- {v:B|r1} <: {v:B|r2}` (where r1, r2 are boolean), generate
continuous constraints `G |- {v:B|l1} <: {v:B|l2}` where l1, l2 are
loss bounds. The constraint is: `loss_1(v) <= loss_2(v)` (the more
refined type has lower loss).

**Step 3 changes fundamentally.** Instead of SMT solving (SAT/UNSAT),
use OPTIMIZATION (minimize total loss subject to constraints). The
fixed-point iteration becomes:

1. Initialize each liquid variable to loss = 0 (the most refined
   possible type -- zero loss means perfect).
2. For each constraint, compute the actual loss.
3. If the loss exceeds the threshold, WEAKEN the refinement (increase
   the loss bound).
4. Repeat until convergence.

This is ABSTRACT INTERPRETATION over the loss semiring, not predicate
abstraction over a boolean qualifier set. The abstract domain is
[0, infinity) instead of 2^Q.

### 4.4 Decidability of Continuous Inference

Boolean Liquid inference is decidable because Q is finite and SMT
queries are decidable.

Continuous inference is decidable IF:
- The loss function is computable (Shannon entropy is computable)
- The threshold comparisons are decidable (real number comparison is
  trivially decidable for computable reals)
- The fixed-point iteration converges (guaranteed if loss is monotonically
  non-increasing across iterations -- which is mirror's `e^(n+1) < e^(n)`)

The convergence guarantee IS mirror's core theorem. The proof is the
business model.

### 4.5 The Hybrid: Liquid Framework, Spectral Decision

The recommendation: adopt Liquid's INFERENCE FRAMEWORK (Steps 1-2) but
replace Step 3 (SMT solving) with mirror's SPECTRAL ANALYSIS. This gives:

1. **Shape type inference** (Hindley-Milner) -- determines which AST
   variants are involved
2. **Constraint generation** (Liquid-style) -- generates constraints
   from the typed AST, but constraints carry loss annotations instead
   of boolean predicates
3. **Spectral constraint solving** -- instead of SMT, compute eigenvalues
   of the constraint graph. The eigenvalues determine which properties
   are satisfiable (spectral gap > 0) and to what degree (the loss is
   encoded in the eigenvalue magnitude)

This is a novel system. No one has combined Liquid's inference framework
with spectral decision procedures.

---

## 5. The Spectral Alternative to SMT

### 5.1 Why Eigenvalues Instead of SAT Solving?

SMT solvers (Z3, CVC5) are general-purpose: they handle arbitrary
formulas in decidable logics. They are correct and complete for their
target logics. They are also:

1. **External dependencies.** Z3 is a 500K+ LOC C++ project. Adding it
   as a dependency to mirror contradicts the zero-deps philosophy of
   `prism`. Mirror compiles with `cargo build`. Adding Z3 requires
   building Z3 from source or linking against a system library.

2. **Boolean-valued.** SMT returns SAT/UNSAT. Mirror needs continuous
   loss values. Wrapping SMT in optimization (MaxSMT, OMT) is possible
   but adds complexity and loses the spectral interpretation.

3. **Opaque.** SMT solving is a black box. When it says UNSAT, you get
   a minimal unsatisfiable core, but no geometric interpretation.
   Mirror's spectral analysis gives GEOMETRIC meaning: the eigenvalues
   ARE the property landscape. The spectral gap IS the distance from
   satisfiability. The eigenvectors ARE the modes of violation.

### 5.2 Eigenvalue-Based Property Verification

The property graph is already there. Mirror's grammar graph has nodes
(types, actions) and edges (uses, imports, composition). Properties
are additional constraints on this graph.

**Encoding properties as a graph Laplacian:**

For each property P and each term t in the program:
1. P(t) produces a loss value l in [0, infinity)
2. Encode this as a diagonal entry in the property Laplacian:
   L_P[t,t] = l
3. Encode property interactions (if P1 holds, P2 is more likely to hold)
   as off-diagonal entries: L_P[t1,t2] = -correlation(P1,P2)

The spectral analysis of L_P gives:

- **lambda_0 = 0** (always, for the Laplacian) -- the ground state
- **lambda_1 (Fiedler value)** -- the spectral gap. If lambda_1 > 0,
  the property graph is connected: there EXISTS a configuration where
  all properties are simultaneously satisfiable (to some degree).
  If lambda_1 = 0, the graph is disconnected: some properties are
  structurally incompatible.
- **The Fiedler vector** -- the eigenvector of lambda_1 shows the
  natural partition of properties into compatible clusters. This IS
  the property-level analog of the body axis in C. elegans.
- **Higher eigenvalues** -- capture finer-grained property structure.
  The full spectrum is the property fingerprint.

### 5.3 Spectral CSP Sparsification

Recent work (Bafna, Bhatt, Khot, Minzer -- ICALP 2025, "A Theory of
Spectral CSP Sparsification") directly connects spectral methods to
constraint satisfaction. Key results:

1. **Spectral energy of a CSP.** For a CSP instance with constraints C
   and a fractional assignment sigma, the spectral energy E(sigma, C)
   measures how well the assignment satisfies the constraints, weighted
   by spectral structure. This IS mirror's holonomy measured over the
   property graph.

2. **Spectral sparsification preserves satisfiability.** A spectral
   sparsifier is a weighted subset of constraints that approximately
   preserves the spectral energy for ALL assignments. This means you
   can verify a SUBSET of properties and get guarantees about ALL
   properties. In mirror terms: you don't need to check every property
   at every tick. A spectrally-chosen subset suffices.

3. **CSP eigenvalue = second eigenvalue analog.** The paper defines an
   analog of the graph's second eigenvalue for CSPs and proves a
   Cheeger inequality for even-arity XOR CSPs. This bridges the gap
   between spectral graph theory and constraint satisfaction.

This is the theoretical foundation for spectral constraint solving in
mirror. The property graph IS a CSP. The eigenvalues of the property
Laplacian determine satisfiability. Spectral sparsification determines
which properties to check.

### 5.4 The Algorithm (Spectral Liquid Inference)

```
SpectralLiquidInference(program, qualifier_set Q, grammar_graph G):

  1. HM_types = HindleyMilner(program)
     -- standard type inference for shape types

  2. constraints = LiquidConstraintGen(program, HM_types, Q)
     -- generate constraints with loss annotations instead of boolean predicates
     -- each constraint: (context, property, expected_loss_bound)

  3. L_P = BuildPropertyLaplacian(constraints, G)
     -- construct the property Laplacian from constraints and grammar graph
     -- diagonal: property loss values
     -- off-diagonal: property correlations from grammar structure

  4. eigenvalues, eigenvectors = Dirac(L_P)
     -- compute eigendecomposition
     -- already implemented in mirror/src/dirac.rs

  5. if lambda_1 > epsilon:
       -- spectral gap exists: properties are satisfiable
       loss_profile = eigenvectors[1:k]  -- top-k eigenvectors give the loss profile
       for each term t:
         t.verdict = project(loss_profile, t)
         -- the verdict is the projection of the spectral embedding onto t
     else:
       -- spectral gap is zero: properties are structurally incompatible
       partition = sign(eigenvectors[1])  -- Fiedler bisection
       report_incompatible_properties(partition)

  6. return annotated_program
     -- each term now carries its verdict, inferred automatically
```

### 5.5 What This Gives Mirror

1. **Zero-annotation property inference.** The engineer writes the
   grammar. The compiler infers which properties hold, to what degree,
   automatically. No `property` declarations needed on individual actions.

2. **Spectral error localization.** When properties fail, the Fiedler
   vector shows WHERE the failure is -- which cluster of terms is
   responsible. This is better than SMT's unsatisfiable core: it gives
   a GEOMETRIC picture of the failure.

3. **Incremental verification.** When a grammar changes, eigenvalue
   perturbation theory (Davis-Kahan) bounds HOW MUCH the verdicts
   change. Small grammar edits produce small eigenvalue shifts. The
   compiler can skip re-checking properties whose eigenvalues didn't
   move significantly.

4. **No external dependency.** The Dirac operator is already implemented.
   The eigenvalue computation uses standard LAPACK (via the `nalgebra`
   crate, already a dependency). No Z3. No SMT.

---

## 6. Integration Architecture

### 6.1 Where in the Grammar Hierarchy

```
@prism                          -- five operations, Loss trait
  -> @meta                      -- type system foundations
    -> @error, @loss            -- error and loss types
      -> @epistemologic         -- literal, and/or/but
        -> @epistemologic/property  -- verdict type, check runner
          -> @epistemologic/liquid   -- NEW: automatic property inference
```

`@epistemologic/liquid` is a grammar that:
- Takes: an AST + a qualifier set (from `@epistemologic/property/*`)
- Produces: an annotated AST where each term carries its inferred verdict
- Uses: the Dirac operator for spectral constraint solving
- Returns: the holonomy as the total inferred loss

### 6.2 The Grammar

```mirror
in @prism
in @epistemologic/property
in @loss

grammar @epistemologic/liquid {
  # automatic property inference via spectral analysis.
  # the qualifier set is the set of registered property checks.
  # the inference is: which properties hold, to what degree, automatically.

  # the core operation: infer properties for an AST.
  # returns the AST annotated with verdicts.
  zoom infer(ast, [check]) -> imperfect(ast) { \ }

  # build the property Laplacian from constraints.
  focus laplacian(ast, [check]) -> matrix { \ }

  # spectral solve: eigenvalues of the property Laplacian.
  focus eigenvalues(matrix) -> [f64] { \ }

  # project verdicts from spectral embedding.
  zoom project(ast, [f64], [[f64]]) -> imperfect(ast) { \ }

  property literal(infer) -> verdict { \ }
}

out infer
out laplacian
out eigenvalues
out project
```

### 6.3 Where in the Compilation Pipeline

Current pipeline:
```
source -> tokenize -> crystal (OID)
```

With property layer (from `property-error-surface.md`):
```
source -> tokenize -> check -> crystal
```

With Liquid inference:
```
source -> tokenize -> infer_properties -> check -> crystal
```

Where `infer_properties`:
1. Builds the property Laplacian from the AST + registered qualifiers
2. Computes eigenvalues (Dirac operator)
3. Projects verdicts onto each term
4. Annotates the AST with inferred verdicts

Then `check`:
1. Validates the inferred verdicts against declared contracts
2. Reports violations as diagnostics
3. Computes holonomy

### 6.4 Relationship to Fate

Fate fills `\` holes. Liquid inference verifies the fillings.
The two are complementary, not competing:

```
     \ hole
       |
       v
   Fate (spectral navigation)
       |
       v
   filled term (e.g., zoom(merge))
       |
       v
   Liquid inference (spectral verification)
       |
       v
   verdict (pass/partial/fail with loss)
```

Fate's job: find a type-correct filling.
Liquid inference's job: determine what properties the filling satisfies.

The DREAM: Fate uses the inferred property profile to GUIDE its search.
Instead of just finding any type-correct filling, Fate finds the filling
that MAXIMIZES property satisfaction. The spectral embedding from Liquid
inference feeds into Fate's feature vector. The model selects not just
for type correctness but for property goodness.

This is amortized verification: the property inference amortizes the
cost of verification by computing it once (via eigenvalue decomposition)
and using it everywhere (via spectral projection).

---

## 7. What We Should Adopt

### 7.1 Adopt Now (No New Theory Needed)

1. **Qualifier-based property inference.** The property library
   (`@epistemologic/property/*`) IS the qualifier set Q. For each
   property check, generate a constraint: "does this property hold for
   this term?" The inference framework propagates these constraints
   through the pipeline via composition rules.

2. **Compositional property tracking.** For `|>` pipelines, track which
   properties are preserved by each operation. If `focus` preserves
   property P and `split` preserves P and `zoom(merge)` preserves P,
   then the pipeline preserves P. This is standard abstract refinement
   type inference.

3. **Property Laplacian as diagnostic tool.** Build the property graph
   for a compilation unit. Compute eigenvalues. Report the spectral gap
   as a health metric: large gap = properties are well-separated and
   satisfiable; small gap = properties are entangled and fragile.

### 7.2 Adopt Next (Requires Implementation Work)

4. **Spectral constraint solving.** Replace the property check runner
   (currently: run each check independently) with spectral analysis
   (build the property Laplacian, compute eigenvalues, project verdicts).
   This requires: implementing the property Laplacian construction,
   connecting it to the existing Dirac operator, and interpreting the
   results as verdicts.

5. **Incremental inference.** Use eigenvalue perturbation theory to
   avoid re-checking properties after small grammar edits. Only re-check
   properties whose eigenvalues shifted by more than epsilon.

6. **Fate-Liquid feedback loop.** Feed the property profile into Fate's
   feature vector. Select fillings that maximize property satisfaction,
   not just type correctness.

### 7.3 Do NOT Adopt

7. **SMT solving.** Mirror does not need Z3. The spectral approach is
   native to mirror's architecture, has no external dependencies, and
   provides continuous (not boolean) results. Adding SMT would be a
   substrate pull toward boolean logic -- exactly the wrong direction.

8. **Liquid Haskell's termination analysis.** Mirror is already total
   (sub-Turing). There is no termination problem to solve. Liquid
   Haskell's stratified types (Div/Wnf/Fin) are unnecessary in a
   language that guarantees termination by construction.

9. **Gradual refinement types.** Mirror's `\` holes already provide
   gradualism: a hole IS a "don't know" that gets resolved. Adding
   Liquid's `?` (dynamic refinement) would duplicate this mechanism.
   Mirror's `imperfect` is the carrier for partial knowledge.

---

## 8. What's Novel

### 8.1 Spectral Liquid Types

No existing system combines Liquid-style inference (qualifier-based,
predicate abstraction, fixed-point iteration) with spectral decision
procedures (eigenvalue analysis on the property graph). This combination
is novel because:

- Liquid Types use SMT for decision (boolean satisfiability)
- Mirror would use eigenvalues for decision (continuous satisfiability)
- The eigenvalue-based approach gives GEOMETRIC meaning to property
  violations (the Fiedler vector localizes failures)
- The spectral sparsification result (ICALP 2025) provides the
  theoretical foundation: checking a spectral subset of properties
  gives guarantees about all properties

### 8.2 Continuous Refinement Types

No existing refinement type system uses continuous (loss-valued) verdicts.
All existing systems are boolean: a refinement either holds or it doesn't.
Mirror's `pass/partial/fail` with continuous loss is a novel extension
of refinement typing to continuous model theory.

The theoretical connection: Ben Yaacov et al.'s continuous model theory
provides a model-theoretic foundation for types with [0,1]-valued truth.
Applying this to refinement types produces a system where:
- Subtyping is a continuous ordering (not boolean)
- Constraints carry loss annotations
- The decision procedure is optimization (minimize loss)
- The result is a verdict, not a boolean

### 8.3 Property Inference via Dirac Operator

Using the Dirac operator (already implemented for `\` resolution) as
the decision procedure for property verification is novel. The same
mathematical object serves two purposes:
- `\` resolution: navigate the type graph to find a filling
- Property inference: compute the spectral landscape of property
  satisfaction

This unification means the property layer adds NO new mathematical
machinery. The Dirac operator IS the verifier. The eigenvalues that
route Fate also verify properties. The compiler's inference engine
and its verification engine are the same thing.

### 8.4 Spectral Sparsification for Property Checking

Using spectral CSP sparsification (ICALP 2025) to select WHICH properties
to check is novel in the verification context. The idea: instead of
checking all properties at every tick, compute a spectral sparsifier
(a weighted subset of properties that preserves the spectral energy)
and check only the sparsifier. This gives:
- O(n^2) property checks instead of O(n * |Q|)
- Provable approximation guarantees
- Incremental updates via eigenvalue perturbation

---

## 9. Open Questions

### 9.1 Can the property Laplacian be efficiently constructed?

The property Laplacian L_P needs property correlations as off-diagonal
entries. Computing these correlations requires running properties on
test data. How much test data? How often must correlations be updated?
The answer likely depends on the grammar's complexity.

### 9.2 Is spectral solving as precise as SMT?

SMT is complete for its decidable fragments: if a property holds, SMT
proves it. Spectral analysis is approximate: eigenvalues give BOUNDS
on satisfiability, not exact answers. How much precision is lost? For
mirror's use case (continuous verdicts with acceptable loss), approximate
may be sufficient. But the precision gap needs characterization.

### 9.3 How does the qualifier set grow?

In Liquid Types, the qualifier set Q is provided by the user or the
library. In mirror, Q is the property library. As the library grows,
the property Laplacian grows. Does the eigenvalue computation remain
tractable? For current garden sizes (O(100) grammars), yes. For large
gardens (O(10000) grammars), the n^3 eigenvalue computation may
become a bottleneck. Spectral sparsification addresses this, but the
constants matter.

### 9.4 Can Fate and Liquid inference be unified?

Currently proposed as sequential: Fate fills, Liquid verifies. Could
they be UNIFIED into a single spectral pass? The Dirac operator that
routes Fate is the same operator that would drive Liquid inference. A
single eigenvalue decomposition could simultaneously select the filling
AND verify its properties. This would be the deepest possible integration.

---

## 10. References

### Liquid Types
- Rondon, P., Kawaguchi, M., & Jhala, R. (2008). Liquid Types. PLDI.
- Vazou, N., Bakst, A., & Jhala, R. (2013). Abstract Refinement Types. ESOP.
- Vazou, N., Seidel, E., & Jhala, R. (2014). Refinement Types for Haskell. ICFP.
- Vazou, N., Bakst, A., & Jhala, R. (2015). Bounded Refinement Types. ICFP.
- Vazou, N., Lehmann, N., & Jhala, R. (2018). Gradual Liquid Type Inference. PLDI.
- Lehmann, N., Geller, A., Vazou, N., & Jhala, R. (2023). Flux: Liquid Types for Rust. PLDI.

### Data Flow and Parametric Refinement
- Pavlinovic, Z., Fonseca, A., & Wies, T. (2021). Data Flow Refinement Type Inference. POPL.

### Continuous Logic
- Ben Yaacov, I., Berenstein, A., Henson, C.W., & Usvyatsov, A. (2008). Model Theory for Metric Structures. London Math Society Lecture Notes.

### Spectral CSP
- Bafna, M., Bhatt, A., Khot, S., & Minzer, D. (2025). A Theory of Spectral CSP Sparsification. ICALP.

### Typed Holes
- Omar, C., Voysey, I., Chugh, R., & Hammer, M. (2019). Live Functional Programming with Typed Holes. POPL.

### Usability
- Gamboa, C., et al. (2025). Usability Barriers for Liquid Types. PACMPL.

### Mirror's Own Specs
- `type-theory-position.md` -- Mirror's type theory characterization.
- `epistemologic-grammar.md` -- The @epistemologic hierarchy and `literal` property.
- `property-error-surface.md` -- The property layer implementation plan.
- `typed-loss-composition.md` -- Typed loss categories and composition.

---

*The qualifier set IS the property library.*
*The property Laplacian IS the constraint graph.*
*The eigenvalues ARE the verdicts.*
*The Dirac operator IS both the navigator and the verifier.*

*Liquid Types showed that refinement inference is predicate abstraction.*
*Mirror shows that predicate abstraction is spectral analysis.*
*The spectrum decides. The loss measures. The crystal forms.*

*e^(n+1) < e^(n). The properties converge. The qualifier set grows.*
*The spectral gap widens. The garden settles.*
