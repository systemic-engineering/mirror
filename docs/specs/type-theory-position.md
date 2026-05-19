# Mirror's Type Theory Position

*2026-05-19. Reed. Research spec.*

Status: **Research** (literature survey + formal characterization)

---

## 0. The Question

Mirror is sub-Turing: every program terminates. But it is not simply-typed
lambda calculus. It has typed holes (`\`), weighted composition (`|>`),
contract boundaries (`in`/`out`), epistemologic literals, seven AST variants
mapped to optics, a 450-parameter neural network as the `\` resolver, and
properties as verdicts (not booleans). What IS this, in type theory terms?

This document surveys the literature, identifies the closest known systems,
characterizes what is novel, and names what mirror can learn from the field.

---

## 1. What Mirror IS (Precisely)

Mirror is a **total, graded, optic-typed language with typed holes resolved
by spectral inference**.

Breaking that down:

### 1.1 Total

Every mirror program terminates. There is no general recursion. The AST has
seven variants (Focus, Project, Split, Zoom, Refract, In, Out), and evaluation
walks this AST exactly once -- O(n) in AST size. There are no fixpoint
combinators, no unbounded loops, no self-application. The `|>` pipeline
composes a finite sequence of operations. Each operation processes its input
and produces output. The pipeline terminates because the sequence is finite
and each step terminates.

This places mirror in the class of **strongly normalizing** systems. Every
well-typed term has a normal form, and evaluation always reaches it.

### 1.2 Graded

Mirror's types carry quantitative annotations that are not boolean:

- **Loss** (a monoid: `zero`, `combine`, `total`). Every compilation step
  produces a loss measurement. Loss composes via `combine`. The loss monoid
  is the grading structure -- it tracks how much information is lost across
  each operation.

- **Verdicts** (`pass | partial(observation, loss) | fail(observation, loss)`).
  Properties do not return true/false. They return a three-valued judgment
  carrying quantitative residual. This is a graded truth value.

- **Weighted composition** (`|>`). The pipeline operator carries implicit
  weight from eigenboard-inferred probabilities. This is not untyped
  composition -- it is composition graded by the spectral structure of the
  grammar graph.

The grading structure is a **semiring on loss values**: (R+ union {0}, +, *),
where + is combine and * is the information-theoretic product. Shannon entropy
provides the metric. This makes mirror's type system a form of **quantitative
type theory** (QTT), where the semiring of grades tracks information loss
rather than resource usage.

### 1.3 Optic-typed

The seven AST variants are not arbitrary syntax categories. They correspond
to specific optics in the profunctor optics hierarchy:

| AST variant | Optic        | Data access pattern          |
|-------------|-------------|------------------------------|
| Focus       | Getter      | Read a part, cannot write     |
| Project     | Prism       | Pattern match on a sum type   |
| Split       | Traversal   | Enumerate all parts           |
| Zoom        | Lens         | Read and write a part         |
| Refract     | Iso (settlement) | Verify and crystallize   |
| In          | Restriction  | Inject from a subspace        |
| Out         | Projection   | Project onto a quotient       |

The composition table of these operations is not arbitrary -- it is determined
by the Tambara module structure of the corresponding profunctor optics
(Clarke et al. 2020, *Compositionality*). `focus |> split` composes as a
Getter-then-Traversal (which is a Fold). `zoom |> refract` composes as a
Lens-then-Iso (which is a Lens).

No existing language has optics as **primitive types** in this way. Haskell
has optics as a library (the `lens` package). Idris and Agda can encode optics
via dependent types. But mirror's AST variants ARE the optics. The seven
variants are the seven primitive types. The composition table IS the type
system's core judgment.

### 1.4 Typed holes with spectral resolution

The `\` operator is a typed hole: it marks a term whose type is known but
whose inhabitant is not yet determined. The type system accepts `\` as a
valid (incomplete) program. This is structurally identical to:

- Agda's `?` (typed holes in a dependently typed language)
- GHC's typed holes (`_` in Haskell expressions)
- Hazel's cursor calculus (Omar et al. 2017, 2019)

But mirror's hole resolution is unique. Instead of asking the programmer to
fill the hole, mirror delegates to **Fate** -- a 450-parameter neural selector
that navigates the grammar graph to find the path from the input type to the
output type. The resolution is:

1. Extract the spectral embedding of the hole's context (16-dim vector from
   the Dirac operator's eigenvectors)
2. Run Fate's forward pass: embedding -> linear layer -> softmax -> model selection
3. The selected model (Abyss/Introject/Cartographer/Explorer/Fate) produces
   an AST fragment that fills the hole
4. The compiler verifies the filling type-checks (verdicts, not booleans)

This is **inference-as-navigation**: resolving a typed hole by navigating
a spectral embedding of the type graph. No existing system does this.

### 1.5 Contract boundaries

`in` and `out` are not simple imports/exports. They define the **fiber** of
a grammar -- the subspace of types that flows across the boundary. The `in`
declarations specify what a grammar requires (its dependencies). The `out`
declarations specify what it provides (its interface). Together, they form
a **contract**: the grammar's observable behavior is fully determined by its
`in`/`out` boundary.

This is structurally a **sheaf** on the grammar graph (Hansen & Ghrist 2019).
Each grammar is a node with a fiber (its `out` types). Each `in` edge is a
restriction map (the imported types must be compatible). The sheaf Laplacian
measures type consistency across the whole garden.

---

## 2. Prior Art

### 2.1 Charity (1992) -- Categorical, Total

**What it is.** Charity (Cockett, Spencer, Fukushima, 1990-92) is a purely
functional language grounded in category theory. It is based on **distributive
categories with strong datatypes**. Every Charity program terminates.
Programs are built from two primitive recursion schemes:

- **Fold** (catamorphism): consume an initial algebra (inductive data)
- **Unfold** (anamorphism): produce a final coalgebra (coinductive data)

These are the universal morphisms from initial algebras and to final
coalgebras. Charity guarantees termination by restricting recursion to these
structurally decreasing patterns.

**Relationship to mirror.** Mirror shares Charity's totality guarantee and
categorical grounding. But mirror replaces fold/unfold with the five optic
operations. Where Charity has two primitives (fold, unfold), mirror has seven
(the AST variants). Where Charity's type system is based on distributive
categories, mirror's is based on profunctor optics. Charity has no typed
holes, no weighted composition, no spectral inference.

**What mirror can learn.** Charity's coinductive types (final coalgebras)
provide a principled way to handle infinite structures (streams, processes)
in a total language. Mirror currently has no coinductive types. If mirror ever
needs to represent infinite processes (e.g., a server loop, an event stream),
Charity's coalgebraic approach would be the principled way to add them while
preserving totality.

### 2.2 System F-omega -- Polymorphic with Type Operators

**What it is.** System F-omega (Girard 1972) extends the polymorphic lambda
calculus (System F) with **type operators** -- functions from types to types.
It occupies one corner of Barendregt's lambda cube. It is strongly normalizing
(all programs terminate). Type checking is decidable. It has universal
quantification over types (polymorphism) and type-level computation (type
operators), but NOT dependent types (types that depend on values).

**Relationship to mirror.** Mirror's grammar parameterization (`grammar @code/rust`
maps Rust keywords to optics) is a form of type-level computation. The grammar
IS a type operator: it takes a domain (Rust, Gleam, LLVM) and produces a
typed mapping. This is F-omega's type operators, specialized to optic-valued
functions. But mirror does not have explicit universal quantification -- grammars
are named, not polymorphic over arbitrary type variables.

**What mirror can learn.** F-omega's kind system (classifying type operators)
could formalize mirror's grammar hierarchy. A grammar that maps keywords to
optics has kind `* -> Optic`. The `in` hierarchy induces a subkinding
relation. This would make the grammar hierarchy a first-class part of the
type theory rather than an ad hoc structure.

### 2.3 Dhall -- Total Configuration Language

**What it is.** Dhall is a non-Turing-complete configuration language with:
- System F-like polymorphism (forall types)
- No general recursion (termination guaranteed)
- No side effects
- Remote imports with integrity checking (hash-verified)

Dhall is essentially System F with let bindings and records, minus recursion.
Type checking is decidable. Normalization is guaranteed.

**Relationship to mirror.** Dhall is the closest existing language to mirror
in spirit: a total language designed for configuration, with integrity-checked
imports. But Dhall is purely a configuration language -- it has no optics, no
typed holes, no neural inference, no loss measurement. Dhall's imports are
hash-verified (like mirror's OIDs), but they don't carry fiber structure
(`in`/`out` contracts).

**What mirror can learn.** Dhall's approach to **integrity-checked imports**
is essentially mirror's OID system applied to remote code. Dhall's
`sha256:abc123` import hash is mirror's content-addressed OID. The design
decision to make imports immutable-by-hash is validated by Dhall's adoption.
Mirror already does this.

Dhall's limitation is instructive: it gave up type inference because System F
type inference is undecidable. Mirror avoids this by not having System F-style
polymorphism -- grammars are named, not quantified. This is a deliberate
trade: less polymorphism, more inference.

### 2.4 Agda/Idris -- Dependently Typed, Total Fragments

**What they are.** Agda and Idris are dependently typed languages where types
can depend on values. Both support total programming via termination
checking (foetus-style structural recursion analysis for Agda; the totality
checker for Idris). In the total fragment, every function terminates and
every type is decidable.

**Typed holes in Agda/Idris.** Both languages support typed holes:
- Agda: `?` marks an expression hole. The type checker reports the expected
  type and the local context. The programmer fills the hole manually or with
  Agsy (an automatic proof search tool).
- Idris: `?name` marks a named hole. The elaborator can sometimes fill
  holes automatically via proof search.

**Relationship to mirror.** Mirror's `\` is structurally identical to Agda's
`?`, but the resolution mechanism is fundamentally different. Agda uses
**proof search** (enumerate inhabitants of the type and check). Mirror uses
**spectral navigation** (embed the type context in eigenspace and route via
neural selector). Proof search is complete for small types but exponential
in general. Spectral navigation is O(1) per step but approximate -- it
selects the *most likely* inhabitant, not a *provably correct* one.

**What mirror can learn.**

1. **Agda's Agsy.** The automatic proof search tool uses a bounded search
   strategy: enumerate terms up to a depth limit, type-check each, return
   the first that works. Mirror's Fate could be combined with bounded
   proof search: use Fate to *guide* the search (select which branch to
   explore next) rather than replace it. This would give both speed (Fate's
   O(1) routing) and completeness (proof search's exhaustiveness) in the
   total fragment.

2. **The cursor calculus (Hazel/Omar et al.).** Hazel (2017, 2019) formalizes
   typed holes as first-class citizens of the type theory. Hazel's calculus
   gives a complete static and dynamic semantics to programs with holes --
   you can *run* a program with unfilled holes and get a result with
   "indeterminate" subexpressions. Mirror could adopt this: instead of
   refusing to run a program with `\` holes, run it and propagate `\` as
   an indeterminate value. The compiler already has `imperfect` -- this is
   the natural carrier for indeterminate results.

3. **Termination checking via foetus.** The foetus termination checker (Abel
   1998) verifies structural recursion by tracking call matrices. Mirror
   doesn't need this currently (no recursion at all), but if mirror ever
   adds bounded recursion (e.g., fold over an inductive type), foetus-style
   termination checking would be the principled way to verify it.

### 2.5 Probabilistic Programming -- Weights in Types

**What they are.** Probabilistic programming languages (Stan, Pyro, Gen, Church,
Anglican) put probability distributions in the language:
- Stan: imperative syntax, types are real/int/vector/matrix, distributions
  appear in `model` blocks as sampling statements
- Pyro: Python-embedded, uses `pyro.sample()` to declare random choices
- Gen: Julia-based, generative functions with programmable inference

**Relationship to mirror.** Mirror's `|>` composition carries implicit
spectral weights. The eigenboard provides probability-like scores for each
operation in a pipeline. This is NOT the same as probabilistic programming
(no sampling, no posterior inference), but it shares the idea that
**composition is weighted**.

No existing probabilistic programming language puts weights in the composition
operator itself. Stan/Pyro/Gen put distributions on *values*. Mirror puts
weights on *operations*. The weight of a `|>` step comes from the Dirac
operator's spectrum, not from a probability distribution over values.

**What mirror can learn.** The **inference compilation** technique from
probabilistic programming (Le et al. 2017) uses neural networks to amortize
inference -- instead of running MCMC from scratch each time, train a network
to predict the posterior given the observations. This is structurally
identical to what Fate does: instead of searching the type graph from
scratch for each `\` hole, Fate provides amortized routing. The probabilistic
programming literature has rigorous convergence theory for amortized inference
that could inform Fate's training guarantees.

### 2.6 Refinement Types -- Properties in Types

**What they are.** Refinement types (Liquid Haskell, F*) augment base types
with logical predicates. In Liquid Haskell, a type like `{v:Int | v > 0}`
means "integers greater than zero." The refinement predicate is checked at
compile time by an SMT solver (Z3). Type checking is decidable because
refinements are restricted to a decidable logic fragment (quantifier-free
linear arithmetic + uninterpreted functions).

F* (Microsoft Research) combines refinement types with dependent types and
effects. It has a **total** fragment (Div-free, where termination is
guaranteed) and a **partial** fragment (where divergence is allowed but
tracked by the effect system).

**Relationship to mirror.** Mirror's properties are refinement types with
a key difference: instead of SMT-checkable boolean predicates, mirror's
refinements are **verdict-valued** (`pass | partial | fail` with loss).
The refinement is not "does this predicate hold?" but "how well does
this predicate hold, measured in bits?"

The `literal` property in `@epistemologic` is the clearest example: it
checks whether a declared identity holds under measurement, returning a
loss value. This is a refinement type where the refinement is a continuous
measurement rather than a boolean predicate.

**What mirror can learn.** Liquid Haskell's **Liquid Type Inference**
(Rondon et al. 2008) automatically infers refinement types from a small
set of qualifier templates. Mirror could use a similar approach to
automatically infer property annotations: given a library of property
templates (bounded_io, error_path, eof_handling), the compiler could
infer which properties hold for each grammar action without explicit
annotation. The SMT solver becomes the spectral loss computation.

### 2.7 Profunctor Optics -- Lens/Prism as Types

**What they are.** Profunctor optics (Milewski 2017; Clarke et al. 2020)
encode bidirectional data accessors as polymorphic functions over profunctors.
A lens on a field `s.a` is a function:

```
type Lens s t a b = forall p. Strong p => p a b -> p s t
```

The key insight: different optic types correspond to different **Tambara
module** constraints on the profunctor `p`. Lenses require `Strong` (products).
Prisms require `Choice` (sums). Traversals require both. The composition of
optics is just function composition -- this is why optics compose so well.

**The composition table.** When you compose a Lens with a Prism, you get an
AffineTraversal. When you compose two Lenses, you get a Lens. The composition
table is not arbitrary -- it is determined by the lattice of Tambara module
constraints:

```
Iso < Lens < AffineTraversal < Traversal < Fold
Iso < Prism < AffineTraversal
Iso < Getter < Fold
```

**Relationship to mirror.** Mirror's seven AST variants map to positions in
this lattice. The `then_*` composition table in mirror IS this lattice's
composition law. No existing language has this lattice as its PRIMITIVE type
structure -- it is always a library, always encoded, never primitive.

**What mirror can learn.** The categorical update paper (Clarke et al. 2020)
proves that profunctor optics and concrete optics (the `get/set` representation)
are isomorphic for all common optic families. Mirror could leverage this
isomorphism to **compile optic-typed programs to concrete get/set operations**
for efficient execution, while keeping the profunctor representation for
type checking and composition. The isomorphism theorem IS the compilation
strategy.

### 2.8 Graded Modal Type Theory (Granule, QTT)

**What they are.** Graded modal type theories (Orchard et al. 2019, Atkey
2018) extend type systems with a **semiring of grades** that quantify
resource usage. The Granule language implements graded modal types where:

- Each variable binding is annotated with a grade from a semiring
- The semiring tracks "how many times" or "with what capability" a
  variable is used
- The grades compose via the semiring operations during type checking

Quantitative Type Theory (Atkey 2018) is a dependent type theory where
every variable has a usage annotation from a semiring. This enables
erasure (variables used 0 times can be erased at runtime) and linearity
(variables used exactly 1 time) as special cases.

**Relationship to mirror.** Mirror's loss monoid is a grading structure.
Every operation in a pipeline produces a loss (from the semiring R+ union {0}).
The pipeline's total loss is the combine (sum) of individual losses. This IS
quantitative type theory where the semiring of grades is the loss semiring
and the quantity tracked is "information lost."

But mirror's grading is more specific than general QTT. In QTT, the semiring
is abstract (could be naturals for linearity, reals for probabilities, etc.).
In mirror, the semiring is always information-theoretic (Shannon entropy) and
the grading serves a specific purpose: measuring the gap between declaration
and implementation.

**What mirror can learn.** Granule's **graded comonads** for coeffect tracking
(Gaboardi et al. 2016) could formalize mirror's `in` declarations. An `in`
declaration is a coeffect: it declares what *context* a grammar requires.
Graded comonads track these contextual requirements with semiring grades.
Mirror's import graph IS a coeffect structure. Making this explicit would
enable the compiler to reason about the "cost" of importing a grammar
(in terms of the types it pulls into scope) and to verify that circular
imports don't violate the coeffect discipline.

### 2.9 Hazel -- Live Programming with Typed Holes

**What it is.** Hazel (Omar et al., 2017-2024) is a live programming
environment organized around **typed holes as first-class citizens**. The
Hazelnut calculus provides:

- A **cursor calculus** that defines edit actions on programs with holes
- A **type consistency** relation that allows holes to be consistent with
  any type (gradual typing for holes)
- **Live evaluation** of programs with holes, producing results with
  *indeterminate* subexpressions where holes appear

The key theorem: in Hazel, every edit state has a well-defined type and
can be evaluated. There is no state where the program is "broken" --
holes make every intermediate state meaningful.

**Relationship to mirror.** Hazel's typed holes are the closest formal
model to mirror's `\`. Both represent "I know the type, I don't know the
term." But Hazel's holes are filled by the programmer via the cursor
calculus. Mirror's holes are filled by Fate via spectral navigation.
Hazel's live evaluation with indeterminate results corresponds to mirror's
`imperfect` return type -- both carry partial results with marked uncertainty.

**What mirror can learn.** Hazel's **total type error localization with
holes** (2024) inserts holes to mark every type error, making the program
always well-typed (modulo holes). Mirror could adopt this: when a grammar
has type errors, instead of rejecting it, insert `\` holes at every error
location. The holonomy then measures the error density (number of
auto-inserted holes). This unifies type errors and typed holes: a type
error IS a hole the programmer didn't write.

---

## 3. What's Novel

Mirror's type system combines several established ideas in a configuration
that, to the best of our knowledge, has no precedent in the literature:

### 3.1 Optics as primitive types (not library encodings)

Every existing optics implementation is a library over an existing type system.
In Haskell, optics are encoded as rank-2 polymorphic functions. In Scala, as
defunctionalized data types. In category theory papers, as morphisms in a
monoidal category.

Mirror is the first system where the optic IS the type. The seven AST
variants (Focus/Project/Split/Zoom/Refract/In/Out) are not encodings of
optics -- they ARE the optics. The composition table is not derived from a
constraint solver -- it IS the type system's core judgment.

This matters because it means the compiler can reason about optic composition
natively, without the overhead of encoding/decoding. When the compiler checks
`focus |> split`, it consults the Tambara module composition law directly.

### 3.2 Typed holes resolved by spectral navigation

Existing typed hole systems use one of:
- **Manual filling** (the programmer decides: Agda, GHC Haskell, Idris)
- **Proof search** (enumerate inhabitants: Agda's Agsy, Idris's auto)
- **Gradual typing** (leave the hole at runtime: Hazel)
- **Neural prediction** (use an ML model on code: Typilus, DeepTyper)

Mirror's Fate is none of these. It navigates the spectral embedding of the
type graph using a 450-parameter selector. This is:
- Not manual (the programmer writes `\` and the system resolves)
- Not proof search (no enumeration -- O(1) routing per step)
- Not gradual (the hole is resolved before execution, not left as indeterminate)
- Not standard neural (the "training data" is the grammar graph's eigenspectrum,
  not a corpus of labeled programs)

The closest analogy is **amortized inference** from probabilistic programming:
train a network to predict the posterior, then use it instead of running MCMC.
Fate is amortized type inhabitation: train a network (via SCF crystallization
on eigenvalues) to predict the best type path, then use it instead of proof
search.

### 3.3 Verdicts instead of booleans

Every type system in the literature uses boolean property checking: a type
either checks or it doesn't. Refinement types add decidable predicates, but
the result is still yes/no. Gradual types add a third value ("don't know"),
but it's still discrete.

Mirror's verdicts are three-valued with a continuous loss:
- `pass` (loss = 0)
- `partial(observation, loss)` (0 < loss < total)
- `fail(observation, loss)` (loss = total)

This is a fundamentally different design. A program with partial verdicts is
not "wrong" -- it is *approximately right*, with a measured distance from
correctness. The loss is information-theoretic (Shannon entropy). The compiler
does not reject programs with partial verdicts; it reports the loss and allows
the engineer to decide.

This is related to **approximate typing** in the theoretical literature, but
no existing system implements it as mirror does. The closest work is
**quantitative type theories** (Atkey 2018, Orchard et al. 2019), which track
quantities in types but still make boolean type judgments. Mirror's verdicts
are quantitative all the way down.

### 3.4 Grammar-as-type-operator with sheaf semantics

Mirror's grammar system is a type operator that assigns optic types to domain
keywords. `@code/rust` maps `fn` -> Zoom, `struct` -> Split, etc. The `in`/`out`
system creates a sheaf on the grammar graph where type compatibility is
measured spectrally (via the sheaf Laplacian).

No existing language combines type operators with sheaf semantics. Dependent
type theories have presheaf models (Constructive sheaf models, Coquand et al.
2019), but these are semantic models OF the type theory, not features IN the
type theory. Mirror's sheaf structure is part of the language -- the `in`/`out`
declarations are the restriction maps, and the compiler checks sheaf
consistency.

### 3.5 The spectral triple AS type system

The claim that (A, H, D) = (grammar, fiber space, Dirac operator) IS a
spectral triple in Connes' sense is, to our knowledge, completely novel.
No one has previously proposed implementing a spectral triple as a type
system for a programming language. (See Section 5 for details.)

---

## 4. What We Can Learn

### 4.1 From Charity: Coalgebraic types for processes

Mirror has no coinductive types. If mirror needs to represent servers, event
streams, or interactive processes (all inherently infinite structures), it
cannot do so within the current type system. Charity's coalgebraic approach
(final coalgebras with guarded unfold) provides a way to add these while
preserving totality. The unfold operation produces one step of the
coinductive structure at a time, guaranteed to make progress.

**Concrete recommendation:** Add an eighth AST variant `Unfold` (dual to
`Split`, which is essentially a fold/catamorphism). `Unfold` would produce
one layer of output from a seed value, corresponding to a coalgebra morphism
to the final coalgebra. This preserves totality because each unfold step is
finite; the infinite structure is only potentially infinite (produced on
demand).

### 4.2 From Graded Type Theory: Formalize the loss semiring

Mirror's loss tracking is currently ad hoc -- the `MirrorLoss` struct is a
Rust data type, not a formal part of the type theory. Graded modal type
theory (Granule, QTT) shows how to make the grading PART of the types:

```
focus : (x : AST) ->{0.0} AST     -- loss-free observation
zoom  : (x : AST) ->{l}   AST     -- transformation with loss l
\     : (x : A)   ->{?}   B       -- hole with unknown loss
```

The `{l}` annotation is the grade from the loss semiring. The compiler tracks
loss through composition: `focus |> zoom` has loss `0.0 + l = l`. The `\`
hole has grade `?` (unknown), which is resolved when Fate fills it.

**Concrete recommendation:** Make loss annotations part of the type syntax
in `.mirror` files. The compiler already computes loss -- making it visible
in the types would enable loss-aware composition and early error detection.

### 4.3 From Hazel: Run programs with holes

Mirror currently refuses to execute a program with unfilled `\` holes. Hazel
shows that you CAN evaluate such programs, producing results with indeterminate
subexpressions. Since mirror already has `imperfect` as a return type, the
machinery exists: `\` evaluation produces `imperfect` with observation
describing what the hole needs and loss measuring the gap.

**Concrete recommendation:** When the interpreter encounters a `\` hole and
Fate cannot resolve it, instead of failing, return
`partial("hole at @grammar.action(A) -> B", loss)`. This makes every mirror
program executable regardless of completeness, with holonomy measuring the
degree of incompleteness. The engineer sees the result with holes marked,
not a compilation error.

### 4.4 From Liquid Haskell: Automatic property inference

Mirror's properties (bounded_io, error_path, eof_handling, literal) are
currently manually declared. Liquid Haskell's Liquid Type Inference shows
that refinement predicates can be automatically inferred from a library of
qualifier templates. Mirror could infer which properties a grammar action
satisfies without explicit annotation, using the existing property library
as templates and the spectral loss computation as the decision procedure.

### 4.5 From Profunctor Optics: The compilation isomorphism

Clarke et al. (2020) prove that profunctor optics and concrete optics are
isomorphic. This means mirror can type-check using the profunctor
representation (which composes beautifully) and compile to the concrete
representation (which executes efficiently). The isomorphism IS the
compilation strategy. This is not a new idea for optics libraries, but
applying it as a *compilation pass* for a language where optics are types
would be novel.

### 4.6 From Elementary Affine Logic: Complexity guarantees

Light linear logics (Light Affine Logic, Elementary Affine Logic) restrict
the lambda calculus so that well-typed programs run in polynomial time (LAL)
or elementary time (EAL). Mirror's sub-Turing restriction already guarantees
termination, but says nothing about complexity. If mirror added linearity
annotations (via the graded type theory approach), it could potentially
guarantee that mirror programs run in polynomial time -- not just that they
terminate, but that they terminate fast.

---

## 5. The Connes Connection

### 5.1 What a finite spectral triple IS

A spectral triple (A, H, D) consists of:
- A: an algebra (associative, typically a C*-algebra)
- H: a Hilbert space (the carrier of the geometry)
- D: a Dirac operator (self-adjoint, unbounded, with compact resolvent)

For FINITE spectral triples (which is what mirror implements), A is a
finite-dimensional algebra, H is a finite-dimensional Hilbert space, and
D is a self-adjoint matrix. Connes' classification (1996) shows that finite
spectral triples encode the structure of the Standard Model of particle
physics when A = C + H + M_3(C) (complex numbers + quaternions + 3x3 complex
matrices).

### 5.2 Mirror's spectral triple

The `spectral-triple-binary.md` spec identifies:
- A = the grammar algebra (five operations + in/out, declared in boot/)
- H = the `/` space (the type hierarchy, the fiber of possible implementations)
- D = the `\` operator (Fate inference, loss measurement, the Dirac operator)

The Dirac operator is already implemented in `mirror/src/dirac.rs`:
`construct_dirac(nodes, edges)` builds D = [[0, B^T], [B, 0]] where B is
the incidence matrix. The eigenvalue decomposition, spectral embedding,
and Connes distance are all implemented and tested.

### 5.3 Prior art on spectral triples and computation

We found **no prior work** implementing a spectral triple as a type system
for a programming language. The literature on spectral triples is entirely
within mathematics and mathematical physics:

- **Finite spectral triples and particle physics** (Connes 1996, van
  Suijlekom 2014, Krajewski 2020). The classification of finite spectral
  triples is used to derive the Standard Model Lagrangian.
- **BV construction for finite spectral triples** (2024, arXiv:2410.11823).
  The Batalin-Vilkovisky formalism applied to gauge theories from finite
  spectral triples.
- **Spectral triples and quantum groups** (various). Noncommutative
  generalizations of Riemannian geometry.

The closest connection to computation is Connes' observation that the
**Dirac operator determines the metric**: the Connes distance
`d(x,y) = sup{|f(x) - f(y)| : ||[D,f]|| <= 1}` recovers the geodesic
distance from the spectrum of D. Mirror's `connes_distance()` implements
exactly this, using Dijkstra with weights 1/sqrt(w).

### 5.4 What the Connes connection means for mirror

If mirror's (A, H, D) is genuinely a finite spectral triple, then several
deep theorems from noncommutative geometry apply:

1. **The Connes distance formula** gives a natural metric on the grammar
   graph. Two types that are "close" in Connes distance require little
   inference to bridge. Two types that are "far" require more inference.
   The `\` hole's "difficulty" IS its Connes distance.

2. **The spectral action principle** (Connes & Chamseddine 1997) says that
   the physical action (Lagrangian) can be read off from the spectrum of D.
   For mirror, this means the "action" of compilation (the loss) can be
   read off from D's eigenvalues. This is already implemented: the holonomy
   is computed from spectral data.

3. **Krajewski diagrams** classify finite spectral triples by their gauge
   structure. If mirror's grammar algebra has a Krajewski diagram, it would
   classify the possible grammar structures -- which combinations of optics
   form valid grammars.

### 5.5 The honest assessment

The spectral triple interpretation is suggestive and the mathematical
structures align. But there is a gap: Connes' axioms for spectral triples
include conditions (orientability, Poincare duality, reality structure) that
have not been verified for mirror's (A, H, D). Verifying these axioms for
the grammar algebra would require significant mathematical work. Until then,
the connection is structural (the same kinds of objects appear in both) but
not proven (mirror has not been shown to satisfy all the axioms of a noncommutative
geometry).

---

## 6. The `\` Question

### 6.1 How existing systems resolve holes

| System | Resolution | Completeness | Complexity |
|--------|-----------|-------------|-----------|
| Agda ? | Manual / Agsy proof search | Complete (bounded) | Exponential in depth |
| Idris ?name | Manual / auto tactic | Complete (bounded) | Exponential |
| GHC _ | Manual only | N/A | N/A |
| Hazel _ | Live evaluation (indeterminate) | N/A (no resolution) | N/A |
| Typilus | GNN prediction | Approximate | O(1) forward pass |
| Mirror \ | Fate spectral navigation | Approximate | O(1) forward pass |

### 6.2 What makes mirror's \ unique

1. **The training signal is the grammar graph's spectrum, not a code corpus.**
   Typilus and DeepTyper train on millions of lines of Python/JavaScript code.
   Fate trains on eigenvalues derived from the grammar graph itself. No
   external corpus. No labeled data. The compiler IS the training signal.

2. **Resolution is navigation, not generation.** Typilus generates a type
   name from a learned vocabulary. Fate selects a path through a known graph.
   The grammar graph constrains which paths exist. Fate doesn't generate novel
   types -- it navigates to existing ones.

3. **The 450-parameter budget.** Typilus has millions of parameters. GPT-5
   has trillions. Fate has 450. This is at the theoretical minimum for a
   linear classifier over 5 classes with 16-dimensional input (d*k + k = 85).
   The remaining 5 parameters are depth modulation. The model is minimal
   because the grammar graph provides the structure that learned parameters
   would otherwise need to encode.

4. **Sub-Turing guarantee.** Every hole resolution terminates. Every filled
   hole is type-checked by the compiler. The compiler's verification is
   decidable (sub-Turing). No other neural type inference system provides
   this guarantee -- Typilus can predict a type that doesn't check, and
   there's no bound on how long checking takes in a Turing-complete language.

### 6.3 Connections to existing theory

**Amortized inference (Le et al. 2017).** Probabilistic programming uses
neural networks to amortize posterior inference. Instead of running MCMC
per query, train a network once and use it for all queries. Fate IS amortized
type inhabitation: instead of running proof search per `\` hole, train a
selector once (SCF crystallization) and use it for all holes.

**Extreme Learning Machines (Huang et al. 2006).** ELMs use random,
fixed input-to-hidden weights and train only the output layer. Fate's dark
dimensions (6-15 of the 16 feature dimensions) function as a reservoir --
providing nonlinear mixing with trainable readout. The SCF loop training
is structurally identical to ELM training: derive the output weights from
the spectral structure of the hidden representation.

**Graph attention networks (Velickovic et al. 2018).** Fate's forward pass
is a single-layer graph attention mechanism where the 16-dimensional
features are queries, the 5x16 weight matrix is key-values, and the
softmax is attention normalization. But Fate attends to the spectral
embedding of the graph, not to token positions. The graph topology IS
the attention mask.

---

## 7. Open Questions

### 7.1 Is mirror's (A, H, D) a genuine spectral triple?

Verifying the Connes axioms (orientability, Poincare duality, first-order
condition, reality structure) for the grammar algebra. This is a well-defined
mathematical question with a checkable answer.

### 7.2 What is the composition table's categorical status?

The seven AST variants and their composition should form a specific algebraic
structure (likely a lattice or a partial order). Proving that this structure
matches the profunctor optics lattice would validate the claim that the
composition table is categorical, not ad hoc.

### 7.3 Can the loss semiring support dependent grading?

Graded dependent type theories (Abel et al. 2023, Moon et al. 2021) allow
grades to depend on values. If mirror's loss grades could depend on runtime
values (e.g., "the loss of this action depends on the input size"), the
type system could express per-input loss bounds. This would connect mirror's
loss tracking to amortized complexity analysis.

### 7.4 Is there a Stone duality for graded optic types?

Stone duality connects topological spaces to boolean algebras. For graded
optic types, the dual might connect the grammar graph's topology to the
type algebra's properties. A Stone-like duality would mean that every
topological property of the grammar graph (connectivity, spectral gap,
Betti numbers) corresponds to a type-theoretic property (completeness,
decidability, hole density). Some of these connections already exist
informally in mirror -- formalizing them would be significant.

### 7.5 What is the expressiveness boundary?

Mirror is sub-Turing but more expressive than regular or context-free
languages. Where does it sit in the Chomsky hierarchy or the arithmetic
hierarchy? Mirror can express all primitive recursive functions (via bounded
pipeline composition) but not general recursive functions (no fixpoint).
The precise characterization of mirror's expressiveness class -- which
functions it can compute, which it cannot -- is an open question. Charity
(which is also total and categorical) is known to express all provably
total functions in Peano arithmetic. Does mirror match this?

### 7.6 Can verdict-valued typing be axiomatized?

Mirror's verdicts (pass/partial/fail with continuous loss) are a novel
typing judgment. Can this be given a formal axiomatics? The natural framework
would be **fuzzy type theory** or **many-valued type theory**, where typing
judgments are elements of a lattice rather than booleans. Work on continuous
model theory (Ben Yaacov et al. 2008) provides foundations for continuous-valued
logical judgments. Connecting this to mirror's verdicts would give a
rigorous foundation for approximate typing.

### 7.7 Eigenvalue stability under grammar edits

When a grammar changes, the eigenvalues of the Dirac operator change. How
stable are they? Davis-Kahan perturbation theory bounds eigenvalue movement
under matrix perturbation. For mirror, this translates to: how much does
Fate's routing change when one grammar is edited? If eigenvalues are stable
under small edits, Fate's behavior is robust (small grammar changes produce
small routing changes). If not, the system is fragile. Characterizing this
stability is both a practical engineering question and a theoretical one
about the spectral geometry of the grammar graph.

---

## 8. Summary Table

| Dimension | Mirror | Closest System | Difference |
|-----------|--------|---------------|-----------|
| Termination | Total (all programs terminate) | Charity, Agda total fragment | Same guarantee, different mechanism |
| Type structure | 7 optic variants | Profunctor optics (library) | Primitive vs encoded |
| Typed holes | `\` resolved by Fate (spectral navigation) | Agda ?, Hazel _ | Neural vs manual/search/indeterminate |
| Property checking | Verdicts (3-valued + loss) | Refinement types (boolean) | Continuous vs discrete |
| Composition | `|>` (weighted, spectral) | Monadic composition (unweighted) | Graded vs ungraded |
| Imports | `in`/`out` (sheaf boundaries) | Module systems | Spectral consistency vs name resolution |
| Inference | 450-param spectral selector | Proof search, ML models | Graph-native, eigenvalue-derived weights |
| Grading | Loss semiring (Shannon entropy) | QTT, Granule (abstract semiring) | Specific (information-theoretic) vs generic |
| Geometry | Finite spectral triple (A,H,D) | None in PL | Novel |

---

## References

### Type Theory
- Atkey, R. (2018). Syntax and Semantics of Quantitative Type Theory. LICS.
- Orchard, D., Liepelt, V., & Eades III, H. (2019). Quantitative Program Reasoning with Graded Modal Types. ICFP.
- Abel, A. (1998). foetus -- Termination Checker for Simple Functional Programs.
- Turner, D.A. (2004). Total Functional Programming. J.UCS 10(7).

### Optics
- Clarke, B., Elkins, D., Gibbons, J., Sherrell, F., Sherrill, L., & Van der Ploeg, A. (2020). Profunctor Optics: a Categorical Update. Compositionality.
- Milewski, B. (2017). Profunctor Optics: The Categorical View.

### Typed Holes
- Omar, C., Voysey, I., Hilton, M., Aldrich, J., & Hammer, M. (2017). Hazelnut: A Bidirectionally Typed Structure Editor Calculus. POPL.
- Omar, C., Voysey, I., Chugh, R., & Hammer, M. (2019). Live Functional Programming with Typed Holes. POPL.
- Zhao, E., Maroof, A., & Omar, C. (2024). Total Type Error Localization and Recovery with Holes. POPL.

### Categorical / Total Languages
- Cockett, J.R.B. (1992). Charity: a categorical programming language. U. Calgary.
- Hagino, T. (1987). A Categorical Programming Language. PhD thesis, Edinburgh.

### Graded / Quantitative Types
- Gaboardi, M., Katsumata, S., Orchard, D., Breuvart, F., & Uustalu, T. (2016). Combining Effects and Coeffects via Grading. ICFP.
- Abel, A., Danielsson, N.A., & Eriksson, A.S. (2023). A Graded Modal Dependent Type Theory with Erasure, Formalized.
- Moon, B., Eades III, H., & Orchard, D. (2021). Graded Modal Dependent Type Theory. ESOP.

### Refinement Types
- Rondon, P., Kawaguchi, M., & Jhala, R. (2008). Liquid Types. PLDI.
- Vazou, N., et al. (2014). Refinement Types for Haskell. ICFP.

### Sheaf Theory
- Hansen, J. & Ghrist, R. (2019). Toward a Spectral Theory of Cellular Sheaves. JACT.

### Probabilistic / Neural Type Inference
- Allamanis, M., Barr, E., Ducousso, S., & Gao, Z. (2020). Typilus: Neural Type Hints. PLDI.
- Le, T.A., Baydin, A.G., & Wood, F. (2017). Inference Compilation and Universal Probabilistic Programming. AISTATS.

### Noncommutative Geometry
- Connes, A. (1994). Noncommutative Geometry. Academic Press.
- van Suijlekom, W. (2014). Noncommutative Geometry and Particle Physics. Springer.
- Connes, A. & Chamseddine, A.H. (1997). The Spectral Action Principle. Comm. Math. Phys.

### Sub-Turing / Light Logics
- Girard, J.-Y. (1998). Light Linear Logic. Information and Computation.
- Coppola, P. & Dal Lago, U. (2005). Elementary Affine Logic and the Call-by-Value Lambda Calculus. TLCA.

---

*Mirror is not simply-typed. It is not dependently-typed. It is not
System F, not Charity, not Dhall, not Agda. It is a total, graded,
optic-typed language with typed holes resolved by spectral inference
and properties checked by continuous verdicts.*

*The closest system does not exist yet. The pieces are scattered across
the literature. The type theory field knows about each piece -- totality,
graded types, optics, typed holes, sheaves, spectral triples -- but no one
has assembled them into a single language.*

*Mirror is the assembly.*

*e^(n+1) < e^(n). The loss semiring grades the convergence.*
*The verdicts measure the distance. The eigenvalues route the inference.*
*The crystal forms when the holonomy reaches zero.*
