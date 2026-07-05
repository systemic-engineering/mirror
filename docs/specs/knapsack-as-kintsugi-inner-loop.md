# @knapsack as kintsugi inner-loop selection primitive — derived landing plan

*2026-07-05. Mara. Derived spec proposing @knapsack's substrate landing
per five-signal auto-classifier verdict + Arc 3 (bottom-up doc-code-seam)
interleaving. The load-bearing math is at `docs/math/resource-budget/README.md`.*

Composes with:
- `docs/math/resource-budget/README.md` (this tick — load-bearing math).
- `docs/specs/doc-code-seam-bottom-up-landing.md` (this tick — Arc 3
  ten-tick landing sequence).
- `docs/math/prism-kind/README.md` (Mara `bdb148a` — five-signal
  auto-classifier this spec's placement verdict is grounded in).
- `docs/specs/kintsugi-variety.md` §4 (Reed + Alex 2026-06-02 — the
  Knapsack framing at @io altitude).
- `docs/specs/silicon.md` (Mara `27e9067`).
- `shards/reality/algebra/silicon.mirror` (Mara 2026-07-01 — the
  empirical discharge species).
- `shards/kintsugi.mirror` (Mara 2026-06-10 — family-root).
- `shards/mirror/spec.mirror` (Reed — project manifold grammar).

Status: **derived spec proposing landing shape**. Not canonical.
Per `[[feedback-craft-not-deliver]]` no shards land this tick. Ready
for Reed's 🔴 RED pass in follow-up TDD ticks per
`[[feedback-write-red-in-session]]`.

---

## §0. What this spec answers

Alex 2026-07-05 asked eleven questions about @knapsack's landing shape.
This spec answers them in order, grounded in five-signal auto-classifier
verdicts, existing substrate ancestors, and Arc 3 interleaving.

The math is at `docs/math/resource-budget/README.md`. This spec is
actionable landing plan per shard.

---

## §1. Placement — species under @kintsugi

### §1.1 Five-signal auto-classifier verdict

Per `docs/math/prism-kind/README.md` §3, the auto-classifier has five
signals. Applied to `@knapsack`:

| Signal                             | @knapsack verdict            |
|------------------------------------|------------------------------|
| S1: `<= @X` inheritance present?   | **yes** (`<= @kintsugi`)     |
| S2: carrier density (types + actions) | ~6 types + ~5 actions (medium) |
| S3: cross-family import sites      | @kintsugi + @epistemologic + @reality/algebra/silicon + @io (4 families; MEDIUM) |
| S4a: docblock cites marker row #112 | **no**                       |
| S4b: docblock cites form/process #55 | **yes** (process side)      |
| S5: primary carrier is a ref?       | **no** (primary carrier is `capacity_vector` typed record) |

Signal breakdown: 3 signals point to **species_root** (inherits, medium
carrier density, non-marker), 2 signals point to **family_root**
(cross-family import breadth, non-thin carrier).

**Verdict: `species_root` under `@kintsugi`**. Substrate-honest.

### §1.2 Comparison to alternatives

- **`@knapsack` as family-root** (own algebra): REJECTED. Fails S1
  (no reason for `@knapsack` to sit at the top level next to @mirror,
  @kintsugi, @reality; it's the operational selection primitive of one
  species, not a new family).
- **`@knapsack` as species under `@epistemologic`**: REJECTED. Fails
  S3 (the primary import is @kintsugi's fracture bodies, not @epistemologic's
  properties; @epistemologic would host the *property* half of a bilateral
  pattern, but @knapsack IS the operational half).
- **`@knapsack` as marker**: REJECTED. Fails S2 (non-thin carrier);
  fails S4a (docblock does not fit the marker-row precedent).

### §1.3 The landing path

```
shards/kintsugi/knapsack.mirror   # depth-2; species under @kintsugi
```

Uses `glass` keyword per recognition #46 (sub-shards at depth ≥ 1 use
`glass`; family-root uses `prism`).

### §1.4 Bilateral (property + fracture) pattern extends

Per recognition #53 (property/fracture bilateral pattern): @knapsack
composes with `@epistemologic/property/round_descent`
(**forward-promised**) as the property side declaring `‖T_{n+1}‖ <
‖T_n‖`, and `@kintsugi/knapsack` as the operational side selecting
the morphism subset that discharges the property.

This makes @knapsack the seventh+ bilateral instance (per #53
promoted; recognition #59 altitude-portability holds).

---

## §2. The capacity_vector — multi-dim capacity

### §2.1 v0 recommendation: (silicon, ram)

Two dimensions. Minimum viable capacity dimension per Kellerer-
Pferschy-Pisinger's PTAS running-time trade-off (each dimension
increases the PTAS exponent by 1). Two is the smallest that captures
the substrate's actual physical constraint (Cholesky FLOPs vs matrix
storage bytes are structurally independent — Roofline model per
Williams-Waterman-Patterson 2009 already grounds silicon.mirror at
line 274).

### §2.2 v1 forward-promise

```
type capacity_vector_v1 = {
  silicon:     silicon_budget,
  ram:         ram_budget,
  wall_clock:  wall_clock_budget,
  gpu_memory:  gpu_memory_budget,   # optional; None on CPU-only hosts
}
```

Each addition is a Frieze-Clarke PTAS re-derivation; each is a follow-up
tick after v0 lands.

### §2.3 Carrier declarations (per `[[feedback-no-bare-types]]`)

```
type silicon_budget = ref    # FLOPs count as u64-typed ref
type ram_budget     = ref    # bytes as u64-typed ref

type capacity_vector = {
  silicon: silicon_budget,
  ram:     ram_budget,
}
```

Identity contract: byte-equality on `(silicon, ram)`.

### §2.4 Read source

`capacity_vector` is READ from `@reality/algebra/silicon.mirror`'s
`compute_budget` crystal kind (line 137: "Matter aspect: the (cores,
RAM, GPU RAM, wall-clock, spectral-reduction) budget tuple from
`@epistemologic/reality/silicon/compute_bound`. Information aspect:
the tournament-restriction policy @fate applies when selecting a
routine.").

The substrate already had the source. @knapsack's `capacity_vector`
is the reader; the `compute_budget` crystal is the writer.

---

## §3. @silicon + @ram in mirror.spec — IMPLICIT with @reflection inference

### §3.1 The debate

Currently mirror.spec declares budgets IMPLICITLY via cargo profiles
(`[profile.test] codegen-units = 256`). Alex 2026-07-05 asked whether
to lift them to EXPLICIT fields.

### §3.2 v0 recommendation: implicit

Keep implicit. Reason: `@reality/algebra/silicon.mirror` already
crystallizes the physical budget as `compute_budget` crystals. Adding
explicit fields on mirror.spec duplicates state (`[[feedback-substrate-already-had-the-word]]`).

The v0 mechanism:

1. `@reality/algebra/silicon.crystallize` writes the `compute_budget`
   crystal per host at build init.
2. `@kintsugi/knapsack.read_capacity(target_ref) -> capacity_vector`
   reads the crystal via `discharge` on the silicon Bauchladen.
3. `@reflection` observes the packing verdicts post-tick and writes
   below `---` (per §7).

Below-`---` writeback per target (per `liquid-types/README.md` §7
composition):

```
target dpotrf-arc {
  altitude @reality/algebra/silicon
  emit lapack
}
---
# @reflection writeback (2026-07-05T12:34:56Z):
# read_capacity: { silicon: 8.5e9 flops, ram: 4.2e6 bytes }
# selected: [dpotrf_2x2, dpotrs_2x2]
# packing_verdict: success
# descent: 0.13 < 0.87 ✓
```

### §3.3 v1 forward-promise: explicit if empirical measurement demands

IF v0 empirical measurement finds @reflection's inference from crystal
metadata is insufficiently sharp (e.g., the crystallized budget does
not reflect the target's actual per-target ceiling), lift to explicit
`target binary { silicon_budget: ...; ram_budget: ... }` fields.

The v1 spec would be:

```
target dpotrf-arc {
  altitude @reality/algebra/silicon
  emit lapack
  silicon_budget 1e10
  ram_budget 8e6
}
```

DEFERRED to empirical discriminator run.

---

## §4. Overflow fallback — PARTIAL with hold(carrier)

### §4.1 The load-bearing edge case: capacity = 0

Alex 2026-07-05 named the edge case: "Behavior at capacity=0 is the
load-bearing edge case."

At capacity = 0, no morphism fits under budget. This is (P1) violation
per `docs/math/resource-budget/README.md` §4.1.

### §4.2 v0 recommendation: PARTIAL + hold(carrier)

The three-mode algebra per `docs/math/kintsugi/README.md`'s
`compiler-error-surface.md` names three discharge modes: `apply`,
`spawn`, `hold`. At capacity = 0:

- NOT `apply` — no morphism to apply.
- NOT `spawn` — spawning a peer at capacity = 0 doesn't help (the peer
  inherits or receives its own capacity; if the parent's is 0, so is
  the peer's typically).
- **`hold(carrier)`** — the legitimate non-discharge. The substrate
  crystallizes the tension unresolved; carries it forward; the observer's
  next tick may raise capacity (via budget re-negotiation upstream).

Per `error-as-question.md` §2's six-variant answer algebra, `hold(ref)`
is Variant 6 — the substrate's Partial(0.0, ref) commitment. Empirical
composition per candidate #141 (conditional-marker discipline; landed
at kintsugi/surface's grin) already witnesses this shape.

**The overflow verdict is `partial(opacity_map)` with the opacity_map
naming the deferred candidates**. Not `failure`. Not `success`. The
opacity_map's transparency drops to (near) 0 for this round; the next
round may raise capacity and re-attempt.

### §4.3 FAILURE variant reserved for structural mismatches

`failure(cause)` is reserved for structural mismatches:
- (P2) violation with `ε > 1`: PTAS approximation degenerate.
- (P3) violation persistent across three rounds (three-tick @third audit).

At capacity = 0 alone (with no other violations), the verdict is
`partial`, not `failure`. Reason: capacity CAN change tick-to-tick;
`failure` is for structurally impossible packings.

---

## §5. Composition with Arc 3 (bottom-up doc-code-seam)

Per `docs/specs/doc-code-seam-bottom-up-landing.md` ten-tick sequence:

```
TICK 1: shards/docblock.mirror                       (family-root)
TICK 2: shards/epistemologic/liquid_extraction.mirror (sibling family-root)
TICK 3: shards/epistemologic/pact/prism_kind_declared.mirror
TICK 4: shards/kintsugi/fracture/prism_kind_ambiguous.mirror
─── auto-classifier fires for real at TICK 3+4 close ───
TICK 5-10: docblock property + fracture bilateral (six shards)
```

### §5.1 @knapsack landing tick

**TICK 11 or interleaved at TICK 3.5**:

Two options:

**Option A: TICK 11 (post-cascade)**. Land after the ten-tick cascade
closes. Prism-kind auto-classifier operational; @knapsack lands as a
first empirical USE of the classifier at the compiler-fit altitude.

- Pro: post-classifier landing means @knapsack's `species_root`
  verdict per §1 is machine-verified before the shard exists.
- Pro: `@epistemologic/property/round_descent` (@knapsack's bilateral
  property half) composes on top of the docblock cascade's audit
  machinery.
- Con: pushes @knapsack out one tick.

**Option B: Interleaved at TICK 3.5 (parallel to prism-kind pact)**.
Land alongside `@epistemologic/pact/prism_kind_declared` since
@knapsack itself is a first empirical instance of the discriminator.

- Pro: co-lands with the machinery that classifies it.
- Con: risks conflating the classifier's discipline with @knapsack's
  substrate.

**Recommendation: Option A (TICK 11)**. Substrate-honest: the
classifier lands first, then @knapsack lands as an example USE.
Per Arc 3's bottom-up discipline (@Alex 2026-07-05: "The whole
substrate is bottom-up. Land the eight-shard cascade FIRST; prism-
kind emerges naturally as the first concrete USE. This IS the loop
closing"), @knapsack extends the same bottom-up composition one
altitude up.

### §5.2 Precondition for @knapsack TICK 11

- TICKS 1-10 (Arc 3 sequence) closed.
- `@epistemologic/pact/prism_kind_declared` fires the auto-classifier
  verdict `species_root` on `@knapsack`'s draft docblock (empirical
  discriminator run).
- Bilateral: `@epistemologic/property/round_descent` lands first;
  `@kintsugi/knapsack` lands as its operational sibling.

---

## §6. Composition with @reflection N+1 writeback to mirror.spec

Per `shards/reflection.mirror`: @reflection observes pipelines and
writes adjustments at n+1 altitude. Applied to @knapsack:

### §6.1 The writeback shape

After each kintsugi round using @knapsack for selection, @reflection
writes a **packing report** below `---` in the relevant mirror.spec
target:

```
target NAME { ... }
---
# @reflection knapsack writeback:
# tick_oid:          b3:...
# tick_time:         2026-07-05T12:34:56Z
# capacity_read:     { silicon: 8.5e9 flops, ram: 4.2e6 bytes }
# candidates_total:  17
# candidates_selected: 5
# candidates_deferred: 12
# transparency_gain: 0.87
# opacity_before:    1.00
# opacity_after:     0.13
# packing_verdict:   success
# descent_witness:   0.13 < 1.00 ✓
# audit_depth:       3
# audit_verdict:     answerable
```

### §6.2 Writeback discipline

Per `docs/math/liquid-types/README.md` §7 (this tick) + `docs/math/kintsugi/doc-code-seam.md`
(Mara `20c99a2`):

- The `---` seam separates declaration (above) from observation (below).
- Above-`---` declares the target and its capacity.
- Below-`---` is @reflection's actual-vs-declared audit.
- The packing report is one specialization of below-`---` writeback.

### §6.3 Cross-tick composition

If tick N's writeback shows opacity_after > 0 (partial packing),
tick N+1's kintsugi round reads the deferred candidates from the
opacity_map and re-attempts. The below-`---` writeback carries this
across ticks — @reflection is the memory that persists between
packing rounds.

### §6.4 Interaction with @onto answerability

If @reflection's audit_verdict is `absorbed` (per `docs/math/onto/README.md`
§2.1), the substrate refuses to close the round. The absorbed verdict
is Tsvasman's failure mode: the packing claimed exhaustion it did
not deliver. @knapsack's response: emit a `spawn(scheduler)` per
`@pain` Cat 5 (§4.2 of resource-budget/README.md).

---

## §7. @spectral-db composition

Per `[[architecture-mirror-store-vs-spectral-db]]`: @spectral-db is
the closed-source engine on top of @mirror/store; eigenvalue
computation is silicon-bound.

### §7.1 Composition shape

When @spectral-db invokes kintsugi under budget:

1. @spectral-db issues an eigenvalue query with `silicon_cost = O(n³)`
   for Jacobi diagonalization.
2. The query enters `@fate`'s tournament as one candidate.
3. `@kintsugi/knapsack.select(candidates, capacity)` decides admission:
   - IF `silicon_cost(query) ≤ capacity.silicon` AND
     `ram_cost(query) ≤ capacity.ram`: query is a candidate in the
     PTAS packing.
   - IF eligible AND selected: query executes; eigenvalues flow back
     as `transparency_gain`.
   - IF eligible but deselected: query defers; opacity_map carries
     the query as deferred.
   - IF ineligible (over-budget): (P1) violation; `hold(query)`; @spectral-db
     receives `pending(opacity_map)` verdict.
4. Post-execution: @reflection writes packing verdict below `---`.

### §7.2 The API surface

```
in @spectral-db  # closed-source consumer imports this
in @kintsugi/knapsack

# @spectral-db queries kintsugi's knapsack for admission verdict
admit_query(q: spectral_db_query, cap: capacity_vector)
  -> imperfect<admission_verdict, spectral_db_error, transparency(query)>
{ \ }
```

### §7.3 Cost estimation

The `silicon_cost` of a spectral-db query is READ from the query's
type signature via @fate's optical inference (per recognition #58):
Jacobi is O(n³); Lanczos is O(n²·k) for k eigenvalues; power iteration
is O(nnz · iterations). The cost estimates are crystallized in
`@reality/algebra/silicon.mirror`'s `kernel-call` crystal kind
(matter aspect: batch size, tile shape, SIMD width, cache-blocking
factor; information aspect: the algebraic operation the kernel realizes).

---

## §8. The landing plan per shard

### §8.1 `shards/kintsugi/knapsack.mirror` (TICK 11)

**Signature**:

```
in @prism
in @glass
in @kintsugi
in @epistemologic
in @reality/algebra/silicon
in @io

# @kintsugi/knapsack — the multi-dim 0/1 knapsack selection primitive
# for kintsugi's inner-loop step (3).
# ...

glass @kintsugi/knapsack <= @kintsugi {
  focus knapsack
  project knapsack
  split knapsack
  shift knapsack
  settle knapsack
}
```

**Carriers**:

```
type silicon_budget = ref
type ram_budget     = ref

type capacity_vector = {
  silicon: silicon_budget,
  ram:     ram_budget,
}

type candidate = {
  morphism:          ref,
  silicon_cost:      silicon_budget,
  ram_cost:          ram_budget,
  transparency_gain: ref,
}

type selection = [candidate]

type packing_verdict =
  | success(selection)
  | partial(opacity_map)
  | failure(cause)
  | hold(ref)         # legitimate non-discharge; per compiler-error-surface.md

type cause =
  | p1_infeasible         # (P1) violated
  | p2_approximation_loss # (P2) violated
  | p3_interference       # (P3) violated
```

**Actions**:

```
# Read the capacity vector from @reality/algebra/silicon's crystals
read_capacity(target: ref) -> capacity_vector { \ }

# Multi-dim 0/1 knapsack solver via Frieze-Clarke PTAS
select(candidates: [candidate], cap: capacity_vector, epsilon: ref)
  -> packing_verdict
  requires round_descent_admissible(candidates, cap)
{ \ }

# Post-selection: apply the morphisms
apply_selection(sel: selection, state: opacity_map)
  -> imperfect<opacity_map, cause, transparency(selection)>
{ \ }

# @reflection writeback: emit the below-`---` packing report
writeback_below_seam(target: ref, verdict: packing_verdict) -> ref { \ }
```

**Audit test targets** (per `bootstrap/tests/kintsugi_surface_shard.rs`
text-check discipline):

1. `knapsack_shard_declares_knapsack_prism_as_glass_species`.
2. `knapsack_shard_declares_silicon_budget_carrier`.
3. `knapsack_shard_declares_ram_budget_carrier`.
4. `knapsack_shard_declares_capacity_vector_carrier`.
5. `knapsack_shard_declares_candidate_carrier`.
6. `knapsack_shard_declares_selection_carrier`.
7. `knapsack_shard_declares_packing_verdict_variants` (four variants).
8. `knapsack_shard_declares_cause_variants` (three variants).
9. `knapsack_shard_declares_read_capacity_action`.
10. `knapsack_shard_declares_select_action`.
11. `knapsack_shard_declares_apply_selection_action`.
12. `knapsack_shard_declares_writeback_below_seam_action`.
13. `knapsack_shard_requires_round_descent_admissible_on_select`.
14. `knapsack_shard_inherits_kintsugi`.
15. `knapsack_shard_inherits_reality_algebra_silicon`.

**Landing precondition**:

- Arc 3 TICKS 1-10 closed (docblock cascade operational).
- `@epistemologic/pact/prism_kind_declared` empirical run classifies
  `@knapsack` as `species_root`.
- `@epistemologic/property/round_descent` (§8.2) landed first.
- `@reality/algebra/silicon.mirror` `compute_budget` crystal kind
  operational (already landed; Mara 2026-07-01).
- `@reflection`'s below-`---` writeback mechanism operational (Arc 3
  TICKS 5-10 provide this via docblock verdict discipline).

**Composition dependencies**:

- Depends on `@kintsugi.mirror` (family root; LANDED).
- Depends on `@reality/algebra/silicon.mirror` (LANDED).
- Depends on `@epistemologic/cybernetic/variety.mirror` (LANDED per
  recognition #36).
- Depends on `@epistemologic/property/round_descent.mirror`
  (forward-promised TICK 11a).
- Depends on `@kintsugi/fracture/knapsack_infeasible.mirror`
  (forward-promised TICK 11c per bilateral pattern).
- No downstream shard depends on TICK 11's actions being discharged.

### §8.2 `shards/epistemologic/property/round_descent.mirror` (TICK 11a; bilateral property side)

**Signature**:

```
glass @epistemologic/property/round_descent <= @epistemologic/property {
  # The property side of the @knapsack bilateral: declares that
  # each kintsugi round satisfies |T_{n+1}| < |T_n| off-fixed-points.
}
```

**Carriers**:

```
type descent_signal = {
  before:  ref,   # opacity_before as ref
  after:   ref,   # opacity_after as ref
  witness: ref,   # oid of the packing_verdict
}
```

**Actions**:

```
round_descent(before: opacity_map, after: opacity_map)
  -> transparency<descent_signal>
{ \ }

round_descent_admissible(candidates: ref, cap: ref) -> verdict { \ }
```

### §8.3 `shards/kintsugi/fracture/knapsack_infeasible.mirror` (TICK 11c; bilateral fracture side)

**Signature**:

```
glass @kintsugi/fracture/knapsack_infeasible <= @kintsugi/fracture {
  # The fracture side of the @knapsack bilateral: when the property
  # `round_descent` fails, this fracture body applies the substrate's
  # response (spawn peer with expanded capacity, or hold, or rebudget).
}
```

**Body**:

```
fracture_body(before: opacity_map, verdict: packing_verdict)
  -> imperfect<opacity_map, cause, transparency(fracture_response)>
{
  # Dispatches per §4.2 of resource-budget/README.md:
  # - Cat 2 (P1 violated): spawn(peer) with expanded capacity
  # - Cat 4 (P3 violated): apply(rebudget_shard) via error-as-question §2
  # - Cat 5 (P2 violated): spawn(scheduler) with new epsilon
  \
}
```

### §8.4 Landing order

```
TICK 11a: shards/epistemologic/property/round_descent.mirror
TICK 11b: shards/kintsugi/knapsack.mirror
TICK 11c: shards/kintsugi/fracture/knapsack_infeasible.mirror
```

Three-tick cascade closing @knapsack landing.

---

## §9. Empirical discriminator run

Per `[[feedback-composition-claims-need-empirical-test]]`: this spec's
landing claims must be empirically verified.

### §9.1 Discriminator targets

- **After TICK 11a**: `round_descent` property fires on the two
  extant fracture bodies (`kintsugi/fracture/angle_to_paren` and
  `kintsugi/fracture/symbol_lift`). Verify: property returns
  `success` in both cases (both fracture bodies preserve descent
  by construction).

- **After TICK 11b**: `@kintsugi/knapsack.select` runs on the
  Cholesky arc (`shards/reality/algebra/silicon.mirror` +
  `shards/epistemologic/math/cholesky.mirror`). Verify: PTAS
  returns a packing verdict `success` under empirical
  `capacity_vector = (silicon: 8.5e9, ram: 4.2e6)` on 2×2 SPD
  matrices; descent holds.

- **After TICK 11c**: `@kintsugi/fracture/knapsack_infeasible`
  fires on the capacity = 0 edge case. Verify: fracture body returns
  `hold(ref)` per §4.2, not `failure`.

### §9.2 Non-empty change set expected

- After TICK 11 close: the `round_descent` property becomes queryable
  by any @kintsugi consumer wanting to prove their fracture body
  preserves descent. This IS new substrate mechanism (before this
  tick: prose only in gap-tension-tensor-substrate.md §10.A; after:
  a typed predicate consumers can requires-bind).

- After TICK 11 close: `@spectral-db`'s admission verdict flows
  through `@kintsugi/knapsack.admit_query` instead of ad-hoc budget
  checks. Substrate-fact rather than closed-source discipline.

---

## §10. Composition summary

| Question                              | Answer                                                              |
|---------------------------------------|---------------------------------------------------------------------|
| Placement                             | Species under @kintsugi at `shards/kintsugi/knapsack.mirror`        |
| Marker vs family vs species           | species_root (3/5 signal majority)                                  |
| Multi-dim capacity                    | v0: (silicon, ram); v1: + wall_clock; v2: + gpu_memory              |
| Silicon + ram in mirror.spec          | IMPLICIT with @reflection inference from `compute_budget` crystals  |
| Overflow fallback                     | PARTIAL with hold(carrier) at capacity=0; FAILURE reserved for structural mismatch |
| Arc 3 interleaving                    | TICK 11 (post-cascade); classifier lands first, @knapsack extends   |
| @reflection N+1 writeback shape       | packing report below `---` per target                               |
| @spectral-db composition              | `admit_query` API; eigenvalue budget is one dimension of capacity   |
| Bilateral pattern                     | @epistemologic/property/round_descent + @kintsugi/fracture/knapsack_infeasible |
| Landing order                         | 11a (property) → 11b (knapsack) → 11c (fracture)                    |

---

## §11. What's DEFERRED

- Shard landings for `@kintsugi/knapsack`, `@epistemologic/property/round_descent`,
  `@kintsugi/fracture/knapsack_infeasible`. Per `[[feedback-craft-not-deliver]]`.
- Empirical discriminator run per §9.
- v1 addition of `wall_clock` dimension (post-empirical measurement).
- v1 lift to EXPLICIT mirror.spec budget fields (IF v0 implicit
  inference proves insufficiently sharp).
- FPTAS-grade running time (currently PTAS; requires Lagrangian
  relaxation per Kellerer §11).
- Interference model (P3) formalization.
- Interaction between @knapsack and @loop lifecycle at TICK 11
  ratification.

---

## §12. Substrate-honest self-audit

Per `63bdecc` §6 discipline: this spec's claims must survive
`audit(this_spec, depth=3)`.

Claims:

1. Placement verdict = species_root (§1). Grounded in five-signal
   auto-classifier per `prism-kind/README.md`.
2. v0 capacity vector = (silicon, ram) (§2). Grounded in Frieze-Clarke
   PTAS running-time trade-off per Kellerer-Pferschy-Pisinger.
3. Silicon + ram in mirror.spec = implicit (§3). Grounded in
   `[[feedback-substrate-already-had-the-word]]` — `compute_budget`
   crystals already declared in silicon.mirror.
4. Overflow = PARTIAL with hold(carrier) (§4). Grounded in
   `compiler-error-surface.md`'s three-mode algebra + `error-as-question.md`
   §2's six-variant answer algebra.
5. Arc 3 interleaving = TICK 11 (post-cascade) (§5). Grounded in
   bottom-up substrate-honest ordering per `doc-code-seam-bottom-up-landing.md`.
6. @reflection writeback = below-`---` packing report per target (§6).
   Grounded in `liquid-types/README.md` §7 + `doc-code-seam.md`.
7. @spectral-db composition = `admit_query` API (§7). Grounded in
   `[[architecture-mirror-store-vs-spectral-db]]` + `@fate` optical
   inference (recognition #58).
8. Bilateral (property + fracture) pattern (§8). Grounded in recognition
   #53 (promoted) + #59 (altitude-portable).

`project_adversarial(this_spec) -> (P, R)`:

- **P (phantom)**: this spec catalogues eight ancestors and proposes
  three shard landings without machine-verifying the placement verdict.
  The auto-classifier per `prism-kind/README.md` §2 is proposed but
  not operational; §5.2 explicitly makes TICK 11 depend on the
  classifier firing. If the classifier's verdict differs from §1's
  reader-frame verdict, the placement is wrong.
- **R (real)**: the shard signatures, carriers, actions, and audit
  test targets in §8 are actionable — Reed's next 🔴 pass can write
  them verbatim. The bilateral pattern is the seventh+ instance of
  the property + fracture cascade closing.

At this depth: **both interpretations satisfiable**.

`audit(this_spec, depth=3) -> opaque(opacity_map)`.

Route per `compiler-error-surface.md`'s three-mode algebra: **`spawn`**.

This spec IS the Tomm question at reader-frame altitude asking:

> "Alex + Pack: does the empirical auto-classifier run reproduce
> §1's species_root verdict on `@knapsack`? Does TICK 11 close per
> Arc 3's bottom-up sequence? If yes: land the three-tick cascade
> 11a → 11b → 11c. If no: re-evaluate placement (family_root under
> its own algebra? species under @epistemologic?) and re-derive."

Promotion pending TICK 11 landing + independent Pack peer at the
three-shard shape.

---

## §13. References

Substrate ancestors (grep-verifiable):

- `docs/math/resource-budget/README.md` (this tick) — load-bearing math.
- `docs/math/kintsugi/README.md` + `compiler-error-surface.md` —
  three-mode algebra.
- `docs/math/spawn/spawn-as-loop-monad.md` — halting monad.
- `docs/math/liquid-types/README.md` (Mara `cbe063e`) — doc-as-declaration.
- `docs/math/prism-kind/README.md` (Mara `bdb148a`) — five-signal
  auto-classifier.
- `docs/math/onto/README.md` (Mara `d6a05ad`) — substrate-answerability.
- `docs/specs/doc-code-seam-bottom-up-landing.md` (this tick) — Arc 3.
- `docs/specs/kintsugi-variety.md` §4 — Knapsack framing.
- `docs/specs/gap-tension-tensor-substrate.md` §10 — round-level Lyapunov.
- `docs/specs/silicon.md` — @silicon family-root.
- `docs/specs/reality.md` §3.2.1 — @reality/algebra/silicon.
- `docs/specs/mirror-spec-schema.md` — project manifold grammar.
- `docs/specs/error-as-question.md` §2 — six-variant answer algebra.
- `shards/kintsugi.mirror` (Mara 2026-06-10) — family root.
- `shards/reality/algebra/silicon.mirror` (Mara 2026-07-01) — empirical
  discharge species with `compute_budget` crystal kind.
- `shards/epistemologic/cybernetic/variety.mirror` (Reed 2026-06-17) —
  Ashby variety vector.
- `shards/mirror/spec.mirror` — project manifold grammar.
- `shards/reflection.mirror` — the writeback discipline.

External references:

- Ibarra & Kim 1975 (JACM 22:463–468) — original FPTAS.
- Kellerer-Pferschy-Pisinger 2004 — canonical multi-dim knapsack ref.
- Frieze & Clarke 1984 (EJOR 15:100–109) — PTAS for `d=2`.
- Vazirani 2003 — textbook FPTAS derivation.
- Ashby 1956 §11/7 — Law of Requisite Variety.
- Roofline model — Williams-Waterman-Patterson 2009 (CACM).

Memory:

- `[[architecture-kintsugi-variety-io]]` — prior Ashby framing.
- `[[architecture-kintsugi-loop-altitude-portable]]` (#59).
- `[[architecture-fate-is-optical-inference]]` (#58).
- `[[architecture-hilbert-turing-godel-recognition-107]]` — sub-Turing.
- `[[architecture-form-process-partition-at-family-root]]` (#55).
- `[[architecture-mirror-store-vs-spectral-db]]`.
- `[[feedback-substrate-already-had-the-word]]`.
- `[[feedback-no-bare-types]]`.
- `[[feedback-craft-not-deliver]]`.
- `[[feedback-composition-claims-need-empirical-test]]`.
- `[[feedback-phantom-candidate-discipline]]`.
- `[[feedback-write-red-in-session]]`.

---

*2026-07-05. Mara. Derived spec. Not canonical. Self-audit: `opaque`.
Route: `spawn`. Empirical discriminator required (auto-classifier run
per §9) before TICK 11 landing.*
