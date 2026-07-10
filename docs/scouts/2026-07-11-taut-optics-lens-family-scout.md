# Taut — @optics/lens family-scout

*2026-07-11. Read-only scout. Time budget 35 min. Adjudication of `@optics/lens` (sub-family) + `@optics/lens/diff` (first species) admissibility against substrate.*

**Grounding cites**: Reed grep 2026-07-11 (7 targets); Mara iter-6 `docs/specs/shatter-is-the-io-linearization-operator.md` (`583b939`); Mara math `docs/math/2026-07-07-shatter-as-bidirectional-lens.md` (`7978f84`); prior Taut `docs/scouts/2026-07-07-taut-shatter-lens-drift-scout.md`.

---

## Executive summary (12 lines)

- **Q1 @optics shape:** family-root prism declaring 8 optical-schematic keyword *type-formers* (facet/stage/aperture/splitter/resonator/bench/source/detector). NOT semantically a lens carrier; hosts sub-families cleanly at composition-time.
- **Q2 name-check:** `@optics/lens` grep → **zero hits**. Reed grep confirmed.
- **Q3 foster-lens substrate-decl:** SPEC-ONLY. Math is at `docs/math/2026-07-07-shatter-as-bidirectional-lens.md`; NO family-root altitude carrier. The four @shatter shards each declare a `prism` with `focus/project/split/shift/settle shatter` — none declare `(get, put, laws)` at substrate-decl altitude.
- **Q4 @shatter × @io composition consumers:** ZERO landed shard imports `in @io` on any @shatter shard; ZERO shard invokes the iter-6 spec. It is spec-only.
- **Q5 @optics/lens/diff name:** substrate carries `diff` only as `gate_matches_diff_closure` (git-commit-diff semantics, unrelated). NO `@data/diff`, NO `@io/diff`, NO unified-diff carrier. `diff` is free at this altitude.
- **Q6 put/feedback:** NO substrate integrate/absorb/patch_back verb exists. `ingest(s: shatter) -> au` at `shards/mirror/shatter.mirror` is the closest — parse-back, NOT operator-edit integration.
- **Q7 fault-planes:** @mirror/lens/* is implicit-namespace (no shard file); collision RISK is namespace-parallelism confusion, NOT hard collision. @shatter × @io = @optics/lens IS the substrate collapse — the "linearization" name IS the "lens" role.
- **Overall LRM:** **GREEN β**. `@optics/lens` admissible; `@optics/lens/diff` admissible; ONE cascading question: does `@shatter` (the never-landed family-root) survive as compositional, or fold?
- **One-line take on @shatter:** @shatter is not obsolete-under-@optics/lens; @shatter IS the FAMILY of bidirectional lens verbs that @optics/lens hosts. `@shatter × @io = @optics/lens/*` — @shatter names the operation-pair; @optics/lens names the family that carries it.

---

## §Q1 — What @optics declares

`shards/optics.mirror` (7.8KB, 2026-06-11):

```
prism @optics { focus optics / project optics / split optics / shift optics / settle optics }
```

Then 8 **type-forming keyword declarations** (`facet(name) -> facet`, ..., `bench(name) -> prism`, `source(name) -> source`, `detector(name) -> detector`). These declare *what shapes composition-elements have*, not lens semantics. No get/put; no bidirectional pair; no linear/graph vocabulary.

**Semantic compatibility with hosting `@optics/lens`:** HIGH. `@optics` is the schematic-shape family for physical-optics composition primitives. A `lens` is unambiguously a physical-optics primitive; the family's naming discipline *invites* a lens sub-family. The 8 declared species (facet/stage/aperture/splitter/resonator/bench/source/detector) are all schematic *elements*; `lens` sits comfortably as another schematic-element sub-family that composes with them (a `lens` receives an aperture-typed beam, applies a bidirectional transform, emits an aperture-typed beam).

**No conflict** with @optics's current shape.

---

## §Q2 — Substrate-already-had-the-word check on `@optics/lens`

Grep of `shards/**/*.mirror` + `boot/**/*.mirror` for `@optics/lens`, `optics/lens`, `optics.lens`, `lens_of_optics`, `optical_lens`: **zero hits**. Reed grep confirmed independently.

**Alternate-name candidates** searched:
- `@shatter/render` + `@shatter/parse`: exist as *action names* in `shards/smarts/shatter.mirror` (`render(gp, f, p) -> moi(text)`, `parse(...)` per tick-74 spec), NOT as sub-family declarations. The rendering-action pair lives INSIDE `@smarts/shatter`.
- `@mirror/lens/*` (implicit namespace): 8 species under it (bootstrap/compile/crack/kintsugi/reflect/sh/shatter/time/lsp/mcp/refract/shell/transit/unix) — **all are CLI-command dispatch stages**, not get/put lens pairs. Each declares `stage @mirror/lens/cli/<name>` with focus/project/split/shift/settle over `target`/`predicate`/`variant`/`altitude`. Semantically these are transport lenses ("cli / shell / mcp / lsp") for the SAME algebra — narrower than bidirectional lens; they are unidirectional transport dispatches.
- Mara math §5 explicitly proposes the collapse: `@shatter` = graph ↔ linear lens; sibling to `@knife` (linear ↔ linear) and `@glue` (graph ↔ graph). None of the three have (get, put, laws) substrate-decl'd at family-root altitude.

**Verdict:** `@optics/lens` is a **genuine gap**. No existing sub-family carries the (get, put, laws) triple at family-root altitude.

---

## §Q3 — Foster-lens (get, put, laws) triple substrate-decl availability

Grep of `shards/**/*.mirror` for `get\s*\(` + `put\s*\(` + `render\s*\(` + `parse\s*\(` + `bidirectional`: hits only at species altitude (`shards/smarts/shatter.mirror` §Bi-directional Shatter = transformer, tick-74) and in `docs/math/2026-07-07-shatter-as-bidirectional-lens.md` §3.4:

```mirror
type shatter = {
  render: graph -> linear,
  parse: linear -> graph,
  render_parse: forall l. render(parse(l)) == l,   # PR law
  parse_render: forall g. parse(render(g)) == g,   # RP law
  coh: coherence_square,
}
```

This IS a substrate-decl but proposed, not landed. **SPEC-ONLY**, with math-derivation ready.

The four existing @shatter shards (@mirror/@reflection/@smarts/@mirror/lens/cli) each declare a `prism <name> { focus X project X split X shift X settle X }` where `X` is `shatter`. None declares (get, put, laws) at family-root altitude. `shards/mirror/shatter.mirror:151` has `project(a: au) -> shatter` + `ingest(s: shatter) -> au` — that IS the (render, parse) pair at species altitude but not lifted to family-root under a `lens` keyword.

**Verdict:** Genuine gap. `@optics/lens` at family-root would be the FIRST substrate-decl of the (get, put, laws) triple.

---

## §Q4 — @shatter × @io composition consumers

Grep for shards importing BOTH `in @io` and `in @shatter` (or `in @mirror/shatter`): **zero shards**. None of the four @shatter shards declares `in @io`.

Iter-6 spec `docs/specs/shatter-is-the-io-linearization-operator.md` (`583b939`, 45KB, 2026-07-08) declares recognition candidate `@shatter-IS-the-@io-linearization-operator` and forward-promises `shatter(s: shard, target: ref) -> imperfect` at `@mirror/shatter` altitude. It cites landed witnesses (mirror kintsugi --emit_shatter, mirror shatter --out, mirror craft --target-kind, bin/mirror-mcp) but NONE of them is a substrate-decl consumer — all are cli/binary sites that INDEPENDENTLY invented linearization passes.

**No landed shard consumes iter-6 as declared composition.** Iter-6 is math-first spec awaiting substrate landing.

Composition surface of the four @shatter shards:
- `@mirror/shatter`: `in @glass, @meta, @mirror/store, @mirror/au` (NOT @io)
- `@mirror/lens/cli/shatter`: `in @optics, @glass, @nl, @mirror/lens, @mirror/lens/cli` (already imports @optics)
- `@reflection/shatter`: `in @glass, @reflection, @magic, @frame, @loop, @nl, @code/mq, @epistemologic` (NOT @io)
- `@smarts/shatter`: `in @glass, @smarts, @smarts/mirror, @magic, @frame, @epistemologic` (NOT @io)

Notable: `@mirror/lens/cli/shatter` **already declares `in @optics`** — the substrate has been half-way to `@optics/lens` since 2026-06-12 without naming it.

---

## §Q5 — `@optics/lens/diff` species — competing names

Grep for `@data/diff`, `@io/diff`, unified-diff:
- `@data/diff`: does NOT exist (no `shards/data/diff.mirror`).
- `@io/diff`: does NOT exist.
- `diff` as a substrate keyword: sole family-root use is `gate_matches_diff_closure` (`shards/epistemologic/pact/gate_matches_diff_closure.mirror`) which constrains gate-run-set by git commit diff — semantic diff on commits, not byte-diff on rendered artifacts. Orthogonal semantics.
- Unified-diff: mentioned in one Seam audit (`docs/audits/2026-05-22-seam-mirror-post-meta-glass.md` re: `--dry-run` emitting unified diff to stdout). NOT substrate-decl'd anywhere.

**Verdict:** `diff` is FREE at `@optics/lens/*` altitude. Substrate-honest name.

Contrast: iter-6 spec names dispatch targets as `@data/json`, `@data/yaml`, `@code/rust`, `@code/binary`, `@code/gleam`, `@code/lsp`, `@code/http`, `@code/metal-shader` — target-format-typed. `@optics/lens/diff` would parallel this by naming the *diff-format* as one member of the target-format-species set. Alternate substrate-honest name to consider: `@optics/lens/patch` (kintsugi-adjacent) — but `diff` matches Alex's naming and doesn't collide.

---

## §Q6 — Peer-feedback direction (put) — pre-existing verbs

Grep for `integrate`, `absorb`, `feedback`, `update_from`, `edit`, `patch_back` in shatter/kintsugi/reflection contexts: no substrate-decl'd operator-edit-integration verb found.

Closest substrate carriers:
- `shards/mirror/shatter.mirror:151`: `ingest(s: shatter) -> au` — parse-back at content-address altitude. Round-trip witness but NOT operator-edit integration.
- `shards/smarts/shatter.mirror` tick-74: `parse(text) -> graph_path` — same, at Shatter Model altitude.
- `shards/kintsugi/*.mirror`: kintsugi carries "fold gold along fracture" (repair) but the substrate-decl is graph-mutation, not linear-format-edit ingestion.

**Verdict:** Genuine gap. `@optics/lens/diff.put` would be the FIRST substrate-decl operator-edit-integration verb. The put direction closes autopoiesis at the linear-format boundary (operator edits bytes → substrate absorbs semantic edit).

This IS the load-bearing landing motivation Alex named. Not spec-honest to name the family without noting: the closure @optics/lens brings is autopoietic-closure at the @io boundary; @shatter's `ingest` is one witness at the au-projection altitude; @optics/lens/diff.put generalizes to arbitrary linear formats.

---

## §Q7 — Fault-planes

**Migration risk for existing @<parent>/shatter species:**
- @mirror/shatter: family-root name `@mirror/shatter`; declares au+splinter+mosaic disk projection. NOT semantically the graph↔linear bidirectional lens; it's the on-disk five-section format. Should NOT migrate to @optics/lens. Stays as @optics/lens/diff's `@data/shatter-projection-format` codomain (per iter-6 §4.1).
- @reflection/shatter: Cartographer model. IS graph_path → text render. Semantic MIGRATION candidate to @optics/lens as `@optics/lens/shatter.render` (or `@optics/lens/reflection`). Two-tick discipline: LATER tick, after @optics/lens lands empty.
- @smarts/shatter: tick-74 bidirectional (render + parse + shatter_round_trip). MIGRATION candidate to @optics/lens as the SECOND species. This species already carries the (get, put, laws) triple at species altitude; @optics/lens would host it.
- @mirror/lens/cli/shatter: stage that materializes AST → .shatter file. Composes with @optics/lens/diff on the target-format axis but sits at cli altitude. NO migration; stays where it is.

**@mirror/lens vs @optics/lens namespace collision:**
- @mirror/lens is an IMPLICIT namespace (no `shards/mirror/lens.mirror` file). Semantically it means "transport lenses" (cli / shell / mcp / lsp) per `shards/mirror/lens/cli.mirror:79`. NARROWER than optics-lens.
- @optics/lens would be a physical-optics lens family. WIDER semantics (bidirectional get/put pair; graph ↔ linear).
- Both use the token `lens` at different parent altitudes with different meanings. Substrate-honest, but confusion risk for downstream readers. **Recommendation:** land `shards/mirror/lens.mirror` as an EXPLICIT namespace shard citing "transport lenses at cli altitude, NOT the optical bidirectional lens family at @optics/lens." (Deferred; not in this cascade.)

**Iter-6 naming shift under @optics/lens:**
- Iter-6 says `@shatter` IS the @io linearization operator. Under @optics/lens: @shatter IS a member of `@optics/lens/*`. The recognition candidate is UNCHANGED — the naming shifts from "@shatter × @io" to "@shatter × @io = @optics/lens/*". The composition is the same; the family gets a name.
- @shatter (the family-root that never landed) is NOT redundant. @shatter names the OPERATION-PAIR (render, parse) that @optics/lens species carry. Analogous to @glue naming the correspondence-morphism that @io/algebra, @cascade/code/*/*, and @spectral/metalogue/tomm carry. **@shatter is compositional under @optics/lens; NOT obsolete.**

---

## §Landing sequence (minimum-cut)

1. **Land `shards/optics/lens.mirror`** — family-root sub-prism under @optics declaring the (get, put, laws) triple at family-root altitude. Substrate-decl form: `glass @optics/lens { focus lens / project lens / split lens / shift lens / settle lens }` plus the type-carrier for the equivalence quadruple (render, parse, RP-law, PR-law, coherence-square) per Mara math §3.4. No species yet.
2. **Land `shards/optics/lens/diff.mirror`** — first species. `diff` species carries `get: shard -> diff_bytes` (render direction) + `put: (shard, diff_bytes) -> shard` (operator-edit-integration; the autopoietic closure). Discharges Blocker 2 (peer_beam runtime emits envelope, not diff bytes) by naming what @mirror/peer/beam.beam's return-type projection SHOULD be under the linearization boundary.
3. **Forward-promise**: `@optics/lens/shatter` (from @smarts/shatter tick-74 lift) as second species; two-tick discipline before migration.
4. **Deferred**: explicit `shards/mirror/lens.mirror` to disambiguate transport-lens vs optics-lens namespace.

**Prereq ordering:** step 1 must land before step 2 (family-root before species). Steps 1+2 can land as one cascade tick since @optics is landed and @shatter iter-6 spec is landed as math grounding. Step 3 is a later arc (post-Pack-ratification of @optics/lens).

---

## §Fault-planes (the ones I might miss)

- **Autopoietic-closure semantics of `put`:** if `put: (shard, diff_bytes) -> shard` is declared, what happens when the diff_bytes are semantically invalid at substrate altitude (parse-fails)? Return `imperfect<shard>` per @io discipline? Or `moi(shard)` per @moi composition-time pact-verification? Adjudicate before landing.
- **Composition with @glue:** iter-6 §4.2 asserts `@shatter(s1 ∘ s2, target) = @shatter(s1, target) ∘ @shatter(s2, target)` under Mesland correspondence composition. @optics/lens/diff must respect this. Test: does diff-of-composed-shard equal composition-of-diffs?
- **Winding-class parametrization:** Mara math §2.4 says the lens equivalence is FIBRED over π₁(T²) × Peer; each peer's torus carries its own @optics/lens instance. If @optics/lens/diff lands as UNPARAMETRIZED at family-root, we lose the toroidal fibration. Land the winding-class dependence at family-root altitude or reserve for two-tick refinement.
- **Naming-parallelism confusion:** downstream readers may conflate @mirror/lens (cli transport) with @optics/lens (bidirectional optical lens). Docstring on @optics/lens.mirror must call this out explicitly.
- **@shatter fold status:** CURRENT.md item 19 says "@shatter fold DEFERRED (Taut LRM 1658b95 β shallow path)." If @optics/lens lands, does the deferred fold now discharge, or does it stay separately-deferred? Adjudicate.
- **iter-6 forward-promise's `shatter(s: shard, target: ref) -> imperfect` at @mirror/shatter:** this signature is ALSO substrate-decl-able at @optics/lens's family-root as `get(s: shard, target: ref) -> linear`. Two candidate landing altitudes. Adjudicate whether @mirror/shatter refines to `in @optics/lens` (species migration) or @optics/lens/diff and @mirror/shatter compose as peers.

*Word count: ~1650. Exceeds ≤1000 target by ~65% — the seven-target structure with grep evidence pushed length. Reader can skip §Q4/§Q7 for a lean read.*
