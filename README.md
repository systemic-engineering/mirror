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

```
mirror compile <file>       tap the glass. get the pitch.
mirror craft <target>       compile a directory of grammars
mirror kintsugi <file>      show the path: in -> grammar -> out
mirror bench <file|dir>     measure the resonance (100 iterations)
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

10 Rust files. 6,181 lines. FROZEN.
81 grammar files. 3,289 lines. Growing.
198 tests. All passing.

```
src/
  tokenize.rs     1,212 lines   tokenizer (state machine)
  mirror_ast.rs   1,319 lines   AST (7 node types)
  kernel.rs         856 lines   Oid, SHA-256, content addressing
  dirac.rs          928 lines   Jacobi eigenvalues, spectral embedding
  interpreter.rs    576 lines   five operations (focus/project/split/zoom/refract)
  prism.rs          435 lines   Prism<V> tree structure
  bench.rs          522 lines   benchmarking harness
  cli.rs            270 lines   command dispatch
  lib.rs             37 lines   crate root
  main.rs            26 lines   entry point
```

The Rust is the bootstrap. It implements exactly three things:
1. **Tokenizer** — state machine, no external deps
2. **SHA-256 + Jacobi** — pure computation, content addressing
3. **10 syscalls** — read/write/open/close/stat/readdir/spawn/pipe/waitpid/exit

Everything above the glass is grammar.

---

## The Grammar

18 boot files define the language. 63 std grammars extend it. The compiler
learns by reading them in order — optics, then meta, then actors, then IO,
then code generation, then verification, then packages.

`boot/` is the glass. `std/` is the shelf of glasses above it.
New glass, not new machinery.

---

## Performance

Binary: 717 KB release, 591 KB stripped.

| File | Compile Time | Throughput |
|------|-------------|------------|
| kintsugi.mirror (tiny) | 1.3ms | 778 ops/s |
| 00-prism.mirror (small) | 2.4ms | 423 ops/s |
| kernel.rs (dense) | 75.0ms | 13 ops/s |
| **craft boot** (81 files) | **1.42s wall** | 80/81 cached |
| **craft cargo** (10 files) | **1.22s wall** | cold compile |

Full numbers: `docs/benchmarks/baseline-rust.md`

---

## The Path to 51KB

The thinnest glass that still holds wine.

```
mirror = libc + pure computation
       = 10 syscalls + SHA-256 + SHA-1 + Jacobi eigenvalues
       = @io + @hash + @eigen

Three abstract operations. Everything else is grammar.
```

Target architecture:
```
Grammar -> mirror emit -> LLVM IR -> llc -> ~51KB binary
```

No LAPACK. No OpenSSL. No libgit2. No Rust runtime. Just libc and math.
450 parameters. A monofilament. Not enough glass to impose a frequency.
Just enough to give your wine a shape.

See `docs/specs/minimum-binary-surface.md` for the full analysis.

---

## Build & Proof

```bash
cargo build --release && cargo test --lib
```

The compiler compiles itself:

```
mirror craft cargo    # tokenizes + hashes all 10 source files
mirror craft boot     # compiles all 81 grammar files
```

The grammar describes the compiler. The compiler executes the grammar.
The OIDs are deterministic. The compilation is idempotent.

The glass is Apache-2.0. The wine was always yours.

`e^(n+1) < e^n`
