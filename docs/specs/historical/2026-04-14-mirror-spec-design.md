# mirror.spec — The CLI IS the Spec IS the Config

**Author:** Reed + Alex
**Date:** 2026-04-14
**Status:** Design

---

## Principle

The spec file mirrors the CLI surface. Each top-level block IS a CLI command.
Each block's contents ARE that command's configuration. The help text IS the
spec rendered. Adding a block adds a command. Adding a target adds a build
artifact. One file. One surface. Both sides.

---

## File Extension

`mirror.spec` — classified by `@code/spec` via the extension classifier.

Located at project root. One per project. The compiler discovers it
automatically (walks up from working directory, like `.git`).

---

## Structure

```
mirror.spec
  @oid          project identity
  store { }     content-addressed store configuration
  craft { }     build targets
  kintsugi { }  formatter configuration
  properties { } property checks
  infer { }     observation/classification configuration
  enact { }     execution configuration
```

Each top-level block maps 1:1 to a CLI command:

```
mirror craft        → reads craft { }
mirror kintsugi     → reads kintsugi { }
mirror properties   → reads properties { }
mirror infer        → reads infer { }
mirror enact        → reads enact { }
```

---

## The Spec

```mirror
# mirror.spec

@oid("@mirror-lang")

store {
  path = .git/mirror
}

craft {
  target boot("boot/*.mirror") {
    @prism
    @meta
    @meta/action
    @meta/io
    @shatter
    @property
    @package
    @package/git
  }

  target std("boot/std/*.mirror") {
    @code
    @code/rust
    @actor
    @list
    @map
    @set
    @option
    @result
    @order
    @bool
    @text
    @number
    @beam
    @properties
    @mirror
    @cli
  }

  target boot => mirror out @code/rust("rust/mirror/") {
    @prism
    @meta
    @property
    @package
    @list
    @map
    @set
    @option
    @result
    @order
    @bool
    @text
    @number
  }

  target boot => cli out @code/rust("rust/mirror-cli/") {
    @lsp
    @git
    @store
    @code
    @shatter
  }

  target boot => docs out @lang/en("docs/") {
    @prism
    @meta
    @property
  }

  target boot => po out @i18n("locales/") {
    locales = [en, de, ja]
    default = en
  }

  target shatter {
    boot => @shatter
  }

  default boot mirror cli
}

kintsugi {
  --hoist
  --sort-deps
  --normalize
  --align
  naming = snake_case
  indent = 2
}

properties {
  requires {
    types_lowercase
    action_is_named_type
    unique_variants
    every_type_reachable
    no_dead_variants
  }
  invariant {
    deterministic
    pure
    no_cycles
  }
  ensures {
    always_halts
  }
}

infer {
  --classify
  --learn
}

enact {
  --budget 1000
}
```

---

## Semantics

### @oid

The project's identity. Content-addressed. Used in `mirver`, in the store,
in the garden. Must start with `@`.

### store { }

Where the content-addressed artifacts live.

```mirror
store {
  path = .git/mirror     # git-integrated
}
```

Or standalone:

```mirror
store {
  path = .mirror         # standalone (no git)
}
```

### craft { }

Build targets. The core of the spec.

**Target declaration:**

```mirror
target name("path/glob") { grammars }
```

- `name` — the target name, used as CLI argument: `mirror craft name`
- `"path/glob"` — optional, where source files live. No path = content-addressed only
- `{ grammars }` — manifest of grammars this target includes

**Target flow:**

```mirror
target source => name out @lens("output/path/") { grammars }
```

- `source` — which target feeds this one
- `name` — the output target name
- `@lens` — the grammar that renders the output (`@code/rust`, `@lang/en`, `@i18n`)
- `"output/path/"` — where files materialize
- `{ grammars }` — which grammars from the source are included

**Default:**

```mirror
default boot mirror cli
```

What `mirror craft` (no arguments) builds. If omitted, `mirror craft`
with no arguments prints help.

### kintsugi { }

Formatter configuration. Each entry is a lens.

```mirror
kintsugi {
  --hoist           # reorder declarations by kind
  --sort-deps       # dependency sort within sections
  --normalize       # fix spacing around operators
  --align           # align split pipes vertically
  naming = snake_case   # naming convention
  indent = 2            # indentation width
}
```

`mirror kintsugi` applies all lenses declared here.
`mirror kintsugi --check` checks without writing.

### properties { }

Property checks, organized by lifecycle.

```mirror
properties {
  requires { ... }     # must hold before compilation
  invariant { ... }    # must hold throughout
  ensures { ... }      # must hold after compilation
}
```

`mirror properties` runs all checks.
`mirror properties --strict` makes Partial into Failure.

### infer { }

Observation and classification defaults.

```mirror
infer {
  --classify         # enable eigenvalue classification
  --learn            # enable grammar learning from source
}
```

`mirror infer` observes the project.
`mirror infer --classify src/` classifies files.
`mirror infer --learn src/` learns grammars from source.

### enact { }

Execution configuration.

```mirror
enact {
  --budget 1000      # maximum computation steps (sub-Turing gate)
}
```

`mirror enact app.shard` executes.
`--budget` bounds execution.

---

## CLI Dispatch

The CLI reads `mirror.spec` and dispatches:

```rust
fn main() {
    let spec = find_and_parse_spec();
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("craft") => dispatch_craft(&spec.craft, &args[2..]),
        Some("kintsugi") => dispatch_kintsugi(&spec.kintsugi, &args[2..]),
        Some("properties") => dispatch_properties(&spec.properties, &args[2..]),
        Some("infer") => dispatch_infer(&spec.infer, &args[2..]),
        Some("enact") => dispatch_enact(&spec.enact, &args[2..]),
        _ => print_help_from_spec(&spec),
    }
}
```

`print_help_from_spec` generates help text FROM the spec. Not a hardcoded
string. The spec's top-level blocks become the command list. Each block's
contents become the command's help detail.

---

## Help Generation

```
$ mirror

  mirror-lang — the measurement substrate

  commands:
    craft       build targets (default: boot mirror cli)
    kintsugi    format with golden repair
    properties  check invariants
    infer       observe and classify
    enact       execute shards

  run `mirror <command> --help` for details

$ mirror craft --help

  mirror craft [targets] [--lenses]

  targets:
    boot        kernel grammars (boot/*.mirror)
    std         standard library (boot/std/*.mirror)
    mirror      optics crate (rust/mirror/)
    cli         CLI binary (rust/mirror-cli/)
    docs        documentation (docs/)
    po          translations (locales/)
    shatter     crystal artifact

  default: boot mirror cli

  lenses:
    --strict    reject partial results
    --check     check without writing
    --kintsugi  format before crafting
```

Every line generated from the spec. Add a target, help updates. Add a lens,
help updates. The help IS the spec rendered as text.

---

## Discovery

The compiler discovers `mirror.spec` by walking up from the current directory:

```
./mirror.spec
../mirror.spec
../../mirror.spec
...
```

First match wins. Like `.git` discovery. If none found, the compiler runs
without a spec (bare mode — no targets, no properties, just parse + compile).

---

## Content Addressing

The spec itself is content-addressed. Its Oid is part of the project's
identity. Change the spec, change the project's Oid. The spec IS part
of the crystal.

```
project Oid = hash(spec Oid + boot crystal Oid)
```

The spec participates in the content address. A project with different
properties or different targets IS a different project, even if the
source grammars are identical.

---

## What This Replaces

| Traditional file         | mirror.spec block |
|--------------------------|-------------------|
| Cargo.toml               | craft { target } |
| Makefile / build.rs      | craft { target => } |
| .formatter.exs           | kintsugi { } |
| .credo.exs               | properties { } |
| .eslintrc                | properties { } + kintsugi { } |
| tsconfig.json            | craft { target } |
| package.json             | @oid + craft { } |
| mix.exs                  | the whole spec |
| .github/workflows/*.yml  | craft { } + properties { } |

One file. Type-checked. Content-addressed. The CLI IS the spec IS the config.
