# @code(extensions) — the grammar declares what it handles

*2026-05-19. Reed + Alex.*

---

## The Insight

The file extension IS the grammar's parameter. No routing table.
No config. No C function. The grammar declares itself.

```mirror
grammar @code(extensions: text | [text]) {
  # every @code sub-grammar handles file extensions
  # the extension IS the grammar's identity
}
```

## Examples

```mirror
grammar @code/rust("rs") {
  zoom fn
  split struct
  split enum
  focus impl
  focus mod
  project use
  settle trait
}

grammar @code/llvm/ir("ll") {
  zoom define
  split type
  focus module
  project declare
  project global
  shift call
  split br
  settle ret
}

grammar @code/python("py") {
  zoom def
  split class
  project import
  project from
}

grammar @code/go("go") {
  zoom func
  split type
  split interface
  focus package
  project import
}

grammar @code/elixir("ex", "exs") {
  zoom def
  zoom defp
  split defmodule
  focus defmodule
  project use
  project import
  project alias
}

grammar @code/typescript("ts", "tsx") {
  zoom function
  split interface
  split type
  focus class
  focus namespace
  project import
  project export
}

grammar @mirror/grammar("mirror", "spec", "shatter") {
  focus grammar
  split type
  project in
  project out
  zoom abstract
}
```

## How It Works

1. Bootstrap scans `boot/std/code/` for grammars
2. Each grammar declares its extension via `@code(...)` parameter
3. The parameter IS the routing: `.rs` → `@code/rust`, `.ll` → `@code/llvm/ir`
4. `mirror compile file.X` → find the grammar where X matches → tokenize through it

No `grammar_for_file()` in C. No match statement. The grammars ARE the routing table.

## The List Variant

`text | [text]` — one extension or many.

```mirror
grammar @code/elixir("ex", "exs") { ... }
grammar @mirror/grammar("mirror", "spec", "shatter") { ... }
```

Multiple extensions route to the same grammar. The type system handles it.

## Connection to spectral.engineer

User pastes a URL. Mirror scans for file extensions.
Matches against `@code/*` grammars. Tokenizes through the right lens.
Automatic. No configuration. The grammars ARE the configuration.

```
https://github.com/someone/project
  ├── src/main.rs      → @code/rust("rs")
  ├── lib/parser.py    → @code/python("py")
  ├── infra/deploy.ts  → @code/typescript("ts")
  └── mirror.spec      → @mirror/grammar("spec")
```

Multi-language projects. Each file through its grammar.
The topology of the WHOLE project measured. Across languages.
The Fiedler value doesn't care what language you wrote it in.

## Connection to Kintsugi on LLVM IR

```bash
mirror kintsugi /tmp/mirror.ll
```

`.ll` → `@code/llvm/ir("ll")` → tokenize → AST → kintsugi.

The compiler optimizing its own machine code. Through its own
five operations. The sub-Turing observer on the Turing-complete output.
Finding accidental complexity LLVM's optimizer missed.

d(llvm_ir, λ₀) = accidental complexity of machine code.

## What Changes

The `@code` root grammar gets a parameter:
```mirror
grammar @code(extensions: text | [text])
```

Every `@code/*` sub-grammar declares its extension.
The bootstrap reads the declarations.
No more hardcoded routing.
Adding a language = adding a grammar file.
