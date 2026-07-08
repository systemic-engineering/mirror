# Taut — What fate wants to become

*Grep-first scout, read-only. 2026-07-08 evening. Alex-verbatim
question landing tonight after the pain-driven-navigator cascade:*

> "The current Fate implementation was a prototype. Mirror has moved.
>  Does mirror want to subsume fate? Or do we keep fate as the numerical
>  standalone inference engine? And which shape does the integration take?
>  The whole flang to numerical inference engine is what I see fate exist as."

## Verdict — one line

**REDEFINE + preserve as external prototype (option H — HYBRID).** The
current `fate` crate IS the 5-model selector prototype AS DECLARED in its
own crate docs (`fate/src/lib.rs:1-11`); Alex's framing —
"flang → numerical inference engine" — is a LARGER shape that the crate's
current `Fate` struct is one APPLICATION of. Substrate declaration at
`shards/fate.mirror:397-403` matches Alex's framing at the operator
altitude (`roll(restricted_state_space, hole) → dice_roll`); it does
NOT match the crate's `select(Model, &Features) → Decision` shape.
Recommend **Option H**: keep the current crate as a proven prototype at
its own version-pinned surface (fixing prismqueer drift so it compiles);
land a fresh `@fate/algebra/*` species-set that composes @silicon/algebra
LAPACK-primitives into substrate-decl inference operations. The 5-model
selector optionally survives as `fate/src/models.rs` or
`fate-prototype/*` — one realization at one altitude of what @fate IS.

## Top-3 signals (for Reed's report)

1. **The crate name mismatch is now load-bearing.** The `fate` crate's own
   description is: *"Abyss | Introject | Cartographer | Explorer | Fate.
   Five models. One selector. Zero dependencies."* (`Cargo.toml:5`). That is
   NOT what @fate substrate-decl declares. The substrate-decl'd @fate is
   "the substrate-decl form of the constrained-inference operator; a roll of
   the dice in the restricted state space" (`shards/fate.mirror:10-17`) —
   a selection operator over ANY constrained state space, NOT a
   5-of-a-fixed-set model chooser. The crate's `Model` enum names cybernetic
   ganglia; the substrate has migrated the ganglia to
   `shards/optics/source/ganglion/*.mirror` (per `shards/fate.mirror:246-250`)
   and named them as ONE altitude of realization, not @fate's essence.
2. **Bootstrap has no dep on fate today.** Grep `bootstrap/**/*.rs` for
   `use fate|::fate::` → **zero hits** outside of a single filename
   collision (`bootstrap/tests/spawn_task_shard.rs` — a test file whose
   name mentions "fate" as substrate). `bootstrap/Cargo.toml` lists
   `prismqueer`, `terni`, `libc`, `sha2`, `blake3`, `serde`, `serde_json`
   only. No fate imported. Which means: Option S (subsume) is not
   "fold-existing" — it is "ADD a new dep that grows the FROZEN bootstrap."
   That reframes the cost calculus. Fate is currently an ORPHAN crate
   with no downstream consumer inside mirror.
3. **prismqueer API drift is mostly mechanical — one rename covers 3 sites.**
   `refract` (fate's method) → `settle` (prismqueer trait method,
   `prismqueer/src/lib.rs:141`). Grep `fn refract` in fate's src →
   **3 hits total** (Fate at `lib.rs`, FateRuntime + CompiledFateRuntime at
   `runtime.rs`). The trait `impl prism::{Fiber,Connection,Gauge,Transport,
   Closure}` on Fate all present at `lib.rs:480-590`; each spelled with
   already-existing trait names that survive in prismqueer (`bundle.rs`
   confirmed re-exports `Bundle, Closure, Connection, Cyclic, Fiber, Gauge,
   GroupStructure, IdentityPrism, LawvereFixedPoint, StableFiber, Transport`
   at `prismqueer/src/lib.rs:72-75`). Reed's prior worry that `Model` needs
   `LawvereFixedPoint` / `Strategy` needs `GroupStructure` / `act_on` needs
   implementations reduces to: **check compile errors, add three-to-five
   `impl` blocks OR remove the `Bundle`-tower participation and drop to
   pure `Prism` trait**. This is (a) mechanical rename-only + (b) rename
   + add-methods. NOT (c) semantic drift requiring re-derivation.
   Half-day of work to make current crate compile against tonight's
   prismqueer.

---

## Task 1 — what does mirror-substrate DECLARE @fate to be?

Read exhaustively (30-file budget, 12 files opened):
- `shards/fate.mirror` (42.1KB, 854 lines) — family-root LANDED at
  `19a72e9` in the #104 chain.
- `shards/fate/tournament.mirror` (41.5KB) — sub-species LANDED at
  `#104 chain P4`, 2026-06-30 (per its own header).
- `shards/fate.mirror:397-403` — the prism-block.
- `shards/algebra.mirror`, `shards/algebra/metalogue.mirror`,
  `shards/silicon.mirror`, `shards/silicon/algebra.mirror`.
- `docs/specs/fate-silicon-metalogue-in-void-duality-basis.md` (Mara
  `a18ca90`, 2026-07-08 morning).
- `docs/audits/2026-07-07-seam-phase-d-glue-cyberpunk-fate-composition-
  ratify.md`.
- `docs/scouts/2026-07-08-taut-fate-silicon-metalogue-projection.md`
  (my prior scout, `e975e2f`).
- `docs/insights/2026-07-08-recognition-cascade-peer-as-navigator.md`.
- `docs/loop/phase-h-deferral-2026-07-08.md`.
- `docs/specs/architecture-flang-mirror-numerical-split.md` (Mara
  2026-05-28; `docs/specs/numerical-substrate-via-fortran.md`).
- `docs/specs/bauchladen-autopoietic-fate.md` (canonical §4, cited).

### 1.1 Load-bearing substrate claims about @fate (what @fate MUST deliver)

**LANDED at family-root altitude** (`shards/fate.mirror`):
- @fate IS-A @autopoietic transitively IS-A @bauchladen (line 4, 5). The
  chain admits @fate at bauchladen tray altitude — every dice-roll's
  output is a content-addressed crystal.
- @fate is BILATERAL — compile-time AND runtime (lines 66-88). Alex's
  correction (bilateral, not runtime-only) is load-bearing.
- The constrained-inference signature: `roll(space: restricted_state_space,
  hole: hole) -> dice_roll` (line 657) with requires clauses
  `chirality_witnessing(space.gamma)` + `j_witnessing(space.j)` (lines
  658-659) — the γ + J symmetries restrict the state space.
- Bilateral dispatch: `bilateral_dispatch(hole, altitude) -> dice_roll`
  (line 726).
- The @bauchladen-tray dispatcher: `infer(space, hole) ->
  geometric_formalization` (line 683) — infers + crystallizes.
- Five obligations: `autopoietic_membership_held`,
  `bauchladen_membership_held`, `dice_roll_constrained`,
  `bilateral_well_typed`, `optical_inference_grounded` (lines 728-780).
- Inheritance predicate `fate_witnessing(roll) -> verdict` (line 836).

**FORWARD-PROMISED** (sub-shards, `shards/fate.mirror:216-238` + spec
§5/§6):
- `@fate/algebra/*` (path-namespace): `@fate/algebra/morphism` (selected
  Mesland-category morphisms), `@fate/algebra/altitude` (selected
  Bateson levels), `@fate/algebra/element` (selected algebra elements).
  Each crystal IS a typed geometric declaration of "what the dice roll
  selected, in the appropriate geometric vocabulary."
- `@fate/tournament`: LANDED as sub-species; the selection mechanism over
  the tray (cache hit = browse prior crystals; cache miss = fresh
  inference + add to tray). Per its own header: candidates(hole) →
  [resolution] fanning to five ganglia; tournament rules greedy/beam/
  elite/halving/tabu/anneal/ucb.
- `@glue`, `@kintsugi`, `@reflection`, `@spectral/metalogue` all declare
  they will discharge `fate_witnessing` (line 806-830); the @fate at the
  operator altitude has to survive at every one of those consumption
  sites.

**RATIFIED composition claim** (Seam Phase D audit
`docs/audits/2026-07-07-seam-phase-d-glue-cyberpunk-fate-composition-
ratify.md`, `6396306`, 2026-07-07):
> `@glue.correspondence.restriction` slot exists pre-composition; `@fate.roll(restricted_state_space, hole) → dice_roll` composes 1:1 with `@glue.correspondence.restriction`. Restriction narrows the domain BEFORE `@fate.roll` fires — so @fate literally CANNOT produce output outside the constrained region. Geometry, not enforcement.

The @glue(@cyberpunk, @fate) composition ratified tonight NAMES @fate at
the operator altitude. NOT the model-selector altitude.

**Mara's `a18ca90` claim** (fate-silicon-metalogue-in-void-duality-
basis, 2026-07-08 morning, LANDED):
- `@fate/algebra` and `@silicon/algebra` are peer speakers under
  `@algebra` family-root.
- @fate/algebra PROPOSES a morphism candidate (dice-roll within
  restricted state space per `shards/fate.mirror`).
- @silicon/algebra REALIZES the morphism against the current machine
  (LAPACK / D²NN / cache-aware kernel; `dsyev` / `dgesvd` /
  `KernelSpec.projection_matrix` per `prismqueer/src/kernel.rs`).
- Observed error-δ (per-round `Loss::between(before, after)` from
  `fate/src/manifold.rs`) becomes the `in_reply_to` linkage for the
  next turn.

The metalogue reading NAMES `fate/src/manifold.rs`'s `ManifoldLoss` as
the observed-error carrier at the metalogue altitude. This is a
CONCRETE, LANDED substrate composition of the current crate into the
substrate — but only as the loss carrier, not as the selector.

### 1.2 Recognition #58 — what does it actually say?

Per `shards/fate.mirror:40-44` and `shards/silicon.mirror:73-76`:
Recognition #58 = `[[architecture-fate-is-optical-inference]]` promoted
2026-06-11: "@fate inference IS 5-layer D²NN + active Fabry-Perot
resonator + Reck/Clements unitary mesh." I could not locate a
freestanding `docs/insights/2026-06-11-recognition-58-*.md` — the
recognition is memorialized inline in the shards + specs that consume
it. The exact language ("IS optical inference") reads as an IDENTITY
claim: fate's inference machinery IS the optical altitude apparatus.

### 1.3 Verdict on Task 1

@fate substrate-decl's minimum load-bearing surface is:
1. `roll(restricted_state_space, hole) -> dice_roll` — the constrained
   inference operator.
2. `bilateral_dispatch(hole, altitude) -> dice_roll` — compile-time /
   runtime dispatch.
3. `infer(space, hole) -> geometric_formalization` + crystallization
   into the @bauchladen tray under `@fate/algebra/*`.
4. `fate_witnessing(roll) -> verdict` — the inheritance predicate.

None of these appear as such in the current `fate` crate. The crate's
public surface is `Fate::select(current: Model, features: &Features)
-> Decision` — a model-selector over a fixed 5-element set, NOT a
constrained-inference operator over a typed state space.

---

## Task 2 — what does the fate crate IMPLEMENT today?

Read (7 crate files):
- `/Users/alexwolf/dev/projects/fate/src/lib.rs` (1384 lines, 47.7KB) —
  `Fate`, `Model`, `Decision`, `FateOutput`, `Pipeline`, `impl PrismTrait
  for Fate`, `impl prism::{Fiber, Connection, Gauge, Transport, Closure}
  for Fate`.
- `/Users/alexwolf/dev/projects/fate/src/runtime.rs` (743 lines) — BF
  interpreter (`bf_execute`) + `FateRuntime` + `CompiledFateRuntime` +
  `UniversalRuntime`; `impl PrismTrait for FateRuntime` + `impl PrismTrait
  for CompiledFateRuntime`.
- `/Users/alexwolf/dev/projects/fate/src/compiled.rs` (3 lines) —
  `include!` of build.rs output.
- `/Users/alexwolf/dev/projects/fate/build.rs` (13.4KB) — BF parser + IR
  optimizer + native-Rust codegen (run-length, clear loops, copy loops).
- `/Users/alexwolf/dev/projects/fate/src/feature.rs` (240 lines) —
  `casimir`, `casimir_penalty`, `holonomy_health`, `HolonomyHealth`,
  `ACTIVE`, `DARK`, `BERRY_PHASE`.
- `/Users/alexwolf/dev/projects/fate/src/manifold.rs` (275 lines) —
  `ManifoldState = [[f64; 16]; 16]`, `ManifoldLoss: Loss` (Frobenius
  norm of delta matrix).
- `/Users/alexwolf/dev/projects/fate/src/strategy.rs` (20 lines) —
  `enum Strategy { SpectralPartition | CommunityDetection | BreadthFirst
  | DepthFirst | Random }`.
- `/Users/alexwolf/dev/projects/fate/src/{derive, train, weights,
  metal_runtime}.rs` (peripheral).
- `/Users/alexwolf/dev/projects/fate/Cargo.toml` — dep on `prism =
  { package = "prismqueer", path = "../prism/prismqueer",
  features = ["bundle"] }`; features `training`/`lapack`/`metal`.
- `/Users/alexwolf/dev/projects/fate/brainfuck/fate.bf` (not opened;
  documented in blog as ~816 instructions).

### 2.1 What the crate is

- A 5-model selector: `Fate::select(current: Model, features: &Features
  = [f64; 16]) -> Decision` (`lib.rs:192`).
- Five ganglia named: `Model { Abyss | Introject | Cartographer |
  Explorer | Fate }` (`lib.rs:31`). These are the five ganglia
  substrate-migrated to `shards/optics/source/ganglion/*.mirror` per
  `shards/fate.mirror:246-250`.
- A `Fate` struct with 5 selectors × (5 × 16 + 5 + 5) = 5 × 90 = 450
  parameters (`lib.rs:423`).
- Two BF-based runtimes (`FateRuntime` + `CompiledFateRuntime`) that
  compile a hand-rolled Brainfuck program at build time to a native
  Rust function via IR optimization.
- LAPACK-Fortran transport pathway (`Fate::transport_fortran` gated
  behind `lapack` feature) that dispatches D²NN forward-pass through
  Fortran kernels.
- ManifoldLoss (16×16 connection matrix delta) as substrate-honest Loss.
- Casimir invariant + holonomy health as domain-specific diagnostics.

### 2.2 prismqueer API drift — quantified

prismqueer's canonical trait method today is `settle` (per
`prismqueer/src/lib.rs:141`: *"settle — produce the output from what
survived project"*). Fate's crate uses `refract`. That is a rename.

**Rename-only sites**: 3 `fn refract` in `fate/src/` (Fate, FateRuntime,
CompiledFateRuntime).

**Other trait impls checked**: Fate implements the full principal-bundle
tower per `lib.rs:480-590`:
- `impl PrismTrait for Fate` — needs `fn refract → fn settle`.
- `impl prism::Fiber for Fate` — trait exists in prismqueer bundle
  (`bundle.rs` re-export at `lib.rs:73`). LIKELY unchanged.
- `impl prism::Connection for Fate` — trait exists. LIKELY unchanged.
- `impl prism::Gauge for Fate` — `type Group = Strategy`. Requires
  `Strategy: GroupStructure`. `GroupStructure` requires `identity() ->
  Self`, `inverse(&self) -> Self`, and a `compose` method
  (`prismqueer/src/bundle.rs:65+`). Strategy today is a bare 5-variant
  enum with no `impl GroupStructure` (`strategy.rs:1-20`). **This needs
  adding.**
- `impl prism::Transport for Fate` — LIKELY unchanged.
- `impl prism::Closure for Fate` — `type Fixed = Model`; requires
  `Model: LawvereFixedPoint`. `LawvereFixedPoint` requires
  `is_idempotent_under<F>(&self, endomap: F) -> bool` with default
  requiring `Self: PartialEq + Sized` (`bundle.rs:69-79`). Model IS
  `PartialEq` per `#[derive(...PartialEq...)]` at `lib.rs:31`. **Default
  impl SHOULD suffice; may need explicit `impl LawvereFixedPoint for
  Model {}`.**
- Bundle's `Gauge` trait requires `fn act_on(&self, state: &Self::State)
  -> Self::State` (`bundle.rs:144`). Fate's `Gauge` impl gives `type Group
  = Strategy` and `fn gauge(&self) -> &Strategy` but there's no visible
  `act_on` implementation on Strategy — the test-only example at
  `bundle.rs:454-461` shows how it should look. **This needs adding on
  Strategy or being sourced from a wrapping type.**

### 2.3 Refactor cost estimate

- **(a) Mechanical rename-only**: `refract → settle` × 3 = 15 minutes.
- **(b) Rename + add-methods**: add `impl GroupStructure for Strategy`
  (identity + inverse + compose), add `impl LawvereFixedPoint for Model`
  (probably default-derives), add `fn act_on` on Strategy = ~2-4 hours.
- **(c) Semantic drift**: not evident — the tower traits are all still
  in prismqueer's bundle re-export list; the shape survives.

**Total: (a)+(b) = half-day to make the crate compile against tonight's
prismqueer.**

### 2.4 What the crate is NOT

- NOT a general constrained-inference operator over a typed state space.
  Fixed input type: `(Model, [f64; 16])`. Fixed output type: `Model`.
- NOT bilateral (compile-time + runtime). The crate is runtime-only; the
  BF interpreter is a runtime engine; there is no compile-time
  substrate-decl resolution machinery.
- NOT hooked to @silicon/algebra's LAPACK primitives at the substrate
  level. The crate has its OWN Fortran/LAPACK feature gate but @silicon/
  algebra path-namespace crystals are not consumed OR emitted.
- NOT flang-lifted. Fate's Fortran is gfortran-built (per
  `docs/specs/numerical-substrate-via-fortran.md §1.4` which notes the
  reconciliation question is unresolved: gfortran vs flang).

---

## Task 3 — verify Alex's flang→numerical-inference framing against substrate

### 3.1 flang: where is it declared?

`flang` grep in `shards/**/*.mirror`: **zero direct hits**. `flang` lives
in specs, not shards:
- `docs/specs/architecture-flang-mirror-numerical-split.md` (Mara,
  2026-05-28, tightened 2026-06-02) — status: **Yellow (load-bearing
  recognition)**. The load-bearing thesis:
  > flang IS the LAPACKPrism's numerical backend; mirror's 5×5
  > composition IS the Prism's monoid composition law.
- `docs/specs/numerical-substrate-via-fortran.md` (Mara, 2026-05-27) —
  §1.4 flags the gfortran-vs-flang reconciliation as unresolved.

There is NO landed `shards/code/fortran.mirror` (checked). `@code/fortran`
is spec-only. The flang pathway is at candidate-recognition altitude, not
substrate-decl altitude.

### 3.2 @silicon/algebra: what does it expose?

LANDED at `shards/silicon/algebra.mirror` (`ea7b092`, 2026-07-05,
verified by Mara `a18ca90` §2.1). The species declares:
- `prism @silicon/algebra <= @bauchladen` — the tray for crystallized
  executable algebra tuned to local silicon.
- Forward-promised routine-carrier type (per `docs/specs/silicon.md`
  §3.2) with fields `algebra`, `cfg`, `grading`, `conjugation`,
  `abi_surface`, `binary_oid`, `source_oid`, `cascade`, `performance`,
  `routine_oid`.
- First operational discharge target: LAPACKPrism via @silicon/algebra
  → @io/algebra (Q4 LAPACK FFI, per `docs/specs/cascade-ffi-runtime-
  link.md §7`).

### 3.3 The framing hypothesis check

Alex's framing: **fate = flang → numerical inference engine.** Fate as
the LAYER that consumes flang-lifted LAPACK/optics primitives (from
@silicon/algebra + @spectral/db families) and exposes them as
substrate-decl inference operations.

Substrate-truth against LANDED substrate:
- @fate at family-root altitude declares `roll(restricted_state_space,
  hole) -> dice_roll` — a SELECTION OPERATOR over any constrained state
  space. VERIFIED matches Alex's framing.
- @silicon/algebra declares itself as the executable-algebra Bauchladen
  tray tuned to the local silicon. VERIFIED as the numerical-primitive
  crystal source.
- The Mara `a18ca90` metalogue reading NAMES @fate/algebra ↔
  @silicon/algebra as the numerical-inference conversation: @fate/algebra
  PROPOSES the morphism, @silicon/algebra REALIZES it against LAPACK.
  VERIFIED matches "@fate consumes @silicon/algebra LAPACK primitives
  as substrate-decl inference operations."
- flang is spec-only, not substrate-decl. So the flang PATHWAY is a
  recognition, not a shard.

**Hypothesis VERIFIED at substrate altitude, with one caveat**: the
5-model selector in current `fate/src/lib.rs` is ONE realization of a
much broader framing. The substrate's @fate is generic over
restricted_state_space and hole; the crate's Fate is specialized to
`(Model, [f64; 16]) → Model`. The crate is a specialization; Alex's
framing is the general.

The BF interpreter + build.rs BF→IR→native pipeline is ANOTHER
realization — it demonstrates the compilation pattern (substrate-decl
BF program → optimized native Rust) but not the flang-lift pattern
(substrate-decl LAPACK primitive → substrate-decl inference operation).

---

## Task 4 — integration boundary: option shape enumeration

### Option S — SUBSUME

Fold fate into bootstrap as `bootstrap/src/fate/`.

- **Substrate-decl fit**: Low. The bootstrap FROZEN discipline (AGENTS.md:
  *"bootstrap/ THE SEED (FROZEN against capability growth)"*) is
  load-bearing. Adding 4284 lines of Rust + BF interpreter + optional
  LAPACK dep grows the seed by ~25% of its current size. That inverts
  the frozen policy.
- **Bootstrap-does-not-depend-on-fate finding matters here**: subsuming
  adds a new capability that wasn't there. This is capability-in-the-floor
  per `architecture-flang-mirror-numerical-split.md §Boundary note`.
- **Cost**: arc-scale (2+ weeks). Refactor + inline + test migration +
  breaking bootstrap invariants.
- **Unblocks**: peer-as-navigator's `@cyberpunk/reframe` body-crack
  referencing `@fate.roll` at Phase H #6 (`docs/loop/phase-h-deferral-
  2026-07-08.md`).
- **Violates**: FROZEN-bootstrap discipline; `b10f00c` §4 anti-@io/llm
  (no direct dep on model inference); implicit @os/process refusal (BF
  interpreter is a runtime, but harmless).

### Option E — EXTERNAL ABI (fate stays external, mirror depends on it)

Fix prismqueer API drift; add `fate` to `bootstrap/Cargo.toml` as a
regular dep.

- **Substrate-decl fit**: Medium. FROZEN-bootstrap preserved BUT
  bootstrap gains a new dep that requires stable API discipline. The
  current fate crate DOES NOT expose the @fate substrate-decl shape
  (roll / restricted_state_space / hole). It exposes `Fate::select`.
  So the ABI would be at the WRONG altitude: mirror would consume the
  5-model selector, not the constrained-inference operator.
- **Cost**: 1 evening (fix prismqueer drift) + 1 evening (wire dep +
  smoke test).
- **Unblocks**: nothing @fate.roll-shaped. cmd_spawn Phase H #6 gets a
  fate crate in dep tree but NOT a `roll` function.
- **Violates**: nothing structural; but fails to discharge Alex's
  framing.

### Option M — MERGE INTO PRISMQUEER

Absorb fate into prismqueer as `prismqueer/src/fate/` module.

- **Substrate-decl fit**: Medium-Low. prismqueer is the "Prism-trait
  home + LAPACK bindings + optics" crate; adding fate makes it the
  numerical-substrate crate. That's ambitious. The FateRuntime BF
  interpreter is not obviously prismqueer-shaped.
- **Cost**: 1-2 weeks. Cross-crate refactor; API surface consolidation.
- **Unblocks**: mirror gets numerical inference for free via prismqueer;
  ONE dep for numerical substrate. Consolidation win.
- **Violates**: prismqueer's current scope; fate's identity dissolves.

### Option R — REDEFINE (current crate deprecated; fresh implementation matching Alex's framing)

Deprecate current `fate` crate; land a fresh substrate-decl'd
implementation of `roll(space, hole) → dice_roll` on top of
@silicon/algebra.

- **Substrate-decl fit**: HIGH. Matches Alex's framing directly. The
  fresh implementation would:
  1. Consume @silicon/algebra crystals as the operations `A`.
  2. Consume @spectral/db numerical-dimension crystals as the state
     space `H`.
  3. Emit `dice_roll` values with `verdict` reporting whether the roll
     is in the restricted state space.
  4. Crystallize outputs into `@fate/algebra/{morphism,altitude,element}`.
- **Cost**: 2-3 weeks + genuine substrate-pull design work; requires
  landing `shards/fate/algebra.mirror` species-set first, then
  Rust discharge.
- **Unblocks**: everything the substrate has promised at @fate — @glue,
  @kintsugi, @reflection, @spectral/metalogue, cmd_spawn Phase H #6,
  peer-as-navigator's algedonic-gradient inference.
- **Violates**: throwaway of 4284 lines of proven prototype code — the
  5-model selector's 450 params, the BF→IR→native pipeline, the LAPACK
  Fortran forward-pass, the training corpus. Alex's framing at the top
  of the scout NAMES the crate as "prototype" — so this is throwaway of
  something Alex has already labeled as such.

### Option H — HYBRID (RECOMMENDED)

Two-layer @fate:
1. **Preserve the current `fate` crate as external prototype.** Fix
   prismqueer drift (half-day per §2.3). Keep 5-model selector +
   BF-runtime + ManifoldLoss + Casimir working. Version-pin at
   `fate = "0.1.0"` and DO NOT ADD to bootstrap dep tree.
2. **Land fresh `@fate/algebra/*` species + Rust discharge** on top of
   @silicon/algebra, matching Alex's framing. This IS Option R but
   without throwing away the prototype.

The current crate remains as: (i) an operational proof that the 5-model
selector + BF-runtime + Manifold + LAPACK forward-pass can be composed,
(ii) the loss-carrier substrate that `docs/specs/fate-silicon-metalogue-
in-void-duality-basis.md` NAMES for the metalogue observation record
(`ManifoldLoss` per Mara `a18ca90` §2.5.3), (iii) a fallback runtime for
optical inference (recognition #58 realization) once @fate/algebra's
substrate-decl'd operator is landed.

The fresh @fate/algebra implementation composes @silicon/algebra crystals
into substrate-decl inference operations. The old crate's 5-model
selector becomes ONE downstream consumer of @fate/algebra — a specific
instantiation where the algebra A = {Abyss, Introject, Cartographer,
Explorer, Fate} and the Hilbert space H = ℝ¹⁶.

- **Substrate-decl fit**: HIGH. Alex's framing preserved; substrate
  cleanly composes @silicon/algebra × @fate/algebra as declared.
- **Cost**: half-day (prismqueer drift fix) + 1-2 weeks (fresh species
  + Rust discharge). Total: 2 weeks arc.
- **Unblocks**: everything Option R unblocks + preserves the prototype
  as citation-truth for Recognition #58 optical-inference realization.
- **Violates**: nothing structural. Bootstrap dep tree stays clean; the
  fresh discharge lives at `bootstrap/src/fate/*.rs` or as a new
  `mirror-fate` crate that IS in bootstrap dep tree.
- **Substrate-already-had-the-word check**: `@fate/algebra` — the
  path-namespace already exists in prose at `shards/fate.mirror:216-232`.
  `@fate/tournament` — LANDED at `shards/fate/tournament.mirror`. Only
  new species needed: `shards/fate/algebra.mirror` sub-family-root + the
  three morphism/altitude/element sub-species. Two-tick discipline: land
  `@fate/algebra` family sub-root first, then sub-species when consumers
  pull.

---

## Task 5 — Recognition #58 recharacterization

Recognition #58 as promoted 2026-06-11: `[[architecture-fate-is-optical-
inference]]` — @fate inference IS 5-layer D²NN + active Fabry-Perot
resonator + Reck/Clements unitary mesh.

Under Alex's flang→numerical-inference framing:
- Optical inference IS ONE REALIZATION of what @fate does.
- @fate's essence is: "roll of the dice in the restricted state space"
  (`shards/fate.mirror:14-15`) — the constrained-inference operator over
  any restricted state space.
- Optical inference is the case where the state space is the D²NN's
  Hilbert space + the Fabry-Perot cavity modes + the Reck/Clements
  unitary mesh's parameter space. That is ONE (A, H, D, γ, J, tray)
  Connes tuple.
- Other realizations: LAPACK dsyev eigendecomposition (state = symmetric
  matrix, dice_roll = eigenvector/eigenvalue pair); statistical inference
  (state = sample space, dice_roll = MAP estimate); optical inference
  (state = photon field, dice_roll = mode selection); tournament
  selection over @bauchladen tray (state = candidate crystals, dice_roll
  = selected crystal).

**Recommend a recharacterization tick** (candidate wording, NOT
declared):

> Recognition #58 recharacterization candidate:
> @fate IS the constrained-inference operator (dice_roll within
> restricted state space per (A, H, D, γ, J, tray) Connes tuple).
> `[[architecture-fate-is-optical-inference]]` names ONE realization —
> the D²NN + Fabry-Perot + Reck/Clements realization at the optical
> altitude. The recognition PROMOTES from optical-inference-identity to
> constrained-inference-operator; optical inference remains as the
> load-bearing FIRST-witness realization.

This is a **PROMOTION**, not a demotion — the recognition becomes
broader while the optical altitude witness remains cited. It aligns with
recognition #58's role in the mirror-silicon.md §1.2 substrate landing
where @silicon is declared "the tournament's mechanism at the hardware
altitude."

**Alex to adjudicate.** Taut does not promote/recharacterize
unilaterally.

---

## Discipline summary

- **Read-only.** No shards edited. No substrate declared.
- **Files opened**: 12 (well under 30-file budget).
- **Grep-first**: every substrate claim cited to a file:line or
  commit SHA.
- **Substrate-already-had-the-word check**:
  `@fate` LANDED at `shards/fate.mirror`; `@fate/algebra` prose-namespace
  landed; `@fate/tournament` species LANDED at `shards/fate/tournament.
  mirror`; `@silicon/algebra` LANDED at `shards/silicon/algebra.mirror`.
  No new family-root proposals. Only new sub-species proposed:
  `shards/fate/algebra.mirror` sub-family + `@fate/algebra/{morphism,
  altitude,element}` sub-species when consumers pull.
- **Two-tick discipline**: sub-family-root before species. Land
  `shards/fate/algebra.mirror` FIRST; add sub-species only when the
  first consumer (peer-as-navigator's algedonic-inference or cmd_spawn's
  Phase H #6) actually pulls.
- **Pure-markdown commit**: 📝 marker, auto-bypasses pre-commit hook.
  NO `--no-verify`.
- **Read-only.** Reed adjudicates. Alex ratifies.

---

## Under-350-word summary (for Reed's session-cascade)

The current `fate` crate is a 5-model selector prototype AS DECLARED in
its own docs (`fate/Cargo.toml:5`: "Five models. One selector. Zero
dependencies."). It implements `Fate::select(Model, &[f64;16]) →
Decision` with 450 hardcoded params, a Brainfuck-interpreter runtime, a
build.rs BF→IR→native codegen pipeline, and an optional LAPACK Fortran
forward-pass.

Mirror substrate has moved past that. `shards/fate.mirror` (LANDED at
19a72e9) declares @fate as the constrained-inference operator with
signature `roll(space: restricted_state_space, hole: hole) → dice_roll`
+ bilateral compile-time/runtime dispatch + inheritance from
@autopoietic + crystallization into @bauchladen tray under
`@fate/algebra/*` path-namespace. This is generic over ANY constrained
state space; the current crate is specialized to `(Model, [f64;16])`.

**Bootstrap does not depend on fate today.** Grep confirms zero
`use fate` imports in `bootstrap/src/**/*.rs`. Fate is an orphan crate.

Alex's framing (2026-07-08) — "fate = flang → numerical inference
engine" — is verified at substrate altitude via Mara `a18ca90`
metalogue reading: @fate/algebra PROPOSES morphisms; @silicon/algebra
REALIZES them against LAPACK. Recognition #58 (@fate IS optical
inference) is ONE realization at the optical altitude, not @fate's
essence.

prismqueer API drift is mostly mechanical: `fn refract` × 3 sites → `fn
settle` rename. Plus 3-5 impl block additions for GroupStructure /
LawvereFixedPoint / act_on. Half-day of work.

**Verdict: HYBRID (Option H).** (i) Fix prismqueer drift in current
crate; keep it as external prototype at `fate = "0.1.0"`. It carries
Recognition #58's optical-inference witness + ManifoldLoss (which Mara
`a18ca90` §2.5.3 NAMES as the metalogue observation record). (ii) Land
fresh `shards/fate/algebra.mirror` sub-family + Rust discharge on top of
@silicon/algebra matching Alex's framing. The 5-model selector becomes
ONE downstream consumer of @fate/algebra.

Recharacterize Recognition #58 to promote optical inference from
identity to load-bearing first realization — @fate's essence is
constrained-inference over any restricted state space. Alex to
adjudicate.

Top-3 signals summarized at scout head.

*Taut, 2026-07-08 evening, read-only.*
