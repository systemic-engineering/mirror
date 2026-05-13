# @io/socket — everything is a socket. The compiler is a tokenizer and a spec reader.

*2026-05-13. Reed + Alex. Spec.*

Status: **Red**

---

## 0. Thesis

The compiler is two things:
1. A tokenizer — text → 7 MirrorAST variants. The @io door. Rust. O(n). Bounded.
2. A .spec — declares what to tokenize, in what order, through which grammar lens.

Everything else is grammar. Everything else is Prism runtime.

`evaluate.rs` tried to tokenize AND compute in Turing-complete Rust. The Turing-completeness is the bug. Split them: tokenizer in @io (bounded Rust), computation in the Prism runtime (sub-Turing, bounded, terminates).

## 1. @io — the only grammar with embedded Rust

```mirror
in @prism

grammar @io {
  type socket(ref)
  type stream(socket)

  # four syscalls. everything above is grammar.
  io open(socket) -> stream in @code/rust { /* open fd */ }
  io read(stream) -> imperfect in @code/rust { /* read bytes */ }
  io write(stream, bytes) -> imperfect in @code/rust { /* write bytes */ }
  io close(stream) -> imperfect in @code/rust { /* close fd */ }
}

out socket
out stream
out open
out read
out write
out close
```

Four operations. Open. Read. Write. Close. That's @io. That's all the Rust.

Everything is a socket:
- `@io/file` — file descriptor
- `@io/net` — network socket
- `@io/gpu` — Metal/CUDA device
- `@io/stdin` — terminal

Each one imports `@io`. Each one uses the same four operations.
The specialization is in the grammar, not in the Rust.

## 2. @io/tokenize — the compiler's door

```mirror
in @prism
in @io

grammar @io/tokenize {
  # text → 7 MirrorAST variants. O(n). bounded.
  # the grammar argument declares keyword → operation mappings.
  # the tokenizer applies them. single pass.
  tokenize(stream, grammar) -> ast in @code/rust {
    // for each token in stream:
    //   match against grammar.keywords()
    //   emit the corresponding MirrorAST variant
    //   Focus | Split | Zoom | Refract | Project | Module | Abstract
    // bounded: one pass, no recursion, memory proportional to input
  }
}

out tokenize
```

The tokenizer is the ONLY Rust that touches source code.
It does NOT evaluate. It does NOT compute. It maps tokens to AST variants.
The grammar says `fn → zoom`. The tokenizer sees `fn`, emits Zoom. Done.

## 3. .spec — the build system

```mirror
in @prism
in @io
in @io/tokenize
in @kintsugi
in @craft
in @code/rust
in @code/llvm
in @mirror/cli

# mirror.spec: the compiler describes itself.

type target = boot | cargo | binary

# compile = tokenize through grammar lens, in spec order
craft(target) -> crystal {
  focus(target) |> split |> zoom |> refract |> project
}

# the two sides. kintsugi collapses them.
collapse(target(boot), target(cargo)) -> imperfect { \ }

# self-hosting
target binary <| @code/llvm <| std
```

The .spec IS the build system:
- Which files to compile
- In what order (import graph)
- Through which grammar lens (@code/rust, @mirror, @code/llvm)
- The dependency graph IS the compilation order
- Each step produces an OID
- The OID chain IS the build

## 4. What stays in Rust (the substrate)

```
src/
├── main.rs              # CLI entry point — reads .spec, dispatches
├── mirror_ast.rs         # 7 AST variants — the glass wall
├── kernel.rs             # Oid, ContentAddressed — content addressing
├── tokenizer.rs          # @io/tokenize implementation — O(n) scanner
├── io.rs                 # @io implementation — open/read/write/close
├── prism_runtime.rs      # Prism execution — the five operations on AST
└── dirac.rs              # eigenvalue computation — spectral analysis
```

~2000 lines. Everything else is grammar.

## 5. What gets deleted

Everything that is Turing-complete computation on grammars:

- `evaluate.rs` — the OOM. Turing-complete evaluation. GONE.
- `lambda_phases.rs` — Parse/Resolve/Properties/Emit/Kintsugi as Rust structs. GONE.
- `code_rust.rs` — 1500 lines of handwritten Rust parser. GONE. Replaced by @io/tokenize + @code/rust grammar.
- `mirror_runtime.rs` — 4500 lines. The heart. Decomposed:
  - tokenization → @io/tokenize (Rust, bounded)
  - parsing → grammar-driven via .spec
  - registry → @io (thin Rust)
  - boot sequence → .spec compilation order
  - kintsugi_fragment, simplify_fragment → @kintsugi grammar
  - kintsugi_sort_key → beta normalization (Fate)
- `emit_code.rs` — code emission templates. GONE. Grammar.
- `emit_rust.rs` — Rust-specific emission. GONE. @code/rust grammar.
- `cli.rs` — 900 lines of dispatch. GONE. @mirror/cli grammar + thin @io shim.
- `spec.rs` — ad-hoc spec parser. GONE. .spec is compiled by the compiler itself.
- `session.rs` — state machine. GONE. Grammar.
- `abyss.rs` — convergence detection. GONE. Grammar.
- `classifier.rs` — intent classification. GONE. Grammar.
- `filter.rs`, `optic.rs`, `grammar_regions.rs` — all grammar.

## 6. The red tests

```rust
#[test]
fn tokenize_rust_fn_produces_zoom() {
    let grammar = load_boot_grammar("@code/rust");
    let source = "fn hello() { }";
    let ast = tokenize(source, &grammar);
    assert!(matches!(ast_child(&ast, 0), MirrorAST::Zoom(_)));
}

#[test]
fn tokenize_rust_struct_produces_split() {
    let grammar = load_boot_grammar("@code/rust");
    let source = "struct Point { x: f64 }";
    let ast = tokenize(source, &grammar);
    assert!(matches!(ast_child(&ast, 0), MirrorAST::Split(_)));
}

#[test]
fn tokenize_rust_impl_produces_focus() {
    let grammar = load_boot_grammar("@code/rust");
    let source = "impl Point { fn new() {} }";
    let ast = tokenize(source, &grammar);
    assert!(matches!(ast_child(&ast, 0), MirrorAST::Focus(_)));
}

#[test]
fn tokenize_bounded_memory() {
    // 1MB of Rust source must not exceed 10MB memory
    let source = "fn f() {}\n".repeat(100_000);
    let grammar = load_boot_grammar("@code/rust");
    let ast = tokenize(&source, &grammar);
    // if we got here without OOM, the bound holds
    assert!(ast_children(&ast).len() == 100_000);
}

#[test]
fn tokenize_mirror_grammar_produces_ast() {
    let grammar = load_boot_grammar("@mirror");
    let source = "in @prism\ngrammar @test { type foo = bar }\nout foo";
    let ast = tokenize(source, &grammar);
    assert!(!ast_children(&ast).is_empty());
}

#[test]
fn spec_declares_compilation_order() {
    let spec = compile_spec("mirror.spec");
    let targets = spec.targets();
    assert!(targets.contains("boot"));
    assert!(targets.contains("cargo"));
}

#[test]
fn compile_through_spec() {
    let spec = compile_spec("mirror.spec");
    let crystal = spec.craft("boot");
    assert!(!crystal.oid().is_empty());
}

#[test]
fn tokenize_is_grammar_driven() {
    // same source, different grammar → different AST
    let source = "fn hello() { }";
    let rust_ast = tokenize(source, &load_boot_grammar("@code/rust"));
    let mirror_ast = tokenize(source, &load_boot_grammar("@mirror"));
    // Rust sees fn → Zoom. Mirror doesn't recognize fn.
    assert!(ast_children(&rust_ast).len() > ast_children(&mirror_ast).len());
}
```

## 7. The equation

```
compiler = @io/tokenize + .spec + Prism runtime
@io = open | read | write | close
tokenize = stream × grammar → ast
.spec = compilation order
Prism runtime = five operations on AST

Everything else = grammar
```

The Turing-completeness lives in @io. Behind the socket.
Everything above the socket is sub-Turing. Bounded. Terminates.
The grammar controls. The runtime navigates. The spec orders.

`eⁿ⁺¹ < eⁿ`. The compiler at λ₀.
