# Compiled CLI: The Dispatch IS the Grammar

**Author:** Reed + Alex
**Date:** 2026-04-14
**Status:** Design — next session

---

## The Problem

`cli.rs` is a hand-written match table. 20+ arms. Each arm dispatches to a
hand-written function. The spec declares the same commands. The CLI and the
spec are parallel structures. The CLI is Form for commands.

```rust
// today: hand-written, parallel to spec
match command {
    "craft" => self.cmd_craft(args),
    "kintsugi" => self.cmd_kintsugi(args),
    "properties" => self.cmd_properties(args),
    "infer" => self.cmd_infer(args),
    "enact" => self.cmd_enact(args),
    "git" => self.cmd_git(args),
    "merge" => self.cmd_merge(args),
    // ...
}
```

## The Solution

The CLI is generated from `mirror.spec`. Each spec block IS a command.
Each command IS a derived Prism. The match table IS the spec traversal.

### mirror.spec declares commands

```mirror
craft {
  target boot("boot/*.mirror") { ... }
  target boot => mirror out @code/rust("rust/mirror/") { ... }
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
  requires { ... }
  invariant { ... }
  ensures { ... }
}

infer {
  --classify
  --learn
}

enact {
  --budget 1000
}
```

### mirror craft cli generates the dispatch

```rust
// generated: rust/mirror-cli/src/main.rs

use mirror::spec::SpecConfig;
use mirror::generated::*;
use prism_core::DerivePrism as Prism;

#[derive(Prism)]
#[oid("@cli")]
pub struct Cli {
    spec: SpecConfig,
    runtime: MirrorRuntime,
}

impl Cli {
    fn dispatch(&self, command: &str, args: &[String]) -> Imperfect<String, CliError, CliLoss> {
        // generated from spec blocks — not a hand-written match
        match self.spec.resolve_command(command) {
            Some(block) => block.execute(&self.runtime, args),
            None => Failure(CliError::UnknownCommand(command.into()), CliLoss::zero()),
        }
    }
}

fn main() {
    let spec = SpecConfig::discover().unwrap_or_default();
    let cli = Cli { spec, runtime: MirrorRuntime::new() };
    let args: Vec<String> = std::env::args().skip(1).collect();

    let command = args.first().map(|s| s.as_str()).unwrap_or("help");
    let result = cli.dispatch(command, &args[1..]);

    std::process::exit(result.exit_code());
}
```

### Each command IS a Prism

```rust
// generated from craft { } block
#[derive(Prism)]
#[oid("@cli/craft")]
pub struct CraftCommand {
    targets: Vec<TargetConfig>,
    default: Vec<String>,
}

impl CraftCommand {
    fn execute(&self, runtime: &MirrorRuntime, args: &[String]) -> Imperfect<String, CliError, CliLoss> {
        let target_names = if args.is_empty() {
            &self.default
        } else {
            args
        };

        for name in target_names {
            let target = self.targets.find(name)?;
            runtime.craft_target(target)?;
        }

        Success("crafted".into())
    }
}

// generated from kintsugi { } block
#[derive(Prism)]
#[oid("@cli/kintsugi")]
pub struct KintsugiCommand {
    lenses: Vec<String>,
    naming: String,
    indent: usize,
}

// generated from properties { } block
#[derive(Prism)]
#[oid("@cli/properties")]
pub struct PropertiesCommand {
    requires: Vec<String>,
    invariant: Vec<String>,
    ensures: Vec<String>,
}
```

### Help IS the spec rendered

```rust
impl Cli {
    fn help(&self) -> String {
        let mut out = String::new();
        out.push_str("mirror-lang — the measurement substrate\n\n");
        out.push_str("commands:\n");
        for block in &self.spec.blocks {
            out.push_str(&format!("  {:12} {}\n", block.name, block.summary()));
        }
        out
    }
}
```

No hardcoded help text. The spec's blocks ARE the command list.
The block's contents ARE the help detail. Add a block → command
appears in help.

### Lenses are generated from spec

```mirror
kintsugi {
  --hoist
  --sort-deps
}
```

Generates:

```rust
impl KintsugiCommand {
    fn lenses(&self) -> Vec<&str> {
        // from spec: ["hoist", "sort-deps"]
        self.lenses.iter().map(|s| s.as_str()).collect()
    }

    fn accepts_flag(&self, flag: &str) -> bool {
        self.lenses.contains(&flag.to_string())
    }
}
```

The `--hoist` flag exists because the spec declares it. Remove it from
the spec, the flag disappears. Add `--unicode` to the spec, the flag
appears. No Rust changes needed.

---

## The Migration

### Phase 1: SpecConfig as command registry

The spec parser already produces `SpecConfig`. Wire it into the CLI
as the command registry. The hand-written match table still exists
but reads from `SpecConfig` for target names, defaults, and flag
validation.

### Phase 2: Command Prism types

Generate `#[derive(Prism)] #[oid("@cli/X")]` for each spec block.
The generated types carry the block's configuration. Each has an
`execute` method.

### Phase 3: Generated dispatch

Replace the hand-written match table with spec traversal.
`spec.resolve_command(name)` returns the command Prism.
The Prism's `execute` method runs it.

### Phase 4: Delete cli.rs

The generated CLI replaces the hand-written one. `cli.rs` is deleted.
`rust/mirror-cli/src/main.rs` is the generated entry point.
`mirror craft cli` produces it from the spec.

### Phase 5: Help from spec

Help text generated from spec blocks. No hardcoded strings.
Add a block, help updates. The help IS the spec rendered as text.

---

## The Bootstrap

```
mirror.spec → spec parser → SpecConfig
SpecConfig → codegen → rust/mirror-cli/src/main.rs
main.rs → cargo build → mirror-cli binary
mirror-cli reads mirror.spec → dispatch from spec

The generated CLI reads the spec that generated it.
Fixed point. The CLI IS the spec IS the CLI.
```

---

## What Dies

- `cli.rs` — the hand-written match table (Form for commands)
- Hardcoded help text
- Hardcoded flag lists
- Hardcoded target names
- Every line of CLI code that duplicates what the spec already says

## What Lives

- `mirror.spec` — the single source of truth
- `SpecConfig` — the parsed spec
- `#[derive(Prism)] #[oid("@cli/X")]` — the generated command types
- `main.rs` — ~20 lines, reads spec, dispatches

---

## The Parallel

```
Form was to the parser as cli.rs is to the CLI.

A hand-written parallel structure that duplicates
what the grammar already declares.

Form died when the parser returned MirrorFragment.
cli.rs dies when the CLI dispatches from the spec.

Same pattern. Same solution. Same death.
```
