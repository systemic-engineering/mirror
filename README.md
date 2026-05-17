# mirror

The compiler that compiles itself.

10 Rust files. 6,181 lines. FROZEN.
81 grammar files. 3,289 lines. Growing.
198 tests. All passing.

```
mirror compile boot/std/kintsugi.mirror
5b4178705fe449cc95b08e26cd2665c3ce3aea9562e82a5aa9a20d80cfef23b8
```

## What It Is

An emergent holonomy compiler. `.mirror` files in, content-addressed OIDs out.
The Rust substrate is frozen — all extension happens through grammar.
The compiler reads its own source as grammar. It compiles itself.

## Commands

```
mirror compile <file>       compile to content-addressed OID (git blob)
mirror craft <target>       compile all .mirror files in a directory tree
mirror kintsugi <file>      show the prism path: in → grammar → out
mirror bench <file|dir>     benchmark tokenize + hash (100 iterations)
```

## Architecture

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

Everything above is grammar.

## The Grammar

```
boot/
  00-prism.mirror        the five optics
  00a-sigil.mirror       navigation sigils (. .. ... ~ @ ^ HEAD)
  01-meta.mirror         meta operations
  01a-error.mirror       error handling (recover/rescue)
  01b-nl.mirror          natural language interface
  02-actor.mirror        actor model
  02-epistemologic.mirror epistemology
  02a-io.mirror          IO boundary (@io)
  02b-runtime.mirror     runtime primitives
  03-shatter.mirror      crystal format
  04-code.mirror         code generation
  04a-code-rust.mirror   Rust target
  04b-code-gleam.mirror  Gleam target
  05-property.mirror     verification properties
  06-action.mirror       action optic (GAT)
  07-package.mirror      package management
  07a-package-git.mirror git packages
  07b-package-spec.mirror package specs
  std/                   63 library grammars
```

The boot files ARE the language. The compiler learns by reading them in order.
`std/` extends without touching Rust.

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

## The Path to 50KB

The minimum binary needs only libc. No LAPACK. No OpenSSL. No libgit2.

```
mirror = libc + pure computation
       = 10 syscalls + SHA-256 + SHA-1 + Jacobi eigenvalues
       = @io + @hash + @eigen

Three abstract operations. Everything else is grammar.
```

Target architecture:
```
Grammar → mirror emit → LLVM IR → llc → ~50KB binary
```

The current Rust binary is the proof-of-concept. The LLVM binary is the product.
See `docs/specs/minimum-binary-surface.md` for the full analysis.

## Content Addressing

Every compiled artifact is a git blob. No filesystem cache. No `.shatter/` directory.

```
compile:  source → tokenize → AST → eigenvalues → SHA-256 → OID
store:    OID → git hash-object -w
lookup:   OID → git cat-file --batch-check
```

Git IS the content store. Always has been.

## Build

```bash
cargo build --release
cargo test --lib
```

The release binary lands at `$CARGO_TARGET_DIR/release/mirror`.

## The Proof

The compiler compiles itself:

```
mirror craft cargo    # tokenizes + hashes all 10 source files
mirror craft boot     # compiles all 81 grammar files
```

The grammar describes the compiler. The compiler executes the grammar.
The OIDs are deterministic. The compilation is idempotent.

`e^(n+1) < e^n` — the errors get smaller. The growth is monotonically non-decreasing.
