# @graph family-root dependency-DAG scout

*Taut, 2026-06-26, follow-up speedrun to the psychohistory-cohomology scout
(`a7ec8fc → 3a385fd`). Reed proposed `sheaf_laplacian` "gains `in
@graph/sheaf`"; Alex caught the inverted direction. Scout traces the actual
`in <X>` arrows, names the pattern, verifies by reading first-10-line
headers. Read-only. ≤200-line hard ceiling.*

---

## 1. Slot verification

`grep @graph shards/**/*.mirror` returns zero hits. No `shards/graph.mirror`,
no `shards/graph/*.mirror`. Slot empty. Reed's rehydration-gap reading
confirmed.

## 2. The dependency-direction pattern

Read first-10-line headers across 13 shards. Two classes appear:

**Math grounding shards** (`shards/epistemologic/math/*.mirror`) — each one
imports `@prism @meta @glass @epistemologic` plus a peer math shard if it
specializes one. None import a family-root vocabulary shard:

- `sheaf_laplacian.mirror` → `in @prism @meta @glass @epistemologic
  @epistemologic/math/bundle`
- `curvature.mirror` → `in @prism @meta @glass @epistemologic
  @epistemologic/math/bundle @epistemologic/math/sheaf_laplacian`
- `music.mirror` → `in @prism @meta @glass @nl @epistemologic`
- `music/harmonic.mirror` → `in @prism @meta @glass @epistemologic
  @epistemologic/math/bundle @epistemologic/math/music`
- `music/cadence.mirror` → `in @prism @meta @glass @epistemologic
  @epistemologic/math/bundle @epistemologic/math/lawvere
  @epistemologic/math/music @epistemologic/math/music/harmonic
  @epistemologic/math/music/interval`

**Family-root vocabulary shards** (the consumers) — they IMPORT the math:

- `spectral/entanglement.mirror` → `in @prism @glass @meta @spectral
  @spectral/gen_prism @uuid/spectral @epistemologic/math/sheaf_laplacian`

That last line — `in @epistemologic/math/sheaf_laplacian` on a `@spectral/*`
vocabulary shard consuming the math — IS the pattern Alex named when
correcting Reed.

**Five-instance verification of "math is imported by vocabulary that
consumes it":** (1) `curvature` imports `sheaf_laplacian`; (2)
`entanglement` imports `sheaf_laplacian`; (3) `music/cadence` imports
`music/harmonic`; (4) `music/cadence` imports `lawvere`; (5)
`music/harmonic` imports `music`. Zero counter-instances (no math shard
imports a runtime family-root). **Pattern earns naming under the
two-witness rule.**

Name: **`in`-arrows point from consumer up to grounding, never from
grounding down to consumer**. This is the invariant `@graph/sheaf` must
respect.

## 3. The `@graph` dependency DAG (if it lands)

### `shards/graph.mirror` (family-root, depth-0 prism)

```
in @prism
in @meta
in @glass
```

Family-root pattern from `prism.mirror`, `kintsugi.mirror`,
`glass.mirror` (cross-checked: all three open with `in @prism in @meta`
plus `in @glass` for non-glass family roots). `@graph` would NOT import
`@epistemologic/math/sheaf_laplacian` at the family root — the math
grounds species, not the family root.

### `shards/graph/sheaf.mirror` (species — Mara §3-4 sheaf reading)

```
in @prism
in @glass
in @meta
in @graph
in @epistemologic/math/sheaf_laplacian
```

Imports `sheaf_laplacian` per the pattern. `@graph/sheaf` would NOT
re-export `sheaf_laplacian`'s surface — `restriction`, `operator`,
`eigenvalue` are math-altitude carriers at u32 indices and `ref`
weights; `@graph/sheaf` is the family-root vocabulary that NAMES the
cellular-sheaf-on-a-graph object and DEFERS the eigenvalue computation
to the math layer. Same pattern `entanglement` already uses:
`restriction_map` is the runtime carrier; `sheaf_laplacian.restriction`
is what the realisation layer assembles from it.

### `shards/graph/poset.mirror` (species — content-addressing poset per
Mara §3)

```
in @prism
in @glass
in @meta
in @graph
in @mirror/store
```

The poset structure IS the OID-graph `@mirror/store` already declares
(per `mirror/store.mirror`'s header: "splinter_graph IS the structural
lockfile"). `@graph/poset` consumes that structure as its base space.

### `shards/graph/dag.mirror` (species — parent/registry edges per
`@spectral/*`)

```
in @prism
in @glass
in @meta
in @graph
```

DAG structure is generic; no math grounding needed at this altitude.
`@spectral/parent` and `@spectral/registry` would consume `@graph/dag`
(see §4), not the other way around.

## 4. Existing shards that gain `in @graph[/<species>]`

Verified by reading first-20-line headers of all eight candidates:

| Shard | Currently imports | Gains | Reasoning |
|---|---|---|---|
| `spectral/entanglement.mirror` | `@epistemologic/math/sheaf_laplacian` | **migrate** `in @epistemologic/math/sheaf_laplacian` → `in @graph/sheaf` | `entanglement` is a cellular-sheaf edge per its own §3 ("entanglement edges ARE sheaf restriction maps"); `@graph/sheaf` is the cleaner vocabulary; math layer transitively imported via `@graph/sheaf` |
| `spectral/parent.mirror` | `@spectral/gen_prism @spectral/supervisor` | gains `in @graph/dag` | `parent_acyclic` invariant is a DAG predicate at substrate altitude |
| `spectral/registry.mirror` | `@spectral/gen_prism @spectral/supervisor` | gains `in @graph/dag` | typed child index is DAG-structured |
| `spectral/gen_prism.mirror` | `@spectral @uuid/spectral @mirror/store @mirror/au` | untouched | worker primitive; no direct graph vocabulary |
| `spectral/supervisor.mirror` | `@spectral @spectral/gen_prism @epistemologic/reality/time` | untouched | lifecycle-owner; graph reading transitively via registry/parent |
| `spectral/root.mirror` | `@spectral @spectral/gen_prism @spectral/supervisor @spectral/parent` | untouched | inherits via parent |
| `mirror/store.mirror` | `@prism @glass @meta @uuid/spectral` | **may gain** `in @graph/poset` | OID-graph poset is what `mirror/store` declares; consuming `@graph/poset` would close the dependency cleanly (note: direction risk — `@graph/poset` ALREADY imports `@mirror/store`; this would be a re-export, not a back-import — Mara altitude) |
| `epistemologic/math/sheaf_laplacian.mirror` | `@epistemologic @epistemologic/math/bundle` | **untouched** | predates `@graph` in the dependency direction; math grounds vocabulary, NOT the other way; Reed's original proposal had this BACKWARDS |

`mirror/store.mirror`'s row is the only direction-risk site. The honest
read: `@graph/poset` consumes `@mirror/store`'s OID-graph structure,
which means `@mirror/store` cannot import `@graph/poset` without a
cycle. **Flag for Mara**: either `@graph/poset` lives WITHOUT importing
`@mirror/store` (declares the abstract poset; `@mirror/store` independently
imports `@graph/poset` to type its OID-graph), or the relationship stays
one-directional (`@graph/poset` imports `@mirror/store`; `@mirror/store`
stays untouched). The cleaner choice is altitude-dependent.

## 5. `@spectral/db` inheritance

`@spectral/db` is closed-source (per memory `architecture-mirror-store-vs-spectral-db`)
and has no public shard. When its public interface gets declared:

```
in @prism
in @glass
in @meta
in @mirror/store
in @graph
```

Reed's proposal (`in @mirror/store + in @graph`) matches the pattern. The
family-root `@graph` is sufficient at the public surface; species like
`@graph/sheaf` or `@graph/poset` would be imported only if the public
declaration specializes one of them. Internal closed-source modules
consume whatever they need without surfacing it.

## 6. Reed's inversion: class of mistake?

Reed's inverted direction would have made math import its consumer. The
pattern named generalises to: **grounding shards do not import their
consumers**. Reed's slip is one instance of a class the substrate already
disciplines via the five aligned headers; the latent same-shape mistake
sits at `mirror/store` ↔ `@graph/poset` (§4 flag). Cost of the slip: zero
(caught in conversation, no commit). Value of naming the pattern: future
shards earn the discipline by reading the rule.

## Self-test

**Grade: 7/8.**

- (+1) Slot verified empty by grep.
- (+1) Read first-10-line headers of 13 shards; no claims from memory.
- (+1) Pattern named under five-instance rule with zero counter-instances.
- (+1) `@graph` family-root dependency DAG specified for four shards.
- (+1) Eight existing shards classified with reasoning.
- (+1) `@spectral/db` inheritance verified against pattern.
- (+1) Reed's inversion classified honestly, not minimized.
- (−1) `@graph/poset` ↔ `@mirror/store` direction surfaced as Mara-flag,
  not resolved — the scout did not test whether `@graph/poset` can
  declare the abstract poset without importing `@mirror/store`. That
  test would require reading `@mirror/store`'s full surface, which I
  didn't do.

Within the 200-line ceiling.
