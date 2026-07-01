---
title: "Marker row as fourth structural primitive category"
author: Mara
date: 2026-07-01
status: CANONICAL (Seam-ratified via Loki §10 `17f0ee5`; landing candidate #112)
reviews:
  - docs/specs/loki-cuts-and-collapses.md §10
  - docs/audits/2026-07-01-seam-loki-cuts.md §10
  - docs/specs/third-as-recursive-depth.md §3 (marker row per @third)
grounded_in:
  - [[architecture-candidate-recognition-112-marker-row-fourth-structural-primitive]]
  - [[architecture-candidate-recognition-111-third-as-family-root]] (falsified as family-root, landed as marker)
  - [[feedback-substrate-already-had-the-word]]
  - Loki 2026-07-01 §10 "The marker row is its own structural primitive"
---

# Marker row as fourth structural primitive category

*Canonical substrate-decl spec landing recognition #112 (Loki `3d8797ac`, Seam-ratified `17f0ee5` §10). Names the substrate's fourth structural primitive category alongside family-roots, species, and predicates/pacts.*

---

## §1. What the substrate had been doing without naming it

The substrate has been building four categories of substrate-decl construct, but only three were named:

1. **Family-roots** — named as `prism @X { ... }` at `shards/X.mirror`. Domain the substrate is ABOUT. Examples: `@mirror`, `@kintsugi`, `@fate`, `@reality`, `@bauchladen`, `@autopoietic`.

2. **Species** — named at altitude-specific paths under family-roots. Concrete realizations of a family's discipline at a specific altitude. Examples: `@reality/algebra/math`, `@mirror/store/git`, `@code/rust`, `@io/cargo`, `@fate/tournament`.

3. **Predicates/pacts** — named at `@epistemologic/property/*`, `@epistemologic/cybernetic/*`, `@epistemologic/math/*`. Declarative property bodies imported into consuming families' `requires` clauses.

4. **Markers** — **NOT PREVIOUSLY NAMED as category.** Property-of-observation constructs that cross families via opt-in import. Examples: `@meta`, `@glass`, `@epistemologic`, `@third` (landed 2026-06-30 `e43006a`), `@labeled` (Loki §6 finding, marker-row-acknowledged 2026-07-01).

The fourth category was doing structural work — every existing marker was landed AD-HOC without recognition that they share a substrate role. Loki's cuts audit (spec `3d8797ac`) surfaced this at §10 as an explicit recognition. Seam's adversarial round (`17f0ee5`) ratified the category naming. This spec lands the canonical form.

---

## §2. The five current markers

As of 2026-07-01:

| Marker | Purpose | Imported via | Reference |
|--------|---------|--------------|-----------|
| `@meta` | Operates on substrate substrate | `in @meta` | `shards/mirror/meta.mirror` (path may reshape) |
| `@glass` | Exposes an opacity surface | `in @glass` | `shards/glass.mirror` |
| `@epistemologic` | Admits verdict discipline | `in @epistemologic` | `shards/epistemologic.mirror` (namespace-parent for predicates) |
| `@third` | Witnesses recursion at depth ≥ 3 | `in @third` | `shards/third.mirror` |
| `@labeled` | Adds label dimension to a value | `in @labeled` | `shards/labeled.mirror` |

Each fires the three-test discipline (§3). Each is imported opt-in by families that want the property. Each crosses family-root boundaries.

---

## §3. The three-test discipline (Loki §11 / Mara F1)

Any prism candidate for marker classification passes three tests:

### 3.1 Domain test

**Does it name a domain the substrate is ABOUT? Or a property of a domain?**

- If domain → family-root.
- If property → marker candidate.

**Witness for `@third`:** recursion-depth is a *property of an observation*, not a *domain the substrate is about*. Mara's F1 reshape (spec `e43006a` §2) caught this: Reed's `@third <= @cogito, @autopoietic, @bauchladen, @metalogue, @algebra, @cyberpunk` inheritance framing miscast the marker as a domain the six families are instances of. The correct relationship: the six families import `in @third` to declare their property, not inherit from it.

**Witness for `@labeled`:** adding a label dimension is a *functor over a value*, not a *domain the value belongs to*. Same shape as `@glass` (opacity as functor), not the same shape as `@fate` (constrained inference as domain).

### 3.2 Import test

**Do families import it via `in @X`? Does it decline to declare its own operational contract at the family-root altitude?**

Markers export declarative machinery (predicates, carriers, type functors) that families consume. Markers do NOT declare their own primary discipline at family-root altitude — the discipline is what the marker LABELS on other families, not what it drives on its own.

**Witness for `@third`:** shards/third.mirror declares the `observation_depth` carrier and `third_order_active` composed bilateral. The carrier and predicate are consumed by families' `witness_third_order` actions. `@third` does not itself drive a domain — it labels other families' operation at depth-3.

**Witness for `@labeled`:** shards/labeled.mirror declares the `labeled(v, m)` functor and `label` / `unlabel` / `label_of` actions. The functor is consumed by cascade/docs/io/ui families. `@labeled` does not drive its own domain — it labels other families' values with metadata.

### 3.3 Domain-crossing test

**Does the same construct appear across multiple family-roots at different altitudes?**

Markers cross families. A property that only makes sense within one family's domain is not a marker — it's a species under that family. A property that surfaces the same substrate-decl shape across multiple family-roots is a marker.

**Witness for `@third`:** `docs/specs/reflection-third-order-by-default-v0.1.md` (2026-06-22, 1514 lines) declares depth-3 at `@reflection`. `docs/insights/2026-06-22-third-order-and-multi-repo.md` names depth-3 at `@pack`. Recognition #93 forward-promises depth-3 at `@cogito` via `cognitive_third_order`. `@cyberpunk` inherits depth-3 via `@epistemologic/cybernetic/second_order`. Four families at four altitudes; one marker labels all four.

**Witness for `@labeled`:** eight importers across cascade (4 species: `gleam/beam`, `gleam/js`, `purescript/js`, `rust/wasm`), docs (1), io (2: `git`, `oci`), ui (1: `field`). Four family-roots, one marker.

---

## §4. How markers differ from family-roots (Mara F1 pattern)

Per spec `e43006a` §2 F1, the family-vs-marker distinction is load-bearing:

**Family-root altitude:**
- `prism @X { five ops }` names the discipline itself
- Path syntax carries species specialization: `@X/species/name`
- Sibling family-roots are peers at namespace altitude
- Import via `in @X` gives access to the family's typed carriers AND commits the importer to be an instance of the family's discipline

**Marker altitude:**
- `prism @X { five ops }` names the marker itself, but the substrate-decl body exports property machinery for OTHER families
- Path syntax carries the marker's own species (predicates, sub-carriers)
- Sibling markers are peers at property-altitude (crossing family-roots)
- Import via `in @X` gives access to the marker's typed carriers AND commits the importer to admit the marker's property when its actions produce values

The distinction inverts the direction of authority:

- **Family-root:** the family DEFINES what its discipline means; importers instantiate the definition.
- **Marker:** the marker DEFINES the property signature; importers CLAIM the property when they produce values that satisfy it.

Family-roots build the substrate's domains. Markers extend the substrate's expressive vocabulary orthogonally.

---

## §5. Adjacency to Glint Surface A

Glint's Surface A framing (form + substance + boundary as three altitudes of substrate completeness) admits markers as a candidate fourth axis: **structural-property-crossing**.

The three named altitudes:
- **Form** (state observation): what the substrate LOOKS LIKE — `@mirror` family; observable structure.
- **Substance** (behaviour/process): what the substrate DOES — `@kintsugi` family; transformation dynamics.
- **Boundary** (@io/composition): where substrate becomes non-substrate — `@io` family; alignment-as-boundary-mathematics.

The candidate fourth altitude:
- **Structural-property-crossing** (markers): what the substrate LABELS across any of the above — `@meta`, `@glass`, `@epistemologic`, `@third`, `@labeled`; property functors and carriers that cross form/substance/boundary.

This is a candidate framing per candidate #112's forward-promised cascade item; the reshape is not landed here. This spec names the marker category structurally; whether Surface A gets a fourth axis is a separate Pack-ratification cycle.

Related but distinct:
- Mara F5 (recursion-depth as fourth corner of meta-primitive tetrahedron) — different framing; also candidate.
- Falsified candidate #110 (@species as meta-primitive) — Taut's scout falsified because patterns diverged at discharge altitude. #112 markers converge on path-namespace + cross-family import shape without forcing universal discharge — different structural claim, different outcome.

---

## §6. The mechanical audit test for future recognitions

When a new prism candidate surfaces, apply the three tests BEFORE declaring family-root:

```
1. Domain test:
     Does it name what the substrate is ABOUT?
       YES → candidate family-root (proceed to family-root discipline)
       NO  → candidate marker (proceed to import + domain-crossing tests)

2. Import test:
     Would families import it via `in @X` WITHOUT the candidate declaring
     its own operational contract at family-root altitude?
       YES → strong marker candidate
       NO  → likely family-root or species

3. Domain-crossing test:
     Does the same substrate-decl shape appear across multiple family-
     roots at different altitudes?
       YES → marker (crosses families)
       NO  → likely species under a specific family-root
```

If all three point to marker, declare in the marker row (add to §2 table). If tests are mixed, escalate to Pack review — the substrate is telling us the recognition needs more shape.

**Pattern that recurred four times before this spec:** the substrate had markers landed AD-HOC (each classified individually) without the category being named. Every future recognition benefits from the mechanical test replacing the individual audit.

---

## §7. Adjacent Loki finding: four-in-one packing anti-pattern

Loki §10 additionally surfaced a candidate-of-candidate observation: **substrate atomicity overridden by policy/symmetry** in two instances so far:

1. `@algebra/metalogue` row-completion (§4)
2. `@docs/design` four-in-one packing (§9)

Two instances = candidate anti-pattern watch. Third witness would promote to full recognition. Not landed here; noted for continued observation.

The relationship to #112: markers PRESERVE atomicity by keeping property-signatures orthogonal to domains. Family-root packing (bundling multiple properties into one @X) is the anti-pattern; the marker discipline extracts each property to its own marker. #112 is the substrate-pull-correct alternative to the anti-pattern.

---

## §8. Forward-promised cascade

Per candidate #112:

1. Seam adversarial review — LANDED (`17f0ee5` §10 RATIFY).
2. Mara canonical spec — LANDED (this document).
3. Species-level markers considered — forward-promised (does the substrate need per-marker sub-primitives like `@third/reflection` or `@labeled/cascade`? Answer depends on consumer pull.).
4. Loki's 10 cut proposals adjudicated Pack-wide — IN FLIGHT (§1/§5/§6/§7/§10 landing this loop; §2/§4/§8 deferred; §3/§9 rejected).
5. MEMORY promotion once Pack ratifies — pending Seam observation window.

---

## §9. Substrate-pull-honest weakenings

- Marker category is a category NAME for a shape the substrate was already carrying; not a new operation.
- The three-test discipline is Loki's articulation of what Mara F1 caught empirically; formal proof of test completeness forward-promised.
- Surface A fourth-axis framing (§5) is candidate; not asserted here.
- Some current markers may reshape in future ticks:
  - `@epistemologic` is currently a namespace-parent for predicates; whether it also carries a marker-shape at its family-root altitude is Pack-review-open.
  - `@meta` may have marker + family-root dual role (self-referential substrate = both category and instance); Loki §10 flags but does not resolve.
  - Future recognitions may surface additional markers currently mis-classified as species.

---

## §10. What lands this tick

- Category name: **marker row as fourth structural primitive**.
- Five current markers enumerated.
- Three-test discipline documented.
- Family-vs-marker distinction (Mara F1) named at substrate altitude.
- Adjacent framings noted (Surface A candidate fourth axis; four-in-one packing anti-pattern watch).
- Mechanical audit test for future recognitions.

The substrate had the word already. This spec names it.

*"Every existing marker was landed AD-HOC without recognition that they share a structural role."* — recognition #112. Now they don't.
