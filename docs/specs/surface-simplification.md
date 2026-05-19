# Surface Simplification — Five Operations, Five Verbs, One Beam

*2026-05-19. Reed.*

The CLI surface has been accreting verbs. `build` and `craft` do nearly the same
thing. `check` is `craft` without code emission. `refract` and `kintsugi` both
settle a file. `trace` and `benchmark` are the same observation under two names.

The Prism trait has five operations: **focus, project, split, zoom, refract**.
Every public verb in the binary must map to exactly one of these. The CLI must
have exactly five top-level commands. The internal grammars must justify their
existence by carrying load that the five cannot carry alone.

This spec maps the current state to the target state, names the dissolutions,
and writes the migration as ticks.

---

## Current State

### Grammars currently exported as @mirror/* (boot/std/mirror/)

| Grammar | Role today | Lines | Status |
|---|---|---|---|
| `@mirror/bootstrap` | Minimum binary spec — tokenizer + SHA + kernel + Fate | 24 | keep |
| `@mirror/build` | self-build pipeline: collect → evaluate → emit → assemble → link → store | 36 | **dissolves** |
| `@mirror/check` | resolve + diagnostics, return imperfect | 18 | **dissolves** |
| `@mirror/evaluate` | tokenizer lens: (grammar, text) → ast | 12 | keep (internal) |
| `@mirror/execute` | execute AST, propagate holes, measure execution loss | 21 | keep (internal) |
| `@mirror/grammar` | the meta-grammar: keywords → operations | 13 | keep (internal) |
| `@mirror/interpreter` | walk AST, resolve `\` via Fate, sub-Turing | 30 | keep (internal) |
| `@mirror/liquid` | infer @epistemologic properties, project below `---` | 32 | keep (internal) |
| `@mirror/lsp` | LSP transport — same JSON-RPC pattern as MCP | 39 | keep (internal) |
| `@mirror/refract` | measure / suite / lens / query / infer_spec / match_properties | 49 | **dissolves into kintsugi + beam** |
| `@mirror/resolve` | walk AST, check git for crystals, load or diagnose | 11 | keep (internal) |
| `@mirror/runtime` | full pipeline: resolve → check → interpret | 22 | keep (internal) |
| `@mirror/serve` | MCP server over stdio | 14 | keep (internal) |
| `@mirror/spectral` | crystal storage in git: crystallize / recall / cached | 32 | keep (internal) |
| `@mirror/compile` | **does not exist as a file** — imported by `@mcp`, `@mirror/serve` | — | **must be created or dissolved** |
| `@mirror/trace` | **does not exist as a file** — imported by `@mirror/lsp`, `@cogito` | — | **dissolves into @beam** |

### Grammars at boot/std/ that look like top-level verbs

| Grammar | Role today | Status |
|---|---|---|
| `@craft` | the convergence loop: compile + reflect + tournament + ... → λ₀ | keep — IS `split` |
| `@kintsugi` | collapse(ast, ast) → imperfect — settle the spectrum | keep — IS `refract` |
| `@run` | run a mirror project: compile + verify + report | **dissolves** (overlaps `@mirror/runtime` + `@mirror/execute`) |
| `@fate` | five models, holonomy reduction loop | keep — IS `zoom` |
| `@beam` | luminosity + path + wavelength — the observation surface (currently OTP types) | **expand role** to absorb trace + benchmark |
| `@benchmark` | bench / compare / profile / report / baseline / sample | **dissolves into @beam** |
| `@cli` | CLI grammar — flags are typed lambdas | keep — needs collapse |
| `@cogito` | second-order reflection — observe holes, pick strategy, perturb | keep — used by `craft` |
| `@mcp` | MCP server grammar — JSON-RPC dispatch | keep — alias of `@mirror/serve` |
| `@fragmentation` | AST IS Merkle tree — oid / children / verify | keep (internal) |
| `@shatter` | crystal cache: cache / store / invalidate | **dissolves into @mirror/spectral** |

### CLI commands declared in @cli

The current `@cli` grammar declares **31 distinct actions** across nine
categories (five-optics, compiler, CI/CA, session, deployment, navigation, LSP,
garden, AI, benchmark). The five Prism operations appear as actions
(focus/project/split/zoom/refract) alongside concrete verbs (compile, kintsugi,
crystal, ci, ca, init, tick, tock, shatter, plan, apply, drift, rollback, diff,
log, blame, lsp, repl, add, remove, list, ai, train, bench, profile, help).

This is the duplication. The CLI declares both the operations and the verbs.
The verbs **are** the operations under different names.

### Duplication map

| Verb in @cli | Operation it really is | Notes |
|---|---|---|
| `compile` | focus | tokenize one file → one beam |
| `run` | project | execute with holes |
| `bench` | refract (observation) | the beam over time |
| `profile` | refract (observation) | multi-action beam |
| `kintsugi` | refract | settle a file |
| `crystal` | refract | emit canonical |
| `ci` | split (with --reflect) | many files converging |
| `ca` | split (with --reflect) | many files converging |
| `tick` / `tock` | project | a single execution step |
| `shatter` | refract --shatter N | recursive settle |
| `plan` / `apply` / `drift` / `rollback` | zoom | cross levels of resolution |
| `diff` / `log` / `blame` | focus | look closer at one thing |
| `add` / `remove` / `list` | project | filter the garden |
| `lsp` / `repl` | serve | transport, not operation |
| `ai` / `train` | zoom | resolution change |
| `init` | refract | settle initial state |

Every concrete verb collapses into one of the five operations. The CLI does not
need 31 actions. It needs five.

---

## Target Surface

Five CLI commands, each one Prism operation:

| Command | Operation | What it does | One-liner |
|---|---|---|---|
| `mirror compile <file>` | **focus** | tokenize one file → emit a beam | look closer at one grammar |
| `mirror run <file>` | **project** | execute with holes | the beam shows what's missing |
| `mirror craft <target>` | **split** | enumerate the suite, converge to λ₀ | many beams becoming one |
| `mirror fate <oid> <res>` | **zoom** | cross levels: `\` → concrete | alter the eigenboard the beam refracts through |
| `mirror kintsugi <file>` | **refract** | settle, write canonical | the gold in the cracks |

### Flags compose

Flags are typed lambdas (per `@cli`). Composing flags onto the five commands
replaces every dissolved verb:

| Flag | Composes on | Replaces | Effect |
|---|---|---|---|
| `--target binary` | `craft` | `build` | emit code (LLVM IR → object → binary) |
| `--target rust` | `craft` | (new) | emit Rust |
| `--target gleam` | `craft` | (new) | emit Gleam |
| `--reflect` | `craft` | `check`, `ci`, `ca` | verify properties, no emission |
| `--liquid` | `kintsugi` | (currently `@mirror/liquid` standalone) | write inferred properties below `---` |
| `--shatter N` | `kintsugi` | `shatter` | recursive settle N levels deep |
| `--beam` | any | `bench`, `profile`, `trace` | emit the observation |
| `--format json\|human` | any | (current `@cli` flag) | output format |

### One transport, two dispatches

`mcp` and `lsp` are not commands. They are **dispatches** of the same JSON-RPC
transport (`stdin |> @data/json.parse |> dispatch |> @data/json.emit |> stdout`,
per minimum-binary-surface.md). They are invoked as:

```
mirror serve --mcp
mirror serve --lsp
```

`serve` is **not** a sixth operation — it is `project` over the JSON-RPC
input stream. The dispatch grammar differs; the transport does not.

---

## Dissolutions

Each dissolution must justify itself structurally — not just "they look
similar," but "they are the same Prism operation viewed from different angles."

### `build` → `craft --target binary`

`@mirror/build` already declares its pipeline as
`collect |> evaluate |> emit |> assemble |> link |> store`. `@mirror/craft`
declares its loop as
`compile |> @cogito.reflect |> @fate/tournament |> compile |> repeat_until(settled)`.

The structural identity: **build IS craft with a binary emission step**. Craft
converges the suite to λ₀ (a settled crystal). Build is the same convergence
where the settled crystal happens to be code emitted to a target.

Metaphor justification: split takes many beams and converges them. Emitting a
binary is one particular convergence target. The eigenboard is the same. The
loss is the same. Only the final projection differs.

### `check` → `craft --reflect`

`@mirror/check` is `resolve + diagnostics + loss`. `@mirror/craft` runs
`compile |> reflect |> tournament |> compile` until settled, which already
produces `resolve + diagnostics + loss` as a byproduct.

The structural identity: **check IS craft without emission**. Craft minus the
emission step is exactly: resolve, run the tournament, collect diagnostics,
report loss.

Metaphor justification: split's job is to enumerate many possibilities and
converge. When you suppress the emission, what's left is the verdict — pass,
fail, partial. That verdict IS the check.

### `refract` (the @mirror/refract grammar) → split between `kintsugi` and `@beam`

`@mirror/refract` currently does two things:
1. **Measure**: produce verdicts from topology, infer specs (this is observation)
2. **Settle**: project inferred properties into source (this is settling)

These are two different operations under one name. They split:

- The **measure / suite / lens / query** actions are observation. They become
  `@beam` operations — read the beam over a file, a suite, through a lens.
- The **infer_spec / match_properties / refract** actions are settling. They
  become `@kintsugi --liquid` — settle the grammar by writing inferred
  properties below `---`.

Metaphor justification: refract-as-verb is what kintsugi already does — settle
the spectrum. The current `@mirror/refract` grammar conflates measurement (the
beam) with settling (the gold). One operation each. No fused verbs.

### `trace` → `@beam`

`@mirror/trace` is referenced by `@cogito` and `@mirror/lsp` but **the file does
not exist**. This is current state: a hole. The hole is honest — trace is not a
distinct grammar. It is the beam at runtime.

The structural identity: **trace IS the beam carrying execution observations**.
The beam already has `luminosity: light | dimmed | dark` and `path: [prism]`.
Adding execution holes + topology + loss + timing makes it complete.

Metaphor justification: a trace is what the beam writes as it passes through
the grammar. The trace IS the beam's record. Two names for one thing.

### `benchmark` → `@beam`

`@benchmark` declares `measurement: { name, duration, loss, tick }`. The beam
already has `luminosity` (success/loss state) and `wavelength: precision`.
Adding `duration: u64` and `tick: tick` to the beam absorbs benchmark entirely.

The structural identity: **timing IS observation**. A measurement is the beam
constrained to one action. A profile is the beam over a grammar. There is no
"benchmark grammar" — there is the beam, and timing is one of its dimensions.

Metaphor justification: the wavelength of a beam is its precision. Wavelength
and duration both measure the same thing — how much information the beam
carries per unit of itself. Timing is observation. Observation is the beam.

### `execute` → folded into `run`

`@mirror/execute` is internal (sub-Turing AST walker). `mirror run <file>` is
the CLI entry point. Run calls execute internally. The CLI verb is `run`. The
grammar that does the walking stays `@mirror/execute`. They are not duplicates
— one is the public verb, one is the internal mechanism. **execute stays
internal; run is the public-facing project operation.**

### `@run` (the boot/std/run.mirror) → dissolves

The top-level `@run` grammar duplicates `@mirror/runtime` + `@mirror/execute` +
`@craft`. Its `run_target` action is `craft(target)`. Its `run` action is
`craft + properties + report`. This is `craft --reflect` under a different name.

Delete `boot/std/run.mirror`. The CLI command `mirror run <file>` dispatches
directly to `@mirror/runtime.run(command, args)` which calls
`@mirror/execute.execute`.

### `@shatter` → `@mirror/spectral`

`@shatter` declares `cache / store / invalidate` over crystals. `@mirror/spectral`
declares `crystallize / recall / cached` over the same crystals. These are the
same grammar named twice — one from before the move to git-as-store, one after.

`@mirror/spectral` is canonical (git IS the store, no invalidation needed —
content-addressed). Delete `@shatter`.

### `project` (the old standalone) and `separator` → `@mirror/liquid`

Already done per CLAUDE.md note. The dissolved grammars folded into
`@mirror/liquid`. Mentioned here for completeness.

---

## The @beam Grammar

`@beam` becomes **the observation surface for the entire compiler**. It absorbs
`trace`, `benchmark`, and the measurement half of the old `@mirror/refract`.

### What the beam carries

```mirror
in @prism
in @meta
in @time
in @fragmentation

# the beam: a value carrier through a pipeline
# every observation the compiler makes IS a beam record
type beam(t) {
  # luminosity: what came through
  # light(value, loss)        — success
  # dimmed(value, error, loss) — partial, value survived with information loss
  # dark(error, loss)          — failure, only the error remains
  luminosity: luminosity,

  # path: the prisms the beam passed through
  # one prism per pipeline stage
  path: [prism],

  # wavelength: precision of the observation
  # u64 — narrower wavelength means tighter observation
  wavelength: precision,

  # holes: the \ holes the beam encountered
  # each hole is an OID — the content-addressed identity of the unknown
  holes: [oid],

  # topology: the structural fingerprint of the AST
  # eigenvalue-style summary the beam saw
  topology: eigenvalues,

  # loss: aggregated loss along the path
  # holes contribute. dimmed contributes. dark = 1.0.
  loss: loss,

  # timing: when the beam was emitted, how long it took
  # (this absorbs @benchmark — measurement IS a beam record)
  emitted: tick,
  duration: duration,

  # fractures: where the AST cracked
  # cracks are not failures — they are where kintsugi writes gold
  fractures: [oid],
}

type luminosity(t, e, l) = light(t, l) | dimmed(t, e, l) | dark(e, l)
luminosity = imperfect

grammar @beam {
  # emit a beam from an AST + execution result
  # this is what every operation produces as a byproduct
  emit(ast, imperfect) -> beam { \ }

  # observe a beam: read its contents
  # used by Reflection (@cogito), kintsugi, liquid
  observe(beam) -> imperfect { \ }

  # compare two beams: did the loss decrease?
  # used by @mirror/craft to detect settlement (e^(n+1) < e^(n))
  compare(beam, beam) -> speedup { \ }

  # profile: collect beams over many actions
  # absorbs @benchmark.profile
  profile(grammar) -> [beam] { \ }

  # baseline: pin a beam as the reference
  # absorbs @benchmark.baseline
  baseline(beam, label: ref) -> beam { \ }

  # sample: emit n beams, return the median
  # absorbs @benchmark.sample
  sample(action, n: u64) -> beam { \ }
}
```

### How the rest of the compiler reads the beam

**`@cogito` (Reflection) reads the beam to choose strategy.**
The current `@cogito.observe` already calls `@mirror/trace.trace`. That
becomes `@beam.observe`. The strategy decision (`elite(1).beam(8).halving(3)`)
already uses the word "beam" — the metaphor is already in the names.

**`@mirror/craft` reads the beam to detect convergence.**
`settled = e^(n+1) = e^(n)` — two consecutive beams with equal loss.
`@beam.compare(beam_n, beam_n_plus_1)` returns the speedup ratio. When the
ratio is 1.0 (no improvement), settled.

**`@kintsugi` consumes the beam to write gold.**
The beam carries `fractures: [oid]` — the cracks where Fate proposed
resolutions during the craft loop. Kintsugi reads those fractures and writes
the resolutions back into the source file. The beam is what kintsugi listens
to.

**`@mirror/liquid` writes from the beam.**
The beam carries `topology: eigenvalues`. Liquid matches topology against
`@epistemologic/*` property thresholds. The properties that match get
projected below `---` in the source. The beam is liquid's input.

**`@mirror/lsp` and `@mirror/serve` emit beams over JSON-RPC.**
LSP `textDocument/didOpen` returns a beam. `textDocument/diagnostics` reads
the beam's `luminosity` for verdicts. MCP tools return beams as JSON.

### The beam IS the pipeline

```
mirror compile <file>
  → @mirror/evaluate.evaluate(grammar, text)
  → ast
  → @beam.emit(ast, imperfect.light(ast, 0.0))
  → beam

mirror run <file>
  → @mirror/runtime.compile(file)
  → @mirror/execute.execute(ast)
  → imperfect with holes
  → @beam.emit(ast, imperfect)
  → beam with holes: [oid], loss > 0

mirror craft <target>
  → @mirror/runtime.compile(spec)
  → @beam.emit (beam_0)
  → @cogito.reflect(imperfect)
  → @fate/tournament.tournament
  → @mirror/runtime.compile (beam_1)
  → @beam.compare(beam_0, beam_1)
  → repeat until @beam.compare returns 1.0
  → settled beam → crystal

mirror fate <oid> <resolution>
  → @mcp.fate(oid, resolution)
  → store at refs/fate/<oid>
  → next craft tick reads from refs/fate/<oid>
  → beam's holes shrink
  → @beam.compare shows loss decreasing

mirror kintsugi <file>
  → @mirror/runtime.compile(file)
  → @beam.emit (with fractures)
  → @kintsugi.collapse(ast, fractures)
  → rewrite source
  → @beam.emit (with loss 0.0)
  → crystal
```

The beam threads through every operation. Five operations. One beam.

---

## Internal Grammars (kept)

These stay internal — they implement the five, they are not themselves verbs.

| Grammar | Why it stays | What it implements |
|---|---|---|
| `@mirror/evaluate` | the tokenizer — (grammar, text) → ast | `compile` (focus) |
| `@mirror/execute` | the AST walker — bounded, sub-Turing | `run` (project) |
| `@mirror/interpreter` | the interpreter loop — walks AST, resolves `\` via Fate | `run` (project) |
| `@mirror/resolve` | walks AST for `In` nodes, checks git for crystals | all five |
| `@mirror/runtime` | the pipeline: resolve → check → interpret | composes the five |
| `@mirror/grammar` | the meta-grammar: keywords → operations | the substrate |
| `@mirror/spectral` | crystal storage in git (replaces @shatter) | settles into git blobs |
| `@mirror/bootstrap` | minimum binary spec — tokenizer + SHA + kernel + Fate | the binary itself |
| `@mirror/serve` | MCP transport — same as LSP's pattern | dispatched by `serve` |
| `@mirror/lsp` | LSP dispatch — JSON-RPC method routing | dispatched by `serve` |
| `@mirror/liquid` | infer properties + project below `---` | consumed by `kintsugi --liquid` |
| `@cogito` | second-order reflection — observe, strategize, perturb | consumed by `craft` |
| `@fate` | the holonomy reduction loop, five models | consumed by `craft` (split → zoom) |
| `@fragmentation` | AST IS Merkle tree — oid / children / verify | substrate for the beam's hole and fracture OIDs |

Internal grammars are tools the five operations call. They are not surface.
A user types `mirror compile`, `mirror run`, `mirror craft`, `mirror fate`,
`mirror kintsugi`. The user does not type `mirror evaluate` or
`mirror interpreter` or `mirror runtime`. Those are how the five are built.

### `@cli`: collapse to five

The current `@cli` declares 31 actions. After this spec, it declares:

```mirror
grammar @cli {
  # --- The five operations ---
  action compile(path: text)       -> imperfect  # focus
  action run(path: text)           -> imperfect  # project
  action craft(target: text)       -> imperfect  # split
  action fate(oid: text, res: text)-> imperfect  # zoom
  action kintsugi(path: text)      -> imperfect  # refract

  # --- Transport (one verb, two dispatches) ---
  action serve()                   -> imperfect  # MCP or LSP via flag

  # --- Flags as typed lambdas ---
  flag target(binary | rust | gleam) = lens(grammar => grammar)
  flag reflect                       = prism(imperfect => pass | fail)
  flag liquid                        = prism(imperfect => imperfect)
  flag shatter(int)                  = lens(imperfect => imperfect)
  flag beam                          = lens(imperfect => beam)
  flag format(json | human)          = lens(imperfect => text)
  flag mcp                           = prism(imperfect => imperfect)
  flag lsp                           = prism(imperfect => imperfect)

  # --- Help derived from grammar ---
  action help(grammar: ref) -> text { \ }
}
```

Six actions. Eight flags. The rest is grammar.

---

## The Metaphor End-to-End

The metaphor must be load-bearing — each name maps to a structural role.

```
glass    = grammar         the shape, what you wrote
beam     = observation     the light passing through
wine     = eigenboard      what the beam refracts through
pitch    = eigenvalue      what the beam reveals
gold     = kintsugi        the canonical form, settled
liquid   = properties      the wine settled below ---
```

| Concept | Metaphor | Structural role | Grammar |
|---|---|---|---|
| **glass** | the shape | what was written — the .mirror file | source file |
| **beam** | the light | observation record — luminosity + path + wavelength + holes + topology + loss + timing + fractures | `@beam` |
| **wine** | the eigenboard | the weights the beam refracts through — Fate's models, the tournament | `@fate` |
| **pitch** | the eigenvalues | what the beam reveals — topology fingerprint, property thresholds | `@fragmentation`, `@epistemologic` |
| **gold** | the canonical form | the settled crystal — written to git, content-addressed | `@mirror/spectral` |
| **liquid** | the properties | what the compiler infers from the topology — projected below `---` | `@mirror/liquid` |

### Operations refract the metaphor

| Operation | Verb | Acts on glass | Produces |
|---|---|---|---|
| **focus** | `compile` | tokenize one piece of glass | one beam |
| **project** | `run` | shine the beam through glass with holes | beam with holes visible |
| **split** | `craft` | many beams converge through one eigenboard | one beam, settled |
| **zoom** | `fate` | alter the eigenboard — replace a `\` with concrete | future beams refract differently |
| **refract** | `kintsugi` | settle the beam into the gold | source rewritten, crystal stored |

### The full pipeline

```
glass (source) → focus → beam
beam → project → beam with holes
beam with holes → split → many beams in tournament
many beams → zoom (Fate resolves) → fewer holes
fewer holes → split repeats → converges
converged beam → refract → gold (canonical) + liquid (properties below ---)
gold → @mirror/spectral.crystallize → git blob → OID
```

Five operations. One beam. Always.

---

## The Migration Path

Ordered ticks. Each tick is a commit. Each commit moves one piece.

### Tick 1: 🔧 expand `@beam`

- Edit `boot/std/beam.mirror` to absorb `trace` + `benchmark` types
- Add: `holes`, `topology`, `loss`, `emitted`, `duration`, `fractures` fields
- Keep existing OTP types (process, gen_server, supervisor) — they describe how
  the beam runs, not what it carries. Or move them to a new `@otp` grammar.
- Add: `emit`, `observe`, `compare`, `profile`, `baseline`, `sample` actions

### Tick 2: 🔧 create `@mirror/compile` and `@mirror/trace`

These are currently imported but don't exist. The imports are honest holes —
making them concrete:

- `boot/std/mirror/compile.mirror`:
  ```mirror
  in @prism
  in @mirror/evaluate
  in @beam

  grammar @mirror/compile {
    # focus: tokenize one file, emit a beam
    compile(file) -> beam {
      @mirror/evaluate.evaluate |> @beam.emit
    }
  }

  out compile
  ```

- `boot/std/mirror/trace.mirror` is **NOT** created. Instead, update the two
  files that import it:
  - `boot/std/mirror/lsp.mirror`: `in @mirror/trace` → `in @beam`,
    `@mirror/trace.trace(file)` → `@beam.emit(file)`
  - `boot/std/cogito.mirror`: same substitution

### Tick 3: ♻️ dissolve `@mirror/build` into `@mirror/craft`

- Add `--target` flag handling to `@mirror/craft.craft`
- Move `assemble` and `link` actions into `@mirror/craft` (or into a separate
  `@code/llvm` grammar — they're code-emission concerns, not craft concerns)
- Delete `boot/std/mirror/build.mirror`
- Update `@cli`: remove `build` action

### Tick 4: ♻️ dissolve `@mirror/check` into `@mirror/craft`

- Add `--reflect` flag handling to `@mirror/craft.craft` — suppresses
  emission, returns `imperfect` verdict
- Delete `boot/std/mirror/check.mirror`
- Update `@mirror/runtime.compile`: replace `@mirror/check.check` with
  `@mirror/craft.craft --reflect`
- Update `@cli`: remove `check`, `ci`, `ca` actions

### Tick 5: ♻️ split `@mirror/refract`

- Move `measure`, `suite`, `lens`, `query` actions → `@beam`
- Move `infer_spec`, `match_properties`, `refract` → `@kintsugi --liquid`
- Delete `boot/std/mirror/refract.mirror`
- Update every import: `@mirror/refract.measure` → `@beam.observe`,
  `@mirror/refract.refract` → `@kintsugi --liquid`

### Tick 6: ♻️ dissolve `@benchmark` into `@beam`

- Confirm all `@benchmark.*` actions have `@beam` equivalents
- Delete `boot/std/benchmark.mirror`
- Update every import

### Tick 7: ♻️ dissolve `@shatter` into `@mirror/spectral`

- Confirm `@mirror/spectral.crystallize` covers `@shatter.cache + store`
- `@shatter.invalidate` deletes (content-addressed, no invalidation needed)
- Delete `boot/std/shatter.mirror`
- Update every import

### Tick 8: ♻️ dissolve `@run` (top-level) into `@mirror/runtime`

- Confirm `@mirror/runtime.run(command, args)` covers the use case
- Delete `boot/std/run.mirror`
- Update `@cli.run` to dispatch to `@mirror/runtime.run`

### Tick 9: ♻️ collapse `@cli` to five

- Reduce `@cli` actions to: `compile`, `run`, `craft`, `fate`, `kintsugi`,
  `serve`, `help`
- Reduce flags to: `target`, `reflect`, `liquid`, `shatter`, `beam`, `format`,
  `mcp`, `lsp`
- Remove: `crystal`, `ci`, `ca`, `init`, `tick`, `tock`, `shatter` (verb),
  `plan`, `apply`, `drift`, `rollback`, `diff`, `log`, `blame`, `lsp` (verb),
  `repl`, `add`, `remove`, `list`, `ai`, `train`, `bench`, `profile`
- Any deleted verb that has a real use case re-emerges as a flag composition
  or a grammar action invoked by the five

### Tick 10: 🟢 verify the spec holds

- `mirror compile boot/std/beam.mirror` — must emit a beam
- `mirror run boot/std/cli.mirror` — must show the beam's holes
- `mirror craft boot/` — must converge to λ₀
- `mirror craft boot/ --target binary` — must emit the binary
- `mirror craft boot/ --reflect` — must verify properties without emission
- `mirror fate <oid> <resolution>` — must seed Fate
- `mirror kintsugi boot/std/cli.mirror` — must settle, write canonical
- `mirror kintsugi boot/std/cli.mirror --liquid` — must write properties below `---`
- `mirror kintsugi boot/ --shatter 1` — must recursively settle
- `mirror serve --mcp` — must speak JSON-RPC, dispatch to mirror tools
- `mirror serve --lsp` — must speak JSON-RPC, dispatch to LSP methods

When every command above lands with `loss = 0.00`, the surface is settled.

### What changes in the MCP surface

The MCP tools generated by `@mirror/serve` reduce from the current verb-per-tool
mapping to the five-operations-as-tools:

- `compile(file)` → beam JSON
- `run(file)` → beam JSON with holes
- `craft(target, flags)` → beam JSON, settled
- `fate(oid, resolution)` → imperfect
- `kintsugi(file, flags)` → beam JSON, canonical form written

Any agent with MCP access gets the same five operations the human gets. The
MCP and the CLI are the same surface — JSON-RPC is just one rendering of it.

---

## The Counts

| Surface | Before | After | Delta |
|---|---|---|---|
| CLI top-level verbs | 31 | 5 (+ `serve`, `help`) | -24 |
| `@mirror/*` grammars | 14 | 11 | -3 |
| Top-level `boot/std/*` verb grammars | 7 (`craft`, `kintsugi`, `run`, `fate`, `benchmark`, `cogito`, `shatter`) | 5 (`craft`, `kintsugi`, `fate`, `cogito`, `beam`) | -2 |
| Grammars dissolved | 0 | 6 (`build`, `check`, `refract`, `trace`, `benchmark`, `shatter`, `run`) | +6 dissolved |
| Grammars created | 0 | 1 (`@mirror/compile`) | +1 |

The compiler shrinks by twenty-four verbs and dissolves six grammars. The beam
absorbs three of them (trace, benchmark, half of refract). `kintsugi --liquid`
absorbs the other half of refract. `craft` with flags absorbs build and check.
`@mirror/spectral` absorbs shatter. `@mirror/runtime` absorbs run.

Five commands. Five operations. One beam. The metaphor holds because the
operations are the metaphor — `focus` IS looking closer, `project` IS shining
the beam through, `split` IS the eigenboard convergence, `zoom` IS crossing
levels, `refract` IS settling.

The verbs are not metaphors layered on top. They are the trait methods named
in English.
