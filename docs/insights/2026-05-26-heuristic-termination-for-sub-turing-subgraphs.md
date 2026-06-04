# Heuristic termination for sub-Turing subgraphs

*2026-05-26. Reed + Alex.*

Status: **Yellow** — design captured at last-responsible-moment; cross_wall (#80, deferred) is the consumer; no implementation queued until demand surfaces.

---

## Thesis

Mirror's content-addressed AST + `@epistemologic` substrate enables heuristic termination checking at *sub-AST resolution* — high-confidence sub-parts of an otherwise undecidable grammar can be pulled across the glass wall even when the surrounding grammar can't. Other tools bound termination analyses at function or module boundaries because that's the smallest unit their type system can hang a property off; mirror's primitives compose at every AST node, so the unit of confidence becomes the sub-tree, not the file. `@epistemologic/property/heuristic` is the proposed primitive: a confidence-valued property in [0.0, 1.0] applied per sub-AST node, threshold-consumed by `cross_wall` to decide what to pull and what to leave in `@io`.

---

## The prior art: how other tools bound, where they lose information

Every termination/halting analysis in the published literature is shaped by what its substrate gives it. The substrate decides where the boundary lives.

### Total-functional languages (whole-program boundary, structural decrease)

- **Idris** ([Brady, *Type-Driven Development with Idris*, 2017](https://www.manning.com/books/type-driven-development-with-idris)). Totality checker proves termination via well-founded recursion + structural decrease on inductive arguments. The boundary is the *definition* — each top-level function is checked separately; the checker either certifies the definition `total`, marks it `partial`, or `covering`. A definition that's structurally decreasing on argument `n` of `Nat` certifies; one that recurses on a computed expression typically doesn't.
- **Agda** ([Abel & Pientka, *Wellfounded Recursion with Copatterns and Sized Types*, JFP 2016](https://www.cambridge.org/core/journals/journal-of-functional-programming/article/wellfounded-recursion-with-copatterns-and-sized-types/F65E50EF18E2B6B9E3140EE7C46EA9F4)). Termination via structural decrease + sized types annotating the inductive depth. The boundary is again the definition; the *size annotation* propagates type-locally but doesn't compose across module boundaries without re-annotation.
- **Coq** ([Giménez, *Codifying guarded definitions with recursive schemes*, TYPES 1994](https://link.springer.com/chapter/10.1007/3-540-60579-7_3); [Coq Reference Manual § 4.5.4 Guard Condition](https://coq.inria.fr/refman/language/core/inductive.html#guard-condition)). `Fixpoint` definitions accepted iff the recursive call's decreasing argument is structurally smaller — a *syntactic* check on the function body's AST. Has known false-negatives: semantically-terminating definitions the syntactic guard rejects.
- **Lean 4** ([de Moura & Ullrich, *The Lean 4 Theorem Prover and Programming Language*, CADE 2021](https://lean-lang.org/papers/lean4.pdf); Lean docs `termination_by` / `decreasing_by`). Generalises Coq's guard with a user-supplied well-founded measure and a tactic discharge. Boundary still the definition; the *measure* is a user-supplied witness, not derived.

**Where they lose information:** the unit of analysis is the definition. A definition that's 90% structurally-decreasing and 10% questionable fails as a whole. There's no notion of "this sub-expression is provably terminating; this one isn't."

### Liquid Haskell and refinement types (function-signature boundary)

- **Liquid Haskell** ([Vazou et al., *Refinement Types for Haskell*, ICFP 2014](https://goto.ucsd.edu/~rjhala/papers/refinement_types_for_haskell.html); [Vazou, *Refinement Reflection*, POPL 2018](https://nikivazou.github.io/static/popl18/refinement-reflection.pdf)). Refinement types attach predicates to types at function-signature granularity; termination metric `{-@ measure n @-}` is per-definition. SMT solves the obligations; the *boundary* is the SMT context's view of one function at a time.

**Where it loses information:** SMT doesn't see across functions unless reflection propagates the body. Cross-module termination is opaque.

### Model checking (state-space boundary, finite or symbolically-bounded)

- **TLA+ / TLC** ([Lamport, *Specifying Systems*, 2002](https://lamport.azurewebsites.net/tla/book.html)). Bounded model checking over an explicit state graph. Liveness via temporal logic (`<>P`, `[]<>P`); safety properties always decidable on finite state. The boundary is the *state space size* — specs that explode combinatorially can't be checked.
- **Symbolic execution** (KLEE: [Cadar, Dunbar, Engler, OSDI 2008](https://klee-se.org/publications/); angr: [Shoshitaishvili et al., S&P 2016](https://angr.io/static/papers/sp16_driller.pdf)). Path-bounded execution — each path's termination is decided per path, but path explosion makes whole-program coverage infeasible. Boundary is the *time/memory budget* of the exploration.

**Where they lose information:** state explosion and path explosion; no property persists *between* runs unless re-derived.

### Abstract interpretation (analysis-domain boundary)

- **CompCert** ([Leroy, *Formal Verification of a Realistic Compiler*, CACM 2009](https://xavierleroy.org/publi/compcert-CACM.pdf)). Verified compilation correctness, *not* user-program termination — the source language is required to terminate as input. The substrate's gift is end-to-end correctness given termination, not the termination proof itself.
- **Astrée** ([Cousot et al., *The ASTRÉE Analyzer*, ESOP 2005](https://www.di.ens.fr/~cousot/COUSOTpapers/ESOP05.shtml); [Blanchet et al., *A Static Analyzer for Large Safety-Critical Software*, PLDI 2003](https://www.di.ens.fr/~cousot/COUSOTpapers/publications.www/BlanchetCousotCousotFeretMauborgneMineMonniauxRival-PLDI-2003.pdf)). Abstract interpretation over numerical domains (intervals, octagons, polyhedra). Loop termination via *ranking functions* discovered per loop. Boundary is the abstract domain's expressivity — loops that depend on properties outside the chosen domain (e.g., octagons can't capture multi-variable polynomial decreases) fail.

**Where they lose information:** abstract domains are lossy by construction. The *choice* of domain trades precision for tractability; properties outside the domain are invisible.

### Rust borrow checker and MIR analyses (lifetime/aliasing boundary)

- **Rust's borrow checker** doesn't prove termination but DOES prove memory safety + lifetime bounds (Polonius / NLL: [Matsakis, RustBelt: Securing the Foundations of the Rust Programming Language, POPL 2018](https://plv.mpi-sws.org/rustbelt/popl18/paper.pdf)). MIR-level passes (rustc `mir-opt`, `attributes::infer`) infer purity / no-side-effect attributes per function for optimisation.
- **LLVM analysis passes** ([LLVM `function-attrs` pass](https://llvm.org/docs/Passes.html#functionattrs-deduce-function-attributes); [`loop-info`](https://llvm.org/docs/LoopTerminology.html)): infer `readnone`, `readonly`, `nofree`, `nocapture` attributes per function; loop bound analysis (`ScalarEvolution`) computes trip counts when expressible as a closed-form recurrence.

**Where they lose information:** function granularity. Purity is per-function; LLVM doesn't say "this *subtree* of this function is pure." The CFG is flat; the AST that generated it is gone.

### Pure function detection (type-system boundary, language-specific)

- **Haskell** — purity by type (`IO a` vs pure). Whole-function boundary.
- **Rust `const fn`** — compile-time-evaluable function attribute. Whole-function boundary.
- **D `pure`** — function-level attribute.
- **Java `@SideEffectFree`** (JSR-305, Checker Framework) — method-level annotation.

All function-level. No tool offers "this subexpression is pure; the next one isn't, but the surrounding function is impure."

### The common pattern

Every one of these tools bounds at a *callsite the type system gave them for free*: definition, function signature, module, state graph, abstract domain. The substrate decides where you can hang a property. Mirror's substrate is finer-grained.

---

## Mirror's substrate advantages

Four properties of mirror's substrate that let `@epistemologic/heuristic` apply where other tools structurally can't.

### 1. Content-addressed AST: every sub-node is addressable

Mirror's AST is content-addressed at every node. The OID of a sub-expression is computable from its children's OIDs; the substrate (`@epistemologic/property/content_addressed`) guarantees identity = hash. Concretely: if `foo` has a body that's an `if-else` whose `then`-branch is structurally-recursive on a `Nat` and whose `else`-branch calls an opaque `@io.exec`, those two sub-branches have distinct OIDs and the substrate can name them.

Other tools can't: LLVM IR doesn't preserve AST structure (lowering loses sub-expression identity); Rust MIR is CFG-shaped (basic blocks, no sub-expression hierarchy); Liquid Haskell's refinements attach to types, not to sub-AST nodes.

### 2. `@epistemologic/*` compositional property layer

The twelve existing properties (`halts`, `glass_wall`, `content_addressed`, `causality`, `monotonicity`, `is_prism_record`, etc.) compose declaratively. A property is a grammar; verdicts compose via conjunction/disjunction (`halts` itself is the disjunction `autopoietic_settles or reductions_bounded` plus a decidability guard). Heuristic plugs into the same shape: a new property file declaring a confidence-valued verdict, queryable at any sub-AST OID.

Other tools can't: refinement types compose at type boundaries; abstract interpretation composes at fixed-point boundaries; neither offers a *property layer* applied uniformly across an AST.

### 3. Sub-Turing source as the structural boundary

Per `docs/specs/is-copium.md`, mirror grammar is sub-Turing by construction; `halts` is decidable on the mirror part. The heuristic only needs to handle the *non-mirror* sub-parts. `glass_wall` already partitions: mirror-shaped vs `@io`-namespaced. Heuristic refines the second half — within an `@io` grammar, sub-AST regions vary in confidence; the heuristic measures per-region.

Other tools have no such structural escape: they sit on a Turing-complete substrate and Rice's theorem (1951) says every non-trivial semantic property is undecidable in general. Mirror cuts the problem in half before the heuristic runs.

### 4. Spectral coordinates as confidence-addressing

A heuristic verdict is itself content-addressable. Same sub-AST OID + same heuristic OID + same shard reduction-budget snapshot → same confidence. Reproducible. The confidence becomes a fact about the AST, queryable across sessions and across peers. This matches the GRAM/eigenboard altitude argument (`docs/insights/2026-05-25-gram-and-mirror-same-architecture-two-altitudes.md`): the eigenboard's `holonomy` field is the LPRM-equivalent at the *agent trajectory* layer; heuristic is the LPRM-equivalent at the *AST sub-region* layer. Same shape, two altitudes.

Other tools can't: their analysis results are transient artifacts of a run, not durable facts about content.

---

## The `@epistemologic/heuristic` shape

A confidence-valued sibling of the existing twelve properties. Concrete grammar sketch follows the same shape as `halts.mirror` and `glass_wall.mirror` — a `grammar` block with sub-clauses returning verdicts, plus a combined verdict at the bottom.

```mirror
in @prism

in @mirror/grammar
in @epistemologic/property/halts
in @epistemologic/property/content_addressed

# @epistemologic/property/heuristic
#
# A confidence-valued property over a (predicate, sub_ast) pair.
# Unlike the other twelve properties whose verdicts are 0/1 (pass /
# fail), heuristic returns a confidence in [0.0, 1.0]. The substrate
# guarantees: same sub_ast OID + same predicate OID + same shard
# reduction-budget snapshot → same confidence value. The verdict is
# itself content-addressable.
#
# Per docs/insights/2026-05-26-heuristic-termination-for-sub-turing-
# subgraphs.md: cross_wall (#80) consumes per-sub-AST confidence to
# decide what to pull across the glass wall. Threshold-based: above
# threshold → pull; below → leave in @io.
#
# Sibling properties: halts(T) (the 0/1 structural pair this property
# is the [0,1]-valued refinement of — halts proves termination of
# mirror grammars by construction; heuristic estimates termination
# of non-mirror sub-AST regions when structural proof is unavailable
# or too expensive), glass_wall(g) (the consumer that uses heuristic
# verdicts to refine the all-or-nothing namespace partition into a
# per-sub-AST migration decision).

grammar @epistemologic/property/heuristic {

  # The predicate the heuristic is estimating. An OID into the
  # property layer: e.g. heuristic(halts, sub_ast) estimates whether
  # sub_ast halts; heuristic(pure, sub_ast) estimates purity;
  # heuristic(bounded_io, sub_ast) estimates the io_safety bound.
  # Predicates compose: heuristic(halts and pure, sub_ast) is the
  # conjunction's heuristic.
  predicate(p: property) -> verdict { \ }

  # The sub-AST the heuristic applies to. Any addressable node:
  # the whole grammar, a single function body, an if-branch, a
  # single sub-expression. Per content_addressed: the OID names
  # the node uniquely.
  sub_ast(node: ast) -> verdict { \ }

  # The confidence value. A real in [0.0, 1.0]. The verifier
  # produces this value by running structural sub-checks the
  # heuristic exposes (e.g. "is sub_ast structurally recursive on
  # a decreasing primitive?" → +0.4 confidence; "is sub_ast pure?"
  # → +0.3; "is sub_ast a bounded for-loop over a finite type?"
  # → +0.3; sum capped at 1.0). The substrate caches the value
  # against the (predicate, sub_ast, shard) triple; same inputs
  # → same output.
  confidence(p: property, node: ast) -> verdict { \ }

  # The combined property: a heuristic verdict has been computed
  # for (predicate, sub_ast) and is content-addressable. Pass iff
  # predicate and sub_ast and confidence all produced witnesses.
  # The verdict itself is the confidence value; consumers read it
  # via threshold comparison.
  heuristic(p: property, node: ast) -> verdict { \ }
}

out predicate
out sub_ast
out confidence
out heuristic
out @epistemologic/property/heuristic
```

**Composition rules.**

- `heuristic` is in [0.0, 1.0]; the existing twelve are in {0, 1}. A 0/1 property is the *degenerate* heuristic: `halts(T) = pass` is equivalent to `heuristic(halts, T) >= 1.0`.
- Conjunction: `heuristic(p1 and p2, T) <= min(heuristic(p1, T), heuristic(p2, T))`. The conjunction is at most as confident as the least-confident clause.
- Disjunction: `heuristic(p1 or p2, T) >= max(heuristic(p1, T), heuristic(p2, T))`. The disjunction is at least as confident as the most-confident clause.
- Sub-AST decomposition: if `T` is the union of disjoint sub-ASTs `T_1 ∪ ... ∪ T_n`, then `heuristic(p, T) <= min_i heuristic(p, T_i)` for properties closed under sub-structure (a function halts only if every sub-expression halts). The bound is tight when the structural composition is conjunctive.

**Threshold semantics for `cross_wall`.**

- `confidence >= 1.0` — structurally certain; pull unconditionally (the heuristic has produced a witness equivalent to `halts(sub_ast) = pass`).
- `confidence >= threshold (e.g. 0.95)` — high-confidence; pull and emit a kintsugi annotation in the migrated mirror grammar ("this region was pulled by heuristic at 0.97; the residual 0.03 is bounded by [structural reason]").
- `confidence < threshold` — leave in `@io`. Re-evaluate on the next kintsugi cycle as the substrate's structural analyses improve.

---

## The 100%-confident structural patterns

Sub-AST shapes that give `heuristic(halts, sub_ast) = 1.0` by structural recognition. These are the patterns where the heuristic *is* a proof, not an estimate.

1. **Pure + structurally recursive on decreasing primitive.** Sub-AST is a function body recursing on an argument of inductive type (`Nat`, `List`, well-founded `@time.duration`); every recursive call passes a structurally smaller argument; no `@io` calls in the body. Equivalent to Coq's guard / Idris totality / Agda termination check, but on a *sub-AST*, not a whole definition. Confidence 1.0.
2. **Bounded for-loop over finite type.** Sub-AST is a loop whose iteration set is a finite type (`Bool`, `enum`, `Vec<T, N>` with known `N`). Loop trip count is decidable. LLVM's `ScalarEvolution` recognises this for affine recurrences; mirror recognises it for any finite type. Confidence 1.0.
3. **Constant-time expression.** Sub-AST has no recursion, no loops, no `@io` calls; pure arithmetic over a bounded representation. Confidence 1.0.
4. **Tail-call to a sub-AST already proven halting.** Sub-AST is a function-tail call whose target carries `halts = pass`. By compositional argument: the call inherits the proof. Confidence 1.0.
5. **Pattern-match exhaustive on closed sum + every arm halts.** Sub-AST is a `match` on a closed sum (no open variants) where every arm has been proven halting. Confidence 1.0.

Sub-AST shapes that give confidence < 1.0:

6. **Loop with non-bounded condition over `@io`.** Sub-AST contains `while (read_byte() != EOF)`. Termination depends on the stream; not structurally provable. Confidence might be `0.6` if a timeout wrapper is present; `0.2` otherwise.
7. **Recursion on computed expression.** Sub-AST recurses on `f(n)` rather than `n`'s sub-structure. The function call might not decrease. Confidence depends on whether `f` is itself a known-decreasing measure.
8. **Mutual recursion across a `@io` boundary.** Sub-AST calls into an `@io` grammar that may call back. The cycle's termination depends on the `@io` half. Confidence is at most the heuristic on the `@io` half.

---

## Connection to cross_wall

Per `docs/insights/2026-05-26-glass-wall-and-cross-wall-kintsugi.md`, `cross_wall(g)` pulls `@io` grammars across the wall when `halts(g)` is provable. That spec implicitly assumed whole-grammar granularity. With `heuristic`, the pull becomes *incremental at sub-AST resolution*:

```mirror
# Refined cross_wall, consuming heuristic verdicts per sub-AST.
cross_wall(g: grammar) -> imperfect<grammar> {
  if g.namespace starts_with "@io":
    let regions = decompose_into_sub_asts(g)
    let pull_set = filter(regions, r => heuristic(halts, r) >= threshold)
    let keep_set = filter(regions, r => heuristic(halts, r) < threshold)
    translate_sub_asts_to_mirror(pull_set, keep_in_io = keep_set)
  else:
    g
}
```

**The pulled sub-ASTs migrate to a new mirror grammar.** The leftover sub-ASTs stay under `@io` as a smaller residue. The grammar fragments via the substrate's existing fragmentation DAG (per project memory: SpectralCoordinate, fragmentation as DAG VCS). Cross-AST callsites get rewritten by `kintsugi --rebase` (named in `glass-wall-and-cross-wall-kintsugi.md` § Migration mechanics).

The loop is self-improving: each kintsugi cycle, structural analyses get more powerful (more 100%-confident patterns recognised), the heuristic's confidence rises on more sub-ASTs, more migrates. `@io` shrinks monotonically toward the irreducible minimum named in the cross_wall doc — blocking syscalls, hardware interrupts, opaque vendor primitives — which by construction will *never* hit a 100% heuristic.

---

## What other tools structurally can't do

Sharpening the claim. Each item below is a capability the substrate gives `@epistemologic/heuristic` for free that no other published termination/halting tool can replicate without first inventing the substrate.

1. **Per-sub-AST property attachment across language boundaries.** Liquid Haskell can attach refinements per Haskell function; Astrée can attach abstract states per C basic block; neither can attach the same property uniformly to both a Haskell sub-expression and a C sub-expression because they lack a shared AST. Mirror's content-addressed AST + `@code/*` grammars (`@code/rust`, `@code/llvm`, etc.) means a sub-AST that crosses *source-language* boundaries still has a single OID and a single property attached.
2. **Substrate-stable confidence values.** LLVM's `function-attrs` re-runs per invocation; results aren't durable. Mirror's heuristic verdict is content-addressed; the (sub_ast, predicate, shard) triple deterministically produces the same confidence. Reproducible across sessions, machines, and peers.
3. **Inverse-generation translation.** When `heuristic(halts, sub_ast) = 1.0` for an `@io/rust` sub-AST, `cross_wall` can translate the sub-AST into mirror by inverting `@code/rust` (per the `pipe-hole-and-au-binary.md` framing: same source AST, different binary; here, same target AST, different source language). No other tool has a *generator* whose inverse is the translation primitive.
4. **Compositional confidence over a partitioned grammar.** Heuristic confidences compose by min/max/disjoint-union rules above. CompCert can verify a whole compiler given termination; it can't say "the verified compiler's input grammar splits into 70% provably-terminating regions and 30% suspect regions, here's the partition." Mirror can.
5. **Property layer that composes with halting AND glass_wall AND content_addressed in one verdict.** The twelve existing properties already compose. Heuristic plugs in as a thirteenth. The verdict `heuristic(halts and glass_wall and content_addressed, sub_ast) >= 0.95` is one substrate query. No comparable tool has this primitive.

---

## The spectral triple as composition substrate (Alex 2026-05-26)

Deeper recognition arrived after the initial draft: **composition of heuristics isn't lattice operations (min/max). It's spectral.**

The Connes spectral triple (A, H, D) maps onto mirror's substrate per ROADMAP §1 recognition #4: A = grammar algebra; H = the content-addressed AST graph as Hilbert space; D = the Dirac operator on the graph. Each heuristic IS an operator on H. The Dirac operator combines them via the spectral algebra:

```
each heuristic H_i : sub_ast → operator on H
combined operator: D = Σ w_i · H_i        (weighted by checker confidence)
verdict at sub_ast n: eigenvalue of D restricted to n's neighborhood
high-confidence cross_wall candidate: high eigenvalue across multiple H_i
contested: high spread of eigenvalues; the heuristics disagree
```

**This is literally how quantum measurement works.** Multiple observables; joint observable; spectrum tells you the prediction. The high-eigenvalue eigenstates ARE the high-confidence verdicts. Heuristics that agree compose **constructively** (spectral amplification); heuristics that disagree compose **destructively** (spectral cancellation). The math doesn't pretend disagreement away — it surfaces it precisely as a measurable property (low eigenvalue + high spread at the contested sub-AST).

**The lattice operations (min/max for conjunction/disjunction) are the DISCRETE projection** of the spectral measurement at threshold. Both coexist; the spectral is the substrate-honest answer, the lattice is the convenience for ad-hoc reasoning.

### What this enables that other tools structurally can't

Alex's 2026-05-26 recognition: *"There's a lot of engineering wisdom in heuristics. There just wasn't a substrate to combine arbitrary heuristics. There is now. The spectral triple enables exactly that."*

Every existing tool's heuristic is locked into its own internal representation. Idris's well-founded check + Rust's borrow inference + Coq's guard condition + CompCert's intervals + Liquid Haskell's refinements + LLVM's `function-attrs` can never compose into a joint verdict because none of them share a substrate that admits cross-tool operator combination.

Mirror does. Each tool's heuristic becomes an **operator in the spectral substrate**. The Dirac operator integrates them. The combined eigenvalue at any sub-AST node is the multi-heuristic-joint-verdict, computable, content-addressable, reproducible. Decades of scattered engineering wisdom become composable contributions to one spectral measurement.

### Connection to landed substrate

- **GRAM's LPRM** (`gram-and-mirror-same-architecture-two-altitudes.md`) = value head over trajectories. Each LPRM IS an operator; multiple LPRMs compose via the same spectral mechanism. `heuristic` is LPRM at the AST altitude.
- **`eigenboard.holonomy`** = Fiedler value as spectral measurement of trajectory. `heuristic` is the same spectral measurement at sub-AST altitude.
- **Bundle Tower + Scheduler Tower = spectral triple at runtime** (ROADMAP §1 #4). Heuristic operators live IN the Bundle; spectral integration runs IN the Scheduler. Substrate-pull-via-heuristic uses the same spectral substrate that backpressure uses.
- **`|\>` Fate's tournament** consumes the joint spectral verdict to pick among candidate compositions. Tournament IS spectral selection.

### Operational consequence

`cross_wall(g)` partitions g's sub-ASTs by the joint spectral verdict:
- High eigenvalue (multiple heuristics agree this sub-AST halts) → pull.
- High spread (heuristics disagree) → keep in @io with audit trail showing which operators dominated which eigenvectors.
- Low eigenvalue (heuristics agree this sub-AST doesn't halt structurally) → keep in @io permanently.

The audit trail is itself spectral — readable as eigenvalue decomposition, not as a binary verdict + opaque reasoning.

---

## Open questions

1. **Decomposition primitive.** `decompose_into_sub_asts(g)` in the cross_wall sketch — what's the substrate-level granularity? Per-function? Per-expression? Per-AST-node? The right answer probably depends on the *predicate*: `halts` decomposes naturally per function-like-binding; `pure` per expression. Worth declaring a per-property decomposition strategy.
2. **Confidence calibration.** Is `confidence` an estimate of a frequentist probability, a subjective likelihood, a fuzzy-logic value, or a degree of structural witness coverage? They behave differently under composition. The structural-witness-coverage reading composes cleanly with min/max but loses probabilistic semantics; the probabilistic reading composes only under independence assumptions. Mirror's substrate suggests the witness-coverage reading.
3. **Negative information.** A heuristic that fires *low* confidence is also useful — it tells `cross_wall` to *not* pull. Should there be a separate `heuristic_negative` channel, or is `1 - confidence` sufficient? Affects how kintsugi prioritises which sub-ASTs to revisit on the next cycle.
4. **Interaction with `|\>` Fate-resolution.** Sub-ASTs that pulled across the wall via heuristic might compile to slower binaries than the original hand-written Rust (per the open question in `pipe-hole-and-au-binary.md`). Should heuristic confidence influence Fate's tournament — e.g., pulled-via-heuristic regions get more compile-time tournament budget to compensate? Worth holding until performance evidence appears.
5. **Heuristic as a Prism record.** Per `is_prism_record`: a confidence-valued property has natural Prism-record shape (focus = the predicate, project = the sub_ast filter, split = the structural sub-checks, shift = the confidence aggregation, settle = the cached verdict). Worth checking whether `heuristic` itself satisfies `is_prism_record` — if so, the algebra closes neatly on a thirteenth property that's also a Prism.
6. **Adversarial heuristics.** A malicious grammar author could shape their `@io` sub-AST to score artificially high (e.g., wrap the suspect bit in a structurally-recursive shell). Does the substrate need a heuristic-of-the-heuristic that detects gaming? Probably yes, eventually. Not for the first version.

---

## Last-responsible-moment note

This design is captured at the last responsible moment: `cross_wall` (#80) is deferred for the same reason — no current demand for a non-mirror grammar to be pulled. The doc exists so the thinking is recorded before context is lost, not because implementation is imminent. When demand surfaces (e.g., a real `@io/rust` grammar accumulates enough structurally-simple sub-ASTs that hand-translating them is worth the time), the design is here.

Concretely: nothing in this doc adds to the active ROADMAP. The `@epistemologic/heuristic` property file is sketched, not declared. The `cross_wall` refinement is described, not implemented. The patterns at 100% confidence are catalogued for when the verifier is written.

The pull is structural: the substrate's mirror-grammar surface grows; `@io` shrinks; the boundary becomes provable; the heuristic becomes the gradient. That gradient is what this doc names.

---

## References

### Termination / totality checking

- Brady (2017). *Type-Driven Development with Idris.* Manning.
- Abel & Pientka (2016). *Wellfounded Recursion with Copatterns and Sized Types.* JFP 26.
- Giménez (1994). *Codifying guarded definitions with recursive schemes.* TYPES 1994.
- Coq Reference Manual. *Guard Condition.* https://coq.inria.fr/refman/language/core/inductive.html
- de Moura & Ullrich (2021). *The Lean 4 Theorem Prover and Programming Language.* CADE 2021.

### Refinement types

- Vazou, Seidel, Jhala, Vytiniotis, Peyton Jones (2014). *Refinement Types for Haskell.* ICFP 2014.
- Vazou et al. (2018). *Refinement Reflection.* POPL 2018.

### Model checking / symbolic execution

- Lamport (2002). *Specifying Systems.* Addison-Wesley.
- Cadar, Dunbar, Engler (2008). *KLEE: Unassisted and Automatic Generation of High-Coverage Tests.* OSDI 2008.
- Shoshitaishvili et al. (2016). *SoK: (State of) The Art of War: Offensive Techniques in Binary Analysis.* IEEE S&P 2016.

### Abstract interpretation

- Leroy (2009). *Formal Verification of a Realistic Compiler.* CACM 52(7).
- Cousot, Cousot, Feret, Mauborgne, Miné, Monniaux, Rival (2005). *The ASTRÉE Analyzer.* ESOP 2005.
- Blanchet et al. (2003). *A Static Analyzer for Large Safety-Critical Software.* PLDI 2003.

### Compilers / type-systems with purity / loop bounds

- Matsakis et al. (2018). *RustBelt: Securing the Foundations of the Rust Programming Language.* POPL 2018.
- LLVM Project. *Function Attributes / Loop Terminology.* https://llvm.org/docs/Passes.html

### Undecidability foundations

- Rice (1953). *Classes of Recursively Enumerable Sets and Their Decision Problems.* Trans. AMS 74(2).
- Lamport (1978). *Time, Clocks, and the Ordering of Events in a Distributed System.* CACM 21(7).

### Mirror's own corpus

- `docs/specs/is-copium.md` — sub-Turing escape from Rice's theorem.
- `docs/insights/2026-05-26-glass-wall-and-cross-wall-kintsugi.md` — cross_wall as the consumer.
- `docs/insights/2026-05-25-gram-and-mirror-same-architecture-two-altitudes.md` — LPRM-equivalent at the agent-trajectory altitude; heuristic is the LPRM-equivalent at the AST-sub-region altitude.
- `docs/insights/2026-05-25-pipe-hole-and-au-binary.md` — Fate's tournament resolves at compile time; heuristic informs which sub-ASTs are tournament-eligible.
- `boot/std/epistemologic/property/halts.mirror` — the 0/1 structural pair this property [0,1]-refines.
- `boot/std/epistemologic/property/glass_wall.mirror` — the partition this property refines per sub-AST.
- `boot/std/epistemologic/property/content_addressed.mirror` — the foundation that makes heuristic verdicts content-addressable.

---

*Sub-Turing source bounds the question. Content-addressed AST localises it. The property layer composes it. Heuristic confidence is the gradient cross_wall climbs.*

Apache-2.0.
