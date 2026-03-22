# Practice

Engineering discipline for the conversation codebase. If a new contributor reads
this, they understand how we build here.

---

## Content Addressing Is the Organizing Principle

Everything is content-addressed. An `Oid` is a SHA-512 hash of `kind:name:value`.
Two structurally identical nodes produce the same Oid. Identity is structural,
not positional.

This means:
- Equality is hash comparison, not deep traversal
- Trees are deduplicated by construction
- The content address format carries semantic identity (`Decl:in:@filesystem`)
- Naming is infrastructure — CA makes things real in the system by naming them

The `ContentAddressed` trait is the contract. If a type implements it, it has
an identity independent of where it lives.

---

## TDD: Red, Green, Refactor

Non-negotiable. Every feature follows:

1. **Red** (`🔴`) — write compile-failing tests that describe the desired behavior
2. **Green** (`🟢`) — implement the minimum to make them pass
3. **Refactor** (`♻️`) — clean up without changing behavior

Each phase is a separate commit with its emoji marker. The pre-commit hook
enforces this: red phase accepts failures; green and refactor phases require
all checks to pass.

The instinct to skip understanding is strongest when the path is clear. That is
exactly when it matters most.

---

## 100% Line Coverage

The gate: `cargo llvm-cov --package conversation --fail-under-lines 100`.
The commit is rejected if coverage drops below 100%. This is enforced by
`just check` and by the pre-commit hook.

**Closure monomorphization:** Each unique closure literal creates a separate
LLVM monomorphization. A passthrough test that never calls the closure leaves
that monomorphization's body uncovered. Fix: use shared named `fn` pointers
across test pairs that exercise complementary branches.

**`domain_oid!` macro:** Macro expansion coverage is attributed to the
definition file (`kernel.rs`), not the call site. Exercise `From<Oid>`,
`AsRef<str>`, `Display` on at least one domain OID type.

---

## The AST Decomposes All the Way Down

Stringly typed is the devil. Every meaningful distinction in the language
belongs in the `Kind` enum, not in `value: String`. If you pattern-match on
a string to dispatch behavior, the structure wants to move into the enum.

The `value` field on `AstNode` is for names and literals — things without
enumerable structure. Everything with structure gets a type. The compiler
exhaustively checks every variant at every callsite. The string version
silently misses cases.

See `AGENTS.md` for the concrete example (`Kind::When(Op)` vs string operators).

---

## Grammar as the Unit of Composition

A `.conv` file declares a grammar. A grammar defines types and actions for a
domain. The grammar compiles to a BEAM module that handles dispatch.

Composition is grammar composition. `in @actor` declares lineage. Types flow
through `use $t from @module`. Actions call across domains with
`@domain.action(args)`.

The Rust parser produces the AST. The BEAM owns everything after parse:
resolution, compilation, actor lifecycle, package discovery. See `AGENTS.md`
for the boundary.

---

## Supervision Architecture

```
conversation_supervisor (RestForOne)
├── @compiler       — compiles grammars, loads BEAM modules
└── garden          — factory supervisor for domain servers
```

**RestForOne** is structural, not a convenience pick. If `@compiler` crashes,
the garden and all domain servers restart from a clean slate — because any
domain server might hold stale compiled state. If a single domain server
crashes, the factory restarts only that one.

**Witness is structural.** In reed's embedding, witness starts before
`@compiler`. Observation must be running before the first compilation. Not
optional. Not "add later."

Two boot paths exist:
- **Imperative** (`boot.boot_from_files`) — standalone, starts its own tree
- **Supervised** (`boot.supervised_boot_from_files`) — embeds into a parent tree

---

## The NIF Boundary

Rust is the kernel. BEAM is the orchestration.

Two FFI functions cross the boundary (`ffi.rs`):
- `conv_parse` — source to Prism, returns OID
- `conv_compile_grammar` — source to EAF (ETF bytes for BEAM module loading)

The Rust side does not know about actors, supervision, package discovery, or
the garden. It parses and compiles. The BEAM side receives OIDs and bytes.

Do not extend Rust to handle BEAM concerns. The boundary is the architecture.

---

## Litmus Tests Frame Work

Don't say "build X." Give the exact assertion. Make it true.

```
gestalt://document/p|css(.hero)|outer == "<span class='hero'>Hi!</span>"
```

The target IS the spec. The agent works backward from a concrete assertion.
Same pattern as TDD but at the architectural level. If the pitch can't produce
a litmus test, the pitch is not sharp enough.

---

## Commit Discipline

- Commit as `Reed <reed@systemic.engineer>`
- Agents work on their own branch, never commit to main
- Phase markers: `🔴` red, `🟢` green, `♻️` refactor, `🔧` chore, `🔀` merge
- If it is not committed, it did not happen
- CI green before session close
