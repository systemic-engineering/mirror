# conversation — Agent Notes

Design principles and hard lessons for working in this codebase.

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
