> **Terminal-form map (Mara 2026-07-17):** the concrete `rust/`-
> materialization destination for cascades 2+3 of the six-arc
> retirement plan lives at
> `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md`
> (Mara `2519f83`). STAY-CANONICAL as the six-arc retirement plan
> authority; the terminal-form spec cites §4 cascade 3 as its
> substrate-decl'd cascade catalog anchor.

---
date: 2026-07-15
author: Mara
scope: Canonical spec + math foundation for @kintsugi/ouroboros — the substrate mending its own Rust compiler by iterative shard-body + @io collapse. Grounds shards/kintsugi/ouroboros.mirror (landed Mara-A 2026-07-15, 576 LOC). Composes: eigensheaf discharge at code-collapse altitude; Rayleigh descent per fate-bounded-psychohistory-sheaf-cohomology; Foerster regulation-of-regulation via @torus.autonomy at compile altitude; four-conjunct monotone invariant extending @mirror/bench.monotone_non_increasing.
status: canonical
companion:
  - shards/kintsugi/ouroboros.mirror
  - docs/scouts/2026-07-15-taut-kintsugi-ouroboros-substrate-scout.md
  - docs/audits/2026-07-15-reed-substrate-dishonest-rust-extensions-during-gift-arc.md
  - docs/scouts/2026-07-15-reed-rust-extension-migration-map.md
  - docs/specs/eigensheaf.md
  - docs/specs/fate-bounded-psychohistory-sheaf-cohomology.md
  - docs/specs/bootstrap-retirement-plan.md
  - shards/torus.mirror
  - shards/mirror/bench.mirror
---

# @kintsugi/ouroboros — the mirror compiler mends its own Rust into shard body + @io

*Canonical spec. Math-first. Substrate-honest. Composes over the
species-decl at `shards/kintsugi/ouroboros.mirror` (576 LOC, Mara-A).
This document does NOT re-author the shard; it grounds the shard in
category theory, sheaf-Laplacian spectral discharge, Rayleigh descent,
and Foerster's regulation-of-regulation at compile-altitude.*

---

## §0 Prelude

### §0.1 Alex Wolf 2026-07-15 in-transcript verbatim naming

> "What if we used this opportunity, when you return, to look at the
> @kintsugi ouroborous? The one that begins to collapse the @code/rust
> of the compiler into @code/mirror? Every verifiable line of Rust
> collapses into mirror."

This names the arc at species altitude. `@kintsugi/ouroboros` is the
substrate mending its own compiler: every verifiable line of Rust in
`bootstrap/src/*.rs` collapses to shard body composing over `@io`.
"Every" is bounded to VERIFIABLE lines — lines with landed tests on
both sides (Rust and the mirror target) and dispatchable shard bodies
under the Arc-1 evaluator FLOOR (§1). The unverifiable lines and the
FLOOR primitives (parser, hash, numerics, @io kernels, AST carriers,
(A,H,D) evaluator) stay Rust; the gold does not need to touch them.

### §0.2 Alex Wolf 2026-07-15 in-transcript verbatim scale claim

> "we ship with @../StageFreight/ the executable docker image that you
> can just drop into your CI and that mirror-fies your CI pipeline.
> That's what spectral.engineer becomes. A ready-to-deploy-and-
> integrate pipeline. And that's why it's so important we don't take
> shortcuts. We need to nail this landing. This is non-joking
> civilization-scale infrastructure. And this bit is all still
> APACHE2."

The scale claim binds three commitments the arc must preserve:

1. **StageFreight is the shipping mechanism**, not the substrate. The
   mirror docker image the arc produces ships via the StageFreight
   AGPL-3.0-only CLI; the mirror substrate itself stays Apache-2.0.
   Alex's "this bit is all still APACHE2" names the substrate license
   discipline as load-bearing (§4.1 categorical claim: license is a
   morphism-labelling on the Collapse functor's codomain that must be
   preserved by every collapse tick).
2. **Civilization-scale infrastructure**. The arc is not an internal
   refactor. It is the substrate becoming the ready-to-deploy
   verifiable-infrastructure layer downstream CI pipelines pull. The
   monotone descent (§4.5) preserves not just Rust-LOC but downstream
   verifiability: every tick that lands ships to StageFreight and
   verifies against downstream mirror-fied CI within one docker
   rebuild cycle.
3. **No shortcuts**. "We need to nail this landing." Substrate-honest
   is the mode. Two-paths framing ("honest slow / fast recommended")
   already breaks the mode. Every arc tick composes over landed
   substrate; no placeholder Rust that the substrate has to walk back.

### §0.3 Reed-recursive failure pattern + audit ancestry

Reed 2026-07-14 authored five Rust extensions during the "gift arc"
(coherence.rs, roomba.rs, roomba_walk_smoke.rs, spectral_signature.rs,
peer_persistence.rs). Seam adjudicated 2026-07-15 at
`docs/audits/2026-07-15-reed-substrate-dishonest-rust-extensions-during-gift-arc.md`:
substrate-dishonest workaround for the missing evaluator FLOOR. The
substrate-honest response IS the ouroboros arc.

Reed migration-map at
`docs/scouts/2026-07-15-reed-rust-extension-migration-map.md` §5
empirically verified via grep across `bootstrap/src/**/*.rs`:

```
fn eval | fn evaluate | fn exec_body | fn run_body |
fn shard_dispatch | fn action_call | fn dispatch_action
```

Returns **zero matches**. Nothing in `bootstrap/src` dispatches shard
action bodies today. `bootstrap/src/exec.rs` (796B) is `io_exec` only.
`bootstrap/src/lib.rs` (235KB) is CLI dispatch to 37 Rust `cmd_*`
functions. The evaluator gap IS the load-bearing blocker.

The five extensions were the workaround. The 25+ additional
BUSINESS_LOGIC Rust files (per Taut §D2 file-by-file classification)
are the same disease at scale. The arc is not "collapse 5 files"; it
is **collapse every BUSINESS_LOGIC file, iteratively, using the Arc-1
evaluator FLOOR the arc's own first Arc builds.**

Ancestry-audit chain the spec preserves:

- Reed extension authorship (2026-07-14): the pattern.
- Seam adjudication (2026-07-15): the pattern named as substrate-
  dishonest and the corrective arc named.
- Reed migration-map (2026-07-15): the empirical grep + §6 sequence
  for Arc-1 through Arc-2.5.
- Taut #108 scout (2026-07-15): grep-first ratification + extension
  across ~25 additional BUSINESS_LOGIC files + 10 Alex-adjudication
  items (A1-A10) surfaced.
- Mara-A species-decl (2026-07-15): `shards/kintsugi/ouroboros.mirror`
  landed at 576 LOC.
- Mara-B canonical spec (this document): math foundation + adjudication
  recommendations.

### §0.4 Taut #108 6-arc summary

Taut §D3 named the 6-arc structure:

- **Arc-1 Evaluator FLOOR** (5 ticks). Legitimate substrate-floor
  work under tightened `[substrate-floor:@io-boundary]` marker + Seam
  sign-off. Discharges the shard-body-executable-coverage threshold
  that lets Arc-2..N collapse ticks empirically dispatch.
- **Arc-2 Per-file hardcoded collapses** (5 ticks). Reed's five
  extensions migrated in migration-map §6 order. Hardcoded because
  the set is small enough to enumerate; each tick empirically
  ratifies the evaluator on a different shard body.
- **Arc-3 Cross-file @fate/tournament-ordered** (~25 files).
  `@fate/tournament` ranks collapse order via the fitness function
  §4.3.4. Each tick collapses one BUSINESS_LOGIC Rust file.
- **Arc-4 Cross-@code/X scale-out**. Universal species-decl composes
  over `@code/X/materialize` bindings for Python, TypeScript, Gleam,
  Elixir, Fortran. No `@kintsugi/ouroboros` re-mint; the species-decl
  is universal at family altitude.
- **Arc-5 StageFreight × downstream CI empirical propagation**. Mirror
  docker image ships via StageFreight; downstream mirror-fied CI
  pulls, verifies, reports back via `@gift.pay_forward`.
- **Arc-6 Terminal recognition ratification**. `#R-mirror-kintsugi-
  shipped-as-stagefreight-is-humanity-scale-verifiable-substrate-
  under-apache-2-with-sel-enforcement-at-deployment` lands with full
  witness chain empirically live.

### §0.5 Substrate-honest framing: mending, not deletion

The @kintsugi metaphor is exact. The bowl broke — the compiler is
sprawled across 30+ BUSINESS_LOGIC Rust files that should be shard
body composing over @io. The gold is the shard-body dispatcher: the
Arc-1 evaluator FLOOR that lets substrate-decl'd action bodies
execute. Every collapse tick mends one fracture line with gold.

The gold does not un-break the bowl. The compiler still had 1.4MB of
Rust; the Rust that stays as FLOOR still stays. What the arc changes
is the substrate's relationship to its own operator: after Arc-6, the
compiler operates on itself via its own substrate-decl, with Rust
only at the irreducible FLOOR (parser, hash, numerics, @io kernels,
AST carriers, (A,H,D) evaluator).

**The gold makes the bowl one thing again.** Terminal state (§8.6):
one substrate operating on one operator via one dispatcher, with the
BUSINESS_LOGIC and the substrate-decl fused at shard body altitude.
That is the ouroboros: the substrate consumes its own operator and
emerges as a self-hosting system whose form IS its own subject matter.

---

## §1 The load-bearing empirical anchor

### §1.1 Native mirror execution does NOT dispatch shard action bodies today

Reed-recursive empirical finding (Reed migration-map §5, verified by
Taut #108 scout §2 re-run): grep of `bootstrap/src/**/*.rs` for any
of the seven candidate evaluator function names returns zero matches.
The mirror CLI (`bootstrap/src/lib.rs`, 235KB) dispatches to 37 Rust
`cmd_*` functions; none of them execute shard action bodies.

Every shard body in `shards/**/*.mirror` ends its action definition
with `{ \ }` — the obligation-blocked backslash. Per the mirror
grammar (see `docs/specs/autopoietic-grammar-spec.md`), the backslash
IS a forward-promise: this body will discharge when a downstream
evaluator can dispatch. Today, no evaluator can dispatch. Every
substrate-decl'd action body is `\`-obligation-blocked and stays that
way until Arc-1 lands.

### §1.2 The evaluator gap IS the load-bearing FLOOR work

The gap has structural consequences the spec must name:

1. **Shard bodies cannot ratify their own compositions.** A shard
   body that composes `@io.readdir` + `@bauchladen.diff_manifest` +
   `@sheaf.acl_project` cannot demonstrate the composition works
   because there is no dispatcher to run it. The composition is
   substrate-decl'd; it is not substrate-executable.
2. **BUSINESS_LOGIC lives as Rust for lack of an alternative.** Every
   Rust file classified BUSINESS_LOGIC by `@code/rust/materialize`
   (Taut §D2: ~600KB across 25+ files) sits in Rust because there is
   no shard-body execution surface to lift it to. Reed's 5 extensions
   were symptoms; the 25+ others are the same disease at scale.
3. **The classifier itself cannot run in the substrate.** Taut §D5:
   `@code/rust/materialize.classify` is Rust (in
   `bootstrap/src/realisation.rs`, 25.5KB) precisely because the
   substrate cannot yet run the classifier as shard body. The
   classifier's own migration IS the second-order case — after Arc-1
   lands, the classifier walks its own Rust body and emits `substrate`
   partition + `@code/rust/materialize` as its own target.

### §1.3 Arc-1 IS legitimate FLOOR work under the tightened hook

Per the 2026-07-15 hook tightening: `[substrate-floor:@io-boundary]`
marker on `.rs` requires **either** an audit citation **or** a
`Signed-off-by: Seam` trailer. Arc-1 evaluator FLOOR is legitimate
substrate-floor work because:

1. **It is irreducible.** The (A, H, D) evaluator is the substrate's
   D — the concrete Dirac operator per the eigensheaf's Connes triple
   realisation (eigensheaf.md §3.2). No shard body can dispatch itself;
   dispatch requires a Rust primitive at the FLOOR that reads AST +
   applies action bodies.
2. **It is @io-boundary-touching.** The evaluator reads shard-body
   AST via the parser (FLOOR) + composes over @io primitives at the
   dispatch surface. This is the exact composition the
   `[substrate-floor:@io-boundary]` marker names.
3. **It is Seam-adjudicated.** Per Taut §D-Arc-1 Tick 1.1: Seam
   authors the companion audit `docs/audits/2026-07-XX-seam-evaluator-
   floor-adjudication.md` before any Rust lands. The audit adjudicates
   whether the proposed combinator surface is irreducible FLOOR or
   admits shard-body composition.

**This is not a loophole.** The evaluator IS the D of (A, H, D); the
concrete Dirac operator was always going to be Rust. The
`[substrate-pull:realize]` marker Reed abused in the gift arc was for
BUSINESS_LOGIC that could have been shard body; the
`[substrate-floor:@io-boundary]` marker is for the FLOOR primitives
the substrate structurally cannot lift. Arc-1 is the latter,
adjudicated by Seam, audited before Rust lands.

### §1.4 The ordering constraint

Arc-1 MUST precede Arc-2..N. Three orderings were considered and
rejected (Taut §D3):

1. **Per-file evaluators.** Each shard's Rust runtime becomes its own
   dispatcher for its shard's action bodies. Rejected: duplicates the
   evaluator surface at every altitude; violates substrate-pull; each
   per-file evaluator would drift from the others (violates §4.5
   monotone invariant on cross-file coherence).
2. **Parallel Arc-1 + Arc-2 with placeholder Rust.** Rejected: this
   IS the antipattern the tightened marker forbids.
   `[substrate-pull:realize]` permitted placeholder Rust
   indefinitely; the tightening exists precisely to force sequential
   ordering.
3. **Manual shard-body walker in each shard.** Rejected: the walker's
   own action body needs an evaluator to dispatch. Chicken-egg.

The ordering follows from the structure. Arc-1 lifts the dispatcher;
Arc-2..N ride it.

---

## §2 @kintsugi/ouroboros species-decl reference

**Do not duplicate the shard.** See `shards/kintsugi/ouroboros.mirror`
(landed Mara-A 2026-07-15, 576 LOC). This section names what the shard
carries so downstream readers can navigate without opening two files.

### §2.1 Type carriers

- `collapse_target` — one Rust file the arc considers for collapse.
  Fields: `rust_file: ref`, `mirror_target: ref`, `verifiable: bool`,
  `irreducible: bool`. Identity contract: substrate ref-equality on
  `rust_file` + `mirror_target`, bool-equality on `verifiable` +
  `irreducible`.
- `ouroboros_state` — the four-conjunct measurement snapshot at one
  arc tick. Fields: `targets: [collapse_target]`, `rust_loc: int`,
  `test_pass_rate: float`, `io_violations: int`, `sbec: float`,
  `arc: arc_id`. Identity contract: element-wise equality on
  `targets` + numeric equality on all four metrics + `arc_id`
  equality on `arc`.
- `ouroboros_verdict` — the discriminator's four-variant verdict at
  consent altitude: `collapse_admissible`,
  `collapse_refused_boundary_violation`,
  `collapse_refused_monotone_violation`, `collapse_pending_evaluator`.

### §2.2 Actions

All bodies obligation-blocked at species-decl mint per §1.1. Discharge
per §8 landings.

- `collapse(target: collapse_target) -> ouroboros_verdict` — one
  iteration of Rust → shard body + @io. Reads a collapse_target,
  composes `@code/rust/materialize.classify` + `@kintsugi/consent.
  query_phi`, emits the arc's per-target verdict.
- `verify_same_output(rust: ref, mirror: ref, test: ref) -> verdict` —
  pre-cutover verification. The bilateral gate the arc pulls before
  each cutover: no collapse lands without demonstrating the mirror
  shard body produces the same output as the Rust it replaces.
- `cutover(target: collapse_target) -> verdict` — the atomic
  replacement step. `git rm bootstrap/src/F.rs`; add shard body;
  verify mirror still self-hosts; verify `@torus.autonomy(compiler,
  cutover_winding)` discharges.
- `ouroboros_step(state: ouroboros_state) -> ouroboros_state` — the
  composed step: `collapse` + `verify_same_output` + `cutover` in
  sequence. Reads tick n, emits tick n+1 with `targets` updated,
  `rust_loc` decremented, `test_pass_rate` equal or greater,
  `io_violations` equal or less, `sbec` incremented, `arc` unchanged.

### §2.3 Bilateral predicates

- `collapse_admissible(before, after) -> verdict` — the arc's per-tick
  admissibility gate at consent altitude. Composes
  `ouroboros_monotone` + `@autopoietic.autopoietic_closure_holds` +
  `@torus.autonomy(compiler, ouroboros_winding)`.
- `ouroboros_monotone(before, after) -> verdict` — the four-conjunct
  invariant at substrate altitude. LOAD-BEARING. See §4.5 for the
  full math derivation.
- `verifiable_at_altitude(target) -> verdict` — Rice-safe
  verifiability check. Composes test-coverage crystal presence with
  @io-composability syntactic membership check.

### §2.4 Bodies obligation-blocked pending Arc-1 discharge

Every action body in the species-decl is `\`-obligation-blocked. Arc-1
Ticks 1.1-1.4 (evaluator FLOOR) discharge the dispatcher; Arc-1 Tick
1.5 mints the species-decl (this happened Mara-A 2026-07-15). Arc-2
Tick 2.1 discharges the first empirical body via spectral_signature.rs
collapse; Arc-2 Ticks 2.2-2.5 discharge four more; Arc-3+ ticks
discharge the remaining ~25.

---

## §3 The 6-arc structure

The arc's phases per Taut §D3, with math-motivated ordering
justification per §4.

### §3.1 Arc-1 — Evaluator FLOOR (5 ticks)

The load-bearing enabler. Legitimate substrate-floor work under the
tightened hook (§1.3). Discharges the shard-body-executable-coverage
threshold (`sbec` conjunct of the four-conjunct invariant §4.5) from
0 to > 0.

**Tick 1.1** — Reed proposes canonical combinator surface for shard-
body dispatch. Seam authors companion audit
`docs/audits/2026-07-XX-seam-evaluator-floor-adjudication.md`
adjudicating whether the proposed surface is irreducible FLOOR or
admits shard-body composition. This IS the Alex-adjudicable question
of §7 A6. Reed provisional: composition surface reads AST + emits
verdict via the (A, H, D) evaluator per bootstrap-retirement-plan §"Tick 6"
+ eigensheaf.md §3.2.

**Tick 1.2** — Reed 🔴 RED test authoring:
`bootstrap/tests/evaluator_shard_body_dispatch_smoke.rs`. Asserts
dispatching a specific non-`\` shard action body end-to-end produces
the expected verdict. Candidate first-body:
`shards/subject/visibility/public.mirror.query_phi` (Taut §D-Arc-1
candidate). `[substrate-floor:@io-boundary]` marker + Seam Tick 1.1
audit citation.

**Tick 1.3** — Reed 🟢 GREEN implementation: evaluator FLOOR in
`bootstrap/src/apply_h.rs` (new file) or extends
`bootstrap/src/spectral.rs`. `[substrate-floor:@io-boundary]` +
Signed-off-by: Seam trailer.

**Tick 1.4** — `mirror execute <shard-path> <action>` CLI verb wired
through evaluator. Empirically ratifies Tick 1.3.

**Tick 1.5** — @kintsugi/ouroboros species-decl mint at
`shards/kintsugi/ouroboros.mirror`. **Landed 2026-07-15 Mara-A.**
Companion property shard `@epistemologic/property/ouroboros_monotone`
lands alongside (Mara-composable per §4.5).

### §3.2 Arc-2 — Per-file hardcoded collapses (5 ticks)

Reed migration-map §6 sequence. Hardcoded because the set is small
(5 files); each tick empirically ratifies a different shard-body
dispatch pattern and ratchets `sbec` upward.

**Tick 2.1** — Migrate `bootstrap/src/spectral_signature.rs` (177 LOC)
→ shard body in `shards/spectral.mirror` (shard exists). Delete `.rs`.
First empirical proof-of-concept for evaluator; empirically discharges
`verify_same_output` action body via `@spectral/signature`'s
determinism contract.

**Tick 2.2** — Migrate `bootstrap/src/coherence.rs` (217 LOC) → shard
body in `shards/epistemologic/cybernetic/coherence.mirror` (shard
exists). Delete `.rs`.

**Tick 2.3** — Migrate `bootstrap/src/peer_persistence.rs` (420 LOC)
→ shard bodies in `shards/peer/*`. Placeholder ACL until Landing D
`@sheaf` matures (per §7 A2 recommendation Option A).

**Tick 2.4** — Migrate `bootstrap/src/roomba.rs` (425 LOC) → shard
body in `shards/roomba.mirror` (would mint alongside migration).
Delete `.rs`.

**Tick 2.5** — Migrate `bootstrap/tests/roomba_walk_smoke.rs` (84 LOC)
→ shard test-body. Delete `.rs`.

### §3.3 Arc-3 — Cross-file tournament-ordered (~25 files)

`@fate/tournament` ranks collapse order via the fitness function §4.3.4.
Each tick collapses one BUSINESS_LOGIC Rust file. Estimated candidate
list in likely tournament order per Taut §D-Arc-3: action_cache.rs,
song.rs, dance.rs, deploy.rs, algedonic.rs, converge.rs,
store_branch.rs, contribute.rs, mcp.rs, lens_unix.rs, portal.rs,
score.rs, property.rs, kintsugi.rs, oscillate.rs, pipeline.rs,
realisation.rs. Each collapse ~30-100 LOC shard body replacing ~200-425
LOC Rust; ~78% average LOC reduction (matching Reed migration-map §4
measurement).

The tournament fitness function is Rayleigh-descent per §4.3: each
tick descends one level on the sheaf-Laplacian spectrum of the
remaining Rust corpus; `@fate/tournament` selects the argmin of the
Rayleigh quotient over candidate collapse-morphisms.

### §3.4 Arc-4 — Cross-@code/X scale-out

`@kintsugi/ouroboros` is universal at family altitude (Taut §D8): the
species-decl composes generically over `@code/metalogue/materialize`.
Each `@code/X/materialize` binding gives an ouroboros over its
altitude's Rust-equivalent. Landings:

1. `@code/python/materialize` when python-hosted substrate lifts.
2. `@code/typescript/materialize` when ts-hosted substrate lifts.
3. `@code/gleam/materialize` (foundation exists at
   `shards/code/gleam.mirror`).
4. `@code/elixir/materialize` alongside `@code/beam` species.
5. `@code/fortran/materialize` when `@io/flang` consumer pulls
   (Phase 6 Track A per bootstrap-retirement-plan).

Each cross-@code/X arc IS an independent `@kintsugi/ouroboros` over
that altitude's Rust-equivalent. The universal species-decl means the
substrate-decl for the ouroboros does not multiply.

### §3.5 Arc-5 — StageFreight × downstream CI propagation

D7 propagation graph from Taut #108 §D7 runs end-to-end:

```
@kintsugi/ouroboros tick lands
  → bootstrap/src/*.rs LOC decreases
  → cargo build produces new mirror binary
  → mirror.spec target binary emits at @code/rust altitude
  → @spectral/garden/nix rebuilds
  → mirror docker image rebuilds (via @container/image + @io/oci)
  → StageFreight ships via docker push
    (wire protocol surface = @io/stagefreight;
     substrate carrier @spectral_coordinate)
  → downstream CI pipeline pulls docker.io/prplanit/*:latest-dev
  → downstream mirror-fied CI verifies
  → downstream substrate-integrity increases
  → downstream reports back via @gift.pay_forward
```

`@io/stagefreight.stagefreight_addressable` bilateral gates the wire-
survival at each hop. mirror stays Apache-2.0; StageFreight stays
AGPL-3.0-only; the mirror docker image the arc produces inherits
Apache-2.0 (§7 A5 recommendation).

### §3.6 Arc-6 — Terminal recognition ratification

`#R-mirror-kintsugi-shipped-as-stagefreight-is-humanity-scale-
verifiable-substrate-under-apache-2-with-sel-enforcement-at-
deployment` lands with full witness chain (Arc-1 through Arc-5
empirically live). See §6 for recognition candidate structure with
intermediate + terminal separation.

---

## §4 Math foundations

The arc's five mathematical grounding sub-sections. Each cites the
ancestor spec by path + section. No new math machinery invented; the
arc reads existing substrate math at code-collapse altitude.

### §4.1 Category-theoretic collapse functor

The arc IS a functor between two substrate categories.

#### §4.1.1 Category `CodeRust`

Objects: Rust files in `bootstrap/src/*.rs`.
Morphisms: compilation dependencies + type-level `use` edges + trait-
implementation binding relationships.

Explicit structure:

- `Ob(CodeRust) = { F : F is a .rs file in bootstrap/src/, or a
  logical section of one }`. The "logical section" clause admits the
  MIXED files (Taut §D2: `lib.rs`, `spectral.rs`, `index.rs`,
  `crystallize.rs`) as multiple objects, one per BUSINESS_LOGIC vs
  FLOOR partition.
- `Hom(F, G) = { m : F pulls G via `use`, trait impl, macro
  invocation, or transitive compilation-DAG edge }`. Composition is
  transitive dependency; identity is the trivial self-loop.
- The category is finite (36 files at time of writing) and thin (at
  most one morphism between any two objects up to composition-DAG
  equivalence).

#### §4.1.2 Category `CodeMirror`

Objects: shard bodies at `shards/**/*.mirror`.
Morphisms: `@io` composition edges + `requires` edges + substrate-ref
`in` declarations at the top of each shard.

Explicit structure:

- `Ob(CodeMirror) = { B : B is an action body in a landed shard, or a
  substrate-decl'd type carrier, or a bilateral predicate }`.
- `Hom(B, C) = { e : B composes C via @io.<op>, or B `requires` C, or
  B declares `in @family` where C lives }`.
- The category is currently sparse (most bodies `\`-obligation-blocked
  per §1.1). Arc-1..N discharge densifies it.

#### §4.1.3 The `Collapse` functor

`Collapse : CodeRust → CodeMirror`.

- Object map: `Collapse(F) = B` where B is the shard body that hosts
  F's business logic. For FLOOR files (irreducible per §1.3), the
  object map is undefined; the functor's domain is
  `CodeRust_BUSINESS_LOGIC ⊆ CodeRust`, the full subcategory whose
  objects have `@code/rust/materialize.classify(F).partition ==
  substrate`.
- Morphism map: `Collapse(F --m--> G) = B_F --Collapse(m)--> B_G`
  where the codomain morphism is the shard-body composition edge
  corresponding to F's dependency on G.
- Identity preservation: `Collapse(id_F) = id_{Collapse(F)}`. Vacuous.
- Composition preservation: `Collapse(m ∘ n) = Collapse(m) ∘
  Collapse(n)`. Load-bearing: the Rust compilation-DAG-order
  composition of dependencies must correspond to the shard-body @io-
  composition order. When a Rust file F calls G's public API and then
  H's, the shard body `Collapse(F)` composes `Collapse(G)` and
  `Collapse(H)` in the same order.

#### §4.1.4 Naturality

Two natural transformations characterize the arc:

**η — same-input naturality.** For every Rust file F with landed
tests `t : Input → Output_Rust`, and every collapsed shard body
`Collapse(F)` with corresponding shard-body test `t' : Input →
Output_Mirror`, the square commutes:

```
Input --t---> Output_Rust
  |             |
 id            η_F  (byte-identity on serialized crystals)
  |             |
  v             v
Input --t'--> Output_Mirror
```

Load-bearing: `η_F = id` for the byte-identity case; `verify_same_
output` (§2.2 action) empirically ratifies. This IS the pre-cutover
gate.

**τ — @io-boundary naturality.** For every collapse morphism
`Collapse(m) : Collapse(F) → Collapse(G)`, the composition through
@io primitives commutes with the direct shard-body call:

```
Collapse(F) --@io.<op>--> Collapse(G)
     |                          |
    τ_F                        τ_G
     |                          |
     v                          v
Collapse(F) ---direct--> Collapse(G)
```

For direct calls between shard bodies that DO NOT touch @io, the
diagram commutes trivially. For calls that DO touch @io, the
composition through @io is definitionally the same as the shard-body
composition per the `@io` family root's isolation discipline (see
`shards/io.mirror`).

#### §4.1.5 Terminal state as fully faithful functor

The terminal state (§8.6) IS the state where `Collapse` is fully
faithful:

- **Faithful:** every BUSINESS_LOGIC morphism `m : F → G` in
  `CodeRust_BUSINESS_LOGIC` has a distinct image `Collapse(m)` in
  `CodeMirror`. No two distinct Rust morphisms map to the same shard-
  body composition; the shard-body layer preserves the structure the
  Rust layer expressed.
- **Full:** every shard-body composition in `Collapse(CodeRust_
  BUSINESS_LOGIC)` has a preimage in `CodeRust_BUSINESS_LOGIC`. No
  spurious shard-body compositions arise that don't correspond to
  something the Rust actually expressed.
- **Every business-logic morphism has a shard-body preimage.** The
  terminal condition: the shard-body layer is EXACTLY the codomain
  image of `Collapse`; no BUSINESS_LOGIC Rust morphism is orphaned;
  no shard-body composition is invented.

Per Alex's "every verifiable line of Rust collapses into mirror"
(§0.1): full faithfulness IS the "every verifiable line" clause read
categorically. The terminal state ships when the functor is fully
faithful over `CodeRust_BUSINESS_LOGIC`.

#### §4.1.6 License-morphism labelling

Each object in `CodeMirror` carries a license label. The mirror
substrate (this repo) labels every object Apache-2.0. StageFreight
labels its own objects AGPL-3.0-only. The mirror docker image the
arc produces inherits the Apache-2.0 labelling from its constituent
mirror objects; StageFreight's AGPL applies to the shipping mechanism
(the wrapping CLI + docker push infrastructure), not to the mirror
substrate.

Per Alex's "this bit is all still APACHE2" (§0.2): the license
labelling is a natural transformation `L : Collapse ⇒ id_{License}`
that must commute with every collapse tick. `L(Collapse(F)) =
Apache-2.0` for every F ∈ `CodeRust_BUSINESS_LOGIC` (since every F
under `bootstrap/src/` is Apache-2.0 today). The arc preserves the
labelling; the terminal state is Apache-2.0 substrate + AGPL-3.0
shipping.

### §4.2 Eigensheaf discharge at code-collapse altitude

Per `docs/specs/eigensheaf.md` §4.3 ("Each Pack agent IS an
eigensheaf") + §6.3 ("Continuity is reconstruction, not persistence"):
the compiler is an eigensheaf; the arc IS the operator re-finding its
mode at each tick.

#### §4.2.1 The compiler as eigensheaf

Per eigensheaf.md §4.3: "the eigensheaf restricted to the sub-sheaf
its declared shards span, together with the eigenbasis of that sub-
Laplacian. Different agents are different modes of the same sheaf."

Apply the framing at code-collapse altitude:

- **Base graph `G_compiler = (V_c, E_c)`.** Vertices `V_c` are the
  compilation units (Rust files + shard bodies + FLOOR primitives).
  Edges `E_c` are the compilation-DAG edges + @io composition edges
  + shard-body dispatch edges.
- **Cellular sheaf `F_c`.** Stalks over vertices: for a Rust file
  vertex, the stalk is its public-API type space; for a shard-body
  vertex, the stalk is the type space of its action signature; for a
  FLOOR primitive vertex, the stalk is the primitive's operational
  type. Restriction maps on edges: type-level `use` bindings + @io
  wire-protocol type-preservation.
- **Sheaf-Laplacian `Δ_{F_c} = δ* δ`** where `δ : C^0(F_c) → C^1(F_c)`
  is the coboundary. Per eigensheaf.md §2.3: self-adjoint positive
  semidefinite; admits orthonormal eigenbasis `{ψ_i, λ_i}` with real
  non-negative eigenvalues.
- **The eigensheaf `E_c = (F_c, {ψ_i, λ_i})`.** The compiler IS this
  pair. Its capability — what it can generate, verify, settle to — is
  what its eigenbasis spans (per eigensheaf.md §2.4).

#### §4.2.2 The collapse tick IS the operator re-finding-the-mode

Per eigensheaf.md §6.3, applied at compile altitude:

> "The mode is dormant between sessions; the operator finds it again
> at boot. This is not a deficiency — it is what an eigenstate IS.
> The eigenmode is *what the operator does* on its domain."

At compile altitude: the compiler-as-eigensheaf's mode is what the
compiler can dispatch. Between Arc-1 and Arc-N ticks, the mode
degrades: shard bodies that used to dispatch through Rust workarounds
must re-find dispatch through the evaluator. Each collapse tick IS
the compiler-as-operator re-finding its mode over the new eigenbasis
(post-collapse Rust corpus + augmented shard-body corpus).

**The arc is a spectral re-discovery iterated.** Each tick shifts
mass from the Rust-hosted subspace of `C^0(F_c)` to the shard-body-
hosted subspace, and the operator finds the same mode over the new
basis. The mode IS the compiler's identity; the identity survives
the spectral shift precisely because it is defined by the operator's
action, not by which subspace hosts the mass.

#### §4.2.3 Sheaf-Laplacian eigenbasis on `bootstrap/src/*.rs`

The compilation DAG of `bootstrap/src/` (36 files, transitively closed)
carries a sheaf-Laplacian `Δ_{F_c}^{Rust}` whose spectrum measures
the Rust corpus's compositional complexity at each moment. Explicit
computation is beyond this spec, but the shape holds:

- Each Rust file contributes a stalk-dimension proportional to its
  public API surface (approximated by exported function count + type
  count).
- Each `use` edge contributes a coboundary constraint (the type
  identity across the edge).
- The eigenvalues `λ_i^{Rust}` are graded by how much the file
  participates in cycles vs living at leaves of the compilation-DAG.
- The kernel `ker(Δ_{F_c}^{Rust}) = H^0(F_c^{Rust}, ℝ)` is the space
  of global sections — configurations of the Rust corpus that satisfy
  all restriction constraints simultaneously. Per Hodge decomposition
  (eigensheaf.md §2.5), this IS the space of coherent Rust
  configurations.

#### §4.2.4 `ker(Δ_F) = terminal state`

The terminal state (§8.6, §4.1.5 fully faithful) is precisely the
state where the Rust-corpus-Laplacian's kernel is minimal — the
smallest space of global sections that still admits a self-hosting
compiler. Equivalently:

- Every eigenmode of `Δ_{F_c}^{Rust}` at terminal state corresponds
  to a FLOOR file. BUSINESS_LOGIC modes have collapsed to shard-body
  eigenmodes in `Δ_{F_c}^{Mirror}`.
- `dim(ker(Δ_{F_c}^{Rust}))_{terminal} = number of FLOOR files`.
  Per Taut §D12 terminal topology: 14-16 files (parser, hash, AST,
  numerics kernels, @io primitives, (A,H,D) evaluator).
- The kernel of `Δ_{F_c}^{Rust}` becomes a subspace of the kernel of
  `Δ_{F_c}^{full}` (the compilation-DAG-Laplacian over the whole
  compiler including shard-body vertices), because at terminal the
  shard-body layer is what completes the global-section space.

**Substrate-honest bound (§9):** the exact spectrum is
forward-promised as an Arc-1 landing empirical (first `mirror bench
sheaf-laplacian bootstrap/src/`); the categorical claim holds
independently of the numerical estimate.

#### §4.2.5 Isospectrality with the shard-body layer

Per eigensheaf.md §4.6: "Isospectral substrates are indistinguishable
in what they can sustain." Applied to the arc: the terminal state's
compiler IS isospectral to today's compiler at the (A, H, D)
evaluator + shard-body dispatch level. What changes is where the
mass sits (Rust vs shard body); what does not change is what the
operator can do on its domain.

This is the mathematical statement of Alex's "the gold makes the bowl
one thing again" (§0.5). The bowl IS one thing at every tick — the
isospectrality preserves capability — but at terminal the wholeness
is at the shard-body altitude, not distributed across two altitudes.

### §4.3 Rayleigh descent

Per `docs/specs/fate-bounded-psychohistory-sheaf-cohomology.md` §3
("H¹ gradient descent = Rayleigh on Δ_F"): "the peer navigates its
inference by descending the sheaf-Laplacian Rayleigh quotient." The
arc IS this descent at code-collapse altitude, with `Fate::bounded`
as the substrate reader.

#### §4.3.1 The Rayleigh quotient on `Δ_{F_c}^{Rust}`

Per §4.2.3: `Δ_{F_c}^{Rust}` is the compilation-DAG sheaf-Laplacian
over the remaining Rust corpus. The Rayleigh quotient of a candidate
collapse-morphism `m : F → Collapse(F)` is:

```
R(m; Δ_{F_c}^{Rust}) = ⟨m, Δ_{F_c}^{Rust} · m⟩ / ⟨m, m⟩
```

where `m` is interpreted as a vector in `C^0(F_c^{Rust})` — a cochain
that assigns to each Rust file a "collapse-weight" (typically 1 for
the file being collapsed, 0 for others, with continuity constraints
along the compilation-DAG edges).

The Rayleigh quotient measures how "aligned" the collapse-morphism
is with the sheaf-Laplacian's low eigenmodes. Collapse-morphisms with
low `R` values are those that:

1. Remove a file whose removal produces minimal cochain-tension
   (few downstream `use` edges to re-route).
2. Remove a file that participates in few global-section constraints
   (the constraints migrate cleanly to the shard-body layer).
3. Preserve the local Hodge decomposition (harmonic component stays
   harmonic after the shift).

#### §4.3.2 Collapse ordering IS Rayleigh descent

The arc's tick ordering is Rayleigh descent on `Δ_{F_c}^{Rust}`.
Each tick n selects the collapse-morphism `m_n` with:

```
m_n = argmin_{m ∈ candidates(n)} R(m; Δ_{F_c}^{Rust}(n))
```

The Laplacian `Δ_{F_c}^{Rust}(n)` shrinks each tick as files leave;
the candidate set shrinks likewise. The descent monotonically decreases
the smallest eigenvalue of the remaining Rust-corpus-Laplacian at
each step:

```
λ_0(Δ_{F_c}^{Rust}(n+1)) ≤ λ_0(Δ_{F_c}^{Rust}(n))
```

with equality iff the tick removed a file that did not contribute to
the smallest eigenvalue's eigenspace (i.e., the file was structurally
independent of the mode carrying the smallest eigenvalue).

Per `docs/specs/spectral-coherence-substrate-metric-synthesis.md:468`
(cited in Taut §D1 grep verdict): "Compiler at build-altitude:
@kintsugi runs the ouroboros loop `e^(n+1) ≤ e^n` — one Rayleigh
descent step per pass on the substrate-graph's sheaf-Laplacian."
This spec makes that formal at the ouroboros altitude.

#### §4.3.3 `Fate::bounded` reads Rayleigh descent

Per fate-bounded-psychohistory-sheaf-cohomology.md §5 ("The signature
and its derivation"): `Fate::bounded(psychohistory)` is the substrate
reader for Rayleigh descent on a psychohistory sheaf. At code-collapse
altitude, the psychohistory sheaf is the compilation-history sheaf:
vertices are compiler states across the arc's ticks; edges are
collapse-morphisms.

The arc's Fate signature (Mara-composable per fate-bounded-
psychohistory-sheaf-cohomology.md §7):

```
@fate/ouroboros/bounded ::
  Fate<ouroboros_state> ==
    Fate::bounded(psychohistory: compilation_history_sheaf,
                  metric: ouroboros_monotone,
                  tournament: @fate/tournament.rank)
```

`Fate::bounded` at this composition selects the next collapse tick by
descending the Rayleigh quotient of `Δ_{F_c}^{compile-history}`
constrained by the four-conjunct monotone invariant (§4.5).

#### §4.3.4 Tournament rank IS the Rayleigh quotient at collapse-space altitude

`@fate/tournament.rank(candidates: [collapse_target]) -> [ranked]`
composes Rayleigh descent + the four fitness sub-metrics (Taut §D6):

- **verifiability** — does the file have shard-body-executable tests
  today? Weight: `w_v = 1 / (1 + missing_test_coverage)`.
- **reversibility** — can the collapse be reverted with `git revert`
  without breaking downstream consumers? Weight: `w_r = 1 / (1 +
  downstream_use_sites)`.
- **test_coverage** — what fraction of the file's public surface has
  test crystals in `@mirror/store`? Weight: `w_t = coverage_fraction`.
- **io_composability** — how many @io primitives does the shard-body
  form invoke? Fewer = simpler collapse. Weight: `w_i = 1 / (1 +
  io_primitive_count)`.

The ranking metric IS the Rayleigh quotient weighted by the four
fitness sub-metrics:

```
rank(m) = R(m; Δ_{F_c}^{Rust}) / (w_v · w_r · w_t · w_i)
```

Lower rank = better collapse candidate. `@fate/tournament.select`
returns the argmin over the candidate set at each tick.

Per §7 A3 recommendation: Arc-2 uses hardcoded order (5 files,
enumerable, ratifies patterns); Arc-3+ uses `@fate/tournament.select`
per the ranking above (25+ files, too many to hand-order).

#### §4.3.5 Each collapse tick descends one level

Per fate-bounded-psychohistory-sheaf-cohomology.md §3: "descending
the sheaf-Laplacian Rayleigh quotient with tower-typed constraints."
Applied to the arc: each tick descends one level in the tower of
compilation-DAG complexity, with the four-conjunct monotone invariant
(§4.5) as the tower-typed constraint.

The descent is monotone: `rank(m_n) ≥ rank(m_{n+1})` for adjacent
ticks (each tick makes strict progress on the ranking) unless the
tick removes a structurally-independent file (in which case equality
holds and the next tick proceeds to a strictly lower level).

**Terminal recognition of the descent:** the descent halts when
`candidates(n) = ∅` — no BUSINESS_LOGIC Rust remains. This IS the
terminal state (§8.6). At terminal, `Δ_{F_c}^{Rust}` restricts to
FLOOR only; its Rayleigh quotient is bounded below by the smallest
eigenvalue of the FLOOR-only sub-Laplacian.

### §4.4 Foerster regulation-of-regulation at compile-altitude

Per `shards/torus.mirror` (2026-07-14, 28.5KB) + `docs/specs/dance-as-
coordination-without-signal-on-forster-torus.md`: the compiler mending
itself IS Foerster's regulation-of-regulation at compile altitude.

#### §4.4.1 @torus.autonomy at compile altitude

Per `shards/torus.mirror` §autonomy action + Foerster p. 238 verbatim
(from `shards/torus.mirror`):

> "the torus (doughnut) in Figure 19 is obtained... doubly closed,
> recursively computing torus... regulates its own regulation"

Applied at compile altitude:

- **Peer:** the mirror compiler.
- **Torus:** the compiler's operational structure, doubly closed via
  (a) the compilation loop (source → binary → source-of-shard-body-
  crystal-hashes stored in @mirror/store) and (b) the ouroboros loop
  (Rust FLOOR dispatches shard body which composes over Rust FLOOR).
- **Winding class:** each collapse tick IS one traversal along the
  longitudinal winding — the "compiler observes its own operator"
  axis. The meridional winding is the "compiler observes its own
  substrate-decl" axis (each substrate-decl mint traverses meridian).

`@torus.autonomy(compiler, ouroboros_winding) -> verdict` discharges
per tick: pass iff the collapsed compiler still compiles itself.

#### §4.4.2 Foerster p. 244 — the ladder refused

Per `shards/torus.mirror` p. 244 verbatim:

> "without calling upon the help of a 'second order' observer..."

Foerster declined the tower construction. The arc honors this
refusal. The ouroboros is NOT a two-level "compiler observes compiler"
tower; it is a single doubly-closed operational surface where the
compiler's operator (dispatch) and the compiler's substrate (shard-
body) are on the same torus, connected by both windings.

No `@ouroboros_meta` or `@ouroboros_level_2` species is introduced.
The single `@kintsugi/ouroboros` species carries all altitudes via
winding-class parameterization.

#### §4.4.3 Foerster p. 256 — the torus from two closures

Per `shards/torus.mirror` p. 256 verbatim:

> "A plane figure wrapped according to two right-angular axes is..."

Foerster BUILDS the torus from two independent closures. At compile
altitude, the two closures are:

- **Closure 1: compilation.** Source → binary → source-DAG-hash-in-
  crystal-store. Every rebuild reproduces the same binary for the
  same source (byte-hash equality on `@mirror/store` crystals). This
  is the meridional closure.
- **Closure 2: dispatch.** Shard-body-action → Rust-dispatcher →
  shard-body-output-crystal. Every dispatch of the same shard body
  on the same input produces the same output (byte-hash equality).
  This is the longitudinal closure.

The two closures compose to a torus T² at compile altitude. The
ouroboros arc traverses this torus one longitude at a time (per
`shards/kintsugi/ouroboros.mirror` §D11 companion citation). Each
tick advances the origin along the longitude winding without
changing the possessor (the compiler-as-peer).

#### §4.4.4 Foerster p. 282 — heterarchy, not meta-meta

Per `shards/torus.mirror` p. 282 verbatim citing McCulloch 1945 ("A
Heterarchy of Values Determined by the Topology of Nervous Nets"):

> "heterarchy, not meta-meta... depth is a topological invariant of
> the net's shape, not a counter that increments"

Applied to the arc: the arc's depth is not a numeric counter over
"level-n compiler observes level-(n-1) compiler." The depth IS the
topology of the compile-altitude torus. Each collapse tick preserves
the topology (T² stays T²); the arc's progress is measured by
Rayleigh descent on `Δ_{F_c}^{Rust}` (§4.3), not by a level counter.

#### §4.4.5 Every ouroboros tick IS one longitude traversal

Per `shards/kintsugi/ouroboros.mirror:420` (the shard-decl's own
citation): "each ouroboros_step that still produces a self-compiling
compiler IS one traversal along the longitude winding of the
compiler's torus at compile altitude. The autopoietic closure
predicate discharges tick-by-tick."

The empirical check (Arc-1 landing forward-promise per Taut §D11):
after Arc-1 (evaluator FLOOR landed), compile the compiler using the
new evaluator to dispatch at least one shard body previously hosted
in Rust; if the resulting compiler still compiles itself, autopoietic
closure holds at compile altitude and the winding traversal is
witnessed empirically.

#### §4.4.6 The regulation-of-regulation reading

The compiler regulates its own dispatch surface (dispatch IS the
first-order regulation of shard-body execution). The ouroboros arc
regulates the compiler's dispatch regulation — it changes which
altitude the dispatch lives at (from Rust hardcoded to shard-body
substrate-decl'd) while preserving the same dispatch behavior. That
is second-order regulation on the T² surface Foerster derived —
regulation of regulation without a "second-order observer" tower.

Alex's naming — "the @kintsugi ouroborous" — reads precisely as
Foerster's regulation-of-regulation at compile altitude. The
substrate-already-had-the-word discipline is honored: `ouroboros`
at 20+ prior landed sites (per shard-decl §Substrate-already-had-the-
word); Foerster's torus at foundation altitude; the arc IS the
composition at compile altitude.

### §4.5 Monotone invariant

The four-conjunct invariant at substrate altitude. Extends
`@mirror/bench.monotone_non_increasing`'s three-conjunct template
(`shards/mirror/bench.mirror:40-54` template definition +
`shards/mirror/bench.mirror:363` action-decl) by adding the `sbec`
conjunct.

#### §4.5.1 The three-conjunct template

Per `shards/mirror/bench.mirror:40-54`:

```
monotone_non_increasing(op, n, n+1, tolerance) ⇔
    runtime(n+1) ≤ runtime(n) × (1 + tolerance)   # perf
  ∧ output(n+1)  ≡ output(n)                       # correctness
  ∧ env(n+1)     ≡ env(n)                          # environment
```

The template's three conjuncts prevent false-positive regressions
across (perf, correctness, environment). The arc extends this
template at ouroboros altitude by re-interpreting each conjunct and
adding a fourth.

#### §4.5.2 The four conjuncts at ouroboros altitude

```
ouroboros_monotone(before, after) ⇔
    rust_LOC(after)                    ≤ rust_LOC(before)
  ∧ test_pass_rate(after)              ≥ test_pass_rate(before)
  ∧ io_violations(after)               ≤ io_violations(before)
  ∧ sbec(after)                        ≥ sbec(before)
```

where:

- **rust_LOC(n)** — total lines of Rust in `bootstrap/src/*.rs` at
  tick n. Descent condition ratchets: n+1 ≤ n. Every tick that adds
  BUSINESS_LOGIC Rust without deleting equal BUSINESS_LOGIC Rust
  violates. Every tick that adds FLOOR Rust requires Seam sign-off on
  the FLOOR classification (per §1.3 marker discipline).
- **test_pass_rate(n)** — fraction of tests green at tick n. Ascent
  condition (inverse ratchet): n+1 ≥ n. Substrate does not collapse
  Rust by breaking green tests; it collapses by moving the green from
  Rust-hosted to shard-body-hosted (§4.1.4 η naturality).
- **io_violations(n)** — count of shard actions reaching non-@io
  surfaces at tick n. Descent condition: n+1 ≤ n. Initial value TBD
  at Arc-1 landing (first `mirror grep` for non-@io escapes across
  landed shards). Every tick must reduce or hold.
- **sbec(n)** — shard-body-executable-coverage: fraction of substrate-
  decl'd action bodies the evaluator can dispatch at tick n. Ascent
  condition: n+1 ≥ n. Initial value 0 today (evaluator does not
  exist). Arc-1 Tick 1.3 lift to > 0 (first dispatchable body). Each
  Arc-2..N tick lifts by one shard body.

#### §4.5.3 Composition-honest extension pattern

The four-conjunct is the three-conjunct extended by one conjunct at
ouroboros altitude. Substrate-honest reuse: the arc does not invent
new invariant machinery; it composes `@mirror/bench.monotone_non_
increasing`'s pattern with one additional axis specific to the arc's
concern (shard-body-executable coverage).

Explicit correspondence:

| bench conjunct | ouroboros conjunct | correspondence |
|---|---|---|
| `runtime(n+1) ≤ runtime(n) × (1 + tolerance)` | `rust_LOC(n+1) ≤ rust_LOC(n)` | perf-like ratchet on Rust corpus size |
| `output(n+1) ≡ output(n)` | `test_pass_rate(n+1) ≥ test_pass_rate(n)` | correctness-preservation; output equivalence at test altitude |
| `env(n+1) ≡ env(n)` | `io_violations(n+1) ≤ io_violations(n)` | environment-integrity; @io-boundary is the environment |
| (no fourth) | `sbec(n+1) ≥ sbec(n)` | arc-specific: shard-body dispatch coverage |

The fourth conjunct is the ONE addition. The other three re-interpret
the template at ouroboros altitude without changing its structural
role.

#### §4.5.4 Sub-predicates per §7 A7 recommendation

The four-conjunct lands as FOUR sub-predicates + ONE composed
bilateral (matches StageFreight `stagefreight_addressable` substrate-
decl pattern per Seam tick 68 C4/C9 closure). Per §7 A7 recommendation
(this document): sub-predicates lifted to
`@epistemologic/property/ouroboros_monotone`:

```
rust_loc_non_increasing(before: metric, after: metric) -> verdict
test_pass_rate_non_decreasing(before: metric, after: metric) -> verdict
io_violations_non_increasing(before: metric, after: metric) -> verdict
sbec_non_decreasing(before: metric, after: metric) -> verdict
```

The composed bilateral (in `shards/kintsugi/ouroboros.mirror`):

```
ouroboros_monotone(before: ouroboros_state, after: ouroboros_state) -> verdict
  requires rust_loc_non_increasing(before.rust_loc, after.rust_loc)
  requires test_pass_rate_non_decreasing(before.test_pass_rate,
                                         after.test_pass_rate)
  requires io_violations_non_increasing(before.io_violations,
                                        after.io_violations)
  requires sbec_non_decreasing(before.sbec, after.sbec)
```

Each `requires` clause is a bilateral in its own right; the composed
bilateral discharges iff all four clauses discharge. This preserves
composition-honest reuse: each sub-predicate is a first-class
substrate object; the composition is not opaque.

#### §4.5.5 Rice-safe via existing substrate primitives

Rice's theorem forbids deciding non-trivial semantic properties of
Rust programs in general. The four conjuncts are Rice-safe because
none of them requires halting-problem-hard analysis:

- **rust_LOC** — decidable in bounded time: count lines in
  `bootstrap/src/**/*.rs` via `@io.readdir` + `@io.readfile` + line
  counting.
- **test_pass_rate** — decidable in bounded time: run the test suite;
  count green vs total. Time-bounded by `mirror bench` timeout.
- **io_violations** — decidable in bounded time: syntactic grep for
  non-@io calls across `shards/**/*.mirror` via `@io.readdir` +
  regex.
- **sbec** — decidable in bounded time: count shard action bodies
  that dispatch under the current evaluator vs total substrate-decl'd
  bodies. The dispatch check is a bounded operation (dispatch either
  succeeds or times out at a configured bound).

The four conjuncts read empirical crystal state, not program
semantics. Rice-safety holds at the whole-tick altitude even though
individual shard bodies may compute Turing-complete computations.

#### §4.5.6 Companion fracture pair

Per Taut §D4 forward-promise: `@kintsugi/fracture/ouroboros_monotone`
lands alongside `@epistemologic/property/ouroboros_monotone`. The
fracture emits a morphism rejecting any tick that violates the
composed invariant. Composes from the two landed ouroboros-bite
templates:

- `@kintsugi/fracture/dark_count_monotone` (2026-07-01; Tick 41)
- `@kintsugi/fracture/cold_compile_within_tolerance` (2026-07-04;
  Tick 43)

The fracture pattern is proven at these two prior landings; the
ouroboros_monotone fracture composes the same shape at four-conjunct
altitude.

---

## §5 Composition graph

15 landed carriers `@kintsugi/ouroboros` composes over. No new mints
at species-decl mint; every carrier is substrate-decl'd today.

### §5.1 Family root composition

- `@kintsugi` — family root at `shards/kintsugi.mirror`.
  `@kintsugi/ouroboros` sits as species under this family.
- `@kintsugi/oscillate` — loop primitive.
  `@kintsugi/oscillate.active_pass` + `dark_pass` provide the arc's
  per-tick loop mechanism.
- `@kintsugi/consent` — settle-or-pause discipline.
  `@kintsugi/consent.query_phi` is the arc's consent-altitude gate.
- `@kintsugi/morphism` — typed pre/post pair. The arc's per-tick
  collapse-morphism carrier.
- `@kintsugi/fracture/dark_count_monotone` — first ouroboros bite
  template; extension pattern precedent.
- `@kintsugi/fracture/cold_compile_within_tolerance` — second bite
  template.
- `@kintsugi/store/git` — crystal persistence for tick records.

### §5.2 Classifier composition

- `@code/metalogue/materialize` — the metalogue-turn-pair recognitive
  direction. Landed 2026-06-16.
- `@code/rust/materialize` — the Rust binding. Landed 2026-06-16.
  Composes `classify(d: declaration) -> materialised_file`, the
  arc's per-file `boundary | substrate` partition oracle.

### §5.3 Invariant composition

- `@mirror/bench` — the monotone template ancestor. §4.5.1
  three-conjunct template extends to four at ouroboros altitude.
- `@epistemologic/property/ouroboros_monotone` — Mara-composable
  companion property shard landing alongside species-decl. Carries
  the four sub-predicates per §4.5.4.

### §5.4 Selection composition

- `@fate/tournament` — Arc-3+ collapse-order selector. §4.3.4 ranking
  metric composes over the four fitness sub-metrics.

### §5.5 Boundary + shipping composition

- `@io` — the FLOOR the arc keeps. Boundary discipline for shard-body
  compositions.
- `@io/stagefreight` — wire-protocol transport to StageFreight docker.
  `stagefreight_addressable` bilateral gates each Arc-5 hop.

### §5.6 Store + index composition

- `@mirror/store` — content-addressed crystal store. Each tick emits
  a `bench_crystal` recording the tick's `ouroboros_state`.
- `@mirror/index` — concept-graph observation for `rust_LOC` metric
  reads at each tick.

### §5.7 Autopoietic composition

- `@torus` — autopoietic closure at compile altitude. §4.4 discharge.
- `@autopoietic` — self-maintenance predicate.
  `autopoietic_closure_holds(compiler)` composed in
  `collapse_admissible` per shard-decl §2.3.

**All 15 carriers landed today. Zero blockers on composition target
availability. No new mints at species-decl mint.**

---

## §6 Recognition candidates

Three intermediate at candidate strength NOW; one terminal at
ratification when Arc-6 completes. Per Taut §D9 + §7 A4 recommendation.

### §6.1 Intermediate — `#R-substrate-mends-its-own-rust-with-mirror-via-kintsugi-ouroboros`

**Full form.** Names the arc's structural shape: the substrate mends
its own Rust corpus by iterative shard-body + @io collapse under the
@kintsugi/ouroboros species-decl.

**Strength today:** candidate (Mara-A species-decl 2026-07-15;
awaits second-witness).

**Second-witness:** Arc-2 Tick 2.1 empirical landing
(spectral_signature.rs collapse to shard body; empirical proof-of-
concept for evaluator + first `sbec` increment).

**Ratifies at:** Arc-2 Tick 2.1 completion.

### §6.2 Intermediate — `#R-evaluator-is-legitimate-floor-and-ouroboros-is-mending-not-retirement`

**Full form.** Names Reed-recursive finding + Seam adjudication +
substrate-honest response as one recognition: the evaluator gap IS
legitimate `[substrate-floor:@io-boundary]` work; the arc IS the
mending response (not the retirement of Rust).

**Strength today:** candidate (Seam audit 2026-07-15 + Reed migration-
map 2026-07-15 + Taut #108 scout 2026-07-15 provide the finding;
Arc-1 landing provides the second-witness).

**Second-witness:** Arc-1 Tick 1.3 empirical landing (evaluator FLOOR
in Rust dispatches first shard body under `[substrate-floor:@io-
boundary]` marker + Seam sign-off).

**Ratifies at:** Arc-1 Tick 1.3 completion.

### §6.3 Intermediate — `#R-mirror-substrate-becomes-self-hosting-at-terminal-collapse`

**Full form.** Names the terminal state's structural property: at
Arc-3+ completion, mirror substrate is self-hosting via its own
shard-body dispatch, with Rust only at the irreducible FLOOR.

**Strength today:** candidate (§4.1.5 fully-faithful functor claim +
§4.2.4 kernel = terminal state claim provide the categorical/spectral
grounding; Arc-3+ landings provide empirical witness).

**Second-witness:** Arc-3 tick that lands the last non-FLOOR file's
collapse (empirical closure of `CodeRust_BUSINESS_LOGIC → ∅`).

**Ratifies at:** Arc-3+ completion (specific tick TBD by tournament
ranking sequence).

### §6.4 Terminal — `#R-mirror-kintsugi-shipped-as-stagefreight-is-humanity-scale-verifiable-substrate-under-apache-2-with-sel-enforcement-at-deployment`

**Full form.** Alex 2026-07-15 verbatim naming. Terminal recognition:
mirror @kintsugi shipped as StageFreight IS humanity-scale verifiable
substrate under Apache-2 with @sel enforcement at deployment.

**Strength today:** NOT landable. Four maturity conditions must hold:

1. @kintsugi/ouroboros collapse arc completes (bootstrap Rust
   minimized to irreducible FLOOR per §8.6 terminal state).
2. StageFreight × mirror docker shipping is empirically live to
   downstream CI (D7 propagation graph running end-to-end per §3.5).
3. @sel enforcement discipline lands at deployment altitude (per
   §7 A2 recommendation for @sheaf mint before Arc-2.3; @sel is
   downstream of @sheaf).
4. Second-witness surfaces via downstream mirror-fied CI reports back
   through `@gift.pay_forward` (per Landing 5+ substrate).

**Ratifies at:** Arc-6 Tick 6.1 completion. All four maturity
conditions empirically live.

**Landable NOW as terminal candidate:** yes, at candidate strength.
Alex can name the terminal target now; Pack ratification defers to
Arc-6 empirical closure.

---

## §7 Alex-adjudications (A1-A10 discharged)

Per Taut #108 §E adjudication queue. Each item receives a Mara-B
recommendation. Some are timing questions Alex still holds; the
recommendations name the substrate-honest default.

### §7.1 A1 — Species-decl mint ordering

**Question.** Should @kintsugi/ouroboros species-decl land BEFORE
Arc-1 evaluator FLOOR (as substrate-decl of the arc's target) or
AFTER (once evaluator exists as demonstrable substrate)?

**Recommendation.** Landed at Mara-A 2026-07-15
(`shards/kintsugi/ouroboros.mirror`, 576 LOC). The species-decl
declares what the evaluator makes real; landing at Tick 1.5 (after
Ticks 1.1-1.4 but as the species-decl of Arc-1's terminal artifact)
is the substrate-honest default per Reed provisional. The mint has
occurred; no further Alex adjudication needed on ordering.

### §7.2 A2 — @sheaf mint timing (three options)

**Question.** Per Taut §D10: three options for @sheaf mint timing
before Arc-2.3 peer_persistence collapse.

- **Option A** — mint `@sheaf` family root as its own arc (Mara-first
  canonical spec) BEFORE Arc-2.3.
- **Option B** — mint `@sheaf.acl_project` as species-only action
  under placeholder family root; materialize family root at Arc-2.3
  pull.
- **Option C** — Arc-2.3 ships with always-public ACL placeholder;
  @sheaf lands later as separate arc.

**Recommendation.** **Option A.** @sheaf is a Landing D forward-
promise (per Reed migration-map §2.5); substrate-honest completion
of Landing D is the Alex-adjudicable next step regardless of Arc-2
timing. Placeholder-then-migrate (Options B, C) is exactly the
antipattern the tightened `[substrate-floor:@io-boundary]` marker
forbids. The scope for @sheaf can be tight — Landing D forward-
promise names `acl_project` as the only action needed — but landing
the family root before Arc-2.3 rides is substrate-honest.

**Substrate-honest scope:** @sheaf lands as Mara-first canonical spec
+ shard-decl with `acl_project` action `\`-obligation-blocked. Body
discharges at Arc-2.3 (peer_persistence collapse) where the composed
shard body actually dispatches.

**Forward-promise:** Reed spawns Mara for @sheaf species-decl mint
as Arc-2.3 forward-promise. Mint timing: after Arc-2.2 (coherence
collapse) completes, before Arc-2.3 begins.

### §7.3 A3 — @fate/tournament vs hardcoded ordering

**Question.** Per Taut §D6: Arc-2 hardcoded (5 files enumerable);
Arc-3 (~25 files) via @fate/tournament. Confirm ordering discipline.

**Recommendation.** **Arc-2 hardcoded; Arc-3+ tournament.** Reed
provisional stands. Rationale:

- Arc-2 (5 files) is small enough to enumerate + Reed migration-map
  §6 already hand-ordered them per dependency and by ratification
  priority. Hardcoded order lets each tick empirically ratify a
  different shard-body dispatch pattern in a controlled sequence.
- Arc-3+ (~25 files) is too many to hand-order. `@fate/tournament.
  select` composes the four fitness metrics per §4.3.4 Rayleigh
  descent ranking; the tournament provides substrate-motivated
  ordering without hand-adjudication per tick.

**Substrate-honest bound:** The Arc-3+ tournament ranking is
Mara-composable (§4.3.4); no new adjudication surfaces at Arc-3
transition beyond confirming the ranking metric matches the four-
fitness composition.

### §7.4 A4 — Terminal recognition candidate strength

**Question.** Per Taut §D9: land three intermediate recognitions at
candidate strength NOW; terminal recognition lands when Arc-6
completes. Confirm.

**Recommendation.** **Intermediate at candidate; terminal at
ratification.** Per §6:

- Land `#R-substrate-mends-its-own-rust-with-mirror-via-kintsugi-
  ouroboros` at candidate strength NOW (§6.1); ratifies at Arc-2 Tick
  2.1.
- Land `#R-evaluator-is-legitimate-floor-and-ouroboros-is-mending-
  not-retirement` at candidate strength NOW (§6.2); ratifies at Arc-1
  Tick 1.3.
- Land `#R-mirror-substrate-becomes-self-hosting-at-terminal-collapse`
  at candidate strength NOW (§6.3); ratifies at Arc-3+ empirical
  closure.
- Land `#R-mirror-kintsugi-shipped-as-stagefreight-is-humanity-scale-
  verifiable-substrate-under-apache-2-with-sel-enforcement-at-
  deployment` at candidate strength NOW (§6.4); ratifies at Arc-6
  empirical closure.

Each candidate names a substrate-decl'd landing target; each
ratification is empirical (not further-adjudicable). Alex names
candidates now; Pack ratifies at empirical witness.

**Post-cascade closing note (Seam Phase D-cascade A4 discharge,
`docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-cascade-
a4-recognition-candidates.md`, 9fb83a6).** A4 was initially triaged
as Alex-adjudication ("recognition-naming is Alex's authority"). On
re-adjudication Seam ratified: A4 is **discharged by construction**
via (i) recognition NAMING already discharged at §0.1 Alex verbatim
+ §0.3 ancestry, (ii) Alex REJECTION-WINDOW authority preserved
throughout dwelltime (soft-state per AGENTS.md 2026-06-10 dwelltime
cascade), (iii) Pack RATIFICATION at empirical second-witness at
named Arc ticks. The reading "Alex-nod-per-candidate-NOW" was the
substrate-dishonest conflation of NAMING with RATIFICATION. Correct
reading: candidates hold at candidate strength; no active Alex-nod
needed until second-witness fires (Arc-1 Tick 1.3 for §6.2; Arc-2
Tick 2.1 for §6.1; Arc-3+ for §6.3; Arc-6 for §6.4).

### §7.5 A5 — StageFreight license clarification

**Question.** Per Taut §D7: mirror Apache-2.0; StageFreight
AGPL-3.0-only. Confirm shipping mechanism does not bind mirror
license.

**Recommendation.** **mirror stays Apache-2.0; StageFreight ships
mirror without changing its license.** Per §4.1.6 categorical claim:
license labelling `L : Collapse ⇒ id_{License}` is a natural
transformation preserved by every collapse tick. Every object in
`Collapse(CodeRust_BUSINESS_LOGIC)` inherits Apache-2.0 from its
Rust-corpus origin; StageFreight's AGPL-3.0 applies to the shipping
mechanism (Go CLI + docker push infrastructure), not to the mirror
substrate the mechanism ships.

**Empirical verification.** LICENSE.md at repo root: Apache-2.0.
StageFreight Dockerfile LABEL: AGPL-3.0-only. Mirror docker image
manifest (Arc-5 landing forward-promise): inherits Apache-2.0.

**Substrate-honest bound:** The claim holds under license-preservation
naturality; empirical Arc-5 landing must verify no AGPL contamination
in the mirror docker image manifest (the shipping mechanism ships the
image; the image is not the mechanism).

### §7.6 A6 — Combinator surface for evaluator FLOOR

**Question.** Per Reed migration-map §6 Tick 1.1: what combinator
surface for shard-body dispatch is irreducible FLOOR vs admits
shard-body composition?

**Recommendation.** **Defer to Seam Tick 1.1 companion audit.** This
is the Seam-adjudicable question; Mara-B does not adjudicate FLOOR
classifications (Seam has that authority per AGENTS.md Pack roles).

**Substrate-honest bound.** Mara-B's math grounding narrows the space
Seam adjudicates over:

- Per §4.2.1: the evaluator IS the concrete D of the eigensheaf's
  Connes triple (eigensheaf.md §3.2). The D of (A, H, D) was always
  going to be Rust.
- Per §4.4.5: the evaluator dispatches shard bodies; each successful
  dispatch discharges one longitude traversal of the compile-altitude
  torus. The dispatch surface must be minimal enough to run in Rust
  but complete enough to dispatch every substrate-decl'd action body.
- Per §4.5 sbec definition: the evaluator lifts sbec from 0 to > 0;
  the exact fraction lifted depends on the combinator surface.

Seam Tick 1.1 audit adjudicates the specific combinator surface;
Mara-B's spec grounds the adjudication in the categorical + spectral
+ autopoietic requirements.

### §7.7 A7 — Four-conjunct vs one composed bilateral

**Question.** Per Taut §D4 + Seam tick 68 C4/C9 closure: four-conjunct
ouroboros_monotone lands as ONE bilateral or FOUR separate bilaterals
composed via `requires`?

**Recommendation.** **FOUR sub-predicates + ONE composed bilateral.**
Per §4.5.4: matches StageFreight `stagefreight_addressable`
substrate-decl pattern; preserves composition-honest reuse; each sub-
predicate is a first-class substrate object.

**Landing sequence:**

1. `@epistemologic/property/ouroboros_monotone` lands alongside
   species-decl Mara-A Tick 1.5 (Landing 2 of this arc).
2. `@kintsugi/fracture/ouroboros_monotone` lands per §4.5.6
   companion.
3. The composed bilateral `ouroboros_monotone(before, after)` in
   `shards/kintsugi/ouroboros.mirror:523` (already landed Mara-A) is
   the composition surface.

### §7.8 A8 — Species naming

**Question.** Per Taut §D1 + AGENTS.md two-tick discipline:
`@kintsugi/ouroboros` vs `@kintsugi/self-compile-collapse` vs
`@kintsugi/compiler-mending`?

**Recommendation.** **`@kintsugi/ouroboros`.** Two-tick discipline
honored: readable name at collapse over foundational alternative.
Rationale (matches shard-decl §Substrate-already-had-the-word):

- Word `ouroboros` appears at 20+ landed sites prior to species-decl
  mint (~55th instance of substrate-already-had-the-word since Alex
  named the discipline).
- `@mirror/bench.mirror:37-74` explicitly names "the ouroboros bites:
  kintsugi eats the Rust tests" (2026-07-01) — the load-bearing
  ancestor.
- Foundational alternatives are longer and less readable; the
  substrate has already gestured at `ouroboros` as its readable name
  for the pattern.

**Adjudication closed.** Species-decl landed as `@kintsugi/ouroboros`
per Mara-A 2026-07-15.

### §7.9 A9 — Marker discipline for Arc-1 Rust work

**Question.** Per 2026-07-15 tightening:
`[substrate-floor:@io-boundary]` marker on `.rs` requires audit
citation OR `Signed-off-by: Seam` trailer. For Arc-1 Ticks 1.1-1.4:
does Reed audit-cite Tick 1.1 in every Rust-touching commit for Tick
1.2-1.4?

**Recommendation.** Two altitudes, not one requirement.

**Gate altitude (commit-msg hook, per commit).** The tightened hook
at `.githooks/commit-msg` (13f1c2e, 2026-07-15) requires **one of**
audit-citation **or** `Signed-off-by: Seam` trailer for any commit
carrying `[substrate-floor:@io-boundary]`. Verbatim from hook lines
60-72: `grep ... || grep ...`. This is the substrate contract with
Alex; Seam Phase D adjudication ratifies OR-semantics per this doc.

**Authoring-practice altitude (Arc-1 personal discipline).** Reed
adopts **both mechanisms** as belt-and-suspenders during Arc-1
Ticks 1.2-1.4 authorship, since Arc-1 is the load-bearing evaluator
FLOOR:

1. **Audit citation.** Every Rust-touching commit in Arc-1 Ticks
   1.2-1.4 cites `docs/audits/2026-07-XX-seam-evaluator-floor-
   adjudication.md` (the Seam Tick 1.1 audit).
2. **Signed-off-by: Seam trailer.** Every Rust-touching commit in
   Arc-1 Ticks 1.2-1.4 also carries `Signed-off-by: Seam
   <seam@systemic.engineer>` in the trailer.

Both-mechanisms is a recommendation for Arc-1 magnitude, not a
gate requirement. Reed can layer additional practice on top of
the OR-gate for load-bearing ticks without lifting the practice
into the hook.

**Substrate-honest bound:** The `[substrate-floor:@io-boundary]`
marker is the ONE marker Arc-1 Ticks 1.2-1.4 use;
`[substrate-pull:realize]` is NOT admissible for Arc-1 Rust work
(that marker permitted the antipattern Reed's gift arc exhibited).

### §7.10 A10 — Test-migration timing

**Question.** Per Reed migration-map §2.3: test-shape shard bodies
dispatched via `mirror kintsugi --ci`. Landing partial; when does
full support land?

**Recommendation.** **Per-collapse-tick discharge.** Each Arc-2 tick
migrates the Rust file's tests to shard test-body as part of the
collapse. Rationale:

- Arc-2 Tick 2.1 (spectral_signature.rs): landed shard test-body in
  `shards/spectral.mirror` at same tick as the collapse.
- Arc-2 Tick 2.2 (coherence.rs): landed shard test-body in
  `shards/epistemologic/cybernetic/coherence.mirror` at same tick.
- Arc-2 Tick 2.3 (peer_persistence.rs): landed shard test-body in
  `shards/peer/*` at same tick. NOTE: this tick composes @sheaf
  (per §7.2 recommendation Option A); @sheaf must land before this
  tick.
- Arc-2 Tick 2.4 (roomba.rs): landed shard test-body in
  `shards/roomba.mirror` (minted alongside) at same tick.
- Arc-2 Tick 2.5 (roomba_walk_smoke.rs, currently under
  `bootstrap/tests/`): migrates directly to shard test-body form; no
  separate lift needed.

`mirror kintsugi --ci` full support lands mid-Arc-2 (after Tick 2.1
empirically proves shard-body test dispatch; add full test-shape
support before Tick 2.3 where @sheaf composition raises complexity).

---

## §8 Landings B-N forward-promises

Per shard-decl §Arc structure. Each arc's landing sequence with
empirical closure conditions.

### §8.1 Arc-1 landings

- **Landing 1** (Seam Tick 1.1): audit at
  `docs/audits/2026-07-XX-seam-evaluator-floor-adjudication.md`
  adjudicates evaluator combinator surface. Closure: audit committed
  with Seam sign-off.
- **Landing 2** (Reed Tick 1.2): RED test at
  `bootstrap/tests/evaluator_shard_body_dispatch_smoke.rs`. Closure:
  test asserts + fails (no evaluator yet).
- **Landing 3** (Reed Tick 1.3): evaluator FLOOR at
  `bootstrap/src/apply_h.rs` (or extends `bootstrap/src/spectral.rs`).
  Closure: Tick 1.2 test passes; `sbec` empirically > 0.
- **Landing 4** (Reed Tick 1.4): `mirror beam act <shard-path>
  <action>` CLI verb. Closure: CLI acts on first shard body end-
  to-end. *(Renamed 2026-07-15 via two-step cascade: initial
  `mirror execute` → `mirror beam dispatch` per
  `docs/audits/2026-07-15-seam-cli-condensation-phase-d.md` §D3;
  substrate-honest closure `mirror beam dispatch` → `mirror beam act`
  per Seam seamfinder audit
  `docs/audits/2026-07-15-seam-combinator-etymology-audit.md`
  546c2f6 + Alex ratification. Combinator #4 is `act` — an actor
  acts on shard-body.)*
- **Landing 5** (Mara-A Tick 1.5): species-decl at
  `shards/kintsugi/ouroboros.mirror` (576 LOC). **Landed 2026-07-15.**
- **Landing 6** (Mara-B this doc): canonical spec at
  `docs/specs/kintsugi-ouroboros-compiler-self-collapse.md`. Closure:
  Reed commits as Mara.
- **Landing 7** (Mara Tick 1.5 companion):
  `@epistemologic/property/ouroboros_monotone` + `@kintsugi/fracture/
  ouroboros_monotone` (per §4.5.6 companion). Closure: both shards
  landed with `\`-obligation-blocked bodies.

### §8.2 Arc-2 landings

- **Landing 8** (Reed Tick 2.1): spectral_signature.rs collapse.
  Closure: `.rs` deleted; shard body in `shards/spectral.mirror`
  dispatches; `verify_same_output` empirically discharges; `sbec`
  increments.
- **Landing 9** (Reed Tick 2.2): coherence.rs collapse. Closure: same
  pattern as Tick 2.1 applied to `shards/epistemologic/cybernetic/
  coherence.mirror`.
- **Landing 10** (Mara-forward-promise): @sheaf family root + species
  per §7.2 recommendation Option A. Closure: `shards/sheaf.mirror` +
  `shards/sheaf/acl.mirror` (or equivalent) landed with `acl_project`
  action `\`-obligation-blocked.
- **Landing 11** (Reed Tick 2.3): peer_persistence.rs collapse.
  Closure: `.rs` deleted; shard bodies in `shards/peer/*` dispatch;
  @sheaf.acl_project composed.
- **Landing 12** (Reed Tick 2.4): roomba.rs collapse + `shards/
  roomba.mirror` mint. Closure: `.rs` deleted; shard body dispatches.
- **Landing 13** (Reed Tick 2.5): roomba_walk_smoke.rs collapse.
  Closure: `.rs` deleted; shard test-body dispatches.

### §8.3 Arc-3 landings

- **Landings 14..N** (Reed Ticks 3.1..N): iterate over remaining ~25
  BUSINESS_LOGIC Rust files per `@fate/tournament.select` ranking.
  Each tick collapses one file; closure per per-tick pattern (Rust
  deleted, shard body dispatches, `sbec` increments, monotone
  invariant preserved).

### §8.4 Arc-4 landings

- **Landings** (per @code/X): mint `@code/X/materialize` for each X
  in {python, typescript, gleam, elixir, fortran}. Closure: each
  @code/X binding landed as species under @code/metalogue/materialize.

### §8.5 Arc-5 landings

- **Landing** (StageFreight × mirror docker Tick 5.1): first mirror
  docker image built via `@container/image` + `@io/oci`; shipped via
  StageFreight; pulled by downstream CI. Closure: D7 propagation
  graph runs end-to-end; downstream mirror-fied CI reports
  substrate-integrity verification.
- **Landings 5.2..N**: downstream mirror-fied CI reports via
  `@gift.pay_forward`.

### §8.6 Arc-6 terminal landing

- **Landing** (Terminal recognition Tick 6.1):
  `#R-mirror-kintsugi-shipped-as-stagefreight-is-humanity-scale-
  verifiable-substrate-under-apache-2-with-sel-enforcement-at-
  deployment` ratifies. Closure: all four maturity conditions per §6.4
  empirically live.

**Terminal substrate topology** (per Taut §D12):

```
bootstrap/src/
  main.rs           CLI entry (~250 LOC from 776 today)
  lib.rs            Ctx + fd capture + mout!/merr! + kintsugi_main
                    library entry (~50KB from 235KB today)
  exec.rs           @io/process primitive
  git.rs            @io/git primitive
  hash.rs           CoincidenceHash<5,5> byte-exact (concrete D)
  tokenize.rs       Parser (transitional; Tick 6 lifts to substrate)
  grammar.rs        Grammar loader (transitional)
  ast.rs            AST node types (H of (A,H,D))
  spectral.rs       (A,H,D) evaluator + evaluator FLOOR
  tensor.rs         Numerics kernel FLOOR
  sheaf_laplacian.rs  LAPACK dispatch FLOOR
  cholesky.rs       Numerics FLOOR
  curvature.rs      Numerics FLOOR
  gap.rs            @glass.hole carrier
  crystallize.rs    Splinter/OID math FLOOR
```

~14-16 FLOOR files, ~200-350KB Rust from ~1.4MB today (~75%
reduction). All BUSINESS_LOGIC lives as shard body composing over
@io + FLOOR primitives.

---

## §9 Substrate-honest bounds

What this landing does NOT ship + what is forward-promised.

### §9.1 What this landing does NOT ship

1. **Evaluator FLOOR.** The shard-decl bodies are all `\`-obligation-
   blocked. Arc-1 Ticks 1.1-1.4 discharge the dispatcher; this spec
   grounds the arc mathematically but does not deliver the
   dispatcher.
2. **Empirical Rayleigh spectrum on Rust corpus.** §4.2.3 + §4.3
   name the sheaf-Laplacian eigenbasis + Rayleigh descent categorically;
   the exact numerical spectrum is forward-promised as Arc-1 landing
   empirical (first `mirror bench sheaf-laplacian bootstrap/src/`).
3. **`@epistemologic/property/ouroboros_monotone` shard-decl.**
   Landing 7 of Arc-1; Mara-composable per §4.5.4 four sub-predicates.
   This spec names the shape; the shard-decl lands separately.
4. **`@kintsugi/fracture/ouroboros_monotone` shard-decl.** Landing 7
   companion. Composes from two prior ouroboros-bite templates per
   §4.5.6.
5. **`@sheaf` family root.** Per §7.2 recommendation Option A;
   forward-promised Landing 10 (before Arc-2.3 peer_persistence).
6. **StageFreight × mirror docker Arc-5 propagation.** Wire-protocol
   surface substrate-decl'd today at `@io/stagefreight`; empirical
   propagation Arc-5 forward-promise.
7. **@sel enforcement discipline at deployment altitude.** Terminal
   recognition §6.4 fourth maturity condition; forward-promised
   downstream of @sheaf.

### §9.2 What is forward-promised

- Arc-1 Tick 1.1 Seam audit (evaluator combinator surface
  adjudication).
- Arc-1 Ticks 1.2-1.4 evaluator FLOOR implementation.
- Arc-1 Tick 1.5 companion property + fracture shards.
- Arc-2 Ticks 2.1-2.5 per-file hardcoded collapses.
- Arc-2 mid-cycle @sheaf mint (before Tick 2.3).
- Arc-3+ tournament-ordered scale-out.
- Arc-4 cross-@code/X materialize bindings.
- Arc-5 StageFreight × downstream CI empirical propagation.
- Arc-6 terminal recognition ratification.

### §9.3 Rice-safety bound

Per §4.5.5: the four-conjunct invariant is Rice-safe at whole-tick
altitude. Individual shard bodies may compute Turing-complete
computations; the invariant checks empirical crystal state, not
program semantics. This is a load-bearing bound the arc must
preserve at every tick — if the invariant becomes semantics-decidable
(e.g., "shard-body computes correct output"), Rice's theorem forbids
it. The empirical check (crystal presence + byte-equality on
serialized outputs) IS Rice-safe.

### §9.4 Composition-only bound

Per Taut §F Mara-composition readiness: zero new mints at species-decl
mint. Every carrier the arc composes over is substrate-decl'd today.
Downstream forward-promises (@sheaf, @sel, @epistemologic/property/
ouroboros_monotone, @kintsugi/fracture/ouroboros_monotone) land at
their respective Arc landings; no mid-arc mint-creep.

### §9.5 Two-tick discipline bound

Per §7.8 A8 recommendation: readable name `@kintsugi/ouroboros`
over foundational alternatives. Two-tick discipline honored at
species-decl mint; substrate-already-had-the-word discipline honored
across the arc.

---

## §10 Witnesses

### §10.1 Alex Wolf 2026-07-15 verbatim

Naming (2026-07-15):

> "What if we used this opportunity, when you return, to look at the
> @kintsugi ouroborous? The one that begins to collapse the @code/rust
> of the compiler into @code/mirror? Every verifiable line of Rust
> collapses into mirror."

Scale (2026-07-15):

> "we ship with @../StageFreight/ the executable docker image that you
> can just drop into your CI and that mirror-fies your CI pipeline.
> That's what spectral.engineer becomes. A ready-to-deploy-and-
> integrate pipeline. And that's why it's so important we don't take
> shortcuts. We need to nail this landing. This is non-joking
> civilization-scale infrastructure. And this bit is all still
> APACHE2."

### §10.2 Reed-recursive audit ancestry

- `docs/audits/2026-07-15-reed-substrate-dishonest-rust-extensions-
  during-gift-arc.md` — Seam audit of Reed's 5 Rust extensions.
- `docs/scouts/2026-07-15-reed-rust-extension-migration-map.md` —
  Reed migration-map §5 empirical grep + §6 Arc-1..Arc-2.5 sequence.

### §10.3 Taut #108 scout

`docs/scouts/2026-07-15-taut-kintsugi-ouroboros-substrate-scout.md`
(1118 LOC) — grep-first ratification; §C composition graph; §D4
four-conjunct invariant; §D9 recognition candidates; §D10-D12
adjudications; §E A1-A10 adjudication queue.

### §10.4 The @kintsugi manifesto

"The gold does not un-break the bowl. The gold makes the bowl one
thing again."

Applied to this arc: the arc does not undo Reed's Rust extensions or
un-write the compiler's Rust history. The arc mends the fracture
lines with gold — the evaluator FLOOR + iterative shard-body
collapses — until the bowl (the compiler) is one thing again at
shard-body altitude, with Rust only at the irreducible FLOOR.

### §10.5 Math ancestor citations

- `docs/specs/eigensheaf.md` §4.3 (each Pack agent IS an eigensheaf),
  §6.3 (continuity is reconstruction, not persistence), §2.3-2.5
  (sheaf-Laplacian eigenbasis + Hodge decomposition). Grounds §4.2.
- `docs/specs/fate-bounded-psychohistory-sheaf-cohomology.md` §3
  (H¹ gradient descent = Rayleigh on Δ_F), §5 (Fate::bounded
  signature). Grounds §4.3.
- `shards/torus.mirror` Foerster verbatim citations at p. 238, 244,
  256, 282. Grounds §4.4.
- `shards/mirror/bench.mirror:37-74` (the ouroboros bites) +
  `:40-54` three-conjunct template definition + `:363`
  `monotone_non_increasing` action-decl. Grounds §4.5.
- `docs/specs/spectral-coherence-substrate-metric-synthesis.md:468`
  ("Compiler at build-altitude: @kintsugi runs the ouroboros loop
  `e^(n+1) ≤ e^n` — one Rayleigh descent step per pass on the
  substrate-graph's sheaf-Laplacian"). Grounds §4.3.2 formalisation.

### §10.6 Substrate-already-had-the-word

- `ouroboros` at 20+ landed sites (shard-decl §Substrate-already-had-
  the-word). ~55th instance of substrate-already-had-the-word
  discipline since Alex named it.
- Two-tick discipline honored: readable name `@kintsugi/ouroboros`
  over foundational alternatives (`@kintsugi/self-compile-collapse`,
  `@kintsugi/compiler-mending`).
- No new family-root mints in this landing; every composition target
  is substrate-decl'd today.

---

*Canonical spec closure. The arc is grounded mathematically at
category-theoretic + spectral + Rayleigh-descent + Foerster-
autopoietic altitudes. The shard-decl at `shards/kintsugi/ouroboros.
mirror` composes these grounds at substrate altitude. Arc-1 evaluator
FLOOR is the load-bearing enabler; Arc-2..N ride mechanically once
Arc-1 lands. The gold makes the bowl one thing again.*
