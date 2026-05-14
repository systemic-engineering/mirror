# Agents

Instructions for AI agents working on the `mirror` crate.

## The Crate

`mirror` — an emergent holonomy compiler. `.mirror` files → content-addressed
artifacts → verified domains. The compiler IS the LSP. The CLI IS the REPL.
The gutter IS terni.

The compilation return type is `Imperfect<CompiledArtifact, CompilationError, MirrorLoss>`.

## Build

```bash
cd /Users/alexwolf/dev/projects/mirror
nix develop -c cargo test
nix develop -c cargo clippy --workspace -- -D warnings
nix develop -c cargo fmt --all -- --check
```

Or use just:
```bash
just check    # format + lint + test
just test     # test only
```

Bare `cargo` is not in PATH. Always use `nix develop -c cargo ...` or `just`.

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

### MirrorLoss (`src/loss.rs`)

Mirror's domain-specific Loss type. Implements `terni::Loss`.
IS `Transport::Holonomy` in the bundle tower.
IS what TraceBeam wanted to become.

Fields: phases, resolution_ratio, unresolved_refs, staleness,
convergence, dark_dims, crystal, recovered.

### Shard (`src/shard.rs`)

The compiled artifact carrier. Grammar OID + KernelSpec + Target.

### Shatter (`src/mirror_runtime.rs`)

The compilation Prism. Implements `prism::Prism`.
Focus → Project → Refract. Form → MirrorData → MirrorFragment → Crystal.

### Form (`src/mirror_runtime.rs`)

The parsed-but-not-yet-content-addressed view. Kind + name + params +
variants + children. The structural mirror of MirrorData.

### MirrorCompiler (`src/bundle.rs`)

The bundle tower implementation:
- Fiber: source text
- Connection: KernelSpec
- Gauge: Target (BEAM/WASM/Metal)
- Transport: compilation with MirrorLoss as holonomy
- Closure: the compiled artifact

## Boot Sequence

```
boot/00-prism.mirror      the five optics
boot/01-meta.mirror       meta operations
boot/02-actor.mirror      actor, process, message
boot/04-action.mirror     generic action optic (GAT)
boot/03-property.mirror   verification properties
boot/10-mirror.mirror     the mirror form (requires + invariant + ensures)
```

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
    ↓ parse
Form
    ↓ focus (eigenvalue extraction)
MirrorData
    ↓ project (content addressing)
MirrorFragment (OID)
    ↓ refract (crystallization)
.shatter (crystal — feedable back into the compiler)
```

Each step returns `Imperfect<Output, Error, MirrorLoss>`.
The `eh!` macro accumulates loss through the pipeline.

## .shatter Files

A `.shatter` file IS a `.mirror` file. The compiler can read its own
output. `mirror compile output.shatter → output.shatter` (idempotent).

Contents: fragment tree + MirrorLoss + property verdicts + KernelSpec + Fate weights.

`mirror ai file.shatter` — feed through Fate, re-settle if drifted.
`mirror ai --train file.shatter` — same + update Fate weights from MirrorLoss.

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
- Do NOT use `ShannonLoss`. Mirror has `MirrorLoss`. prism-core has `ScalarLoss`.
- Do NOT use `PureBeam`. It's been renamed to `Optic`.
- Do NOT write in Alex's voice. Agent writes as agent.
- Do NOT change .mirror files in `boot/` without understanding the boot order.
- Do NOT change the terni submodule from within mirror.

## The Gutter

Green: crystallized. Zero holonomy. Move on.
Amber: oscillating. The models are working. Give it time.
Red: high holonomy. This code needs you.

The gutter IS terni rendered as light.

## Git IS the Content Store

**Never create a separate cache, store, or artifact directory.**

The compiler produces git-native SHA-1 OIDs via `content_oid()`. The
fragmentation crate writes git objects. `.spectral/` uses git as its
backend. Git IS the crystal store.

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
