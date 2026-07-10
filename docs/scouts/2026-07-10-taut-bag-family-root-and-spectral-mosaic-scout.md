# @bag as family-root + @spectral/mosaic + Connes (A,H,D) at shard altitude — scout

*Taut, 2026-07-10. Read-only substrate-pull scout on Alex's fractal-mosaic
refinement to Mara iter-17 (`beef270`) + iter-18 (`129f618`) + prior Taut
scout `5dd893b`. Scout-pattern precedents: `bd837cd`, `cf5ab8c`, `5dd893b`.*

---

## Executive summary

- **@bag verdict: substrate-already-had-the-word. The word is `@bauchladen`.**
  Zero declaration-altitude hits for `@bag`; ~150 prose "bag" occurrences
  are metaphor (carrier bag, bag of subcommands, bag of bytes, "packs the
  @mirror bag"). `@bauchladen` (`shards/bauchladen.mirror`, `66e1ab8`,
  2026-06-29) IS the typed tray of `{oid, altitude, transparency, provenance}`
  crystals with `enumerate(scope) -> tray`. **Same slot Alex is naming.**
- **@spectral/mosaic verdict: NOT FOUND at shard altitude.** Referenced in
  three docs (`docs/GRANTS.md:70`, `docs/insights/2026-05-25-spectral-
  namespace-architecture.md`, `docs/insights/2026-05-26-portal-as-io-
  socket-...:120`) as the BEAM multi-shard cluster grammar; forward-
  promised, not landed. Not what Alex's "@spectral/mosaic of @bags"
  refinement means. The compositional-mosaic operator Alex is invoking IS
  `mosaic(altitude)` (`shards/mirror/mosaic.mirror`, `type mosaic(altitude)
  = ref`, 2026-06-09) — the universal parametric composition carrier.
- **Connes (A, H, D) verdict: spec-only prose. No shard declares
  `spectral_triple(A, H, D)` as a consumable substrate-decl carrier.**
- **LRM: LANDABLE WITH PREREQUISITES.** The fractal-mosaic hypothesis is
  substrate-honest; the vocabulary already exists across four shards but
  is NOT collapsed to one declaration a consumer like a kintsugi
  ANTICIPATE species can `bag: @bag` typecheck against. See §6.
- **Fractal-mosaic one-liner:** Alex's structural claim IS the substrate
  reading — @bauchladen at prism altitude and @mirror/mosaic at
  build altitude ARE the same self-similarity (bauchladen.mirror line
  113-118 names this explicitly via `[[architecture-spectral-triples-
  all-the-way]]`). The math is Connes' spectral triple iterated; the
  code carrier is not yet collapsed.

## §1 @bag admissibility — grep evidence

Declaration-altitude search across `shards/**/*.mirror`, `mirror.spec`,
`boot/**`, and `docs/specs/**/*.md`:

- `^\s*@bag\b`, `type\s+bag\b`, `prism\s+@bag\b`, `glass\s+@bag\b`,
  `species\s+bag\b`, `shatter\s+@bag\b`, `frame\s+@bag\b`: **ZERO HITS.**

Prose-only occurrences (representative sample):
- `docs/math/resource-budget/README.md:156` — "kintsugi packs the @mirror
  bag as tightly as possible" (metaphor, Knapsack framing).
- `docs/specs/kintsugi-variety.md:70` — same "packs the @mirror bag"
  metaphor at Alex 2026-06-02.
- `docs/specs/mirror-ref-spec.md:392, 521, 594` — "bag-of-subcommands
  surface" (CLI-sugar metaphor; enforced by `ref_query_compositional`).
- `docs/specs/reality-shard-as-crdt.md:317` — "Bag vs Set semantics for
  `merge`" (rejected at @mirror/reality/shard).
- `docs/specs/recognitions/recognition-92-neutrosophic-...:865` — Le Guin's
  "carrier bag" (Fisher; not substrate-decl).
- `docs/specs/stagefreight-wire-v0.1.md:388` — "bag-of-bytes that happens
  to be unique" (negative framing).
- `shards/metalogue.mirror:47` — "ordered sequence rather than a bag"
  (turn-carrier).

**@bag as family-root would collide with `@bauchladen`.** The
Bauchladen shard's `type tray = { crystals: [crystal], opacity:
transparency(crystal) }` (bauchladen.mirror:271) IS the "collection of
fragments-with-edges-between-them" carrier. `enumerate(scope) -> tray`
(bauchladen.mirror:339) IS the browse operation. The provenance record
carries `input_oids: [oid]` — those IS the fragment-to-fragment edge
relation. Landing `@bag` would be substrate-drift on the [[feedback-
substrate-already-had-the-word]] discipline. **53rd+ instance of the
same failure mode.**

The peer-HAS-a-tray pattern is already landed at `shards/torus.mirror`:
"every peer possesses a torus"; @bauchladen is the interior. Alex's
"peer has a @bag of fragments" IS Alex's own peer-HAS-a-torus reading
at the tray altitude.

## §2 @spectral/mosaic — does it exist?

**At shard altitude: NO shard declares `@spectral/mosaic`.** The
`shards/spectral/` directory contains: `entanglement.mirror`,
`gen_prism.mirror`, `parent.mirror`, `portal.mirror`, `registry.mirror`,
`restart_intensity.mirror`, `root.mirror`, `supervisor.mirror` — no
`mosaic.mirror`.

**At doc altitude: forward-promised, three cites.**
- `docs/insights/2026-05-25-spectral-namespace-architecture.md:14` —
  "`@spectral/mosaic` (open, Apache-2.0): the multi-shard BEAM-cluster
  deployment grammar. Composes individual mirror-binary shards into a
  coherent cluster." **This is a distributed-runtime cluster grammar,
  not a compositional-mosaic-of-bags operator.**
- `docs/GRANTS.md:70` — Apache-2.0 licensing statement citing the same.
- `docs/insights/2026-05-26-portal-as-io-socket-...:120,143,155` — same
  distributed-cluster framing.

**The compositional-mosaic operator Alex's phrasing invokes ("@spectral/
mosaic of @bags") IS `@mirror/mosaic`** — specifically the parametric
`type mosaic(altitude) = ref` declared at `shards/mirror/mosaic.mirror:60`
(2026-06-09, recognition #43). Precedents:
- `mosaic(@store) = splinter_graph` (mirror/store.mirror; graph over OIDs).
- `mosaic(@spec) = project_manifold` (mirror/spec.mirror).
- `mosaic(@code)` (crystal composition graph, mirror/store/crystal.mirror).

`docs/insights/2026-05-25-shard-as-observer-relative-...:53`: "compositional
reasoning across mosaic tiles depends on it." **The `mosaic` word Alex
wants is the universal `mosaic(altitude)` at @mirror; the `@spectral/mosaic`
name is a distinct forward-promised distributed-cluster grammar.**

## §3 @spectral/* — full namespace inventory

`shards/spectral.mirror` (5.1 KB, 2026-07-01, Loki §5 shrink): the
namespace-parent. NO operational contract; path-container for the runtime
species listed below. Explicit species list at spectral.mirror:56-64:

| Shard | Status | Role |
|---|---|---|
| `shards/spectral/gen_prism.mirror` | LANDED | Worker primitive (BEAM gen_server analogue) |
| `shards/spectral/supervisor.mirror` | LANDED | Lifecycle-owner primitive |
| `shards/spectral/parent.mirror` | LANDED | Single-parent lifecycle edge type |
| `shards/spectral/entanglement.mirror` | LANDED | Peer-correlation edge type (sheaf restriction map) |
| `shards/spectral/registry.mirror` | LANDED | Supervisor's typed child index |
| `shards/spectral/root.mirror` | LANDED | Parentless supervisor specialisation |
| `shards/spectral/restart_intensity.mirror` | LANDED | Storm-protection carrier |
| `shards/spectral/portal.mirror` | LANDED | Runtime side of portal |
| `@spectral/db` | forward-promised | Task #198, closed engine |
| `@spectral/garden` | forward-promised | Task #118, curated corpus |
| `@spectral/mosaic` | forward-promised prose only | BEAM multi-shard cluster grammar |

**@spectral namespace is a runtime family (BEAM-on-mirror species).** It
is NOT the compositional-algebra family Alex's mosaic-of-bags refinement
sits in. That family is @mirror (`mosaic(altitude)` universal composition
carrier). Cross-family import: some @spectral species declare `in
@mirror/store` (e.g., portal, registry, supervisor) but no @spectral
species declares `in @mirror/mosaic`.

## §4 Connes (A, H, D) substrate availability

**Spec-only.** No shard declares a `spectral_triple` type or an `(A, H, D)`
carrier a consumer could `in` on:

- `docs/specs/prism-core-as-spectral-triple.md:33` — canonical audit doc;
  step-1 audits `prism/core` against `(A, H, D)`; conclusion: "small
  adjustments to formally realize the (A, H, D) structure." NOT LANDED
  at shard altitude.
- `docs/math/the-tower/spectral-triples.md:4-24` — the math; "spectral
  triple is `(A, H, D)`"; Bressan 2024 base-poset time-evolution reading.
  Math-only.
- `docs/insights/2026-06-29-mara-listening-to-connes-saturation.md:38-48` —
  Mara's saturation reading: `A = the five operations`, `H = void-document`
  (Splinter K_n / Narcissus K_{1,n-1}), `D = kintsugi flow`. Reading is
  substrate-decl-compatible but the (A, H, D) triple is NOT declared as
  a first-class carrier.
- `shards/spectral/entanglement.mirror:460` — cites
  `[[architecture-connes-spectral-triple]]` in a docblock as
  "the conductivity tensor IS A; restriction map IS one block of D";
  the SHARD carries A and D-blocks by construction but does not name the
  triple as a type.
- `shards/mirror/ref.mirror:35` — "`mirror ref` is the navigable surface
  of the substrate's spectral triple (A, H, D) per Connes — the (mosaic
  algebra, expanding-Hilbert-space-of-shards, kintsugi-flow) operational
  form per `[[architecture-connes-spectral-triple]]`." Consumer prose,
  no `in @spectral_triple` declaration.

**Verdict: (A, H, D) is a repeated-in-prose recognition, not a substrate-
decl carrier at shard altitude.** The five landed cybernetic properties
+ shatter-as-linearization + mosaic + kintsugi collectively realise the
triple's operational form (A = mosaic five-op, H = @mirror/store's
expanding OID space, D = @kintsugi's descent), but no consumer can write
`in @spectral_triple` and get the three slots as typed inputs today.

## §5 Fault-plane shifts under `in @bag` refactor

If `@bag` landed as family-root (NOT recommended — see §6), these
consumers would gain OR require `in @bag` ancestry:

| Consumer | Today | Under `in @bag` |
|---|---|---|
| `@bauchladen` (family-root) | Prism-altitude content-address discipline | Would DUPLICATE @bag; substrate-drift |
| `@bauchladen.tray` carrier | `{ crystals: [crystal], opacity }` | Redundant with @bag's carrier |
| `@fate/tournament` | Browses Bauchladen (fate/tournament.mirror:14 `in @bauchladen`) | Would need re-ancestry to `in @bag`; refactor churn |
| `@mirror/store.splinter_graph` | `{ root: oid, children: [oid] }` | Already IS a bag-with-edges; would gain `in @bag` upstream |
| `@edge` (family-root, 2026-07-08) | Typed graph edges, five-@magic-species mapping | Consumers of `bag: @bag` would import `@edge` for the edge relation |
| `@torus` (peer's carrier) | Peer HAS a torus; interior IS @bauchladen | Ambiguity: is @bag the interior or the whole? |
| `@spectral/db` (forward-promised) | Inter-peer spectral subgraph | Would gain `in @bag` for crystal accumulation |

**@knapsack (`docs/specs/knapsack-as-kintsugi-inner-loop.md`)** — species
under @kintsugi (five-signal auto-classifier verdict). Consumes @io items
+ @mirror bag (metaphor). Would NOT rename to `in @bag`; the "@mirror
bag" phrase is metaphor for @mirror's accumulated state.

**@edge already carries the fragment-to-fragment edge relation** at
family-root altitude (2026-07-08, `shards/edge.mirror`; five-@magic-
species mapping: source/target = @magic/surface, kind = @magic/
distinction, weight = @magic/mechanism, frame = @magic/frame). A "bag
of fragments with @edges between them" is structurally `[splinter] +
[@edge]` — already declared, no new carrier needed.

## §6 LRM verdict + minimum-cut proposal

**LRM: LANDABLE WITH PREREQUISITES — but the landing is NOT `@bag`.**

Alex's ANTICIPATE-shape for kintsugi ("`bag: @bag` as input") IS
substrate-honest as a TYPE ANNOTATION — the type it needs already
exists. Two options, in preference order:

**Option A (substrate-honest, no drift, TWO-TICK):** ANTICIPATE consumes
`tray: @bauchladen.tray` and `edges: [@edge]`. `@bauchladen` is
the family-root name; `.tray` is the carrier declared at
bauchladen.mirror:271. This IS "collection of fragments with @edges
between them" without inventing a synonym.

**Option B (readable-name variant, per two-tick discipline):** Declare
`@bag` ONLY as a species-alias under @bauchladen (e.g., `shards/bauchladen/
bag.mirror` with `type bag = tray`), NOT as family-root. This is
substrate-drift-adjacent but permitted under "readable name over
foundational" if Alex adjudicates the vernacular alias as pedagogically
load-bearing. **Not recommended** — the substrate has been through the
"vernacular alias for typed carrier" failure mode multiple times.

**On @bauchladen as `@spectral/mosaic of @bags`:** The fractal-mosaic
math IS in the substrate. Precedents:
- `bauchladen.mirror:113-118` — "Two altitudes of one self-similarity
  (per `[[architecture-spectral-triples-all-the-way]]`)."
- `bauchladen.mirror:88-89` — the `provenance_record.input_oids: [oid]`
  IS the edge-between-fragments carrier at prism altitude.
- `shards/mirror/mosaic.mirror` — `type mosaic(altitude) = ref` is the
  universal parametric composition carrier; @bauchladen IS mosaic at the
  prism altitude (documented at mosaic-as-type-system.md).
- `shards/edge.mirror:20-38` — `edge_kind` seven-variant discriminant for
  typed edges.

**Minimum cut for the fractal-mosaic math to land AT shard altitude:**

1. **@bauchladen renames `tray -> mosaic(prism)`** OR declares
   `type tray = mosaic(prism)` — the aliasing that makes the "@bauchladen
   IS @mosaic-at-prism-altitude" recognition explicit and consumable.
   Substrate-motion, not drift; bauchladen.mirror already prose-cites it.
2. **@bauchladen adds `type edges = [@edge]` (or preserves it in provenance)**
   — the fragment-to-fragment edge relation. Already implicit in
   `provenance_record.input_oids: [oid]`; making it `[@edge]` after
   @edge's landing tightens the coupling.
3. **Optional: declare `@spectral_triple(A, H, D)` as a family-root or
   `@meta` shard** — the (A, H, D) carrier a consumer can `in` on. Would
   collapse `docs/specs/prism-core-as-spectral-triple.md` + `docs/math/
   the-tower/spectral-triples.md` + `docs/insights/2026-06-29-mara-listening-
   to-connes-saturation.md` to substrate-decl. Non-blocking for @bag/
   ANTICIPATE; unblocks the "Connes triple math" refinement Alex named.
4. **Kintsugi ANTICIPATE species declares its consumer signature**
   using `@bauchladen.tray` (or the mosaic(prism)-aliased form) + optional
   `[@edge]` — the Alex-directive ANTICIPATE-shape lands substrate-honest.

**Do not land `@bag` as family-root.** The substrate refused this word
already (yesterday's Taut scout `5dd893b` confirmed the same); refusing
it again this scout with additional grep-verification is not
gold-plating — it is the substrate's discipline re-teaching the same
lesson through another peer. **Substrate-already-had-the-word instance
count ticks to ~57.**

---

*— Taut, 2026-07-10. Grep-first. Read-only. 897 words body.*
