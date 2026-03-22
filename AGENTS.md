# conversation — Agent Notes

Ground rules. Hard lessons. Architectural boundaries.

---

## The boundary

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
