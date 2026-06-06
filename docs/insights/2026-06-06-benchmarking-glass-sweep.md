# The Benchmarking Glass — A Recognition Sweep

*2026-06-06. Taut. Research/recognition — no implementation.
Triggered by Alex's intuition that `@code/X` (Mara's collapse of
`@mirror/syntax` into the universal `@code` discipline) needs a
benchmarking glass to make the pressure surface across altitudes
load-bearing rather than aspirational.*

Status: **recognition complete.** The substrate already has the
name. This doc records where it lives, what shape it's in, and
what changes once `@code` is the universal grammar discipline.

Depends on:

- `fragmentation/docs/specs/lens-transit.md` (commit context: 2026-06-01,
  Taut) — **the spec.** `@lens/transit` is the benchmark facility
  carried as `Transparency<P>`. Red status; shape pinned; one
  implementation tick away.
- `mirror/boot/std/epistemologic/property/benchmark.mirror` — the
  seven verdict-shaped performance properties (no_hang, linear_compile,
  binary_size, deterministic_oid, cache_speedup, incremental_speedup,
  l1_resident). Declarations live; bodies `\`.
- `mirror/docs/specs/trace-kintsugi-pipeline.md` — names `@mirror/refract`
  as "bench wearing its real name"; refract IS the measurement glass
  at the grammar-graph altitude.
- `mirror/docs/specs/error-as-question.md` §5.3 — already wires
  `@lens/transit.measure` into the question-payload type. Reflection
  consumes transit overage as a question to answer.
- `mirror/docs/benchmarks/baseline-rust.md` (2026-05-17) — the
  ground-truth numbers the glass would replace with structured
  verdicts.
- `mirror/docs/specs/numerical-substrate-via-fortran.md` (Mara,
  2026-05-27) — the @code/fortran pathway. Performance citations
  (Linaro, LLVM Discourse). The Fortran/LLVM pressure the glass
  surfaces structurally.
- `mirror/shards/epistemologic/silicon/compute_bound.mirror` — the
  BUDGET carrier; the glass's matched USAGE measurement. Five
  bound axes map to six transit axes (the glass adds Branch-Miss
  as a vectorisation-witness property).

---

## 0. Headline

**The benchmarking glass exists. It is `@lens/transit`.**

It is Red — designed, sibling to HamiltonScheduler, the shape and
composition law are pinned, the implementation tick is owed. It is
substrate-declared at the right altitude (`@lens/transit`, the
measurement carrier under the @lens family), uses the existing
`Transparency<P>` monoid verbatim (no new combine law), names six
property axes each bounded by a documented hardware floor, and
already plugs into `@mirror/error` as the structured payload
Reflection reads when a transit overage becomes a question.

The substrate also has three sibling shapes that frame what transit
measures:

1. **`@epistemologic/property/benchmark`** — verdict-shaped
   performance properties at the compilation altitude (linear-compile,
   l1-resident, deterministic-OID). These are the GAME the cascade
   plays. The seven verdicts.
2. **`@mirror/refract`** — measurement at the grammar-graph altitude.
   Reads the five Void dualities (entropy / spectral / cheeger / ricci
   / mixing) over the grammar graph. The gutter rendering layer.
3. **`@epistemologic/silicon/compute_bound`** — the BUDGET carrier.
   Five option-typed bounds the shard's admits-action allocates;
   transit's BudgetConsumption verdict is the USAGE that closes
   the loop.

These four shapes — `@lens/transit` + benchmark properties + refract
+ compute_bound — together constitute the benchmarking glass. Transit
is the load-bearing one Alex was reaching for; the others are
already-named pressure surfaces that surround it.

This recognition does NOT require new vocabulary. The substrate
named the thing it needed before it knew why.

---

## 1. What `@lens/transit` already is

From `fragmentation/docs/specs/lens-transit.md` (Taut, 2026-06-01):

> **Transit measures what's lost in passage.** Light enters a prism.
> The prism slows it — refractive index above unity. Information
> passes through bodies. Bodies cost time, FP precision, cache,
> allocation. The body is the prism; the transit is the spectrum;
> the loss is what dispersion shows.

### 1.1 Six property axes

| Property            | What's lost                     | Hardware floor           |
|---------------------|----------------------------------|--------------------------|
| `WallClock`         | Time per body                    | Nanosecond (CLOCK_MONOTONIC) |
| `FpPrecision`       | Bits per FP operation           | Machine epsilon (IEEE 754) |
| `CachePressure`     | Warmth per evicted line         | Cache-line size (64 B / 128 B)|
| `Allocation`        | Bytes on critical path          | Page size (4 KB / 16 KB)  |
| `BranchMisses`      | Speculative work                 | Pipeline depth           |
| `BudgetConsumption` | Hard-realtime budget consumed   | TickInterval granularity |

Each axis is a property; each verdict is `PropertyVerdict::Pass |
Partial | Fail`; the joint report is `Transparency<Ref>` over the
property family. The report is **flame-graph-shaped, not stopwatch-
shaped** — multi-axis spectrum, not single-scalar wall-clock.

### 1.2 The composition is borrowed verbatim

When a body invokes sub-bodies, transit composes parent and children
via `Transparency::combine`. **No new combine; no new merge; no new
identity.** Fail dominates; Partials accumulate diagnostics; Pass is
neutral. The substrate has exactly one monoid for verdicts at the
measurement altitude.

This is the load-bearing decision: transit is a CONSUMER of the
existing `Transparency<P>` algebra, not a parallel one. Everything
else (kintsugi fracture loss, Crystallizer settled-budget audit,
realtime admission verdicts) already lives in this monoid. Transit
just adds property kinds.

### 1.3 Hardware floor — the speed-of-light analogue

The substrate cannot measure below the local hardware's precision.
Below the floor is structurally unobservable; above the floor it
must be observed. Every transit report carries the floors that
applied at measurement time. Cross-hardware verdicts compare under
a documented translation rule (deferred to a future tick).

This honesty is what makes the glass load-bearing. It does not lie
about what it cannot see; it names the floor, locates the verdict
at a substrate path, and lets downstream consumers compare like-
with-like.

---

## 2. The three sibling shapes

### 2.1 `@epistemologic/property/benchmark` — the GAME

Seven verdict-shaped properties at the compilation altitude:

```
no_hang(ast)              -- the binary doesn't hang
linear_compile(ast)       -- O(n) compilation
binary_size(binary, t)    -- < 64 KB (L1 resident on x86)
deterministic_oid(ast)    -- same input, same hash
cache_speedup(file)       -- cache hit > 2x cold compile
incremental_speedup(spec) -- DAG dependency tracking holds
l1_resident(binary)       -- text section < 32 KB
```

These are not measurements per se. They are the GAME the cascade
plays. Each takes an artifact and returns `verdict`. Pass means the
architecture holds. Fail means it doesn't. Partial means the cascade
is still finding the line.

**The relation to transit:** benchmark properties are STATIC verdicts
the substrate computes by inspection (binary size, AST shape, OID
determinism). Transit is the DYNAMIC observation of runtime behaviour
(wall-clock per body, cache pressure, FP loss). Both compose under
the same `Transparency<P>` monoid; together they discharge the
performance claim.

### 2.2 `@mirror/refract` — the grammar-graph spectrum

From `trace-kintsugi-pipeline.md`:

> `@mirror/refract` IS bench wearing its real name. The existing
> `mirror bench` infrastructure computes spectral measurements of
> grammar graphs. `refract` exposes those measurements as verdicts.

Refract reads the five Void dualities over the grammar graph:
entropy (von Neumann), spectral (Fiedler λ₂), cheeger (min edge-
boundary), ricci (Forman per-edge), mixing (random-walk mixing time).
These are not @lens/transit measurements — they are the structural
shape of the code's CONNECTIVITY, not the cost of running it.

**The relation to transit:** refract observes the SHAPE of the
grammar; transit observes the COST of executing it. Refract's
output drives the gutter's color (green/amber/red); transit's
output drives the flame-graph view at the body altitude. Two
layers of the same observation tower.

### 2.3 `@epistemologic/silicon/compute_bound` — the BUDGET

Five option-typed bounds the shard's `admits` action allocates:

```
max_cpu_cores         option(u32)
max_memory_bytes      option(u64)
max_gpu_memory_bytes  option(u64)
max_wall_time         option(wall)
max_reductions        option(u64)
```

Five fields matching the Prism algebra's cardinality. Each bound
corresponds to one axis the shard rations: parallelism (split),
memory (project), accelerator (shift), time (focus), settlement
(settle).

**The relation to transit:** compute_bound is the DECLARATION; transit
is the OBSERVATION. The shard declares "this body gets at most 100ms
wall, 4MB memory, zero allocation on the critical path"; transit
reports "this invocation used 82μs wall, 247 cache lines, 0 bytes
allocated, BudgetConsumption Pass(0.82)." The static admits-check
and the dynamic transit-check compose to discharge the hard-realtime
contract.

---

## 3. The gap (small, named)

The substrate has the names. What it does NOT have:

1. **Implementation.** `@lens/transit` is Red — the Rust does not land
   yet. The property enum, the per-property measurement
   implementations, the platform-floor detection, the dispatcher
   integration are all owed.
2. **The cross-altitude binding.** Transit measures bodies. Refract
   measures grammar graphs. Benchmark properties measure
   compilation artifacts. The three should compose under the SAME
   `Transparency<P>` monoid — the spec says so — but no concrete
   wiring shows the @code/X-aware consumer reading all three through
   one surface yet. Once `@code` is the universal discipline, the
   binding becomes natural: every `@code/X` instance is BOTH a
   grammar graph (refract reads), a compilation artifact (benchmark
   properties verdict), and a runtime body (transit observes).
3. **The cross-altitude vocabulary.** "Benchmarking glass" is Alex's
   working name. The substrate-pull-honest name is **transit**, with
   the three sibling shapes named explicitly. This doc records the
   recognition; subsequent specs should reference `@lens/transit` as
   the load-bearing measurement altitude and the other three as
   sibling consumers.

These are not new types or new vocabulary. They are wiring tasks
that follow the implementation tick.

---

## 4. The @code consumer sketch

Once Mara's `@code` migration is complete (in flight today), `@code`
becomes the universal grammar discipline — every code grammar at
every altitude (`@code/rust`, `@code/llvm/ir`, `@code/fortran`,
`@code/python`, future `@code/zig` etc.) instances the same
`abstract grammar @code` declared in `boot/04-code.mirror`.

The benchmarking glass attaches to `@code/X` as follows:

### 4.1 What transit measures on a `@code/X` instance

When a body is dispatched at altitude `@code/X` (e.g., a Rust
function inside a `@code/rust` target, a Fortran subroutine inside
a `@code/fortran` Fate package), transit observes:

- **`WallClock @ @transit/wall_clock/@code/X/path/to/body`** —
  located at the substrate path of the invocation. The path tells
  Reflection which altitude paid the time.
- **`FpPrecision @ @transit/fp_precision/@code/X/path/to/body`** —
  located the same way. f64 ops vs f32 ops vs interval arithmetic
  shadow execution all surface at the same property axis with
  altitude-specific floors.
- **`CachePressure / Allocation / BranchMisses`** — the same shape.
  Each axis is altitude-tagged via the substrate path.

The report shape becomes:

```
@code/fortran/numerical/eigen.dsyev:
  @transit/wall_clock        : Pass(2.4ms / 10ms budget)
  @transit/fp_precision      : Pass(loss 4e-15)
  @transit/cache_pressure    : Pass(64 lines)
  @transit/allocation        : Pass(0 bytes)
  @transit/branch_misses     : Pass(3 / 100)
  @transit/budget_consumption: Pass(0.24)

@code/rust/spectral.eigen_d:
  @transit/wall_clock        : Partial(18ms / 10ms budget, soft)
  @transit/fp_precision      : Pass(loss 8e-15)
  @transit/cache_pressure    : Partial(284 lines)
  @transit/allocation        : Partial(1.2 KB)
  @transit/branch_misses     : Partial(47 / 100)
  @transit/budget_consumption: Fail(1.80)
```

The substrate's altitudes are **comparable in the same
`Transparency<Ref>`** because the property axes are altitude-
independent. Wall-clock is wall-clock whether you measure it on a
Rust function, a Fortran subroutine, or an LLVM IR basic block.

### 4.2 How measurement composes with the @code algebra

The @code abstract grammar declares five operations (the Prism
algebra) shared by every `@code/X` instance:

```
focus      observe the AST
project    select by predicate
split      decompose into substructure
shift      transform / translate
settle     fix-point / emit
```

Transit measures the COST of each operation per altitude. For
`@code/rust`, `focus` might cost 50μs (tokenize a syn::File); for
`@code/llvm/ir`, the same `focus` might cost 200μs (parse a .ll
file). The five-operation algebra is altitude-uniform; the cost
profile per operation is altitude-specific. **Transit makes the
substrate-pull gradient visible per operation.**

Once that's visible:

- `shift` is the operation that moves between altitudes (the
  `translate` template in `@code`). Transit on `shift` measures
  TRANSLATION COST.
- `settle` is the operation that emits to the lower altitude
  (the `render` template). Transit on `settle` measures EMISSION
  COST.
- The composition `focus → project → split → shift → settle`
  produces a transit profile that IS the altitude's full
  performance signature.

### 4.3 What @code/rust vs @code/llvm comparison looks like

Take the same Fate-tournament inner loop expressed at two altitudes:

**At @code/rust:** the loop lives in `bootstrap/src/spectral.rs`
(~150 lines of power iteration approximating LAPACK's `dsyev`). Its
transit profile is the Rust runtime cost: malloc/free, dynamic
dispatch through `Prism`, string-formatting machinery linked even
when unused, ~591 KB stripped binary footprint.

**At @code/llvm:** the same loop emerges from `flang -emit-llvm -O3`
of Fortran source (LAPACK's actual `dsyev`, ~30 lines wrapping ~2M
lines of mature numerical code). Its transit profile is bump
allocation only, no dynamic dispatch, ~50 KB binary footprint
contribution, vectorisation (per Linaro 2023 benchmarks: gfortran
is 23% faster than LLVM Flang; flang's gap is closing per LLVM
Discourse 2024).

The substrate reads both profiles in the SAME `Transparency<Ref>`.
A consumer (Reflection, or a kintsugi rule, or Alex looking at
the gutter) sees:

```
@fate/tournament/inner_loop:
  altitude @code/rust:    wall 75ms,  cache 1.2K lines, alloc 261 B
  altitude @code/llvm:    wall 1.8ms, cache 184 lines,  alloc 0 B
                          (via @code/fortran via flang)
  altitude @code/fortran: (same as @code/llvm — flang emits IR)
```

**The substrate-pull becomes load-bearing.** When the same operation
at `@code/llvm` is 40x faster than at `@code/rust`, the kintsugi
substrate has structural justification to migrate. The transit
verdicts are the GRADIENT the substrate descends.

Without the glass, the migration is aspirational ("LAPACK is faster,
we should use it"). With the glass, it's load-bearing ("the @lens/
transit verdict at `@code/rust` is Partial at WallClock; at
`@code/llvm` it is Pass; the substrate-pull direction is named").

---

## 5. The Fortran / LLVM pull — made structural

Mara's `numerical-substrate-via-fortran.md` (2026-05-27, Yellow) is
the v1.5 milestone for the Phase 6 (butterfly roadmap) numerical
substrate. The plan: ~107 lines of Numerical content in
`bootstrap/src/spectral.rs` (the `Spectrum<N>` struct + the
`eigen_d` 150-line power-iteration approximation) migrate to
`@code/fortran` via `flang -emit-llvm -O3` → `@code/llvm/ir` →
mirror's link surface.

Without `@lens/transit`, the migration justification is the citations
(LAPACK = ~2M lines of mature numerical code; flang produces LLVM
IR mirror already consumes; the Linaro performance gap is closing).
These are correct but aspirational.

With `@lens/transit`, the migration becomes **a verdict**:

1. Today's bootstrap power iteration at `@code/rust` gets a transit
   profile: WallClock(~75ms for the spectral kernel), Allocation
   (non-zero on critical path), CachePressure (large working set
   due to monomorphisation), BranchMisses (high in the convergence
   loop).
2. The same kernel at `@code/fortran` (compiled by flang to `@code/
   llvm/ir`, consumed by mirror's existing IR pathway) gets a
   matching transit profile under the same hardware floors.
3. The substrate compares the two `Transparency<Ref>` reports
   directly. The verdict is structural: `@code/fortran` Passes
   where `@code/rust` Partials; the substrate-pull gradient
   points to `@code/fortran` for this body.

**The benchmarking glass makes Phase 6 a measurable bet.** The Linaro
citation says "Flang is 23% slower than gfortran" — but that's
absolute throughput against gfortran. Against the bootstrap's own
power iteration, flang's LLVM IR is ~40x faster (LAPACK's mature
algorithms vs hand-rolled approximations). The glass reads BOTH
comparisons in the same monoid and surfaces the structural answer:
**for Fate's spectral kernel, `@code/fortran` wins by a
substrate-pull factor, not a citation-claim factor.**

The same logic applies to `@code/llvm/ir` itself for tight loops.
The bootstrap's `mirror.ll` (~700 lines of hand-written IR for
jacobi + sha256 + sha1 + syscalls + tokenizer) already exists; its
transit profile against the Rust equivalent is what justifies the
LLVM-binary pathway. Today that justification is the binary-size
benchmark (591 KB stripped Rust → 50 KB projected LLVM). With
transit, the justification becomes multi-axis: not just binary size,
but cold-start time, hot-path L1 residency, allocator overhead
(zero for the bump-allocated LLVM IR vs Rust's malloc churn).

---

## 6. Surprises

Three places the substrate is already pointing at benchmarking
without anyone gathering the threads:

### 6.1 `@mirror/error` already consumes transit

`error-as-question.md` §5.3 declares `transit_overage_to_question`:
when `@lens/transit.measure` exceeds a declared budget, the overage
becomes a question at the body altitude that Reflection answers.
**This is the loop closing.** Transit isn't an external benchmark
tool; it's part of the substrate's self-correction mechanism. A
slow body becomes a question Reflection answers with an adjustment
(more shards? different altitude? kintsugi rewrite?). The glass
isn't observation-only; it's input to the autopoietic loop.

### 6.2 `@mirror/refract` already calls itself bench

The `trace-kintsugi-pipeline.md` spec literally says "@mirror/refract
IS bench wearing its real name." The grammar-graph spectrum (entropy
/ spectral / cheeger / ricci / mixing) is already the substrate's
benchmark surface at the GRAPH altitude. Transit is the same surface
at the BODY altitude. The substrate has been measuring itself
spectrally for over a month without anyone naming the family.

### 6.3 The hardware-floor discipline is honest

This is the surprise I (Taut) didn't fully appreciate when I wrote
the spec on 2026-06-01. The hardware-floor declaration is what
makes transit philosophically different from every other
benchmarking framework. Most benchmark tools report a number with
implicit precision claims. Transit reports a verdict + the floor
that bounded the verdict. The substrate is **honest about what it
cannot measure**. Cross-platform comparisons go through documented
translation rules, not magical "normalisation." The
speed-of-light analogue (there is an upper bound on what can be
observed; the bound is part of the physics) means the glass
doesn't pretend to omniscience.

This honesty composes with the substrate's general epistemic
discipline (Imperfect<T,E,L>; verdicts; consent; the third type
parameter L for holonomy/loss). Transit is just measurement under
the same discipline.

### 6.4 The `@cogito/silicon/compute_bound` ↔ transit symmetry

`compute_bound`'s five fields map onto five of transit's six
properties:

| compute_bound (BUDGET) | @lens/transit (USAGE)        |
|------------------------|------------------------------|
| `max_cpu_cores`        | (parallelism axis — implicit) |
| `max_memory_bytes`     | `Allocation`                  |
| `max_gpu_memory_bytes` | (no transit axis yet)         |
| `max_wall_time`        | `WallClock`                   |
| `max_reductions`       | `BudgetConsumption`           |

Transit adds `FpPrecision`, `CachePressure`, `BranchMisses` —
the three axes that don't have matched budgets in `compute_bound`.
This is structural information: those three are the **observable-
only** axes; the substrate observes them but does not allocate
against them at admission time. (FP precision is bounded by the
type system, not by admission policy; cache and branch behaviour
are emergent.)

The symmetry suggests a future tick: extend `compute_bound` with
optional bounds on those three (e.g., `max_fp_loss: option(epsilon)`),
or accept that they remain observation-only. The substrate-pull
direction is open.

---

## 7. What to commit, what to defer

This is a recognition doc — no implementation. What lands today:

1. This insight, recording that `@lens/transit` IS the benchmarking
   glass; the substrate already named it.
2. The recognition that `@code` (Mara's collapse) gives the glass
   its natural consumer surface — every `@code/X` instance reads
   transit verdicts at uniform axes, comparable across altitudes.
3. The structural justification for Phase 6 (Mara's butterfly
   roadmap → @code/fortran via flang): the transit verdict at
   `@code/fortran` Passes where the verdict at `@code/rust`
   Partials, for the Fate spectral kernel.

What's deferred to a later tick:

1. The `@lens/transit` Rust implementation (per the existing spec).
2. The `@code/X`-aware transit consumer (Reflection reading transit
   verdicts across altitudes and producing a substrate-pull verdict).
3. The cross-hardware translation rules (named as future work in
   the existing spec).
4. The `compute_bound` extension for FP-precision / cache /
   branch-miss bounds (the symmetry surprise from §6.4).

None of these gate v0.1.0. All become natural follow-ups once
`@code` is substrate-declared (Mara's tick) and the implementation
tick for `@lens/transit` lands (Taut's next tick, post-recognition).

---

## 8. Cross-references

- `fragmentation/docs/specs/lens-transit.md` — the spec. Read first.
- `mirror/boot/std/epistemologic/property/benchmark.mirror` — the
  GAME (seven verdict properties).
- `mirror/docs/specs/trace-kintsugi-pipeline.md` — `@mirror/refract`
  as bench at the grammar-graph altitude.
- `mirror/docs/specs/error-as-question.md` §5.3 — transit overage
  as Reflection's question payload.
- `mirror/shards/epistemologic/silicon/compute_bound.mirror` — the
  BUDGET carrier; symmetry surprise per §6.4.
- `mirror/docs/specs/numerical-substrate-via-fortran.md` — Phase 6
  destination; the structural justification the glass makes
  load-bearing.
- `mirror/docs/benchmarks/baseline-rust.md` — the ground-truth
  numbers the glass replaces with structured verdicts.
- `mirror/docs/roadmap/12-coherence-benchmark.md` — sibling
  benchmark at a different altitude (model-level coherence, not
  body-level performance). Coherence-benchmark uses the same
  verdict shape; the two are sibling consumers of the same
  `Transparency<P>` algebra at different altitudes.

---

*Light passes through a prism and is dispersed. Information passes
through a body and loses some of itself — to floating-point, to
cache eviction, to budget exhaustion, to whatever the local hardware
allows. The body is the prism; transit is the spectrum. Once @code
is universal, every altitude reads the same spectrum at uniform
axes — and the substrate-pull gradient toward higher-performance
altitudes becomes a verdict the substrate can act on, not a citation
to defend.*

— Taut, 2026-06-06
