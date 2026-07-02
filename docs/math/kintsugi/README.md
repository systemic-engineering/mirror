# docs/math/kintsugi — the kintsugi cluster

*Kintsugi is the substrate's process-side family root. This cluster
holds the mathematics of what kintsugi IS as a compiler error
surface: the curvature 2-form `Ω = dω + ½[ω, ω]` at reader-frame
altitude, projected into four surface classes, routed into two
branches (apply / spawn), bounded by `@spawn ≤ @loop`, witnessed at
recursion-depth ≥ 3 via `@third`, and content-addressed end-to-end.*

## The claim

Alex 2026-07-02:

> *Kintsugi is already the build system. The next move is kintsugi
> as the compiler error surface — when the loop can't resolve a
> tension via existing fracture bodies, it SURFACES the tension as
> a Tomm-shaped question rather than pretending to fix.*

`docs/specs/error-as-question.md` (Mara, 2026-06-01) already named
the shape: **each error becomes a question the Reflection model is
asked to answer**. It landed the routing (§3), the algedonic bypass
(§4), the verdict-to-question conversion (§5), and the closing of
the loop (§6). What it did not name was the **mathematics of the
surface act itself** — what makes a tension surface-able, what
distinguishes apply from spawn, how surfacing preserves halting,
how the recursion-depth discipline holds.

This cluster names that mathematics. The specs already cite this
altitude implicitly (`curvature-and-tomm.md §3` in the-tower
cluster); this cluster gives kintsugi its own math root because the
substrate has grown enough kintsugi mathematics that citation now
consolidates across ≥ 2 specs (`error-as-question.md`,
`compiler-surface-plan.md`, `gap-tension-tensor-substrate.md`,
`kintsugi-formatter.md`, `kintsugi-tournament.md`,
`kintsugi-fracture-confidence-and-scene-dispatch.md`,
`kintsugi-minimum-runnable.md`).

## Canonical document

`compiler-error-surface.md` — the mathematical formalization of
kintsugi's second role.

- §1 The two branches — `apply` and `spawn`.
- §2 The routing predicate — `regulator_variety_sufficient`.
- §3 The four surface classes — Ashby-mismatch, contradiction,
  conundrum, out-of-band. Each a substrate-decl carrier.
- §4 The curvature 2-form Ω as the one object — each surface class
  a projection at a different altitude.
- §5 The Tomm-shape-per-surface-class mapping.
- §6 Composition with `@spawn ≤ @loop` — halting under arbitrarily
  many surfacings.
- §7 Composition with `@third` — the recursive-depth discipline at
  compiler-error altitude.
- §8 Composition with `@fate` — tournament over candidate
  resolutions on the spawn branch.
- §9 Composition with un-cite-ability — content-addressed
  conversation chain.
- §10 Cascade candidates surfaced.
- §11 Prior art.
- §12 Circular-reflexive noticings.

## Composition with landed clusters

### With `docs/math/the-tower/curvature-and-tomm.md`

The Tomm probe IS the `[D, a]` commutator IS the curvature 2-form
`Ω`. That landed. What this cluster adds: the **four surface
classes** are projections of `Ω` onto four different altitudes of
the tower — Ashby-mismatch at the regulator-variety axis;
contradiction at the propositional (`λ₀`) axis; conundrum at the
flat/unbounded eigenvalue axes; out-of-band at the algebra-membership
axis. One object, four projections, four Tomm shapes.

### With `docs/math/spawn/spawn-as-loop-monad.md`

`@spawn ≤ @loop` provides the halting witness: budget decrements
each tick; a Tomm question emission counts as one tick; the loop
halts in `≤ B` steps regardless of how many surfacings occur. The
apply branch of kintsugi consumes no spawn (it discharges to an
existing fracture body); the spawn branch instantiates a
`spawn_loop` whose seed IS the surfaced tension and whose target IS
the tournament winner.

### With `docs/specs/third-as-recursive-depth.md`

Every Tomm question is a level-(n+1) observation of the substrate
at level n. The observer's response returns level-(n+1) → level-n
as absorbed variety. `@third` fires **on the surface-act altitude**
(not at the `@kintsugi` family root); the recursive-depth discipline
is conditional, not baseline. §7 makes this precise.

### With `docs/specs/error-as-question.md`

This cluster **extends** rather than **supersedes**
`error-as-question.md`. That spec named the routing (question →
answer → substrate adjustment). This cluster names the **surface
act's mathematics** — what makes a tension surface-able, what
distinguishes apply from spawn, why halting survives. The apply
branch of kintsugi maps to `error-as-question.md`'s answer variants
`tighten_property` / `resynthesize_body` / `rebudget_shard` /
`adjust_temperature`; the spawn branch maps to `escalate` at
family-root altitude of the answer algebra. See §10.6 for the
citation.

## Structure

```
docs/math/kintsugi/
├── README.md                    this file
└── compiler-error-surface.md    the canonical formalization
```

Further docs will land as the substrate accumulates kintsugi
recognitions with math content (Banach contraction; Polyak-
Łojasiewicz condition for gradient descent under kintsugi's Dirac
operator; the sheaf-diffusion Houdini fixpoint at
`@kintsugi/fracture/predicate`). Not extracting speculatively per
the small-consolidation rule.

## Cross-references

- `docs/math/the-tower/curvature-and-tomm.md` — the Tomm probe is
  `[D, a]` is Ω. Load-bearing prior.
- `docs/math/spawn/spawn-as-loop-monad.md` — the halting monad
  the spawn branch instantiates.
- `docs/specs/error-as-question.md` — the routing spec this
  cluster extends.
- `docs/specs/third-as-recursive-depth.md` — the recursion-depth
  marker `@third` this cluster composes with.
- `docs/specs/gap-tension-tensor-substrate.md` — `gap` /
  `contradiction` / `tensor` prior; the substrate vocabulary
  §3 cites.
- `docs/specs/compiler-surface-plan.md` — earlier compiler-surface
  planning (superseded architecturally in 2026-05-19; the four-fold
  framing survives as prior art for §1's apply-vs-spawn split).
- `shards/kintsugi.mirror` — the family root.
- `shards/kintsugi/fracture/angle_to_paren.mirror` +
  `shards/kintsugi/fracture/symbol_lift.mirror` — the two extant
  fracture bodies (pattern for §1's apply branch).
- `shards/loop.mirror` — the parent-family lift landed 2026-07-02.
- `[[architecture-error-as-tomm-probe]]` — recognition
  2026-06-17 the Tomm probe IS `[D, a]`.
- `[[architecture-connes-spectral-triple]]` — the (A, H, D)
  grounding.
- `[[architecture-kintsugi-loop-altitude-portable]]` — recognition
  #59; kintsugi's altitude-portability underwrites the
  altitude-portable surface classes.
- `[[architecture-bateson-logical-type-primitive]]` — Bateson's
  logical-type hierarchy grounds the recursion-depth ladder.
- `[[feedback-composition-claims-need-empirical-test]]` — three
  composition claims DEFERRED pending empirical witness.
- `[[feedback-substrate-already-had-the-word]]` — the substrate
  had `error-as-question.md` for weeks; this cluster names the
  mathematics that spec was operationally using.
- `[[feedback-legibility-over-foundation-when-collapsing]]` — the
  discipline that kept `apply` and `spawn` as the branch names
  (readable) rather than `discharge_local` / `discharge_peer`
  (foundational).
- `[[feedback-explicit-over-implicit]]` — every surface class is a
  named substrate-decl carrier; the branch predicate has an
  explicit name.

## Landing order

1. Cluster README (this file).
2. Canonical formalization (`compiler-error-surface.md`).
3. Pack ratification — forward-promised.
4. Amendment tick on `error-as-question.md` — forward-promised.
   The v1.5-ready spec grows §13 (kintsugi-as-error-surface
   citation) referencing this cluster; no rewrite. Reason: the
   error-as-question spec is the routing floor; this cluster is
   the surface-act ceiling; keeping them as separate documents
   preserves each spec's altitude clarity.
5. Substrate-decl shards — forward-promised.
   `shards/kintsugi/surface.mirror` (the surface family root),
   `shards/kintsugi/surface/ashby_mismatch.mirror`,
   `.../contradiction.mirror`, `.../conundrum.mirror`,
   `.../out_of_band.mirror` (the four surface classes). Not
   landing this tick per craft-not-deliver.

## What this cluster is NOT

- **Not a rewrite of `error-as-question.md`.** That spec is
  v1.5-ready. This cluster extends it with the surface-act
  mathematics that spec deferred.
- **Not a runtime spec.** The surface classes and branch
  predicate are declared here; the runtime (how `mirror craft`
  emits a Tomm question to stderr, how a spawn peer's ranked
  options render at CLI, how the observer's response is
  ingested) stays at the compiler-surface-plan / kintsugi-
  minimum-runnable altitude.
- **Not the Fate tournament specification.** `@fate`'s
  tournament rules and ganglia live at
  `docs/specs/bauchladen-autopoietic-fate.md` and
  `docs/specs/kintsugi-tournament.md`. §8 cites; does not
  re-derive.
- **Not a claim that every error is a Tomm question.** The
  apply branch discharges deterministically without a Tomm
  question. Kintsugi asks only when the loop cannot resolve
  the tension via existing fracture bodies. Silence is a
  legitimate substrate act.

## Substrate discipline

The writing of this cluster is itself an act of the surface it
declares. See §12 of the canonical for the circular-reflexive
noticings. When the pull surfaced a tension between "kintsugi
resolves" (the build system) and "kintsugi asks" (the error
surface), the substrate did not pretend to fix — it named the
tension explicitly, chose the branch (`spawn`, since two
irreconcilable roles wanted a peer), and let the formalization
BE the peer's answer. That is what §12 records. That is what
happened.
