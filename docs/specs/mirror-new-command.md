# mirror new — the project-scaffolding command + the canonical mirror-project shape

*2026-05-28. Mara. Spec — design, not implementation. No Rust.*

Status: **Yellow** (the canonical-project shape is grounded in `mirror.spec`
itself + the `cosmos-mirror-scaffold` requirements; the kintsugi-generated-`.spec`
mechanism reconciles a documented `mirror init` framing with prior `@new`-grammar
art. The one place this spec *changes* a prior design — folding the static
`@new_template` into the refraction — is called out explicitly as a finding.)

Depends on:
- `docs/specs/spectral-db-three-tier-architecture.md` §"CLI surface" — the
  documented framing: *"mirror init is what creates the project's .spec file
  through a first kintsugi application that refracts into the .spec"* (Alex
  2026-05-26), the `@mirror/init` scene grammar, and the `--store` flag.
- `docs/specs/spec-inference.md` — `mirror settle <path>` infers a `.spec`
  from a project's measured topology (above `---` declared, below `---`
  measured). `new` and `settle` are the two halves of one settlement.
- `docs/specs/cosmos-mirror-scaffold.md` (HEAD, `7357130`) — the first real
  user project; what `mirror new cosmos-mirror` must be able to produce.
- `mirror.spec` (repo root) — mirror describing itself; the canonical shape a
  scaffolded `.spec` is folded from.
- `boot/std/new.mirror` + `boot/std/new.template.mirror` (commit `87c2686`,
  branch `reed/mirror-new`) — the prior `@new` grammar. Pre-five-op-collapse,
  pre-`@io`/`@mirror/store` correction. Reconciled below, not adopted whole.
- `AGENTS.md` §"Boundary Rust is not frozen capability", §"The Glass Wall" —
  the `[substrate-pull:realize]` / `@io` floor boundary that decides where the
  filesystem writes sit.
- `docs/specs/surface-simplification.md` — the five-verb collapse; `init` was
  one of the 31 verbs that dissolved into `settle`. This spec re-derives `new`
  as a composition, not a sixth operation.
- `docs/specs/merkle-package-manager.md` — Store IS a MerkleTree; a project's
  dependency set is a tree of `in @…` imports, resolved by traversal, not SAT.

Unblocks:
- cosmos-mirror's migration step 0: produce the skeleton the scaffold spec's
  §4 `.spec` + `@cosmos` grammars drop into.
- A canonical answer to "what IS a mirror project," pinned by the command that
  creates one — every user project follows whatever `new` produces.

---

## 0. Headline

`mirror new <name>` does not write boilerplate. It runs the project's **first
refraction**: a seed (the name + the user's stated intent + the chosen store)
is settled by one kintsugi pass into a fixed-point `.spec`. The `.spec` is the
**crystal of the init scene** — content-addressed, provenance-tied, and
structurally a `fixed` declaration of the project. Everything else `new`
creates — the `.spectral/` session dir, the seed grammar that names the
project's namespace, the `.gitignore`, `git init` — is the *floor* the
refraction lands on. The scaffold *logic* is grammar (`@new` composing the five
operations); only the filesystem writes and `git init` are `@io`.

This is the move that makes `new` *mirror* and not `cargo new` reskinned. Every
other tool drops static text. mirror settles a measurement. The `.spec` is the
same kind of object whether `new` writes it from a seed or `settle` infers it
from an existing tree — one settle operation, two entry points (§3).

The vocabulary stays closed: `new` is **not** a sixth operation. It is a
named composition of `focus . split . shift . settle` (§4), exactly as `craft`
is in `mirror.spec`.

---

## 1. The landscape (surveyed, cited)

Project-scaffolding commands sort along three axes: **minimal ↔ batteries**,
**flag-driven ↔ interactive**, **static-template ↔ template-system**. mirror's
distinctive axis — *generated-by-settlement vs written-as-boilerplate* — has no
prior occupant; every tool below writes static text (dotnet's template engine
is the closest, and it still expands a fixed template, it does not *converge*
one).

| command | what it creates | philosophy | interactive vs flags | distinctive idea worth borrowing |
|---|---|---|---|---|
| **cargo new `<name>`** | new dir; `Cargo.toml`, `src/main.rs` (or `src/lib.rs` w/ `--lib`), `.gitignore`; runs `git init` | batteries-light; one manifest + one source file | flag-driven (`--lib`/`--bin`, `--name`, `--vcs`) | the **new-vs-init split**: `new` makes a dir, `init` adapts the cwd; with a path arg `init` behaves like `new` [1][6] |
| **cargo init [`<path>`]** | same files, in an *existing* dir; writes `.gitignore` only if no git repo present; does **not** force a new git repo | in-place adoption | flag-driven | "adopt what's here" — never clobber an existing VCS [6][9] |
| **mix new `<name>`** | `lib/<name>.ex`, `test/`, `mix.exs`, `README.md`, `.gitignore`, `.formatter.exs` | batteries; full tree incl. test + formatter config | flag-driven (`--sup`, `--umbrella`, `--app`, `--module`) | **`--sup`** generates a supervision-tree `Application` callback; **`--umbrella`** makes an `apps/` workspace of sibling apps — *structure expresses the runtime model* [3][5] |
| **go mod init `<path>`** | exactly one file: `go.mod` (module path + go directive) | the anti-batteries extreme | single positional arg | minimalism: the manifest is the whole scaffold; everything else is the user's [11][14] |
| **dotnet new `<template>`** | depends on template (`console`, `classlib`, `webapi`, …); installable template **packs** via `dotnet new install <nupkg>` | template-**system**, not a fixed tree | flag-driven, per-template options | **installable template packs** distributed as NuGet packages — the template set is itself an extensible registry [7][9] |
| **deno init** | two files: `main.ts`, `main_test.ts` (+ `deno.json` debated) | minimal, single-file-ish | nearly argless (`--npm`, `--lib`, `--serve`) | "one file to get going" — config is optional, not mandatory [1][4][5-deno] |
| **bun init** | `package.json` (`type: module`), `tsconfig.json`/`jsconfig.json`, entrypoint, `README`, `.gitignore` | batteries-light, fast | mostly argless, sensible defaults | speed + sensible defaults; no questions unless `-y` is omitted [2][6-bun] |
| **swift package init --type `<t>`** | `Package.swift` + `Sources/`/`Tests/` per type | template-by-type | flag-driven (`--type`) | **typed templates**: `executable`, `library`, `tool`, `build-tool-plugin`, `command-plugin`, `macro`, `empty` — one flag picks a project *kind* [15] |
| **zig init** | `build.zig`, `build.zig.zon`, `src/` | batteries-light; build script is first-class | argless (a `-m` minimal mode was discussed) | the **build script is part of the scaffold** (`build.zig` is code, not config) [13][16] |
| **nimble init / cabal init / lein new / flutter create / rails new** | spectrum endpoints: nimble/cabal *interactive* (ask questions), lein/rails *batteries-heavy from a named template*, flutter `create` *platform-tree* | mixed | nimble/cabal **interactive prompts**; lein/rails template-named | **interactive elicitation** (cabal/nimble ask before writing) is the UX opposite of go's silence |

Synthesis of borrowable ideas, ranked by fit to mirror:

1. **cargo's new-vs-init split** — adopt directly (§3). It is the cleanest
   resolution of "create a dir" vs "adapt the cwd," and it maps onto a real
   distinction in mirror (`new` seeds a fresh crystal; `init`/`settle` settles
   over what exists).
2. **mix's structure-expresses-the-runtime** (`--sup`, `--umbrella`) — mirror's
   analog is *which grammars the seed imports* (a pure project vs one with a
   numerical floor vs an actor/scene project). High relevance: mirror's runtime
   body is BEAM, and the project shape should be able to say "this is a
   five-op-only project" vs "this needs `@code/fortran`."
3. **swift/dotnet typed-templates** — adopt as `--from <template>` (§3), where a
   *template is itself a grammar* (`@new/template/<kind>`), not a directory of
   files. This is dotnet's installable-pack idea made content-addressed: a
   template is a crystal in the garden, resolved by OID like any dependency.
4. **go's minimalism** — adopt as the *default*: the smallest honest skeleton is
   a `.spec` + `.spectral/`, nothing more. Batteries are opt-in flags, not the
   default (§2).
5. **cabal/nimble interactivity** — *reject as default*, keep as opt-in. mirror's
   seed-and-settle is deterministic and content-addressed; an interactive
   prompt that injects free-form intent breaks "same inputs → same OID." A
   `--interactive` flag may elicit the seed *intent string*, but the refraction
   over that seed stays deterministic (§3).

---

## 2. What a mirror project IS (the canonical skeleton)

A mirror project is the smallest tree that lets the compiler answer three
questions: *what is this project's namespace, what does it depend on, and where
does it settle its crystals?* That is a `.spec` and a `.spectral/`. Everything
else is optional.

```
<name>/
├── <name>.spec          REQUIRED. the project's identity + dependency tree +
│                        properties. the crystal of the init refraction.
├── <name>.mirror        REQUIRED. the seed grammar: declares the project
│                        namespace @<name> and re-exports its public surface.
├── .spectral/           REQUIRED. session state (initialized empty-but-valid).
│   ├── gestalt/         crystals — the user's understanding state
│   ├── sessions/        per-session data
│   ├── crystals/        crystallized subgraphs
│   ├── HEAD             current session timestamp (the init timestamp)
│   └── log              tick log (TSV: timestamp, event, message, growth)
├── .gitignore           OPTIONAL-but-default. ignores build floor artifacts.
└── flake.nix            OPTIONAL. ONLY when the project declares a native
                         floor (a `@code/fortran` / `@prism/rust` dependency
                         that needs flang/LAPACK). Pure five-op projects omit it.
```

What is deliberately **absent** versus the prior `@new` grammar (`87c2686`):

- **No `mirror/` source directory.** The old `@new` created `mirror/.gitkeep`
  and pointed `craft`/`run` at `mirror/*.mirror`. Post-collapse, the project's
  grammars live at the root next to the `.spec` (as mirror itself does — `boot/`
  is mirror's, but a *user* project is flat: `cosmos.spec` + `*.mirror`). A
  `mirror/` subdir is a convention the project may adopt, not one `new` imposes.
- **No `target/`-style build dir created.** Per AGENTS.md "Do NOT create
  filesystem caches": git IS the store. `.spectral/crystals/` is session
  understanding, not a compiled-artifact cache. Build emission (`craft --target
  binary`) writes git crystals; it does not seed a `target/`.
- **No `Cargo.toml`.** A mirror project is not a Rust crate. The *only* reason a
  native manifest appears is a declared numerical floor, and then it is a
  `flake.nix` (the household substrate), not a `Cargo.toml` — see §5.

The required minimum is therefore **two files and one dir**: `<name>.spec`,
`<name>.mirror`, `.spectral/`. This is closer to `go mod init`'s minimalism than
`mix new`'s tree — and deliberately so: mirror's *batteries are imports*, not
files. A project gains capability by adding `in @…` lines to its `.spec`
(resolved as MerkleTree traversal, per `merkle-package-manager.md`), not by
`new` pre-writing source files the user will delete.

### The `.spec` shape (folded from `mirror.spec`)

A scaffolded `.spec` is `mirror.spec` *folded to a single project*. mirror's own
spec (`in @prism` … `cli = @mirror/cli { … }`) is the worked example; a user
project's `.spec` is the same structure with the project's namespace as the
construct and its dependency imports above it:

```mirror
in @prism                  -- always. the five operations.
in @<name>                 -- this project's own namespace grammar (the seed)

# <name>: <one-line intent, captured from the seed>
# settled at <init-timestamp> — crystal <oid>

type <name>_target = default

out @cli/*                 -- the project's CLI surface (if any)

<name> = @<name> {
  default <name>/*.mirror  -- what craft/run settle by default
}
```

Above `---` (if present) is what the engineer declared at `new` time. Below
`---` is what `settle` measures later (`spec-inference.md` §"The Spec Format").
At `new` time the below-`---` block is *empty* — there is no topology to measure
yet. The first `mirror settle .` after the project has grammars fills it. This
is the loop: `new` seeds the spec; `settle` sharpens it; `kintsugi` resolves
the holes it names. `e^(n+1) < e^n`.

### Navigation + refs from day one (§6 expanded here for the skeleton)

`new` initializes the navigation model (`CLAUDE.md` "Navigation References") so
the project is addressable immediately:

- `~` (home / gestalt root) → `.spectral/gestalt/`, created empty-but-valid.
- `@` (author / grammar origin) → resolves to `@<name>`, the seed grammar; the
  project IS its own grammar origin from the first commit.
- `HEAD` → `.spectral/HEAD`, set to the init timestamp.
- `^` (last crystal) → the init crystal (the `.spec`'s OID). The project's first
  `^` is its own genesis — the refraction that created it.
- `...` (garden / others' paths) → the project's dependency imports resolve into
  the garden (`@prism`, later `@code/fortran`) as Lens nodes per
  `merkle-package-manager.md` §5; `new` does not copy them, it references them.

So from the moment `new` returns, `mirror focus .`, `~`, `@`, and `^` all
resolve. The project is a navigable graph at t=0, not an inert directory.

---

## 3. `new` vs `init` (and `settle`) — resolved

There are three commands in the neighborhood, and the prior docs reference all
three. They resolve cleanly along the **cargo new/init axis crossed with
mirror's settle direction**:

| command | direction | precondition | what it settles | analog |
|---|---|---|---|---|
| `mirror new <name>` | **create** | dir `<name>` does not exist (or is empty) | a fresh seed → `.spec` | `cargo new` |
| `mirror init [.]` | **adapt** | cwd is a non-mirror project (has source) | the existing tree → `.spec` | `cargo init` |
| `mirror settle <path>` | **measure** | a `.spec` exists; re-infer below-`---` | the measured topology → updated `.spec` | (no cargo analog) |

The resolution in one sentence: **`new` and `init` are the same settlement with
different seeds; `settle` is that settlement's *re-run* over a now-populated
tree.**

- `new <name>` seeds from *nothing but a name + intent + store choice*. It
  creates the directory, then refracts the seed into `<name>.spec`. The
  below-`---` block is empty (no topology yet).
- `init .` seeds from *what is already in the cwd* — it scans the existing source
  (the `spec-inference.md` language-detection pipeline), so its first refraction
  already produces a populated below-`---` block. `init` is `new` whose seed is
  "the project that's already here." This is exactly cargo's rule that `init`
  with no special handling behaves like `new` over the cwd [6].
- `settle` is `init`'s measurement step run again, later, after grammars have
  been added. It never creates a `.spec`; it updates one.

The shared mechanism is the kintsugi settlement (§4). `new` ⊂ `init` ⊂ `settle`
as *seed richness* grows: name-only → existing-source → measured-topology. One
operation, three entry points, monotone in how much the seed already knows.

This **reconciles** the two prior framings without overriding either:

- `spectral-db-three-tier-architecture.md` documents *`mirror init`* as the
  kintsugi-settle-into-`.spec`. That framing is correct and is preserved — it
  is the *in-place* entry point. This spec adds `new` as the *create-a-dir*
  entry point sharing the identical mechanism, exactly the cargo split.
- The prior `@new` grammar (`87c2686`) had `new` writing a *static*
  `@new_template` spec. This spec **supersedes that one design choice**: the
  template is not written verbatim, it is the *seed* the refraction settles. See
  §4 and the findings (§7). The grammar's *composition* (`focus . split . shift .
  settle`) survives almost intact; only the "static template" body changes to
  "settle the seed."

### Flags (stance: flag-driven by default, deterministic; `--interactive` opt-in)

```
mirror new <name>                      # default: --store git, pure five-op seed
mirror new <name> --store nix          # production-shaped floor from day one
mirror new <name> --store sqlite       # embedded / portable
mirror new <name> --store bare         # tiny / no-deps
mirror new <name> --from <template>    # seed from a template grammar (see below)
mirror new <name> --native fortran     # declare a @code/fortran floor (adds flake.nix)
mirror new <name> --interactive        # elicit the intent string before refracting
mirror init [.]                        # adapt the current directory in place
```

Stance and justification:

- **Flag-driven, not interactive, by default.** mirror's defining property is
  content-addressing: same inputs → same OID (`spectral-db-three-tier` §"Properties
  this gives"). A free-form interactive prompt injects non-reproducible intent.
  Flags keep the seed an explicit, hashable value. `--interactive` is offered
  for humans who want elicitation, but the elicited intent string becomes part
  of the seed crystal — the refraction over it is still deterministic.
- **No `--lib`/`--bin` split.** That distinction is Rust-shaped (a crate is one
  or the other). A mirror project's "kind" is *which grammars it imports*, not a
  binary/library toggle. The analog of `--bin` is "this project has a CLI
  surface" → `out @cli/*` in the seed; the analog of `--lib` is "this project
  only exports grammars" → no `@cli`. These are template choices (`--from`),
  not a binary flag.
- **`--from <template>` is dotnet's installable-pack idea, content-addressed.**
  A template is a grammar `@new/template/<kind>` (e.g. `@new/template/numerical`,
  `@new/template/actor`, `@new/template/cli`). It is resolved from the garden by
  OID like any dependency (`merkle-package-manager.md`), not unpacked from a
  bundled directory. `--from numerical` seeds a project that already imports
  `@prism` + `@code/fortran` + the newtype scaffold; `--from cli` seeds one with
  `@cli` + `out @cli/*`. The template set is extensible by publishing a
  `@new/template/*` grammar to the garden — no change to the binary.
- **`--store` is preserved verbatim** from `spectral-db-three-tier` §"CLI surface":
  it selects the project's settlement backend (`git` default, `nix`/`sqlite`/`bare`).
  It writes the `store { … }` block of the `.spec`. This is the one piece of the
  old `@new_template` that was already correct.

---

## 4. The distinctive mechanism — the kintsugi-generated `.spec`

This is the part that makes `new` mirror. Other languages expand a template;
mirror *settles* one. The `.spec` is the fixed point of one kintsugi refraction
over a seed.

### The seed

The seed is the smallest object that distinguishes this project from every
other: a record carrying the name, the captured intent, the store choice, and
the template kind. No bare types (per `feedback-no-bare-types`):

```mirror
# boot/std/new/seed.mirror — the input to the first refraction
in @prism
in @kintsugi

type project_name = newtype(text)          -- NOT bare text
type intent       = newtype(text)          -- one-line "what is this," hashable
type store_kind   = git | nix | sqlite | bare
type template_kind = pure | cli | numerical | actor

type seed = record {
  name:     project_name,
  intent:   intent,
  store:    store_kind,
  template: template_kind,
}

out seed
```

### The first refraction

The settlement is one application of `@kintsugi` whose `flow` (the convergent
Prism operation, `fixed = settle`) lands the `.spec`. The seed enters with a
`\` hole where the dependency tree and properties belong; the settlement settles
that hole into the concrete spec. The grammar — the descendant of the old
`@new`, now five-op-correct and `@io`-correct:

```mirror
# boot/std/new.mirror — scaffold a mirror project as a refraction
in @prism
in @kintsugi
in @new/seed
in @new/template          -- the @new/template/<kind> family (garden-resolvable)
in @io                    -- the floor: filesystem writes + git init
in @mirror/store          -- the floor: store init per --store

# A new project is the crystal of its init scene (spectral-db-three-tier
# §"mirror init IS a kintsugi application"). The composition is the four
# operations; settle is the settle that produces the .spec.
action new(s: seed, root: path) -> imperfect {
  focus  scaffold(s, root)                 -- name the target; choose template grammar
  split  |                                 -- the floor writes (each becomes @io in codegen)
    io mkdir(root)
    io write(root, dotspectral)            -- .spectral/ tree, empty-but-valid
    io write(root, gitignore)              -- the build-floor ignore
  shift  git_init(root)                    -- cross to VCS register: git init + store init
  settle spec_of(s)                        -- SETTLE: the seed -> the fixed-point .spec
}

# the settle. NOT a static template write — a kintsugi pass over the seed.
# the body is the hole the first refraction fills; the result is the .spec
# crystal, content-addressed by `s`.
settle spec_of(s: seed) -> crystal {
  @kintsugi.settle(@new/template.for(s.template, s.name, s.store), \)
}

out @new
```

What the first refraction *produces* and **why it beats a static template**:

1. **It is content-addressed by construction.** `spec_of(s)` is a pure function
   of the seed; two `new`s with the same `(name, intent, store, template)`
   produce byte-identical `.spec`s with the same OID. The project's audit trail
   starts at its genesis crystal (`spectral-db-three-tier` §"Properties"). A
   `cargo new` produces a `Cargo.toml` whose content is *incidental*; a `mirror
   new` produces a `.spec` whose content *is its identity hash*.
2. **It runs the compiler's own settlement on the project before any code
   exists.** The settlement is the same `@kintsugi.settle` the project will use
   on its grammars. So the project's *first* operation and *every subsequent*
   operation are the same operation — the project is self-similar from t=0. The
   genesis crystal is a real `settle` verdict, not a file copy. If the seed is
   ill-formed (a name that isn't a valid namespace, a store that isn't a
   `store_kind`), the refraction returns `Imperfect` and the project is **not**
   created — the scene does not close (the `@mirror/init` scene's `invariants:
   [well_formed(spec), store_declared, consent_of_curator]` from
   `spectral-db-three-tier`). A static template would have written a broken file
   and let the user discover the breakage at first compile.
3. **It folds, rather than copies, mirror's own spec.** The template grammar
   `@new/template.for(kind, …)` *is* `mirror.spec` projected to the chosen kind
   (`focus`/`project` onto the relevant imports). A `pure` project gets `in
   @prism` + the namespace; a `numerical` project additionally gets `in
   @prism/rust` + `in @code/fortran`; a `cli` project gets `in @cli` + `out
   @cli/*`. The template is a *projection of the canonical spec*, so a user
   project can never drift structurally from mirror's own shape — they are the
   same object at different `project` filters. This is the deep reason the
   template is a grammar, not a string: a string template can rot independently
   of `mirror.spec`; a `project` of `mirror.spec` cannot.
4. **The hole is honest.** The below-`---` properties are a `\` at `new` time
   because *there is no topology to measure yet*. The static `@new_template`
   (`87c2686`) hard-coded `requires { types_lowercase … }` + `invariant {
   deterministic pure no_cycles }` — asserting properties the empty project
   hadn't earned. The settlement instead leaves below-`---` empty and lets the
   first `mirror settle .` (`spec-inference.md`) *measure* which properties the
   project's actual topology satisfies. Earned, not asserted.

The loop, stated once:

```
new  → seed → settle → .spec (below-`---` empty)
        ↓ user adds grammars
settle . → measures topology → fills below-`---`
        ↓ holes named
kintsugi  → resolves holes → updates .spec
        ↓
settle . → e^(n+1) < e^n
```

`new` is the n=0 of the loop the project lives in forever. That is the
justification for "the command that creates the project defines what a project
IS" — the genesis operation is the same operation the project runs for the rest
of its life.

---

## 5. Substrate vs floor — where `mirror new` lives

`mirror new` is a **substrate-declared command** (`@new` in `boot/std/`,
dispatched by `@cli`), whose body composes the five operations. Only two things
touch the floor, and both are already-blessed `@io` boundaries:

1. **The filesystem writes** (`io mkdir`, `io write`) — creating the directory,
   the `.spectral/` tree, the `.gitignore`, and writing the settled `.spec` to
   disk. Per AGENTS.md "The Glass Wall," these are legitimately `@io`: writing a
   byte to a path is an opaque effect on the world, not a capability a grammar
   can describe of itself.
2. **`git init` + store init** (`shift git_init`, `settle store_init` via
   `@mirror/store`) — initializing the VCS and the chosen `--store` backend.
   `@mirror/store` already owns this `io` boundary (`spectral-db-three-tier`:
   *"@fragmentation is pure. @mirror/store owns io"*).

Everything *between* those floor calls is grammar:

- choosing the template (`@new/template.for`) — pure `project` of `mirror.spec`;
- settling the seed into the `.spec` (`@kintsugi.settle`) — the five-op settle;
- computing the genesis crystal OID — `@fragmentation` content addressing, pure.

So the boundary sits exactly where AGENTS.md puts it: the scaffold *logic* is
capability (frozen to grammar — declare it in `@new`, never in the bootstrap);
the filesystem creation and VCS init are *floor* (`@io` / `@mirror/store`,
allowed, and if any thin Rust is needed it carries `🔧 [substrate-pull:realize]`,
**not** `🟢`).

### Does the bootstrap need to grow for `new`?

Assessment against the FROZEN policy: **no new capability Rust.** `new` needs
exactly two floor primitives, and the question is whether the bootstrap already
exposes them:

- **`io write` / `io mkdir`** — the bootstrap already shells filesystem effects
  (it does `git hash-object -w`, `fs::copy`, `fs::read_dir` in `craft`). A
  generic "write these bytes to this path" + "make this dir" is the same
  altitude of floor primitive. If it is not yet surfaced as an `@io` action the
  grammar can call, surfacing it is **boundary Rust** (`🔧 [substrate-pull:realize]`),
  not capability — a grammar genuinely cannot describe a `write(2)` of itself.
- **`git init`** — same: shelling `git init` is floor. The bootstrap already
  shells `git` and `clang`; `git init` joins that set as a marked boundary
  realize, if not already present.

The *dispatch* (`mirror new <name>` → `@new.new`) is pure `@cli` wiring — the
same five-verb collapse `surface-simplification.md` describes. No bootstrap
parser change: `new` is a `@cli` action, and `@cli` actions are substrate
declarations. The prior `be76713` wired `new` into the *old* Rust `src/cli.rs`;
that is exactly the capability-in-Rust the collapse removed. The correct shape
is `new` as a `@cli`/`@new` grammar action, with the floor calls marked.

The honest minimum: `@new` (grammar) + `@new/seed` (grammar) + `@new/template`
(grammar, a `project` of `mirror.spec`) + at most a marked `@io.write` /
`@io.mkdir` / `@io.git_init` floor surfacing if those three primitives are not
already callable from grammar. Nothing else.

---

## 6. `mirror new cosmos-mirror` — the concrete walkthrough

This is the migration's first step. `cosmos-mirror-scaffold.md` §4 specifies the
target `.spec` and `@cosmos` grammars; `new` produces the skeleton they drop
into. cosmos needs a numerical floor (`@code/fortran` / `@prism/rust` for the
eigendecomposition `D`), so it is the `--from numerical` / `--native fortran`
path.

```
mirror new cosmos-mirror --from numerical --native fortran --store git
```

Step by step, mapping to the §4 composition:

1. **`focus scaffold(seed, "cosmos-mirror")`** — seed is
   `{ name: cosmos-mirror, intent: "the world engine: one spectrum, many
   physics", store: git, template: numerical }`. The `numerical` template grammar
   is selected: `@new/template.for(numerical, …)` projects `mirror.spec` onto the
   numerical-floor imports.
2. **`split` (floor writes)** — creates:
   ```
   cosmos-mirror/
   ├── cosmos-mirror.spec      ← settled in step 4
   ├── cosmos-mirror.mirror    ← seed grammar: declares @cosmos-mirror namespace
   ├── .spectral/              ← gestalt/ sessions/ crystals/ HEAD log (empty-but-valid)
   ├── .gitignore              ← ignores the flang/LAPACK build floor artifacts
   └── flake.nix               ← because --native fortran: pins flang + LAPACK
   ```
   The `flake.nix` appears **only** because of the native floor (§2). A pure
   project would not get one.
3. **`zoom git_init`** — `git init cosmos-mirror`; `@mirror/store` initializes the
   git store backend (`--store git`).
4. **`settle spec_of(seed)`** — settles `cosmos-mirror.spec`. Because the
   template is `numerical`, the projection of `mirror.spec` yields exactly the
   import set `cosmos-mirror-scaffold.md` §4 calls for:
   ```mirror
   in @prism                  -- the five operations (always)
   in @prism/rust             -- the LAPACK floor: D (eigensystem, spectral_dimension)
   in @code/fortran           -- aligned target for the eigendecompose primitive
   in @cosmos-mirror          -- this project's namespace (the seed grammar)
   in @io                     -- io boundary: curve output, config read, CLI
   in @cli

   # cosmos-mirror: the world engine: one spectrum, many physics
   # settled at 2026-05-28T… — crystal <oid>

   type convention = l_sym | l_combinatorial   -- which Laplacian D diagonalises

   out @cli/*

   cosmos = @cosmos-mirror {
     default cosmos-mirror/*.mirror
   }
   ```
   Below `---` is **empty** — there is no `@cosmos` grammar yet to measure. The
   project is now the skeleton; the scaffold spec's `@cosmos/{rgg,observe,flow,
   partition,types}` grammars are what the *user* (the migration) adds next, at
   which point `mirror settle .` measures them and fills below-`---`.

What `new` produces is therefore *precisely* the empty frame
`cosmos-mirror-scaffold.md` §4 + §5-step-1 need: the `.spec` with the right
imports (incl. the `@prism/rust` / `@code/fortran` floor), the namespace seed
grammar, the `.spectral/` session dir, the `flake.nix` pinning flang/LAPACK, and
git initialized. The migration's "step 0" is `mirror new cosmos-mirror --from
numerical --native fortran`; its "step 1" (port the d_s `focus`) is dropping
`boot/std/cosmos/types.mirror` + `observe.mirror` into the skeleton and running
`mirror settle .`.

The genesis crystal of `cosmos-mirror.spec` is reproducible: anyone who runs the
same `new` invocation gets the same `.spec` OID. The project's content-addressed
history begins at a number — exactly as the scaffold spec's §5 closing line
wants ("The scaffold's first proof already exists as a number"), now extended to
the project's own genesis.

---

## 7. Findings, surprises, contradictions

- **Prior art exists and is richer than expected.** A `reed/mirror-new` branch
  and worktree (`/Users/alexwolf/dev/projects/mirror-new`) carry a working-ish
  `@new` grammar (`87c2686`), CLI wiring (`be76713`), and a `mirror-init-spec.md`
  (`d5d08ce`, 729 lines, "LAPACK-backed VCS store initialization"). This spec
  treats `87c2686`'s `@new` as the structural ancestor and updates it for three
  things that postdate it: the five-op surface collapse, the `@io`/`@mirror/store`
  placement correction, and the kintsugi-settle-into-`.spec` framing.
- **One deliberate supersession.** The old `@new_template` (`87c2686`) wrote a
  *static* `.spec` with hard-coded `properties { requires { … } invariant { … }
  }`. This spec changes that single choice: the `.spec` is *settled from a
  seed*, and below-`---` properties are left empty for `settle` to *measure*,
  not asserted at birth. The composition (`focus . split . shift . settle`) and
  the `--store` flag survive; only "static template" → "settle the seed" changes.
  This is surfaced, not silent, per the stop-and-report condition.
- **No contradiction between `init` and `new`.** The `spectral-db-three-tier`
  doc commits `init` to the kintsugi-settle mechanism; this spec adds `new` as
  the create-a-dir sibling sharing that exact mechanism (the cargo split). They
  reinforce rather than conflict — `new ⊂ init ⊂ settle` by seed richness (§3).
- **mirror's batteries are imports, not files.** The most surprising design
  consequence: the *right* default is closer to `go mod init` (one manifest +
  near-nothing) than to `mix new` (full tree), because in mirror you gain
  capability by adding `in @…` lines (MerkleTree traversal), not by editing
  pre-written source. The `mix --sup`/`--umbrella` instinct ("structure
  expresses the runtime") survives as `--from <template>` selecting *which
  imports* the seed carries — but the template is a `project` of `mirror.spec`,
  so user projects structurally cannot drift from mirror's own shape.
- **The template-as-grammar is dotnet's installable-pack idea, content-addressed.**
  `@new/template/<kind>` resolves from the garden by OID; publishing a new
  template kind is publishing a grammar, with no binary change — and because each
  template is a projection of `mirror.spec`, the canonical project shape is
  single-sourced.

---

## 8. Open

- **Intent capture format.** `--interactive` elicits the one-line `intent`
  string. Whether intent is purely cosmetic (a comment in the `.spec`) or
  load-bearing (fed to Fate to *suggest* a template kind) is a `@new/seed`
  concern; the deterministic-refraction property requires that whatever intent
  resolves to, the seed → `.spec` map stays pure.
- **`--from` template registry bootstrapping.** The first `@new/template/*`
  grammars (`pure`, `cli`, `numerical`, `actor`) must exist in the boot tree or
  garden before `--from` resolves them. Minimum viable: `pure` + `numerical`
  (cosmos-mirror needs `numerical`); `cli` + `actor` follow demand
  (defer-until-consumer, per AGENTS.md).
- **`init` over a polyglot existing tree.** `init .` over a directory with mixed
  source leans on `spec-inference.md`'s per-language detection; how the seed
  composes multiple `@code/*` floors into one `.spec` is the same composition
  `spec-inference.md` §"The Detection Grammar" parks. Tracks that spec's tick.
- **Whether `flake.nix` is the right native-floor manifest** vs a thinner
  `@mirror/store/nix` declaration. `flake.nix` is the household substrate (mirror
  itself has one); a project might prefer to declare its native floor purely in
  the `.spec`'s `in @code/fortran` and let `@mirror/store --store nix` derive the
  flake. Near-term: write the `flake.nix` (concrete, copyable); aligned target:
  derive it. Tracks `numerical-substrate-via-fortran`.

---

## References

[1] deno init — Deno docs. https://docs.deno.com/runtime/reference/cli/init/
[2] bun init — Bun docs. https://bun.com/docs/runtime/templating/init
[3] mix new / umbrella — Elixir School. https://elixirschool.com/en/lessons/advanced/umbrella_projects
[4] deno init --npm deno.json — denoland/deno#29507. https://github.com/denoland/deno/issues/29507
[5] Mix cheatsheet (project scaffolding, mix.exs). https://1337skills.com/cheatsheets/mix/
[6] cargo init vs cargo new — Stack Overflow. https://stackoverflow.com/questions/65631196/cargo-init-creates-new-dir-as-like-cargo-new
[7] Custom templates for dotnet new — Microsoft Learn. https://learn.microsoft.com/en-us/dotnet/core/tools/custom-templates
[9] cargo init .gitignore / no forced git — Rust for C-Programmers. https://rust-for-c-programmers.com/ch23/23_2_the_cargo_command_line_interface_cli.html
[11] Go Modules Reference (go mod init). https://go.dev/ref/mod
[13] Default/implicit build.zig — Ziggit. https://ziggit.dev/t/default-or-implicit-build-zig/10300
[14] go mod init explained — Stack Overflow. https://stackoverflow.com/questions/67606062
[15] swift package init --type — Swift PM docs. https://docs.swift.org/swiftpm/documentation/packagemanagerdocs/packageinit/
[16] zig init templates — ziglang/zig#20363. https://github.com/ziglang/zig/issues/20363

---

*The seed is the name. The refraction is the spec. The project IS its genesis crystal.*
*Other tools write boilerplate. mirror settles a measurement.*
*`new` is the n=0 of the loop the project lives in forever.*
