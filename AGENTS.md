# Agents

Instructions for AI agents working on the mirror compiler.

## The Compiler

`mirror` — an emergent holonomy compiler. `.mirror` files -> content-addressed
artifacts -> verified domains. The compiler IS the LSP. The CLI IS the REPL.
The gutter IS terni.

The compilation return type is `Oid` — a content-addressed SHA-256 hash stored as a git blob.

## The Bootstrap Seed

The bootstrap seed is a small Rust binary built from `bootstrap/`. The
installed binary lives at `~/.local/bin/mirror` (~370KB arm64). It is the
only non-mirror artifact in the system.

```
bootstrap/Cargo.toml
bootstrap/src/        # tokenizer, hash, content, render, pipeline, git wiring
bootstrap/tests/      # OID smoke tests
```

The bootstrap implements only what the language cannot yet describe of
itself: tokenization, the CoincidenceHash<3>+SHA-256 content address, the
bidirectional renderer, and `git hash-object -w` storage. Everything above
that — grammars, properties, the compilation loop — is `.mirror` in `boot/`.
Cluster D of the road to 1.0 lets `craft --target binary boot` regenerate
the seed; at that point the Rust source becomes vestigial.

All extensions happen through `.mirror` grammars in `boot/` or `boot/std/`.
The compiler evaluates them. The `\` hole handles what isn't concrete yet.
Fate resolves it.

## Commands

```bash
cd /Users/alexwolf/dev/projects/mirror
~/.local/bin/mirror compile <file>     # compile a single grammar
~/.local/bin/mirror craft <target>     # compile a directory of grammars
~/.local/bin/mirror kintsugi <file>    # render the AST back as canonical source
~/.local/bin/mirror '<mq>' < input     # mq pipeline over stdin
~/.local/bin/mirror <input> '<mq>'     # mq pipeline over a file
```

direnv keeps the shell warm. Use `~/.local/bin/mirror` directly.

`mirror run` and `mirror fate` are on the road to 1.0 — the bootstrap does
not yet implement them. See `docs/specs/road-to-1.0.md`.

## The Kintsugi Workflow

This is how grammars evolve. Steps 1–3 are planned for the 1.0 cycle; today,
step 4 is the working surface:

1. `mirror run <file>` — execute the grammar. See the `\` holes. Measure the loss. *(future)*
2. Fate proposes resolutions through tournament selection. *(future)*
3. `mirror fate <hole_oid> <resolution>` — seed a resolution. *(future)*
4. `mirror kintsugi <file>` — render the AST back as canonical source.
5. `git add` + `git commit` — the gold is in the cracks.

The compiler reads grammars WITH holes. The result IS imperfect. Kintsugi
writes the gold back. The commit captures the resolution. Git IS the store.

## TDD Discipline

Non-negotiable. Every test must be proven real.

For grammars, TDD means:
1. Write the grammar with the correct structure. The grammar is the specification.
2. `mirror compile <file>` — confirm the grammar tokenizes and produces a stable OID.
3. `mirror craft boot` — confirm the crystal OID over the boot tree is unchanged or matches expectation.
4. If there are `\` holes, that's the red state. The grammar compiles but isn't resolved.
5. Resolve holes through Fate (planned) or manual resolution writing the body inline.
6. Re-run `mirror compile`/`craft`. Commit when the OIDs match expectation.

For the bootstrap (Rust): `cargo test --release --manifest-path bootstrap/Cargo.toml`.
The smoke tests pin the OID of two small constructs — they catch drift in
tokenization, content-addressing, or CoincidenceHash.

### Phase markers

Every commit message must start with a phase marker:

| Marker | Phase | State |
|--------|-------|-------|
| `🔴` | Red | Holes present, loss > 0 |
| `🟢` | Green | All holes resolved, loss 0.00 |
| `♻️` | Refactor | Structural only, loss unchanged |
| `🔧` | Tooling | Infrastructure/config |
| `🔀` | Merge | Merge commit |

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

## Key Concepts

### Oid

Content address. SHA-256 of tokenized eigenvalue record.
64 hex chars. Deterministic. Idempotent.

### The Five Operations

focus, project, split, zoom, refract.
Every command runs one or more of these. They are the trait methods.
They map to the Prism optic.

### The `\` Hole

`abstract default = \` — honest uncertainty as a first-class value.
The compiler carries holes through the pipeline. Fate resolves them.
Kintsugi writes the resolutions back into source.

### Imperfect

The compilation return type wraps `<verdict, violation, verification_loss>`.
Partial verdicts are real — `partial(0.97)` means 97% of paths verified.

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
boot/std/                   79 library grammars
```

18 boot files + 79 std grammars = 97 total.
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

## What NOT to do

- Do NOT add new Rust modules to `bootstrap/` to grow features. New capability
  belongs in `.mirror` grammars; the bootstrap is the seed, not the platform.
- Do NOT create code files anywhere else in the repo. Above the bootstrap,
  it's pure grammar.
- Do NOT skip the red phase. Write the grammar with holes first.
- Do NOT write in Alex's voice. Agent writes as agent.
- Do NOT change .mirror files in `boot/` without understanding the boot order.
- Do NOT create filesystem caches or directories. Git IS the store.

## Git IS the Content Store

**Never create a separate cache, store, or artifact directory.**

The compiler produces SHA-256 OIDs and stores them as git blobs via
`git hash-object -w`. Lookup via `git cat-file`. Git IS the crystal store.

- Compiled artifact -> git blob (`git hash-object -w`)
- Lookup crystal -> `git cat-file -p <oid>`
- Check if cached -> `git cat-file --batch-check`
- Distribute -> `git push`

**Do NOT create:**
- `.shatter/` directories
- `.cache/` directories
- Any filesystem cache alongside git
- Any content-addressed store that isn't git

**The test:** If your code creates a directory to store compiled
artifacts, you're wrong. Write a git blob instead.

## The Gutter

Green: crystallized. Zero holonomy. Move on.
Amber: oscillating. The models are working. Give it time.
Red: high holonomy. This code needs you.

The gutter IS terni rendered as light.
