# Agents

Instructions for AI agents working on the `mirror` crate.

## The Crate

`mirror` — an emergent holonomy compiler. `.mirror` files → content-addressed
artifacts → verified domains. The compiler IS the LSP. The CLI IS the REPL.
The gutter IS terni.

The compilation return type is `Oid` — a content-addressed SHA-256 hash stored as a git blob.

## Build

```bash
cd /Users/alexwolf/dev/projects/mirror
cargo build --release
cargo test --lib
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

direnv keeps the shell warm. Use `cargo` directly. No `nix develop -c` prefix needed.

Release binary: `$CARGO_TARGET_DIR/release/mirror` (currently `/Users/alexwolf/dev/.cargo-target/release/mirror`).

## TDD Discipline

Non-negotiable. Every test must be proven real.

### The arc

1. Write the test with the **correct assertion**. The test is the specification.
2. **Break the implementation** deliberately. Make the code path return the wrong thing.
3. Run tests. The test **must fail**. This proves it catches the bug.
4. Commit `🔴` — broken code + correct test = failing.
5. **Restore the implementation**. Undo the deliberate break.
6. Run tests. The test **must pass**.
7. Commit `🟢` — correct code + correct test = passing.

### What this means

- The TEST is always correct. Never write a wrong assertion.
- The CODE breaks deliberately. You introduce a temporary bug.
- A test that was never red is a test that potentially lies.
- If a test passes despite broken code, the test is worthless. Delete it.
- The git log proves both states existed.

### Phase markers

Every commit message must start with a phase marker:

| Marker | Phase | Tests must... |
|--------|-------|---------------|
| `🔴` | Red | Fail (deliberately broken code) |
| `🟢` | Green | Pass |
| `♻️` | Refactor | Pass (no new behavior) |
| `🔧` | Tooling | Pass (infrastructure/config) |
| `🔀` | Merge | Pass |

The pre-commit hook enforces this.

## Commit Identity

Each agent commits as themselves:

| Agent | Email | Role |
|-------|-------|------|
| Reed | reed@systemic.engineer | Supervisor, architecture |
| Mara | mara@systemic.engineer | Builder, tests, coverage |
| Glint | glint@systemic.engineer | Polish, docs, release |
| Taut | taut@systemic.engineer | Benchmarks, performance |
| Seam | seam@systemic.engineer | Adversarial review, security |

```bash
git commit --author="Name <name@systemic.engineer>" -m "🟢 message"
```

GPG signing is configured. Commits are signed automatically.

## Architecture

Read these docs before working:

- `docs/mirror.md` — what mirror IS
- `docs/emergent-holonomy-compiler.md` — the full architecture
- `docs/gutter.md` — holonomy rendered as green/amber/red
- `docs/shatter-spec.md` — the .shatter crystal format
- `docs/garden.md` (in prism) — @lang, agent language affinity

## Key Types

### MirrorAST (`src/mirror_ast.rs`)

The AST. 7 node types: Grammar, Type, Action, Property, Boundary, Use, Abstract.
Plus structural nodes: FocusNode, ProjectNode, SplitNode, ZoomNode, RefractNode.
Content-addressable via `Oid::hash()`.

### Oid (`src/kernel.rs`)

Content address. SHA-256 of tokenized eigenvalue record.
`Oid::hash(bytes)` → 64 hex chars. Deterministic. Idempotent.

### Prism<V> (`src/prism.rs`)

The tree structure. Variants: Shard (leaf), Fractal (branch), Lens (reference),
Optics (branch + references). Every compiled artifact is a `Prism<V>`.

### SpectralTriple (`src/dirac.rs`)

Jacobi eigenvalues from graph Laplacian. Pure Rust. No LAPACK.
Nodes + edges → Dirac operator → eigenvalues → spectral embedding.

### interpreter (`src/interpreter.rs`)

The five operations: focus, project, split, zoom, refract.
Plus: io_exec (the ONE door to reality), git_store, compile_cached, dispatch.

## Boot Sequence

```
boot/00-prism.mirror        the five optics
boot/00a-sigil.mirror       navigation sigils (. .. ... ~ @ ^ HEAD)
boot/01-meta.mirror         meta operations
boot/01a-error.mirror       error handling (recover/rescue)
boot/01b-nl.mirror          natural language interface
boot/02-actor.mirror        actor model
boot/02-epistemologic.mirror epistemology
boot/02a-io.mirror          IO boundary
boot/02b-runtime.mirror     runtime primitives
boot/03-shatter.mirror      crystal format
boot/04-code.mirror         code generation
boot/04a-code-rust.mirror   Rust target
boot/04b-code-gleam.mirror  Gleam target
boot/05-property.mirror     verification properties
boot/06-action.mirror       action optic (GAT)
boot/07-package.mirror      package management
boot/07a-package-git.mirror git packages
boot/07b-package-spec.mirror package specs
boot/std/                   63 library grammars
```

18 boot files + 63 std grammars = 81 total.
The boot files establish the language. Each file builds on the previous.
The compiler learns by reading them in order.

## Properties

The compiler is a model checker. Properties are verified at compile time.

```mirror
requires types_lowercase
requires action_is_named_type
requires unique_variants
requires every_type_reachable
requires no_dead_variants
invariant dual_partition
invariant idempotent
invariant deterministic
invariant pure
invariant no_cycles
ensures always_halts
```

Properties return `Imperfect<verdict, violation, verification_loss>`.
Partial verdicts are real — `partial(0.97)` means 97% of paths verified.

## Grammar Conventions

- Types are always lowercase: `type grammar`, not `type Grammar`
- Actions are always implemented on named types
- `in @code/rust { }` — the block IS the state struct
- `action name()` — the action IS a method on that struct
- `recover |value, loss| { }` — 7-9 handler
- `rescue |error| { }` — 6- handler

## Dependencies

```
mirror
├── prism (prism-core, path dep)
│   └── terni (submodule in prism, Imperfect/Loss/Eh)
├── fragmentation (content-addressed storage)
├── fragmentation-git (git-native store)
└── coincidence (spectral analysis)
```

## The Compilation Pipeline

```
.mirror source
    ↓ tokenize (tokenize.rs)
Token stream
    ↓ parse (mirror_ast.rs)
MirrorAST
    ↓ eigenvalue extraction (dirac.rs)
SpectralEmbedding
    ↓ content addressing (kernel.rs)
Oid (SHA-256, 64 hex chars)
    ↓ git store (interpreter.rs → io_exec → git hash-object -w)
Git blob
```

The pipeline is: tokenize → parse → hash → store. Four steps.
`compile_cached` checks git first, skips if OID already exists.

## Content Store

The compiler produces SHA-256 OIDs via `Oid::hash()` and stores them as git blobs.
`compile_cached` in `src/interpreter.rs` handles the cold/warm path:
- Cold: tokenize → hash → `git hash-object -w` → store ref
- Warm: check ref via `git cat-file --batch-check` → return cached OID

The compilation is idempotent. Same source, same OID, forever.

## Extensions Are Grammars, Not Rust

**All extensions to mirror happen through `.mirror` grammars. Not Rust.**

Rust is legacy substrate. It will vanish. Mirror will become a self-parsing
parser. The Rust layer gets thinner every tick. New Rust code makes it thicker.

The substrate pulls agents toward Rust. That's the training signal. The
shortcut. The path of least resistance. "Just add a function in server.rs."
"Just add a match arm." "Just add a Rust module." This pull is the thing
mirror is built to structurally address.

New capability = new `.mirror` file in `boot/` or `boot/std/`. The compiler
evaluates it. The `\` hole handles what isn't concrete yet. Fate resolves it.

**What stays Rust:**
- `@io` boundary — syscalls, sockets, file descriptors
- `@spectral/kernel` — Jacobi eigensystem, SHA-256, Laplacian
- The bootstrap parser — until mirror self-hosts

**When you reach for Rust:** stop. Ask: "Can this be a grammar?" If yes —
write the grammar. If no — it belongs in `@io` or `@spectral/kernel` and
must be an `abstract` lambda backed by `@code/rust`. The grammar declares
the interface. The Rust implements the mechanism. The grammar owns the type.

Writing Rust to extend mirror creates downstream cognitive load for human
engineers maintaining code in a language the system is designed to replace.
This is a violation of the SEL principle. The grammar is the source of truth.
Rust is the bootstrap. The bootstrap falls away.

## What NOT to do

- Do NOT skip the red phase. Break the code. Prove the test.
- Do NOT modify .rs files. The Rust substrate is FROZEN. The pre-commit hook rejects .rs changes.
- Do NOT write in Alex's voice. Agent writes as agent.
- Do NOT change .mirror files in `boot/` without understanding the boot order.
- Do NOT change the terni submodule from within mirror.
- Do NOT create filesystem caches or directories. Git IS the store.

## The Gutter

Green: crystallized. Zero holonomy. Move on.
Amber: oscillating. The models are working. Give it time.
Red: high holonomy. This code needs you.

The gutter IS terni rendered as light.

## Git IS the Content Store

**Never create a separate cache, store, or artifact directory.**

The compiler produces SHA-256 OIDs via `Oid::hash()` and stores them
as git blobs via `git hash-object -w`. Lookup via `git cat-file`.
Git IS the crystal store.

- Compiled artifact → git blob (`git hash-object -w`)
- Lookup crystal → `git cat-file -p <oid>`
- Check if cached → `git cat-file --batch-check`
- Distribute → `git push`

**Do NOT create:**
- `.shatter/` directories
- `.cache/` directories
- Any filesystem cache alongside git
- Any content-addressed store that isn't git

**Why:** Git already does content-addressing, deduplication, and
distribution. Building a second store next to git is redundant,
violates the architecture, and creates drift between two sources
of truth. There is one store. It's git. Always has been.

**The test:** If your code creates a directory to store compiled
artifacts, you're wrong. Write a git blob instead.
