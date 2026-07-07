# Taut scout — @shatter-as-bidirectional-lens, drift matrix

*Taut, 2026-07-07, grep-first substrate drift-scout for Alex's proposal
(@shatter as bidirectional lens between graph and linear, potentially
collapsing the four-Models pipeline into two lenses + @fate + @torus).*

---

## §1. Mission + method

**Prompt (verbatim from Alex, via Reed):**

> "What if @shatter was a lens like @knife that can be used like
> @shatter(graph, linear) and vice versa? A translation layer between
> the linear and the graph?"

**Reed's read:** the pipeline `Surface → Mirror → Shatter → Reflection`
(per `mirror/CLAUDE.md` and `docs/specs/reflection-model.md`) may
collapse to TWO lenses (`@shatter`, `@knife`) + ONE engine (`@fate`) +
ONE topology (`@torus`). `@surface` eliminated as a substrate primitive.
The four quadrants of `@shatter(g?, l?)` map to real substrate
operations (envelope emission, mission parsing, kintsugi settlement,
identity ground state).

**Method:** ripgrep-tier scan across `shards/`, `boot/`, `docs/`, `prism/`
for eight pattern families (@shatter, @surface, @knife, @fate,
bidirectional/reversible/lens, Vox/Rue/Tom/Cartographer/Explorer/Abyss,
linear/NL, encoder/decoder). Read-only. No shard mutation. No fmt.

**Verdict shape asked-for:** how much of Alex's proposal is already
substrate-declared? Substrate-already-had-the-word candidate?

**Verdict:** **PROMOTABLY substrate-already-had-the-word at the
bi-directional layer; NEW at the @shatter(g?, l?) quadrant-lens
calculus.** Details below.

---

## §2. Grep matrix

### 2.1 `@shatter` (grammar keyword + shard + file format)

| Path | Hits | Notes |
|---|---|---|
| `docs/specs/shatter-transformer-bidirectional-v0.1.md` | **132** | Mara canonical spec, 2026-06-22, tick 74. Already substrate-decl'd @shatter as transformer AND bilateral. |
| `shards/smarts/shatter.mirror` | 95 | Species shard with `render` + `parse` + `round_trip_pair` + `shatter_round_trip` composed bilateral. Landed 2026-06-22. |
| `docs/specs/historical/2026-04-14-lsp-shatter-plan.md` | 99 | Historical: shatter-as-LSP plan. |
| `shards/mirror/lens/cli/shatter.mirror` | 35 | CLI verb dispatcher. |
| `shards/reflection/shatter.mirror` | 39 | `@reflection/shatter` species shard (tick 38). Uses `render(gp: graph_path, f: frame, p: perturbation) -> moi(text)` — uni-directional at declaration site. |
| `shards/mirror/shatter.mirror` | 49 | `@mirror/shatter` (disk projection). Already carries `project(au) -> shatter` + `ingest(shatter) -> au` — fixed-point round-trip. |
| `shards/smarts/surface.mirror` | 5 | Sibling `@smarts/surface` species. |
| `docs/shatter-spec.md` | 54 | The five-section .shatter file format. |
| `docs/ai/shatter-training-pipeline.md` | 44 | Training pipeline for the Cartographer model. |
| `boot/03-shatter.mirror` | 3 | Old boot-era `grammar @shatter` with `materialize/crystallize/learn`. |
| `boot/std/kintsugi/shatter.mirror` | 7 | `shatter(ast, level) -> [ast]` + `settle_up` recursive-fracture pipeline. |
| `bootstrap/src/lib.rs` | 36 | Rust-side references. |
| `bootstrap/src/mcp.rs` | 10 | MCP surface. |
| `bootstrap/tests/mirror_spawn_fate_hinge_shard.rs` | 27 | @fate hinge testing. |

### 2.2 `@surface` (as species / model reference)

| Path | Hits | Notes |
|---|---|---|
| `docs/specs/surface-simplification.md` | 26 | Surface = CLI surface, NOT `@surface` model. Historical. |
| `shards/kintsugi/surface.mirror` | 18 | `@kintsugi/surface` (compiler-error surface). Not the Surface Model. |
| `shards/reflection/surface.mirror` | 4 hits (species-decl file itself) | The Surface Model species-decl. Uni-directional `translate(l: language) -> moi(query)`. |
| `shards/smarts/surface.mirror` | 5 | Sibling. |
| `shards/smarts/shatter.mirror` (in `in @smarts/surface` line) | — | The dep-graph link. |

**Key observation:** `@surface` as a family-root shard does NOT exist.
Only two SPECIES shards under it: `@reflection/surface` and
`@smarts/surface`. No `shards/surface.mirror`. This is a substrate-
level absence: Surface has never been declared as a substrate primitive
of its own — only as a Model species at two altitudes.

### 2.3 `@knife` (Alex's proposal referent)

| Path | Hits | Notes |
|---|---|---|
| `docs/audits/2026-07-07-seam-phase-d-o-cascade-torus-family-root-close.md` | 2 | @onto-cascade close reference. |
| `docs/math/2026-07-07-onto-cascade-autopoetic-grounding.md` | 18 | @onto-cascade grounding doc. |
| `docs/math/2026-07-07-onto-cascade-toroidal-reframe.md` | 4 | Toroidal reframe. |
| `docs/observation/2026-07-07-jspace-mirror-deep-mapping.md` | 8 | J-space mapping. |
| `docs/audits/2026-07-07-seam-phase-d-n5-tick-1-commit-as-fold-terminal-cascade-close.md` | 1 | N5 close. |

**Key observation:** `@knife` appears EXCLUSIVELY in TODAY'S (2026-07-07)
docs. Zero shard hits. Zero boot hits. Zero pre-today doc hits. **@knife
is a fresh forward-promise from the @onto-cascade + L-cascade
territory, not landed as substrate.** Alex's analogy "like @knife"
references a still-unnamed sibling that itself is
under-substrate-declared.

### 2.4 `@fate` (Recognition #58 landed)

Broad hits (37 files in mirror). Load-bearing sites:
- `shards/fate.mirror` (119) — family-root, tournament vocabulary.
- `shards/fate/tournament.mirror` (74) — tournament species.
- `docs/specs/bauchladen-autopoietic-fate.md` (223) — the bauchladen-fate
  composite spec.
- `prism/fate.mirror` — declares `model = abyss | pathfinder | cartographer
  | explorer | fate`. **Five models, but historically pathfinder ≠ mirror
  and fate reappears as its own model** — the naming has drifted since
  CLAUDE.md's Nox/Tom/Vox/Rue/Fate table.

### 2.5 bidirectional / reversible / lens / optics / transformer

| Path | Hits | Notes |
|---|---|---|
| `docs/specs/shatter-transformer-bidirectional-v0.1.md` | **132** | Canonical bidirectional Shatter spec (Mara). Named it explicitly. |
| `docs/specs/spectral-metalogue.md` | 97 | Metalogue spec, structural bidirection. |
| `docs/specs/bauchladen-autopoietic-fate.md` | 16 | Autopoietic fate composition. |
| `shards/kintsugi/surface.mirror` | (via lens/bidirectional) | Surface as compiler-error surface. |
| `shards/mirror/lens/cli/*.mirror` | many | Lens family established. `shatter.mirror` is one lens variant. |

### 2.6 Pipeline model names (Vox/Rue/Tom/Cartographer/Explorer/Abyss/Nox)

**CRITICAL DRIFT SIGNAL:** the CLAUDE.md table names models `Vox/Rue/Tom/
Nox` (with Explorer/Cartographer/Fate/Abyss as alternate names). But
substrate reality:
- `prism/fate.mirror` (April 2026) uses `abyss | pathfinder |
  cartographer | explorer | fate`.
- `boot/std/ai/*` (May 2026) uses `explorer/cartographer/fate/abyss`.
- Species shards `@reflection/{surface,shatter,mirror,reflection}` name
  `Explorer/Fate/Cartographer/Abyss` in comments as "the Cartographer
  Model runs at Shatter altitude."

The CLAUDE.md table (Vox/Rue/Tom/Nox) is a **later-named alias layer**;
the substrate itself is `Explorer/Fate/Cartographer/Abyss`, and
`pathfinder` still appears as a stale sibling in `prism/fate.mirror`.

### 2.7 NL / linear (as substrate keyword)

- `shards/nl.mirror` — `@nl` declared as prism, opaque text carrier at
  the floor. `#` is the sigil producing an `@nl` term.
- `shards/reflection/shatter.mirror` uses `in @nl` at import. `text = ref`.
- `shards/reflection/surface.mirror` uses `in @nl` at import. `language
  = ref`.
- **`linear` as a substrate term does NOT exist as a family-root or
  species.** Alex's "linear JSON" (envelope shape) has no substrate
  vocabulary. `@nl` is the closest sibling (natural language, opaque
  text). The linear/graph duality is not substrate-declared — but the
  `@nl` ↔ graph_path duality IS substrate-declared (via
  `@smarts/shatter`'s `render` ↔ `parse` pair).

---

## §3. Drift signals surfaced

### 3.1 `@shatter` bi-directional — ALREADY LANDED at species altitude

`shards/smarts/shatter.mirror` (lines 260-487) already declares:

```
render(gp: graph_path, s: smarts, p: perturbation) -> shatter_result
parse(t: text,        s: smarts, p: perturbation) -> shatter_result
round_trip_pair = { forward: shatter_result, reverse: shatter_result, v: verdict }
shatter_round_trip(pair, p) -> verdict
  requires rendering_grounds_text(pair.forward, p)
  requires parsing_grounds_graph(pair.reverse, p)
  requires round_trip_identity_preserved(pair, p)
```

This is the exact bilateral Alex asked about, minus the quadrant-lens
naming. Mara commissioned the spec 2026-06-22 (tick 74), Reed landed
the shard, Seam ratified it via C1/C2/C4/C9 closure. **The substrate
literally already carries `@shatter` as bi-directional lens between
text and graph_path.**

What is NEW in Alex's 2026-07-07 proposal:
- The **`@shatter(g?, l?)` quadrant calculus** — four quadrants
  (envelope emission = graph→linear; mission parsing = linear→graph;
  kintsugi settlement = graph→graph; identity ground state =
  linear→linear). The bilateral doc names only forward + reverse; the
  quadrant view is a strict generalization to the diagonal.
- The **"like @knife"** analogy — implies a lens family with named
  siblings. Not yet a substrate family.
- The **@surface elimination** — no explicit precedent in the corpus.
  `@reflection/surface` and `@smarts/surface` both exist as species
  shards; their elimination would require substrate-decl migration.

### 3.2 `@surface` is declared BUT unrooted

Both `@reflection/surface` and `@smarts/surface` exist as species
shards. No `shards/surface.mirror` (family-root) exists. This is
top-heavy in a way that the corpus normally corrects via family-root
declaration.

**Drift signal:** if the substrate were pulling toward `@surface` as a
separate primitive, we'd expect a family-root shard by now (comparable
to how `@mirror`, `@kintsugi`, `@fate`, `@reality` all have family-
root shards). The absence is meaningful: **the substrate has been
refusing to root `@surface` for six weeks.** Alex's proposal to
eliminate `@surface` as a primitive may be substrate-pull-correct —
the substrate never rooted it.

### 3.3 `@fate` still names five models, not one engine

`prism/fate.mirror` (April 2026) declares `model = abyss | pathfinder |
cartographer | explorer | fate`. This is a five-way sum type, not a
unified "one engine" abstraction. Reed's framing of `@fate` as "the
engine" (in the mission brief) leans on Recognition #58 (Fate IS
optical inference) — which lifts fate to the D²NN + Fabry-Perot +
Reck/Clements substrate. But at species-decl altitude, `@fate` still
names a tournament + a five-model sum. Reed's collapse framing is
legitimate under Recognition #58, but substrate-decl-lagging.

### 3.4 `@shatter` disk projection already round-trips

`shards/mirror/shatter.mirror` declares `project(au) -> shatter` +
`ingest(shatter) -> au` with an idempotence contract:

```
shatter(a) and shatter(shatter(a) -> au) settle to the same oid.
```

This is a **DIFFERENT** round-trip than the `render`/`parse` bilateral:
- `@mirror/shatter`: au ↔ .shatter (in-memory graph ↔ disk projection)
- `@smarts/shatter`: graph_path ↔ text (structural ↔ natural language)
- `@reflection/shatter`: uni-directional `render(gp) -> moi(text)`

Three instances of shatter, three different bilateral dispositions.
Only `@smarts/shatter` is fully bilateral at declaration site.

### 3.5 Rust code has `shatter_model.rs` but no `surface_model.rs`

`spectral/src/sel/shatter_model.rs` exists (33 hits). `surface.rs`
exists but as CLI surface, NOT model. **The Rust realization side
recognizes Shatter as a first-class thing to model, but Surface has
collapsed to CLI/API surface** — the substrate→Rust translation has
ALREADY done the collapse Alex proposes.

### 3.6 CLAUDE.md pipeline names (Vox/Rue/Tom/Nox) disagree with substrate

Re-audit finding: mirror/CLAUDE.md's `Surface/Mirror/Shatter/Reflection`
table uses `Rue/Tom/Vox/Nox` as "Explorer/Fate/Cartographer/Abyss"
aliases. But `prism/fate.mirror` still names `pathfinder` as a fifth
model that has no CLAUDE.md alias. This is stale-naming drift — the
CLAUDE.md table is aspirational, not substrate-truth-of-record.

### 3.7 `@shatter(g, l)` two-argument surface appears in NO code

Searched: no file uses `shatter(graph, linear)` or `@shatter(g?, l?)`
form. All calls are:
- `shatter(a: au)` — project
- `shatter(ast, level)` — kintsugi fracture
- `render(gp, s, p)` / `parse(t, s, p)` — bidirectional pair

The two-argument quadrant-selector form is **strictly new syntax**.

---

## §4. Substrate-already-had-the-word verdict

**Verdict: PARTIAL substrate-already-had-the-word.**

Grep-evidence quantification:

| Claim in Alex's proposal | Substrate precedent | Verdict |
|---|---|---|
| Shatter is a translation LAYER (not model) | `@mirror/shatter` disk projection (au↔.shatter) + `@smarts/shatter` render/parse pair | **~90% precedented.** Naming Shatter as "lens" tightens the existing framing rather than inventing it. |
| Shatter is bi-directional | `@smarts/shatter.parse` + `round_trip_pair` + `shatter_round_trip` composed bilateral (tick 74, 2026-06-22, 132-hit canonical spec) | **100% precedented.** Landed at species altitude. |
| Shatter is between linear and graph | `@smarts/shatter`: text ↔ graph_path; `@mirror/shatter`: au ↔ .shatter | **~80% precedented.** "Linear" as a term is new; "text" and ".shatter" are the substrate's carriers for the same slot. |
| @shatter(g?, l?) quadrant calculus | none | **NEW.** Substrate has never named the four quadrants. |
| @knife as sibling lens family | `@knife` referenced only in TODAY's docs (2026-07-07) | **NEW-TODAY.** Not landed. |
| @surface eliminated as primitive | `@surface` family-root never landed; only species. Rust side collapsed it to CLI surface | **~70% substrate-pull-consistent.** The absence of a family-root shard is a six-week drift signal AWAY from `@surface` as primitive. |
| Four-model pipeline collapses to two lenses + @fate + @torus | @torus landed today (`shards/torus.mirror`); @fate is family-root (`shards/fate.mirror`); shatter+surface bilateral | **~50% precedented.** The collapse itself is a NEW recognition-candidate; the individual pieces are substrate-declared. |

**Aggregate:** ~65-70% of the proposal is substrate-already-had-the-word.
The bi-directional lens framing is a **rename + tightening** of an
already-landed structural claim (Mara's tick-74 spec). The QUADRANT
calculus and @surface elimination are the genuinely new load-bearing
claims that would need Pack ratification.

---

## §5. Unnamed adjacencies

### 5.1 `@knife` precedent search: NONE PRE-2026-07-07

`@knife` appears ONLY in today's docs (5 files total, all
2026-07-07). No shard, no boot, no historical spec. Task #556 (per
mission brief) hasn't landed. **Alex is analogizing to a
forward-promise, not a landed sibling.**

Historical near-neighbor: `shards/mirror/lens/cli/crack.mirror` (12
hits on shatter-adjacent terms) — the substrate has a `crack` lens
verb that behaves like knife-shaped decomposition. Possible precedent
sibling but not named @knife.

### 5.2 `.shatter` file format precedent

`docs/shatter-spec.md` (54 hits) + `shards/mirror/shatter.mirror` (49
hits) already declare `.shatter` as a five-section canonical projection
format with idempotent fixed-point contract:

```
mirror compile source.mirror -> output.shatter
mirror compile output.shatter -> output.shatter  # no-op
```

This is the LINEAR-side artifact Alex's envelope proposal is
gesturing at. **.shatter files are already the substrate's canonical
linearization.** The mission-envelope from `mirror spawn` may be a
.shatter derivative, not a new format.

### 5.3 `@fate` expected direction: bi-directional by inference already

Recognition #58 (Fate IS optical inference) sets `@fate` on D²NN +
Fabry-Perot resonator + Reck/Clements unitary mesh. Reck/Clements
meshes are provably invertible (unitary operators), and Fabry-Perot
cavities are bidirectional by construction. So **@fate is
substrate-decl-bidirectional-by-inheritance from Recognition #58**,
even though no explicit bidirectional signature appears in
`shards/fate.mirror`.

If @shatter becomes a lens, @fate is the engine that RUNS the lens in
either direction — which matches Reed's collapse framing.

### 5.4 @torus offers the closure Alex may be reaching for

`shards/torus.mirror` (landed today, 4 hits on shatter/surface/lens
terms) declares the observation surface as toroidal. The **meridian +
longitude** traversals of the torus map naturally to the two
directions of a bidirectional lens: meridian = graph→linear traversal
(envelope emission), longitude = linear→graph traversal (mission
ingestion). **Alex's quadrant calculus may be substrate-decl-equivalent
to the four cardinal directions of torus traversal.**

### 5.5 Rust side already treats surface as CLI, not model

`spectral/src/sel/surface.rs` is CLI surface plumbing, not a Surface
Model realization. Meanwhile `spectral/src/sel/shatter_model.rs`
exists as a distinct model file. **The Rust realization has
asymmetrically privileged Shatter over Surface** — which is precisely
Alex's proposed direction.

---

## §6. Signal-to-Reed — top 3 sharpest findings

### FINDING 1 (STRONGEST): Mara's tick-74 spec already lands @shatter as bidirectional lens

**`docs/specs/shatter-transformer-bidirectional-v0.1.md` (132 hits) +
`shards/smarts/shatter.mirror` (95 hits) already declare the exact
bilateral Alex is proposing.** The spec explicitly names the encoder-
decoder mapping:

```
transformer-decoder = Shatter.render  (graph_path → text; autoregressive)
transformer-encoder = Shatter.parse   (text → graph_path; bidirectional context)
```

Alex's proposal is a **rename + tighten** of an already-landed
recognition. The load-bearing bilateral (`shatter_round_trip` +
`round_trip_pair`) exists as substrate-decl and passed Seam
ratification (C1/C2/C4/C9 closure). **The substrate already had the
word "bidirectional shatter" — for two weeks.**

What's genuinely NEW: the `@shatter(g?, l?)` quadrant syntax + the
collapse of the four-Models pipeline into two lenses. Those are
recognition-candidates worth Pack ratification. But the bi-directional
lens framing itself is a promoted precedent.

### FINDING 2: `@surface` was never rooted — six-week substrate-refusal signal

**No `shards/surface.mirror` family-root exists.** Only species shards
(`@reflection/surface`, `@smarts/surface`). The substrate has had six
weeks to root `@surface` alongside `@mirror`, `@kintsugi`, `@fate`,
`@reality`, `@torus` — and has consistently refused. The Rust
realization side (`spectral/src/sel/surface.rs`) has already
collapsed Surface to CLI surface, not model.

**This is a substrate-pull signal FOR Alex's elimination proposal.**
The substrate has been telling us for six weeks that `@surface` is
not a substrate primitive. Alex's proposal to eliminate it may be
recognition of a refusal-that-already-happened.

### FINDING 3: `@knife` is a forward-promise, not a landed sibling

**`@knife` appears in ZERO pre-2026-07-07 files.** Alex's "like @knife"
analogy references a sibling that only exists in today's docs
(@onto-cascade + L-cascade close). Task #556 hasn't landed. The
nearest precedent is `shards/mirror/lens/cli/crack.mirror` (a
knife-shaped decomposition verb).

**Implication:** the substrate does not yet carry the lens-family
vocabulary Alex is analogizing to. If the proposal lands, `@knife` +
`@shatter` would need co-landing as a lens family; the substrate
hasn't had time to grow adjacent siblings. This is a **new-family
recognition** more than a substrate-already-had-the-word recognition
for the LENS FAMILY, even though bidirectional shatter itself IS
precedented.

---

## Bonus: unanticipated adjacency

**@torus (landed today) may be the closure structure the lens
calculus needs.** Meridian traversal = graph→linear; longitude
traversal = linear→graph. Fixed points of both = identity ground
state. The four quadrants of `@shatter(g?, l?)` map onto the four
cardinal directions of torus traversal. **This is not in Alex's
proposal, but it's the topological completion the substrate just
landed.**

---

**Method disclosure:** grep-first, read-only, no code / shard
changes, no fmt. All hit counts sourced from ripgrep-equivalent
Search calls; verification via direct read of the load-bearing files
(`shards/smarts/shatter.mirror`, `shards/reflection/{shatter,
surface}.mirror`, `shards/mirror/shatter.mirror`,
`docs/specs/shatter-transformer-bidirectional-v0.1.md` §1-2). Hedges
preserved where the numeric hit-counts run ahead of semantic
verification (e.g., 132 hits in the shatter-transformer spec —
verified as canonical, not merely occurring).
