# mirror

> Pre-v0.1. The shapes are stable enough to document. The implementation is
> not stable enough to depend on.

A self-hosting language whose programs are inferred chains of model
invocations, compiled to spectral content-addressed artifacts. `prism` is
the only hardcoded keyword. Everything else is declared in source.

```
mirror compile boot/00-form.mirror
```

## What it is

A `.mirror` file describes a *form*: a typed declaration tree built from a
small fixed vocabulary of optic operations (focus, project, split, zoom,
refract) plus declared types, lenses, properties, and boundaries. The
compiler turns the file into a `Shatter` — a content-addressed trajectory
in a five-dimensional spectral hash space — and writes it to disk as
`<file>.shatter`.

The compilation primitive is `MirrorFragment`, a
`Fractal<MirrorData, CoincidenceHash<5>>` defined in the `coincidence`
crate. Round-trip is exact: parse → emit → parse yields identical content
OIDs because the OID is derived from `MirrorData::encode()` and recursive
child OIDs.

`prism` is the only keyword the parser hardcodes. The other declaration
words (`form`, `lens`, `fold`, `traversal`, `iso`, `property`, `requires`,
`invariant`, `ensures`, `in`, `type`, `boundary`, `setter`, and the five
prism operations) are vocabulary items declared in the standard library at
`boot/00-form.mirror` through `boot/06-mirror.mirror`. They are recognized
by the parser as content-address tags — recorded, not interpreted.

## Architecture

```
parse        .mirror source           → Form (typed declaration tree)
runtime      Form                     → MirrorFragment (content-addressed)
shatter      Form ↔ MirrorFragment    via Prism trait (focus / project /
                                       split / zoom / refract)
emit         CompiledShatter          → <file>.shatter on disk
```

The compilation pipeline lives in `src/mirror_runtime.rs`:

- `parse_form(source)` → `Form` — line-oriented brace-balanced parser.
- `MirrorRuntime::compile_source` / `compile_file` / `compile_boot_dir` →
  `CompiledShatter` (the form, its content-addressed fragment, the crystal
  OID).
- `Shatter` implements the `Prism` trait. `focus` reads the top-level
  eigenvalues. `project` builds a content-addressed fragment from those
  eigenvalues. `refract` settles a fragment back into a `Form`. `split`
  and `zoom` are conservative no-ops with `// TBD` comments — their
  semantics will be specified when use arrives.

## CLI

```
mirror compile <file>                 compile to <file>.shatter
mirror ai <model> <file>              run one inference step under <model>
mirror ai <file>                      alias for: mirror ai fate <file>
mirror ai <file> --out=<target>       write the result to <target>
mirror fmt <file>                     alias: mirror ai fate --train --out=<file>
mirror '<query>' <file>               TBD — form-as-operation query mode
```

The `ai` subcommands compose through bash pipes. Each invocation reads a
`.mirror` file (or stdin), extracts deterministic spectral features from
its declaration tree, runs `FateRuntime::select` with the named model as
the starting context, and emits a small mirror form on stdout that names
the selected next model. The next `mirror ai` invocation reads that form.

```
mirror ai abyss form.mirror | mirror ai pathfinder - | mirror ai fate -
```

The chain is the program. The chain is also the `.shatter`.

## The five fate models

The `fate` crate provides the inference primitives. Five models, one
selector. Each maps to one operation of the Prism trait:

| Model        | Operation | What it does                              |
|--------------|-----------|-------------------------------------------|
| Abyss        | focus     | Observe the spectral state.               |
| Pathfinder   | project   | Precision cut — which paths survive.      |
| Cartographer | split     | Map the territory. Walk every node.       |
| Explorer     | zoom      | Recover meaning at the boundary.          |
| Fate         | refract   | Crystallize. Select what runs next.       |

The weights are hardcoded. The binary is the model. Inference is bit-for-
bit deterministic — same Form, same features, same selection, forever.

## The standard library

Seven files, ~17 lines plus property declarations. The whole vocabulary of
mirror is defined here:

- `boot/00-form.mirror` — declares `@form` itself, naming the five Prism
  operations as the root vocabulary.
- `boot/01-prism.mirror` — the Prism algebra.
- `boot/02-type.mirror` — types.
- `boot/03-boundary.mirror` — boundaries.
- `boot/04-lens.mirror` — lenses.
- `boot/05-property.mirror` — the nine standard properties.
- `boot/06-mirror.mirror` — declares `form @mirror { ... }`, re-exports
  everything via `in`, and applies the standard properties to mirror
  itself.

The nine standard properties (renamed from math-discipline labels into
operator-legible names so the type system reads as instructions, not as
research notes):

| Property               | Was                     |
|------------------------|-------------------------|
| `unique_variants`      | shannon_equivalence     |
| `every_type_reachable` | connected               |
| `dual_partition`       | bipartite               |
| `no_dead_variants`     | exhaustive              |
| `idempotent`           | (new)                   |
| `always_halts`         | (new)                   |
| `deterministic`        | (new)                   |
| `pure`                 | (new)                   |
| `no_cycles`            | (new)                   |

`06-mirror.mirror` applies them: `requires unique_variants`,
`invariant idempotent`, `ensures always_halts`. The standard library
checks itself.

## Build and run

```
nix develop -c cargo build
nix develop -c cargo run -- compile boot/00-form.mirror
```

Tests:

```
nix develop -c cargo test
```

## A note on N=5

`MirrorFragment = Fractal<MirrorData, CoincidenceHash<5>>`. The `5` is the
spectral dimension. Five eigenvalues per content address.

Five spectral dimensions: meets-and-exceeds the 3+1 of the cosmos. The
hash function has enough degrees of freedom to be a cosmic content
address — every structurally distinct form has a unique slot, with room.
The dimension is over-determined: it is also the count of operations in
the Prism algebra (focus / project / split / zoom / refract) and the count
of fate models (abyss / pathfinder / cartographer / explorer / fate). The
same number lands in three places. That is the reason the choice is
canonical rather than tuned.

See `coincidence/docs/insights/2026-04-07-spectral-hash-as-canonical-default.md`.

## Layering: mirror vs spectral

mirror compiles a single `.mirror` file. spectral wraps multiple mirror
processes for build, runtime, deployment, and collaboration. The Unix
analogy: mirror is to spectral what gcc is to make. The BEAM runtime
backend lives in spectral (`spectral/docs/gen_prism.md`), not in mirror.

## Insights

- `docs/insights/2026-04-07-the-chain-is-the-shatter.md` — why the chain of
  model invocations *is* the `.shatter` file, and what the editing
  experience becomes when that recognition lands.
- `docs/insights/2026-04-07-quantum-native-on-classical-hardware.md` — the
  mapping from quantum-mechanical primitives to mirror types, and the
  implementation strategy that gets quantum semantics on a CPU.

---

Built with math and lots of weed. Both are load-bearing.
