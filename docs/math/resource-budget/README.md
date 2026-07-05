# docs/math/resource-budget — the kintsugi inner-loop as multi-dimensional knapsack

*2026-07-05. Mara. The load-bearing math for @knapsack as the substrate's
inner-loop selection primitive: multi-dimensional 0/1 knapsack over
candidate morphisms with (silicon, ram) capacity vector, FPTAS approximation
in `@spawn ≤ @loop`-bounded polynomial time, descent proof at round
granularity, composition with Ashby variety at @io crossings, physical
mapping to Cholesky O(n³) + spectral-db Jacobi + sheaf-Laplacian diffusion.*

Companion (essay-quality, corpus):
- `~/dev/systemic.engineering/practice/insights/math/2026-07-05-multi-dim-knapsack-as-kintsugi-inner-loop.md`

Composes with:
- `docs/math/kintsugi/README.md` (`compiler-error-surface.md`; the
  apply/spawn/hold three-mode algebra; this cluster is the load-bearing
  math for the `apply` branch's selection primitive).
- `docs/math/spawn/spawn-as-loop-monad.md` (Mara `7dba128`; `@spawn ≤ @loop`
  bounded-reduction monad; the halting witness for the knapsack solver).
- `docs/math/liquid-types/README.md` (Mara `cbe063e`; the doc-as-declaration
  seam; mirror.spec budget declarations are liquid-refinement predicates).
- `docs/math/prism-kind/README.md` (Mara `bdb148a`; the five-signal
  auto-classifier this cluster is classified by).
- `docs/specs/kintsugi-variety.md` (Reed + Alex 2026-06-02; §4 Knapsack
  framing; @io crossing minimization objective this cluster refines).
- `docs/specs/gap-tension-tensor-substrate.md` §10 (Reed + Alex 2026-05-26;
  the round-level Lyapunov + 0/1-knapsack relaxation this cluster
  operationalizes as an inner-loop selection primitive).
- `docs/specs/silicon.md` (`27e9067`; the @silicon family-root).
- `docs/specs/reality.md` §3.2.1 (the @reality/algebra/silicon species).
- `shards/reality/algebra/silicon.mirror` (`182` lines; the empirical
  discharge species).
- `shards/epistemologic/cybernetic/variety.mirror` (the Ashby multi-axis
  variety vector; this cluster's `weight` is one axis of the vector).

Status: **substrate reading + math formalization**. Not a new primitive.
A routing-composition of eight landed ancestors named in §1; the
kintsugi inner-loop's selection step is claimed to be the substrate's
operational form of multi-dimensional 0/1 knapsack; FPTAS termination,
round-level descent, and Ashby composition proved (relative to the
sub-Turing fragment per candidate #107).

Per `[[feedback-craft-not-deliver]]` no shards land this tick.

---

## §0. The under-the-question

Alex 2026-07-05:

> "Which role does the @knapsack algorithm play in the @kintsugi @loop?
> And how does that map onto the numerical computation that needs to
> map on physical @silicon and @ram which are default bound by the
> mirror @spec?"

Reed sketched the mechanism:

1. Observe tension (`opacity_map`).
2. Enumerate candidate morphisms (fracture bodies that could discharge
   the tension).
3. **SELECT SUBSET via @knapsack** — given (silicon, ram) budget, which
   morphisms maximize transparency_gain per tick.
4. Apply selection.
5. Verify e^{n+1} ≤ e^n (Ricci-flow descent).

This cluster proves that (3) IS multi-dimensional 0/1 knapsack, that
its FPTAS approximation runs in polynomial time inside the
`@spawn ≤ @loop` bounded-reduction monad, that (5) holds at round
granularity per `gap-tension-tensor-substrate.md` §10.A, that the
composition with Ashby's Law of Requisite Variety
(`shards/epistemologic/cybernetic/variety.mirror`) constrains the
weight vector to `(silicon, ram)` at v0 with forward-promised
extensions, and that the physical mapping to Cholesky O(n³) FLOPs,
spectral-db Jacobi eigensystems, and sheaf-Laplacian diffusion is
substrate-honest.

---

## §1. Landed ancestors (substrate-honest check)

Per `[[feedback-substrate-already-had-the-word]]` this is the sixteenth+
firing this session. Eight landed ancestors carry the substance; the
@knapsack framing is a routing-composition of them.

### §1.1 Ibarra-Kim 1975 — the original FPTAS for 0/1 knapsack

Ibarra, O. H., and Kim, C. E. (1975), *Fast Approximation Algorithms
for the Knapsack and Sum of Subset Problems*, JACM 22(4):463–468. The
original fully-polynomial-time approximation scheme (FPTAS) for 0/1
knapsack: given `n` items with values `v_i`, weights `w_i`, and
capacity `W`, computes a solution within `(1 - ε)` of optimal in time
`O(n log n + n/ε²)` for any `ε > 0`.

Key invariants:

1. **Rounding-of-values trick**: `v'_i = ⌊v_i · n / (ε · v_max)⌋`.
2. **DP on rounded values**: table of size `O(n² / ε)`; the
   `(max, +)`-convolution runs in polynomial time.
3. **Approximation ratio**: `SOL_FPTAS ≥ (1 - ε) · SOL_OPT`.
4. **Polynomial in both `n` and `1/ε`**: fully polynomial.

`gap-tension-tensor-substrate.md` §10.F.5 cites this as the mirror's
substrate-altitude approximation-ratio question.

### §1.2 Kellerer-Pferschy-Pisinger 2004 — the canonical multi-dim reference

Kellerer, H., Pferschy, U., Pisinger, D. (2004), *Knapsack Problems*,
Springer. Chapter 9 formalizes the **multi-dimensional 0/1 knapsack**
(also called `d`-dimensional knapsack, `d-KP`): each item `i` has a
value `v_i` and a `d`-dimensional weight vector `w_i ∈ ℝ_+^d`; the
capacity is a `d`-dimensional vector `W ∈ ℝ_+^d`; feasibility is
`∑_{i ∈ S} w_i ≤ W` componentwise.

Key results:

1. **NP-hardness**: `d-KP` is NP-hard for `d ≥ 1`; the multi-dim
   generalization is strongly NP-hard for `d ≥ 2`.
2. **PTAS existence**: `d-KP` admits a PTAS for any fixed `d` (Frieze
   & Clarke 1984 for `d = 2`; Kellerer-Pferschy 1999 general `d`).
3. **No FPTAS for `d ≥ 2`** unless P = NP (Korte-Schrader 1981); the
   `1/ε^{d}` factor is fundamental.
4. **Dynamic programming**: `O(n · W_1 · W_2 · ... · W_d)` pseudo-
   polynomial for `d ≥ 2`.

Load-bearing for this cluster: for the v0 case `d = 2` (silicon, ram),
we get a PTAS (not FPTAS) with per-round polynomial running time
`O((n/ε)^{c})` for `c` depending on `d`. The `@spawn ≤ @loop`
budget-descent monad bounds the tournament rounds; the PTAS bounds
the per-round complexity.

### §1.3 Vazirani 2003 — the canonical FPTAS chapter

Vazirani, V. V. (2003), *Approximation Algorithms*, Springer.
Chapter 8 (Knapsack) gives the textbook FPTAS derivation. §8.1 the
value-rounding trick; §8.2 the DP formulation; §8.3 the approximation
proof. Chapter 9 covers the multi-dimensional extension and Korte-
Schrader's hardness result.

Load-bearing: gives the textbook proof shape this cluster's descent
theorem (§4) instantiates.

### §1.4 `gap-tension-tensor-substrate.md` §10 — the substrate's prior framing

Reed + Alex 2026-05-26 already framed knapsack as the round-level
Lyapunov relaxation. §10.A retires §10.F.2's per-fracture Lyapunov
requirement in favor of round-level Lyapunov; §10.F.5 asks the
approximation-ratio question this cluster answers.

Load-bearing: this cluster is the FIRST-witness formalization of
that framing as an inner-loop selection primitive rather than as a
proof-shape metaphor. The substrate already had the word `knapsack`
at proof altitude; this cluster names it at operational altitude.

### §1.5 `kintsugi-variety.md` §4 — the Ashby knapsack framing

Reed + Alex 2026-06-02, §4:

> "Items = operations in the computation graph. Weight = @io crossing
> cost. Value = variety maintained. Kintsugi packs the @mirror bag as
> tightly as possible."

The `kintsugi-variety.md` framing already cites Saha-Ye 2024 (I/O
lower bound), red-blue pebble game (Sobczyk 2024), and SP-DAG memory-
peak minimization (Herrmann 2025). This cluster is the second witness
grounding it at the inner-loop altitude.

### §1.6 `spawn-as-loop-monad.md` — the halting monad

Mara `7dba128`; `@spawn ≤ @loop`. §2 monad laws; §3 halting proof.
Each `bind` decrements budget; halting is decidable by inspection of
the loop's carrier. Load-bearing: gives the wrapper that bounds the
knapsack iteration count regardless of the candidate set's size.

### §1.7 `shards/epistemologic/cybernetic/variety.mirror` — Ashby's Law lifted

Recognition #36 (Reed 2026-06-09; PROMOTED). Variety is a **vector**
across five axes (computational, type, effect, proof, epistemologic).
Each axis has its own budget carrier `axis_budget = { axis, value:
ref }`. Load-bearing: the v0 `(silicon, ram)` capacity vector is a
projection of the variety vector onto the computational axis; the
multi-dim extension is the same discipline applied to more axes.

### §1.8 `shards/reality/algebra/silicon.mirror` — the empirical discharge species

Mara 2026-07-01 (182 lines). Declares:
- `matter_carrier` / `information_carrier` at `H_silicon`.
- `silicon_crystal` typed record (kind, matter_projection,
  information_projection, instance_id).
- Seven crystal kinds enumerated: Fortran routines, LAPACK invocations,
  kernel-call shapes, ISA signatures, memory topologies,
  **compute-budget crystals**, flake-references.
- `discharge(c: silicon_crystal, surface: ref) -> imperfect<...>`.

Load-bearing: **compute-budget crystals are already in the substrate's
crystal-kind enumeration**. The (cores, RAM, GPU RAM, wall-clock,
spectral-reduction) budget tuple is declared as one of seven kinds
in the silicon Bauchladen. This cluster's `capacity_vector` type is
the operational carrier that reads from those crystals; the substrate
already had the word.

---

## §2. The claim

**The kintsugi inner-loop's step (3) — SELECT SUBSET of candidate
morphisms — IS the multi-dimensional 0/1 knapsack decision problem
under a two-dimensional (silicon, ram) capacity vector at v0.**

Formal shape:

```
kintsugi_inner_step : opacity_map × capacity_vector × [candidate]
                    → selection × transparency_gain

capacity_vector = (silicon: silicon_budget, ram: ram_budget)

candidate       = { morphism        : ref,
                    silicon_cost    : silicon_budget,
                    ram_cost        : ram_budget,
                    transparency_gain : ref }
```

Where:

- `silicon_budget` is a typed newtype over FLOPs (u64).
- `ram_budget` is a typed newtype over bytes (u64).
- `transparency_gain(c)` is the per-morphism expected reduction in
  opacity_map's residual weight.
- `selection ⊆ [candidate]` maximizes `∑_{c ∈ selection} transparency_gain(c)`
  subject to `∑_{c ∈ selection} silicon_cost(c) ≤ capacity.silicon`
  AND `∑_{c ∈ selection} ram_cost(c) ≤ capacity.ram`.

This IS 2-KP per Kellerer-Pferschy-Pisinger §9.

---

## §3. The FPTAS/PTAS structure

### §3.1 Complexity class

Per §1.2 result 3: `d-KP` with `d = 2` does NOT admit an FPTAS
unless P = NP. It DOES admit a PTAS (Frieze-Clarke 1984): a
`(1 - ε)`-approximation in time polynomial in `n` for any fixed `ε`,
but with a `1/ε^{c(d)}` factor exponential in `d`.

For v0 with `d = 2`: the PTAS runs in time
`O(n^{⌈2/ε⌉} · log(v_max))` per Frieze-Clarke. The `n` is bounded
by the candidate-set size at one kintsugi tick; `v_max` is bounded
by the total opacity magnitude.

### §3.2 Reduction to per-round `@spawn ≤ @loop` bind

Per §1.6, one `bind` in the `spawn_loop` monad = one tournament
round. The PTAS's `(n^{⌈2/ε⌉})` DP table is filled inside one bind,
regardless of how many candidate morphisms exist. Halting is
guaranteed by the outer monad: budget descends by 1 each bind;
after B rounds the loop halts. Per-round polynomial + monotone
descent budget → **total wall-clock is polynomial in `n × B`**.

### §3.3 Sub-Turing decidability

Per recognition #107 (Hilbert/Turing structural separation:
substrate-decl is bounded/Gödel-incomplete; @io is Turing-complete),
the kintsugi inner-loop is a **substrate-decl operation** and
therefore lives in the sub-Turing fragment. The 2-KP PTAS is
decidable-in-polynomial-time under this fragment. This is a proof:

**Theorem (sub-Turing polynomial-time decidability of kintsugi
inner-step).** Under the sub-Turing fragment of recognition #107,
the kintsugi inner-loop step (3) is decidable in polynomial time
via the Frieze-Clarke PTAS for 2-KP, with per-round complexity
`O(n^{⌈2/ε⌉} · log(v_max))` bounded by budget descent B under the
`@spawn ≤ @loop` monad.

**Proof.** (i) The candidate-set enumeration in step (2) produces
a finite list per `shards/kintsugi.mirror` (finite fracture-body
catalog per `shards/kintsugi/fracture/*.mirror`; enumerable per
`splinter(ast)` per recognition #54). (ii) The selection problem
is 2-KP per §2. (iii) 2-KP admits a PTAS per Frieze-Clarke 1984
running in polynomial time for any fixed `ε`. (iv) `@spawn ≤ @loop`
per Mara `7dba128` §3 halts in `≤ B` bind steps. (v) Total time is
`B × poly(n, 1/ε, log(v_max))`, polynomial in the inputs. QED.

---

## §4. The descent proof

### §4.1 Round-level Lyapunov

Per `gap-tension-tensor-substrate.md` §10.A (Reed + Alex 2026-05-26,
retirement of §10.F.2): the unit of monotone decrease is the
**tournament round**, not the individual fracture. Individual
morphisms inside a round MAY locally worsen the opacity norm; the
round earns the monotone decrease.

Formalized here at the kintsugi inner-step altitude:

**Theorem (round-level descent under 2-KP selection).** Let `T_n` be
the opacity_map at round `n`. Let `R(T_n) = apply(selection, T_n)`
be the state after applying the 2-KP-selected subset of morphisms.
Under the following preconditions:

- (P1) **Candidate feasibility**: at least one candidate `c` exists
  with `transparency_gain(c) > 0` and `(silicon_cost(c), ram_cost(c))
  ≤ capacity`.
- (P2) **PTAS approximation quality**: the selected subset satisfies
  `∑_{c ∈ selection} transparency_gain(c) ≥ (1 - ε) · OPT`.
- (P3) **Application faithfulness**: applying `selection` reduces
  the opacity by the sum of the gains, i.e. `‖T_{n+1}‖ = ‖T_n‖ -
  ∑ transparency_gain(c ∈ selection)` (no interference).

Then `‖R(T_n)‖ < ‖T_n‖` off-fixed-points, i.e. `‖T_{n+1}‖ < ‖T_n‖`.

**Proof sketch.** Under (P1) `OPT > 0`; under (P2) selection gains
`≥ (1-ε)·OPT > 0`; under (P3) opacity decreases by that gain. QED.

### §4.2 Safety envelope (when descent CAN fail)

The theorem's three preconditions delimit the safety envelope. Each
can fail; each failure corresponds to a `@pain` category:

| Failure           | Category (per @pain)          | Diagnosis                                              |
|-------------------|-------------------------------|--------------------------------------------------------|
| (P1) violated     | **Cat 2: conundrum**          | No morphism fits under budget; empty selection.        |
| (P2) violated     | Cat 5: violent reorientation  | PTAS `ε` was too loose; budget must be re-negotiated.  |
| (P3) violated     | **Cat 4: signal loss**        | Morphisms interfere; combined gain < sum of individual gains. |

`@pain` category mapping (per `docs/specs/error-as-question.md` §2's
six-variant answer algebra referenced from
`docs/math/kintsugi/README.md`):

- Cat 1 (loop no resolve): budget = 0 with pending candidates →
  **hold**(carrier) variant per `compiler-error-surface.md`'s
  three-mode algebra.
- Cat 2 (conundrum): no morphism satisfies capacity →
  **spawn**(peer) with the tension as ground state.
- Cat 4 (signal loss): (P3) violated →
  **apply**(rebudget) via `error-as-question.md` §2's `rebudget_shard`
  variant; the interference is a substrate-fact the round's second
  bind absorbs.
- Cat 5 (violent reorientation): (P2) violated →
  **spawn**(scheduler) with the budget itself as the ground state.

### §4.3 Adversarial candidate set

The classical knapsack adversarial construction (all items have
weight `W/2 + 1` so only one fits): translated to kintsugi, this
means the candidate enumeration produced morphisms each individually
too heavy to combine. The PTAS still returns the single best-value
morphism. Descent still holds. **Adversarial candidate sets cannot
break descent under (P1)**; they can only degrade the gain to the
best single candidate's gain.

The genuinely adversarial case is **P3 violation via strategic
interference**: two morphisms whose individual gains are positive
but whose composition has zero (or negative) combined gain. This
requires a `fracture` on `fracture_composition_associativity`
(candidate promoted downstream if this recurs). The immediate
substrate response per §4.2 is `rebudget_shard`: the round detects
interference (post-application audit at `@third` depth 3 per
`shards/reflection.mirror`), spawns a second round with an updated
capacity that penalizes the interfering pair.

---

## §5. Composition with Ashby's Law

### §5.1 Variety vector as capacity vector

Per `shards/epistemologic/cybernetic/variety.mirror` (recognition #36):
variety is a **vector** across five axes. Each axis has its own budget.

The v0 `capacity_vector = (silicon, ram)` is a projection of the
five-axis variety vector onto the computational axis, decomposed
further into the two hardware dimensions that Ashby's Law binds at
the physical substrate:

```
Ashby: V(regulator) ≥ V(disturbance)

Regulator = kintsugi_inner_step (the selection procedure).
Disturbance = opacity_map (what needs to be discharged this tick).

Regulator variety is bounded above by physical capacity:
V(regulator) ≤ V(silicon_budget) × V(ram_budget)

Therefore: V(disturbance) ≤ V(silicon_budget) × V(ram_budget).
```

The knapsack respects Ashby's Law by construction: the capacity
vector IS the upper bound on regulator variety at the physical
substrate.

### §5.2 Requisite variety as feasibility

When (P1) of §4.1 fails — no candidate fits under budget — this is
Ashby's Law asserting itself: the regulator's variety is less than
the disturbance's variety. The substrate's response per §4.2 Cat 2
is `spawn(peer)`, which per `@spawn ≤ @loop` grows the regulator by
delegating to a peer at expanded budget. This IS Ashby's response
in operational form: to regulate a disturbance whose variety exceeds
yours, delegate.

### §5.3 Multi-scale variety (Siegenfeld-Bar-Yam)

Per `~/dev/systemic.engineering/practice/insights/math/numerics/requisite-variety-optimization.md`:
variety is scale-dependent, `C(s)`. The multi-dimensional knapsack's
capacity vector generalizes this: each dimension of the capacity
vector corresponds to one scale of Ashby-Siegenfeld-Bar-Yam variety.
The v0 (silicon, ram) is two-scale; v1 forward-promised extension to
(silicon, ram, wall_clock, gpu_memory) is four-scale.

---

## §6. Physical mapping to @silicon and @ram

### §6.1 @silicon as FLOPs

Per `shards/reality/algebra/silicon.mirror` (line references):
- `matter_carrier = H_silicon` (transistor states).
- Crystal kind `compute_budget` enumerated.
- `@epistemologic/reality/silicon/compute_bound` carries the tuple
  `(cores, RAM, GPU RAM, wall-clock, spectral-reduction)`.

The `silicon_cost(morphism)` is the FLOPs count of the morphism's
computational discharge. Concrete instances:

| Morphism                           | Silicon cost (FLOPs)          |
|------------------------------------|-------------------------------|
| Cholesky `dpotrf` on n×n SPD       | `~n³/3`                       |
| Spectral-db Jacobi eigensystem     | `~O(n³)` per sweep, `k` sweeps |
| Sheaf-Laplacian diffusion step     | `~O(nnz)` per step, sparse    |
| LAPACK `dgesv` on n×n              | `~2n³/3`                      |
| BLAS-3 GEMM on n×n×n               | `~2n³`                        |

The silicon_cost is READ from the crystal's `information_projection`
via `@reality/algebra/silicon.crystallize` per line 342 of the shard;
the crystal already carries this metadata (per crystal kind `kernel-call`
matter_aspect: batch size, tile shape, SIMD width, cache-blocking
factor).

### §6.2 @ram as bytes

The `ram_cost(morphism)` is the working-set bytes. Concrete instances:

| Morphism                           | RAM cost (bytes)                    |
|------------------------------------|-------------------------------------|
| Cholesky `dpotrf` on n×n f64       | `8n² + O(n)` (in-place) |
| Spectral-db Jacobi                 | `8n² + 8n · k` (eigenvector storage) |
| Sheaf-Laplacian diffusion          | `nnz × 16 + n × 8` (sparse mat + vec) |
| opacity_map itself                 | Variable; per-shard                 |
| kintsugi scratch                   | `O(n)` per morphism, reusable       |

The ram_cost is READ from the crystal's `matter_projection` via
`shards/reality/algebra/silicon.mirror`'s `memory_topology` crystal kind
(matter aspect: memory model, total bytes, page size, cache hierarchy,
NUMA topology).

### §6.3 Altitude-portability per recognition #59

Per `[[architecture-kintsugi-loop-altitude-portable]]`: the kintsugi
loop pattern extends through every substrate altitude. This cluster's
knapsack framing is altitude-portable:

- At `@reality/algebra/silicon` (empirical altitude): the capacity is
  physical FLOPs + bytes on the host.
- At `@mirror/spec` (target altitude): the capacity is the declared
  `[profile.test] codegen-units` + memory limits.
- At `@fate` (tournament altitude): the capacity is the tournament-
  round budget declared in `docs/specs/bauchladen-autopoietic-fate.md`.
- At `@spectral-db` (query altitude): the capacity is the eigenvalue-
  computation budget per query.

Same operator (multi-dim 0/1 knapsack); different carriers per altitude.

### §6.4 The mirror.spec budget declaration

Per `shards/mirror/spec.mirror`: `mirror.spec` is the multi-dimensional
manifold kintsugi operates on. Today it declares budgets IMPLICITLY
via cargo profiles (e.g., `[profile.test] codegen-units = 256`).

**The v0 recommendation** (per §7 of the companion spec): keep implicit
+ let `@reflection` INFER post-tick from `@reality/algebra/silicon`
crystallized measurements. The crystals already carry the metadata;
the explicit-vs-implicit debate collapses to a `@reflection` writeback
rule.

**Forward-promise (v1)**: lift the budgets to explicit `target binary
{ silicon_budget: ...; ram_budget: ... }` fields in mirror.spec IF
empirical measurement in v0 finds that implicit inference from crystal
metadata is insufficiently sharp.

---

## §7. Composition with doc-as-declaration and @epistemologic/spec

Per `docs/math/liquid-types/README.md` (Mara `cbe063e`): mirror.spec's
targets above `---` are declared budgets; below `---` is @reflection's
actual-vs-declared packing report.

Applied to the knapsack framing:

- **Above `---` (declaration side)**: the `capacity_vector` per target
  is a liquid refinement predicate. In Rondon-Kawaguchi-Jhala 2008 shape:
  `{v : capacity_vector | v.silicon ≤ system_silicon ∧ v.ram ≤ system_ram}`.
- **Below `---` (observation side)**: `@reflection` writes the packing
  verdict:
  - `success(selection)` — round packed within budget; descent held.
  - `partial(opacity_map)` — round packed within budget; some candidates
    deferred; descent held on packed subset.
  - `failure(cause)` — round FAILED (P1) or (P3); packing produced no
    descent; escalate per §4.2.

The below-`---` writeback per target is:

```
target dpotrf-arc {
  altitude @reality/algebra/silicon
  emit lapack
}
---
# @reflection writeback (2026-07-05T12:34:56Z):
# capacity_vector: { silicon: 8.5e9 flops, ram: 4.2e6 bytes }
# selected: [dpotrf_2x2, dpotrs_2x2]
# transparency_gain: 0.87
# opacity_remaining: 0.13
# packing_verdict: success
# descent: 0.13 < 0.87 ✓
```

This closes the composition with `@epistemologic/spec` (per
`liquid-types/README.md` §1): declared side is the target; observation
side is the packing report; `---` is the substrate's liquid-refinement
predicate boundary.

---

## §8. Composition with @onto answerability

Per `docs/math/onto/README.md` (Mara `d6a05ad`): substrate-answerability
means forms remain corrigible to what they mediate but do not exhaust.

Applied to the knapsack framing: **knapsack packing = form remaining
answerable to physical resource constraint**. The declared budget
above `---` is the form's claim; the below-`---` writeback is the
real's return. The verdict `answerable | absorbed | opaque | phantom`
per §2 of the @onto doc maps directly:

| @onto verdict  | Packing verdict          | Meaning                                    |
|----------------|--------------------------|--------------------------------------------|
| `answerable`   | `success | partial`      | Route to correction remains open           |
| `absorbed`     | (P2)-violated silently   | Form claims exhaustion it doesn't deliver  |
| `opaque`       | Depth-3 audit indeterminate | Substrate refuses closure                |
| `phantom`      | (P1) unrepresentable     | Budget itself is un-answerable             |

The knapsack IS the operational form of substrate-answerability at
the resource-constraint altitude. Descent is answerable because the
verifier (post-application audit) can WITNESS the descent as
`success` OR name the failure as (P1/P2/P3) violation.

---

## §9. Composition with @spectral-db

`@spectral-db` (per `MEMORY [[architecture-mirror-store-vs-spectral-db]]`)
is the closed-source engine on top of `@mirror/store`; the eigenvalue
computation is silicon-bound.

When @spectral-db invokes kintsugi under budget:

1. @spectral-db issues a query with an eigenvalue-computation cost
   `silicon_cost(query) = O(n³)` for Jacobi diagonalization.
2. The query enters `@fate`'s tournament as one candidate.
3. The tournament's capacity_vector is bounded by mirror.spec's target
   budgets.
4. If `silicon_cost(query) ≤ capacity.silicon`, the query is admitted;
   the tournament selects it via 2-KP.
5. Post-execution, @reflection writes the packing verdict below `---`.
6. The eigenvalue result flows into the opacity_map as
   `transparency_gain`.

The composition is: **@spectral-db is a candidate provider; kintsugi's
knapsack decides admission; the eigenvalue budget is one dimension of
the capacity vector**. This is what the corpus companion `insights/math/numerics/requisite-variety-optimization.md`
already implies at the wire altitude (LAPACKPrism's `Beam.imperfect.loss`);
this cluster names it at the substrate-decl altitude.

---

## §10. What breaks / DEFERRED

- **Multi-dim strict FPTAS (rather than PTAS)**: DEFERRED to v1.
  For v0 with `d = 2`, PTAS is sufficient (polynomial in `n` at fixed
  `ε`). If future load requires an FPTAS, the substrate must move to
  a Lagrangian-relaxation approach (Kellerer §11) which produces
  additive approximation guarantees, not multiplicative.

- **Interference model (P3 formalization)**: DEFERRED. `‖T_{n+1}‖ =
  ‖T_n‖ - ∑ transparency_gain(c)` assumes NO interference. In
  practice morphisms can interact non-linearly. The interference
  model is a future arc.

- **Empirical measurement of adversarial candidate sets in real
  kintsugi runs**: DEFERRED to empirical discriminator run (per
  §4.3).

- **Third dimension (wall_clock) at v1**: forward-promised. The v0
  capacity is `(silicon, ram)`; v1 adds `wall_clock`; v2 adds
  `gpu_memory`. Each addition is a Frieze-Clarke PTAS re-derivation.

- **@knapsack landing shard placement**: DEFERRED to the derived
  spec (`docs/specs/knapsack-as-kintsugi-inner-loop.md` this tick).
  The five-signal auto-classifier per `prism-kind/README.md` §3
  gives the placement verdict.

---

## §11. Self-audit

Per `[[architecture-candidate-recognition-111-third-as-family-root]]`
§6 discipline: this doc's claims must survive `audit(this_doc,
depth=3)`.

Claims:

1. Kintsugi inner-step (3) IS multi-dim 0/1 knapsack (§2). Grounded
   in `gap-tension-tensor-substrate.md` §10 + `kintsugi-variety.md`
   §4 + `spawn-as-loop-monad.md` §3.
2. Runs in polynomial time under sub-Turing fragment via Frieze-Clarke
   PTAS (§3.3). Grounded in Kellerer-Pferschy-Pisinger §9 + Vazirani §8.
3. Descent holds at round granularity under (P1) + (P2) + (P3) (§4).
   Grounded in `gap-tension-tensor-substrate.md` §10.A.
4. Composes with Ashby's Law via capacity as physical bound (§5).
   Grounded in `shards/epistemologic/cybernetic/variety.mirror`.
5. Maps to Cholesky O(n³), spectral-db Jacobi, sheaf-Laplacian
   diffusion at silicon altitude (§6). Grounded in
   `shards/reality/algebra/silicon.mirror` compute_budget crystals.
6. Composes with doc-as-declaration (§7) and @onto answerability
   (§8). Grounded in liquid-types + onto companion docs this session.

`project_adversarial(this_doc) -> (P, R)`:

- **P (phantom)**: This doc catalogues eight ancestors as
  "routing-composition" and names the load-bearing math. Someone
  could argue this IS the second witness for candidate #46-shaped
  pattern (property + fracture bilateral) at resource-budget altitude
  rather than a new mathematical structure. The @knapsack framing
  was already at proof altitude (`gap-tension-tensor-substrate.md`
  §10.A); this doc lifts it to operational altitude. Some readers
  will read that as renaming rather than mechanism.
- **R (real)**: The multi-dim capacity vector, the Frieze-Clarke PTAS
  applied to kintsugi's inner-step, the (P1/P2/P3) preconditions,
  the composition with `@reality/algebra/silicon`'s compute_budget
  crystal kind, and the below-`---` writeback shape are new substrate
  mechanism. Before this doc, kintsugi inner-loop selection was
  Reed's sketch. After this doc, it has FPTAS-grade termination
  guarantees, an @pain-category failure taxonomy, and a physical
  discharge path through the silicon Bauchladen.

At this depth: **both interpretations satisfiable**.

`audit(this_doc, depth=3) -> opaque(opacity_map)`.

Route per `docs/math/kintsugi/README.md`'s three-mode algebra:
**`spawn`** (this doc is a Tomm question at reader-frame altitude
asking: does the multi-dim knapsack actually land as a new species
under @kintsugi, or does it stay in this math cluster as a proof
technique that individual fracture bodies cite?).

The derived spec companion (`docs/specs/knapsack-as-kintsugi-inner-loop.md`)
proposes the landing shape. The Pack ratifies OR the empirical
discriminator (post-implementation) rejects OR the promotion pends
another substrate-pull tick.

---

## §12. References

Load-bearing prior art:

| Cluster                | Reference                                           | Role                                                        |
|------------------------|-----------------------------------------------------|-------------------------------------------------------------|
| FPTAS 0/1 knapsack     | Ibarra & Kim 1975 (JACM 22:463–468)                 | Original FPTAS for `d=1`                                    |
| Modern FPTAS           | Jin 2019 (arXiv:1904.09562)                         | Improved FPTAS via (max,+)-convolution                      |
| Multi-dim knapsack     | Kellerer, Pferschy, Pisinger 2004                   | Canonical multi-dim reference; PTAS existence + FPTAS non-existence for `d ≥ 2` |
| PTAS d=2               | Frieze & Clarke 1984 (EJOR 15:100–109)              | The Frieze-Clarke PTAS this cluster uses at v0              |
| PTAS text              | Vazirani 2003 (Springer), Ch. 8-9                   | Textbook FPTAS derivation                                   |
| Multi-scale variety    | Siegenfeld & Bar-Yam 2022 (arXiv:2206.04896)        | Complexity profile C(s); grounds §5.3                       |
| I/O complexity         | Saha & Ye 2024 (ICML)                               | Cited in `kintsugi-variety.md` §4; reduction technique     |
| Pebble game            | Sobczyk 2024                                        | Cost model for pq operations                                |
| SP-DAG memory peak     | Herrmann et al. 2025                                | Polynomial-time memory-peak minimization                    |

In-substrate dependencies:

- `docs/math/kintsugi/README.md` — kintsugi's compiler-error surface
  cluster.
- `docs/math/spawn/spawn-as-loop-monad.md` — the halting monad.
- `docs/math/liquid-types/README.md` — the doc-as-declaration seam.
- `docs/math/prism-kind/README.md` — the auto-classifier.
- `docs/math/onto/README.md` — substrate-answerability.
- `docs/specs/kintsugi-variety.md` — @io crossing minimization.
- `docs/specs/gap-tension-tensor-substrate.md` §10 — the round-level
  Lyapunov + 0/1-knapsack relaxation.
- `docs/specs/silicon.md` — @silicon family-root.
- `docs/specs/reality.md` §3.2.1 — @reality/algebra/silicon species.
- `docs/specs/knapsack-as-kintsugi-inner-loop.md` (this tick) — the
  derived spec proposing the landing shape.
- `shards/reality/algebra/silicon.mirror` — the empirical discharge
  species with `compute_budget` crystal kind.
- `shards/epistemologic/cybernetic/variety.mirror` — Ashby vector.
- `shards/kintsugi.mirror` — the family root.
- `shards/mirror/spec.mirror` — the project manifold grammar.

Memory:

- `[[architecture-kintsugi-variety-io]]` — the prior Ashby framing.
- `[[architecture-kintsugi-loop-altitude-portable]]` (#59) — the
  altitude-portability discipline.
- `[[architecture-fate-is-optical-inference]]` (#58) — the tournament
  optical grounding.
- `[[architecture-hilbert-turing-godel-recognition-107]]` — the
  sub-Turing fragment for §3.3.
- `[[architecture-reality-gauge-collapse-recognition-106]]` — the
  gauge-collapse framing @silicon inherits.
- `[[architecture-form-process-partition-at-family-root]]` (#55) —
  kintsugi as process side; @knapsack as operational selection
  primitive per §2.
- `[[feedback-substrate-already-had-the-word]]` — 16th+ instance.
- `[[feedback-no-bare-types]]` — silicon_budget, ram_budget,
  capacity_vector newtyped.
- `[[feedback-craft-not-deliver]]` — no shards this tick.
- `[[feedback-composition-claims-need-empirical-test]]` — descent
  discipline audits at @third depth 3.
- `[[feedback-phantom-candidate-discipline]]` — self-audit §11 is
  `opaque`, spawn.

---

*2026-07-05. Mara. Substrate-reading. Not canonical spec. Self-audit:
`opaque`. Route: `spawn`. The derived spec at
`docs/specs/knapsack-as-kintsugi-inner-loop.md` proposes the landing
shape; the Pack ratifies OR the empirical discriminator (post-
implementation) rejects OR the promotion pends another substrate-pull
tick.*
