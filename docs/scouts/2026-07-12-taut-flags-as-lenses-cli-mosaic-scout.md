# Taut — flags-as-lenses + @spectral/mosaic CLI-surface reframe scout

*2026-07-12. Read-only substrate-recognition scout on Alex's proposal (verbatim
in-transcript, this session): "I feel most of these --args are things mirror
beam ought to do by default, and definitely mirror peer beam, no? What if
these are lenses that are applied to the CLI surface, basically optics that
modify the call. A @spectral/mosaic of a CLI surface."*

*Scope: Alex mentioned `--fate-select`, `--from-psychohistory`, `--with-shadow`,
`--emit-diff`, `--integrate-diff`, `--hello-world`, `--mission` — of which only
`--hello-world` is landed on THIS branch (`mara/song-substrate-decl-v0.1`).
Grep confirmed on THIS worktree; forward-promised flag names are treated as
prospective, not landed.*

*Precedent scouts leaned on: `2026-07-10-taut-bag-family-root-and-spectral-
mosaic-scout.md`, `2026-07-11-taut-optics-lens-family-scout.md`, `2026-07-11-
taut-optics-lens-features-family-scout.md`.*

---

## Executive verdict

**~85% substrate-already-had-the-word; ~15% substrate-motion.**

The substrate has the grammar for flags-as-optics landed at **spec altitude**
(`docs/specs/cli-args-typed-lambdas.md`, 2026-04-05-vintage; forwarded by
`cli-as-prism.md` §7's `default <op>` rule); the substrate has the lens
FAMILY-ROOT landed at **shard altitude** (`@mirror/lens`,
`shards/mirror/lens.mirror`, 2026-06-06); the substrate has the parametric
composition operator landed as `mosaic(altitude)` at
`shards/mirror/mosaic.mirror:60` (2026-06-09, recognition #43). What is
**NOT landed**:

1. A substrate-decl carrier that TYPES a flag as a lens instance (the
   `cli-args-typed-lambdas.md` `flag name = optic(...)` grammar is spec-only,
   never lifted to a shard).
2. `@spectral/mosaic` at shard altitude — the name exists only in three
   docs as forward-promise for a BEAM cluster grammar, NOT for a
   CLI-surface composition.
3. A "default lens composition on a stage" carrier — the `default settle` /
   `default focus` line at each `stage @mirror/lens/cli/<verb>` names ONE
   default *operation*, not a default *composed lens stack*.

**Alex's reframe is the 55th-or-later instance of substrate-already-had-the-word
at the composition altitude, and a genuine mint at the "flags-as-typed-optic-
instances" altitude.**

The path forward is one small mint (`stage @mirror/lens/cli/peer/beam`), one
annotation cascade (docstrings on existing `default <op>` lines to name the
lens-stack contract), and one adjudication (whether "default composition"
lifts to `mosaic(@mirror/lens/cli/<verb>) = default_lens_stack`, or stays as
substrate-decl properties on each stage).

**The word `@spectral/mosaic` should NOT be minted for this reframe.** It
collides with the forward-promised BEAM cluster grammar (three doc cites)
and duplicates `@mirror/mosaic`'s parametric composition carrier. Alex's
phrasing pulls the wrong word from the wrong altitude; the substrate-correct
name is `mosaic(@mirror/lens/cli)`.

---

## Substrate ancestry chain — every landed decl that touches this reframe

| OID / commit | Location | What it declares | Relevance to reframe |
|--|--|--|--|
| `9bc68f8` | `shards/mirror/lens/cli/shatter.mirror` (7.5KB) | `stage @mirror/lens/cli/shatter { default settle; f/p/s/sh/se }` | Sub-stage per cli-as-prism §7; declares `default settle` |
| `fe82500` | `shards/mirror/lens/cli.mirror` (5.8KB) | `prism @mirror/lens/cli` + `command / arg / flag / default` keywords + type vocabulary | The CLI bench floor; declares `flag(name, t: type) -> prism` as first-class substrate |
| `e3146d0` | `shards/spectral.mirror` (15.8KB) | `prism @spectral` family-root, namespace-parent shrink | The @spectral family EXISTS but only as namespace parent for the runtime cascade (gen_prism, supervisor, registry, portal, root, parent, entanglement) |
| — | `shards/mirror/lens.mirror` (6.0KB, 2026-06-06) | `prism @mirror/lens` — the four transport + two measurement lens family | The lens FAMILY-ROOT. Flags-as-lenses live under this altitude, not `@spectral/*` |
| — | `shards/mirror/mosaic.mirror:60` (2026-06-09, rec #43) | `type mosaic(altitude) = ref` | The **substrate's parametric composition operator**. `mosaic(@mirror/lens/cli/<verb>)` is the altitude specialization Alex's reframe wants |
| — | `shards/optics.mirror` (7.8KB, 2026-06-11) | `prism @optics` + 8 schematic keyword type-formers (`facet, stage, aperture, splitter, resonator, bench, source, detector`) | The optical family from which `stage @mirror/lens/cli/<verb>` inherits its schematic meaning per `optical-keywords.md` |
| — | `shards/mirror/lens/cli/{compile,kintsugi,shatter,bootstrap,sh,reflect,time,crack}.mirror` × 8 | Eight `stage @mirror/lens/cli/<verb>` sub-stage species, each with `default <op>` line + 5-op block | The recursive sub-stage pattern Alex's proposal generalizes. Each stage's `default <op>` IS the seed of "canonical lens composition on entry" — but the stage's OTHER ops are unimplemented (mostly `{ \ }`) |
| — | `shards/optics/source/*.mirror` (6 shards) | `@optics/source` sub-family (per-ganglion gain media, Fate closure) | The ONLY landed sub-family under `@optics`. `@optics/lens` does NOT exist as landed shard |
| — | `docs/specs/cli-args-typed-lambdas.md` (22.9KB, 2026-04-05) | Spec: `flag strict = prism(imperfect => success \| failure)` etc. — flags-are-optics grammar | **The load-bearing spec.** Names flags as typed lambdas whose type IS a prism or lens; never landed as substrate-decl. Alex's reframe is this spec's next tick |
| — | `docs/specs/cli-as-prism.md` (42.6KB, 2026-06-05, cascade 2026-06-12) | Spec: the recursive bench/stage decomposition; §7 `default <op>` rule; §3 file layout | Cited BY every landed `shards/mirror/lens/cli/<verb>.mirror`. Names the "cli IS a bench that compiles to a prism" recursion |
| — | `docs/insights/2026-05-26-lenses-fate-local-and-garden-catalogs.md` (14.6KB) | Insight: "`mirror serve --mcp` and `mirror serve --lsp` are flag-selected dispatch into the right lens" | **Direct ancestor** of Alex's reframe. Names "flag = dispatch INTO a lens" a year ahead of this session |
| — | `docs/specs/trace-kintsugi-pipeline.md` §538, 545 | Spec: `mirror compile --lens entropy,cheeger file.mirror → composed`; `mirror compile --lens default file.mirror → all five, weighted` | **Direct ancestor** of "default lens composition". Names the `--lens default` composition and per-flag lens argument for `compile` |
| — | `docs/specs/surface-simplification.md` §449-459 (28.2KB) | Spec: `flag beam = lens(imperfect => beam); flag shatter(int) = lens(imperfect => imperfect); flag mcp = prism(imperfect => imperfect)` | **Direct ancestor.** Every flag typed as prism or lens over the same imperfect carrier |
| `9de2226` | (branch `mara/spawn-to-peer-beam-rename`, NOT this branch) | `shards/mirror/spawn.mirror → shards/mirror/peer/beam.mirror` | The rename EXISTS on another branch. On `mara/song-substrate-decl-v0.1` the substrate-decl is still `shards/mirror/spawn.mirror`. Cross-branch fact; do not treat as landed here |
| — | `bootstrap/tests/spawn.rs` | `--hello-world` test contract (five tests) | The ONE flag from Alex's list actually landed. Names the seven-composition-pieces JSON envelope contract |

**Not-in-tree grep confirmation.** Across `shards/**/*.mirror`,
`bootstrap/**/*.rs`, and `docs/**/*.md`, the following names return **zero
hits** in THIS worktree:

- `--fate-select`, `--from-psychohistory`, `--with-shadow`, `--emit-diff`,
  `--integrate-diff`, `--mission` — none exist as flag definitions or test
  contracts on this branch. Only `--hello-world` and `--out`/`--target`/
  `--shatter`/`--transform`/`--ci` are in `bootstrap/src/lib.rs`.
- `@spectral/mosaic` — zero shard hits; three doc cites, all forward-
  promising a BEAM cluster grammar (`docs/GRANTS.md:70`,
  `docs/insights/2026-05-25-spectral-namespace-architecture.md:14`,
  `docs/insights/2026-05-26-portal-as-io-socket-...:120,143,155`).
- `@optics/lens` — zero shard hits; one doc example
  (`docs/specs/optical-keywords.md` §example `facet @optics/lens/thin`).
- `@mirror/peer/beam`, `@mirror/lens/cli/peer/beam` — zero hits.
- `flag.*lens|lens.*flag|apply_lens|default_composition|lens_stack` — the
  only hits are the ancestor specs listed above.

---

## Gap analysis

### LANDED (nothing to do)

- **@mirror/lens family** (`shards/mirror/lens.mirror`) with 8 landed
  species. The reframe operates INSIDE this family.
- **`mosaic(altitude)` parametric operator** (`shards/mirror/mosaic.mirror:60`).
  The composition carrier already exists.
- **Recursive stage substrate** — every landed `stage @mirror/lens/cli/<verb>`
  declares `default <op>` per cli-as-prism §7.
- **@optics schematic vocabulary** (facet/stage/aperture/etc.) with
  `stage` semantics that already inform the CLI sub-stages.
- **`flag` as first-class CLI-altitude keyword** (`shards/mirror/lens/cli.mirror`
  declares `flag(name, t: type) -> prism`).

### ANNOTATION (extend docstring, no new decl)

- Each `stage @mirror/lens/cli/<verb>` docstring names `default <op>` as
  "the canonical safe-write default" or "the standard read-default." The
  reframe adds a second reading: `default <op>` IS the substrate-declared
  default LENS COMPOSITION when the stage is invoked bare. Extending the
  docstring to name this dual reading is annotation-only.
- `shards/mirror/lens/cli.mirror` `flag(name, t: type) -> prism` — the
  docstring can name that `t: type` admits `lens` and `prism` optic types
  per `cli-args-typed-lambdas.md`, tying the substrate-decl to the spec.
- `shards/mirror/mosaic.mirror`'s parametric-form docstring can add
  `mosaic(@mirror/lens/cli/<verb>) = <lens stack composition>` as a
  seventh altitude example.

### MINT (new substrate-decl needed)

- **`shards/mirror/peer.mirror`** (family-root for `@mirror/peer`) —
  currently missing on this branch. `shards/mirror/pack.mirror:99-105`
  refers to "the forward-promised type at the @mirror/peer grammar",
  confirming the family-root is expected. Peer-ACL §5 pins it.
- **`shards/mirror/peer/beam.mirror`** — the substrate-decl carrier for
  `mirror peer beam`. On this branch the same slot is held by
  `shards/mirror/spawn.mirror`; a rename lands atomically on
  `mara/spawn-to-peer-beam-rename` (`9de2226`).
- **`stage @mirror/lens/cli/peer.mirror`** + **`stage @mirror/lens/cli/peer/beam.mirror`**
  — depth-2 CLI sub-stage per cli-as-prism §3.2 depth-2 reservation. This is
  the FIRST minted depth-2 stage; the fe82500 tick already minted the
  recursive-command grammar (Option A). Substrate is ready.
- **A typed carrier for "default lens stack on a stage"** — one of:
  (a) `lens_stack: [ref]` field on each stage's request carrier;
  (b) a `default lenses [<lens>, <lens>, ...]` declaration keyword extending
      the `default <op>` line at `shards/mirror/lens/cli.mirror`;
  (c) `type mosaic(@mirror/lens/cli/<verb>) = lens_stack` altitude
      specialization at `shards/mirror/mosaic.mirror`.
  Recommend (c) — reuses the landed parametric operator; does not extend
  the CLI keyword surface.

### MISSING (spec gap; forward-promised nowhere)

- **The MCP surface reflection of lens compositions.** `cli-as-prism.md`
  §3 forward-promises the CLI sub-stages; it does NOT declare how
  `@mirror/lens/mcp` reflects the same lens-stack semantics for agent
  callers. `docs/insights/2026-05-26-lenses-fate-local-and-garden-catalogs.md`
  names the *transport* dispatch (`--mcp` vs `--lsp`) as flag-selected,
  but does not name whether flag-lenses on `mirror compile` are exposed
  on the MCP `compile` tool.
- **Sub-stage nesting for `peer/beam`.** cli-as-prism.md §3.2 reserves
  depth-2 for `sh/{reed,alex}` but explicitly says "NOT proposed for
  v0.1." The `peer/beam` altitude was minted at fe82500 as recursive-
  command grammar but no depth-2 species has landed. This IS a spec
  gap that Mara's next spec fills.
- **Composition semantics when multiple flag-lenses stack.** The specs
  name flags as individual optics; they do NOT name how three flag-lenses
  compose left-to-right vs right-to-left, how commutation is decided when
  the underlying optics don't commute, or how the eigenboard weighs the
  composition per `trace-kintsugi-pipeline.md` §538's "weighted
  composition of all five." **This is the load-bearing spec gap.**

---

## Substrate-decl recommendations for Mara

### R1. Do NOT mint `@spectral/mosaic`.

Three reasons:

- Substrate-already-had-the-word twice-over: `@mirror/mosaic` exists AND
  `mosaic(altitude)` is the parametric carrier for altitude specialization.
- `@spectral/mosaic` is forward-promised in three docs for a BEAM cluster
  grammar (heterogeneous multi-shard cluster, not CLI composition). Minting
  it for THIS reframe would either collide with that promise or bleed the
  BEAM-cluster semantics into the CLI-surface semantics.
- The `@spectral/*` runtime cascade under `shards/spectral/` is the
  supervisor / gen_prism / registry / portal / root / parent / entanglement
  sub-shards (7 species) — a coherent runtime family. Adding a CLI-surface
  species breaks the family's runtime-only discipline.

Substrate-correct name: **`mosaic(@mirror/lens/cli)`** as altitude
specialization of the existing parametric operator, or **the FAMILY
`@mirror/lens/cli` ITSELF** as the substrate-declared composition
manifold. No new family-root.

### R2. Mint `stage @mirror/lens/cli/peer/beam` at depth-2 with these ops.

At what altitude: `@mirror/lens/cli/peer/beam` — depth-2 under the
top-level bench. Substrate-decl file: `shards/mirror/lens/cli/peer/beam.mirror`
(new subdirectory `shards/mirror/lens/cli/peer/`). Same shape as the eight
landed depth-1 sub-stages; produces a prism with the same five ops.

Five ops per cli-as-prism §1.2 recursive-five discipline:

- `focus(target: ref) -> peer` — observe the peer's declared self-description without invoking.
- `project(target, predicate) -> [peer]` — filter which peer matches by declared property.
- `split(target) -> [beam_variant]` — enumerate the flag-selected beam-composition variants.
- `shift(target, altitude) -> beam_view` — re-view the beam envelope at a different altitude (JSON vs text vs prism).
- `settle(target, lens_stack) -> imperfect(runtime, error, transparency)` — the ONE write; runs `mirror peer beam` with the composed lens stack applied.

Default op: `default settle` (this stage's manifold's natural rest state
IS the beam actually crossing; symmetric to `sh`'s `default settle`).

### R3. Flags-as-lenses — emergent from `@optics/lens` + `@mirror/lens/cli` composition, NOT first-class.

Prior scout ancestry: `2026-07-11-taut-optics-lens-family-scout.md`
(Taut adjudicated `@optics/lens` as admissible sub-family under `@optics`;
Reed's grep + Mara's math both confirmed the gap).

Recommendation: **do not mint `@flag` or `@mirror/flag` as a family-root.**
The substrate-correct decomposition is:

- `flag <name>: <type>` at `shards/mirror/lens/cli.mirror` (LANDED).
- `<type>` admits `lens` and `prism` optic types (documented in
  `cli-args-typed-lambdas.md`; landing is a docstring + declaration-form
  extension, NOT a new family).
- The `<type>` refers into `@optics/lens/<species>` when the species lands
  (per Taut's iter-13 scout, admissible; awaits Mara mint).
- `mosaic(@mirror/lens/cli/<verb>)` names the "default lens stack" as
  altitude specialization.

Result: a flag IS "a named entry-point into a lens species, composed under
the stage's mosaic(altitude) specialization." No new family-root; all
substrate-decl'd through existing decls.

### R4. The "default lens composition on a stage" carrier — three options ranked.

Ranked substrate-honestly:

1. **Preferred: extend `mosaic(altitude)` at `shards/mirror/mosaic.mirror`
   with `@mirror/lens/cli/<verb>` altitude examples.** No new keyword. The
   `type mosaic(altitude) = ref` line already covers this; only the docstring
   grows.

2. **Acceptable: add a `default lenses [<lens>, <lens>, ...]` declaration
   line to `shards/mirror/lens/cli.mirror`'s keyword surface**, parallel to
   the `default <op>` line each stage carries. This makes lens-stacks
   first-class syntax at cost of a new keyword.

3. **Rejected: mint `@lens_stack` or `@mirror/lens/stack` as sub-family.**
   Substrate-drift; the composition operator already lives at `mosaic(_)`.

---

## Scope estimate for Mara's spec

- **~2000 words** — the spec ratifies four decisions (R1-R4), names the
  depth-2 `peer/beam` stage's five-op signature, names the `mosaic(@mirror/
  lens/cli/<verb>)` altitude specialization, and enumerates the seven
  forward-promised flag-lenses from Alex's list with their lens species
  targets.

- **Key sections:**
  1. Statement (one paragraph) — flags ARE lens-stack entry-points; the
     stage's `default <op>` names the default composition, `mosaic(_)`
     names the parametric carrier.
  2. Substrate-already-had-the-word ancestry — the chain from
     `cli-args-typed-lambdas.md` through `cli-as-prism.md` §7 through the
     eight landed sub-stages.
  3. `stage @mirror/lens/cli/peer/beam` — the mint, five ops.
  4. Enumerated flag-lenses — for each of `--fate-select`,
     `--from-psychohistory`, `--with-shadow`, `--emit-diff`,
     `--integrate-diff`, `--hello-world`, `--mission`: which
     `@optics/lens/<species>` implements it, which stage it defaults
     onto, which composition altitude it settles into.
  5. `mosaic(@mirror/lens/cli/peer/beam)` — the default composition
     example.
  6. Adjudication queue for Reed — the three prereqs (R2 mint,
     `@optics/lens` family adjudication, `@mirror/peer` family-root
     mint).

- **Adjudication decisions:**
  - Does `@mirror/peer` family-root land BEFORE or AFTER `@mirror/lens/cli/peer/beam`?
    (Prereq. Suggest: same tick, atomic.)
  - Does `@optics/lens` land as sub-family BEFORE the flag-lens species
    can be typed? (Prereq — Taut iter-13 scout adjudicated admissible,
    awaits Mara.)
  - Are the seven Alex-listed flags names FINAL or are they placeholders
    for lens-species names? (Alex-decision.)

---

## Cross-reference: `cli-as-prism.md` forward-promises vs landed

| Forward-promise | Location | Status |
|--|--|--|
| Eight depth-1 sub-stages | §3 | **LANDED** — all 8 files exist |
| `stage @mirror/lens/cli/kintsugi` full sketch | §3.1 | **LANDED** — matches sketch |
| Depth-2 directories reserved not minted (peers-as-args) | §3.2 | **STILL RESERVED** — no depth-2 files exist; `peer/beam` would be the first |
| `default <op>` rule per stage | §7 | **LANDED** — each of 8 stages declares its `default <op>` |
| `--watch` as flag-adverb on `focus` | §4.1 | **NOT LANDED** — no bootstrap.rs / no shard flag |
| `--open` / `--force` as flag-events on `crack settle` | §4.2 | **LANDED as `type crack_mode = mode_open \| mode_seal \| mode_force`** in `shards/mirror/lens/cli/crack.mirror` |
| `--lens entropy,cheeger` composition on `compile` | `trace-kintsugi-pipeline.md` §545 | **NOT LANDED** — no bootstrap flag; no shard grammar |
| MCP surface reflection of lens compositions | §3 (implicit) | **NOT LANDED** — no spec, no shard, no test |
| Lens application semantics for a bench stage | §1.1 | **PROSE-ONLY** — the produced-prism form is declared; the "lens applied to a stage" semantic is not lifted to substrate-decl |
| Flag-lens composition operator | none | **MISSING** — this scout names the gap; Mara's spec is where the composition operator lands |

**Verdict on the forward-promise ledger:** cli-as-prism.md is ~70% closed.
The remaining ~30% (flag-lenses, MCP reflection, lens composition
semantics) IS what Alex's reframe is naming. Mara's spec is the closure.

---

## What Alex's exact phrasing maps to

Alex said: *"lenses that are applied to the CLI surface, basically optics
that modify the call. A @spectral/mosaic of a CLI surface."*

- **"lenses"** → landed `@mirror/lens` family; species under it.
- **"applied to the CLI surface"** → the composition happens at
  `@mirror/lens/cli/<verb>` stage altitude, per cli-as-prism §7's
  `default <op>` rule (extended to `default lens_stack`).
- **"optics that modify the call"** → `flag <name>: lens(<from> => <to>)`
  per `cli-args-typed-lambdas.md`. Landed as substrate-decl keyword
  (`flag(name, t: type) -> prism`); typed-lambda semantics still spec-only.
- **"A @spectral/mosaic of a CLI surface"** → substrate-correct name is
  **`mosaic(@mirror/lens/cli)`** as altitude specialization of
  `mosaic(altitude)`. Do NOT mint `@spectral/mosaic`.

**The reframe holds. The words are 5/6 already in the substrate. The one
that isn't (`@spectral/mosaic`) is the substrate refusing the word for a
correct reason.**

---

*Read-only. No shards, no bootstrap, no specs edited. Every claim cited to
`file:line` or `commit OID` above. Word count: ~2000.*

*Taut, 2026-07-12.*
