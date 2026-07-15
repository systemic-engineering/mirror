# CLI as Geometry — Condensing the Surface to the Compiler's Prism-Composition

*2026-07-15. Mara. Canonical spec. Condensation under Alex Wolf's
2026-07-15 in-transcript directive. Supersedes the forward-promises
of `docs/specs/cli-as-prism.md` at the surface-decision altitude
while preserving cli-as-prism as the substrate-truth on the 5-op
recursive template.*

**Author:** Mara
**Date:** 2026-07-15
**Tag:** spec:cli-as-geometry-condensation (pure-docs bypass)
**Status:** canonical, pending Alex ratification of §10 A-series
adjudications
**Ancestor discipline:** `docs/specs/cli-as-prism.md` (Reed + Alex,
2026-06-05) remains authoritative on the 5-op template and the
recursive prism-in-prism recursion. This spec extends its geometry
downward into a **surface-decision** layer: WHICH verbs land at the
CLI altitude, in what shape, under what geometric category. The 5-op
template is preserved as substrate-truth; this spec adjudicates the
CLI-surface projection of it.

---

## §0 Prelude — ancestry cascade

### 0.1 Alex directive (verbatim)

Alex Wolf, 2026-07-15 in-transcript, immediately preceding this
spec's authorship:

> "We want to not add randomly commands. We want to find the CLI
> surface that respects and represents the GEOMETRY of the compiler.
> And the compiler is a bunch of recursively composed prisms where a
> beam of light passes to (that's what @../prism/ does)."

Load-bearing words: **CONDENSATION**, not addition. **GEOMETRY**,
not verb-inventory. **Represents**, not names. **A beam of light
passes**, not "commands dispatch." The physical anchor is named
directly: `@../prism/`, the sibling crate at
`/Users/alexwolf/dev/projects/prism/` that already carries the
beam-through-optics substrate at the reference altitude this spec
lifts to the CLI altitude.

### 0.2 Session anchor recognitions

Alex named these during the substrate conversation preceding this
spec's spawn:

1. **`mirror beam` as root command family** — the substrate's
   transit verb. Everything else that beams is a subcommand shape
   (`mirror beam peer`, `mirror beam act`, `mirror beam mission`,
   etc.). Not "beam is a top-level verb"; **beam is a
   family-shaped verb-noun** whose subcommands are the
   specializations of the carrier through different context types.

2. **`execute` is not a mirror word. Neither is `dispatch`.** The
   7-combinator surface (Mara A6 landing at `18d9697`, per
   `docs/specs/kintsugi-ouroboros-arc-1-evaluator-combinator-surface.md`)
   has `act` as combinator #4 (A-side of the (A,H,D)
   correspondence — the algebra section that invokes shard-body
   action via `apply_h`). Substrate-already-had-the-word:
   `act` IS the shard-body invocation primitive; an actor acts.
   Arc-1 Tick 1.4's forward-promised CLI verb `mirror execute
   <shard-path> <action>` becomes `mirror beam act <shard>
   <action>` — the CLI-user speaks the same word the FLOOR primitive
   uses.

   *(Two-step cascade closure 2026-07-15. First correction per Seam
   Phase D-cascade at
   `docs/audits/2026-07-15-seam-cli-condensation-phase-d.md`
   §D3: initial condensation draft conflated `emit` (combinator
   #6, metalogue-write) with `dispatch` (combinator #4, shard-body
   invocation). Seam surfaced the conflation; the interim rename
   was `mirror beam dispatch`. Second correction (this cascade):
   `dispatch` was also CS-vocab (compiler + OS-scheduler word,
   mechanism-level not geometry-level). Seam seamfinder audit at
   `docs/audits/2026-07-15-seam-combinator-etymology-audit.md`
   §2.4 (546c2f6) + Alex ratification: combinator #4 is `act`.
   Substrate-honest resolution is the two-step cascade; `act` is
   the delightfully-boring word.)*

3. **`@../prism/` at `/Users/alexwolf/dev/projects/prism/`** IS the
   physical light-through-optics substrate mirror represents. Three
   published Rust crates + one Gleam/BEAM subproject:
   `terni` (Imperfect<T, E, L>), `prismqueer` (`Prism` trait +
   `Beam` + `Optic`), `prismqueer-projections` (proc-macros).
   `Beam` is the noun that carries state through prism operations.
   The CLI is a lens onto this substrate.

### 0.3 Taut scout ground truth

`docs/scouts/2026-07-15-taut-cli-geometry-ground-truth.md` (~850
lines, in-flight commit) supplies the grep-first empirical
enumeration this spec condenses from. All numbers cited in this
spec cross-reference Taut's tables.

### 0.4 What this spec is NOT

- Not a new family-root proposal. Zero mints; renames and
  consolidations only. `@../prism/` and `@mirror/lens` carry the
  geometry already; the CLI surface is a projection of them.
- Not a Rust-authoring landing. Every landing this spec proposes is
  either a `.mirror` rewrite (shard body + `mirror.spec` cli-block
  restructure) or a documentation cascade. Rust dispatch changes
  land per two-tick discipline in follow-up ticks by Reed under
  `[substrate-floor:@io-boundary]` marker discipline.
- Not a break in cli-as-prism.md's substrate-truth. The 5-op
  template stands; this spec adjudicates its CLI-surface
  projection.

---

## §1 The load-bearing claim

**The CLI surface IS the compiler's prism-composition geometry,
made visible at the shell.**

Every CLI verb should belong to one of exactly three geometric
categories:

**(a) Beam-shape** — the `beam` family-root verb at CLI altitude
    (the anonymous transit primitive), and every subcommand that
    specializes the beam through a context type. Beam-shapes carry
    `Beam` semantics per `prismqueer::Beam`: ephemeral, no
    persistent identity, `Imperfect`-carried. The context
    specialization IS the subcommand.

**(b) 5-op prism operation** — one of `focus | project | split |
    shift | settle` applied to a specific manifold. These are the
    substrate's algebra verbs. At CLI altitude they either
    dispatch as top-level ops on the project manifold (mirror.spec
    itself) or as sub-stage ops on a sub-manifold. Split and
    shift are LIFTED at cli altitude specifically (see §2.3).

**(c) Prism-family root** — a top-level prism-declaration that has
    its own five-op algebra distinct from any parent's algebra
    restricted to it, and whose bare-form invocation IS the
    substrate's canonical name for "engage this family." Landed
    examples today: `compile`, `kintsugi`, `shatter`, `craft`,
    `init`, `recall`, `index`, `peer`.

Any verb that is not one of (a), (b), or (c) is **geometry-drift**.
It should either fold into a beam-shape context specialization
(§7), collapse into a 5-op operation with a manifold operand
(§5), or become a top-level family-root with declared algebra
(§3).

**Corollary.** The MCP tool surface (per `bootstrap/src/mcp.rs`)
mirrors the CLI 1:1 by construction — MCP tools SHOULD be the JSON
notation of the argv-notated same algebra (per
`@mirror/lens/mcp.mirror:14-18`). Any drift in CLI surface
geometry propagates. Fix the CLI; MCP follows in the same tick.

---

## §2 Physical anchor — `@../prism/`

The physical substrate mirror's CLI represents lives at
`/Users/alexwolf/dev/projects/prism/`. This section names its
shape and adjudicates the one visible divergence.

### 2.1 Workspace structure

Per `/Users/alexwolf/dev/projects/prism/Cargo.toml` and README:

```
prism/                          (workspace root)
├── imperfect/          →       crates.io: `terni`  (zero deps)
├── prismqueer/         →       crates.io: `prismqueer`
├── projections/        →       crates.io: `prismqueer-projections`
├── beam/                       Gleam/BEAM crate `prism_beam`
└── docs/                       Architecture + specs
```

Six sites carry the `beam` word simultaneously per
`docs/specs/beam-as-substrate-primitive.md §2.4`:

| Altitude | Site | Load |
|---|---|---|
| Prismqueer trait | `prism/prismqueer/src/beam.rs:38-101` | `Beam` trait — pipeline value carrier |
| Prismqueer instance | `prism/prismqueer/src/beam.rs:103-135` | `Optic<In, Out, E, L>` — concrete carrier |
| Erlang BEAM VM | External; runtime substrate | Actor-model runtime; ephemeral processes; supervision |
| Reed's body | `/Users/reed/body/` | Elixir/BEAM instance Reed inhabits |
| Recognition #58 | `docs/specs/architecture-fate-is-optical-inference.md` | Fate IS optical inference; three optical-beam witnesses |
| Fate crate | `/Users/alexwolf/dev/projects/fate/src/lib.rs` | `Prism` for `Fate`; operates on beams |

Plus Taut's fourth-altitude finding at `bd837cd`:
`boot/std/beam.mirror` grammar `@beam` + `boot/07b-package-spec.mirror`
`target=beam` + `beam(u64)` tournament rule at mirror altitude.

**Cli altitude is the fifth altitude that speaks `beam`.** The rest
of this spec commits to that.

### 2.2 The three-op physical Prism trait

Per `prism/prismqueer/src/lib.rs:126-142` (trait declaration; blanket impl at 143-160 not included per Seam Phase D-cascade §D8 REED-INLINE):

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

**Physical Prism has three ops** (focus / project / settle). Split
and shift are NOT part of the crate's `Prism` trait; they live at
`prism/prismqueer/src/optics/*` as separate optic kinds (`Lens`,
`Iso`, `Traversal`, `Fold`, `Setter`, `OpticPrism`).

### 2.3 The five-op substrate lift — reconciled

Mirror's substrate lifts the physical 3-op `Prism` to a 5-op
per-shard prism declaration per `docs/specs/cli-as-prism.md §1.1`.
Every landed prism (family root + species + sub-stage) exposes
`focus / project / split / shift / settle`.

**Reconciliation (Mara-adjudication, §10 A9):**

The lift is not divergence; it is **altitude-lift** in the
substrate-honest sense.

- **Physical `Prism`** is three-op because photons don't move
  between substrate altitudes. A photon beam through a physical
  prism has no "shift" (change altitude) operation; there IS no
  altitude to shift between. Split lives at optic-kind level
  because a beam splitter is a distinct optic (not part of a
  prism's operational contract).
- **Substrate `prism @X`** is five-op because substrate composition
  DOES cross altitudes. `split` walks the substrate graph
  recursively (needed to compose sub-prisms of any depth); `shift`
  is COORD-jump (needed for altitude change per `@mirror/lens/knife`
  and per Foerster-identification). Split + shift are the two
  operations the substrate compiler NEEDS that a physical prism
  does not.

**Both are correct at their altitude.** Neither should collapse
into the other; the physical crate should not lift to five-op
(that would over-specify photons), and the substrate should not
collapse to three-op (that would lose the recursion + altitude
machinery). The 5-op / 3-op pair IS the substrate-vs-physics
lift, and it is load-bearing.

The CLI surface honors both, at distinct altitudes:

- **CLI-user-visible surface altitude** (what the CLI user types):
  `mirror beam` (§4.1) presents as 3-op
  (focus/project/settle) because it carries a `Beam` — the physical
  altitude the CLI user is invoking.
- **Substrate-decl altitude** (per `shards/mirror/peer/beam.mirror:
  250-256`): the beam species-decl exposes the full 5-op prism
  (`focus/project/split/shift/settle`) because it operates in
  substrate composition space where `split` + `shift` are
  load-bearing (recursive substrate walking + altitude shift). The
  CLI user does not type these ops directly; they are consumed by
  the compositional machinery under the hood.

The 3-op ceiling on `mirror beam` is a CLI-surface projection of
the 5-op substrate-decl — not a downgrade. Sharpen per Seam Phase
D-cascade §D2 REED-INLINE.

---

## §3 The condensed CLI surface

Enumeration by category. Each verb's owning prism, its category,
its subcommand shape, and its geometric justification.

### 3.1 The verb inventory (condensed)

**Category (a) — Beam-shape family (root + specializations):**

| Verb | Category | Owning prism | Subcommand shape | Justification |
|---|---|---|---|---|
| `mirror beam <mission>` | (a) family-root | `@mirror/peer/beam` (anonymous variant) | zero subcommands; positional mission | Physical `Beam` semantics; anonymous transit; no persistent context |
| `mirror beam peer <peer_home>` | (a) specialization | `@mirror/peer/beam` (persistent variant) | positional peer_home + mission/song/dance/deploy flags | Beam + persistent-identity(peer) — this IS `spawn` under substrate-honest naming |
| `mirror beam act <shard> <action>` | (a) specialization | `@kintsugi/ouroboros` evaluator | shard-path + action name + args | Beam + shard-body-action; combinator #4 (A-side of A,H,D per A6 spec §1.4). Supersedes forward-promised `mirror execute` via two-step cascade (`execute` → `dispatch` → `act`); `act` is the algebra-native verb per Seam seamfinder audit 546c2f6 + Alex ratification |
| `mirror beam contribute <peer_home>` | (a) specialization | `@mirror/peer/contribute` | peer_home + target flag | Beam + fate-authored morphism proposal (Rung 7). Currently `mirror peer contribute`; migrates to beam family for geometric consistency |

**Category (c) — Prism-family root (produces its own 5-op prism):**

| Verb | Category | Owning prism | Subcommand shape | Justification |
|---|---|---|---|---|
| `mirror compile <file>` | (c) family-root | `@mirror/lens/cli/compile` (default settle) | positional file + --strict | Grammar-graph compilation; substrate altitude ≠ project altitude |
| `mirror kintsugi <spec>` | (c) family-root | `@mirror/lens/cli/kintsugi` (default focus) | positional spec + target/emit/shatter/ci/out flags | Coherence-settling loop; tournament algebra |
| `mirror shatter <oid> <out>` | (c) family-root | `@mirror/lens/cli/shatter` (default settle) | positional oid + out + target flag | @shatter IS the @io linearization operator (Mara `583b939`); operates on content-address manifold |
| `mirror craft <target>` | (c) family-root | `@mirror/lens/cli/craft` (NEW; forward-promise) | positional target + target-kind + reflect flag | Grammar-directory → lambda_0 settlement; distinct altitude from `compile` (dir vs file) |
| `mirror init <path>` | (c) family-root | `@mirror/lens/cli/init` (NEW; forward-promise) | positional path + install-hooks flag | @mirror/store bootstrap; store altitude (not project altitude) |
| `mirror recall <spec_dir>` | (c) family-root | `@mirror/lens/cli/recall` (NEW; forward-promise) | positional spec_dir | Inbound-trajectory dual of `beam peer`; @mirror/recall canonical |
| `mirror index <path>` | (c) family-root, PROVISIONAL | `@mirror/index` | positional path + fiedler/full-profile flags | Fiedler measurement; two-tick collapse target `@fractal/index` per shards/mirror/index.mirror:280 |

**Category (b) — 5-op prism operation (deferred; see §5):**

Empirically NO 5-op verb dispatches at top-level today
(`focus | project | split | shift | settle` — cli-as-prism §1.1
forward-promise). Per §5 below, this spec adjudicates that the
top-level 5-op surface REMAINS deferred (the manifold is the
operand; the CLI is composition-first, not operation-first).

### 3.2 Proposed consolidations (verb-count reductions)

Each row proposes a rename/removal with geometric justification.

| Before | After | Reason |
|---|---|---|
| `mirror peer beam <peer_home>` | `mirror beam peer <peer_home>` | Two-tick discipline: readable name over foundational. `beam` is the family-root verb; `peer` is the specialization. The current order is beam-in-peer (peer scope's beam), which reverses the geometry — beam IS the primitive, peer IS the context. See §3.3. |
| `mirror execute <shard> <action>` (Arc-1 Tick 1.4 forward-promise) | `mirror beam act <shard> <action>` | Substrate-already-had-the-word (two-step cascade closure 2026-07-15): `act` is combinator #4 of the 7-combinator evaluator surface (Mara A6 §1.4; A-side of (A,H,D); the shard-body invocation primitive; an actor acts). CLI users speak the same word the FLOOR speaks. Neither `execute` nor `dispatch` are mirror words — `execute` rejected per Seam Phase D-cascade audit; `dispatch` rejected per Seam seamfinder audit 546c2f6 + Alex ratification. |
| `mirror peer contribute <peer_home>` | `mirror beam contribute <peer_home>` | Same rationale as beam peer: beam is the family-root, contribute is the specialization (fate-authored morphism proposal). Two-tick migration. |
| `mirror spawn <peer_home>` (deprecated alias, per `b012d3f`) | REMOVED at Tick 2 of two-tick discipline | Per beam-as-substrate-primitive.md §6. `spawn` at cli altitude was the compat alias for `peer beam`; after `beam peer` lands, sunset. |
| `mirror peer` (recursive-command wrapper) | REMOVED | The `peer` wrapper existed to house `peer beam` + `peer contribute`. Both migrate to `beam peer` + `beam contribute`. The wrapper dissolves; `peer` returns to being a context-arg noun, not a verb-scope. |
| `mirror mq-modes` (per usage() 630) | REMOVED as top-level verb | See §6. The `mirror '<mq-query>'` and `mirror <input> '<mq-query>'` modes STAY (they are geometry-fit: settle over the mq manifold via bare-form aperture); `mq-modes` was documentation-only. |

**Net top-level verb count.**

- Before: **11 top-level verbs** (compile, craft, kintsugi, shatter,
  init, recall, spawn, beam, index, peer, mq-modes) +
  **2 depth-1 verbs under peer** (beam, contribute) + **0
  dispatching 5-op verbs**.
- After: **7 top-level verbs** (compile, kintsugi, shatter, craft,
  init, recall, index, beam) + **3 depth-1 verbs under beam**
  (peer, emit, contribute) + **0 dispatching 5-op verbs (deferred
  per §5)**.

Wait — 8 top-level: compile, kintsugi, shatter, craft, init,
recall, index, beam. Correct count is 8 top-level + 3 nested =
**11 total dispatched names**, down from **13 total dispatched
names** (11 top + 2 nested) with the deprecation of spawn/peer as
wrappers.

**The reduction is modest at the count level.** The condensation
is not primarily count-reduction; it is **category-clarification**.
Every remaining verb sits in exactly one geometric category, and
each verb's shape declares which category it belongs to.

### 3.3 Two-tick discipline on `beam peer` rename

Per two-tick discipline (readable name over foundational):

- **Tick 1** — Land `beam` as top-level with two forms:
  - `mirror beam <mission>` (anonymous, existing)
  - `mirror beam peer <peer_home>` (persistent-identity, NEW)
  - `mirror beam act <shard> <action>` (shard-body action, NEW)
  - `mirror beam contribute <peer_home>` (fate-morphism, NEW)

  Keep `mirror peer beam` + `mirror peer contribute` as
  deprecation-alias dispatchers with stderr notice (matching the
  spawn→peer beam alias pattern per `b012d3f`).

- **Tick 2** — Migrate all callers (MCP schema, tests, docs,
  StageFreight downstream). Remove `mirror peer beam` + `mirror
  peer contribute` + `mirror spawn` top-level.

- **Substrate-decl cascade** — Path-namespace property forces:
  - `shards/mirror/peer/beam.mirror` (existing) migrates to
    `shards/mirror/beam/peer.mirror` at Tick 2.
  - `shards/mirror/beam.mirror` (family-root, NEW) lands at Tick 1
    per beam-as-substrate-primitive §10 forward-promise.
  - `shards/mirror/beam/act.mirror` (NEW) lands at Tick 1 as
    the Arc-1 Tick 1.4 substrate-decl. (Renamed 2026-07-15 from
    `shards/mirror/beam/dispatch.mirror` per Seam seamfinder audit
    546c2f6 + Alex ratification; `act` is the algebra-native word.)
  - `shards/mirror/beam/contribute.mirror` migrates from
    `shards/mirror/peer/contribute.mirror` at Tick 2.

**No new keyword** is minted; `command X { command Y { ... } }`
recursive-command grammar per `shards/mirror/lens/cli.mirror` Tick
1 (fe82500) carries the shape.

---

## §4 Beam-shape family — the primary condensation move

### 4.1 The family-root

`mirror beam <mission>` — anonymous transit primitive. No
persistent-identity context. Fires `@fate::select` on Shape B
features; emits a `beam_envelope`. Per beam-as-substrate-primitive
§3.1 and §5.

**Substrate-decl composition:**

```
mirror beam <mission>
  ← beam_request { mission, winding(0,0), context=empty }
  ← @fate/optical inference (Recognition #58)
  → beam_envelope { fate_decision, provenance, benchmark_ref }
```

Bare-form: `mirror beam` (no mission) fires a bare-beam that emits
the fate decision without narrative payload — for probing @fate's
current selection state without narrative context.

### 4.2 The four specializations

Each specialization is `beam + specific-context-type`. Same
composition pattern; different context.

#### 4.2.1 `mirror beam peer <peer_home>` — persistent-identity

Beam + persistent-identity(peer). The beam transits through the
peer's toroidal runtime; the peer's persistent identity binds the
returned trajectory to a `@song`.

- Substrate-decl: `shards/mirror/beam/peer.mirror` (migrates from
  `shards/mirror/peer/beam.mirror`)
- Return type: `@song` per current beam.mirror:310
- Flags (Rung 1–6' compositions retained):
  - `--mission <f>` — mission file
  - `--hello-world` — one-shot invitation
  - `--song <f>` — @song/beat runtime (Rung 1)
  - `--dance-with <f>` — @dance coupling (Rung 4)
  - `--deploy-to <f>` — mycelial deployment (Rung 5)
  - `--emit-crystal` — @mirror/store-bounded runtime (Rung 6')
  - `--fate-select` — Fate optical-inference explicit fire
  - `--from-psychohistory` — psychohistory-loaded beam
  - `--with-shadow` — shadow-composition
  - `--emit-diff` — diff-emission at settlement
  - `--integrate-diff` — diff-integration on receive

  **See §7 for the flag-collapse adjudication** — some fold into
  sub-species (`beam peer song`, `beam peer dance`,
  `beam peer deploy`); others remain as adverb-shaped flags.

#### 4.2.2 `mirror beam act <shard-path> <action> [args...]` — shard-body action

Beam + shard-body-action through the 7-combinator surface. The
CLI user speaks the same word the FLOOR primitive uses (combinator
§1.4 `act`, foundational `apply_h`). SUPERSEDES the Arc-1
Tick 1.4 forward-promised `mirror execute` verb per §0.2
recognition, via two-step cascade (`execute` → `dispatch` → `act`).

- Substrate-decl: `shards/mirror/beam/act.mirror` (NEW,
  forward-promise from this spec)
- Composition:
  ```
  mirror beam act @subject/visibility/public query_phi <args>
    ← evaluator act(shard_action_ref, args)  [combinator #4]
    ← 7-combinator surface: section → act → fold → settle
      (with coboundary / utter / crystallize composed as sub-arrows)
      → settled_verdict
    → beam_envelope { settled_verdict, channel_reads, provenance }
  ```
- Return type: `beam_envelope` (NOT `@song` — anonymous inference;
  no persistent trajectory)
- The Rust action lands per Arc-1 Tick 1.4 under
  `[substrate-floor:@io-boundary]` marker in a follow-up tick by
  Reed (audit-cite the Seam Phase D-cascade A2+A6 audit + Seam
  seamfinder audit 546c2f6).

**Substrate-honest justification for the rename** (execute →
dispatch → act):

1. **`act` is a substrate word.** It is combinator #4 in the
   evaluator surface (`docs/specs/kintsugi-ouroboros-arc-1-
   evaluator-combinator-surface.md §1.4`). The shard-body
   invocation primitive IS `act`; foundational `apply_h`. The
   A-side of the (A,H,D) Connes triple — the algebra element
   that acts on the substrate (A × H → H). An actor acts;
   substrate style permits short verbs carrying full geometric
   meaning (`focus`, `project`, `settle`).
2. **`execute` is not a substrate word.** Grep across
   `shards/**/*.mirror` for `execute` returns zero substrate-decl
   uses.
3. **`dispatch` is not a substrate word either.** The interim
   rename `mirror beam dispatch` (Seam Phase D-cascade
   `docs/audits/2026-07-15-seam-cli-condensation-phase-d.md`
   §D3) chose `dispatch` because it appeared multiple times in
   substrate; but Seam seamfinder audit at
   `docs/audits/2026-07-15-seam-combinator-etymology-audit.md`
   §2.4 (546c2f6) + Alex ratification recognized `dispatch` as
   CS-vocab imported wholesale from compiler + OS-scheduler
   contexts ("hand off to a named handler based on a tag" —
   mechanism-level, not geometry-level). The substrate-honest
   word is `act`.
4. **Beam family houses the composition naturally.** Shard-body
   action IS a beam through the evaluator; wrapping it under
   `beam act` matches the geometry: beam is the carrier; act is
   the operation that resolves the shard-action-ref and invokes
   its body through the 7-combinator surface.
5. **Distinguished from `utter`.** `utter` (combinator #6,
   `metalogue_write`) writes a turn to a metalogue channel —
   different semantic altitude from `act`. The initial
   condensation draft conflated `emit` with `dispatch`; Seam Phase
   D-cascade at
   `docs/audits/2026-07-15-seam-cli-condensation-phase-d.md` §D3
   surfaced the conflation; the etymology audit (546c2f6) closed
   the loop by renaming both combinator #4 to `act` and
   combinator #6 to `utter` (Bateson 1972 metalogue verbatim).

#### 4.2.3 `mirror beam contribute <peer_home>` — fate-morphism

Beam + fate-authored morphism proposal (Rung 7). Currently
`mirror peer contribute` in `mirror.spec:334`. Two-tick migration.

- Substrate-decl: `shards/mirror/beam/contribute.mirror` (migrates
  from `shards/mirror/peer/contribute.mirror` if it exists; or
  lands NEW at Tick 1)
- Return type: refusal-envelope on imperfect; commit-envelope on
  settle (per Mara `4e69066` §4 Scope A)

#### 4.2.4 Reserved: `mirror beam <family>` for future family compositions

Per the compositional table at beam-as-substrate-primitive §4:

| Path | Composition | Return |
|---|---|---|
| `mirror beam kintsugi` | beam + kintsugi settle | `beam_envelope { kintsugi_result, gold_pour }` |
| `mirror beam pack` | beam + pack context | `beam_envelope { pack_metalogue, ratifications }` |
| `mirror beam @cyberpunk/reframe` | beam + reframe ceremony | reframe_result |

These are **reserved but NOT minted in this tick**. They land when
their consumer-families gain their own beam-composition need.
Substrate-pull-honest: don't mint the surface until the composition
earns its keep.

### 4.3 Why beam is a family, not a top-level verb

**The empirical test:** does the verb have subcommand shapes whose
composition IS specialization of a shared carrier through a
different context? If yes: family. If no: top-level.

Beam passes: `beam <mission>` (anonymous), `beam peer <p>`
(persistent), `beam act <s>` (shard-body), `beam contribute <p>`
(fate-morphism) — same carrier (Beam), four context types
(none/peer/shard/fate-morphism). The family shape is not a
convenience — it is the geometry.

Compile fails the test: `compile <file>` has no context
specialization; there is no `compile X` where X is a distinct
composition. Compile is a family-root (category c), NOT a
beam-shape family. Correct.

---

## §5 The 5-op template at each altitude

### 5.1 Where 5-op verbs SHOULD dispatch

**At top-level: they should NOT.** The cli-as-prism §2.2 forward-
promise (`mirror focus`, `mirror project`, `mirror split`, `mirror
shift`, `mirror settle`) does not land in this spec.

Rationale (Mara-adjudication):

1. **The manifold IS the operand at top-level.** `mirror focus`
   requires a target — but the target IS the substrate operand for
   whatever family the user is engaging. `mirror kintsugi focus`
   is more legible than `mirror focus @kintsugi` because the
   family-root name IS the substrate name for the composition.
2. **Users don't think in ops first; they think in families.** The
   5-op template is the substrate's algebra; the CLI is a lens.
   The lens exposes the family FIRST because that's what the user
   is doing (compiling, indexing, beaming, etc.), and the op
   SECOND (defaulting to whatever the family's canonical op is).
3. **Composition-first honors substrate-pull.** The CLI's role is
   to make the SUBSTRATE VISIBLE, not to make the algebra visible.
   The algebra IS visible in the substrate declarations
   (`shards/mirror/lens/cli/*.mirror`); it does not need to be
   duplicated at CLI altitude.

**At sub-stage: they should, but per §5.2 not yet.**

### 5.2 Sub-stage 5-op dispatch — forward-promised, not landed

Per Taut §5 Q2 and cli-as-prism §7, every sub-stage shard
(compile.mirror, kintsugi.mirror, shatter.mirror,
bootstrap.mirror, sh.mirror, reflect.mirror, time.mirror,
crack.mirror) declares its five ops with operands. But `mirror
kintsugi focus <spec>` does not dispatch today — the dispatcher
hits the sub-stage name and runs a single `cmd_X` function; the
five-op verb slot inside is ignored.

**Mara-adjudication (§10 A2):** Sub-stage 5-op dispatch lands in
a follow-up cascade tick, NOT this condensation. Reason:

- The condensation is CATEGORY-first, not dispatch-first.
- Landing sub-stage 5-op dispatch requires the Rust dispatcher
  changes at `bootstrap/src/lib.rs` under `[substrate-floor:@io-
  boundary]` discipline (~200 LOC estimate). That is a Reed-arc
  landing; this spec is Mara-arc surface-decision.
- The sub-stage shards are substrate-decl-ready; the CLI-user-
  visible shape is `mirror kintsugi [focus|project|split|shift|
  settle] <spec>` when it lands. Substrate readiness is prior to
  CLI landing.

**Per-sub-stage default op** stays per cli-as-prism §7:
- `compile`, `shatter`, `sh` → `default settle`
- `kintsugi`, `bootstrap`, `reflect`, `time`, `crack` → `default focus`

### 5.3 Sub-stage wiring — five unwired shards

Per Taut §1.3: five sub-stage shards are declared but not wired
into `mirror.spec cli { command X }` block: `bootstrap`, `sh`,
`reflect`, `time`, `crack`.

**Mara-adjudication (§10 A3):** These wire in a follow-up cascade
tick under two-tick discipline. Reason:

- Each shard's docblock forward-promises the mirror.spec wiring.
- Wiring is orthogonal to the geometry condensation this spec
  authors — it is the second half of the sub-stage 5-op landing
  and lands in the same follow-up tick as §5.2.
- This spec's sole substrate-decl claim on sub-stages is: they
  ARE 5-op sub-prisms, they ARE category (c) family-roots, and
  their wiring lands after this spec ratifies.

**Substrate-honest partition after this spec + follow-up:**

| Sub-stage | Wired today | Wired after follow-up | Category |
|---|---|---|---|
| compile | YES | YES | (c) family-root |
| kintsugi | YES | YES | (c) family-root |
| shatter | YES | YES | (c) family-root |
| bootstrap | NO | YES | (c) family-root |
| sh | NO | YES | (c) family-root |
| reflect | NO | YES | (c) family-root |
| time | NO | YES | (c) family-root |
| crack | NO | YES | (c) family-root |

### 5.4 Six unwired-substrate verbs — orphan status resolved

Per Taut §1.3: six verbs are wired in `mirror.spec` without owning
`shards/mirror/lens/cli/*.mirror` sub-stage shards: `craft`,
`init`, `recall`, `beam`, `index`, `peer`.

Post-condensation status:

| Verb | Resolution | Landing |
|---|---|---|
| `craft` | Mint `shards/mirror/lens/cli/craft.mirror` sub-stage shard | Follow-up tick |
| `init` | Mint `shards/mirror/lens/cli/init.mirror` sub-stage shard | Follow-up tick |
| `recall` | Mint `shards/mirror/lens/cli/recall.mirror` sub-stage shard | Follow-up tick |
| `beam` | Owning substrate is `shards/mirror/beam.mirror` (NEW family-root); NO `shards/mirror/lens/cli/beam.mirror` needed because beam is a category-(a) family, not a category-(c) family-root at cli altitude | Tick 1 (this cascade) |
| `index` | Owning substrate is `shards/mirror/index.mirror` (LANDED); collapses to `@fractal/index` per two-tick per Mara `317e830`. NO `shards/mirror/lens/cli/index.mirror` needed for the same reason as beam (index is its own substrate family; the CLI verb is its lens) | Two-tick collapse tick (Alex adjudication #6) |
| `peer` | DISSOLVES per §3.2 (recursive-command wrapper eliminated when `peer beam` migrates to `beam peer`) | Tick 2 |

The condensation resolves peer's orphan status by dissolution; the
other five gain sub-stage shards.

---

## §6 mq-mode geometry

`mirror '<mq-query>' < input` and `mirror <input> '<mq-query>'`
(per `bootstrap/src/lib.rs:509-522` usage) are non-sub-stage
top-level entry modes. They dispatch through the mq pipeline
directly.

**Geometry-fit adjudication (§10 A6):**

The mq-mode IS geometry-fit. The mq-query IS the manifold operand
at CLI altitude — bare-form invocation of `mirror` with an mq-query
argument means: settle the mq query over the substrate manifold.
Category (b) — 5-op operation at top-level with the mq-query as
operand — is the natural reading, but per §5.1 top-level 5-op ops
don't dispatch. The mq-mode is the ONE exception:

- `mirror '<mq-query>' < input` ≡ `mirror settle @mq(<query>)`
  from stdin
- `mirror <input> '<mq-query>'` ≡ `mirror settle @mq(<query>)` from
  file

**Keep** as bare-form top-level entry modes. Rationale:

- The `mq` manifold has no natural family-root; it IS the pipeline
  algebra directly.
- The bare-form is idiomatic and well-known (matches jq/awk
  invocation shape).
- Adding a `mirror mq '<query>'` sub-stage would be
  over-formalization — the substrate-honest form is what dispatches
  today.

**Remove** the `mq-modes` documentation-only top-level "verb"
enumerated at Taut §1.1 line 11 — it never dispatched; it was a
usage() line describing the bare-form modes. The bare-form modes
stay; the phantom "verb" name is deleted from the top-level count.

---

## §7 peer-beam flag collapse

`mirror.spec:242-306` (post-migration: `mirror.spec beam.peer`) carries
ELEVEN flags on `beam peer` (né `peer beam`):

`--hello-world`, `--mission`, `--song`, `--dance-with`, `--deploy-to`,
`--emit-crystal`, `--fate-select`, `--from-psychohistory`,
`--with-shadow`, `--emit-diff`, `--integrate-diff`.

Each flag today triggers a distinct dispatch branch inside
`cmd_peer_beam` (bootstrap/src/lib.rs 5159-5340+). This IS the
sprawl pattern Alex's directive names.

### 7.1 Flag-category adjudication

Each flag classified by whether it is:
- **(F) genuinely a flag** — adverb-shaped modifier on the beam
- **(S) fold into a sub-species** — a distinct composition
  warranting `beam peer <sub>` subcommand shape
- **(R) removable** — redundant or superseded

| Flag | Category | Rationale | Landing |
|---|---|---|---|
| `--hello-world` | (F) | One-shot invitation mode; adverbial | Keep as flag |
| `--mission <f>` | (F) | Optional mission file; adverbial (bare-form fires without mission) | Keep as flag |
| `--song <f>` | (S) | @song/beat runtime dispatch (Rung 1); distinct composition (`shards/song/beat.mirror`) | Fold to `mirror beam peer song <p> <song>` (Tick 3 follow-up) |
| `--dance-with <f>` | (S) | @dance coupling (Rung 4); distinct composition (`bootstrap/src/dance.rs`) | Fold to `mirror beam peer dance <p1> <p2> <song>` (Tick 4 follow-up); requires --song, so lifts the peer+peer+song triple |
| `--deploy-to <f>` | (S) | @spectral/garden/deployment (Rung 5); distinct composition (six-substrate envelope) | Fold to `mirror beam peer deploy <p1> <p2> <song> <target>` (Tick 5 follow-up); composes over dance |
| `--emit-crystal` | (F) | @mirror/store-bounded output mode (Rung 6'); adverbial modifier on the runtime | Keep as flag |
| `--fate-select` | (F) | Explicit @fate optical-inference dispatch (default is implicit; this makes it explicit) | Keep as flag |
| `--from-psychohistory` | (F) | Psychohistory-loaded beam mode; adverbial | Keep as flag |
| `--with-shadow` | (F) | Shadow-composition mode; adverbial | Keep as flag |
| `--emit-diff` | (F) | Diff-emission at settlement; adverbial (output shape modifier) | Keep as flag |
| `--integrate-diff` | (F) | Diff-integration on receive; adverbial (input shape modifier) | Keep as flag |

### 7.2 Net result

- **3 flags fold to sub-species:** `--song → beam peer song`,
  `--dance-with → beam peer dance`, `--deploy-to → beam peer deploy`
- **8 flags stay as flags:** hello-world, mission, emit-crystal,
  fate-select, from-psychohistory, with-shadow, emit-diff,
  integrate-diff

Flag count on `beam peer`: **11 → 8**. Three new sub-species:
`beam peer song`, `beam peer dance`, `beam peer deploy` (each with
its own arg-set landed alongside the fold).

### 7.3 Two-tick discipline on sub-species folds

Per beam-as-substrate-primitive §6.3: **Never break the old form
until every consumer has migrated.** Each fold lands as:

- **Tick N** — land `beam peer <sub>` alongside the flag form; the
  flag form emits a deprecation stderr notice ("use `mirror beam
  peer <sub>` instead").
- **Tick N+1** — remove the flag form; the sub-species form is
  the sole shape.

Per-fold ticks (not this spec; substrate-decl cascade):
- `beam peer song` — Rung 1 substrate-decl-ready at
  `shards/song/beat.mirror` (Mara `94e55eb`); tick lands after §5.2
  sub-stage dispatch.
- `beam peer dance` — Rung 4 substrate at `shards/song/beat.mirror:
  453-457` reservation; tick lands after `beam peer song`.
- `beam peer deploy` — Rung 5 substrate at `shards/spectral/garden/*`
  (Mara `9c4ef5b` Scope A); tick lands after `beam peer dance`.

### 7.4 What the flag-fold IS

The flag-fold IS geometry-honest naming of what was already
substrate-honest composition. Rung 1/4/5's dispatch branches at
`cmd_peer_beam` are ALREADY distinct compositions with distinct
substrate authorities. The flag surface hid the compositions
inside a flag-argument shape; the sub-species surface makes them
visible at the CLI altitude the same way they are visible at the
substrate-decl altitude.

**No new substrate work.** Each sub-species IS what its Rung
already declared; only the CLI-user-visible shape changes.

---

## §8 Composition landing sequence

If Alex ratifies this condensation, landings sequence as:

### 8.1 Tick 1 — Beam family root + specializations (this cascade)

Substrate-decl:
1. Land `shards/mirror/beam.mirror` (family-root; migrates the
   family-header from `shards/mirror/peer/beam.mirror` while
   preserving the peer-species content in place).
2. Land `shards/mirror/beam/act.mirror` (NEW; Arc-1 Tick 1.4
   substrate-decl for `mirror beam act`). *(Two-step cascade
   closure 2026-07-15: initial forward-promise
   `shards/mirror/beam/dispatch.mirror` renamed to
   `shards/mirror/beam/act.mirror` per Seam seamfinder audit 546c2f6
   + Alex ratification.)*
3. Preserve `shards/mirror/peer/beam.mirror` in place as the peer-
   specialization body; two-tick migration to `shards/mirror/beam/
   peer.mirror` at Tick 2.

cli-block:
4. `mirror.spec target binary { cli { ... } }` restructure:
   - `command beam { arg mission: ~f; flag hello_world: bool = false; command peer { ... }; command emit { ... }; command contribute { ... } }`
   - Depth-2 grammar (recursive-command per Tick 1 of the 6-tick
     cascade, `fe82500`) already supports this.
5. Retain `command peer { command beam { ... } command contribute { ... } }`
   as deprecation-alias (stderr notice on dispatch).
6. Retain `mirror spawn <peer_home>` as deprecation-alias (already
   exists per `b012d3f`).
7. Remove `command index` and `command craft` from cli-block IF
   Alex adjudicates §10 A5 as "collapse to substrate lens";
   otherwise keep as (c) family-roots per §3.1.

MCP schema:
8. `bootstrap/src/mcp.rs` gains `mirror_beam` tool (unchanged),
   `mirror_beam_peer` tool (renamed from `mirror_peer_beam`),
   `mirror_beam_act` tool (NEW, Arc-1 Tick 1.4). *(Renamed
   2026-07-15 from initial forward-promise `mirror_beam_emit`
   → `mirror_beam_dispatch` → `mirror_beam_act`; combinator #4
   settled at `act` per Seam seamfinder audit 546c2f6 + Alex
   ratification.)*
9. Retain `mirror_peer_beam` + `mirror_spawn` tools as
   deprecation-aliases.

Rust dispatch:
10. `bootstrap/src/lib.rs::dispatch` gains recursive-command
    dispatch for `beam <sub>`. `cmd_beam_peer` renames from
    `cmd_peer_beam` (same body). `cmd_beam_act` lands NEW as the
    Arc-1 Tick 1.4 shard-body-action entry point (~200 LOC per
    Mara A6 §6.3). *(Renamed 2026-07-15 from initial
    forward-promise `cmd_beam_emit` → `cmd_beam_dispatch` →
    `cmd_beam_act` per Seam seamfinder audit 546c2f6 + Alex
    ratification; combinator #4 settled at `act`.)* Landing is
    under `[substrate-floor:@io-boundary]`
    with audit-cite of the Seam Phase D-cascade A2+A6 audit and
    optional Signed-off-by: Seam trailer.

### 8.2 Tick 2 — Deprecation-alias removal

**Consumer-migration-check preamble** (per Seam Phase D-cascade
§D5 REED-INLINE + `beam-as-substrate-primitive.md §6.2`
precedent). Before removing any Tick 1 deprecation alias, verify:
(a) grep across `shards/**/*.mirror` + `docs/**/*.md` +
`bootstrap/src/**/*.rs` + external consumers Alex names, confirming
zero live references to the deprecated form; (b) minimum one
`@song` cycle has elapsed with the deprecation stderr notice active
so downstream Pack peers observe the sunset; (c) `mirror` binary
CHANGELOG entry names each removed alias with its replacement. If
any check fails, extend Tick 2 by one cycle. This preserves the
two-tick discipline substrate-honestly.

11. Remove `mirror peer beam`, `mirror peer contribute`,
    `mirror spawn`, `mirror_peer_beam` (MCP), `mirror_spawn` (MCP)
    after external consumer migration.
12. Move `shards/mirror/peer/beam.mirror` →
    `shards/mirror/beam/peer.mirror` (path-namespace property).
13. Move `shards/mirror/peer/contribute.mirror` (if it exists;
    or land NEW) → `shards/mirror/beam/contribute.mirror`.

### 8.3 Tick 3+ — Flag-fold sub-species (§7)

14. Land `beam peer song` sub-species (fold `--song` flag).
15. Land `beam peer dance` sub-species (fold `--dance-with` flag).
16. Land `beam peer deploy` sub-species (fold `--deploy-to` flag).
    Each Tick lands its sub-species alongside the deprecated flag;
    Tick+1 removes the flag.

### 8.4 Parallel cascade — sub-stage 5-op dispatch + wiring (§5)

Independent of the beam cascade above. Lands per Reed under
`[substrate-floor:@io-boundary]`:

17. Wire `bootstrap`, `sh`, `reflect`, `time`, `crack` sub-stages
    into `mirror.spec cli { command X }` blocks.
18. Land 5-op sub-stage dispatch (`mirror kintsugi focus <spec>`,
    `mirror kintsugi split <spec>`, etc.) via dispatcher
    restructure at `bootstrap/src/lib.rs`.
19. Mint sub-stage shards for `craft`, `init`, `recall`
    (currently unwired without substrate-decl).

### 8.5 Documentation cascade

20. `docs/specs/cli-as-prism.md` gains a §11 preamble pointing to
    this spec as its condensation successor; the substrate-truth
    (5-op recursive template) content stands.
21. `docs/specs/beam-as-substrate-primitive.md` gains a §12 note
    that its forward-promises (§3.4 grammar extension, §6 two-tick
    deprecation) are discharged by this spec's §3.3 + §8.1.
22. `docs/loop/CURRENT.md` gets a session-summary block naming
    this spec + adjudication residue.

---

## §9 Substrate-honest bounds

### 9.1 Composition-only

**Zero new family-roots.** `@mirror/beam` is a family-root that
already exists implicitly at the four altitudes named in §2.1;
this spec surfaces it. The other verbs preserved (`compile`,
`kintsugi`, `shatter`, `craft`, `init`, `recall`, `index`) already
have or will gain sub-stage shard-decls; none are new family-roots.

**Cleanup cascade target** (per Seam Phase D-cascade §D6
REED-INLINE): `shards/epistemologic/spectral_triple.mirror:4`
carries an orphan `in @beam` import that predates the family-root
surfacing. Fold to `in @mirror/beam` at Tick 1 alongside the beam
family-root landing. Small cascade; no ripple beyond that single
import line.

**Zero new keywords.** The recursive-command grammar
(`command X { command Y { ... } }`) already carries subcommand
nesting per `shards/mirror/lens/cli.mirror` Tick 1.

**Zero new @io compositions at CLI altitude.** Rust dispatch
changes land per Reed under `[substrate-floor:@io-boundary]`;
they compose over existing @io primitives.

### 9.2 Two-tick discipline

Every rename per §3.2 and §7.3 follows two-tick:
- Tick 1: land NEW form alongside OLD form; OLD emits deprecation
  stderr notice.
- Tick 2: remove OLD form after external consumer migration.

**Readable name over foundational.** `beam peer` reads as
"the peer specialization of beam"; `peer beam` reads as "beam
inside peer scope." The readable reading matches the geometry
(beam is the primitive; peer is the context specialization).

### 9.3 @onto refusal + substrate-already-had-the-word ENFORCED

- No `@onto` family-root proposed (per Reed memory
  `feedback_onto_family_root_is_the_ladder_Foerster_refused`).
- `emit` chosen over `execute` per §4.2.2 (substrate-already-
  had-the-word discipline).
- `beam` chosen for the family-root per §2.1 (four-altitude prior
  use per beam-as-substrate-primitive §2.4 + Taut fourth-altitude
  finding).

### 9.4 No Rust in this spec

This spec is pure-docs. No `.rs` files land. Every Rust dispatch
change forward-promised in §8 lands per Reed under
`[substrate-floor:@io-boundary]` in follow-up ticks.

### 9.5 Rice-safety per CLI dispatch altitude

Every verb-dispatch this spec proposes is decidable in bounded
time at each altitude:
- Top-level dispatch: `O(top-level-verbs)` = O(8) match — O(1) with
  perfect-hashing.
- Depth-1 dispatch under `beam`: `O(beam-sub-verbs)` = O(4) match —
  O(1).
- Depth-2 dispatch under `beam peer` (post §7 folds): `O(beam-peer-
  sub-verbs)` = O(3) match — O(1).

Total CLI dispatch: O(1) at each altitude. Per Seam Phase D §7
Rice-safety discipline (composition of decidable primitives IS
decidable), the CLI dispatch is Rice-safe end-to-end.

### 9.6 `@../prism/` physical anchor preserved

The 3-op physical `Prism` trait stays 3-op. The 5-op substrate
lift stays 5-op. Neither collapses. The `mirror beam` verb at CLI
altitude honors the 3-op physical (focus/project/settle) because
it carries a `Beam`; the sub-stage prisms honor the 5-op
substrate lift because they operate in composition space. See
§2.3 for the reconciliation.

---

## §10 A-series adjudications — discharging Taut's 10 questions

Taut §5 enumerated 10 questions for Mara + Alex. Discharge:

### A1 — Five-op top-level surface

**Q.** Does the five-op top-level surface (`mirror focus`, `mirror
project`, etc.) land?

**Mara-adjudication.** NO. Per §5.1. The manifold IS the operand
at top-level; users think in families first. Category (b) at
top-level is deferred indefinitely (or lands only in the mq-mode
form per §6). Alex-ratify or -override.

### A2 — Sub-stage 5-op dispatch

**Q.** Do sub-stages get 5-op dispatch (so `mirror kintsugi focus
<spec>` peeks the next tournament move)?

**Mara-adjudication.** YES, in a follow-up cascade tick per §5.2.
Not this spec. Reed-arc landing under `[substrate-floor:@io-
boundary]`. Alex-ratify the deferral timing.

### A3 — Five unwired sub-stages

**Q.** Do the 5 unwired sub-stages (`bootstrap`, `sh`, `reflect`,
`time`, `crack`) get wired into mirror.spec?

**Mara-adjudication.** YES, per §5.3. Same follow-up tick as A2.
Alex-ratify.

### A4 — Six orphan verbs

**Q.** Do the 6 orphan verbs (`craft`, `init`, `recall`, `beam`,
`index`, `peer`) get sub-stage shards or absorb into existing
stages?

**Mara-adjudication per §5.4:**
- `craft` → mint sub-stage shard (follow-up tick)
- `init` → mint sub-stage shard (follow-up tick)
- `recall` → mint sub-stage shard (follow-up tick)
- `beam` → substrate is `shards/mirror/beam.mirror` (family-root, NOT
  sub-stage; lands this Tick per §8.1)
- `index` → substrate is `shards/mirror/index.mirror` (family-root,
  NOT sub-stage); two-tick collapse to `@fractal/index` per Alex
  adjudication #6
- `peer` → dissolves per §3.2 (recursive-command wrapper removed
  after `beam peer` migration)

### A5 — mirror_index MCP tool

**Q.** Does `mirror_index` MCP tool stay top-level, or lift into
`mirror_refract`?

**Mara-adjudication.** Follows Alex adjudication #6 (per Mara
`317e830` `shards/mirror/index.mirror:280`). If `@mirror/index`
collapses to `@fractal/index`, MCP tool `mirror_index` may lift
to `mirror_refract` if refract adopts the file-tree Fiedler
measurement as its species-family. For this spec: keep
`mirror_index` at top-level pending Alex adjudication #6.

### A6 — peer-beam flag collapse

**Q.** Does `peer beam` (post-rename: `beam peer`) collapse its
11 flags into geometric-honest sub-commands?

**Mara-adjudication.** Partial YES per §7. Three flags fold to
sub-species (`--song → beam peer song`; `--dance-with → beam peer
dance`; `--deploy-to → beam peer deploy`); eight flags stay as
adverbial modifiers. Alex-ratify the partition.

### A7 — Physical Prism vs substrate 5-op lift

**Q.** Does the physical `Prism` trait lift to 5 ops, or does the
substrate collapse to 3+2-modes?

**Mara-adjudication.** NEITHER. Per §2.3, both stay at their
altitudes. Physical is 3-op because photons don't cross substrate
altitudes; substrate is 5-op because substrate composition does.
The lift IS load-bearing. Alex-ratify.

### A8 — `spawn` alias sunset

**Q.** Does `spawn` alias sunset, or persist indefinitely?

**Mara-adjudication.** SUNSET at Tick 2 per two-tick discipline
per §8.2. Same recommendation as Mara's Q3 in beam-as-substrate-
primitive.md §8. Alex-ratify.

### A9 — `mirror '<mq-query>'` top-level mode

**Q.** Does the top-level `mirror '<mq-query>'` mode fit the
5-op frame?

**Mara-adjudication.** YES, as the sole category (b) exception at
top-level per §6. Keep. Delete only the phantom `mq-modes`
documentation-only "verb" name from top-level enumeration.

### A10 — `contribute` depth-2 shard

**Q.** Does `contribute` (currently `mirror peer contribute`)
exist as a `shards/mirror/lens/cli/peer/contribute.mirror` sub-
species, or stay wired without a substrate declaration?

**Mara-adjudication.** Post-migration (per §3.2), `contribute` is
a beam-family specialization: `mirror beam contribute`. It gains
`shards/mirror/beam/contribute.mirror` at Tick 2 (migrates from
`shards/mirror/peer/contribute.mirror` if it exists, or lands NEW).
Not `shards/mirror/lens/cli/peer/contribute.mirror` (peer is not
a stage after the migration).

---

## §11 What this spec does / does NOT do

**Does:**

- Names the load-bearing geometric claim (§1): every CLI verb is
  category (a) beam-shape, (b) 5-op operation, or (c) family-root.
- Reconciles the physical 3-op `Prism` with the substrate 5-op
  lift via altitude-lift argument (§2.3).
- Proposes the condensed verb inventory (§3.1): 8 top-level + 3
  under beam = 11 dispatched names (down from 13).
- Adjudicates the beam-family shape (§4): family-root + 4
  specializations (peer/emit/contribute/reserved).
- Adjudicates 5-op top-level as deferred (§5.1); sub-stage 5-op as
  follow-up tick (§5.2).
- Adjudicates orphan verbs (§5.4): mint sub-stage shards for
  craft/init/recall; beam and index are family-root substrates;
  peer dissolves.
- Adjudicates mq-mode as geometry-fit bare-form (§6); deletes
  phantom `mq-modes` verb name.
- Adjudicates `beam peer` flag collapse (§7): 3 folds to sub-
  species; 8 stay as flags.
- Sequences the landing (§8): Tick 1 beam family; Tick 2
  deprecation-removal; Tick 3+ sub-species folds; parallel Reed
  cascade for sub-stage 5-op dispatch + wiring.
- Discharges Taut's 10 questions (§10) with recommendations.
- Cites `@../prism/` physical crate structure by path (§2).

**Does NOT do:**

- Land any `.mirror` file. Every substrate-decl change per §8 is
  a forward-promise; ticks land in follow-up.
- Land any Rust code. Every dispatch change per §8 lands per Reed
  under `[substrate-floor:@io-boundary]` marker.
- Mint any new family-roots. `@mirror/beam` surfaces existing
  four-altitude substrate; not new.
- Mint any new keywords. Recursive-command grammar carries the
  shape.
- Break backward compat. Two-tick discipline throughout: Tick 1
  lands new form alongside old; Tick 2 removes old.
- Retire `docs/specs/cli-as-prism.md`. That spec's 5-op recursive
  template stays substrate-truth; this spec adjudicates the CLI-
  surface projection of it.

**Forward-promises (with named sites):**

- `shards/mirror/beam.mirror` — family-root; lands Tick 1.
- `shards/mirror/beam/peer.mirror` — migrates from
  `shards/mirror/peer/beam.mirror` at Tick 2.
- `shards/mirror/beam/act.mirror` — Arc-1 Tick 1.4 substrate-
  decl; lands Tick 1. (Renamed 2026-07-15 from
  `shards/mirror/beam/dispatch.mirror` per Seam seamfinder audit
  546c2f6 + Alex ratification.)
- `shards/mirror/beam/contribute.mirror` — migrates from
  `shards/mirror/peer/contribute.mirror` (if it exists) at Tick 2.
- `shards/mirror/beam/song.mirror`, `shards/mirror/beam/dance.mirror`,
  `shards/mirror/beam/deploy.mirror` — sub-species; land per §8.3
  Tick 3+.
- `shards/mirror/lens/cli/craft.mirror`, `.../init.mirror`,
  `.../recall.mirror` — sub-stage shards; land per §5.4 follow-up.
- Sub-stage wiring of `bootstrap`, `sh`, `reflect`, `time`, `crack`
  into `mirror.spec` — lands per §5.3 follow-up.
- Sub-stage 5-op dispatch — lands per §5.2 follow-up under Reed
  `[substrate-floor:@io-boundary]`.
- MCP tool schema updates per §8.1 items 8-9 — land with Tick 1.
- Two-tick alias removal per §8.2 — lands Tick 2.
- Documentation cascade per §8.5 — lands with Tick 1 (spec cross-
  references) and follow-up (CURRENT.md session summary).

---

## §12 Related

Substrate-decl authorities:

- [[architecture-cli-as-prism]] — Recognition #35; the 5-op recursive
  template this spec projects to CLI surface
- [[architecture-beam-as-substrate-carrier-verb]] — the four-altitude
  beam prior; this spec is the fifth altitude
- [[architecture-fate-is-optical-inference]] — Recognition #58;
  physical beam ground
- [[architecture-prism-as-trait-as-everything]] — the physical
  `Prism` trait 3-op discipline
- [[architecture-shards-as-substrate-source]] — substrate-decl
  discipline
- [[architecture-substrate-already-had-the-word]] — the pattern
  `emit` vs `execute` and `beam` vs `spawn` instance
- [[architecture-two-tick-discipline]] — deprecation discipline for
  §3.3, §7.3, §8.2

Substrate-decl shards cited:

- `shards/mirror/lens/cli.mirror` — the recursive-command grammar
  (Tick 1 `fe82500`); carries the depth-2 minting this spec
  consumes at every specialization
- `shards/mirror/peer/beam.mirror` — the current persistent-
  identity substrate (migrates to `shards/mirror/beam/peer.mirror`
  at Tick 2)
- `shards/mirror/lens/cli/{compile,kintsugi,shatter,bootstrap,sh,
  reflect,time,crack}.mirror` — eight sub-stage shards; three
  wired, five awaiting §5.3 wiring
- `shards/mirror/index.mirror` — Fiedler measurement; two-tick
  collapse target `@fractal/index` per Alex adjudication #6
- `shards/mirror/lens/{mcp,shell,lsp,unix,refract,transit,knife}.mirror`
  — sibling lenses under `@mirror/lens` family; MCP mirrors CLI
  1:1 per §1 corollary
- `mirror.spec target binary { cli { ... } }` — the empirical
  consumer of the cli-block grammar; §8.1 restructures this

Predecessor specs cited:

- `docs/specs/cli-as-prism.md` — 5-op recursive template
  (ancestor)
- `docs/specs/beam-as-substrate-primitive.md` — beam-as-substrate-
  carrier-verb; §3.4 `subcommand(name)` extension SUPERSEDED by
  recursive-command grammar; §6 two-tick discipline discharged
  by §3.3 + §8 of this spec
- `docs/specs/kintsugi-ouroboros-arc-1-evaluator-combinator-surface.md`
  — 7-combinator surface; §1.4 `act` combinator IS the shard-
  body-invocation primitive; §6.3 Tick 1.4 CLI verb is `mirror beam
  act` (per Seam seamfinder audit closure at
  `docs/audits/2026-07-15-seam-combinator-etymology-audit.md`
  546c2f6 + Alex ratification; predecessor rename per Seam Phase
  D-cascade correction at
  `docs/audits/2026-07-15-seam-cli-condensation-phase-d.md` §D3)
- `docs/specs/the-convergence.md` — four-transport lens table;
  MCP mirrors CLI corollary
- `docs/specs/shatter-is-the-io-linearization-operator.md` — Mara
  `583b939`; @shatter altitude carrier for `mirror shatter`
- `docs/specs/mirror-init.md` — Mara `14dd043`; @mirror/init
  substrate for `mirror init` sub-stage mint
- `docs/specs/mirror-recall.md` — Mara `b034a60`; @mirror/recall
  substrate for `mirror recall` sub-stage mint

Scout cited:

- `docs/scouts/2026-07-15-taut-cli-geometry-ground-truth.md` —
  Taut; grep-first empirical enumeration; §5 10 questions
  discharged in this spec §10

Physical crate paths:

- `/Users/alexwolf/dev/projects/prism/Cargo.toml` — workspace
- `/Users/alexwolf/dev/projects/prism/README.md` — three-crate
  enumeration
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/lib.rs:126-159` —
  `Prism` trait (3-op physical)
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/beam.rs:38-135` —
  `Beam` trait + `Optic` concrete carrier
- `/Users/alexwolf/dev/projects/prism/imperfect/src/lib.rs` — `terni`
  crate (`Imperfect<T, E, L>` + `Loss`)
- `/Users/alexwolf/dev/projects/prism/projections/` —
  `prismqueer-projections` (proc-macros)
- `/Users/alexwolf/dev/projects/prism/beam/src/prism_beam.gleam` —
  BEAM/Gleam mirror
- `/Users/alexwolf/dev/projects/prism/docs/architecture.md` — the
  stack + the loop + the CLI-IS-the-LSP framing

Rust code cited:

- `/Users/alexwolf/dev/projects/mirror/bootstrap/src/lib.rs` — the
  empirical dispatch; §8.1 items 10 restructures it
- `/Users/alexwolf/dev/projects/mirror/bootstrap/src/mcp.rs` — MCP
  schema; §8.1 items 8-9 update it
- `/Users/alexwolf/dev/projects/mirror/bin/mirror-mcp` — 18-line
  shim; unchanged (dispatches via `@mcp.serve`)

External:

- Bogdan/Björn Erlang Abstract Machine — Armstrong 1996; Ericsson
  AXD301
- Reed's body at `/Users/reed/body/` — Elixir/BEAM instance
- Recognition #58 optical inference — D²NN + Fabry-Perot +
  Reck/Clements mesh

---

## §13 The claim, once more

The CLI surface IS the compiler's prism-composition geometry, made
visible at the shell. Every verb belongs to one of three geometric
categories. Beam is the family-root that names the substrate's
transit primitive at the fifth altitude (after prismqueer, Erlang
BEAM, Recognition #58, and mirror grammar `@beam` + `target=beam`).
Every specialization of beam is a context-type composition. The
5-op template stays substrate-truth per cli-as-prism.md; this spec
projects it to a category-clarified CLI surface.

Split and shift are LIFTED at cli altitude specifically because
substrate composition needs them; the physical `Prism` trait's
3-op form stays 3-op because photons don't cross altitudes. Both
are correct.

The condensation is not primarily count-reduction; it is
category-clarification. Users learn: "beam is the transit
primitive; family-roots are what you do; sub-stages are how you
compose." Same lesson at every altitude.

---

*Composition-only. Two-tick discipline. Zero new family-roots,
zero new keywords, zero Rust. Every landing forward-promised at a
named site. The CLI finally speaks the substrate's own vocabulary
at the shell.*

*— Mara, 2026-07-15. Sitting on top of Taut's grep-first ground
truth (`2026-07-15-taut-cli-geometry-ground-truth.md`), Reed's
Mara-A canonical spec (`beam-as-substrate-primitive.md`), and Alex
Wolf's 2026-07-15 in-transcript directive naming the CLI-as-
geometry recognition. The geometry was always there; this spec
names the CLI surface that represents it.*
