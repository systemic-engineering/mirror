# `mirror kintsugi` — gluing compiled shards (spec)

*2026-10-29. Mara. Status: **Red** (nothing here runs; §9 is the red-test order). Second pass the same day, after Seam's crowbar (`docs/audits/2026-10-29-seam-crowbar-compile-kintsugi.md`, merged at `0fbc75d`) and Alex's seven defaults.*

**Decision (Alex, 2026-10-29, ratified: "yes … excitiiiing!"):** `mirror compile` compiles a single shard. `mirror kintsugi` does kintsugi. Compile computes local sections and can only say "I don't know here" (a **hole**). Kintsugi glues them and says "you two disagree" (a **comma**). The recorded resolution of a comma (rerere) is the gold in the seam.

**Math root:** `docs/math/2026-10-29-mara-compile-kintsugi-local-sections-and-gluing.md` (cited "math §n"). **Siblings:** `docs/specs/mirror-fixpoint-parser.md` (cited "compile §n") and `docs/specs/mirror-convergence.md` (the loop that mends what kintsugi finds). **Prior:** `docs/math/2026-10-10-mara-the-difference-that-makes-a-difference-math-foundation.md` §8; Taut's scouts 2026-10-28 (switchboard) and 2026-10-29 (psychohistory).

Mirror-text is the default output. `--out @data/json` is the only JSON, through `E(G_json)` and `print_json` (compile §9). No serde.

---

## §0 v0: a full run (Alex 2026-10-29: "The model holds. That means we can ship it.")

**v0 glues everything every time.** It recompiles (or reads the compile cache), rebuilds the book, the cover and the line graph, and re-checks every line on every run. That makes it correct by the parts of the math that HOLD (math §9, column "v0"):
- the cover and its counts (Prop 1.4 under (A1)–(A3), Prop 2.2);
- per-line gluing (Prop 3.2);
- the twist check by solving for gauges (Prop 3.6).

**After v0** (folded and corrected, but not needed for a correct full run): the switchboard (§5), early cutoff keys, `reglued`/`cached`, the incremental tests (K3, K7, K7′, K8's cache half), and similar-distance rerere offers (convergence §4, which wait for the `active` ruler).

Seam's FAILS split into two kinds:
- **Data findings.** The compiler reports these, and they are kintsugi's first real test (**K0**, §9): bare names (`verdict` in `roomba.mirror`), duplicate claims (`@song`, `@mirror/spec`; 20 names across `boot/` + `shards/`), the 11 shards without `out`, and the undeclared references.
- **Incremental claims.** R7 is not star-local; "+1 when it joins two" needs Σ connected; the cache key needs start and path; K3/K7 pass with a cache that never invalidates. These are folded into §5 and §9 and marked *after v0*.

---

## §1 What `mirror kintsugi <target>` does

**Input.** A target names a shard set:
- a `.spec` file: the shards it declares (`mirror kintsugi mirror.spec` = all of mirror);
- a directory: every `*.mirror` under it;
- one or more `.mirror` files.

**Pipeline (v0: every stage runs in full, every run).**

| # | stage | input → output | math |
|---|---|---|---|
| 1 | **compile** | each shard → its compile record and interface `I(S)`. Read from the compile cache when `(grammar, start, bytes)` is unchanged (compile §1) | compile §1 |
| 2 | **book** | `@name → shard`, from each normal form's `out @X` / `grammar @X` / `prism @X`. The book must be a **function** (Alex default 1). Two claimants of one name give a `claim` finding, and every `in @X` to that name reads Opaque ("which one?") | §3 |
| 3 | **bind** | each shard's `free` bare names against the exports of its in-targets → bound, **unbound** (Dark), or **ambiguous** (Opaque, "which one?") | §3; math (A1), (A3) |
| 4 | **cover** | qualified uses + bound free names → the reference multigraph Σ, the cover `Kᵢ`, the line graph G (a line is used iff `U(Y, X) ≠ ∅`) | math Def. 1.1–1.3 |
| 5 | **glue** | per line: the kind/arity relation `R` on the two restriction outputs → line commas. Per set: union-find → `among`, `fractured` | math §2, §3 |
| 6 | **mend** | recorded resolutions applied by occurrence. Per line: does the mend glue it? Per set: solve for gauges; a family that isn't gauge-trivial is a **twist** | math Prop 3.6, Def 3.7 |
| 7 | **deposit** | each finding as a splinter at `@kintsugi/record` with question, occurrence and tick; the run's record as a fragment | §6 |

**What kintsugi never does:** read bytes of a shard outside the target, look up a path to resolve `@X`, compare `uuid`s (merges and identity are on `addr`, compile §8), or check types beyond **kinds and arity** (Alex default 7: full type checking needs a type system mirror doesn't have yet).

---

## §2 The record

### §2.1 Summary record

```text
verdict     red
target      "mirror.spec"
shards      410
lines       2311
unused      14
holes       1873
names       11
dark        44
unbound     6
opaque      2
gaps        1
claims      2
commas      3
twists      0
among       2
fractured   1
expected    1
run         <addr of this run's record fragment, 64 hex>
```

| field | meaning | owner | verdict |
|---|---|---|---|
| `shards`, `lines` | compiled shards; used lines (`U(Y, X) ≠ ∅`) | kintsugi | — |
| `unused` | `in @X` with `U(Y, X) = ∅`: questions | kintsugi | partial |
| `holes` | `\` occurrences, summed from compile | compile | partial |
| `names` | shards with a `name?` hole (no `out`) | compile | partial |
| `dark` | undeclared `(S, X)` **pairs** (Dark lines), summed from compile | compile | partial |
| `unbound` | free bare names no in-target exports (Dark: no speaker) | kintsugi | partial |
| `opaque` | ambiguous bindings (two in-targets export the name) and `in @X` to a claimed-twice name: "which one?" | kintsugi | partial |
| `gaps` | `in @X` with no shard X: the kintsugi-altitude hole | kintsugi | partial |
| `claims` | names claimed by more than one shard (the book is not a function) | kintsugi | red |
| `commas` | line commas: kind/arity mismatch or `exports absent` | kintsugi | red |
| `twists` | mend families accepted line by line that are not gauge-trivial (math Def 3.7; name provisional) | kintsugi | red |
| `among` | the topology count `b₁(Σ) − Σᵢ b₁(Kᵢ)`: where a comma *could* live | kintsugi | — (reported) |
| `fractured`, `expected` | `b₀(𝒩) = b₀(G)`, and the count the spec's ruler declares | kintsugi | red iff they differ |

`reglued` and `cached` join the summary **after v0**.

**Every b₁ is named with its complex.** `among` is `b₁(Σ) − Σ b₁(Kᵢ)`. `b₁(Γ(N))` (reuse inside one AST, compile §8) and `b₁(G)` (cycles of lines) are different numbers. Neither is `among`.

### §2.2 One record per finding

Blank-line separated, after the summary, sorted by `(kind, shard name, name)`:

```text
claim       @song
by          shards/song.mirror  shards/song/keywords.mirror

gap         @epistemologic/bogus
in          @kintsugi/fracture/inport
line        14:1

opaque      verdict
in          @kintsugi/roomba
candidates  @glass  @kintsugi
question    which one?

unbound     frobnicate
in          @kintsugi/roomba

comma       line
from        @kintsugi/roomba
to          @kintsugi/fracture
name        detect
expects     action/2
exports     absent
question    <H("comma\0" ‖ addr ρ_X ‖ addr ρ_Y), 64 hex>
resolution  none

twist       mends
lines       @y1—@x  @y2—@x
gauge       unsolvable at @x

among       cycle
via         @a/x  @b/y  @c/z
symbols     a.x  b.y  c.z

unused      @glass
in          @sheaf
```

- `exports` is `absent`, or `<kind>/<arity>` when X exports the name with another kind or arity.
- `resolution` is `none`; `offered exact <addr>` when a recorded resolution exists for this question but hasn't been applied at this occurrence; or *(after the active ruler)* `offered similar <addr> distance <d> ruler <addr>` (convergence §4). Offers are never applied without the guard (convergence §1).
- `among cycle` gives one representative cycle per independent class. The class is the invariant; the representative is a choice and is marked as one.
- Source coordinates come from the erasure record. They are for people and never part of identity.

### §2.3 `--out @data/json`

The same fields as JSON, through the JSON grammar's printer. An `@io` crossing.

---

## §3 The rules

Datalog over compile's records, in the same engine as compile §4:

```text
# compile (one shard; compile §1.3)
undeclared(S, X) :- references(S, X), not imports(S, X), not declares(S, X).   # Dark line

# kintsugi (the set)
claim(X)          :- declares(S1, X), declares(S2, X), S1 != S2.
shard(X)          :- declares(S, X), not claim(X).
gap(S, X)         :- imports(S, X), not declares(_, X).
opaque_in(S, X)   :- imports(S, X), claim(X).
binds(S, n, X)    :- free(S, n), imports(S, X), shard(X), exports(X, n).
unbound(S, n)     :- free(S, n), not binds(S, n, _).                             # Dark
ambiguous(S, n)   :- binds(S, n, X1), binds(S, n, X2), X1 != X2.                 # Opaque
used(S, X)        :- uses(S, X, _).
used(S, X)        :- binds(S, n, X), not ambiguous(S, n).
unused(S, X)      :- imports(S, X), shard(X), not used(S, X).
comma(S, X, n)    :- uses_kind(S, X, n, k), not matches(X, n, k).               # incl. absent
```

- `references` and `free` read **code lines only**, and the comment tokens come from `μ_mirror` (compile §1.3).
- Coverage is **exact**: `in @kintsugi` doesn't cover `@kintsugi/roomba` (compile row 21).
- **No new comma kinds** (Alex default 4). Unbound is Dark (no speaker). Ambiguous is Opaque, "which one?", the same as a `settle` with two candidates and no ruler. A duplicate claim is a `claim` finding (default 1). `observed` waits for the Voice (§7).
- A **gap** is a hole of the cover: the cover names a site that has no section. It is `@epistemologic/pact/composition_closed` reborn; that pact's Rust discharge was deleted with `bootstrap/` in `9de3eca`.
- Assumption (A2) is pinned: **no `out` of a foreign name**. A shard that `out`s a name it doesn't declare is a `claim` finding against the declaring shard.

---

## §4 Verdict, exit codes, and the gate ruler

| verdict | when | exit |
|---|---|---|
| `success` | no finding with a verdict column in §2.1 | 0 |
| `partial` | only holes, `name?`, Dark (undeclared, unbound), Opaque, gaps, unused lines | 1 |
| `red` | any claim, line comma, twist, or `fractured ≠ expected` | 255 |
| `dark` | any shard dark | 2 |

The same table as compile §1.1 and `mirror love` (Alex default 5). Partial, red and dark are three codes.

**The gate is a ruler, declared in `mirror.spec`** and approved by a human like any ruler (compile §10):

```text
ruler @mirror/gate {
  tolerate   partial
  refuse     red  dark
  fractured  1
}
```

- `mirror kintsugi mirror.spec` reads the ruler, and the gate passes iff the run's verdict is tolerated and `fractured` equals the declared count.
- Lean: tolerate partial (honest holes, Dark lines and gaps don't block commits); refuse red and dark. Whether gaps belong in the tolerated set is the ruler's to say, field by field.
- With `boot/` in the target, today's 20 claims make the run red. That is the first convergence work (convergence §3), not a reason to soften the ruler.

---

## §5 The incremental contract (the switchboard) — after v0

The switchboard Alex named (Taut 10-28) is this contract. v0 doesn't use it. Everything below is folded from Seam's crowbar and corrected.

**Content layer (line commas).**
- Cache each line's verdict under the key `κ = H(rule-set addr ‖ X-side key ‖ Y-side key)`. *[Seam 6]* The rule-set address is in κ: change the relation `R` or the Datalog program, and every cached verdict is stale.
- *[Seam 7]* The coarsest sound X-side key is the **kinds and arities** of the used names (plus a body address for `pact` uses), not `addr(ρ_X)`. `addr` is sound but re-glues every kind-preserving signature change (math Prop 4.3(b)).
- `U(Y, X)` includes **all of Y's free bare names** on every line, so X starting or stopping exporting one of them moves κ (math Def 3.1).
- **Undeclared references are pseudo-lines** keyed on X's κ. A shard with any Dark line is otherwise re-glued every run, never served from cache (math §4 item 1).
- **Grammar and μ edits re-glue everything in that format.** They are not shard edits.

**Shape layer (`among`, `fractured`).** *[Seam 3]*
- A private edge changes `among` by `[d ∼ r in Σ] − [d ∼ r in Kᵢ]`.
- R7 (first use of a foreign symbol) changes it by `[d ∼ r in Σ]`.
- Both need Σ's **global** component map: neither is star-local.
- Cache the map globally. An insertion is O(α) by union-find. A deletion recomputes the affected component.

**Twist layer.** A changed mend, or a changed restriction under a mend, re-solves the gauges of the component of G it lies in. The cost is not star-local in general (math Prop 3.6: on a tree the constraints propagate).

**Not used:** `action_cache.mirror`'s closure key (sound, never star-local; math Prop 4.4).

**Determinism.** Same compiled crystals, same recorded mends, same rule set → same `run` address, whether lines came from the cache or not (K7′).

---

## §6 rerere for commas

- A **line comma** has a question, `H("comma\0" ‖ addr ρ_X ‖ addr ρ_Y)` (the disagreement itself, wherever it occurs), and an occurrence, `(addr I(Y), addr I(X), name)`.
- `mirror fate <occurrence> <resolution>` records a mend as a splinter `resolution(question, body)`, with the same verb and store as hole resolutions (compile §5.1). It is **offered** at every occurrence of the question and **applied** only where the human approves it (convergence §1, the guard).
- Kintsugi checks two things, separately (Seam 5):
  - **per line**: does the mend make this line glue? A renaming does not mend a kind mismatch;
  - **jointly**: is the mend family gauge-trivial? If not, it is a **twist**. A twist can sit on a tree of lines (math Prop 3.6) and can be owned by a pair (a mutual `in`). It is "not owned by one line", not "owned by no pair". A new export sometimes repairs it.
- **Holes and commas rotate** (math §5). A line comma is a hole of the mend sheaf. An unfillable hole is a comma with its surroundings.

**Psychohistory is history.** The deposits of §1 stage 7, `(line, question, resolution, tick)`, record what cracked, when, and how it was mended: the history of gluings. `mirror kintsugi --history <@name>` prints them. They are constitutive (the 2026-06-26 insight §4.4), not predictive. **Predicted futures are parked until the light-cone flip** (Alex).

---

## §7 Three readings of one base

1. **Declared topology** (`in @X`): the cover kintsugi routes on (§1–§5). v0.
2. **Voices.** Section ownership via `peer/colony` (Alex: "makes a lot of sense", held lightly). A comma's owners are the voices at its two ends. Reserved field: `owners <voice> <voice>`.
3. **Observed** (the Voice: calls seen at run time). A second sheaf over the same base, over **owner-qualified** facts (math Def 6.0). Per line:
   - `observed explained`: an observed call with no declaration, where a **declared path** joins the two shards (higher-order flow, Seam §9);
   - `observed unexplained`: no declared path (a hidden dependency);
   - `dead`: declared and never observed in the window. A question.

   These wait for the Voice (prism `Beam → Voice` is still "Next").

---

## §8 Collapse plan for the shard declarations

A plan only. No shard is edited by this spec. Each item lands as its own `.mirror`-only commit once ratified, and from now on as a **convergence mend** (convergence §3): offered, approved, written back.

| # | where | today | after |
|---|---|---|---|
| C1 | `shards/sheaf.mirror` `sheaf_of` (`:240`) | `{stalks, restrictions, base_graph}` (all `ref`), no cover; `restrict(s, e)` maps vertex stalk to **vertex** stalk (`:293-299`), against its own prose (`:78`) | one `sheaf_of = { base, cover, nerve, overlaps, stalks, restrictions }`, `restrict(s, v, e)` vertex → **edge** per incidence. `sheaf_laplacian` only for vector stalks; its "LANDED primitive" claim (`:249, :305`) → "deleted in `9de3eca`" |
| C2 | `psychohistory_sheaf`, declared twice: `song/narrative.mirror:1099` (`= ref`, history; consumed by `fate/tournament.mirror`) and `psychohistory.mirror:297` (futures; no consumer) | two | one, at `@psychohistory`, **history** reading. `narrative.mirror` imports it. Futures carriers parked under "light-cone flip" (Alex: parked) |
| C3 | `psychohistory_fiedler_monotone_climb` (`psychohistory.mirror:381-389`) | λ₂ non-decreasing per tick | retired (10-10 §8.6; comma doc Prop 2.7) |
| C4 | **fractured**, three definitions (`sheaf.mirror:38, 91`; `psychohistory.mirror:292-294, 345`; `narrative.mirror:1143-1170`) | three | one at `@sheaf`: `dim H⁰ > 1` (shard altitude: `b₀(𝒩)`). The H¹ gloss becomes "comma"; narrative's λ₀ bilateral splits into hole and comma |
| C5 | **comma** | no declaration | one, at `@kintsugi`: `comma = { question, occurrence, kind: line }`, plus `twist` as its own record (name provisional). No other kinds (Alex default 4) |
| C6 | **hole** | `\`; question/occurrence (compile §5.1) | one, at `@mirror`: `hole = { question, occurrence }`. `gap` and `name?` reuse it |
| C7 | **b₁**, four meanings | bare | always with its complex |
| C8 | `kintsugi/fracture/inport.mirror` | consumer `mirror kintsugi` | consumer `mirror compile`; the 07-23 math §3 points to math §8 (it fails) |
| C9 | `epistemologic/pact/composition_closed.mirror` | discharge in deleted Rust | discharge: kintsugi `gap` |
| C10 | `@song`, `@mirror/spec` claimed twice in `shards/` (each by `X.mirror` and `X/keywords.mirror`) | two claimants | one claimant each. The `keywords.mirror` files carry no `out`, so they are `name?` holes too. Fold them into the parent shard, or give them their own names (`@song/keywords`): a human picks |
| C11 | 20 names claimed across `boot/` + `shards/` | duplicates | `boot/` collapses into `shards/` (`mirror.spec` lists `boot/` as legacy). That is the first convergence run (convergence §3) |

**Names Alex hasn't given yet:** b₁-as-bugs, b₁-as-tasks; the final name for `twist`.

---

## §9 Red tests, in order

Against the binary, the way `rust/tests/pact_vectors.rs` drives `mirror pact eval`. Fixtures under `tests/kintsugi/`. Every count has an oracle independent of the code under test.

### K0. The golden test: reproduce Seam's audit (v0; gates the gate)

**Oracle:** Seam's crowbar (`docs/audits/2026-10-29-seam-crowbar-compile-kintsugi.md` §7.1, §7.2, §7.3, §10), mechanised by `tests/kintsugi/oracles/golden_k0.py`. Its outputs at `346253b` reproduce every count Seam reports: `golden_k0.shards.out` and `golden_k0.all.out`.

**Fixture:** `git archive 346253b shards boot` into a sandbox, so the corpus can't drift under the test.

**K0a, `shards/` only** (`mirror kintsugi <sandbox>/shards`). Must find:
- `claim` for **`@mirror/spec`** and **`@song`** (2);
- `name?` for the **11** shards listed in `golden_k0.shards.out` (e.g. `shards/mirror/book.mirror`, `shards/mirror/lens/knife.mirror`, both `keywords.mirror` files);
- `dark` (undeclared) for the **44 `(S, X)` pairs in 33 shards** listed there.

**K0b, `boot/` + `shards/`.** Must find:
- the **20** claimed names in `golden_k0.all.out` (among them `@kintsugi`, `@prism`, `@io` with three claimants, `@nl` with four, `@fate`);
- the **62** `name?` shards and **50** undeclared pairs listed there;
- Seam's ambiguity (§5.3): `roomba.mirror` uses bare `verdict`, exported by `@glass` (`shards/glass.mirror:467`) and by `boot/std/kintsugi.mirror:93`. Under §3's rules it surfaces **first as `opaque in @kintsugi`** in `@kintsugi/roomba`, because `@kintsugi` is claimed by both `shards/kintsugi.mirror` and `boot/std/kintsugi.mirror`. A companion fixture with the claim collapsed onto the `boot/` claimant must give `opaque verdict`, candidates `@glass @kintsugi`. Collapsed onto the `shards/` claimant (which doesn't export `verdict`), `verdict` binds to `@glass`.

**Pass rule.**
- **Every oracle finding appears** in kintsugi's records, matched by `(kind, shard, name)`. A missing one means the compiler is wrong, and the test is red.
- **Every extra finding is listed** in a `k0-extra` record set and reviewed by a human. An extra is a comma between the compiler and the oracle: either the oracle's lexical rule is too weak (fix the oracle, with a note) or the compiler is wrong (fix the compiler). Extras never pass silently and never fail silently.
- **Bare names (unbound, ambiguous) have no full oracle yet.** `golden_k0.py` doesn't parse free names. K0 asserts the one ambiguity Seam names and lists the rest as extras for review. A free-name oracle is open work (§10).

**K1. Compile is local.** Compile T17a, T17e and T17f are green.

**K2. Čech (port of 10-10 T28).** v0.
- a) **C₆ in three arcs, as an import DAG** (math §2.2 table): `among 1`; each pair alone `among 0`. **P₆ control**: `among 0`.
- b) **Theta**: an oracle test on a raw cover (it can't be built from shards). `Ĥ¹(𝒩;H⁰) = 0`, `Ĥ⁰(𝒩;H¹) = 3`, `Ĥ²(𝒩;H⁰) = 1`, against rank.
- c) **Diamond**: `among 1` while `b₁(𝒩) = 0`.
- d) Random sets: `among` = Euler = Čech rank. *[Seam 13]* This checks the counting, not the model.
- e) *[Seam 13]* **Mutual reference** (`A.a → B.b`, `B.b → A.a`): `among 1` (Σ is a multigraph).
- f) *[Seam 13]* **Absent export** (Y `in @X`, uses `X.f`, X declares no `f`): one `comma line … exports absent`, and the line counts as **used**, not `unused`.
- g) *[Seam 13]* **Bare name bound by two imports**: `opaque`, candidates listed.

**K4. The rules.**
- `in @bogus` → `gap`, `partial`, exit 1.
- The same file under `mirror compile` alone → no `gap` (compile can't know).
- Two shards with `out @x` → `claim @x`, `red`, exit 255; a third shard's `in @x` → `opaque`.
- A bare name exported by no in-target → `unbound`, `partial`.

**K5. Line commas (kinds and arity).**
- Y uses `X.f` as `action/2`, X exports a carrier → `comma line`, `exports carrier/0`.
- X exports `f` as `action/3` → `exports action/3`.
- The same disagreement on two lines → one `question`, two occurrences.
- Y reads a field that `X.T` lacks → **no comma**: kinds only (Alex default 7). The test asserts the absence, so the limit is visible.

**K6. Mends and twists.**
- Record a mend at one occurrence: offered at the other occurrence of the same question, applied only where approved.
- *[Seam 5]* A renaming mend on a **kind** mismatch leaves the line a comma.
- *[Seam 5]* **Tree twist:** `Y₁ — X — Y₂`, both using `X.f`, consumers frozen, mends (swap f↔g, id): every line glues under its mend, there are no cycles, and the result is `twist 1`, `red`.
- **Cycle twist:** three shards in a cycle of lines, renaming mends whose composite isn't the identity: `twist 1`.
- **Repair:** adding a second export name at X makes the tree twist 0.

**K9. Gate.** `mirror kintsugi mirror.spec` reads `ruler @mirror/gate` and exits per §4. The first measured numbers are recorded.

**After v0:**
- **K3. Incremental (port of 10-10 T29).** *[Seam 3, 13]* Each fixture states b₀(Σ). Assert `Δ among = [d∼r in Σ] − [d∼r in Kᵢ]` (a private edge inside a component: 0; joining two components of Kᵢ: +1 iff joined in Σ). R7: `Δ = [d∼r in Σ]`, with Seam's identical-star pair (Δ 1 vs 0). **Positive control** in the same harness: an edit that changes a line's restriction reports `reglued ≥ 1` and the new verdict. Only next to it is "`reglued 0` for a body no interface exposes" meaningful.
- **K7′ (replaces K7).** An edit **without** revert, chosen to change a line verdict: the incremental `run` equals a from-scratch run on the edited set. (K7's edit-and-revert passes with a cache that never invalidates.)
- **K8. Stalk completeness.** X re-exports `T = {a: @Z.U}`, Y uses T only; change U in Z → Y's line re-glues. A shard with an undeclared reference reaches kintsugi as `partial` (Alex's decision), and the test asserts that its lines are **not** served from cache (or are keyed as pseudo-lines on X's κ).
- **K10.** Changing the rule set (the relation `R`) re-glues every line.

**Order:** K0a, K1, K4, K2, K5, K6, K0b, K9; after v0: K3, K7′, K8, K10.

---

## §10 Decisions

**Answered by Alex, 2026-10-29, second round** (recorded by Reed in `002656a`; folded into the body text above):
- **Comma kind:** one word, comma, and the kind says who owns it. `comma line` is owned by a pair. *(The second-round "`comma among` = mends whose holonomy doesn't close" is superseded by default 3 below: that record is `twist`, and `among` is the topology count.)*
- **`gap`:** the kintsugi-altitude hole (formerly `unresolved`). A hole is inside a shard; a gap is between shards.
- **Numbers:** exact values; spelling to erasure. Refined by default 6.
- **Undeclared:** Dark at the line, partial for the document. A hole is Opaque (a question asked); undeclared is Dark (nothing looked).
- **Start rule:** what the grammar exports, named by the caller (compile §2 item 10).
- **`fixed`:** a rule declared `fixed` is already at its fixed point, `x = COORD(x)`. `settle` is the act, `fixed` the result.
- **Voice cover:** `peer/colony`, held lightly.
- **Futures of `@psychohistory`:** parked until the light-cone flip.

**Alex's seven defaults (2026-10-29, relayed by Reed; revisable once the compiler reports real cases):**
1. Several files claiming one `@X` is a finding. The book is a function: one name, one shard.
2. A shard with no `out` gets a `name?` hole, not a name from its path, so compile reads only its own bytes.
3. `among` stays the topology count. The record of mends whose gauge doesn't close is provisionally named **`twist`** (a shear: invisible from any single line). Alex's naming call is still open; it doesn't block v0.
4. No new comma kinds. Unbound bare name → Dark (no speaker). Ambiguous bare name → Opaque, "which one?". Duplicate claim → default 1. `observed` waits for the Voice.
5. Exits follow the ternary as `mirror love` implements it: success 0, partial 1, fail/red 255, dark 2. The gate is a declared ruler in `mirror.spec` (lean: partial yes; comma, fail and dark no). `fractured`: the spec declares the expected count, and a deviation fails.
6. Numbers: μ marks data positions (exact, `1 ≡ 1.0`, spelling to erasure) and typed positions (the type is kept: an `f64` field prints as a float). Unmarked = exact.
7. Kinds and arity now, types later. Full type checking needs a type system mirror doesn't have yet.

**Still open, for Alex:**
- the final name for `twist`; b₁-as-bugs and b₁-as-tasks;
- C10: fold `keywords.mirror` into its parent, or give it its own name;
- exact coverage (compile row 21) is still Mara's lean, not decided;
- whether gaps are tolerated by the gate ruler (field by field);
- frozen shards (math Prop 3.3′): which shards are frozen (`@std`? other peers' crystals? `boot/` until it collapses?), since that decides whether their line commas are mendable from their side.

**Open, for Seam (round 2):**
- the corrected Prop 2.5 and the twist solve (math Prop 3.6) on cycles plus trees;
- Prop 4.3(b);
- whether `golden_k0.py`'s lexical rule matches `μ_mirror` once `mirror.mirror` exists;
- a free-name oracle independent of the compiler (needed to close K0's bare-name half);
- convergence §4's refutation test.

---

## §11 What this spec does not do

- It predicts no futures (§6).
- It builds no sheaf Laplacian. Interface stalks are typed sets (math §3.1).
- It checks kinds and arity, not types (Alex default 7).
- It edits no shards (§8 is a plan; convergence writes back).
- It writes no Rust. Kintsugi's rules are Datalog in compile §4's engine. Union-find and the gauge solve are the non-Datalog pieces, and they live where the engine lives.
