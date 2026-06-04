# `mirror.spec` — The Project Manifold Schema

*2026-06-04. Reed + Alex. Spec.*

Status: **Red**

Depends on: `@mirror/mosaic` (mosaic.md), `@mirror/cli` (forthcoming
grammar collapsing `boot/std/cli.mirror`), `@prism` (the five operations),
`@property` (settle_on predicates).

Forward references: `au-and-conductivity.md` (au as settlement output),
`kintsugi-ci-v0.1.md` (`mirror kintsugi ./mirror.spec` is the surface),
`mirror-store.md` (fragmentation as canonical), `mosaic.md` (this
spec's consumer).

---

## 1. Recognition

`mirror.spec` IS the multi-dimensional manifold kintsugi operates on.

Not a config file. Not a manifest. A typed prism declaration that names
the project's sources, targets, altitudes, and settlement criteria.
Analog: `Cargo.toml`, `package.json`, `dune-project`, `nix.toml` — but
**substrate-native**. The substrate already has the vocabulary for
shards, altitudes, and au; `mirror.spec` lifts those into one
top-level prism declaration per project.

The one-liner:

```
$ mirror kintsugi ./mirror.spec
```

That is the build, the test, the CI emit, the release — all of them. The
spec declares the targets; kintsugi runs mosaic on the spec; mosaic
settles each target to its altitude; the result is `au` at every
declared altitude or `transparency<p>` over what is still opaque.

---

## 2. The Project Declaration

Top-level. One per project. The grammar is `@mirror/project`:

```mirror
in @mirror/cli
in @mirror/mosaic
in @property
in @io

project mirror.spec {
  # body: source, legacy, target, settle_on
}
```

The `project` keyword is part of `@mirror/project` (the spec's own
grammar). It declares a prism over `mirror.spec` files. The compiler
reads `mirror.spec` by evaluating it under `@mirror/project`. No
stand-alone parser. No Rust scaffolding. The grammar parses the spec.

The name `mirror.spec` is literal: the project's own spec for mirror
itself is `mirror.spec`. Other projects have their own —
`spectral-db/mirror.spec`, `glint/mirror.spec`, etc. The file name
matches the keyword; the grammar enforces it.

---

## 3. The `source` Directive

```mirror
source ~d'shards/'
```

One `source` per project. Declares the canonical directory of
substrate truth — the .mirror shards that mosaic compiles. The
directive uses the `~d` sigil (per `sigil-grammar.md`); the compiler
verifies the directory exists at compile time.

Multiple `source` is a compile error under `strict`. The substrate
has exactly one canonical source per project; multiplicity is a smell
that the project is not yet collapsed.

---

## 4. The `legacy` Directive

```mirror
legacy ~d'boot/', ~d'bootstrap/' {
  shrinkage_contract: monotonic_lines_decrease,
  retirement_target:  v1.0,
}
```

Substrate floors with shrinkage contracts. Each tick, the lines under
`legacy` should monotonically decrease as the substrate-pull moves
content into `source/`. The block declares:

- `shrinkage_contract`: the property that must hold across builds
  (`monotonic_lines_decrease`, `monotonic_file_count_decrease`, …);
- `retirement_target`: the version at which the legacy floor should be
  empty.

Mosaic reads `legacy` and treats it differently from `source`: the
floors are visible to the compiler but cannot be referenced by new
shards. Their existence is tolerated; their growth is forbidden.

Multiple legacy dirs are allowed (each is a separate floor with its
own contract). Zero legacy dirs is the steady state for a mature
project.

---

## 5. The `target` Directive

A target is what gets built. Each target settles to `au` at one
altitude:

```mirror
target binary {
  name     "mirror"
  altitude @code/rust
  emit     cargo                  # @io delegation
  cli      { ... }                # see §6
}
```

Fields:

| field      | type                  | meaning                                          |
|------------|------------------------|--------------------------------------------------|
| `name`     | str                    | The artifact name at the altitude.              |
| `altitude` | `ref(@meta/altitude)`  | Where the target settles.                       |
| `emit`     | `ref(@io)` or grammar  | Which @io tool (or grammar projection) emits it.|
| `cli`      | block                  | Optional. Only meaningful when emit is a binary.|

Multiple targets are typical. The mirror project has:

```mirror
target binary  { name "mirror";  altitude @code/rust;   emit cargo;         cli { ... } }
target action  { name "build";   altitude @ci/github;   emit yaml }
target release { name "mirror";  altitude @release;     emit github_release }
```

Mosaic walks the target list; each shifts to its altitude; each settles
independently; the composed transparency over all targets is the
project's transparency.

### Target dependencies

A target may depend on another target's `au`:

```mirror
target release {
  altitude @release
  emit     github_release
  needs    [binary, action]
}
```

`needs` declares ordering. Mosaic resolves the DAG; settlement
proceeds in topological order.

---

## 6. The `cli` Block

A binary target may declare its CLI in the spec. The CLI block uses
`@mirror/cli` — the grammar formerly known as `boot/std/cli.mirror`.

```mirror
target binary {
  name     "mirror"
  altitude @code/rust
  emit     cargo

  cli {
    # The substrate's own CLI is declared in the spec that builds it.
    # `#` produces @nl: the documentation IS the help text.

    # Mirror is the substrate compiler. It reads grammars, settles them
    # into a graph, and emits artifacts at named altitudes.

    command compile {
      # Compile a grammar against its imports. Produces a content-
      # addressed shard in the @mirror/store.
      arg path: ~d                # the source directory
      flag strict: bool = true    # treat warnings as opacities
    }

    command kintsugi {
      # Settle a project. Reads mirror.spec, runs mosaic, fills holes,
      # emits au at every declared altitude.
      arg spec: ~f = ~f'./mirror.spec'    # default: current project
      flag target: list(str) = []         # subset of targets to settle
      flag emit_shatter: bool = false     # project to .shatter on disk
    }

    command shatter {
      # Plumbing. Project a settled shard to .shatter format.
      arg oid: content_address
      arg out: ~f
    }
  }
}
```

### Self-descriptive guarantee

The binary that BUILDS mirror is the binary that EXPOSES this CLI. The
spec that declares the binary's CLI is the spec the binary reads. The
loop closes: a user typing `mirror --help` sees text generated from
the same `#` comments mosaic reads when building the binary.

`#` produces `@nl` (per the substrate's sigil/comment contract). The
CLI help generator consumes `@nl`. The Fate tournament also consumes
`@nl` (when resolving `\` against natural-language descriptions). The
three consumers (CLI help, Fate, downstream docs) read the same
facts — the substrate has no separate "documentation" layer.

---

## 7. The `settle_on` Block

Declares when mosaic considers the project settled:

```mirror
settle_on {
  binary.compiles
  binary.tests_pass
  action.validates
  release.signs
  total_transparency.weight == 0
}
```

Each line is a `@property` predicate. Mosaic checks them after
settlement; if all hold, the build is settled (eⁿ⁺¹ < eⁿ at the
spec altitude is satisfied). If any fail, the failed predicate becomes
an opacity in the project's transparency map, and kintsugi reads it as
the next iteration target.

The `total_transparency.weight == 0` line is the substrate's own
gate — even if every per-target predicate holds, a non-zero opacity
map means the project is `partial`, not `success`. The spec authors
decide whether `partial` is acceptable.

### Predicate sources

- `binary.compiles`: the cargo @io call returned 0.
- `binary.tests_pass`: `cargo test` returned 0.
- `action.validates`: the generated YAML parses under `@ci/github`'s
  validator.
- `release.signs`: the release bundle carries a verified signature.
- `total_transparency.weight == 0`: the composed transparency map is
  empty.

Each predicate is a `@property` declared by the relevant altitude
grammar. The spec just references them; the substrate defines them.

---

## 8. The Self-Descriptive Mirror Spec

Mirror's own `mirror.spec`, complete:

```mirror
in @mirror/cli
in @mirror/mosaic
in @property
in @io

project mirror.spec {
  source ~d'shards/'

  legacy ~d'boot/', ~d'bootstrap/' {
    shrinkage_contract: monotonic_lines_decrease,
    retirement_target:  v1.0,
  }

  target binary {
    name     "mirror"
    altitude @code/rust
    emit     cargo

    cli {
      # Mirror is the substrate compiler. It reads grammars, settles
      # them into a graph, and emits artifacts at named altitudes.

      command compile {
        # Compile a grammar against its imports.
        arg path: ~d
        flag strict: bool = true
      }

      command kintsugi {
        # Settle a project. Run mosaic on the spec.
        arg spec: ~f = ~f'./mirror.spec'
        flag target: list(str) = []
        flag emit_shatter: bool = false
      }

      command shatter {
        # Project a settled shard to .shatter format.
        arg oid: content_address
        arg out: ~f
      }
    }
  }

  target action {
    name     "build"
    altitude @ci/github
    emit     yaml
  }

  target release {
    name     "mirror"
    altitude @release
    emit     github_release
    needs    [binary, action]
  }

  settle_on {
    binary.compiles
    binary.tests_pass
    action.validates
    release.signs
    total_transparency.weight == 0
  }
}
```

This is the spec. This is also a `.mirror`-grammar file that
`@mirror/project` parses. The compiler reads it. Mosaic settles it.
Mirror, the binary, comes out.

The binary that comes out understands `mirror.spec` because it was
built from a spec that uses `@mirror/cli` to declare its own CLI. The
binary already speaks the dialect of the spec that built it. There is
no separate spec parser, no separate CLI declaration, no
documentation layer that drifts. The spec, the CLI, and the help text
are all projections of the same `@nl`-tagged AST.

---

## 9. The cli.mirror Migration

`boot/std/cli.mirror` is the current CLI grammar. It becomes
`@mirror/cli` — a grammar consumed by `mirror.spec`'s `cli` block.

What survives from `boot/std/cli.mirror`:
- The `command`, `arg`, `flag` keywords.
- The argument type vocabulary (`str`, `bool`, `list(t)`, `~d`, `~f`,
  `content_address`).
- The default-value syntax (`= value`).

What needs lifting:
- The block-level `#` consumption pattern. `boot/std/cli.mirror`
  predates the `# = @nl` collapse; it likely still treats `#` as a
  comment. The lift is: every `#` inside a `cli`/`command`/`arg`/`flag`
  body becomes the help text for the enclosing element.
- The `cli { ... }` outer block. `boot/std/cli.mirror` declares
  commands at the top level (`cli @binary_name`); the spec embeds them
  inside a `target`. The lift is the embedding pattern: `target` owns
  `cli`; the cli's commands cannot escape the target.
- The dispatch contract. `boot/std/cli.mirror` does not say what
  happens when a command runs. The spec needs to declare:
  `command X` dispatches to `@mirror.X` (or some declared handler). The
  exact contract is open work (see open questions).

Most of the keyword vocabulary is intact. The structural lift is the
embedding (`target` owns `cli`) and the documentation collapse (`#`
IS the help).

---

## 10. Open Questions

1. **Dispatch.** When `mirror kintsugi ./mirror.spec` runs, who handles
   the `kintsugi` command? The spec declares the command exists; some
   handler in `@mirror/mosaic` (or `@mirror`) actually runs it. The
   binding between `cli.command.kintsugi` and the runtime action that
   executes IS the open piece. Likely the action's name matches the
   command's; the substrate-pull is `command kintsugi {...}` IS the
   action declaration, not a separate handler reference.

2. **Subcommands.** `mirror kintsugi --target binary` is one flag; what
   about `mirror kintsugi binary` as a subcommand? Recommendation:
   only top-level commands in v0.1; subcommands defer.

3. **Multiple `source` directives.** Forbidden in v0.1. Whether that
   stays the rule is open — some projects have multiple canonical
   roots (a monorepo with workspaces). Defer to whoever first hits
   the wall.

4. **`legacy` shrinkage_contract types.** What predicates are
   permitted? `monotonic_lines_decrease`, `monotonic_file_count_decrease`,
   `monotonic_opacity_decrease`. Likely all three; declared in
   `@property` so the spec just references them.

5. **`emit` as a grammar reference.** When `emit yaml`, what does that
   mean? The yaml grammar is consulted to render the manifold as YAML.
   When `emit cargo`, the cargo @io tool is invoked. Two different
   things. The lift: `emit` takes a `ref(@io)` OR a `ref(@code/*)`
   grammar; the disambiguation is by type.

6. **Versioning.** Does `mirror.spec` itself need a version field? The
   spec evolves; the parser needs to know which version it is reading.
   Recommendation: no explicit field; the grammar's content-address
   IS the version. If you load an old spec under a new grammar and it
   does not parse, that IS the version mismatch.

7. **`settle_on` extensibility.** Custom predicates by the user (not
   just substrate-declared ones). Likely allowed: any `@property`
   predicate can be referenced, including user-defined ones. Defer
   formal contract to `@property` spec.

8. **Convergence with `spec-as-projection.md`.** That spec says
   `mirror.spec` is a *projection* of the graph; this spec says
   `mirror.spec` is the *input* to mosaic. Both are true: the human
   writes the spec; mosaic settles it; settlement updates the
   projection; the projection is rewritten to disk; next tick reads
   the updated version. The two specs need a reconciliation pass —
   probably this spec is the schema, `spec-as-projection.md` is the
   tick-loop semantics. Filed for next round.

---

*mirror.spec IS the project manifold.*
*One top-level `project` declaration. source / legacy / target / settle_on.*
*The CLI lives inside the binary target — the spec that builds the binary*
*declares the binary's surface.*
*`#` IS @nl. Help text, Fate input, docs — one source.*
*The binary that runs `mirror kintsugi ./mirror.spec` was built FROM*
*`./mirror.spec`. The loop closes at the substrate's edge.*
