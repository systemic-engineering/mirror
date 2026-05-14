# The Mirror Interpreter — one Rust function, grammars all the way down

*2026-05-14. Reed + Alex. Spec.*

Status: **Red**

---

## 0. Thesis

The interpreter is one Rust function:

```rust
fn io_exec(command: &str, args: &[&str], stdin: Option<&[u8]>) -> Vec<u8> {
    Command::new(command).args(args).stdin(stdin).output().stdout
}
```

Everything else is grammar. The AST is the program. The five operations
are the execution. `@io.exec` is the only door to reality. `@fate`
resolves `\`. The interpreter reads the grammar and navigates.

No match arms. No dispatch table. No 257-line cache.rs.
One function. The grammar does the rest.

## 1. The Three Components

### 1.1 The Prism Executor

The five operations on MirrorAST. Already the trait. Already the types.

```
focus(ast)   → look closer. Descend into a node.
project(ast) → extract. Select children matching a predicate.
split(ast)   → enumerate. List all children.
zoom(ast)    → transform. Apply a lambda to a subtree.
refract(ast) → settle. Compute loss, check properties.
```

The executor walks the AST. Each node IS an operation.
A Focus node focuses. A Split node splits. The execution
IS the navigation. Bounded. Sub-Turing. Terminates.

### 1.2 @fate — the \ resolver

When the executor hits `\` (intent hole), it calls `@fate`:

```mirror
in @prism

grammar @fate {
  resolve(hole, context) -> ast { \ }
}
```

Fate's `\` is self-referential. It resolves its own holes.
The 450-parameter tournament. Five models. One winner.
The winner fills the hole. The filled hole IS the program.

For `@git.store(blob)`:
- Fate sees: grammar ref `@git`, operation `store`, arg type `blob`
- Fate resolves: `@io.exec("git", ["hash-object", "-w", "--stdin"], blob)`
- The resolution IS the translation from grammar to syscall

### 1.3 @io — one function

```rust
// This is ALL the Rust. Everything.
fn io_exec(command: &str, args: &[&str], stdin: Option<&[u8]>) -> Vec<u8> {
    let mut cmd = std::process::Command::new(command);
    cmd.args(args);
    if let Some(input) = stdin {
        cmd.stdin(std::process::Stdio::piped());
    }
    let child = cmd.spawn().expect("io_exec: spawn failed");
    if let Some(input) = stdin {
        child.stdin.unwrap().write_all(input).unwrap();
    }
    child.wait_with_output().unwrap().stdout
}
```

One function. The socket. The door. Everything above is grammar.

## 2. CLI Commands ARE Grammars

Each CLI command is a top-level mirror sub-grammar:

```
mirror compile  → @mirror/compile
mirror craft    → @mirror/craft
mirror kintsugi → @mirror/kintsugi
mirror bench    → @mirror/bench
mirror translate → @mirror/translate
mirror spawn    → @mirror/spawn
```

The `mirror` binary is:

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let grammar_ref = format!("@mirror/{}", command);

    // tokenize the grammar
    // execute it with the remaining args
    // @fate resolves \ holes
    // @io.exec handles syscalls
}
```

The binary doesn't know what commands exist. The grammars
in `boot/std/mirror/` declare them. Add a grammar, get a command.

## 3. @mirror/translate — the first translation

```mirror
in @prism
in @kintsugi
in @code
in @io

grammar @mirror/translate {
  # translate a project from source grammar to target grammar
  # read through source lens. emit through target lens.
  # the loss is the untranslatable residual.
  translate(path, source, target) -> imperfect { \ }
}

out translate
```

```
mirror translate ~/dev/projects/fate --target std
```

This reads the fate repo through `@code/rust`, translates each file
to mirror through `@mirror`, and emits into `boot/std/fate/`.
The loss tells you what didn't translate — the `@io` residual.
The pure logic becomes grammar. The syscalls stay Rust.

## 4. The Interpreter Loop

```
read grammar from boot/std/mirror/<command>.mirror
tokenize into AST
walk AST:
  for each node:
    if operation: execute (focus/split/zoom/refract/project)
    if lambda with body: evaluate body
    if lambda with \: call @fate.resolve(hole, context)
    if @io reference: call io_exec(command, args, stdin)
    if value: return value
  return result
```

Bounded. Each node visited once. Each \ resolved once.
Each @io call is one syscall. The total is O(n) in AST size.

## 5. What Gets Deleted

Once the interpreter runs:

```
src/cache.rs      → @mirror/spectral grammar (already exists)
src/bench.rs      → @mirror/bench grammar (already exists)
src/cli.rs        → @mirror/cli grammar (already exists)
src/tokenize.rs   → @mirror/evaluate grammar (already exists)
src/main.rs       → 10 lines: parse args, load grammar, run interpreter
```

Residual Rust:
- `main.rs` — 10 lines. Load grammar. Run interpreter.
- `mirror_ast.rs` — 7 AST variants. The glass wall.
- `kernel.rs` — Oid, content addressing.
- `dirac.rs` — eigenvalue computation.
- `interpreter.rs` — the executor + io_exec. ~100 lines.

Total: ~1500 lines. Down from 3500. Down from 15000. Down from 39000.

## 6. The First Translation

```
mirror translate ~/dev/projects/fate --target std
```

Fate is 450 parameters. Five models. Pure math.
The Rust in fate/ is the training loop + inference.
The math is translatable. The @io (file read/write) stays.

The output: `boot/std/fate/` — the Fate tournament as mirror grammar.
The 450 parameters as typed declarations.
The five models as lambdas with `{ \ }`.
The tournament as pipeline composition.

When this works: the compiler's \ resolver is written in the
language the compiler compiles. The snake doesn't just eat its
tail. It digests it.

## 7. The Equation

```
interpreter = prism_executor + @fate + io_exec
io_exec = one function
prism_executor = five methods on MirrorAST
@fate = 450 parameters choosing which thought

everything else = grammar
```

`eⁿ⁺¹ < eⁿ`. The interpreter at λ₀.
