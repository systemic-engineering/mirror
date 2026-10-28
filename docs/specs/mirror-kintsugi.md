# `mirror kintsugi` — gluing compiled shards (spec)

*2026-10-29. Mara. Status: **Red** (nothing here runs; §9 is the red-test order).*

**Decision (Alex, 2026-10-29, ratified: "yes … excitiiiing!"):** `mirror compile` compiles a single shard. `mirror kintsugi` does kintsugi. Compile computes local sections and can only say "I don't know here" (a **hole**). Kintsugi glues them and says "you two disagree" (a **comma**). The recorded resolution of a comma (rerere) is the gold in the seam.

**Math root:** `docs/math/2026-10-29-mara-compile-kintsugi-local-sections-and-gluing.md` (cited "math §n"). **Sibling:** `docs/specs/mirror-fixpoint-parser.md` (cited "compile §n"): §1 compile is one shard; §1.1 its record; §1.3 its interface and the `undeclared` lint; §5.1 holes as question/occurrence. **Prior:** `docs/math/2026-10-10-mara-the-difference-that-makes-a-difference-math-foundation.md` §8; Taut's scouts 2026-10-28 (switchboard) and 2026-10-29 (psychohistory).

Mirror-text is the default output. `--out @data/json` is the only JSON, through `E(G_json)` and `print_json` (compile §9). No serde.

---

## §1 What `mirror kintsugi <target>` does

**Input.** A target names a shard set:
- a `.spec` file: the shards it declares (`mirror kintsugi mirror.spec` = all of mirror);
- a directory: every `*.mirror` under it;
- one or more `.mirror` files.

**Pipeline.**

| # | stage | input → output | math |
|---|---|---|---|
| 1 | **compile** | each shard → its compile record and interface `I(S)`. Served from the compile cache when the shard's bytes and grammar are unchanged; kintsugi never re-parses a cached shard | compile §1 |
| 2 | **book** | `@name → current addr` for the set: `shard(X)` holds iff some compiled shard's normal form has `out @X` | §3 |
| 3 | **cover** | the `in @X` lines and the `uses` of each interface → the reference graph Σ, the cover `Kᵢ`, the line graph G | math Def. 1.1–1.3 |
| 4 | **resolve keys** | per line end, `κ = H(addr of the restriction output ‖ κ of every foreign name it mentions)`, bottom-up over G's condensation | math §4, Prop. 4.2 item 3 |
| 5 | **glue** | per line: compare the two restriction outputs → line commas; per set: union-find → `fractured`, comma among all | math §2, §3 |
| 6 | **mend** | recorded resolutions applied by occurrence; around each cycle of mended lines, check holonomy | math §3.3 |
| 7 | **deposit** | each finding as a splinter at `@kintsugi/record`, with question, occurrence and tick; the run's record as a fragment | §6 |

**What kintsugi never does:** read bytes of a shard outside the target, look up a path to resolve `@X`, or compare `uuid`s (merges and identity are on `addr`, compile §8).

---

## §2 The record

### §2.1 Summary record

```text
verdict     comma
target      "mirror.spec"
shards      410
lines       2311
unused      14
holes       1873
undeclared  35
unresolved  1
commas      3
among       2
fractured   1
reglued     17
cached      2294
run         <addr of this run's record fragment, 64 hex>
```

| field | meaning | owner |
|---|---|---|
| `shards`, `lines` | compiled shards; used `in @X` lines (math Def. 1.3) | kintsugi |
| `unused` | declared lines with an empty overlap (`in @X`, nothing of X used): **questions**, not failures | kintsugi |
| `holes`, `undeclared` | sums of the compile records; kintsugi passes them through and **adds no holes of its own** except `unresolved` | compile |
| `unresolved` | `in @X` with no shard X in the set (§3) | kintsugi |
| `commas` | line commas: lines whose two restriction outputs differ (math Def. 3.2) | kintsugi |
| `among` | the comma among all: `Ĥ¹(𝒩; H⁰) = b₁(Σ) − Σᵢ b₁(Kᵢ)` (math Prop. 2.2). With mends recorded, the number of independent cycles of lines with non-trivial holonomy is reported as `holonomy <n>` beside it | kintsugi |
| `fractured` | `b₀(𝒩) = b₀(G)`: how many compositions the set falls into (math Def. 2.4). `1` is glued | kintsugi |
| `reglued`, `cached` | lines re-checked vs. served from the κ cache (§5) | kintsugi |

**Every b₁ is named with its complex** in field docs and output: `among` is `b₁(Σ) − Σ b₁(Kᵢ)`, never "b₁". `b₁(Γ(N))` (reuse inside one AST, compile §8) and `b₁(G)` (cycles of lines, the holonomy capacity) are different numbers, and neither is `among`.

### §2.2 One record per finding

Blank-line separated, after the summary, sorted by `(kind, shard name, name)`:

```text
unresolved  @epistemologic/bogus
in          @kintsugi/fracture/inport
line        14:1

comma       line
from        @kintsugi/roomba
to          @kintsugi/fracture
name        detect
expects     action/2
exports     absent
question    <H("comma\0" ‖ addr ρ_X ‖ addr ρ_Y), 64 hex>
resolution  none

among       cycle
via         @a/x  @b/y  @c/z
symbols     a.x  b.y  c.z

unused      @glass
in          @sheaf
```

- `exports` is `absent`, or `<kind> <addr>` when X exports the name with a different kind.
- `resolution` is `none`, or `offered <addr>` when a recorded resolution exists for this **question** but hasn't been applied at this **occurrence** (rerere: offered by question, applied by occurrence; compile §5.1).
- `among cycle` lists one representative cycle per independent class (a basis from the union-find spanning forest). The class is the invariant; the representative is a choice and is marked as one.
- Source coordinates (`line 14:1`) come from the erasure record. They are for people and never part of identity.

### §2.3 `--out @data/json`

The same fields as JSON, through the JSON grammar's printer. An `@io` crossing.

---

## §3 The lint split: two pact rules, one per verb

```text
# compile (one shard; compile §1.3):
undeclared(S, X) :- references(S, X), not imports(S, X), not declares(S, X).

# kintsugi (the set):
unresolved(S, X) :- imports(S, X), not shard(X).
```

- `references` reads **code lines only**. Prose references are reported (`--prose`), never failed (compile §1.3).
- `shard(X)` is content: some compiled shard declares `out @X` in its normal form (reversed R3 inserts it, decision 13). Never a path lookup.
- **`unresolved` is a hole of the cover**, not a comma: the cover names a site that has no section ("I don't know here"). It is `@epistemologic/pact/composition_closed` reborn (`shards/epistemologic/pact/composition_closed.mirror`: "every `in @X` statement … resolves to a … declaration"); its Rust discharge was deleted with `bootstrap/` in `9de3eca`.
- **Why `undeclared` is compile's and not kintsugi's:** it is decidable from one shard (`inport.mirror` §5's proof), and it is the precondition for kintsugi's cutoff to be sound. A dependency with no line has no key (math Prop. 4.2, item 1).
- **Cross-shard type errors** are line commas: Y uses `X.n` as kind k and X's interface doesn't export `n`, or exports it as another kind. Kintsugi reports them at the line (§2.2).

---

## §4 Verdict and exit codes

| verdict | when | exit | with `--strict` |
|---|---|---|---|
| `success` | no holes, undeclared, unresolved, line commas, or comma among all with non-trivial holonomy | 0 | 0 |
| `partial` | holes only (compile `partial` somewhere; `unused` lines are questions and also land here) | 0 | 1 |
| `comma` | any line comma, any non-trivial holonomy, or any `unresolved` | 1 | 1 |
| `fail` | any compile `fail` (`undeclared > 0`) | 1 | 1 |
| `dark` | any shard dark | 2 | 2 |

- `among > 0` **with no mends recorded** is capacity, not a disagreement: places a comma *could* live (math §2.2, §3.3). It is reported and does not change the verdict. With mends recorded, `holonomy > 0` is a comma.
- `fractured > 1` is reported. Whether it fails the gate is Alex's call (§10 Q4). Lean: report only, since a spec may legitimately hold two compositions.
- **The commit gate is `mirror kintsugi mirror.spec`**, exit 0 required, restored when the verb dispatches. Until the 35 undeclared references (compile T17d) are mended, the gate fails by design: that is the lint doing its job.

---

## §5 The incremental contract (the switchboard)

The switchboard Alex named (Taut 10-28) is this contract. Its book is `@name → current addr` (§1 stage 2). Its lines are `in @X` (math Def. 1.3). It rings the star of what changed.

**Content layer (line commas).** Cache each line's verdict under its two resolved keys `κ` (§1 stage 4).
- An edit to shard S re-glues only the lines whose κ changed. Those lie in the star of S in G, plus any line upward whose resolved key mentions a changed name (math Prop. 4.2).
- **Sound under stalk completeness**: no undeclared references (compile lint); exact coverage (compile §12 row 21); resolved keys through re-exports; `requires` on another shard's behaviour pulls that body's address into the line's stalk (math Def. 4.1).
- The restriction output is the **coarsest** sound key for these equality verdicts (math Prop. 4.3). Nothing smaller works.

**Shape layer (`among`, `fractured`).** Per shard, cache `(b₀(Kᵢ), b₁(Kᵢ), the component of each shared symbol in Kᵢ)`.
- A new or removed reference to a symbol already in `Kᵢ` is private (math Prop. 2.5). `among` updates by 10-10 Thm 8.4: unchanged inside a component, +1 when it joins two.
- A first use of a foreign symbol, or the last use removed, changes an overlap (R7). It is recomputed by union-find over the stars it touches, and is **OPEN** as a locality claim.

**Holonomy layer.** A changed mend, or a changed restriction output under a mend, invalidates only the cycles through that line (math Prop. 4.5). Whether the count updates from star data alone is OPEN.

**Not used:** `action_cache.mirror`'s closure key (`inputs_oid`). It is sound and never star-local (math Prop. 4.4). The kintsugi cache keys on κ. The action cache stays for actions whose inputs genuinely are a closure.

**Determinism.** Same compiled crystals + same recorded mends → same `run` address, independent of which lines came from the cache. Test K7.

---

## §6 rerere for commas

- A **line comma** has a question (`H("comma\0" ‖ addr ρ_X ‖ addr ρ_Y)`: the disagreement itself, independent of where it occurs) and an occurrence (`(line, name)` = `(addr I(Y), addr I(X), name)`).
- `mirror fate <occurrence> <resolution>` records a mend `g_e` as a splinter `resolution(question, body)`. That is the same verb and the same store as hole resolutions (compile §5.1). Recording it **offers** it at every occurrence of the same question and **applies** it only at the named occurrence.
- An applied mend makes the line glue under the mend. The mends of all lines are a 1-cochain, and kintsugi checks holonomy around each cycle of mended lines (math Prop. 3.5). A mend family that every line accepts and some cycle doesn't is a **comma among all**. It is owned by no pair, and no single mend can fix it.
- Holes and commas rotate (math §5): a line comma is a hole of the mend sheaf (no mend recorded yet), and an unfillable hole is a comma with its surroundings.

**Psychohistory is history.** The deposits of §1 stage 7 (`(line, question, resolution, tick)`: what cracked, when, how it was mended) are the history of gluings. `mirror kintsugi --history <@name>` prints them for one shard's lines. They are constitutive (the 2026-06-26 insight §4.4), not predictive. **Predicted futures are out of scope.** Recorded as future work, the light-cone flip: a sheaf over hypothetical edits; 10-10 Thm 8.4(ii)/(iii) already gives its exact one-edit case.

---

## §7 Three readings of one base

All three live over the same shard set (Alex + Reed; Taut-checked 2026-10-29):

1. **Declared topology** (`in @X`): the cover kintsugi routes on. §1–§5. Built first.
2. **Voices** (section ownership; `shards/peer/colony.mirror`, one psychohistory root per K ≥ 2 peers; `@mirror/spectral` sections): a second cover, over who wrote what. A comma's **owners** are the voices on its two ends. Reserved fields: `owners <voice> <voice>` on each comma record, filled once a voice cover is declared. Alex: the colony is "probably" the voice cover (§10 Q3).
3. **Observed** (the Voice: calls seen at run time): a second sheaf over the same base. Per line (math Prop. 6.1):
   - an observed call with no declaration → `comma observed` (an undeclared dependency at run time);
   - a declared line never observed in the window → `question dead` (absence in a window isn't falsity).

   Not built until the Voice lands (prism `Beam → Voice` is still "Next"). Record kinds reserved.

---

## §8 Collapse plan for the shard declarations

The spec only. No shard is edited by this pass: none of these edits is a mechanical rename, and two need Alex's call. Each item lands as its own `.mirror`-only commit once ratified.

| # | where | today | after |
|---|---|---|---|
| C1 | `shards/sheaf.mirror` `sheaf_of` (`:240`) | `{stalks: ref, restrictions: ref, base_graph: ref}`, no cover, restriction `restrict(s, e)` maps vertex stalk to **vertex** stalk (`:293-299`), contradicting its own prose (`:78`: `𝓕_{v◁e}: 𝓕(v) → 𝓕(e)`) | **one** `sheaf_of = { base: cell_complex, cover: [site], nerve: ref, overlaps: ref, stalks: ref, restrictions: ref }` with `restrict(s: sheaf_of, v: ref, e: ref) -> ref { \ }`, vertex → **edge**, per incidence. `sheaf_laplacian` applies only when the stalks are vector spaces (docstring), and its "LANDED primitive" claim (`:249, :305`) becomes "deleted in `9de3eca`" |
| C2 | `psychohistory_sheaf`, declared twice: `shards/song/narrative.mirror:1099` (`= ref`, history, consumed by `fate/tournament.mirror:9, 861`) and `shards/psychohistory.mirror:297` (record over possible futures, no consumer) | two | **one**, at the family root `@psychohistory`, with the **history** reading (Alex 10-29: "the history is history"): base = the trajectory of gluings (§6), stalks = kintsugi deposits. `narrative.mirror` replaces its `type` with `in @psychohistory`. The futures carriers (`possibility_projection`, `future_settlement`, `project_future`, `settle_future`) move under a "light-cone flip: future work" block or are retired: **Alex's call (§10 Q5)** |
| C3 | `psychohistory_fiedler_monotone_climb` (`psychohistory.mirror:381-389`) | λ₂ non-decreasing per tick, from Rec #89 | retired. 10-10 §8.6: "Fiedler strict rise as a law … not in general"; comma doc Prop 2.7: on a fixed frame no letter widens |
| C4 | **fractured**, three definitions: `sheaf.mirror:38, 91` (dim H⁰ > 1); `psychohistory.mirror:292-294, 345` (H¹ > 0 "multiple disconnected"); `narrative.mirror:1143-1170` (λ₀ > 0, no global section) | three | **one**, declared once at `@sheaf`: `fractured(s) := dim H⁰ > 1` (at shard altitude, `b₀(𝒩)`, math Def. 2.4). `psychohistory.mirror`'s H¹ gloss becomes "comma among all"; `narrative.mirror`'s λ₀ bilateral splits into **hole** (some site has no section) and **comma** (sections exist and disagree) |
| C5 | **comma** | no declaration (`type comma` has zero hits) | **one**, declared once (proposed home `@kintsugi`, since kintsugi is the verb that reports it): `comma = { question, occurrence, kind: line \| among \| observed }`. Line = degree 0, owned by a pair; among = the H¹ class (math §3.2–3.3) |
| C6 | **hole** | `\` in the grammar; question/occurrence in compile §5.1 | **one**, compile-side, at `@mirror` (it is syntax): `hole = { question, occurrence }`. `unresolved` reuses it with site = the missing shard |
| C7 | **b₁**, four meanings (Taut 10-28 §2: comma, holes/tasks, reuse, bugs) | bare "b₁"/"H¹" | always written with its complex: `b₁(Σ)`, `b₁(Kᵢ)`, `b₁(G)`, `b₁(Γ(N))`, `Ĥ¹(𝒩; H⁰)`. A docstring with a bare b₁ is a drift finding |
| C8 | `shards/kintsugi/fracture/inport.mirror` | "Consumers: `mirror kintsugi <shard>`" | consumer: `mirror compile` (the lint is compile's, §3). `docs/math/2026-07-23-…inport…` §3's H¹ theorem gets a pointer to math §8 (it fails) |
| C9 | `shards/epistemologic/pact/composition_closed.mirror` | discharge "the Rust impl in `bootstrap/src/lib.rs`" (deleted) | discharge: `mirror kintsugi`, `unresolved` |

**Names Alex hasn't given yet (he names primitives).** Substrate-native names for:
- the **hole** at kintsugi altitude (`unresolved`);
- the **line comma** versus the **comma among all**;
- what the corpus calls b₁ when it means **bugs** (Young's sense) and when it means **tasks** (prism `tension.rs`).

The spec uses the descriptive names above as placeholders.

---

## §9 Red tests, in order

Against the binary, the way `rust/tests/pact_vectors.rs` drives `mirror pact eval`. Fixtures under `tests/kintsugi/`. Every count has an oracle independent of the code under test: union-find, Euler's formula, or a rank computation (math §2.2 gives all three).

**K1. Compile is local (gates everything).** Compile T17a is green: compile reads no other shard.

**K2. Čech (port of 10-10 T28).**
- a) **C₆ in three arcs, as an import DAG** (math §2.2 table): `among 1`. Each pair glued alone (`mirror kintsugi a.mirror b.mirror`, etc.): `among 0`. **P₆ control**: `among 0`.
- b) **Theta** (cannot be built from shards, math §2.2): a test of the union-find oracle on a raw cover. It checks `Ĥ¹(𝒩;H⁰) = 0`, `Ĥ⁰(𝒩;H¹) = 3`, `Ĥ²(𝒩;H⁰) = 1` against rank computations. Not a CLI test.
- c) **Diamond**: `among 1`, while `b₁(𝒩) = 0`. Guards against reading the bare nerve.
- d) Random shard sets (≥ 1000, fixed seed): `among` = Euler (`b₁(Σ) − Σ b₁(Kᵢ)`) = Čech rank.

**K3. Incremental (port of 10-10 T29).** On a connected set:
- a private reference inside a component of `Kᵢ` → `among` unchanged, `reglued` ⊆ the star of i;
- one joining two components of `Kᵢ` → `among` + 1;
- an edit to a body that no interface exposes → `reglued 0` (math Prop. 4.4: the closure key would re-glue every importer).

**K4. Lint split.**
- `in @bogus` with no shard `@bogus` → `unresolved 1`, `verdict comma`, exit 1.
- The same file under `mirror compile` alone → no finding (compile can't know).
- The corpus `undeclared` fixture (compile T17d) passes through: kintsugi's `undeclared` = the sum of compile's.

**K5. Line commas.**
- Y uses `X.f` as `action/2`, X exports `f` as a carrier → `comma line`, `expects action/2`, `exports carrier <addr>`.
- X doesn't export `f` → `exports absent`.
- The same disagreement in two different lines → one `question`, two occurrences.

**K6. rerere.** Record a mend at one occurrence (`mirror fate`): that line glues; the other occurrence of the same question shows `resolution offered <addr>` and still counts as a comma until applied. **Holonomy:** three shards in a cycle of lines, each line mended by a renaming whose composite around the cycle is not the identity → `holonomy 1`, `verdict comma`, while each line alone glues.

**K7. Determinism.** A full run and an incremental run after an edit-and-revert print the same `run` address.

**K8. Stalk completeness (the cutoff is unsound without it).** X re-exports `T = {a: @Z.U}`; Y uses T only. Change U in Z → Y's line re-glues (resolved κ moved). With a fixture where Y references `@Z.U` on a code line without `in @Z`, compile fails (`undeclared`), so the unsound state cannot reach kintsugi.

**K9. Gate.** `mirror kintsugi mirror.spec` prints one summary record and exits per §4. Recorded as the first measured `undeclared`/`unresolved`/`commas` numbers.

Order: K1, K4, K2, K5, K3, K7, K8, K6, K9.

---

## §10 Questions

**For Alex.**
1. **Names** (§8): the kintsugi-altitude hole; line comma versus comma among all; b₁-as-bugs and b₁-as-tasks.
2. **"Commas are H¹ classes."** For compiled shards, a line disagreement is a degree-0 failure owned by a pair, and H¹ classes are the disagreements no pair owns: mends that don't compose (math §3.2–3.3). Keep "comma" for both with a kind field (lean), or reserve "comma" for the H¹ class?
3. Is `peer/colony` the voice cover (§7)?
4. Does `fractured > 1` fail the gate (lean: report only)?
5. The futures half of `@psychohistory`: park under "light-cone flip", or retire (§8 C2)?
6. Rows 18, 20, 21 of compile §12: the number type for `1 ≡ 1.0`; `undeclared` = fail; exact coverage.

**For Seam.** Math §10, items 1–5. Plus: (6) is resolving κ through re-exports enough when `μ` or a ruler changes (a grammar-level dependency that is not a line)? (7) K6's holonomy fixture: find a mend family whose holonomy is trivial on every cycle of G and still fails to glue.

---

## §11 What this spec does not do

- It predicts no futures (§6).
- It builds no sheaf Laplacian. Interface stalks are typed sets, and δᵀδ/λ₂ don't apply (math §3.1).
- It edits no shards (§8 is a plan).
- It writes no Rust. The engine for kintsugi is the same Datalog core as compile §4 (`undeclared` and `unresolved` are its rules). Union-find is the one non-Datalog piece, and it lives where the engine lives.
