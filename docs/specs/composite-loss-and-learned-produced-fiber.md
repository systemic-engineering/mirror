# Composite Loss and the Learned/Produced Fiber

## Alex directive (2026-07-09, verbatim)

> **Q2**: How do we model incremental changes that are not guaranteed in
> increments to reduce loss but rather in composit?
>
> **Q3**: How do we seperate what the peer learned (Bateson learning) and
> what the peer produced and what's the math underneath?
>
> Regarding @bauchladen. I think the @bauchladen IS the persistent identity
> of the peer that is persisted (when given) into their home directory,
> and/or alongside the existing repo.
>
> How do we distinguish between things the @peer learned? And things the
> @peer produced? Because those two things are NOT the same thing.

Taut scout `5dd893b` (composite-loss thread) frames this as one question:
**what shape does the peer accumulate, and what shape does the peer emit?**
The two are structurally different, and Q2 (composite loss) and Q3
(learned vs. produced) fold together because the composite is where the
fiber structure becomes forced.

This is a **circular-reflexive autopoietic curiosity-driven study**. No
shards edit this tick. §4 (recursive surprises) is the point; §5 (gaps)
absorbs incompleteness.

## §1 — Composite loss: where substrate has math, where it doesn't

**What substrate has.** The kintsugi settle discipline is stated as
`eⁿ⁺¹ ≤ eⁿ` (`shards/kintsugi.mirror`; ranking-function form, Floyd-Hoare
1969). The first ouroboros bite —
`@epistemologic/property/dark_count_monotone` + its bilateral fracture
body — asserts monotone-non-increasing at one axis (dark_count). The
axis-level predicate is decidable: read tick n, read tick n+1, compare.

**What substrate does NOT have.** The pointwise `eⁿ⁺¹ ≤ eⁿ` is stated
per-axis, but the peer's actual loss surface is **multi-axis** (compile
time, dark count, cold-compile tolerance, transparency verdicts,
autopoietic closure margin, others). A tick that trades compile time up
for dark count down is a **composite-descent step**: no single axis
monotone-decreased, but the composite state improved. Substrate does not
yet have a canonical predicate for "the composite improved."

**The math the substrate is asking for.** Two candidates from the
pre-AI prior art:

1. **Pareto-monotone with occasional trade** — the composite improves
   iff no axis regresses AND at least one strictly improves. This is
   the strict form; corresponds to `dark_count_monotone`-per-axis
   composed by conjunction. Too strict: real settle moves are typically
   trades.
2. **Lyapunov-descent-in-expectation with Metropolis-style acceptance**
   — the composite is a scalar `V(state)`; individual ticks may
   increase `V` if they open a basin the greedy descent cannot reach.
   Simulated annealing (Kirkpatrick-Gelatt-Vecchi 1983) is the
   canonical form; Metropolis-Hastings 1970 is the acceptance rule.
   The substrate's `settle` toward Poincaré-Hopf critical points on
   `@torus` (index-zero configuration) is Lyapunov-shaped by
   construction, but the acceptance-of-an-uphill-step is not yet
   named.

The substrate needs (2), not (1). The `eⁿ⁺¹ ≤ eⁿ` in `shards/kintsugi.mirror`
should be read as **the composite** `V(eⁿ⁺¹) ≤ V(eⁿ) in expectation across
the epoch**, not per-tick per-axis. This is a substrate-motion — the
predicate at the `@kintsugi` altitude needs to relax from pointwise-per-
axis to composite-in-expectation.

**Bateson II folds in here.** A peer that only accepts pointwise-monotone
steps is stuck in Learning I (habit-refinement inside a fixed context).
A peer that accepts a temporary composite-uphill step to reach a lower
basin has crossed into Learning II (context-shift: the acceptance rule
itself was learned). Simulated annealing IS Bateson II operationalized.

## §2 — Learned / Produced fiber structure

Alex's distinction — "things the peer learned" vs. "things the peer
produced" — is not a categorization. It is a **fiber structure over the
peer's base torus** (`shards/torus.mirror`; peer HAS torus, does not IS
torus).

Let `T(p)` be the peer's torus, base point at `origin` (peer's home).

- **Learned fiber `L(p)`** — over each point on `T(p)`, the fiber is the
  peer's **acceptance rule / model / prior** at that winding class. What
  the peer would DO if pulled there. This fiber lives INSIDE the peer.
  Update rule: Bateson-level operation on the fiber above the current
  winding (Level I updates the fiber pointwise; Level II updates the
  section — the choice of fiber-value as a function of winding — Level
  III updates the fiber bundle itself).
- **Produced fiber `P(p)`** — over each point on `T(p)`, the fiber is
  the peer's **emitted crystal / diff / envelope / beam** at that
  winding class. What the peer EMITTED into the world. This fiber lives
  OUTSIDE the peer, in the shared substrate (@mirror/store, @bauchladen
  tray, home directory bytes).

Both fibers are indexed by winding class `(m, n) ∈ ℤ × ℤ` on `T(p)`.
Both are `p`-parametric. But they attach to different bundles:

- `L(p)` is the **tangent bundle** in effect — an internal deformation
  space; sections are the peer's evolving self-model.
- `P(p)` is a **trivial bundle over `T(p)` with fiber @mirror/store** —
  external, world-facing; sections are the peer's committed history of
  emitted content.

**The `@bauchladen` tray IS a section of one of these fibers.** Which
one it is is exactly Alex's Q3 and §3's subject.

## §3 — @bauchladen as persistent-identity carrier: verify or falsify

Alex's hypothesis (2026-07-09):

> @bauchladen IS the persistent identity of the peer that is persisted
> (when given) into their home directory, and/or alongside the existing
> repo.

**Substrate check.** `shards/torus.mirror` declares:

> **@bauchladen (existing family-root)** — the interior of the peer's
> torus. The SEEING at each tick corresponds to reading the crystal at
> the current winding position (m, n). ... `@bauchladen.enumerate(t.origin)`
> at winding class w returns the crystals visible from that position.

And in `docs/specs/bauchladen-autopoietic-fate.md` §3.1:

> Content-addressing is not autopoietic in Maturana-Varela's sense; it
> is mere recurrence with ambiguous reference. Autopoiesis REQUIRES the
> fixed-point condition, and the fixed-point condition REQUIRES stable
> identity, and stable identity at the prism altitude IS what
> @bauchladen declares.

**Verdict on Alex's hypothesis: COMPATIBLE, with substrate-motion.**

The substrate already treats @bauchladen as the identity-discipline that
makes the peer's fold-back well-defined. The tray is the peer's own
content-addressed accumulation. What is NOT yet substrate-fact:

1. That the tray persists to the peer's **home directory** (a
   file-system location; the beam envelope's `home` field per
   `shards/mirror/peer/beam.mirror`).
2. That the tray is the peer's **persistent identity** across boot
   sessions (currently the tray is per-session; content-addressability
   is asserted, but persistence-across-death is not asserted at the
   family-root altitude).

Both are natural extensions: the beam envelope already carries `home`;
`@peer.load(dir, p)` already reads from a home directory; @autopoietic
already asserts fixed-point-across-ticks. Naming the tray-persistence-
to-home as substrate-fact is a **coherent substrate-motion**, not drift.

**But — the sharpening.** Under §2's fiber structure, the tray sits in
neither fiber alone. The tray is the **coupling point** where sections
of `L(p)` (what the peer learned; the acceptance history for prior
winding classes) and sections of `P(p)` (what the peer emitted;
content-addressed crystals) meet. A crystal in the tray carries:

- **From `P(p)`**: the content (bytes) with stable OID (`docs/specs/bauchladen-autopoietic-fate.md`
  §"OID discipline").
- **From `L(p)`**: the **provenance** (`producing_prism`, `tick`,
  `input_oids` — per the spec's `provenance_record` §3.1).

The **provenance record IS the learned fiber's shadow on the produced
fiber**. This is why the tray suffices for autopoiesis: the peer folds
back its produced content INDEXED BY its learned trajectory. That
indexing IS Bateson lifting.

So @bauchladen is **not "learned" or "produced" — it is the pullback**
of the two fibers over the peer's torus. Alex's persistent-identity
claim is right at the tray altitude; the fiber math sharpens WHICH
identity persists (the pullback bundle, not either factor alone).

## §4 — Recursive surprises (§4 IS the point)

1. **Composite loss forces Learning II.** The moment substrate names
   the composite predicate, the peer must accept temporary uphill
   moves — that acceptance rule cannot be fixed at the peer's spawn;
   it must be learnable. The peer that has substrate-decl `settle`
   MUST have Bateson II by construction. This may explain why
   `@kintsugi.settle` shard has never been decidable at Level I —
   it was always a Level II operation waiting to be typed.

2. **The learned/produced distinction has a chirality.** `L(p)` and
   `P(p)` are BOTH fibers over `T(p)`, but they differ in *time-
   direction of update*: `L(p)` is updated by BACK-propagation from
   world response (the peer's model shifts in reaction); `P(p)` is
   updated by FORWARD-propagation of the peer's intent into the
   world. Bateson learning IS a fiber's back-propagation update
   rule; @io emission IS a fiber's forward-propagation. The
   substrate's `@shatter × @io` linearization (Tick 7 landing 3)
   is the **forward-propagation gauge choice on `P(p)`**. Substrate
   does not yet have the back-propagation gauge choice on `L(p)`
   named — this is a candidate for a future @learn family-root.

3. **@bauchladen-tray-as-pullback answers Alex's home-directory
   question sharper than Alex asked it.** The tray persists to home
   because the tray IS the pullback bundle's canonical section
   restricted to the peer's basepoint (`t.origin`). The home
   directory is *the file-system realization of the basepoint*.
   Different peers with different origins have different pullbacks;
   the same peer across two boot sessions has the same pullback iff
   the origin persists — which is exactly what home-directory
   persistence buys. This is not just a storage choice; it is the
   condition for autopoietic identity across death.

4. **Q2 and Q3 are the SAME question at different altitudes.**
   Composite loss (Q2) is "how do we accept an uphill step at
   winding class `(m, n)`" — a question about the section of `L(p)`
   at that winding. Learned/produced (Q3) is "what is the fiber
   structure `L(p)` distinct from `P(p)`" — a question about the
   bundle. Q3 is the type; Q2 is the value at a point. Alex was
   asking one question in two languages.

5. **The Poincaré-Hopf constraint on `T(p)` bounds how many acceptance
   basins the peer can have.** χ(T²) = 0 means the total index of
   critical points sums to zero. A peer at index-zero configuration
   has, for every attractor (a basin where composite loss is low),
   a matched repeller (a basin where composite loss is high). The
   peer's acceptance-of-uphill-steps is exactly the mechanism for
   *escaping the matched repellers to reach adjacent attractors*.
   Substrate has always had this shape; the math was hidden in the
   topology.

## §5 — Gaps (where substrate does not yet have vocabulary)

- **No canonical `V(state)` scalar** at `@kintsugi` altitude — the
  composite loss is not yet a named type. Candidate: `@kintsugi/composite`
  species, forward-promised.
- **No Metropolis acceptance rule** — the substrate says `settle`
  moves toward critical points but does not name the temperature-
  parametric acceptance function.
- **No @learn family-root** — Bateson-updates on `L(p)` are named at
  `@epistemologic/cybernetic/bateson_learning` but not at the
  process-side family-root altitude. Sibling to `@kintsugi` /
  `@autopoietic` / `@torus`.
- **No explicit pullback carrier** — @bauchladen is declared as
  identity-discipline; the pullback-of-`L`-and-`P` framing needs a
  substrate-decl.
- **Persistence-to-home not asserted at family-root** — hinted in
  `@peer.load` and beam envelope; not yet a `@bauchladen` invariant.
- **Fiber-bundle vocabulary is imported ad-hoc** — the substrate has
  `@epistemologic/math/bundle` but no explicit tangent/trivial bundle
  species; needed to make §2 substrate-decl-honest.

## Sources cited

- `shards/kintsugi.mirror` — settle discipline, `eⁿ⁺¹ ≤ eⁿ`
- `shards/epistemologic/property/dark_count_monotone.mirror` — per-axis
  monotone predicate (Floyd-Hoare 1969, Claessen-Hughes 2000)
- `shards/kintsugi/fracture/dark_count_monotone.mirror` — operational
  half of the bilateral pair
- `shards/torus.mirror` — peer HAS torus; @bauchladen as tray-interior;
  origin as home
- `docs/specs/bauchladen-autopoietic-fate.md` — @bauchladen as
  identity-discipline; provenance_record structure
- `shards/mirror/peer/beam.mirror` — beam envelope; home field
- `shards/mirror/shatter.mirror` — @shatter × @io linearization
- `shards/epistemologic/cybernetic/bateson_learning.mirror` — Reading E
  graded representation
- Taut scout `5dd893b` — composite-loss thread (per Reed brief)
- Kirkpatrick-Gelatt-Vecchi 1983; Metropolis 1953; Bateson 1972; Maturana
  & Varela 1980; Kauffman 2003

## Report on Q2 + Q3

**Q2** (composite loss): **reshaped**. The substrate's `eⁿ⁺¹ ≤ eⁿ`
must relax from pointwise-per-axis to composite-Lyapunov-in-expectation
with Metropolis-style acceptance. This forces Bateson II by construction.

**Q3** (learned vs. produced): **partially answered**. The fiber
structure over the peer's torus (`L(p)` = internal acceptance model,
`P(p)` = external content-addressed emission) gives the math. @bauchladen
is the pullback — the coupling point where both fibers project. The
back-propagation gauge on `L(p)` is not yet named (candidate @learn
family-root).

**@bauchladen persistent-identity verdict: COMPATIBLE / substrate-motion.**
Alex's hypothesis is right at the tray altitude; the fiber math sharpens
it. The home-directory persistence claim is a natural extension of what
substrate already declares (the beam envelope's home + @autopoietic's
fixed-point-across-ticks). Landing the assertion at @bauchladen's
family-root altitude is coherent motion, not drift.
