# Inline Implementation + @runtime Grammar

**Date:** 2026-04-15
**Author:** Reed + Alex
**Status:** Plan

---

## The Insight

Grammars carry their own implementation inline via `in @code/rust { ... }` blocks.
The emit_code pipeline reads these blocks and generates Rust with real bodies.
No `todo!()`. Real code. The grammar IS the implementation.

---

## Three keywords, two purity levels

```mirror
abstract grammar @runtime {
  abstract template craft(input) => crystal     # pure, deterministic
  abstract template infer = ?                    # pure, delegates to a prism
  abstract action enact = ?                      # real, delegates to a prism
}
```

```
template   craft    pure    source → crystal (content-addressed, iso)
template   infer    pure    crystal → observation (eigenvalues, classification)
action     enact    real    crystal → effect (execution, reality crossing)
```

Craft is pure — content-addressed means same input = same output = same Oid.
Writing to the store isn't mutation — it's deduplication into an immutable tree.

`= ?` means: each concrete runtime fills in which prism it delegates to.

## @mirror — the compiler describes itself

```mirror
grammar @mirror < @runtime < @code/mirror {
  template craft(@mirror) => @shatter
  template infer = @fate
  action enact = @rust
}
```

Three assignments. Each one IS a prism:
- `craft(@mirror) => @shatter` — a lambda: mirror in, shatter out
- `infer = @fate` — IS Fate. The observation IS the eigenvalue analysis.
- `enact = @rust` — IS Rust. The execution IS the Rust runtime.

`= @X` is lambda assignment. This method IS that prism. Same Oid. Same identity.

---

## Task 1: @file grammar with inline Rust

**Create:** `boot/std/file.mirror`

```mirror
in @meta
in @io

type path = text

grammar @file(path) {
  io read(path) => imperfect in @code/rust {
    std::fs::read_to_string(path.as_str())
      .map(Imperfect::Success)
      .unwrap_or_else(|e| Imperfect::Failure(e.to_string(), IoLoss::zero()))
  }

  io write(path, content) => imperfect in @code/rust {
    std::fs::write(path.as_str(), content.as_bytes())
      .map(|_| Imperfect::Success(()))
      .unwrap_or_else(|e| Imperfect::Failure(e.to_string(), IoLoss::zero()))
  }

  io exists(path) => imperfect in @code/rust {
    Imperfect::Success(std::path::Path::new(path.as_str()).exists())
  }

  io mkdir(path) => imperfect in @code/rust {
    std::fs::create_dir_all(path.as_str())
      .map(|_| Imperfect::Success(()))
      .unwrap_or_else(|e| Imperfect::Failure(e.to_string(), IoLoss::zero()))
  }
}

out path
out @file
```

**Test:** Parser handles `in @code/rust { ... }` blocks — the body text IS the Rust code.
The parser already handles `body_text` on actions. Verify it captures the inline block.

---

## Task 2: @runtime abstract grammar

**Create:** `boot/std/runtime.mirror`

```mirror
in @meta
in @lambda

abstract grammar @runtime {
  abstract template craft(input) => crystal
  abstract template infer = ?
  abstract action enact = ?
}

out @runtime
```

**Test:** Compiles. `@runtime` has three declarations: two abstract templates, one abstract action.

---

## Task 3: @mirror self-description + @rust runtime

**Update:** `boot/std/mirror.mirror`

```mirror
grammar @mirror < @runtime < @code/mirror {
  template craft(@mirror) => @shatter
  template infer = @fate
  action enact = @rust
}
```

Three assignments. The compiler describes itself. `infer = @fate` IS Fate. `enact = @rust` IS Rust.

**Create:** `boot/std/rust.mirror`

```mirror
in @runtime
in @code/rust
in @file

grammar @rust < @runtime {
  template craft(source) => crystal in @code/rust {
    Parse.then(Resolve).then(Properties).then(Emit).reduce(source)
  }

  template infer(crystal) => observation in @code/rust {
    crystal.data().clone()
  }

  action enact(shard) => effect in @code/rust {
    std::process::Command::new(shard.path())
      .status()
      .map(|s| if s.success() { Imperfect::Success(()) } else { Imperfect::Failure("non-zero".into(), Loss::zero()) })
      .unwrap_or_else(|e| Imperfect::Failure(e.to_string(), Loss::zero()))
  }
}

out @rust
```

**Test:** Both compile. `@mirror < @runtime`. `@rust < @runtime`. Mirror delegates to Fate and Rust. Rust carries inline implementations.

---

## Task 4: emit_code reads inline blocks

**Modify:** `src/emit_code.rs` (when it exists from the other plan)

The emit_code pipeline needs to handle `body_text` on declarations.
When a declaration has `in @code/rust { body }`, the body IS the function body.
Instead of emitting `todo!()`, emit the body text.

```rust
fn emit_function(data: &MirrorData, grammar: &CodeGrammar) -> IoList {
    let body = if let Some(body_text) = &data.body_text {
        IoList::text(body_text)
    } else {
        IoList::text("todo!()")
    };

    IoList::join(vec![
        IoList::text("pub fn "),
        IoList::text(&grammar.templates.function_name(&data.name)),
        // ... params, return type ...
        IoList::text(" {\n"),
        body,
        IoList::text("\n}\n"),
    ])
}
```

**Test:** Emit a declaration with `body_text` — the body appears in the output.
Emit a declaration without `body_text` — `todo!()` appears.

---

## Task 5: Generate Rust with real implementations

**End-to-end test:**

```rust
#[test]
fn emit_file_grammar_has_real_bodies() {
    let runtime = MirrorRuntime::new();
    let source = std::fs::read_to_string("boot/std/file.mirror").unwrap();
    let compiled = runtime.compile_source(&source);
    let fragment = compiled.ok().unwrap();

    let rust = emit_code(&fragment, &CodeGrammar::rust());
    let output = rust.to_string_lossy();

    // The generated Rust should contain real bodies, not todo!()
    assert!(output.contains("read_to_string"), "read body should be inlined");
    assert!(!output.contains("todo!()"), "no todo — bodies are real");
}
```

---

## Task 6: Delete domain/filesystem.rs

`@file` replaces `domain/filesystem.rs`. The grammar carries the implementation.
The domain module had `Folder`, `read_tree`, `Setting`, `ContentAddressed`.
All replaced by `@file(path)` with inline Rust.

**Delete:** `src/domain/filesystem.rs`
**Modify:** `src/domain/mod.rs` — remove filesystem module
**Verify:** No callers remain (Mara's cleanup may have already removed them)

---

## Task 7: Wire @rust as the default runtime

The CLI currently uses `MirrorRuntime` directly. With `@rust < @runtime`, the CLI
should use the `@rust` grammar's methods:

```rust
// Before: hand-written
let result = runtime.compile_source(&source);

// After: grammar-defined
let result = Rust::craft(source);  // @rust.craft
```

This is the final step — the CLI dispatches through the grammar, not through
hand-written Rust. The grammar IS the implementation.

---

## Dependency order

```
Task 1: @file (no deps)
Task 2: @runtime (no deps)
Task 3: @rust (depends on 1 + 2)
Task 4: emit_code inline blocks (depends on emit_code pipeline)
Task 5: end-to-end test (depends on 3 + 4)
Task 6: delete filesystem.rs (depends on 5)
Task 7: wire CLI (depends on 5)
```

Tasks 1, 2, 4 can run in parallel.

---

## What this enables

- Adding a `@beam` runtime: write `boot/std/beam.mirror` with `in @code/gleam { ... }` blocks
- Adding a `@wasm` runtime: write `boot/std/wasm.mirror` with `in @code/wat { ... }` blocks
- Each runtime carries its own implementation in its own target language
- The emit_code pipeline generates each one from the grammar
- No hand-written Rust for runtime implementations
- The grammar IS the implementation IS the codegen IS the product
