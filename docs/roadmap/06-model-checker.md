# 06 — Model Checker Properties

## Status: Infrastructure complete, enforcement pending

Grammar-declared properties flow from .conv source through the full stack to
eigenvalue evaluation. The pipeline is proven end-to-end. Properties are
evaluated but not yet enforced — a failed check does not fail compilation.

---

## Architecture

Three layers. No Gleam types. Grammar all the way down.

```
Layer 0: @property, @topology          vocabulary (types only)
Layer 1: @coincidence                   measurement (actions backed by NIFs)
Layer 2: @training, @reed, etc.        application (declares requires/invariant)
```

The Rust compiler is a grammar/type machine. It parses `requires` and `invariant`
as generic Decl nodes, stores them in TypeRegistry, emits `requires/0` and
`invariants/0` functions on compiled BEAM modules. No interpretation.

The BEAM-side compiler actor reads declarations from compiled modules, calls
`@coincidence.check_property(source, name)` for each. The `@coincidence`
domain server (`coincidence_server.erl`) routes to measurement NIFs that call
into the conversation crate's property evaluation functions.

---

## What's built

### Grammar domains (garden)

- **`@property`** — `requires | invariant | ensures`, `kind`, `verdict`, `builtin`
- **`@topology`** — `graph | node | edge | subgraph | actor`, `measure`, `phase`, `partition`, `boundary`
- **`@training`** — `epoch | step | checkpoint | topology_snapshot`, `layer`, `routing`, `spectral_property`, `phase`, `observation`. Declares `requires shannon_equivalence` and `invariant connected`.
- **`@coincidence`** — `measurement | verdict | spectrum`. Actions: `check`, `measure`, `connected`, `entropy`, `curvature`, `bipartite`, `shannon_equivalence`.

### Rust (conversation crate)

- `parse.rs` — `requires` and `invariant` parsed as Decl nodes in grammar blocks
- `resolve.rs` — TypeRegistry stores `required_properties` and `invariants`
- `compile.rs` — emits `requires/0` and `invariants/0` on compiled BEAM modules
- `property.rs` — `shannon_equivalence()`, `check_builtin()`, `exhaustive_check()`, `connected_check()`, `bipartite_check()`. Used by NIF layer, not by compiler.
- `logic.rs` — FactStore with obligations. ProofCertificate without property results (moved to BEAM).

### BEAM

- `coincidence_server.erl` — custom gen_server routing measurement actions to NIFs
- `compiler.gleam` — `check_requires()` and `check_invariants()` call `@coincidence` after compilation
- `boot.gleam` — `boot_with_infrastructure()` enforces boot ordering
- `loader.gleam` + `loader_ffi.erl` — `get_requires/1`, `get_invariants/1`
- `coincidence.gleam` — Gleam FFI for measurement NIFs and server lifecycle

### NIFs (conversation_nif)

- `check_property(source, name)` — generic dispatch
- `check_shannon_equivalence(source)` — content address uniqueness
- `check_connected(source)` — type graph connectivity (spectral)
- `check_bipartite(source)` — odd cycle detection (spectral)
- `check_exhaustive(source)` — variant coverage

### Tests

- 642 Rust tests, 100% line coverage
- 99 BEAM tests (including 3 capstone property pipeline tests)
- Garden grammar self-tests for all 4 domains

---

## What's evaluated

| Property | What it checks | Backed by |
|----------|---------------|-----------|
| `shannon_equivalence` | All derivations produce unique content addresses | `generate::derive_all` + OID uniqueness |
| `connected` | Type reference graph is a single component | `TypeGraphSpectrum` → Laplacian eigendecomposition (dsyev) |
| `bipartite` | No odd cycles in type reference graph | Spectral analysis + edge counting |
| `exhaustive` | Every declared type has at least one variant | TypeRegistry inspection |

All four are real evaluations, not stubs. `connected` and `bipartite` go
through coincidence's eigendecomposition — actual dsyev calls.

---

## What's next

### Enforcement

The compiler actor discards check results (`let _ = coincidence.check_property(...)`).
Next step: fail compilation when a required property is not satisfied. Return
the failure reason in the compilation error.

### Property results in proof certificate

The BEAM-side compiler actor should assemble property results into the
compilation trace. Each checked property becomes a trace entry — property
name, verdict (pass/fail), and the @coincidence measurement that backs it.

### Properties as grammar actions

Current: property names are strings dispatched through `check_builtin`.
Target: `@property.check("connected")` calls `@coincidence.measure("connected")`.
Property definitions live in .conv files, not Rust match arms. Any domain can
define new properties by declaring actions that return `@property.verdict`.

### Boot ordering formalization

Current: `boot_with_infrastructure()` takes explicit infrastructure/application
lists. Target: the compiler derives boot order from `in @domain` declarations.
Domains that depend on `@property` or `@topology` must boot after them.

---

## Design principle

The Rust compiler is a grammar/type machine. The BEAM side gives meaning to
declarations through the grammar system. The only non-grammar code is the NIF
boundary — below it, math. Above it, grammar. The eigenvalue is the ground truth.
