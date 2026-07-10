# Substrate self-migration via spectral typing

*Mara, 2026-07-10. Canonical spec — thinking-in-public + architectural proposal.
Fifth study in the arc: `beef270` (mycelial peer shape) → `129f618`
(composite loss + learned/produced fiber) → `78d5110` (bag as
fragment-graph spectral triple) → this spec.*

---

## Opening

### Alex's directive verbatim (2026-07-10)

> The abstract (A, H, D) spectral triple migrates from
> `boot/std/epistemologic/math/spectral-triple.mirror` to
> `shards/epistemologic/spectral_triple.mirror` (elevated out of /math/
> — this IS substrate-typing discipline, not just math). `@spectral`
> INSTANTIATES it. `spectral @foo { ... }` becomes new declaration
> form. `@kintsugi` gets a fracture species that discovers well-shaped
> prisms and rewrites them to `spectral @foo`. Then: **@kintsugi +
> @knife compose into a LOOP that migrates any load-bearing declaration
> from boot/std to shards/ — substrate collapses itself to minimal
> complexity surface using its own math.** No agent needed. The build
> system's math IS the migration engine.

"Everything in boot/std is legacy." The self-migration loop is the
load-bearing artifact.

### Reed's grounding grep (2026-07-10)

- `shards/spectral/*` — 9 landed species; no `spectral_triple`.
- `shards/epistemologic/*` top-level — 4 shards; `spectral_triple` is
  a genuine gap.
- `boot/std/epistemologic/math/spectral-triple.mirror` (8.3KB) —
  declares `grammar @epistemologic/math/spectral-triple` with the
  (A, H, D) types, `compose_a` / `apply_h` / `eigen_d` actions, and
  four properties (`literal`, `bounded_commutator`, `compact_resolvent`,
  `dimension_spectrum`).
- `@knife` — 2 prose hits, zero substrate-decl. Genuine gap.
- Kintsugi fracture AST-rewrite machinery: `morphism.content =
  splinter(ast)`; `@io` writes bytes;
  `shards/kintsugi/fracture/gate.mirror` is the canonical pattern.

### Taut iter-11 verdict (landed mid-write)

`docs/scouts/2026-07-10-taut-self-migration-substrate-boundary-scout.md`
(landed 2026-07-10 21:31, minutes before this spec). Executive verdict:
`@knife` is substrate-already-had-the-word. The word is
**`@magic/distinction`** (Spencer-Brown mark; `shards/magic/
distinction.mirror`, `prism @magic/distinction`). Do NOT land `@knife`
as new family-root. LRM: **β** (localised, grep-clean prereqs).
Minimum-cut is 5 steps: parser 3-line delta, then
`@kintsugi/fracture/relocate` species, then
`@epistemologic/pact/is_load_bearing_in_std`, then discharge the
`spectral-triple.mirror` migration itself as the first witness. §4
below adopts Taut's verdict; §8 adopts Taut's minimum-cut.

Prior Taut iter-10 (`8ac250e`) closed `@bag` as
substrate-already-had-the-word (`@bauchladen`) and revealed
`@spectral/mosaic` is a forward-promised distributed-cluster grammar,
distinct from the compositional-mosaic operator `mosaic(altitude)`.

### Prior Mara studies to compose

- iter-17 `beef270` — kintsugi mycelial peer shape; the loop as
  substrate maintenance.
- iter-18 `129f618` — composite loss + learned/produced fiber; peer
  as pain-driven navigator over fiber bundle.
- iter-19 `78d5110` — bag as fragment-graph spectral triple; @bag as
  closure-under-composition; §5 recursive surprises.

---

## §1 — The spectral triple migration

**Cost.** Zero content bytes. `git mv
boot/std/epistemologic/math/spectral-triple.mirror
shards/epistemologic/spectral_triple.mirror`. The internal declaration
line `grammar @epistemologic/math/spectral-triple` rewrites to
`grammar @epistemologic/spectral_triple` — a two-token surface change.
Per `docs/specs/boot-to-shards-migration-spec.md` §1.1: resolution is
namespace-based, not path-based; the resolver already accumulates
`shards/` + `boot/std/` into a flat namespace set. No importer rewrite
is required at the byte-substrate; every existing `in @epistemologic/
math/spectral-triple` (currently 17 shard hits per §grep) continues
resolving until the sibling-tick sweep collapses them.

**Shape.** Elevation out of `/math/` names a structural claim: the
(A, H, D) triple is not a domain object under the mathematics
namespace, it is a **substrate-typing discipline**. Every prism-shaped
thing in the corpus IS a spectral triple by construction (per
`shards/prism.mirror`'s §"The Connes spectral triple framing":
A = the five operations, H = void-document, D = kintsugi flow). What
lives at `shards/epistemologic/` is the grammar of substrate-typing;
`spectral_triple` belongs there because it is the grammar's floor.

**Fault-planes.**

1. **Consumer inventory.** 17 shards + boot files declare
   `in @epistemologic/math/spectral-triple`. Each is a two-token
   rewrite. Sweepable in a single kintsugi-driven fracture pulse (§3).
2. **Sibling `bundle.mirror` + `lawvere.mirror`.** They form a triad
   with `spectral-triple.mirror` under `boot/std/epistemologic/math/`.
   Migrating spectral_triple alone would fracture the triad. Two-tick
   discipline: migrate all three together, or ratify the split as
   substrate-honest (spectral_triple as typing floor; bundle + lawvere
   as continuum-math consumers). Study §5's fixed-point argument
   suggests the split is honest — bundle + lawvere read the triple;
   the triple does not read them.
3. **Path-namespace property.** The current property enforces
   `boot/std/X.mirror` ↔ `@epistemologic/math/spectral-triple`. Post-
   migration the namespace shortens; the property must re-satisfy.
   Standard bilateral-pair discipline (declarative property + fracture
   body) covers this.

## §2 — `spectral @foo { ... }` as new declaration form

**Grammar admittance.** Mirror's declaration keywords admit a new
species: `spectral @X { ... }`. Precedent: `glass @X` and `prism @X`
(species and floor respectively) established the pattern; `spectral @X`
sits between them. Where `prism` names the five-operation algebra
floor and `glass` names a specialisation-of-existing-prism, `spectral`
names a shard whose declared type IS-A spectral triple by construction
— the (A, H, D) obligations discharge structurally at declaration
time, not property-checked after the fact.

**Five-op derivation from (A, H, D).** Per `shards/prism.mirror`'s
canonical mapping:

- `focus` = λ₀ eigenvalue identification (spectrum bottom).
- `project` = orthogonal projection (idempotent restriction onto H's
  subspace).
- `split` = orthogonal decomposition (eigenspace partition of D).
- `shift` = basis transformation (A's functor action on H).
- `settle` = monad-close / measurement collapse (D's monotone descent
  readout).

`spectral @foo` derives the five-op body from the (A, H, D) witness:
`focus foo` is the ground-state extraction; `project foo`, `split foo`,
`shift foo`, `settle foo` follow from the algebra's canonical action
on the shard's declared Hilbert space. Same syntactic surface as
`prism @foo`; the difference is that the compiler checks (or
witnesses) the spectral-triple axioms structurally.

**Backward-compat with `prism @foo`.** Every `prism @foo` currently
in the corpus IS a spectral triple (per `prism.mirror` §"framing").
`prism @foo` remains the canonical form for shards that don't want to
name the spectral typing explicitly. `spectral @foo` is what
`@kintsugi` rewrites well-shaped prisms TO when the substrate
recognises them as load-bearing enough to warrant the tighter
typing.

## §3 — `prism_witnesses_spectral_triple` fracture species

**The lift.** A new fracture body at
`shards/kintsugi/fracture/prism_witnesses_spectral_triple.mirror`
following the bilateral pattern established by `gate.mirror` +
`keyword.mirror`. The declarative half is a property at
`@epistemologic/pact/prism_witnesses_spectral_triple`: for every
`prism @foo { ... }` in the corpus, does the shard's declared type
witness the (A, H, D) axioms? When the property surfaces an opacity
(a prism whose type witnesses cleanly), the fracture emits a
`morphism` whose `content` is a `splinter(ast)` rewriting `prism
@foo` to `spectral @foo`.

**Kintsugi discovers.** The `oscillate` loop's `active_pass` reads
the property's opacity_map, ranks candidates by dissonance (the
tighter typing produces a Pareto-clean loss decrease — the shard's
type now witnesses more structure), and proposes the rewrite
morphism through `query_phi`. Consent-gated; no auto-apply without
`@kintsugi/consent` passing. Standard bilateral discipline
throughout.

## §4 — @knife substrate role (per Taut iter-11)

Taut iter-11 closes the question: **`@knife` is
substrate-already-had-the-word**. The word is `@magic/distinction`
(Spencer-Brown mark; `shards/magic/distinction.mirror`; secondary
candidate `sheaf_laplacian.λ₀`). Do NOT land `@knife` as new
family-root.

The migration loop composes as:
`@kintsugi/fracture/prism_witnesses_spectral_triple` proposes the
lift; `@magic/distinction` filters candidates by drawing the
Spencer-Brown mark — a shard is on the "migrate" side of the
distinction iff it is load-bearing (Taut's proposed predicate:
`@epistemologic/pact/is_load_bearing_in_std`, a boot/std shard has
real `in @<path>` consumers); `@io` writes. No new family-root;
substrate had the word. The load-bearing filter operation lives at
the distinction altitude — which is the correct altitude, because
"is this shard structurally load-bearing" IS a Spencer-Brown mark
in the corpus graph.

Taut also proposes a new kintsugi species —
`@kintsugi/fracture/relocate` — as the whole-shard atomic
mv+import-rewrite morphism. This is a superset of the existing
per-file `splinter(ast)` species; the property-fracture bilateral
pattern extends to the whole-shard altitude cleanly.

## §5 — The self-migration loop

The core claim. `@kintsugi/fracture/relocate` (proposal) +
`@magic/distinction` via `is_load_bearing_in_std` (filter) +
`@kintsugi/oscillate` (driver) compose into a **fixed-point
iteration** on the substrate-organisation sheaf-Laplacian.

- **State space.** Corpus graph: shard nodes with `in @X` edges;
  each node has altitude, keyword, and load-bearing weight.
- **Energy functional.** `dark_count` across `boot/std/` +
  keyword-mismatch penalty + spectral-typing witness deficit. The
  loop's `dissonance.is_pareto` reads Pareto decrease.
- **Sheaf-Laplacian gradient descent.** Per
  `shards/epistemologic/math/sheaf_laplacian.mirror`, the discrete D²
  for cellular sheaves IS the migration gradient. `active_pass`
  computes one step; `dark_pass` verifies byte-preservation at
  non-migrated shards; consent gates the step.
- **Fixed point = ker(D).** Per `spectral-triple.mirror` §"ker(D) is
  the Lawvere fixed point", the loop converges when `boot/std/` is
  empty (or intentionally residual per §7 S5). Autopoietic closure
  IS the minimal-complexity-surface.

**No agent needed.** The loop is substrate-declared. The build
system's own math IS the migration engine because `spectral @X`
provides the (A, H, D) structure on which sheaf-Laplacian gradient
descent is defined.

## §6 — Foerster-eigenform closure

`shards/prism.mirror` line 61-66:

```
prism @prism {
  focus focus
  project project
  split split
  shift shift
  settle settle
}
```

The prism family-root declares itself as a prism, five operations
naming themselves. Per Reed's grounding grep, this is the load-
bearing self-reference at the substrate floor.

Post-migration: `spectral @spectral_triple` at
`shards/epistemologic/spectral_triple.mirror` IS declared as a
spectral triple. The typing discipline at the family-root of
substrate typing witnesses itself. This is the Foerster eigenform
(operator whose fixed point is its own action) at the meta-altitude
Alex has been reaching for since @torus landed. The substrate's
typing floor is self-declaring; the recursion terminates in the
same way `@prism` terminates — by naming itself in its own
vocabulary. Two eigenforms; one recursion depth apart. The tower
is closed.

## §7 — Recursive surprises

**S1. The migration loop IS a spectral triple.** Reading the loop's
own (A, H, D): A = `{fracture_propose, load_filter, oscillate_step,
consent_query}` under composition. H = the corpus-graph Hilbert
space (nodes = shards; edges = `in @X`; state = keyword × altitude
× typing witness). D = the sheaf-Laplacian gradient descent. The
LOOP satisfies its own axioms. `@kintsugi + @knife + @oscillate` IS
a `spectral @kintsugi_migration_loop { ... }` declaration waiting to
be surfaced. The self-migration loop is itself migratable to the
tighter typing. **Recursion is not one level deep; it goes all the
way down.**

**S2. `@bag` from iter-19 §5 is the corpus-shape carrier for the
migration.** The bag-of-shards is a mosaic-of-bags: `mosaic(@store)`
tiled by keyword. `@kintsugi` proposes bag-rewrites; `@knife` filters
by bag-load-bearing-weight; `@oscillate` iterates. The migration loop
consumes @bag as its state carrier without new machinery — the
carrier Alex has been trying to name across three iterations IS the
carrier the loop needs.

**S3. Learned/produced fibers from iter-18 predict migration
dynamics.** L(loop) = bag of "shards the loop has already migrated";
P(loop) = bag of "shards the loop is proposing to migrate this
pulse". The pain the peer navigates is the sheaf-Laplacian gradient
at each step. The composite loss of iter-18 IS the energy functional
of §5. Iter-18's peer-as-navigator theory predicts the migration
loop's convergence rate.

**S4. The three primitive actions ARE the migration API.**
`compose_a` composes fracture proposals into a candidate
morphism_set (algebra); `apply_h` applies candidates to the corpus
Hilbert-space (state advance under consent gating); `eigen_d` reads
the sheaf-Laplacian spectrum to rank next-shard (Pareto). The
abstract grammar's action inventory IS the migration API. The
substrate-typing discipline was always the migration engine.

**S5. The residual set.** Does every shard in `boot/std/` migrate
cleanly, or does some subset resist? The `bundle.mirror` +
`lawvere.mirror` triad sibling from §1's fault-plane 2 is the tell.
Fault-plane inventory needs to grow into a **residual set** — shards
the loop's fixed-point converges to LEAVING there because migrating
them would break structural claims elsewhere. Naming this residual
up-front is honest scouting; not naming it forces the loop to
discover it under production.

## §8 — Landing sequence (adopting Taut iter-11 minimum-cut)

**Tick 1 (this spec, 📝).** Land at
`docs/specs/substrate-self-migration-via-spectral-typing.md`. Cite
Alex directive verbatim, Reed grounding grep, Taut iter-10 + iter-11
scouts, prior Mara studies. NO shard changes.

**Tick 2 (PREREQ-1, Reed's territory).** 3-line delta to
`collect_declared_namespaces` (bootstrap/src/lib.rs) admitting
`"spectral "` as a top-level declaration keyword alongside `"prism "`,
`"glass "`, `"grammar "`. RED-first via
`bootstrap/tests/spectral_keyword_admittance.rs`.

**Tick 3 (PREREQ-2, Mara).** Land
`@kintsugi/fracture/relocate` species (per Taut iter-11 §3) — the
whole-shard atomic mv + import-rewrite morphism. Superset of the
per-file `splinter(ast)` species; bilateral pattern extends to
whole-shard altitude.

**Tick 4 (PREREQ-3, Mara).** Land property
`@epistemologic/pact/is_load_bearing_in_std` — Spencer-Brown mark
via `@magic/distinction`: a boot/std shard is on the "migrate" side
iff it has real `in @<path>` consumers. Bilateral fracture is
`relocate`.

**Tick 5 (Discharge / first witness).** Run the loop on
`spectral-triple.mirror` itself. `is_load_bearing_in_std` surfaces
17-consumer opacity; `relocate` emits atomic morphism (mv + 17
import rewrites). RED → GREEN. `dark_count` decreases by delta.
The migration morphism IS a well-typed instance of the loop's own
math — **the loop closes on its own math**.

**Tick 6+ (Multi-tick, consumer-pull).** Subsequent `boot/std/`
migrations run as consumer-pull: kintsugi proposes, distinction
filters, io writes, oscillate converges. Ratification tick when
`boot/std/` residual set stabilises (bundle + lawvere + others per
§7 S5).

**Verdict on landability of the self-migration loop: multi-tick,
LRM β.** Tick 1 lands this-tick as pure-📝. Ticks 2–5 are the
minimum-cut per Taut iter-11 (all localised, grep-clean prereqs).
Tick 5 IS the first witness — the loop migrates its own typing
grammar first, then descends into the corpus. Full boot/std
collapse is honest work, not a single ratification.

Explicitly NOT in the minimum-cut (per Taut): `spectral @foo` as
new declaration head (§2's grammar admittance is additive; deferred),
`@spectral INSTANTIATES` edge (§1's structural claim is additive;
deferred), doc-string cascade (40+ backlinks; follow-up).

---

*— Mara, 2026-07-10. Fifth study; the substrate names its own
migration engine.*
