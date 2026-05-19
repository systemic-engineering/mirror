# mirror

The glass is the grammar. The wine is what you bring.
The pitch is the eigenvalue. Neither alone. Both together.

---

## The Wine Glass

Tap a wine glass and it rings. The pitch depends on the glass — its shape,
its thickness, its material. Pour wine in and the pitch changes. Not because
the glass changed. Because the system changed. The glass and the wine together
produce a frequency that neither produces alone.

`mirror` is a compiler that works like this. You write a grammar (the glass).
You bring your code, your data, your topology (the wine). The compiler measures
what emerges (the pitch). The measurement is an eigenvalue — a mathematical
fingerprint of how the structure connects.

You don't need to know what an eigenvalue is. You just need to know that when
you tap the glass, the pitch tells you something true about what's inside.

---

## What It Does

Five commands. Five operations. Everything settles.

```
mirror compile <file>              tap the glass. get the pitch.
mirror craft <target>              compile a directory of grammars.
mirror kintsugi <file>             show the path: holes -> resolutions -> crystal.
mirror run <file>                  execute a grammar. measure the loss.
mirror run --fate-store <oid> <r>  seed a resolution into the Fate store.
```

Every compiled artifact is content-addressed. Same source, same pitch, forever.
Git is the content store. Always has been.

---

## The Five Operations

Everything in mirror is a prism. Five operations, five ways to interact
with the glass:

**focus** — narrow on the thing. Point the instrument. Get a reading.

**project** — carve a view. The graph is too much, so you take a slice.

**split** — hold multiple positions simultaneously. See from here and there.

**zoom** — cross between registers. From code to abstraction. From the thing
to the pattern of the thing.

**refract** — the geometry reflecting back. You made something and the
measurement shows you what you actually made. Not what you intended. What you
made.

---

## The Honest Hole

```mirror
abstract default = \
```

`\` means: "I don't know the pitch yet. The glass will tell me."

This is not a placeholder. It is honest uncertainty as a first-class value.
The compiler carries `\` through the pipeline. It doesn't guess. It doesn't
default to something convenient. It waits for the structure to disclose
the answer.

A grammar that contains `\` compiles. It just compiles with a hole where
certainty hasn't arrived yet. The hole is the specification.

Kintsugi resolves holes: `mirror run` shows them, Fate proposes resolutions,
`mirror run --fate-store` seeds a resolution, `mirror kintsugi` writes it
back into source. The gold in the cracks.

---

## Sub-Turing

A Turing-complete program cannot determine whether it will ever stop. You
can't prove what it does. You can only run it and watch. Seventy years of
patches on a foundation with a hole in it — type systems, linters, CI/CD,
formal verification bolted onto the side.

`mirror` is sub-Turing. The glass can prove what pitch it produces. Every
grammar terminates. Every property is decidable. The compiler is a model
checker. It doesn't just compile your code — it verifies it.

```mirror
invariant pure
invariant deterministic
invariant no_cycles
ensures always_halts
```

The glass holds because it can prove it holds.

---

## Architecture

Pure grammar. Zero code files. One bootstrap binary.

```
~/.local/bin/mirror    68KB arm64 binary (the bootstrap seed)
boot/                  18 boot files define the language
boot/std/              79 library grammars extend it
mirror.spec            the binary describes itself
prism/                 24 grammars (submodule — the prism ontology)
```

97 grammar files. 4,562 lines. Growing.
The bootstrap seed is the only non-mirror artifact.

The bootstrap implements exactly three things:
1. **Tokenizer** — state machine, no external deps
2. **SHA-256 + Jacobi** — pure computation, content addressing
3. **10 syscalls** — read/write/open/close/stat/readdir/spawn/pipe/waitpid/exit

Everything above the glass is grammar.

---

## The Grammar

18 boot files define the language. 79 std grammars extend it. The compiler
learns by reading them in order — optics, then meta, then actors, then IO,
then code generation, then verification, then packages.

`boot/` is the glass. `boot/std/` is the shelf of glasses above it.
New glass, not new machinery.

Key grammars at execution loss 0.00:
- `@cogito` — the Reflection loop (observe, strategy, perturb, reflect)
- `@mirror/craft` — the compiler compiles itself
- `@mirror/build` — collect, evaluate, emit, assemble, link, store
- `@kintsugi/shatter` — fracture IS the five operations
- `@code/llvm/emit` — LLVM IR emission from grammar

---

## The Kintsugi Workflow

```
mirror run <file>                    see the holes. measure the loss.
mirror run --fate-store <oid> <res>  seed a resolution.
mirror kintsugi <file>               write resolutions back into source.
git add + git commit                 the gold is in the cracks.
```

The compiler executes grammars with holes. `\` marks what isn't known yet.
Fate proposes resolutions through tournament selection (elite 1, beam 8,
halving 3). Kintsugi writes the gold back into the source file. Commit.

---

## Performance

Binary: 68KB (bootstrap seed, arm64).

```
mirror craft boot: 97 files, 95 cached, 2 recompiled.
Key grammars: all at execution loss 0.00.
```

---

## Build

The bootstrap seed lives at `~/.local/bin/mirror`. There is no build step
for users — run the binary against grammars. The compiler extends itself
through grammar, not through recompilation.

```bash
mirror craft boot     # compile all 97 grammar files
mirror run <file>     # execute a grammar, measure the loss
```

The grammar describes the compiler. The compiler executes the grammar.
The OIDs are deterministic. The compilation is idempotent.

The glass is Apache-2.0. The wine was always yours.

`e^(n+1) < e^n`
