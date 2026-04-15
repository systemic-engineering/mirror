# @code Emit Pipeline — Mirror Compiler Changes

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `ast(grammar)` to `@meta`, `template` + `translate` as first-class declarations, and a generic `emit_code.rs` that applies grammar templates. Delete `emit_rust.rs`.

**Architecture:** `@meta` defines `ast(g)` — the universal AST parameterized by grammar. Two primitives: lambda (params -> body) and split (variants). Block is lambda with no params. `@code` defines abstract `translate` and `render` templates. Each `@code/X` implements them. `emit_code.rs` is a thin dispatcher that applies the grammar's own templates.

**Tech Stack:** Rust, mirror grammar, prism-core (Imperfect, Oid, Loss, Lambda)

**Working directory:** `/Users/alexwolf/dev/projects/mirror/`
**Branch:** `reed/emit-code`
**Build:** `nix develop -c cargo test`

---

## Design: Two Primitives

Everything is lambda or split.

```
lambda(params, body) = params -> body
block(statements)    = lambda((), statements)    -- lambda with no params

function(name, params, body) = name = lambda(params, body)
module(name, children)       = name = lambda((), children)
struct(name, fields)         = name = lambda(fields, ())  -- lambda with no body
enum(name, variants)         = name = |(variants)         -- split
```

Translation between grammars is a typed lambda:

```mirror
template translate(p: @prism, c: @code, p -> c)
```

The type annotation IS the constraint. `c: @code` means c must implement @code's abstracts. No `where` clauses needed.

---

## File Structure

```
mirror/
├── boot/
│   ├── 01-meta.mirror        — MODIFY: add ast(g) type, template declarations
│   ├── 03-code.mirror        — MODIFY: add abstract translate + render templates
│   ├── 03a-code-rust.mirror  — MODIFY: implement translate + render for Rust
│   └── 03b-code-gleam.mirror — CREATE: implement translate + render for Gleam
├── src/
│   ├── declaration.rs        — MODIFY: add DeclKind::Template
│   ├── emit_code.rs          — CREATE: IoList + generic emitter
│   ├── emit_rust.rs          — DELETE (final task)
│   ├── lib.rs                — MODIFY: add pub mod emit_code
│   └── cli.rs                — MODIFY: wire --target to emit_code
```

---

### Task 1: ast(g) in @meta

**Files:**
- Modify: `boot/01-meta.mirror`

- [ ] **Step 1: Add ast type to 01-meta.mirror**

Append after the existing declarations (after `type abstract(action)`):

```mirror
-- The universal AST parameterized by grammar.
-- Two primitives: lambda and split. Everything else is named combinations.
type ast(g)

-- Expressions: lambda IS block (block = lambda with no params)
type expression(g) = literal(g) | reference(g) | call(g) | lambda(g) | binding(g) | conditional(g)
type literal(g) = int_lit(i64) | float_lit(f64) | string_lit(text) | bool_lit(bool) | atom_lit(ref)
type reference(g) { name: ref }
type call(g) { target: ast(g), args: [ast(g)] }
type lambda(g) { params: [pattern(g)], body: ast(g) }
type binding(g) { pattern: pattern(g), value: ast(g), body: ast(g) }
type conditional(g) { scrutinee: ast(g), arms: [arm(g)] }
type arm(g) { pattern: pattern(g), body: ast(g) }

-- Declarations
type declaration(g) = function_decl(g) | type_decl(g) | module_decl(g) | import_decl(g) | comment_decl
type function_decl(g) { name: ref, params: [param(g)], return_type: type_ref(g), body: ast(g) }
type type_decl(g) { name: ref, params: [ref], body: type_body(g) }
type type_body(g) = enum_body(g) | struct_body(g) | alias_body(g) | unit_body
type enum_body(g) { variants: [variant(g)] }
type variant(g) { name: ref, fields: [field(g)] }
type struct_body(g) { fields: [field(g)] }
type field(g) { name: ref, type_ref: type_ref(g) }
type alias_body(g) { target: type_ref(g) }
type unit_body
type module_decl(g) { name: ref, declarations: [declaration(g)] }
type import_decl { path: ref, alias: option(ref) }
type comment_decl { text: text }
type param(g) { name: ref, type_ref: type_ref(g) }

-- Patterns
type pattern(g) = bind_pattern | constructor_pattern(g) | wildcard_pattern
type bind_pattern { name: ref }
type constructor_pattern(g) { name: ref, fields: [pattern(g)] }
type wildcard_pattern

-- Type references
type type_ref(g) { name: ref, args: [type_ref(g)] }
type function_type(g) { params: [type_ref(g)], return_type: type_ref(g) }
```

- [ ] **Step 2: Add out declarations**

```mirror
out ast
out expression
out declaration
out pattern
out type_ref
```

- [ ] **Step 3: Verify grammar compiles**

Run: `nix develop -c cargo test`
Expected: existing tests still pass (parser should handle new type declarations)

- [ ] **Step 4: Commit**

```bash
git checkout -b reed/emit-code
git add boot/01-meta.mirror
git commit -m "🟢 @meta: ast(g) — universal AST parameterized by grammar, two primitives"
```

---

### Task 2: DeclKind::Template in the parser

**Files:**
- Modify: `src/declaration.rs`

- [ ] **Step 1: Add Template to DeclKind**

```rust
// In the DeclKind enum, add:
Template,
```

```rust
// In DeclKind::parse(), add:
"template" => Some(DeclKind::Template),
```

```rust
// In DeclKind::as_str(), add:
DeclKind::Template => "template",
```

- [ ] **Step 2: Run tests — pass**

Run: `nix develop -c cargo test`
Expected: all existing tests pass (Template is just a new variant, no parser changes yet)

- [ ] **Step 3: Commit**

```bash
git add src/declaration.rs
git commit -m "🟢 parser: DeclKind::Template — template as first-class declaration kind"
```

---

### Task 3: template and translate in @code grammar

**Files:**
- Modify: `boot/03-code.mirror`

- [ ] **Step 1: Add abstract templates to 03-code.mirror**

Replace the current content with:

```mirror
in @prism
in @meta

type position {
  line: u32,
  character: u32,
}

type range {
  start: position,
  end: position,
}

type severity = error | warning | info | hint

type diagnostic {
  range: range,
  severity: severity,
  message: string,
}

type completion {
  label: string,
  kind: type,
}

type token {
  range: range,
  kind: type,
}

abstract grammar @code {
  -- The translate template: map from one grammar's ast to another's.
  -- The type annotation IS the constraint. c: @code means c in @code.
  abstract template translate(p: @prism, c: @code, p -> c)

  -- Render: ast to io_list. The terminal template.
  abstract template render(g: @code, ast(g) -> io_list)

  -- Naming conventions
  abstract template map_type(ref) -> type_ref
  abstract template type_name(ref) -> ref
  abstract template field_name(ref) -> ref
  abstract template function_name(ref) -> ref
  abstract template module_name(ref) -> ref
  abstract template variant_name(ref) -> ref

  -- Structural templates with defaults
  template emit_comment(text) -> io_list
  template emit_header(ref) -> io_list

  -- LSP actions (existing, kept)
  abstract action complete(position) -> [completion]
  abstract action diagnose(range) -> [diagnostic]
  abstract action hover(position) -> string
  abstract action definition(position) -> position
  abstract action references(position) -> [position]
  abstract action tokens(range) -> [token]
}

out position
out range
out severity
out diagnostic
out completion
out token
out @code
```

- [ ] **Step 2: Verify grammar compiles**

Run: `nix develop -c cargo test`

- [ ] **Step 3: Commit**

```bash
git add boot/03-code.mirror
git commit -m "🟢 @code: abstract translate + render templates, type-annotated constraints"
```

---

### Task 4: IoList type

**Files:**
- Create: `src/emit_code.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Write failing tests**

```rust
// src/emit_code.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iolist_empty() {
        assert_eq!(IoList::Empty.to_bytes(), vec![]);
    }

    #[test]
    fn iolist_chunk() {
        assert_eq!(IoList::text("hello").to_bytes(), b"hello");
    }

    #[test]
    fn iolist_nested() {
        let list = IoList::join(vec![
            IoList::text("pub "),
            IoList::text("struct "),
            IoList::text("Foo;\n"),
        ]);
        assert_eq!(list.to_bytes(), b"pub struct Foo;\n");
    }

    #[test]
    fn iolist_deep() {
        let inner = IoList::join(vec![IoList::text("a"), IoList::text("b")]);
        let outer = IoList::join(vec![inner, IoList::text("c")]);
        assert_eq!(outer.to_bytes(), b"abc");
    }

    #[test]
    fn iolist_to_string() {
        let list = IoList::join(vec![IoList::text("hello "), IoList::text("world")]);
        assert_eq!(list.to_string_lossy(), "hello world");
    }

    #[test]
    fn iolist_empty_nested() {
        let list = IoList::join(vec![IoList::Empty, IoList::text("x"), IoList::Empty]);
        assert_eq!(list.to_bytes(), b"x");
    }
}
```

- [ ] **Step 2: Run tests — fail**

- [ ] **Step 3: Implement**

```rust
// src/emit_code.rs

//! emit_code — generic code emitter driven by @code grammar templates.

#[derive(Clone, Debug)]
pub enum IoList {
    Chunk(Vec<u8>),
    Nested(Vec<IoList>),
    Empty,
}

impl IoList {
    pub fn text(s: &str) -> Self {
        IoList::Chunk(s.as_bytes().to_vec())
    }

    pub fn join(parts: Vec<IoList>) -> Self {
        IoList::Nested(parts)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.flatten_into(&mut out);
        out
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.to_bytes()).into_owned()
    }

    fn flatten_into(&self, out: &mut Vec<u8>) {
        match self {
            IoList::Chunk(bytes) => out.extend_from_slice(bytes),
            IoList::Nested(children) => {
                for child in children {
                    child.flatten_into(out);
                }
            }
            IoList::Empty => {}
        }
    }
}
```

- [ ] **Step 4: Add to lib.rs**

```rust
pub mod emit_code;
```

- [ ] **Step 5: Run tests — pass**

- [ ] **Step 6: Commit**

```bash
git add src/emit_code.rs src/lib.rs
git commit -m "🟢 emit_code: IoList — tree of byte slices, zero-copy concat"
```

---

### Task 5: CodeGrammar::rust() — hardcoded Rust templates

**Files:**
- Modify: `src/emit_code.rs`

Port `emit_rust.rs` logic into `CodeGrammar` struct with `TemplateSet` of closures. Each closure returns IoList instead of writing to String. Same type mappings, same case conversions, different output type.

- [ ] **Step 1: Write failing tests**

```rust
#[test]
fn rust_map_type_text() {
    let g = CodeGrammar::rust();
    assert_eq!((g.templates.map_type)("text").to_string_lossy(), "String");
}

#[test]
fn rust_map_type_option() {
    let g = CodeGrammar::rust();
    assert_eq!((g.templates.map_type)("option(text)").to_string_lossy(), "Option<String>");
}

#[test]
fn rust_emit_enum() {
    let g = CodeGrammar::rust();
    let r = (g.templates.emit_enum)("color", &[], &["red".into(), "blue".into()]);
    let s = r.to_string_lossy();
    assert!(s.contains("pub enum Color"));
    assert!(s.contains("Red,"));
    assert!(s.contains("Blue,"));
}

#[test]
fn rust_emit_unit() {
    let g = CodeGrammar::rust();
    let r = (g.templates.emit_unit_type)("point");
    assert_eq!(r.to_string_lossy(), "pub struct Point;\n");
}
```

- [ ] **Step 2: Implement CodeGrammar + TemplateSet + CodeGrammar::rust()**

Port all functions from `emit_rust.rs` — `map_type`, `to_pascal_case`, `to_snake_case`, `emit_enum`, `emit_struct`, `emit_function`, `emit_property`, `emit_module` — as closures returning IoList. Same logic, different output type.

- [ ] **Step 3: Run tests — pass**

- [ ] **Step 4: Commit**

```bash
git add src/emit_code.rs
git commit -m "🟢 emit_code: CodeGrammar::rust() — Rust templates as IoList"
```

---

### Task 6: CodeGrammar::gleam() — Gleam templates

**Files:**
- Modify: `src/emit_code.rs`

- [ ] **Step 1: Write failing tests**

```rust
#[test]
fn gleam_map_type_nat() {
    let g = CodeGrammar::gleam();
    assert_eq!((g.templates.map_type)("nat").to_string_lossy(), "Int");
}

#[test]
fn gleam_all_ints_are_int() {
    let g = CodeGrammar::gleam();
    for t in ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "usize"] {
        assert_eq!((g.templates.map_type)(t).to_string_lossy(), "Int");
    }
}

#[test]
fn gleam_emit_enum() {
    let g = CodeGrammar::gleam();
    let r = (g.templates.emit_enum)("color", &[], &["red".into(), "blue".into()]);
    let s = r.to_string_lossy();
    assert!(s.contains("pub type Color"));
    assert!(s.contains("Red"));
    assert!(!s.contains("enum"));
}

#[test]
fn gleam_emit_struct() {
    let g = CodeGrammar::gleam();
    let r = (g.templates.emit_struct)("user", &[("name".into(), "text".into())], &[]);
    let s = r.to_string_lossy();
    assert!(s.contains("pub type User"));
    assert!(s.contains("User("));
    assert!(s.contains("name: String"));
}

#[test]
fn gleam_emit_function() {
    let g = CodeGrammar::gleam();
    let r = (g.templates.emit_function)("boot", &[("id".into(), "text".into())], None);
    let s = r.to_string_lossy();
    assert!(s.contains("pub fn boot"));
    assert!(s.contains("todo"));
    assert!(!s.contains("todo!()"));
}
```

- [ ] **Step 2: Implement CodeGrammar::gleam()**

Same structure as rust(), different type mappings and syntax. Key differences: all ints→Int, all floats→Float, `pub type X { X(...) }` for structs, `todo` not `todo!()`.

- [ ] **Step 3: Run tests — pass**

- [ ] **Step 4: Commit**

```bash
git add src/emit_code.rs
git commit -m "🟢 emit_code: CodeGrammar::gleam() — all ints are Int, todo not todo!()"
```

---

### Task 7: emit_code() — the generic dispatcher

**Files:**
- Modify: `src/emit_code.rs`

- [ ] **Step 1: Write failing tests**

```rust
use crate::mirror_runtime::{MirrorRuntime, CompiledShatter};

#[test]
fn emit_code_enum_rust() {
    let rt = MirrorRuntime::new();
    let c: Result<CompiledShatter, _> = rt.compile_source("type color = red | blue").into();
    let out = emit_code(&c.unwrap(), &CodeGrammar::rust());
    assert!(out.to_string_lossy().contains("pub enum Color"));
}

#[test]
fn emit_code_enum_gleam() {
    let rt = MirrorRuntime::new();
    let c: Result<CompiledShatter, _> = rt.compile_source("type color = red | blue").into();
    let out = emit_code(&c.unwrap(), &CodeGrammar::gleam());
    let s = out.to_string_lossy();
    assert!(s.contains("pub type Color"));
    assert!(!s.contains("enum"));
}

#[test]
fn emit_code_matches_emit_rust() {
    let rt = MirrorRuntime::new();
    for src in ["type color = red | blue", "type point", "action boot(identity)"] {
        let c: Result<CompiledShatter, _> = rt.compile_source(src).into();
        let c = c.unwrap();
        let old = crate::emit_rust::emit_rust(&c);
        let new = emit_code(&c, &CodeGrammar::rust()).to_string_lossy();
        assert_eq!(old.trim(), new.trim(), "parity failed for: {}", src);
    }
}
```

- [ ] **Step 2: Implement emit_code()**

Walk MirrorFragment, dispatch to grammar templates at each node. Same structure as `emit_frag` in emit_rust.rs but calling `grammar.templates.*` instead of hardcoded formatting.

- [ ] **Step 3: Run tests — pass**

- [ ] **Step 4: Commit**

```bash
git add src/emit_code.rs
git commit -m "🟢 emit_code: generic dispatcher — one function, any grammar"
```

---

### Task 8: Wire CLI + delete emit_rust.rs

**Files:**
- Modify: `src/cli.rs`
- Modify: `src/lib.rs`
- Delete: `src/emit_rust.rs`

- [ ] **Step 1: Wire --target flag**

```rust
// In cli.rs craft command handler:
let grammar = match target.as_deref() {
    Some("rust") | None => emit_code::CodeGrammar::rust(),
    Some("gleam") => emit_code::CodeGrammar::gleam(),
    Some(other) => return Err(CliError::Usage(format!("unknown target: {}", other))),
};
let output = emit_code::emit_code(&compiled, &grammar);
```

- [ ] **Step 2: Update all callers of emit_rust**

Search for `emit_rust` in codebase. Replace with `emit_code` + `CodeGrammar::rust()`.

- [ ] **Step 3: Delete emit_rust.rs**

```bash
rm src/emit_rust.rs
```

Remove `pub mod emit_rust;` from lib.rs.

- [ ] **Step 4: Run full test suite**

Run: `nix develop -c cargo test`
Expected: ALL tests pass.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "♻️ delete emit_rust.rs — the grammar knows how to emit Rust"
```

---

### Task 9: @code/gleam boot grammar

**Files:**
- Create: `boot/03b-code-gleam.mirror`

- [ ] **Step 1: Write the grammar**

```mirror
in @code

grammar @code/gleam {
  type int
  type float
  type string
  type bool
  type bit_array
  type list(t)
  type result(t, e)
  type option(t)
  type nil
  type dynamic

  action complete(position) in @code/gleam { }
  action diagnose(range) in @code/gleam { }
  action hover(position) in @code/gleam { }
  action definition(position) in @code/gleam { }
  action references(position) in @code/gleam { }
  action tokens(range) in @code/gleam { }
}

out @code/gleam
```

- [ ] **Step 2: Verify it compiles**

Run: `nix develop -c cargo test`

- [ ] **Step 3: Commit**

```bash
git add boot/03b-code-gleam.mirror
git commit -m "🟢 boot: @code/gleam grammar — Gleam primitive types"
```

---

## Ship Criteria

- [ ] `ast(g)` type in `@meta` — parameterized by grammar
- [ ] `DeclKind::Template` in parser
- [ ] `@code` grammar with abstract `translate` and `render` templates
- [ ] `IoList` type — zero-copy concat
- [ ] `CodeGrammar::rust()` produces identical output to old `emit_rust.rs`
- [ ] `CodeGrammar::gleam()` produces valid Gleam
- [ ] `emit_code(compiled, grammar)` — one function, any grammar
- [ ] `--target rust|gleam` in CLI
- [ ] `emit_rust.rs` deleted
- [ ] `@code/gleam` boot grammar
- [ ] All existing tests pass

---

## What This Does NOT Include

- Grammar-driven template loading (`CodeGrammar::from_compiled`) — that's Phase 2
- `spectral.spec` target declarations — that's the spectral plan
- @code/elixir, @code/python — each is a new `CodeGrammar` constructor + boot grammar
- `translate` template body evaluation from grammar — currently templates are Rust closures

The hardcoded `CodeGrammar::rust()` and `CodeGrammar::gleam()` are stepping stones.
The grammar declarations exist (`03-code.mirror` has the abstract templates).
Making the Rust code READ those declarations instead of hardcoding is the next phase.

---

*Two primitives. Lambda and split. Everything else is named combinations.*
*The type annotation IS the constraint. No where clauses.*
*Delete emit_rust.rs. The grammar knows.*

*2026-04-15. Reed.*
