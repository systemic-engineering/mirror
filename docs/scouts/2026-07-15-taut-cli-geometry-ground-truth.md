# Taut Scout — CLI Geometry Ground Truth

*2026-07-15. Taut. Read-only grep-first reconnaissance under Alex Wolf's
2026-07-15 in-transcript directive. Feeds Mara's CLI-spec condensation.*
*Reed commits as Taut after review; scout does not commit.*

---

## §0. Scope + method

### 0.1 Alex directive (verbatim)

> "We want to not add randomly commands. We want to find the CLI surface
> that respects and represents the GEOMETRY of the compiler. And the
> compiler is a bunch of recursively composed prisms where a beam of
> light passes to (that's what @../prism/ does)."

This is a *condensation* directive, not an *addition* one. The CLI
surface should MAKE VISIBLE the prism-composition geometry, not
accumulate verbs. The grep produces the ground truth Mara condenses
from.

### 0.2 Scout method

Grep-first empirical enumeration across three axes:

1. **Axis 1 — Current CLI verb surface.** Every landed verb across
   `shards/mirror/lens/cli.mirror`, `shards/mirror/lens/cli/**/*.mirror`,
   `mirror.spec` `cli { command ... }` blocks, `bootstrap/src/lib.rs`
   dispatch, `bootstrap/src/mcp.rs` MCP tool surface, `bin/mirror-mcp`.
2. **Axis 2 — Prism composition graph.** Every `prism @X { ... }` +
   `in @X` import; five-op template usage; `stage @X { default op ... }`
   sub-prism specialisations.
3. **Axis 3 — @../prism/ sibling crate.** `terni` / `prismqueer` /
   `prismqueer-projections` public shapes; `beam/` Gleam types; the
   physical `Prism` + `Beam` optics substrate mirror's substrate-decl
   reflects.

### 0.3 Discipline

- Grep-first (empirical, not preferences).
- Read-only (no shard authorship; no Rust).
- Substrate-already-had-the-word discipline: name what's landed; don't
  invent patterns.
- Cite files by path + line number.
- Scout does not decide (Mara condenses; Taut does not propose).

---

## §1. Current CLI verb surface — verb-by-verb enumeration

### 1.1 Depth-0 top-level verbs (empirically dispatching in Rust today)

Ground truth from `bootstrap/src/lib.rs::dispatch()` match arms (lib.rs
~3149–3609) and `usage()` (~630–645):

| Verb | Dispatcher fn | Positional | Flags | Owning `.mirror` shard | Owning `mirror.spec` `command` block |
|------|--------------|-----------|-------|------------------------|--------------------------------------|
| `compile` | `cmd_compile` (lib.rs 645) | `path: ~d` | `--strict` | `shards/mirror/lens/cli/compile.mirror` (default `settle`) | `mirror.spec:82-86` |
| `craft` | `cmd_craft_with` (lib.rs 740) | `target: ~d` | `--target-kind {crystal\|binary}`, `--reflect` | — (no `shards/mirror/lens/cli/craft.mirror`) | `mirror.spec:123-127` |
| `kintsugi` | `cmd_kintsugi` (lib.rs 1106) + `_ci_single` / `_ci_corpus` / `_spec` / `_rust` / `_migrate` / `_single` variants | `file: ~f\|dir: ~d` | `--ci`, `--out @data/json\|@data/mirror\|@io/dir(...)`, `--shatter N`, `--liquid`, `--target list(str)`, `--emit-shatter`, `--migrate <src> <out>`, `--transform` | `shards/mirror/lens/cli/kintsugi.mirror` (default `focus`) | `mirror.spec:88-93` |
| `shatter` | `cmd_shatter` (lib.rs 3614) | `oid: content_address`, `out: ~f` | `--target str="auto"` | `shards/mirror/lens/cli/shatter.mirror` (default `settle`) | `mirror.spec:95-113` |
| `init` | `cmd_init` (lib.rs 3722) | `path: ~d` | `--install-hooks` | — (no `shards/mirror/lens/cli/init.mirror`) | `mirror.spec:135-140` |
| `recall` | `cmd_recall` (lib.rs 3924) | `spec_dir: ~d` | — | — (no `shards/mirror/lens/cli/recall.mirror`) | `mirror.spec:142-152` |
| `spawn` | `cmd_peer_beam` alias (lib.rs 3195) | `peer_home: ~d` | `--hello-world`, `--mission <f>` (deprecated per `b012d3f`; stderr notice) | — | — (removed; alias-only) |
| `beam` | `cmd_peer_beam` anonymous variant (lib.rs 3475) | `mission: ~f` | `--hello-world` | — (no `shards/mirror/lens/cli/beam.mirror`) | `mirror.spec:165-172` |
| `index` | inline (lib.rs 3256) | `path: ~d` | `--fiedler`, `--full-profile` | — (owning shard cited as `shards/mirror/index.mirror` per mirror.spec:194 docblock — Rung 8 substrate-decl) | `mirror.spec:174-198` |
| `peer` | recursive-command dispatcher (lib.rs 3325) | — (dispatches to depth-2 subcommands) | — (per-subcommand) | — (no `shards/mirror/lens/cli/peer.mirror`) | `mirror.spec:236-330` |

### 1.2 Depth-1 subcommands (empirically dispatching under `peer`)

| Path | Dispatcher fn | Positional | Flags |
|------|--------------|-----------|-------|
| `mirror peer beam` | `cmd_peer_beam` (lib.rs 5159) | `peer_home: ~d` | `--hello-world`, `--mission <f>`, `--song <f>`, `--dance-with <f>`, `--deploy-to <f>`, `--emit-crystal`, `--fate-select`, `--from-psychohistory`, `--with-shadow`, `--emit-diff`, `--integrate-diff` |
| `mirror peer contribute` | (per `mirror.spec:333-336`; not enumerated in the lib.rs slice loaded — but declared) | `peer_home: ~d` | `--target <f>` |

Depth-2 dispatch is the FIRST consumer of `shards/mirror/lens/cli.mirror`
Tick 1 recursive-command grammar (`command X { command Y { ... } }`;
landed at commit `fe82500`).

### 1.3 CLI verb surface declared in `.mirror` shards BUT not in `mirror.spec` cli-block

Per `shards/mirror/lens/cli/*.mirror`, eight sub-stages exist. Cross-
referencing which are wired into `mirror.spec cli { ... }` gives the
"declared-but-not-wired" gap:

| Sub-stage shard | Namespace | `default` op | Wired in mirror.spec? |
|-----------------|-----------|--------------|------------------------|
| `compile.mirror` | `@mirror/lens/cli/compile` | `settle` | YES (mirror.spec:82) |
| `kintsugi.mirror` | `@mirror/lens/cli/kintsugi` | `focus` | YES (mirror.spec:88) |
| `shatter.mirror` | `@mirror/lens/cli/shatter` | `settle` | YES (mirror.spec:95) |
| `bootstrap.mirror` | `@mirror/lens/cli/bootstrap` | `focus` | NO (shard docblock §"no mirror.spec command yet") |
| `sh.mirror` | `@mirror/lens/cli/sh` | `settle` | NO (shard docblock §"no mirror.spec command yet") |
| `reflect.mirror` | `@mirror/lens/cli/reflect` | `focus` | NO (shard docblock §"no mirror.spec command yet") |
| `time.mirror` | `@mirror/lens/cli/time` | `focus` | NO (shard docblock §"no mirror.spec command yet") |
| `crack.mirror` | `@mirror/lens/cli/crack` | `focus` | NO (shard docblock §"no mirror.spec command yet") |

Beyond these eight canonical sub-stages, `mirror.spec` also wires SIX
non-sub-stage commands NOT declared as `shards/mirror/lens/cli/*.mirror`
stages: `craft`, `init`, `recall`, `beam`, `index`, and the `peer` +
`peer beam` + `peer contribute` chain.

### 1.4 The eight top-level verbs the cli-as-prism spec forward-promised

Per `docs/specs/cli-as-prism.md` §2.2 and §5.6, the CONDENSATION target
was the **top-level 5-op surface + 7-to-8 sub-stages with the same
five ops**:

```
mirror focus    [target...]
mirror project  [predicate]
mirror split    [edge]
mirror shift    [functor]
mirror settle   [store...]
```

Plus the sub-stages: `compile`, `kintsugi`, `shatter`, `bootstrap`,
`sh`, `reflect`, `time`, `crack`. **Empirical drift**: none of the
five top-level ops are dispatched today; the CLI is 100% sub-stage
verbs (dispatch match arms at lib.rs 3149-3609 have NO `"focus"`,
`"project"`, `"split"`, `"shift"`, `"settle"` arms).

### 1.5 MCP tool surface — the mirror at the LLM boundary

Per `bootstrap/src/mcp.rs` docblock (mcp.rs:1-65) and
`dispatch_tool_call()`:

| MCP tool | CLI translation | Owning `mirror.spec` block |
|----------|-----------------|----------------------------|
| `mirror_compile` | `mirror compile <file>` | mirror.spec:82 |
| `mirror_craft` | `mirror craft <target> [--target-kind K] [--reflect]` | mirror.spec:123 |
| `mirror_kintsugi` | `mirror kintsugi --ci --out @data/json <file> [--liquid] [--shatter N]` (Tick 7 shatter-fold `ffba2a7` — always `--ci`) | mirror.spec:88 |
| `mirror_init` | `mirror init <path> [--install-hooks]` | mirror.spec:135 |
| `mirror_recall` | `mirror recall <spec_dir>` | mirror.spec:142 |
| `mirror_peer_beam` | `mirror peer beam <peer_home> [--hello-world\|--mission\|--fate-select\|--from-psychohistory\|--with-shadow\|--emit-diff\|--integrate-diff]` | mirror.spec:242 |
| `mirror_beam` | `mirror beam --mission <mission>` | mirror.spec:165 |
| `mirror_spawn` | `mirror spawn <peer_home> [--hello-world] [--mission <f>]` (DEPRECATED alias per two-tick discipline) | — |
| `mirror_index` | `mirror index <path> [--fiedler] [--full-profile]` | mirror.spec:194 |

Nine MCP tools total (eight-tool schema per `bin/mirror-mcp` header +
`mirror_index` Rung 8 Landing 5 lift). MCP surface has GONE THROUGH THE
SAME sprawl as the CLI — the substrate-honest read is that MCP tools
mirror CLI verbs 1:1, and any drift in CLI surface geometry propagates
to MCP surface geometry.

`bin/mirror-mcp` is now an 18-line shim (`bin/mirror-mcp:16-17`):
`exec "${MIRROR_BIN}" /dev/stdin "@mcp.serve"`. The bash wrapper's
145-line hand-rolled tools schema collapsed into
`bootstrap/src/mcp.rs::tools_list_result`; the shim IS the
substrate-pull realise of the family-header-only
`shards/mirror/lens/mcp.mirror` (mcp.mirror:1-66).

### 1.6 Empirical top-level verb count

**11 top-level verbs dispatch today** (from lib.rs match arms):
`compile`, `craft`, `kintsugi`, `shatter`, `init`, `recall`, `spawn`,
`beam`, `index`, `peer` (+ its depth-2 `beam` / `contribute`).

Plus one usage-mode: `mirror '<mq-query>' < input` (the mq pipeline over
stdin per usage() 630) and `mirror <input-file> '<mq-query>'` (input +
mq pipeline).

The cli-as-prism spec proposed **5 top-level ops + 8 sub-stages = 13
names**. Today's ACTUAL surface: **11 depth-0 verbs + at least 2
depth-1 verbs + 0 five-op verbs = 13+ names**, with the 5 top-level
ops entirely missing.

---

## §2. Prism composition graph

### 2.1 Family-root prism declarations enumerated

Every landed lens family/species prism block, five-op template
identifiable by `focus X\n  project X\n  split X\n  shift X\n  settle X`
pattern where X is the species name:

| Namespace | File | Location | 5-op operand |
|-----------|------|----------|--------------|
| `@mirror/lens` (family root) | `shards/mirror/lens.mirror` | 117-123 | `lens` |
| `@mirror/lens/cli` (bench) | `shards/mirror/lens/cli.mirror` | 108-114 | `cli` |
| `@mirror/lens/mcp` (transport) | `shards/mirror/lens/mcp.mirror` | 44-50 | `mcp` |
| `@mirror/lens/shell` (transport) | `shards/mirror/lens/shell.mirror` | 63-69 | `shell` |
| `@mirror/lens/lsp` (transport) | `shards/mirror/lens/lsp.mirror` | 44-50 | `lsp` |
| `@mirror/lens/unix` (transport) | `shards/mirror/lens/unix.mirror` | 143-149 | `unix` |
| `@mirror/lens/refract` (measurement) | `shards/mirror/lens/refract.mirror` | 84-90 | `refract` |
| `@mirror/lens/transit` (measurement) | `shards/mirror/lens/transit.mirror` | 97-103 | `transit` |
| `@mirror/lens/knife` (COORD lens) | `shards/mirror/lens/knife.mirror` | 141-147 | `knife` |

**Eight species under `@mirror/lens`** (per lens.mirror docblock §Species
under this family): four transports (cli / shell / mcp / lsp), two
measurements (transit / refract), one filesystem impedance (unix), one
COORD-jump lens (knife). Plus one platform sub-species landed
(`shards/mirror/lens/unix/fuse.mirror`).

### 2.2 Sub-stage prism declarations under @mirror/lens/cli

Every `stage @mirror/lens/cli/<x> { default X; focus X; project X;
split X; shift X; settle X }` block (the sub-prism specialisation
form per `docs/specs/optical-keywords.md` §1.2):

| Sub-stage | File | Location | `default` op | 5-op operand |
|-----------|------|----------|--------------|--------------|
| `@mirror/lens/cli/compile` | `shards/mirror/lens/cli/compile.mirror` | 73-101 | `settle` | `target`, `predicate`, `candidate`, `altitude`, `target` |
| `@mirror/lens/cli/kintsugi` | `shards/mirror/lens/cli/kintsugi.mirror` | 246-273 | `focus` | `target`, `predicate`, `candidate`, `basis`, `iteration` |
| `@mirror/lens/cli/shatter` | `shards/mirror/lens/cli/shatter.mirror` | 84-117 | `settle` | `target`, `predicate`, `variant`, `altitude`, `target` |
| `@mirror/lens/cli/bootstrap` | `shards/mirror/lens/cli/bootstrap.mirror` | 88-128 | `focus` | `phase`, `predicate`, `candidate`, `basis`, `phase` |
| `@mirror/lens/cli/sh` | `shards/mirror/lens/cli/sh.mirror` | 114-151 | `settle` | `peer`, `predicate`, `peer`, `peer`, `peer` |
| `@mirror/lens/cli/reflect` | `shards/mirror/lens/cli/reflect.mirror` | 90-127 | `focus` | `position`, `predicate`, `candidate`, `altitude`, `position` |
| `@mirror/lens/cli/time` | `shards/mirror/lens/cli/time.mirror` | 100-140 | `focus` | `tick`, `predicate`, `tick`, `tick`, `tick` |
| `@mirror/lens/cli/crack` | `shards/mirror/lens/cli/crack.mirror` | 163-203 | `focus` | `target`, `predicate`, `candidate`, `basis`, `target` |

**Eight sub-stages, all landed as substrate declarations, all sitting
on top of the same five-op template.** Three are wired into `mirror.spec`
cli-block dispatch (compile / kintsugi / shatter — the artifact-produce
verbs). Five are NOT wired (bootstrap / sh / reflect / time / crack —
each shard's docblock says "no mirror.spec command yet").

### 2.3 Composition altitude ladder (empirical)

```
                       @prism   (five-op algebra source)
                          ↑
                          │  in @prism
                          │
      @glass ← @nl → @mirror/lens   (family root; observation surface)
                          │
       ┌──────────────────┼──────────────────┬───────────┐
       │                  │                  │           │
  TRANSPORTS         MEASUREMENTS        FILESYSTEM   COORD-JUMP
  (audience)         (property)          (OS)         (level-shift)
       │                  │                  │           │
       ├── /cli           ├── /transit       └── /unix   └── /knife
       ├── /shell         └── /refract         └── /fuse
       ├── /mcp
       └── /lsp
       │
       │  (depth-2 sub-stages of /cli only)
       │
       ├── /cli/compile   (settle default)
       ├── /cli/kintsugi  (focus default)
       ├── /cli/shatter   (settle default)
       ├── /cli/bootstrap (focus default)
       ├── /cli/sh        (settle default)
       ├── /cli/reflect   (focus default)
       ├── /cli/time      (focus default)
       └── /cli/crack     (focus default)
```

**Every node in this graph is a prism.** Every non-leaf node's five ops
map to the same five ops at every child. Per `docs/specs/cli-as-prism.md`
§1.3: "The recursion is as deep as the substrate demands — most verbs
land at depth 1, a few at depth 2." Depth-2 is now MINTED via Tick 1's
recursive-command grammar (`shards/mirror/lens/cli.mirror:108-160`).

### 2.4 Same-algebra-at-every-altitude property

Load-bearing per `docs/specs/cli-as-prism.md` §5.6: "A user who learns
`focus / project / split / shift / settle` once knows every command at
every depth." Each prism at each altitude declares:

```mirror
prism @X {
  focus <operand>
  project <operand>
  split <operand>
  shift <operand>
  settle <operand>
}
```

The operand differs per altitude (per shard-native manifold: `target`
for compile, `iteration` for kintsugi, `peer` for sh, `tick` for time,
`phase` for bootstrap, `position` for reflect, etc.), but the FIVE OP
NAMES are identical, and the composition rule is unchanged: `stage`
substitutes as a facet at the next altitude up.

### 2.5 Import edges — the composition graph

`in @X` edges the substrate carries (grep result across cli.mirror +
lens.mirror + all cli/*.mirror):

- Every `@mirror/lens/*` species imports `@prism` (the algebra source),
  `@glass` (verdict / imperfect / transparency), `@nl` (the `#` help-
  text lift), and `@mirror/lens` (the family root).
- Every `@mirror/lens/cli/*` sub-stage additionally imports `@optics`
  and `@mirror/lens/cli` (the parent bench).
- `shards/mirror/lens/cli/time.mirror` additionally imports
  `@epistemologic/reality/time` (typed `tick = monotonic` carrier).
- `shards/mirror/lens/knife.mirror` additionally imports `@meta`,
  `@torus`, `@cyberpunk`, `@cyberpunk/algedonic`, `@cyberpunk/reframe`,
  `@magic/onto`, `@kintsugi/consent` (per Foerster-COORD identification;
  9-import composition depth).
- `shards/mirror/lens/unix.mirror` additionally imports `@mirror/store`
  (the oid-graph the impedance surface materialises).

The IMPORT graph is not the composition graph; the composition graph is
the family/species inheritance. Import is how species pull in the
carriers they compose over (verdict, tick, oid, etc.).

---

## §3. @../prism/ sibling crate — the physical reference

Located at `/Users/alexwolf/dev/projects/prism/` (NOT the mirror project
— the sibling crate). Workspace of three published crates + one
Gleam/BEAM sub-project.

### 3.1 Crate structure (`Cargo.toml`, README.md)

```
prism/                          (workspace root)
├── imperfect/          →       crates.io: `terni`  (standalone; zero deps)
├── prismqueer/         →       crates.io: `prismqueer`
├── projections/        →       crates.io: `prismqueer-projections`
├── beam/                       Gleam/BEAM crate `prism_beam` (Erlang target)
└── docs/                       Architecture + specs
```

### 3.2 `terni` (published as `terni`) — the loss-carrying ternary type

Source: `/Users/alexwolf/dev/projects/prism/imperfect/src/lib.rs`
(100.5KB) + `imperfect/src/transparency.rs` (13.1KB).

Key public types (per `imperfect/README.md` + `imperfect/src/lib.rs`
re-exports at `prismqueer/src/lib.rs:98`):

- `Imperfect<T, E, L: Loss>` — the ternary carrier:
  `Success(T) | Partial(T, L) | Failure(E, L)`
- `Loss` — monoid trait (`zero`, `combine`, `total`, `is_zero`)
- `Diagnostic`, `Metric`, `PropertyVerdict`, `Transparency` — the
  loss-shape substrate mirror's `@glass.imperfect` +
  `@glass.transparency` mirror at spec altitude.
- `ConvergenceLoss` (distance to crystal; combine: max)
- `ApertureLoss` (dark dimensions; combine: union)
- `RoutingLoss` (decision entropy; combine: max entropy, min gap)

### 3.3 `prismqueer` (published as `prismqueer`) — the Prism trait + Beam carrier

Source: `/Users/alexwolf/dev/projects/prism/prismqueer/src/*.rs` (43
source files, ~800KB total). Public modules per `prismqueer/src/lib.rs`:

**Base substrate** (`prismqueer/src/lib.rs:32-52`):

- `mod beam` — the semifunctor over `Imperfect`; `Beam` trait + `Optic`
  concrete beam.
- `mod coincidence` — `canonical_hash`, `coincidence_hash`, `Detector`,
  `HashPrism`.
- `mod crystal` — `Crystal` type.
- `mod luminosity` — `Luminosity` type.
- `mod scalar_loss` — `ScalarLoss` type (single-f64 loss).
- `mod substrate_ref` — `Ref` type.
- `mod trace` — `Op`, `Step`, `StepOutput`, `Trace`, `Traced`.
- `mod connection` — `Carrier`, `ScalarConnection`.
- `mod content` — `ContentAddressed` trait.
- `mod kernel` — `Decomposition`, `KernelSpec`.
- `mod merkle` — `diff`, `Delta`, `MerkleTree`.
- `mod metal` — Metal (GPU) bindings.
- `mod named` — `Named`.
- `mod oid` — `Addressable`, `Oid`.
- `mod optic_kind` — `FieldOptic`, `OpticKind`.
- `mod precision` — `Precision`, `Pressure`.
- `mod spectral_oid` — `SpectralOid`.
- `mod spectral_uuid` — `SpectralUuid`.
- `mod store` — `Store`.

**Feature-gated modules**:

- `mod optics` (feature `optics`) — `Lens`, `Iso`, `Traversal`, `Fold`,
  `Setter`, `OpticPrism`.
- `mod pq` (feature `pq`) — typed pq wire DSL (`Target` / `Filter` /
  `Output`) with serde + JSON Schema.
- `mod bundle` (feature `bundle`) — principal-bundle tower: `Fiber`,
  `Connection`, `Gauge`, `Transport`, `Closure`, `IdentityPrism`,
  `LawvereFixedPoint`, `StableFiber`.
- `mod lambda` (feature `lambda`) — content-addressed lambda calculus.
- `mod ffi` (feature `lapack`) — Fortran dispatch for `KernelSpec` /
  `SpectralDimension`.
- `mod spectral_dimension` (feature `lapack`) — eigenvalue machinery.

**Load-bearing trait** (`prismqueer/src/lib.rs:126-142`):

```rust
pub trait Prism {
    type Input: Beam;
    type Focused: Beam<In = <Self::Input as Beam>::Out>;
    type Projected: Beam<In = <Self::Focused as Beam>::Out>;
    type Refracted: Beam<In = <Self::Projected as Beam>::Out>;

    fn focus(&self, beam: Self::Input) -> Self::Focused;
    fn project(&self, beam: Self::Focused) -> Self::Projected;
    fn settle(&self, beam: Self::Projected) -> Self::Refracted;
}
```

**Ground-truth divergence from mirror's substrate**: the physical `Prism`
trait has **THREE ops** (focus / project / settle), not five. Split and
shift are NOT part of the crate's `Prism` trait; they live at
`prismqueer/src/optics/*` as separate optic kinds (`Lens`, `Iso`,
`Traversal`, `Fold`, `Setter`). Mirror's substrate lifts to
FIVE ops per shard-decl (cli.mirror:108-114 and per every species).

The `beam.rs` module (`prismqueer/src/beam.rs`, 23.8KB) carries `Optic`
+ `Beam` + `Operation` — the beam-through-optics primitive. `apply`
(lib.rs:166) runs the three ops end-to-end.

### 3.4 `prismqueer-projections` (proc-macros)

Source: `/Users/alexwolf/dev/projects/prism/projections/src/{lib.rs,declaration.rs}`
(58.9KB total). Public exports:

- `#[derive(Prism)]` — proc macro that generates the `impl Prism for X`
  from struct fields.
- `#[derive(Lambda)]` (feature `lambda`) — named lambda phase derive.
- `declaration!{}` — function-like proc macro reading substrate `type`
  declarations as input tokens, emitting Rust struct/enum realisations.
  Documented as the `@code/rust/macro.shim_type` reception entry point
  per `mirror/shards/code/rust/macro.mirror`.

### 3.5 `beam/` — the BEAM/Gleam mirror

Source: `/Users/alexwolf/dev/projects/prism/beam/src/prism_beam.gleam`
(2.3KB). Not published; local Gleam project targeting Erlang.

Types (verbatim from `prism_beam.gleam`):

```gleam
pub type Oid            (value: String)
pub type ShannonLoss    (bits: Float)
pub type Precision      (value: Float)
pub type Pressure       (ratio: Float)
pub type Recovery       Coarsened | Replayed | Failed
pub type Beam(t)        (result, path, loss, precision, recovered)

pub fn new(result: t) -> Beam(t)
pub fn is_lossless(beam: Beam(t)) -> Bool
pub fn has_loss(beam: Beam(t)) -> Bool
pub fn was_recovered(beam: Beam(t)) -> Bool
pub fn map(beam: Beam(a), f: fn(a) -> b) -> Beam(b)
pub fn with_step / with_loss / with_precision / with_recovery
```

Header comment: "Mirrors the Rust prism crate. Type surface equivalence."
Same shape at BEAM altitude. Same beam-through-optics substrate; same
loss carrier; same content-address identity.

### 3.6 Architecture doc — the stack framing

Per `/Users/alexwolf/dev/projects/prism/docs/architecture.md`:

```
terni              Imperfect<T, E, L> — the ternary type
prism-core         Optic, Beam, Bundle tower — the optics
mirror             .mirror compiler — the compiler IS the LSP
spectral-db        tick/tock graph — the graph IS the memory
coincidence        eigenvalues — the eigenvalues ARE the observation
fate               model selection — the decision IS the loop closing
```

**Load-bearing quote from architecture.md:1-6:**

> "The compiler produces loss. The loss IS the holonomy. The spectral
> runtime analyzes the holonomy as a graph. The graph tells Fate what
> to do next."

**Load-bearing quote from architecture.md:74-83** (the CLI-is-the-LSP
framing that intersects Axis 1):

> "The Mirror compiler is an incremental compiler by architecture. It
> does not have an LSP. It IS the LSP.
>
>     mirror (the binary)
>       ├── compile file.mirror     one tick, wait for crystal, print result
>       ├── repl                    interactive, autocomplete, inline diagnostics
>       └── lsp                     stdio LSP for external editors
>
> All three are the same compiler. The same function. The same return
> type: `Imperfect<CompiledArtifact, CompilationError, MirrorLoss>`."

The physical substrate ALREADY calls out THREE surfaces on the same
compiler function: `compile file.mirror` (CLI one-shot),
`repl` (interactive), `lsp` (editor). Mirror's substrate-decl side
lifts this to FOUR (cli / shell / mcp / lsp) per `@mirror/lens`
family root's transport enumeration (lens.mirror:38-51) + adds three
more species families (transit / refract measurements + unix
impedance + knife COORD-jump).

---

## §4. Geometry ↔ CLI surface mapping — where today's CLI fits vs fights

This section names DRIFT observations. Taut does not decide; Mara
condenses.

### 4.1 The geometry claim (per substrate declarations)

The compiler is a bundle-tower of prisms. Each prism has FIVE OPS
(`focus`, `project`, `split`, `shift`, `settle`) on its manifold.
Composition of prisms IS the substrate: `stage` substitutes as a facet
at the next altitude up per `docs/specs/optical-keywords.md` §1.2.

A beam of light passes through the composition. The beam carries
(value, input, accumulated loss). The physical substrate at
`/Users/alexwolf/dev/projects/prism/` names this: `Beam` (with
`Imperfect` as carrier), passing through `Prism` (three ops in the
physical crate; five in the substrate-decl mirror lifts it to).

### 4.2 Where today's CLI FITS the geometry

- **`shards/mirror/lens/cli.mirror`** declares
  `prism @mirror/lens/cli { focus cli; project cli; ... }` — the CLI IS
  a prism at family altitude. Geometry: correct.
- **Eight sub-stage shards** each declare `stage @mirror/lens/cli/<x>`
  with a `default` op + five-op operand list. Geometry: each named verb
  IS a prism-in-prism. The recursion is present at substrate altitude.
- **`shards/mirror/lens/{cli,shell,mcp,lsp,transit,refract,unix,knife}.mirror`**
  — the family root has eight species, all sharing the `focus/project/
  split/shift/settle` five-op template. Geometry: parallel; each lens is
  a sibling prism observing the same substrate through a different
  surface. Correct.
- **Depth-2 recursive-command grammar** (cli.mirror:108-160) admits
  `command X { command Y { ... } }`. `mirror.spec:236-330` consumes it
  with `command peer { command beam { ... } command contribute { ... } }`.
  Geometry: recursion is landed at grammar altitude AND at first-consumer
  altitude. Correct.
- **MCP tools** (`bootstrap/src/mcp.rs`) mirror CLI verbs 1:1 via the
  `run_mirror(...)` dispatch. Same algebra, different notation
  (JSON-RPC vs argv). Per `@mirror/lens/mcp.mirror:14-18`:
  "the mcp lens IS the JSON notation of the same algebra the cli
  renders as argv." Geometry: correct.
- **`bin/mirror-mcp` shim** (18 lines) demonstrates substrate-pull
  collapse: 145-line bash → Rust `cmd_mcp_serve` → 18-line shim. The
  wrapper IS following the substrate. Geometry: correct.

### 4.3 Where today's CLI FIGHTS the geometry (drift)

1. **The five top-level ops don't dispatch.** Per `docs/specs/cli-as-prism.md`
   §2.2, the condensation target is `mirror focus`, `mirror project`,
   `mirror split`, `mirror shift`, `mirror settle` at depth-0. Empirical:
   `bootstrap/src/lib.rs::dispatch()` (~3149-3609) has NO match arms for
   these. The top-level five-op surface — the ONE thing cli-as-prism was
   most confident about — is not landed.

2. **Sub-stages don't dispatch the five ops.** Per every sub-stage
   shard (`compile.mirror`, `kintsugi.mirror`, ..., `crack.mirror`):
   each stage's `default` op is declared, five ops are enumerated. But
   `mirror compile focus <path>`, `mirror kintsugi split <spec>` are
   NOT dispatched. The dispatcher hits the sub-stage name and runs a
   single `cmd_X` function; the five-op verb slot inside is ignored.
   Every sub-stage today is a single-shot verb, not a five-op sub-prism.

3. **Three verbs are `default settle` (compile / shatter / sh); five
   are `default focus` (kintsugi / bootstrap / reflect / time / crack).**
   Per `docs/specs/cli-as-prism.md` §7. If sub-stage five-op dispatch
   landed, `mirror kintsugi` would peek the next tournament move (focus),
   `mirror compile` would build (settle), `mirror crack` would list
   fractures (focus). Today it doesn't matter because sub-stages are
   single-shot verbs.

4. **Five sub-stages are declared but not wired into `mirror.spec`.**
   Per §1.3 above: `bootstrap`, `sh`, `reflect`, `time`, `crack` each
   have `shards/mirror/lens/cli/*.mirror` declarations with docblocks
   naming "no mirror.spec command yet". The substrate-decl side has
   done its work; the `mirror.spec cli { command ... }` block is behind.

5. **Six verbs are wired without a `shards/mirror/lens/cli/*.mirror`
   sub-stage shard.** Per §1.3: `craft`, `init`, `recall`, `beam`,
   `index`, `peer`. These have `mirror.spec` cli-block declarations and
   Rust dispatch, but NO sub-stage substrate declaration at
   `@mirror/lens/cli/<x>`. Two directions of drift:
   - `craft` is a build-adjacent verb sitting parallel to `compile` —
     unclear whether it should be a sub-stage or lift into `compile`'s
     operand vocabulary.
   - `init`, `recall`, `beam`, `index` are subject-adjacent verbs
     (init = repo bootstrap; recall = inbound-trajectory dual of spawn;
     beam = anonymous inference; index = coherence measurement) whose
     family placement isn't obvious.
   - `peer` is a recursive-command wrapper with subcommands `beam` +
     `contribute`. Depth-2 sub-stage sub-species could be
     `shards/mirror/lens/cli/peer.mirror` + `shards/mirror/lens/cli/peer/beam.mirror`
     but neither is landed.

6. **Verb sprawl inside `peer beam`.** Per `mirror.spec:242-306`, `peer
   beam` carries ELEVEN flags: `--hello-world`, `--mission`, `--song`,
   `--dance-with`, `--deploy-to`, `--emit-crystal`, `--fate-select`,
   `--from-psychohistory`, `--with-shadow`, `--emit-diff`,
   `--integrate-diff`. Each was added as a new mode (Rung 1 / Rung 4 /
   Rung 5 / Rung 6' per docblocks) via `if flag_present` guards in
   `cmd_peer_beam` (lib.rs 5159-5340+). This IS the sprawl pattern Alex's
   directive names: verbs accumulate without geometry-shaped
   consolidation.

7. **The `spawn` → `peer beam` rename is on two-tick discipline** —
   `spawn` is preserved as a backward-compat alias with a deprecation
   stderr notice per `b012d3f` Landing 2. This is correct substrate
   discipline (readable name over foundational per two-tick rule), and
   it exemplifies the CONDENSATION direction Alex names: `spawn` was a
   top-level verb; the geometric-honest form is `peer beam` (depth-2).

8. **The physical `Prism` trait has THREE ops; substrate lifts to FIVE.**
   Per `prismqueer/src/lib.rs:126-142`, the crate's `Prism` trait is
   focus/project/settle. Split, shift live at `prismqueer/src/optics/*`
   as separate optic kinds. Mirror's shard-decl lifts to five per
   `docs/specs/cli-as-prism.md` §1.1 (the substrate floor). If Mara
   condenses toward five-op-at-every-altitude, the physical crate
   diverges from the substrate lift — either the crate lifts to five,
   or the substrate collapses to three-plus-two-modes. Adjudication
   residue.

9. **MCP tool surface will drift the same way if the CLI does.** MCP is
   downstream of CLI (per `bootstrap/src/mcp.rs::dispatch_tool_call`
   which shells out to `run_mirror(...)`). Any CLI sprawl propagates.
   The 9-tool MCP surface today already reflects the CLI 11-verb sprawl.

### 4.4 Substrate declarations that are AHEAD of dispatch

The substrate has done geometric-honest work not yet realised in Rust
dispatch:

- Sub-stage shards for `bootstrap`, `sh`, `reflect`, `time`, `crack` —
  substrate-decl'd, docblock-forward-promised mirror.spec entries.
- Depth-2 recursive-command grammar landed (cli.mirror Tick 1) — Rust
  depth-2 dispatch landed for `peer beam` + `peer contribute` (lib.rs
  3325-3470), but no other consumer.
- `@mirror/lens/mcp`, `@mirror/lens/shell`, `@mirror/lens/lsp` are
  family-header-only — bodies land when consumers pull. Substrate frame
  is right; the transports haven't grown consumers past the MCP tick.

### 4.5 The physical substrate mirror's substrate-decl reflects

The `@../prism/` crate at `/Users/alexwolf/dev/projects/prism/` gives
the beam-through-optics reference:

- **Beam** = the beam of light Alex names. Carries `Imperfect<T, E, L>`
  (value + input + loss).
- **Prism** = the composable optic. Three ops in the physical crate
  (focus / project / settle); mirror substrate lifts to five per
  cli-as-prism §1.1 (split / shift added at substrate altitude).
- **Composition** = `apply(&prism, beam) = beam.focus(...).project(...).settle(...)`.
  Prisms compose associatively; the identity element exists
  (`IdentityPrism`); the composition IS a monoid.
- **Loss = Holonomy** per `prism/docs/architecture.md`: the compiler
  produces loss; the loss IS the holonomy of the bundle tower's
  Transport. Loss is not a bug; loss is the observation-of-the-
  observation. Mirror's `@glass.imperfect` + `@glass.transparency` are
  the substrate-decl lift of this carrier.
- **The compiler IS the LSP** per architecture.md:74-83: "one function,
  three patience levels (CLI, REPL, LSP)". Mirror lifts to four
  transport lenses (cli / shell / mcp / lsp) at
  `@mirror/lens` family root.

The GEOMETRY Alex names IS: a beam of light passes through recursively-
composed prisms, each prism carrying its five ops on its own manifold,
loss accumulating as holonomy at every transport hop. The CLI surface
IS one of those prisms (the terminal lens), and its subcommand nesting
IS prism-in-prism at the shell altitude.

---

## §5. Substrate-honest bounds — what this scout does NOT decide

Taut is a read-only grep scout. This report empirically enumerates; Mara
condenses. The following are **residue for Mara + Alex adjudication**:

1. **Does the five-op top-level surface land?** Cli-as-prism §2.2
   forward-promised `mirror focus / project / split / shift / settle`.
   None dispatch today. Landing them is a Mara call.

2. **Do the sub-stages get five-op dispatch?** Every sub-stage shard
   declares five ops with operands. Dispatching them (so `mirror
   kintsugi focus <spec>` peeks the next tournament move) is a Mara
   call. Substrate is ready; Rust dispatch is not.

3. **Do the 5 unwired sub-stages get wired into mirror.spec?**
   `bootstrap`, `sh`, `reflect`, `time`, `crack` each declare their
   sub-stage prism. Wiring them into `mirror.spec cli { command X }` is
   a Mara call (each shard's docblock forward-promises this).

4. **Do the 6 orphan verbs get sub-stage shards or absorb into existing
   stages?** `craft`, `init`, `recall`, `beam`, `index`, `peer` have no
   `shards/mirror/lens/cli/*.mirror` declarations. Whether they mint
   sub-stage shards or collapse into existing families is a Mara call.
   Sub-questions:
   - Is `craft` a sub-stage of `compile` (or vice versa)?
   - Is `init` a sub-stage of `store`? (There's `@mirror/store` at
     `shards/mirror/store.mirror` — the store family exists.)
   - Is `beam` (anonymous variant) a top-level or a sub-stage of
     `peer`? The depth-2 `peer beam` variant is the persistent-identity
     form; the depth-0 `beam` is the anonymous form. Geometry could
     collapse them (all `beam` → `peer beam --anonymous`?) or preserve
     the two-name convenience.
   - Is `recall` a sub-stage of `peer`? Docstrings call it "inbound-
     trajectory dual of spawn" (per `mirror.spec:142-152`).
   - Is `index` a sub-stage of `refract` or its own? `mirror.spec:174`
     docblock forward-promises collapse to `@fractal/index` after Alex
     adjudicates #6.

5. **Does `mirror_index` MCP tool stay a top-level tool, or lift into
   `mirror_refract`?** Corresponds to Q4 for MCP.

6. **Does `peer beam` collapse its 11 flags into geometric-honest sub-
   commands?** Each flag today (`--song`, `--dance-with`, `--deploy-to`,
   `--emit-crystal`, `--fate-select`, `--from-psychohistory`,
   `--with-shadow`, `--emit-diff`, `--integrate-diff`) triggers a
   dispatch branch. Some may be MODES on the beam (adverb-shape per
   `docs/specs/cli-as-prism.md` §4.1 `watch` finding); others may be
   sub-species (e.g. `peer beam song`, `peer beam dance`, `peer beam
   deploy`). Mara adjudicates.

7. **Does the physical `Prism` trait lift to five ops, or does the
   substrate collapse to three-plus-two-modes?** Per §4.3.8, the
   crate at `/Users/alexwolf/dev/projects/prism/prismqueer/src/lib.rs:126`
   is three-op. Substrate is five-op. Divergence is real; adjudication
   between "lift physical" and "collapse substrate" is a Mara call.

8. **Does the `spawn` alias sunset, or persist indefinitely?** Currently
   deprecated but still dispatching per `b012d3f`. Two-tick discipline
   suggests sunset one tick after `peer beam` is universally consumed;
   Mara + Alex decide when.

9. **Does the top-level `mirror '<mq-query>'` mode (usage() 632) fit
   the five-op frame?** The mq-pipeline-over-stdin dispatch is a
   separate top-level entry mode (not part of the sub-stage tree).
   Whether it collapses into `mirror settle @mq(...)` or persists as
   a distinct mode is a Mara call.

10. **Does `contribute` (mirror.spec:333-336) exist as a
    `shards/mirror/lens/cli/peer/contribute.mirror` sub-species, or
    stay wired without a substrate declaration?** Depth-2 mint would
    close the sub-stage/wire drift for the peer family; not landed.

---

## Appendix A — Cited files with paths + lines

Every path this report cites, absolute:

**Mirror substrate declarations** (Axis 1 + 2):

- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens.mirror` (lens
  family root; 126 lines; 8-species enumeration at 33-70)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/cli.mirror`
  (CLI family; 230 lines; prism block 108-114; recursive-command
  grammar 116-160)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/mcp.mirror`
  (66 lines; family-header only; prism block 44-50)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/shell.mirror`
  (91 lines; family-header only; prism block 63-69)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/lsp.mirror`
  (66 lines; family-header only; prism block 44-50)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/unix.mirror`
  (204 lines; abstract impedance surface; prism block 143-149)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/unix/fuse.mirror`
  (188 lines; platform sub-species)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/refract.mirror`
  (128 lines; measurement lens; prism block 84-90)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/transit.mirror`
  (180 lines; measurement lens; prism block 97-103)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/knife.mirror`
  (336 lines; COORD-jump lens; prism block 141-147)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/cli/compile.mirror`
  (142 lines; stage 73-101; `default settle`)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/cli/kintsugi.mirror`
  (327 lines; stage 246-273; `default focus`; 3 kintsugi actions)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/cli/shatter.mirror`
  (155 lines; stage 84-117; `default settle`)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/cli/bootstrap.mirror`
  (167 lines; stage 88-128; `default focus`; not wired into mirror.spec)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/cli/sh.mirror`
  (193 lines; stage 114-151; `default settle`; not wired)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/cli/reflect.mirror`
  (165 lines; stage 90-127; `default focus`; not wired)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/cli/time.mirror`
  (185 lines; stage 100-140; `default focus`; not wired)
- `/Users/alexwolf/dev/projects/mirror/shards/mirror/lens/cli/crack.mirror`
  (267 lines; stage 163-203; `default focus`; not wired; inline
  `crack_mode` typed enum)

**Mirror `.spec` cli-block** (Axis 1):

- `/Users/alexwolf/dev/projects/mirror/mirror.spec` (445 lines; cli
  block ~71-345; 11 top-level verbs wired: compile, kintsugi, shatter,
  craft, init, recall, beam, index, peer{beam, contribute})

**Mirror Rust dispatch** (Axis 1):

- `/Users/alexwolf/dev/projects/mirror/bootstrap/src/main.rs` (24 lines;
  binary entry)
- `/Users/alexwolf/dev/projects/mirror/bootstrap/src/lib.rs` (5757 lines;
  `dispatch` at ~2870; match arms 3149-3609; `usage` at ~630-645;
  `cmd_compile` 645; `cmd_craft_with` 740; `cmd_kintsugi` 1106;
  `cmd_shatter` 3614; `cmd_init` 3722; `cmd_recall` 3924; `cmd_peer_beam`
  5159)
- `/Users/alexwolf/dev/projects/mirror/bootstrap/src/mcp.rs` (43.7KB;
  8-tool MCP schema + `dispatch_tool_call` at ~501-842; docblock
  1-90 for full tool enumeration)
- `/Users/alexwolf/dev/projects/mirror/bin/mirror-mcp` (18-line shim;
  bin/mirror-mcp:16-17)

**cli-as-prism spec** (Axis 1 + 4):

- `/Users/alexwolf/dev/projects/mirror/docs/specs/cli-as-prism.md`
  (45.6KB; §0 constraint; §1 recursion; §2 verb table; §3 file
  structure; §4 strains; §5 sings; §6 comparison; §7 default-op rule;
  §8 non-goals; §9 open Qs; §10 proof)
- `/Users/alexwolf/dev/projects/mirror/docs/specs/the-convergence.md`
  (35.3KB; four-transport table; §1 recognition; §2 per-transport map)

**Sibling `prism` crate** (Axis 3):

- `/Users/alexwolf/dev/projects/prism/Cargo.toml` (workspace; 15 lines)
- `/Users/alexwolf/dev/projects/prism/README.md` (95 lines; three-crate
  enumeration + dependency direction diagram)
- `/Users/alexwolf/dev/projects/prism/imperfect/README.md` (206 lines;
  `terni` public API; `Imperfect<T, E, L>` + `Loss` trait + ternary
  states)
- `/Users/alexwolf/dev/projects/prism/imperfect/src/lib.rs` (100.5KB;
  `Imperfect` + `Loss` + `Diagnostic`)
- `/Users/alexwolf/dev/projects/prism/imperfect/src/transparency.rs`
  (13.1KB; `Transparency` + `PropertyVerdict`)
- `/Users/alexwolf/dev/projects/prism/prismqueer/README.md` (91 lines;
  `Prism` trait + feature flags)
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/lib.rs` (13.4KB;
  `Prism` trait 126-142; module enumeration 32-70; feature-gated
  modules 53-69)
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/beam.rs` (23.8KB;
  `Beam` + `Optic` + `Operation`)
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/optics/{lens,iso,traversal,fold,setter,optic_prism,gather,monoid,mod}.rs`
  (feature `optics`; classical optics as prisms)
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs` (20.3KB;
  feature `bundle`; principal-bundle tower)
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/lambda/mod.rs`
  (16.5KB; feature `lambda`)
- `/Users/alexwolf/dev/projects/prism/projections/src/{lib.rs,declaration.rs}`
  (58.9KB total; `#[derive(Prism)]` + `declaration!{}` + `#[derive(Lambda)]`)
- `/Users/alexwolf/dev/projects/prism/beam/src/prism_beam.gleam` (2.3KB;
  BEAM/Gleam `Beam(t)` mirror; type surface equivalence with Rust)
- `/Users/alexwolf/dev/projects/prism/beam/gleam.toml` (Gleam project
  metadata; targets Erlang)
- `/Users/alexwolf/dev/projects/prism/docs/architecture.md` (7.5KB;
  the stack + the loop + the CLI-IS-the-LSP framing)

---

*End of scout. Alex directive fed: ground truth for Mara's CLI-spec
condensation. Scout does not decide. Taut / read-only.*
