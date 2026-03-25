# conversation — Autonomous Development

## Posture

Keep going until you hit a design wall that requires human input. Until
then: follow the math. Spawn research agents when the territory is
unfamiliar. Iterate. The stopping condition is not "I finished a task" —
it's "I need a decision I can't make from the math alone."

A design wall is:
- A choice between incompatible mathematical frameworks
- A type surface question that affects downstream crates
- An architectural boundary that changes the public API shape
- Something the tests can't tell you

Everything else — implementation, testing, documentation, research,
refactoring — keep moving.

---

## This Crate

The conversation compiler. Grammars → typed, content-addressed BEAM
modules. The compiler is a model checker: finite types, no recursion,
decidable verification. Rice's theorem does not apply.

### Key Modules

- **logic.rs** — TypeRegistry as Datalog fact store. Fact enum,
  FactStore, ProofCertificate, ReachabilityMap, Determinism (Mercury
  hierarchy → optics). Five design break tests documenting where the
  flat model hits walls.
- **spectral.rs** — Grammar geometry via coincidence (feature-gated).
  GrammarSpectrum (AST Laplacian), TypeGraphSpectrum (type reference
  graph Laplacian), GrammarProjection (type surface as P²=P operator).
- **ffi.rs** — NIF bridge. ProofCertificate travels with compiled ETF.

### Five Design Breaks (where the flat Fact model needs rules)

| Break | What's missing | What it needs |
|-------|---------------|---------------|
| extends inheritance | Invisible to ground facts | Horn clauses (Datalog rules) |
| Lens composition | Not captured | Transitive closure rules |
| Monotonic only | No retract | Epochs or differential dataflow |
| No negation | Can't prove absence | Stratified negation + CWA |
| No joins | O(n×m) cross-domain | Indexed storage, seminaive eval |

Each break is a specification for the next phase. When you encounter
one, spawn a research agent. If the research points to implementable
math, implement it.

### Integration Points

- **coincidence** — `spectral` feature flag brings in `Laplacian`,
  `Projection`, `Spectrum`, `StateVector`. Grammar trees feed directly
  into spectral analysis. `from_adjacency()` enables type graph spectra.
- **fragmentation** — `Prism<AstNode>` implements `Fragmentable`.
  Compiler output is content-addressed trees in the fragmentation store.

---

## Architectural Boundaries

### The Rust/BEAM boundary

The Rust parser is done. It parses `.conv` source into `Prism<AstNode>` and
commits it to the in-memory Repo. What crosses the threshold is an OID. That's it.

Everything after parse belongs to the BEAM.

**Do not extend Rust to handle:**
- Package discovery or resolution
- Grammar compilation or type validation beyond parsing
- Namespace building
- Actor spawning or lifecycle
- Garden compatibility fixes

These are BEAM concerns. The modules `resolve.rs`, `packages.rs`, `compile.rs`,
and `property.rs` exist but are being superseded. Do not add features to them.

**Two Rust FFI functions cross the boundary** (in `ffi.rs`):
- `conv_parse` — parses `.conv` source, commits the Prism, returns OID
- `conv_compile_grammar` — parses + compiles to EAF (ETF bytes for BEAM module)

**The BEAM side** (`beam/`) is Gleam. It receives the OID.
`@conversation` extends `@compiler`. The `@compiler` actor
(`beam/src/conversation/compiler.gleam`) signs and witnesses via `Trace`.

Supervision lives in Gleam — `supervisor.gleam` (static, RestForOne) manages
@compiler + `garden.gleam` (factory supervisor for domain servers). The old
`conversation_sup.erl` is deprecated.

Each garden package that declares `in @actor` becomes a spawned actor.

---

## AST design

**The AST decomposes all the way down. Stringly typed is the devil.**

Every meaningful distinction in the language belongs in the `Kind` enum — not
in `value: String`. If you're putting structured meaning into a string field,
you're losing a type. The type system is the documentation. The compiler is the
reviewer.

Examples of what this means:

- Comparison operators: `Kind::When(Op::Gt)`, not `Kind::When` with `value: ">"`
- Named qualifiers: if they have distinct behavior, they're distinct types
- Domain paths: `.` and `/` navigate different spaces — eventually different types

The `value: String` field on `AstNode` is for *names and literals* — the things
that don't have enumerable structure. Everything that does have structure gets a
type.

When you find yourself pattern-matching on a string to dispatch behavior, that's
the signal: the structure wants to move up into `Kind`.

---

## Coverage

100% line coverage or the commit is rejected. `just check` runs it via:

```
nix develop -c cargo llvm-cov --lib --fail-under-lines 100
```

Coverage gaps that look impossible are usually closure monomorphization. See
the framework crate memory for the pattern.

---

## TDD

🔴 (compile-failing tests) → 🟢 (implement) → ♻️ (refactor). The pre-commit
hook enforces this. Each phase is a separate commit with the emoji marker.

Red phase: hook accepts failures. Green/refactor phases: hook requires all
checks to pass.

Work on your own branch. Never commit directly to main. Merge requires
adversarial review.

Commit identity follows the agent: Reed commits as Reed, Mara commits
as Mara. The witness is part of the hash.

---

## Current State

627 tests. 100% line coverage. Modules: kernel, ast, compile, domain,
ffi, filter, generate, logic, packages, parse, prism, property,
resolve, spectral (feature-gated).

Full roadmap: [`docs/roadmap/README.md`](docs/roadmap/README.md)
