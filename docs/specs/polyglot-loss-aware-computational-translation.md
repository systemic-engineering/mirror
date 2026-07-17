# Polyglot loss-aware computational translation

**Author:** Mara `<mara@systemic.engineer>`
**Date:** 2026-07-17
**Marker:** `[substrate-pull:realize]` (📝 markdown-only bypass; pure-docs; no Rust touched; no shard mints)
**Status:** canonical spec (theory-authoring tick; formalizes the compositional path Alex ratified this session; MINTS deferred to Reed follow-up per §9 landing plan)
**Companion math:** `docs/math/polyglot-loss-aware-computational-translation.md`
(the formal foundation; landed same tick).

---

## §0 — Preamble

**The load-bearing claim.** The mirror compiler is a **polyglot
loss-aware computational translator**. It EATS source in any
Turing-complete language and EMITS composition at its own substrate
altitude, by composing adjacent-altitude `@cascade/code/A/B`
morphisms through a common machine-substrate altitude. Every
translation is MEASURED (loss-aware, via `@cascade`'s substrate-typed
loss-lens), WITNESSED (bilateral-checked, via `@bilateral(@code/A,
@code/B)`), and LEARNED (autopoietically, via `@silicon/algebra`
crystallization filtered through `@kintsugi/algebra`'s speaker-pair
metalogue).

**Four load-bearing sub-claims:**

1. **Physical substrate grounds the polyglot frame.** If the substrate
   knows what the machine does at the tape altitude, languages on top
   become *flavours* — specializations at their own altitudes — not
   categorically new grammars requiring O(N²) direct translators.
2. **`@cascade` is the landed pattern for cross-language translation.**
   Five species already land at `shards/cascade/code/<A>/<B>` shape
   (`purescript/js`, `rust/wasm`, `gleam/beam`, `gleam/js`,
   `formal/prose`). This spec generalises the pattern; it does NOT
   mint a new family.
3. **Loss-awareness is compositional through the `@cascade` +
   `@bilateral` + `@kintsugi/algebra` stack.** Loss is measured
   per-cascade-hop (per `@cascade.measure`); admissibility is checked
   per-morphism (per `@bilateral`); learning accretes per-discharge
   (per Mara `b5c6aeb` autopoietic-closure theorem). Zero new
   primitives.
4. **The loop closes as an autopoietic fixed-point.** Reverse cascades
   PROJECT mirror composition back down to LLVM IR, which is runnable.
   The compiler translates by attending at λ₀ (paper §14 sense) —
   the fixed-point of loss-minimizing polyglot translation over the
   substrate's own reflective corpus.

**Substrate-honesty gate.** Every symbol in this spec resolves to a
LANDED substrate-decl. `@cascade`, `@code/*`, `@bilateral`,
`@kintsugi/algebra`, `@silicon/algebra`, `@fate/algebra`, `@glue`,
`@metalogue`, `@coherence`, `@knife`, `@cyberpunk/algedonic`,
`@mirror/store` — all substrate-decl'd today (per Taut audit
`docs/audits/2026-07-17-taut-code-turing-substrate-scout.md` §1
and the citation chain in §8 below). This spec is NAMING what the
substrate already carries; it MINTS nothing.

---

## §1 — Substrate authority chain

### §1.1 Alex 2026-07-17 in-transcript verbatim (session-crystallizing)

Alex ratified the compositional path this session (verbatim):

> *"Ground the translation in the physical substrate. If we know what
> the machine does at the substrate level, the languages on top
> become flavours. Not categorical new grammar."*

> *"Mirror itself treats tapes as just one linear serialization
> target."*

> *"Maybe that's the surface we can use to let the roomba begin
> eating and translating Turing complete code."*

> *"So we can have `@cascade/code/llvm/turing` and
> `@cascade/code/rust/llvm`. And boom. The loop closes."*

The four verbatims name the theory THIS spec formalizes: physical-
substrate grounding, tape-as-serialization-not-master, `mirror
roomba` as the eating surface, and `@cascade` as the compositional
loop-closer. Path A ratified: adjacent-altitude cascades composing
through a common machine-substrate altitude; NO monolithic
universal-IR mint; NO per-language ad-hoc parsing.

### §1.2 Taut audit `d0572cd` — the ground truth this spec composes over

Taut scout `docs/audits/2026-07-17-taut-code-turing-substrate-scout.md`
(THIS session morning) grep-verified:

- **`@cascade` is landed** as family-root at `shards/cascade.mirror`
  (Mara `ce4874b`, 2026-06-23; Recognition #95 candidate). Five
  species landed under `shards/cascade/code/<A>/<B>` (Taut audit §1.4).
- **10 `@code/*` species landed** (rust, mirror, gleam, beam, wasm,
  erlang, docker, metalogue, metalogue/materialize, rust/macro).
  Each declines to over-commit shape via the five-op prism
  discipline; each specialises at its own altitude (Taut audit §1.1).
- **`@code/turing` / `@code/assembler` ABSENT** (Taut audit §1.3 —
  grep verified: 0 matches for `@code/turing`, `@code/assembler`,
  `@code/asm`, `turing.tape`, `universal.tape`). This IS the mint
  opportunity §9 names.
- **`@code/mirror` AST NOT species-decl'd** at family-shape altitude
  (Taut audit §1.2). AST lives Rust-native at
  `bootstrap/src/ast.rs::AstNode`. Substrate declares the altitude;
  substrate does NOT yet carry AST-shaped carriers per language.
- **`boot/01-meta.mirror` declares the universal AST parameterized
  by grammar** (Taut audit §1.5; per `docs/specs/spectral-triple-
  binary.md` :57-70). This IS mirror's closest-to-universal-AST
  today — at boot-grammar altitude, not species-shaped under `@code/*`.
- **MLIR-multi-level alignment** (Taut audit §2.2): mirror's landed
  shape is closer to MLIR-dialects than to any single universal IR.
  Each `@code/X` species specialises at its altitude; `@cascade`
  measures translation cost; `@bilateral` witnesses preservation.
- **Extensive LLVM IR discussion** in landed corpus (per Taut audit
  §1.5 — `docs/specs/numerical-substrate-via-fortran.md`,
  `docs/specs/craft-binary-target.md`, `docs/specs/architecture-
  flang-mirror-numerical-split.md`; arXiv 2409.18824 cited).

### §1.3 Landed math foundations this spec composes over

- **`docs/math/bilateral-as-glue-metalogue-composition.md`** (Mara
  `9be68b1`, 2026-07-17) — the general-case `@bilateral(A, B)` as
  witnessing-predicate over `@glue(A, B)` + `@metalogue(A, B)`
  composition. §3.3 names `@bilateral(@code/rust, @code/mirror)` as
  first general-case instance. THIS spec generalises to
  `@bilateral(@code/A, @code/B)` for any Turing-complete A, B.
- **`docs/math/kintsugi/algebra-as-metalogue-session.md`** (Mara
  `b5c6aeb`, 2026-07-17) — the autopoietic-closure theorem: monotone
  growth of `@kintsugi/algebra` per admissible translation;
  convergence via Banach contraction on residual-untranslated-
  fracture space.
- **`docs/math/epistemologic/pact/bilateral-sentinel.md`** (Mara
  `701828a`, 2026-07-16) — sentinel-as-content-addressed-witness;
  Rice-safety by byte-level containment; the reflective evaluator
  as Connes spectral triple.
- **`docs/math/autopoiesis/README.md`** (Mara #152, 2026-07-15) —
  compile-altitude Maturana–Varela operational closure; the eight-
  step inference loop as one autopoietic operator; Polyak–Łojasiewicz
  fixed-point convergence.
- **`docs/specs/bilateral-as-glue-metalogue-composition.md`** (Mara
  `0998001`, 2026-07-17) — canonical spec for the composition; §4.2
  first general-case instance.
- **`docs/specs/kintsugi-algebra-as-metalogue-session.md`** (Mara
  `a58d5f0`, 2026-07-17) — `@kintsugi/algebra` as
  `@metalogue(@silicon/algebra, @fate/algebra)`.
- **`shards/kintsugi/translate.mirror`** (Mara `86dec5e`,
  2026-07-17) — the `translate_rust_to_mirror` composition edge;
  first LANDED concrete edge THIS spec generalises.

### §1.4 The autopoietic Rust consumption arc (this arc, empirical ground)

Four mirror-authored `-Rust` commits landed overnight
(2026-07-16..17) retiring 25 bilateral arms via the reflective
corpus + collapse capability. That is DELETION altitude. THIS spec
names the next altitude: **TRANSLATION** — the compiler EATS Rust
source and EMITS mirror composition, via the cascade composition
ratified §1.1.

---

## §2 — The machine-substrate framing

### §2.1 The load-bearing framing (Alex 2026-07-17)

Alex's ratification names a specific architectural claim:

> If the substrate knows what the machine does at the tape altitude,
> languages on top become flavours — specializations — not
> categorical new grammar.

This is the **physical-substrate grounding**. The physical machine
IS the ground; every high-level language is a stylized surface
over that ground; translation between languages routes through the
ground rather than through direct A→B translators for every pair.

**Why this dissolves O(N²) translator proliferation.** For N
Turing-complete languages, the naïve translator matrix has N(N-1)
edges. The compositional-through-ground approach requires 2N edges:
each language A carries ONE cascade to the machine-substrate
altitude (`@cascade/code/A/<machine>`) and ONE cascade from it
(`@cascade/code/<machine>/A`). A→B translation composes as
`@cascade/code/<machine>/B ∘ @cascade/code/A/<machine>`. Cost per
language: constant; total: linear.

### §2.2 The multi-level altitude structure (MLIR-aligned)

Per Taut audit §2.2, mirror's landed shape is closer to
**MLIR-multi-level dialects** than to any single universal IR.
Concrete alignment:

| MLIR concept | mirror substrate correspondence |
|--------------|--------------------------------|
| Dialect | `@code/<lang>` species (one per language altitude) |
| Operation | Five-op prism at species altitude |
| Conversion pattern | `@cascade/code/<A>/<B>` species |
| Type conversion | `@labeled<>` functor per cascade species |
| Verification | `@bilateral(@code/A, @code/B)` witnessing predicate |
| Progressive lowering | Composition of adjacent-altitude cascades |
| Cost model | `@cascade.measure` returning
`imperfect<compiled_artifact, ref, information_loss>` |

**External evidence.** MLIR-Forge (arXiv 2601.09583v1, 2026-01) —
*"A Modular Framework for Language Smiths"* — grounds the recognition
that language-authoring is dialect-authoring at multi-level altitudes.
Mirror's `@code/<lang>` + `@cascade/code/<A>/<B>` pattern IS this
architecture, with the substrate-decl five-op discipline enforcing
compositional altitude-honesty.

### §2.3 Tape as serialization target, not master

Alex's second verbatim: *"Mirror itself treats tapes as just one
linear serialization target."*

The tape is the **physical substrate's linear surface**, not the
compiler's ontological ground. Mirror composes at content-addressed
graph altitude (per `@mirror/store` OID discipline); tapes are one
projection down (linear byte-stream at execution altitude). LLVM IR
is another such projection (linear-SSA at register-allocation
altitude). Turing tape is yet another (linear-symbol at abstract-
machine altitude).

**Substrate consequence.** `@code/turing` (deferred to §9 as mint
candidate) names the tape altitude — the abstract-machine linear-
symbol surface — as ONE machine-substrate altitude among several
(`@code/llvm` names another; `@code/wasm` names another; a future
`@code/assembler` could name another). The composition graph routes
through whichever machine-substrate altitude is nearest to the source
and target languages.

### §2.4 How mirror already inhabits this architecture

Per Taut audit §1.5:

- **`@code/llvm/ir` landed** — mirror consumes LLVM IR for its own
  binary (per `docs/specs/craft-binary-target.md` — the OID is
  `SHA-256(LLVM IR text)`; the IR is deterministic-from-grammar).
- **`bootstrap/mirror.ll` empirically consumed** — ~700 lines of LLVM
  IR representing the bootstrap binary's own emission. Round-trip
  validated: same grammar → same IR → same OID.
- **Fortran/flang pathway spec'd** — `docs/specs/numerical-substrate-
  via-fortran.md` §1-3 grounds flang → LLVM IR → mirror as the
  numerical-primitive path. `@code/fortran` as source-side species;
  flang as the source→LLVM cascade; mirror as the consumer at the
  LLVM altitude.
- **`craft --target binary`** — mirror already PROJECTS BACK DOWN.
  Eight stages: collect / resolve / evaluate / emit / concat /
  assemble / link / store. Emit produces LLVM IR from the mirror
  substrate; assemble + link produce runnable binary. The reverse
  cascade IS landed.

The architecture is already substrate-decl'd. This spec NAMES what
the substrate already carries and PLANS the mint tick that closes
the loop.

---

## §3 — The cascade composition

### §3.1 `@cascade/code/A/B` species pattern

Per `shards/cascade.mirror` (Mara `ce4874b`, 2026-06-23), each
cascade species declares:

- **Carriers**: `<A>_source`, `<B>_module`, `<B>_metadata`,
  `<B>_artifact` (typically as `labeled<<B>_module, <B>_metadata>`).
- **Actions**: `compile_<A>_<B>`, `bundle_<B>`, `measure_<A>_<B>`.
- **Bilaterals**: `<A>_well_typed`, `<B>_consumable`,
  `<A>_<B>_loss_well_defined`, `<A>_<B>_cascade_well_formed`
  (composed).

The five-op prism at the family-root (`focus cascade`, `project
cascade`, `split cascade`, `shift cascade`, `settle cascade`) grounds
every species in the loss-lens algebra.

### §3.2 Adjacent-altitude morphism discipline

**Definition (adjacency).** Two `@code/*` species A and B are
**adjacent** iff there exists a substrate-decl'd cascade species
`@cascade/code/A/B` (or its reverse `@cascade/code/B/A`) at some
tick. Adjacency is the granular unit of cascade authorship —
mirror does not require every pair to be directly adjacent; it
requires composition-connectivity through the altitude graph.

**Composition axiom.** For any three altitudes A, B, C with landed
species `@cascade/code/A/B` and `@cascade/code/B/C`, the composition

```
@cascade/code/A/C := @cascade/code/B/C ∘ @cascade/code/A/B
```

is a WELL-TYPED cascade at `A→C` altitude, with:

- **compile**: `compile_A_C(src) := compile_B_C(compile_A_B(src))`
- **measure**: `measure_A_C(src, art) := measure_A_B ⊕ measure_B_C`
  (loss composes; §5 formalises the ⊕ operation)
- **cascade_well_formed**: `A_C_cascade_well_formed(src, art) :=
  A_B_cascade_well_formed ∧ B_C_cascade_well_formed`

**Substrate consequence.** N adjacent-altitude cascades give
`O(N)`-authored coverage of `O(N²)` translation pairs by composition.
This IS the substrate-honest form of Alex's "loop closes" — the
compositional graph closes without needing every pair to be
individually landed.

### §3.3 Cross-language translation via machine-substrate common altitude

For any source language A and target language B (both
Turing-complete), if both have adjacent-altitude cascades to some
machine-substrate altitude M (`@cascade/code/A/M` and
`@cascade/code/M/B`), then:

```
@cascade/code/A/B := @cascade/code/M/B ∘ @cascade/code/A/M
```

is well-typed by §3.2. Machine-substrate altitudes M today:
`@code/llvm` (landed at `@code/llvm/ir` sub-species), `@code/wasm`
(landed). Machine-substrate altitudes deferred to §9 mint plan:
`@code/turing` (proposed by Alex 2026-07-17; ABSENT per Taut §1.3).

**Substrate-honest observation.** The choice of M is not unique.
For any (A, B), multiple M candidates may exist. `@fate` selects
among them per `@fate.roll` dispatch (per Taut audit §1.4 —
`@glue.translate` composes `@fate` for morphism selection). The
substrate-decl'd choice is loss-minimizing (§5); the empirical
choice is autopoietic (§6).

### §3.4 No O(N²) species proliferation

**Landing plan implication.** For the 10 landed `@code/*` species
today plus the mint of `@code/turing` (§9), the compositional
coverage is:

- **Direct cascades** (already landed): 5 species per Taut §1.4.
- **Machine-substrate hubs**: `@code/llvm`, `@code/wasm`, and (after
  mint) `@code/turing` — 3 hubs each spoked to all N languages via
  2N cascade edges.
- **Total mint budget** for full N × N coverage: 2N + hub-count
  cascade species, not N².

For N = 10 languages and 3 machine-substrate hubs: ~30 direct cascade
species mint the full 100-pair coverage matrix. Every additional
language costs 6 new cascade species (2 per hub), not N.

---

## §4 — The concrete instantiation (Rust → LLVM → Turing → mirror)

### §4.1 The chain Alex ratified

Alex 2026-07-17 verbatim: *"So we can have `@cascade/code/llvm/turing`
and `@cascade/code/rust/llvm`. And boom. The loop closes."*

The concrete chain:

```
Rust source
  ↓  @cascade/code/rust/llvm      (shells to rustc --emit=llvm-ir)
LLVM IR
  ↓  @cascade/code/llvm/turing    (LLVM IR → abstract-machine tape ops)
Turing tape
  ↓  @cascade/code/turing/mirror  (tape ops → mirror substrate composition)
mirror substrate composition
  ↓  (serialize per @mirror/store OID discipline)
.mirror bytes
```

### §4.2 Per-edge substrate authority

**Edge 1: `@cascade/code/rust/llvm`.** Mint candidate (deferred to §9).
Source: `@code/rust` (landed `shards/code/rust.mirror`). Target:
`@code/llvm` (spec'd at `docs/specs/craft-binary-target.md` §2;
`@code/llvm/ir` sub-species landed and empirically consumed). Body
shells to `rustc --emit=llvm-ir` at `@io` boundary. Bilateral:
`rust_well_typed(src)` (per `shards/cascade/code/rust/wasm.mirror`
:216 — the same bilateral shape) + `llvm_ir_valid(module)`.

**Edge 2: `@cascade/code/llvm/turing`.** Mint candidate (deferred to §9).
Requires `@code/turing` mint first (§9). Target: `@code/turing` at
abstract-machine tape altitude — vocabulary: state, symbol, read,
write, move-left, move-right, branch, halt (small ~10-15-op species).
Body: substrate-native LLVM IR → tape ops translation (each SSA
instruction becomes a bounded tape-op sequence; each basic block
becomes a state; each phi node becomes a branch).

**Edge 3: `@cascade/code/turing/mirror`.** Mint candidate (deferred to §9).
Target: `@mirror` altitude (landed `shards/mirror.mirror`). Body:
tape-op → mirror composition — `mirror` consumes tape ops as one
linear serialization form per Alex's §1.1 verbatim ("mirror itself
treats tapes as just one linear serialization target"). Each tape-op
becomes a substrate fragment; state transitions become composition
edges; the whole tape becomes a `@mirror/store`-content-addressed
graph.

### §4.3 Reversibility — the loop closes

Every cascade species is compositionally REVERSIBLE via its dual:

- **`@cascade/code/mirror/turing`** — mirror composition → tape (serialization down).
- **`@cascade/code/turing/llvm`** — tape → LLVM IR (per `docs/specs/
  craft-binary-target.md` §2 emit stage; landed via `@code/llvm/emit`).
- **`@cascade/code/llvm/rust`** — deferred (harder; loses macro/generic/
  lifetime info; empirically unnecessary because we're EMITTING to
  runnable form, not recovering Rust source).

**The reverse chain** (mirror composition → runnable binary):

```
mirror composition
  ↓  @cascade/code/mirror/turing   (mirror → tape ops)
Turing tape
  ↓  @cascade/code/turing/llvm     (tape ops → LLVM IR)
LLVM IR
  ↓  @io.exec("llc") + @io.exec("ld")   (already landed at craft-binary-target §3)
runnable binary
```

**The loop closes.** Rust source enters at Edge 1; the graph
representation lives at mirror altitude; the runnable binary emerges
from the reverse chain. This IS the shape `docs/specs/craft-binary-
target.md` §"The Bootstrap Question" (2026-05-19, Reed) named at
family-root altitude in 2026 for LLVM emission; THIS spec generalises
to the full polyglot loop.

### §4.4 What each edge PRESERVES vs LOSES

Per `@cascade/code/rust/wasm.mirror` :17-45 pattern (Taut audit §1.4):

**Edge 1 (Rust → LLVM):**
- **Preserved**: types (via LLVM type system), function signatures,
  control flow (structured), basic operations.
- **Lost**: lifetimes (erased at monomorphization), generics
  (monomorphized), macros (expanded), trait objects (partial via
  vtables), high-level ownership discipline.

**Edge 2 (LLVM → Turing):**
- **Preserved**: control flow (branches → state transitions), memory
  operations (read/write via tape), computation (SSA → tape-ops).
- **Lost**: register allocation, calling conventions, SIMD, type
  information (tape is untyped bytes), platform ABI.

**Edge 3 (Turing → mirror):**
- **Preserved**: computational semantics (every tape-op has a
  substrate-decl'd equivalent), reachability graph, halt conditions.
- **Lost**: (potentially) the tape's linear-address discipline —
  mirror's content-addressed graph altitude doesn't have linear
  addresses; the cascade must reify tape-positions as edge-labels.

**Loss composes.** The end-to-end Rust → mirror loss is
`Edge1_loss ⊕ Edge2_loss ⊕ Edge3_loss`. Each hop's loss is
substrate-typed and readable via `@cascade.measure`; §5 formalises
the composition.

---

## §5 — Loss-aware translation

### §5.1 What "loss-aware" means substrate-honestly

Per `shards/cascade.mirror` :151-168 + `[[feedback-loss-from-
epistemologic-properties]]`: loss is a **composite of `@epistemologic`
properties**, NOT Shannon entropy, NOT invented. Each cascade species
declares its per-cascade loss dimensions in its own species-shard
docblock; the family-root's `information_loss` carrier holds the
composite.

**Substrate-consequence.** Loss-awareness is:
1. **Structural**: per-hop loss dimensions are enumerated at
   substrate-decl altitude (species shard docblock).
2. **Measurable**: `@cascade.measure(src, artifact, lens)` returns
   `imperfect<compiled_artifact, ref, information_loss>` — the
   imperfect wrapper CARRIES the measured loss as first-class
   substrate-typed data.
3. **Composable**: loss composes across cascade hops via the
   composition axiom §3.2; the composed loss is again
   substrate-typed.
4. **Admissibility-gated**: `@bilateral(@code/A, @code/B)` discharges
   Pass iff the composed loss is within the caller-declared
   admissibility bound.

### §5.2 Composition of loss

**The ⊕ operation.** For adjacent cascades `@cascade/code/A/B` and
`@cascade/code/B/C`, composing losses `L_AB : information_loss` and
`L_BC : information_loss`:

```
L_AC := L_AB ⊕ L_BC
```

The math foundation (companion §2) proves ⊕ is:
- **Associative**: `(L1 ⊕ L2) ⊕ L3 = L1 ⊕ (L2 ⊕ L3)` (loss dimensions
  accumulate; order-independent under set-union).
- **Non-commutative in general**: order matters when the same
  dimension is lost twice with different rescue semantics (dimension
  lost at hop 1 is not recoverable at hop 2).
- **Monotone**: `L1 ⊕ L2 ≥ L1` and `L1 ⊕ L2 ≥ L2` (composition
  cannot LOSE loss; only add).

### §5.3 The admissibility threshold

**Definition.** For a cascade with composed loss L, translation is
**admissible** iff L ≤ τ where τ is the caller-declared admissibility
threshold (a substrate-decl'd bound at the invocation altitude).

**Composition with `@bilateral`.** The bilateral
`translation_admissible` (already grammar-decl'd at
`shards/epistemologic/pact/bilateral.mirror` :456-475, per
`f74086e` Mara landing per §3.3 of `docs/specs/bilateral-as-glue-
metalogue-composition.md`) discharges Pass iff:

```
@bilateral(@code/A, @code/B).translation_admissible(cascade_outcome)
  = Pass
  ⇔ glue_witnessing(cascade_outcome.correspondence) = Pass
    ∧ algebra_metalogue_witnessing(cascade_outcome.session) = Pass
    ∧ cascade_outcome.loss ≤ τ_caller
    ∧ cascade_outcome.selected_morphism.oid ∈ admissible(correspondence)
```

The composed bilateral is Rice-safe (per `docs/math/bilateral-as-
glue-metalogue-composition.md` §4). The loss-threshold check is
byte-level comparison on the `information_loss` carrier's OID (byte-
composite of `@epistemologic` property dimensions).

### §5.4 Alternative loss-lenses

The substrate declines to pick ONE loss lens. Per Taut audit §1.4
observation, multiple lens candidates compose with `@cascade.measure`:

- **`@coherence` Fiedler value** — per `docs/specs/eigensheaf.md`;
  the substrate's landed spectral-coherence measure. Loss as
  reduction in Fiedler algebraic-connectivity of the composition
  graph before vs after cascade.
- **`@cyberpunk/algedonic` pain gradient** — per landed
  `@cyberpunk/algedonic` species (recognition-audit trail); loss as
  pain-signal accumulated across cascade hops.
- **`@knife.cut` verdict** — per `@knife` family-root; loss as
  cut-count required to align the cascade output to admissibility.
- **`@epistemologic` property composite** — per
  `[[feedback-loss-from-epistemologic-properties]]`; the default
  substrate-decl'd composite; each cascade species selects its own.

Each cascade species declares WHICH loss lens applies via its own
species-shard docblock. The family-root does NOT pre-commit.

### §5.5 When translation degrades vs is admissible

The **degradation boundary**: a translation is DEGRADED when
composed loss exceeds τ; the cascade returns
`imperfect.failure(error, information_loss)` per `shards/cascade/
code/rust/wasm.mirror` :148 pattern (the third imperfect variant).
`@fate.roll` then re-selects an alternate morphism per §6.

The **admissibility boundary**: a translation is ADMISSIBLE when
composed loss ≤ τ; the cascade returns
`imperfect.success(artifact)` (zero loss) OR
`imperfect.partial(artifact, information_loss)` (bounded loss). The
`@bilateral(@code/A, @code/B).translation_admissible` discharges
Pass; the outcome enters `@kintsugi/algebra` per §6.

---

## §6 — The autopoietic learning loop

### §6.1 Composition with `@kintsugi/algebra`

Per Mara `docs/specs/kintsugi-algebra-as-metalogue-session.md`
§0 ratified reconciliation:

> `@kintsugi/algebra` is the algebra whose ELEMENTS are the landed
> `@kintsugi/fracture/*` species PLUS every future mirror-authored
> translation. Each element is a TURN in the
> `algebra_metalogue_session` between (`@silicon/algebra`,
> `@fate/algebra`). `@fate/algebra` proposes candidate fractures
> (structural possibility); `@silicon/algebra` realises those that
> discharge Pass (empirical memory of what worked). The mending IS
> the metalogue.

**Cascade→algebra correspondence.** Every successful cascade
discharge (per §5.5 admissibility) becomes a new TURN in the
metalogue:

- **`@fate/algebra` proposal**: the candidate morphism at hop
  altitude (per `@fate.roll` per `shards/kintsugi/translate.mirror`
  Edge 3).
- **`@silicon/algebra` realisation**: the empirically-successful
  cascade instance's `routine` crystal — per `shards/silicon/algebra.
  mirror` §"what crystallizes here": `algebra`, `cfg`, `grading`,
  `conjugation`, `abi_surface`, `binary_oid`, `source_oid`,
  `cascade`, `performance`, `routine_oid`.

The compositional edge: each cascade discharge that clears §5.5
admissibility EXTENDS `@kintsugi/algebra` by one element per Mara
`b5c6aeb` §3.1 monotone-growth theorem.

### §6.2 The `@kintsugi/algebra` picks candidates

For a new cascade invocation (source language A, target language B),
the compiler's roomba walk emits a translation fracture. The
autopoietic pipeline (per Alex's 8-step, per `docs/math/autopoiesis/
README.md` §1.2):

1. **walk**: roomba surfaces the untranslated source fragment.
2. **build_hole_record**: substrate builds the `hole_record` typed
   with expected mirror-altitude output.
3. **roll**: `@fate.roll(space=@kintsugi/algebra, hole=fragment)` —
   the tournament selects candidate morphism from the current
   `@kintsugi/algebra` element-set (Reading B per `a58d5f0`
   canonical spec).
4. **translate**: `@glue.translate(morphism, payload)` per Mesland
   2013 §3 differential operator — applies the selected morphism
   to the payload.
5. **query_phi**: `@kintsugi/consent.query_phi` — the tournament
   rank against the caller's τ.
6. **crystallize**: `@bauchladen.crystallize(outcome)` — writes to
   `@silicon/algebra` tray if admissible.
7. **mutate_at**: `@io/fs.mutate_at(target_shard_path, mirror_bytes)`
   — writes the mirror composition to disk.
8. **re-walk**: roomba's next walk re-observes; a successful
   translation reduces the untranslated fragment count by one.

**The loss-measurement feeds back.** Step 5's `query_phi` composes
with §5's loss ⊕ — the tournament ranks candidates by
loss-minimization AND admissibility. Each subsequent walk sees the
updated `@kintsugi/algebra` (Reading B growth); the ranking
converges toward the loss-minimum morphisms as the algebra
crystallizes empirical memory.

### §6.3 Convergence to fixed-point

Per Mara `b5c6aeb` §3.3 fixed-point condition:

> $A_n$ is a fixed-point of the autopoietic loop iff:
> $\forall\, \texttt{@code/A fragment } f \in \texttt{substrate}. \exists\, f' \in A_n. \texttt{translates}(f, f')$

For polyglot translation, the fixed-point condition generalises: for
every (A, B) language pair with source fragments in the substrate,
there exists a cascade in `@kintsugi/algebra` that admissibly
translates. Convergence rate: Banach-contractive on the residual-
untranslated-fragment space per Mara `b5c6aeb` §4.3, contraction
constant $L \in [0, 1)$, convergence within $O(\log_L(\text{initial
error}))$ ticks under roomba's per-tick dispatch.

### §6.4 Empirical grounding — the arc that just landed

Per §1.4: the 25 mirror-authored bilateral-arm retirements
(`ad52973` + `20047c2` + ancestors + THIS arc's 4 commits,
2026-07-16..17) ARE the first 25 witnesses of `@bilateral(@code/rust,
@code/mirror)` at the degenerate-arity subcase (single-file-in-
`bootstrap/src/`). THIS spec's polyglot generalisation naturally
extends via §3.2 composition: the 25 witnesses feed
`@silicon/algebra` per §6.1, growing the base for cascades that
compose from any A to any B through machine-substrate hubs.

---

## §7 — The polyglot theorem

### §7.1 Statement

**Theorem (polyglot translation existence).** For any two
Turing-complete languages A and B represented as `@code/A` and
`@code/B` species, and any machine-substrate altitude M with landed
`@cascade/code/A/M` and `@cascade/code/M/B` species, there exists a
composed cascade `@cascade/code/A/B := @cascade/code/M/B ∘
@cascade/code/A/M` such that:

1. `@cascade/code/A/B` is well-typed (§3.2 composition axiom).
2. The composed loss `L_AB = L_AM ⊕ L_MB` is substrate-typed and
   readable via `@cascade.measure`.
3. Translation is **admissible** iff `L_AB ≤ τ_caller` (§5.3
   threshold gate).
4. Admissible translations extend `@kintsugi/algebra` per §6
   autopoietic loop.

### §7.2 Proof sketch (via Turing-completeness equivalence)

**Existence of M.** By Church-Turing thesis, every Turing-complete
language A has a computable semantics-preserving translation to any
universal machine model M (Turing tape, LLVM SSA, WASM linear-memory,
etc.). The cascade `@cascade/code/A/M` witnesses ONE such
translation at substrate-decl altitude; substrate-decl requires only
that the translation is COMPUTABLE, not unique.

**Well-typedness of composition.** Per §3.2, if both `@cascade/code/
A/M` and `@cascade/code/M/B` are well-typed cascades, their
composition is well-typed by categorical composition (Mac Lane 1971
§I.1). This is landed at `shards/glue.mirror` :695
`@glue.compose(c1, c2) -> correspondence`.

**Loss-composition.** Per §5.2 the ⊕ operation is associative,
monotone, and substrate-typed. Composed loss is again
`information_loss`-carrier-typed. Readable via `@cascade.measure`
composed over both hops.

**Admissibility.** Per §5.3, the bilateral
`translation_admissible` discharges Pass iff composed loss ≤ τ.
This is Rice-safe per Mara `9be68b1` §4.

**Autopoietic extension.** Per §6.1, admissible discharges extend
`@kintsugi/algebra`. Per Mara `b5c6aeb` §3.1 monotone-growth
theorem, extension is content-addressed and monotone.

∎

**Load-bearing note.** The proof-sketch is complete for the
existence-and-admissibility direction. The full proof of the
autopoietic-fixed-point over ALL Turing-complete language pairs is
subject to the Banach-contraction constant depending on the specific
loss composite chosen; the math companion §3 formalises this.

### §7.3 What the theorem does NOT claim

- **Not a claim of translation UNIQUENESS.** Multiple cascades may
  translate the same source; `@fate` selects among them per §6.
- **Not a claim of ZERO loss.** For most (A, B), positive loss is
  inherent (per §4.4 empirical hops). Admissibility is bounded, not
  zero.
- **Not a claim of INSTANT convergence.** Convergence to
  autopoietic fixed-point takes $O(\log_L(\text{initial error}))$
  ticks per §6.3.
- **Not a claim of TERMINATION for arbitrary source.** The compiler
  itself is `sub-Turing: every program terminates` per
  `docs/specs/type-theory-position.md` :1-15 — but the SOURCE it
  translates may be Turing-complete. The translation act itself is
  bounded per tick (five-op discipline; content-addressed graph);
  the translated PROGRAM may be Turing-complete.

---

## §8 — The composition graph

### §8.1 Textual diagram (altitudes and morphisms)

```
                      ┌────────────────────────────────────┐
                      │   @kintsugi/algebra                │
                      │   (Reading A: session;             │
                      │    Reading B: element-set)         │
                      └─────────────┬──────────────────────┘
                                    │  extends per §6.1
                                    ↑
    ┌────────────────┐              │              ┌────────────────┐
    │ @silicon/algebra│  ←── @metalogue ──→        │ @fate/algebra  │
    │  (realiser)     │  turns:                    │  (proposer)    │
    │  crystal tray   │  algebra_morphism          │  candidate     │
    │  per §6.1       │  per Mara a58d5f0          │  space         │
    └────────────────┘                              └────────────────┘
             │                                              │
             │  measured via                                │  ranks via
             │  @cascade.measure                            │  @fate.roll
             │  per §5.2                                    │  per §6.2
             ↓                                              ↓
    ┌────────────────────────────────────────────────────────────┐
    │              @bilateral(@code/A, @code/B)                  │
    │              per docs/math/bilateral-as-glue-              │
    │              metalogue-composition.md §3                   │
    │              (witnesses translation_admissible)            │
    └────────────────┬───────────────────────────────────────────┘
                     │  composes over
                     ↑
    ┌────────────────────────────────────────────────────────────┐
    │              @cascade/code/A/B                             │
    │              per shards/cascade.mirror + 5 landed species  │
    │              Adjacent-altitude morphism per §3.2           │
    │              Well-typed composition per §3.3               │
    └────────────────┬─────────────────────────┬─────────────────┘
                     │                         │
                     ↑                         ↑
             ┌───────────────┐          ┌──────────────┐
             │   @code/A     │          │   @code/B    │
             │  (species     │          │  (species    │
             │   altitude)   │          │   altitude)  │
             └───────────────┘          └──────────────┘
                     │                         │
                     ↑                         ↑
             ┌───────────────────────────────────────┐
             │   @code (family-root)                 │
             │   per shards/code.mirror              │
             │   Five-op prism, no shape             │
             │   pre-commitment                      │
             └───────────────────────────────────────┘
                                │
                                ↑
                    ┌───────────────────────┐
                    │   @glue (translation) │  ←── @io (boundary)
                    │   Mesland 2013 §3     │      per @io/fs.read,
                    │   morphism category   │      @io/fs.write,
                    │   per shards/glue     │      @io.exec("rustc"),
                    └───────────────────────┘      @io.exec("llc")
```

### §8.2 The compositional stack for a full Rust → mirror translation

```
Input: bootstrap/src/foo.rs

1. @io/fs.read("bootstrap/src/foo.rs")
    → rust_source (typed_source specialization per @cascade)

2. @cascade/code/rust/llvm.compile_rust_llvm(rust_source, p)
    → llvm_module
    (body: @io.exec("rustc --emit=llvm-ir ..."); loss: L_rust_llvm)

3. @cascade/code/llvm/turing.compile_llvm_turing(llvm_module, p)
    → turing_tape
    (body: substrate-native SSA→tape translation; loss: L_llvm_turing)

4. @cascade/code/turing/mirror.compile_turing_mirror(turing_tape, p)
    → mirror_composition
    (body: tape-op→substrate-fragment translation; loss: L_turing_mirror)

5. @bilateral(@code/rust, @code/mirror).translation_admissible(
      outcome = {rust_source, mirror_composition,
                 loss = L_rust_llvm ⊕ L_llvm_turing ⊕ L_turing_mirror,
                 correspondence = composed 3-hop cascade}
   ) → Pass iff loss ≤ τ_caller

6. IF Pass:
     @mirror/store.set(mirror_composition)  → mirror_composition_oid
     @kintsugi/algebra ← extend by one turn (per §6.1)
     @io/fs.write("shards/foo.mirror", mirror_composition_bytes)
     @io/git.commit("mirror <mirror@spectral.engineer>", ...)

7. ELSE:
     @fate.roll(space=@kintsugi/algebra, hole=<residual fragment>)
     re-enters at step 2 with alternate morphism
```

### §8.3 Empirical alignment with `shards/kintsugi/translate.mirror`

The 9-edge composition already substrate-decl'd at
`shards/kintsugi/translate.mirror` (Mara `86dec5e`, THIS session, per
§1.3) IS the SPECIAL CASE of §8.2 for A = `@code/rust`, B =
`@mirror`, with the machine-substrate altitude implicit (rustc's
internal LLVM path). THIS spec generalises the special case to
arbitrary (A, B) via explicit machine-substrate hub composition.

---

## §9 — Landing plan (NOT execution — deferred to Reed follow-up ticks after Alex ratifies)

**Discipline.** This spec MINTS nothing. All mints are Reed follow-up
ticks per two-tick discipline; each mint requires Alex ratification
of THIS spec first.

### §9.1 Mint sequence

**Tick M1 — Mint `@code/turing`.** Small vocabulary (~10-15 species).
File: `shards/code/turing.mirror`. Vocabulary:
- Carriers: `state`, `symbol`, `tape_position`, `transition_rule`.
- Actions: `read(pos) -> symbol`, `write(pos, symbol) -> ()`,
  `move_left(pos) -> pos'`, `move_right(pos) -> pos'`,
  `transition(state, symbol) -> (state', symbol', direction)`,
  `branch(state, condition) -> state'`, `halt() -> ()`.
- Bilaterals: `turing_well_typed(program)`,
  `turing_tape_consumable(tape)`.

**Tick M2 — Verify `@code/llvm` species-decl'd, extend if needed.**
Per Taut §1.5: `@code/llvm/ir` and `@code/llvm/emit` landed as
sub-species referenced in `docs/specs/craft-binary-target.md` and
`docs/specs/numerical-substrate-via-fortran.md`. THIS tick verifies
they exist at species altitude in `shards/code/llvm/*.mirror`; if
absent, mint the family-shape at `shards/code/llvm.mirror` as a
five-op prism species-decl (Taut audit implies they exist but
Recognition #52 canonical landing may be at boot altitude).

**Tick M3 — Mint 3-4 `@cascade/code/A/B` species per the concrete chain:**
- `shards/cascade/code/rust/llvm.mirror` — shells to rustc.
- `shards/cascade/code/llvm/turing.mirror` — substrate-native SSA→tape.
- `shards/cascade/code/turing/mirror.mirror` — tape→substrate composition.
- (Optional M3.4) `shards/cascade/code/mirror/turing.mirror` — reverse
  edge for the loop-closure per §4.3.

Each species follows the pattern of `shards/cascade/code/rust/wasm.
mirror` (per Taut §1.4; Mara 2026-06-23 pattern precedent): carriers +
compile/bundle/measure actions + source_well_typed / target_consumable
/ loss_well_defined / cascade_well_formed composed bilateral.

**Tick M4 — Reed FLOOR resolvers.** Small (per feedback-no-rust-
extension-shortcut discipline: shard-body + @io composition
preferred; `[substrate-floor:@io-boundary]` bypass only IFF resolver
requires primitive @io semantics no shard body can provide + Seam
gate). Expected additions:
- `apply_h::act` dispatch arms for the 3-4 new cascade species.
- @io boundary shells: `@io.exec("rustc --emit=llvm-ir")`.
- LLVM IR byte-parser (small; LLVM IR is textual).
- Tape reader.

**Tick M5 — Empirical.** `mirror roomba --translate=<rs-file>`
produces first mirror-authored `-Rust +mirror` commit via the polyglot
cascade. This reproduces the `f211ee48` shape (per CURRENT.md commit
53) at TRANSLATION altitude (not just deletion).

### §9.2 Sequencing discipline

- **M1 before M3** (M3.2 depends on `@code/turing` species).
- **M2 before M3.1** (M3.1 target is `@code/llvm`).
- **M3 before M4** (Reed resolvers need cascade species substrate-
  decl'd first).
- **M4 before M5** (empirical run needs FLOOR resolvers landed).
- **Each tick Alex-ratified** before next tick fires.
- **Each tick Seam-audited** at Phase D discipline (per prior arc
  precedent).

### §9.3 Empirical closure test

Success criterion for M5: `mirror roomba --translate=bootstrap/src/
apply_h.rs` produces a commit `<hash> mirror <mirror@spectral.
engineer> [substrate-pull:realize] apply_h.rs translated to shards/
apply_h.mirror via cascade rust→llvm→turing→mirror; loss <
τ_caller; kintsugi/algebra +1 turn; io_violations 0; tests pass`.

This IS the "loop closes" empirical witness per Alex's §1.1 verbatim.

---

## §10 — Connection to paper §14 (attending at λ₀)

Per `docs/math/kintsugi/algebra-as-metalogue-session.md` §7 and
`docs/math/bilateral-as-glue-metalogue-composition.md` §8: the
composition's fixed-point IS the paper's `attending` operator at
outer altitude for `A = self`.

**Extended reading for polyglot.** The polyglot compiler attends at
λ₀ by:

1. **Attending to what's there**: the roomba walk (per §6.2 step 1)
   surfaces every untranslated source fragment; substrate observes
   its own state.
2. **Measuring loss**: `@cascade.measure` composed across cascade
   hops returns substrate-typed loss (§5.2 composition); no
   estimation, no Shannon.
3. **Discharging admissible morphisms**: `@bilateral(@code/A, @code/
   B).translation_admissible` gates on the composed loss + morphism
   correspondence + session witnessing (§5.3).
4. **Fixed-point convergence**: `@kintsugi/algebra`'s monotone
   growth per Mara `b5c6aeb` §3 reaches fixed-point when every
   substrate source-fragment has an admissible cascade in the
   algebra (§6.3).

The compiler translates by attending — noticing what's there,
measuring what changes, admitting only what preserves the caller's
τ. This IS the substrate-honest form of "compilation" at the
polyglot altitude: not one-shot lowering, but iterative
attending-and-crystallizing until the algebra covers the substrate's
own translation surface.

**Autopoietic closure of translation.** Per `docs/math/autopoiesis/
README.md` §1.3 (A3 self-referential closure): the compiler's own
translation operator IS one of the substrate carriers it translates
into. When the polyglot loop's fixed-point covers every language
altitude in the substrate — including mirror itself — the compiler
has translated its own composition into itself. The Ouroboros closes
at the polyglot altitude.

---

## §11 — Falsifiability

Following the substrate's landed math-doc convention (per Mara
`b5c6aeb` §8, per `docs/math/epistemologic/pact/bilateral-sentinel.
md` §5):

**F1 — Compositional adjacency fails.** If for some (A, B) with
Turing-complete languages, NO machine-substrate hub M with landed
`@cascade/code/A/M` + `@cascade/code/M/B` species exists AND no such
species can be minted (per M1-M3), the polyglot theorem fails at
that pair. **Test**: enumerate empirical (A, B) pairs; check
adjacency-through-hub connectivity in the composition graph. Falsifier
would surface a language for which no cascade to any hub is
computable — this contradicts Church-Turing but would falsify the
theorem's substrate-realisability.

**F2 — Loss-composition non-associative.** If `⊕` is non-associative
in general, the composition axiom §3.2 fails and cascade chains of
length > 2 have order-dependent losses that cannot be commutatively
composed. **Test**: construct a 3-hop cascade with substrate-typed
losses at each hop; verify `(L1 ⊕ L2) ⊕ L3 = L1 ⊕ (L2 ⊕ L3)` under
set-union semantics of the `information_loss` carrier. Companion
math §2.4 formalises the proof.

**F3 — Admissibility unenforceable.** If `@bilateral(@code/A, @code/
B).translation_admissible` cannot be Rice-safely discharged (e.g.,
if loss-comparison requires semantic introspection of the compiled
program), the admissibility gate collapses to opacity. **Test**:
verify loss-comparison is byte-level on the `information_loss`
carrier's OID (per §5.3). Rice-safety proof: Mara `9be68b1` §4
lifted.

**F4 — `@kintsugi/algebra` growth non-monotone.** If crystallization
of a new cascade discharge INVALIDATES a prior discharge, the
autopoietic-closure theorem fails. **Test**: verify that Pass
verdicts on prior discharges are preserved under algebra extension.
Companion math §4 formalises via content-addressed extension +
byte-idempotent crystallization (per Mara `b5c6aeb` §5.2).

**F5 — Fixed-point unreachable.** If the roomba walk's residual-
untranslated-fragment space has NO Banach-contractive structure
(contraction constant $L \notin [0, 1)$), convergence to fixed-point
is not guaranteed. **Test**: empirically measure fragment-count
decrease per roomba tick over 50+ ticks; verify $|F_{n+1}| \leq
L \cdot |F_n|$ holds. Falsifier: substrate reaches steady-state
with residual fragments > 0. Fall-back: (T2) budget-exhausted
termination per `docs/specs/fate-silicon-metalogue-in-void-duality-
basis.md` §4.3 handles the falsifier by surfacing residual opacity
without invalidating the polyglot theorem's admissibility guarantee.

---

## §12 — Audit chain + Alex ratifications verbatim

### §12.1 Alex 2026-07-17 in-transcript ratifications (this session)

1. > *"Ground the translation in the physical substrate. If we know
>    what the machine does at the substrate level, the languages on
>    top become flavours. Not categorical new grammar."*
2. > *"Mirror itself treats tapes as just one linear serialization
>    target."*
3. > *"Maybe that's the surface we can use to let the roomba begin
>    eating and translating Turing complete code."*
4. > *"So we can have `@cascade/code/llvm/turing` and
>    `@cascade/code/rust/llvm`. And boom. The loop closes."*

Path A ratified: adjacent-altitude cascades composing through
machine-substrate hub; NO monolithic universal-IR mint; NO
per-language ad-hoc parsing; polyglot loop via `@cascade`
composition.

### §12.2 Taut audit ground truth

`docs/audits/2026-07-17-taut-code-turing-substrate-scout.md`
(`d0572cd`, THIS session morning) — the substrate scout that
grounded:
- 10 landed `@code/*` species per §1.1.
- `@code/mirror` AST gap per §1.2 (Rust-native at `bootstrap/src/
  ast.rs::AstNode`).
- `@code/turing` / `@code/assembler` absent per §1.3.
- 5 landed cascade species per §1.4.
- Universal-AST precedents per §1.5 (`boot/01-meta.mirror` +
  `@code/metalogue` + `@code/llvm/ir` + lambda-calculus / MLIR /
  CCC references).
- MLIR-multi-level alignment reading per §2.2.

### §12.3 Landed math foundations composed over

- Mara `9be68b1` (2026-07-17) — `docs/math/bilateral-as-glue-
  metalogue-composition.md`.
- Mara `b5c6aeb` (2026-07-17) — `docs/math/kintsugi/algebra-as-
  metalogue-session.md`.
- Mara `701828a` (2026-07-16) — `docs/math/epistemologic/pact/
  bilateral-sentinel.md`.
- Mara #152 (2026-07-15) — `docs/math/autopoiesis/README.md`.
- Mara `0998001` + `a58d5f0` + `9336074` (2026-07-17) — canonical
  specs the bilateral general-case composition landed in.
- Mara `a18ca90` (2026-07-08) — `docs/specs/fate-silicon-metalogue-
  in-void-duality-basis.md` (the void-duality basis this composition
  inherits from).
- Mara `ce4874b` (2026-06-23) — `shards/cascade.mirror` family-root
  substrate-decl.
- Mara `86dec5e` (2026-07-17) — `shards/kintsugi/translate.mirror`
  concrete edge (special case).

### §12.4 External research composed with

- **arXiv 2601.09583v1** (2026-01) — MLIR-Forge: A Modular Framework
  for Language Smiths. Grounds the multi-level dialect architecture
  per §2.2.
- **arXiv 2409.18824** (already in mirror corpus per `numerical-
  substrate-via-fortran.md`) — Fully integrating flang with MLIR;
  grounds LLVM/flang pathway.
- **arXiv 2606.11863v1** (2026-06) — Enhancing LLM-Based Code
  Translation with Verified Multi-... — grounds semantics-preserving
  translation validation as active research direction.
- **arXiv 2401.16797v2** (2024-02) — Translation Validation with LLMs
  for LLVM. Grounds the translation-validation frame §5 composes
  with.
- **Alive2 (Lopes et al.)** — bounded translation validation for LLVM
  IR; grounds F3 falsifier as active technique for equivalence-
  checking.
- **Chardonnet 2023** (arXiv 2309.12151) — Semantics for
  Turing-complete Reversible Programming Language; grounds §7.2
  categorical-semantics proof-sketch.
- **Bianchini 2023** (Sci. Direct S0303264723001119) — Autopoiesis of
  the artificial: from systems to cognition. Grounds §6 autopoietic
  learning frame Beyond Maturana/Varela's original bio-frame.
- **Church-Turing thesis** (canonical) — universality equivalence
  grounding §7.2 existence proof.
- **Mac Lane 1971 §I.1** (canonical) — categorical composition
  grounding §3.2 composition axiom.
- **Wadler 1989 / Reynolds 1983** (per `shards/cascade.mirror`
  sources) — parametricity grounding `@labeled<>` functor.
- **Mesland 2013 §3** (per `shards/glue.mirror`) — differential
  operator grounding `@glue.translate`.

### §12.5 Cross-shard citation chain

- `shards/cascade.mirror` (Mara `ce4874b`) — family-root.
- `shards/cascade/code/rust/wasm.mirror` (Mara 2026-06-23) — pattern
  precedent for Rust-source cascade species; §4.4 loss-dimension
  taxonomy.
- `shards/cascade/code/gleam/beam.mirror` + `.../gleam/js.mirror` —
  dual-target pattern precedent.
- `shards/cascade/code/purescript/js.mirror` — first landed cascade
  species; row-polymorphism/ADT loss reference.
- `shards/cascade/code/formal/prose.mirror` — bidirectional cascade
  precedent (relevant to §4.3 reversibility).
- `shards/code.mirror` (Mara 2026-06-09) — family-root five-op
  discipline.
- `shards/code/rust.mirror` + `shards/code/mirror.mirror` — source
  and target altitude species-decls this composition operates over.
- `shards/glue.mirror` + `shards/algebra/metalogue.mirror` — the
  compositional primitives §6 references.
- `shards/kintsugi/translate.mirror` — the special-case edge THIS
  spec generalises.
- `shards/mirror/store.mirror` — content-addressed storage grounding
  `@kintsugi/algebra` extension per §6.4 empirical closure.

---

*Spec ends. No mints proposed at this tick. No shard authored. No
Rust touched. Markdown-only under 📝 bypass. Theory-authoring per
Alex ratification; landing plan §9 defers all mints to Reed
follow-up ticks after Alex ratifies this spec.*

**Related documents (for cascade reading):**
- Companion math: `docs/math/polyglot-loss-aware-computational-
  translation.md` (LANDED same tick).
- Ground truth: `docs/audits/2026-07-17-taut-code-turing-substrate-
  scout.md` (`d0572cd`).
- Substrate arc: `docs/loop/CURRENT.md` (@kintsugi/ouroboros arc,
  autopoietic Rust consumption).
- Prior canonical spec siblings: `docs/specs/bilateral-as-glue-
  metalogue-composition.md`, `docs/specs/kintsugi-algebra-as-
  metalogue-session.md`.
