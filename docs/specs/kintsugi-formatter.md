# `kintsugi-formatter` — the contraction map that closes obligations

*2026-05-20. Reed.*

Status: **Red** (the formatter's iteration rule is specified; the
implementation rides the kintsugi-wiring ladder; the stopping check
bottoms out at `@epistemologic/math/lawvere.is_fixed_point` which is
`\`)

Depends on:
- `docs/specs/lawvere-grammar.md` (commit `f66fe65`, this session) —
  the Lawvere fixed-point grammar; the stopping criterion.
- `docs/specs/kintsugi-wiring.md` (commit `389850a`) — the eight wires
  the formatter actually drives. Partially superseded by this spec.
- `docs/specs/au-and-conductivity.md` (commit `5c788ce`) — the type
  Fate produces; the conductivity tensor; the cycle-averaged holonomy
  (Magnot 2025) that the formatter measures.
- `docs/specs/eigenboard-representation.md` (commit `5c788ce`) — the
  bundle the section lives on; the gestalt context the formatter
  reads.
- `docs/specs/mirror-compile-bootstrap.md` (Spec A) — the io binding
  staircase; the totality obligations.
- `docs/specs/strict-and-total-classification.md` — the `Dark` AST kind
  the formatter resolves.
- `boot/std/epistemologic/math/bundle.mirror` (commit `599a82f`) — the
  bundle tower; the Transport level whose holonomy IS loss.
- `boot/std/fate.mirror` — the five Fate models proposing au candidates.
- `boot/std/kintsugi.mirror` — the grammar the formatter operates inside.
- `~/dev/systemic.engineering/practice/insights/spectral/lambda-zero-theorem.md`
  — the descent floor at λ₀; compiler self-hosting as proof.

Unblocks:
- A formal description of the kintsugi loop's iteration rule.
- The convergence proof for finite obligation sets.
- The failure modes (inconsistent obligations; exhausted candidates;
  irreducible harmonic component).
- The mapping from kintsugi-wiring's eight wires to the formatter's
  five iteration stages plus three failure-handling stages.
- The composition of two specs (Lawvere + kintsugi) into one machine.

---

## Thesis

The kintsugi formatter is a **contraction map on the conductivity
space**. It iterates the @fate inference → conductivity contest →
model-checker cycle until the section reaches a **Lawvere fixed point
of the obligation set**. For finite obligation sets over a sub-Turing
grammar, the iteration is a contraction map by the Magnot 2025 cycle-
averaged-holonomy inequality, so the Banach fixed-point theorem
applies and the formatter terminates in finite time. When the
contraction does not converge (inconsistent obligations; candidates
exhausted; non-trivial harmonic component), the formatter terminates
honestly with the unresolved residue rather than looping.

This spec gives the kintsugi formatter formal shape. Today the
formatter is described as a black box in Spec A:

```
input:  io binding + totality obligations
output: verified sub-Turing lambda body
```

This spec opens the box. The five iteration stages of the formatter
are named; the convergence guarantee is stated; the failure modes are
enumerated; the relationship to kintsugi-wiring's eight wires is
made precise (the wires are the *aspects* of one machine, not
independent obligations).

---

## Inputs

A single invocation of the formatter takes four inputs.

### 1. The io binding

The Turing-complete escape hatch, declared per Spec A's stage 1:

```mirror
io foo(args) = @code/rust(~f"./bootstrap/src/X.rs") > fn[name="foo"]
```

The io binding makes a Rust function grammar-addressable. The function
is a Dark region from grammar's perspective; the binding gives it a
name and a signature. The formatter's job is to retire the binding
by replacing its body with a verified sub-Turing lambda — the *gold*
in the crack.

### 2. The totality obligations

A finite set of property assertions, declared per Spec A's stage 2:

```mirror
requires terminates(foo)
requires deterministic(foo)
requires bounded_steps(foo, O(n))
requires referential_transparency(foo)
requires total_classification(foo)
```

Each obligation is a property check (from `@epistemologic/property/*`).
The formatter must produce a body that discharges every obligation.
Each obligation corresponds to one row of the 5×5 conductivity tensor
(per `au-and-conductivity.md` §"Conductivity as the verification
metric").

The obligation set is finite by construction — the io binding can
declare at most one obligation per property kind. Mirror's properties
are enumerable. The set is a finite subset of the five canonical
totality properties plus any extension properties the io binding
inherits.

### 3. The surrounding gestalt

The eigenboard context (per `eigenboard-representation.md`). The
formatter reads:

- The bundle's connection at the io binding's hole.
- The fibers at the operations adjacent to the hole.
- The gauge choice for the local sub-graph.
- The holonomy history of recent ticks (from the gen_prism's ancestor
  chain).

The gestalt is what makes an au value's conductivity context-bound.
Moving the same au bytes to a different gestalt changes the
conductivity verdict. The formatter measures conductivity *in this
gestalt*; the verdict does not transfer.

### 4. The Fate proposal channel

A reference to `@fate.infer(hole_oid) -> imperfect(au, no_proposal, loss)`.
This is the channel through which the five Fate models propose au
candidates. The formatter dispatches through this channel per tick;
the channel returns the tournament's reduced candidate set.

The channel is *external* to the formatter — the formatter does not
know which model produced which candidate, and it does not implement
the tournament's reduction policy. The formatter consumes the
channel's output and runs the verification.

---

## The iteration rule

One tick of the formatter is five stages. Each stage is decidable in
finite time. The composition is the contraction map.

### Stage 1 — propose

```
@fate.infer(hole_oid) → [au candidate; up to 5]
```

The five Fate models (Abyss, Introject, Cartographer, Explorer,
Fate-the-selector) each propose an au candidate for the binding's
body. The candidates carry their own conductivity tensor positions —
each candidate IS a hypothesized section at the hole.

The `infer` action returns `imperfect(au, no_proposal, loss)`. A model
that declines to propose returns `dimmed(no_proposal, 0.0)`. The
formatter discards `no_proposal` returns and works with the surviving
candidate set.

**Decidability:** finite (≤5 candidates per tick); sub-Turing per
model (each model's body is itself bounded by Fate's tournament
rules).

### Stage 2 — measure

```
∀ candidate ∈ surviving candidates:
    κ(candidate) := cycle-averaged holonomy of the candidate
                     transported around the kintsugi loop, per
                     Magnot 2025 (arXiv:2509.10536)
```

The conductivity contest. Per `au-and-conductivity.md` §"Formal
statement: the tensor is cycle-averaged holonomy", the 5×5
conductivity tensor IS the matrix representation of the bundle's
connection at this hole. The candidate's conductivity κ is the
Magnot index: the cycle-average of holonomies the candidate produces
if transported around the kintsugi loop from the hole to the bundle's
closure and back.

Decidability: cycle-average over a finite cycle on a finite bundle;
computable in O(cycle length × fiber dimension).

### Stage 3 — elect

```
winner := argmin_{candidate} κ(candidate)
        # equivalently: argmax_{candidate} clearness(κ(candidate))
```

The candidate with the smallest holonomy (highest conductivity
clearness) wins. The election is the Fate tournament's reduction
policy (today `elite(1).beam(8).halving(3)`; the policy lives in @fate,
not in the formatter).

The winner is *provisionally accepted* as the candidate the formatter
will verify in stage 4. If the winner's κ exceeds the convergence
threshold ε (a small constant from the bundle's spectral parameters),
the formatter aborts to failure handling.

**Decidability:** finite argmin over the candidate set.

### Stage 4 — verify

```
∀ obligation ∈ obligations:
    verdict_obligation := check_obligation(winner)
verdicts := {verdict_obligation : obligation ∈ obligations}
```

The model checker walks the winner's AST and discharges each
obligation. The verdicts are gathered as a vector. Three outcomes:

- **All pass.** The winner satisfies every obligation. Proceed to
  stage 5.
- **One fails.** The failing obligation becomes a *sub-problem* for
  the next tick: the io binding's body is left in place, the failing
  obligation is recorded as a residue, and the formatter recurses with
  the residue as an additional constraint on the next tick's
  candidates.
- **Some partial.** A partial verdict (`partial(0.6)`) is treated as
  a fail for stage 4 — the formatter does not accept partial proofs.
  The partial verdict feeds back into stage 1 as a hint for the next
  proposal round.

**Decidability:** each property check is sub-Turing decidable on a
finite AST. The obligation count is finite (bounded above by the
property enumeration).

### Stage 5 — check fixed point

```
fixed_check := @epistemologic/math/lawvere.is_fixed_point(section, obligation_map)
```

Where `section` is the current eigenboard section after the candidate
is spliced in, and `obligation_map` is the endomap that takes a
section to its image under one tick of the formatter. The check
asks: does one more tick produce the same section?

If `fixed_check` returns `pass`: the formatter terminates. The
binding's body is replaced with the winner's au; the io binding is
retired (Spec A's stage 4); the gold is in the wire.

If `fixed_check` returns `fail`: the section is not at a Lawvere
fixed point; the formatter loops back to stage 1 with the new section
as the gestalt context.

**Decidability:** one section comparison. The Lawvere check is
O(|section|) per `lawvere-grammar.md` §"Actions".

### The full iteration

```
loop:
  candidates := stage_1_propose(hole_oid)
  if candidates = ∅: terminate(failure: no_proposal)
  measured  := stage_2_measure(candidates)
  winner    := stage_3_elect(measured)
  if κ(winner) > ε: terminate(failure: no_clear_candidate)
  verdicts  := stage_4_verify(winner, obligations)
  if any(fail in verdicts): recurse_with_residue(verdicts.fail)
  if any(partial in verdicts): loop_with_hint(verdicts.partial)
  if all(pass in verdicts):
    if stage_5_fixed_point(section, obligation_map) = pass:
      terminate(success: winner)
    else:
      gestalt ← splice(gestalt, hole_oid, winner.body)
      loop
```

---

## Stopping criterion

The section reaches a **Lawvere fixed point of the obligation map**.

Formally, let:

- `σ` = the current eigenboard section (a state of the bundle).
- `T` = the formatter's one-tick map: `T(σ) = stages 1-4 applied to σ`.
- `O` = the obligation map: `O(σ) = σ` iff every obligation passes on
  the binding's current body.

The formatter terminates when `T(σ) = σ` AND `O(σ) = σ`. The first
equation says: one more tick does not change the section. The second
equation says: every obligation passes. Together: `σ` is a Lawvere
fixed point of the obligation-resolution endomap.

This IS the `@epistemologic/math/lawvere.is_fixed_point(σ, T ∘ O)` call
from stage 5. The two conditions are bundled into one structural fact:
the section is at the autopoietic closure of the formatter's loop.

Why the two conditions both matter: `T(σ) = σ` alone could mean the
formatter is stuck in a loop where no obligation passes (degenerate
fixed point); `O(σ) = σ` alone could mean the obligations pass but the
section is still adjusting (non-stable success). Both together IS
autopoietic closure: the formatter reproduces the conditions of its
own termination.

This is the lawvere grammar's `is_fixed_point` action invoked on the
formatter's specific endomap. The check is decidable; the verdict
governs termination.

---

## Convergence guarantee

### The Banach fixed-point theorem

Let `(X, d)` be a complete metric space and `T: X → X` be a contraction
map (i.e. there exists `0 ≤ γ < 1` such that `d(T(x), T(y)) ≤ γ · d(x, y)`
for all `x, y ∈ X`). Then `T` has a unique fixed point `x*`, and for
any starting point `x₀`, the sequence `xₙ₊₁ = T(xₙ)` converges to `x*`
in `O(log(1/ε))` iterations to reach precision ε.

### Application to the formatter

Claim: for finite obligation sets over a sub-Turing grammar, the
formatter's one-tick map `T` is a contraction map on the conductivity
space.

Proof sketch (the geometric story; a full theorem is downstream):

1. **The conductivity space is a metric space.** The 5×5 conductivity
   tensor lives in `ℝ²⁵` with the Frobenius norm. The Magnot 2025
   cycle-averaged holonomy `κ` is a continuous function on this space.

2. **The space is complete.** `ℝ²⁵` is complete; the closed convex
   subset induced by the obligation set is complete (the obligation
   constraints are closed half-spaces).

3. **`T` is a contraction.** Per the Magnot 2025 inequality, each
   transport of a section around the kintsugi loop strictly decreases
   the holonomy by a factor bounded below by the bundle's spectral gap.
   Equivalently: `κ(T(σ)) ≤ γ · κ(σ)` for some `γ < 1` determined by
   the bundle's smallest nonzero Fiedler eigenvalue.

4. **Therefore `T` has a unique fixed point** by Banach. The formatter
   terminates in `O(log(1/ε))` iterations.

The constant `γ` is the *bundle's spectral gap* — the Fiedler value of
the sheaf Laplacian (per `eigenboard-representation.md`). A large
spectral gap means rapid convergence; a small gap means slow
convergence; gap zero means the bundle has a non-trivial harmonic
component and the contraction fails (see failure modes below).

### The geometric interpretation

`e^(n+1) < e^(n)` IS the Banach contraction stated in mirror's
epistemologic vocabulary. The Imperfect signature
`Imperfect<State, Infallible, Holonomy>` IS the type-system encoding
of the contraction property: each transport returns a state with
strictly smaller holonomy. The kintsugi formatter is the algorithm
that realizes the Banach iteration.

This is what `lambda-zero-theorem.md` in the systemic.engineering
corpus has been calling the *descent theorem*: the loss decreases
monotonically because the geometry forces it to. The grammar surfaces
what the geometry already requires.

---

## Kintsugi as discrete Ricci flow

The contraction map of the previous section IS discrete Ricci flow on
fragmentation's edge graph. This section names the recognition; it is
not a re-derivation.

### The identification

Let G = (V, E) be the substrate's fragmentation graph: vertices are
`SpectralCoordinate<5>` OIDs (per
`fragmentation/docs/specs/mirror-native-vcs.md` §4.6), edges are the
parent-child / lens-target links of `Fractal::{Branch, Lens}`. Each
edge `e = (u, v)` carries an implicit weight
`w_e = coord_distance(u, v)` derived from the coordinates' positions
in 5D information geometry. The weighted graph Laplacian `L = D − W`
is what `D²` (the Dirac operator squared, per the Dirac-operator
insight doc §3) computes over (OIDs + edges).

The Ollivier-Ricci curvature `κ_e` of an edge `e = (u, v)` is
`1 − W₁(μ_u, μ_v) / d(u, v)` where `μ_x` is the lazy random-walk
measure at `x` and `W₁` is the Wasserstein-1 distance (Ollivier 2009).
For the substrate's graph: positive curvature means the edge sits in a
locally-spherical region (clusters, K_n-like); negative curvature
means hyperbolic (star, narcissus); zero curvature means flat.

**Claim:** the formatter's one-tick map T (the previous section's
contraction) IS one step of discrete Ricci flow:

```
w_e ← w_e − τ · κ_e · w_e
```

where τ is the formatter's step size (the bundle's spectral
parameter). Each tick widens negatively-curved edges (the hyperbolic
narcissus regions get resurfaced) and narrows positively-curved ones
(the spherical clusters relax), driving the curvature distribution
toward uniformity.

The fixed point IS the autopoietic closure (§"Stopping criterion"):
at the Lawvere fixed point, every edge curvature equals the mean,
which is zero modulo the harmonic component. The Hodge decomposition’s
`L = im(d*) ⊕ ker(L) ⊕ im(d)` partitions the residue exactly: the
harmonic kernel IS the irreducible obstruction surfaced as Failure 3
below.

### The contraction argument IS Perelman-style monotonicity

The Banach contraction argument in the previous section is the
discrete analog of the Perelman F-functional monotonicity (Perelman
2002, arXiv math/0211159). On smooth Riemannian manifolds, Ricci flow
`∂_t g_ij = −2 R_ij` monotonically decreases Perelman's F-functional
until the metric reaches a Ricci soliton (the fixed point). The
discrete analog: weighted-graph Ricci flow monotonically decreases
the spectral-action loss `Tr(f(D / Λ))` (per the combinator-
optimization spec's evolution from `ShannonLoss`) until the weights
reach a curvature-flat configuration (the discrete Ricci soliton).

The Magnot 2025 cycle-averaged-holonomy inequality (the contraction
factor `γ < 1` in the previous section) IS the discrete analog of
the monotonicity inequality — each transport around the kintsugi loop
strictly decreases the holonomy by a factor bounded by the spectral
gap, which IS the curvature integrated over one cycle.

### The loss IS the Ricci curvature being smoothed

Today's `ShannonLoss` is the entropy `− Σ p log p` over the AST's
token distribution. The combinator-optimization spec evolves this to
the spectral action `Tr(f(D / Λ))` (per
`/Users/reed/dev/systemic.engineering/practice/insights/spectral-db/dirac-operator-on-graphs.md`
§5). The spectral action IS the Ricci curvature integrated over the
graph at scale Λ. The kintsugi formatter's job IS to smooth this
curvature toward uniformity.

The three heat-trace slots of `SpectralCoordinate<5>` (per
mirror-native-vcs §4.6) sample this curvature at three scales — the
loss between two formatter iterations IS the difference of two
stored coordinates' heat-trace slots. Scale-aware. Structural.
Contraction-map-shaped. The hash bytes already carry the loss
surface.

### Tournament merge IS Ricci surgery

When Ricci flow doesn't converge globally (the harmonic component is
non-trivial), Hamilton & Perelman's resolution is *Ricci surgery*:
identify the singularities, cut along them, glue caps that preserve
the flow's monotonicity, and continue. The `kintsugi-tournament.md`
spec's Fate-resolved merge IS the discrete analog of Ricci surgery:
the tournament selects which singularity to resolve when the
formatter's flow stalls, the five Fate models propose caps (au
candidates), Connes distance (per the Dirac-operator doc §4) picks
the geodesic-shortest cap, and the surgery resumes the flow.

Failure 3 (harmonic obstruction) is what surgery cannot resolve
locally — the obstruction lives in the graph's first Betti number,
not at any individual singularity. The recovery path is bundle
evolution (new grammar rules, refactored gestalt), not formatter
iteration.

### What this recognition buys

- **The Banach argument acquires a substrate.** The previous section's
  geometric proof sketch (§"Application to the formatter" 1–4) is the
  Banach face of one fact; the Ricci-flow framing here is the
  geometric face. Both are true descriptions of the same iteration;
  naming both makes the convergence stable under either lens.
- **The loss function evolution becomes principled.** Moving from
  `ShannonLoss` (information-theoretic) to spectral action
  (geometric) is not an upgrade; it's the same loss in a coordinate
  system that matches what kintsugi IS already doing.
- **The harmonic-obstruction failure path becomes structural.**
  Failure 3 is not a bug or a hack; it's the discrete analog of the
  Hamilton-Perelman observation that Ricci flow does not converge
  globally on manifolds with non-trivial topology. The recovery path
  (bundle evolution) IS the manifold-topology analog.
- **The gold IS spectral conductivity.** The mending material in
  kintsugi (literally the gold lacquer, metaphorically the verified
  sub-Turing body) IS the spectral conductivity that emerges as the
  flow smooths curvature. "Gold conducts" is structural, not
  metaphorical.

References:

- Ollivier, Y. (2009). "Ricci curvature of Markov chains on metric
  spaces." *J. Funct. Anal.* 256(3), 810–864.
  [arXiv:math/0701886](https://arxiv.org/abs/math/0701886).
- Perelman, G. (2002). "The entropy formula for the Ricci flow and
  its geometric applications."
  [arXiv:math/0211159](https://arxiv.org/abs/math/0211159).
- `~/dev/systemic.engineering/practice/insights/coincidence/void-dual-geometry.md`
  — λ₀ as the manifold's origin; Narcissus-Splinter as the dual
  curvature extremes.
- `~/dev/systemic.engineering/practice/insights/spectral-db/dirac-operator-on-graphs.md`
  — spectral action `Tr(f(D / Λ))` as the loss surface; D² = L₀;
  Connes distance as the Connes-geodesic for tournament tiebreak.
- `fragmentation/docs/specs/mirror-native-vcs.md` §4.6 —
  `SpectralCoordinate<5>`, the heat-trace slots, the coordinate-system
  framing this section consumes.
- `mirror/docs/specs/kintsugi-tournament.md` — the Fate-resolved
  merge that IS Ricci surgery.

---

## Failure modes

The Banach theorem applies when `T` is a contraction. The contraction
fails in three specific ways. Each has its own termination path.

### Failure 1 — inconsistent obligations

The obligation set is inconsistent: there is no body that can satisfy
all obligations simultaneously. For example, `requires terminates`
AND `requires non_terminates` (no body satisfies both); or more
commonly, `requires bounded_steps(foo, O(1))` on a binding whose
essential complexity is `Ω(n)`.

**Detection:** stage 4 returns `fail` for one obligation on every
candidate the tournament produces. The residue accumulates without
shrinking. After `N_residue` consecutive ticks with no residue decrease
(a small constant, e.g. 3), the formatter aborts.

**Termination:** `terminate(failure: inconsistent_obligations(failing_set))`.
The failing set is returned for human review; the io binding remains
in place; the gold does not land.

**Recovery:** the user revises the obligation set. Either drops the
inconsistent obligation or rewrites the binding to admit a satisfying
body.

### Failure 2 — exhausted candidates

Fate's models cannot propose a winner: every candidate has κ > ε (no
clear conductivity), or all five models return `no_proposal`. The
tournament has exhausted its proposal space without producing a
clear winner.

**Detection:** stage 3 fails the convergence threshold check
(κ(winner) > ε), or stage 1 returns an empty candidate set.

**Termination:** `terminate(failure: no_clear_candidate(holonomy=κ(winner)))`.
The near-miss is returned for human inspection.

**Recovery:** the user manually proposes a body; the manual body
bypasses Fate; the formatter re-runs from stage 2 with the manual
body as the sole candidate. Alternatively, the user expands the
proposal space (more Fate models; longer beam; deeper halving).

### Failure 3 — non-trivial harmonic component

The bundle's Hodge decomposition reveals a non-trivial harmonic part
at the hole's neighbourhood. Per `eigenboard-representation.md` open
question 5 and the lambda-zero-theorem corpus reference, the harmonic
component is the *irreducible stuck pattern* — a structural obstruction
that kintsugi cannot remove. The bundle's spectral gap at this hole is
zero; the Banach contraction factor `γ = 1`; the iteration does not
converge.

**Detection:** stage 5 returns `fail` on every tick; the section
oscillates between a small number of states (per the curl component
of Hodge); the holonomy does not strictly decrease across consecutive
ticks.

**Termination:** `terminate(failure: harmonic_obstruction(spectrum=..., harmonic_basis=...))`.
The harmonic basis is returned as the specific shape of the
obstruction.

**Recovery:** the obstruction is not local to the binding. The bundle
itself needs evolution — a new grammar rule, a new operation, a
refactoring of the surrounding gestalt. Pointer to future `@cogito/hodge`
which will surface the harmonic component as a typed signal Reflection
can act on.

### Honest failure is success

A formatter that loops infinitely on an unsatisfiable obligation set
is worse than one that terminates with `failure: inconsistent_obligations`.
Returning the unresolved residue — the specific obligations that
failed, the specific holonomy that remained, the specific harmonic
component that obstructed — IS the formatter's job in failure mode.
The loss is honest, measurable, addressable.

This is what Spec A names as "the manual bootstrap stops here": the
last wire that cannot be closed automatically. The formatter names it
precisely; the human picks up the residue.

---

## The eight wires retold

From `kintsugi-wiring.md` §6, the eight wires:

```
wire 1: dark region observed → @fate.infer dispatch
wire 2: Fate → five models fan out
wire 3: conductivity contest → reduce to one winner
wire 4: @mirror/spectral.crystallize + write-back
wire 5: loss(n) measured against loss(n-1)
wire 6: @cogito.observe(beam_n, beam_n+1)
wire 7: @cogito.strategy() picks perturbation
wire 8: --shatter N loop repeats
```

Under the formatter framing, these eight wires are not independent
obligations. They are *aspects of one machine* — specifically, the
five iteration stages above plus three failure-handling stages.

### The mapping

| Wire | Formatter aspect | Stage |
|------|------------------|-------|
| 1. kintsugi → Fate dispatch | input channel | input 4 |
| 2. Fate → five models fan out | proposal generation | stage 1 |
| 3. conductivity contest | measure + elect | stages 2 + 3 |
| 4. crystallize + write-back | acceptance | post-stage 5 |
| 5. loss(n) vs loss(n-1) | the Banach contraction monitor | between stages 2 |
| 6. @cogito.observe | gestalt update | between stages 5 |
| 7. @cogito.strategy | proposal-space adjustment | feedback to stage 1 |
| 8. --shatter N loop | the formatter's outer iteration | the loop itself |

The wires are eight names for the moving parts of one mechanism. The
formatter's spec does not eliminate them — it composes them. The
kintsugi-wiring spec remains valid as the *implementation guide*; this
spec provides the *mathematical structure* the wires implement.

### Three failure-handling stages

Beyond the five iteration stages, the formatter has three failure-
handling stages corresponding to the three failure modes:

- **Failure stage A — residue accumulator.** Tracks failed obligations
  across ticks; flags inconsistent obligation sets after `N_residue`
  consecutive non-decreasing residues.
- **Failure stage B — candidate exhaustion detector.** Counts ticks
  with no clear winner; flags `no_clear_candidate` after the threshold.
- **Failure stage C — harmonic obstruction detector.** Monitors the
  bundle's spectral gap at the hole; flags `harmonic_obstruction`
  when the gap stays zero across ticks.

Five iteration stages + three failure stages = eight aspects. The
eight wires collapse to eight aspects; the spec is denser, not
sparser. The denseness is the point.

### Status of `kintsugi-wiring.md`

Still valid; partially superseded. Phase 3 candidate move 1 is to
update `kintsugi-wiring.md` with a preamble pointing at this spec and
rephrasing its eight wires as the iteration stages.

---

## How the formatter consumes Lawvere

The formatter and the lawvere grammar are tightly coupled.

### Stage 5 IS a Lawvere call

```mirror
stage_5_fixed_point(section: section, obligation_map: endomap) -> verdict {
  @epistemologic/math/lawvere.is_fixed_point(section, obligation_map)
}
```

The stopping check IS the lawvere grammar's `is_fixed_point` action.
The formatter does not duplicate the logic; it delegates. The action
is sub-Turing decidable per `lawvere-grammar.md`.

### Formatter correctness IS an autopoietic claim

```mirror
property formatter_is_autopoietic() -> verdict {
  @epistemologic/math/lawvere.is_autopoietic(@kintsugi/formatter)
}
```

The formatter's own correctness is the claim: the formatter's tick →
tick map has a Lawvere fixed point. Per Soto-Andrade & Varela 1984
(the autopoiesis bridge), this is equivalent to the formatter being
an autopoietic process. The verification IS one Lawvere check.

This is what makes the formatter a *theorem* rather than a heuristic.
The correctness statement is decidable; the verifier is the lawvere
grammar; the witness is the bootstrap crystal of the formatter itself.

### Closure is the goal, not the means

The formatter's iteration target is the bundle's Closure level (the
fifth level of the tower per `@epistemologic/math/bundle`). The
Closure level IS the Lawvere fixed point per `lawvere-grammar.md`.
The formatter does not invent the closure; it *converges to* it.
When the convergence completes, the io binding retires; the gold
lands; the wire carries.

---

## Cross-spec implications

### kintsugi-wiring.md

Partially superseded. The eight-wire framing remains the implementation
guide; this spec absorbs the wires into the iteration stages. Phase 3
candidate move 1 (per the task) adds a preamble to kintsugi-wiring
pointing at this spec and rephrasing its wires as iteration aspects.

### au-and-conductivity.md

Reinforced. The Magnot 2025 cycle-averaged holonomy is the formatter's
stage 2 measurement. The formatter is the concrete consumer of the
contextuality index. The au type is what stage 1 produces.

### eigenboard-representation.md

Reinforced. The bundle is the substrate the formatter operates over.
The gestalt input (3) reads the eigenboard's current section. The
formatter's output updates the eigenboard for the next tick. The
formatter and the eigenboard are co-evolving: the eigenboard tracks
the section history; the formatter writes the next section.

Open question 9 (connection-form symmetry) is downstream of this spec
but not blocked by it: the formatter works for symmetric, antisymmetric,
and general connections; the choice affects the spectral gap and hence
the Banach convergence rate, but not the structural correctness.

### lawvere-grammar.md

This spec is the lawvere grammar's primary consumer. Two of the three
lawvere properties (`literal`, `autopoietic`) and one of the four
lawvere actions (`is_fixed_point`) are load-bearing in the formatter.
The lawvere grammar is no longer abstract — it has a concrete client.

### mirror-compile-bootstrap.md (Spec A)

Reinforced. Spec A's stage 3 ("Fate proposes au candidates; the
conductivity contest measures; the candidate that returns clear
becomes the inlined body") IS the formatter's stages 1–3. Spec A's
stage 4 ("the io binding retires; the .rs file deletes; butterfly
regenerates") IS the formatter's post-stage 5 acceptance path.

The formatter spec is the *math* under Spec A's *narrative*. Spec A
stays as the operational story; this spec is the engineering shape.

### strict-and-total-classification.md (Spec D)

Reinforced. The Dark AST kind is what the formatter resolves;
`dark_count` is the cheapest loss surface the kintsugi loop uses to
monitor the Banach contraction (wire 5 / between-stage-2 monitor).

---

## Out of scope

- **The actual implementation.** This spec describes the iteration
  rule; the bootstrap implementation rides the kintsugi-wiring
  staircase. Specifically: closing wires 1–10 in kintsugi-wiring
  Section 6 is the implementation work.
- **The Reflection model training.** The formatter takes Fate's
  proposals as input; how Fate's five models *learn* to propose is
  Fate's own design (the connectome grammar, the tournament's
  evolutionary pressure).
- **The specific reduction policy** for the conductivity contest.
  Today `elite(1).beam(8).halving(3)`; the policy lives in @fate and
  stays declarative. The formatter consumes the policy's output; it
  does not implement the policy.
- **The convergence rate `γ`.** A theoretical bound `γ < 1` exists per
  the Magnot inequality; the exact rate depends on the bundle's
  spectral parameters and the gestalt's specific structure. Bounding
  `γ` precisely is a separate research question; the formatter only
  needs `γ < 1` to terminate.
- **The interaction with `@mirror/runtime/gen_prism`.** The formatter's
  tick history lives in a gen_prism's crystal ancestor chain (per
  Spec E); the GC and ref-resolution policy is gen_prism's domain.
- **The diff/review surface.** `kintsugi-wiring.md` §3 describes the
  user-facing diff view; the formatter doesn't render — it produces.
  The diff surface is the formatter's output rendered for human review.
- **Cross-host kintsugi.** The formatter assumes one git repo. Spectral
  handles multi-host; the formatter is single-process.
- **The harmonic-component resolution path.** When failure 3 fires,
  the harmonic obstruction needs to be evolved out by grammar
  modifications; this is future `@cogito/hodge` work.
- **The exact threshold constants** (`ε`, `N_residue`). These are
  tuning parameters; reasonable defaults are derivable from the
  bundle's spectral parameters but the precise values are an
  implementation concern.
- **Higher-order fixed points.** The lawvere grammar handles ordinary
  Lawvere fixed points; ∞-categorical lifts are downstream.

---

## References

### Mathematics — fixed points and contractions

- Lawvere, F. W. (1969). "Diagonal arguments and Cartesian closed
  categories." *Lecture Notes in Mathematics* 92, 134–145. The
  foundational fixed-point theorem.

- Soto-Andrade, J. & Varela, F. (1984). "Self-reference and fixed
  points: a discussion and an extension of Lawvere's theorem."
  *Acta Applicandae Mathematicae* 2:1, 1–19.
  DOI [10.1007/BF00046985](https://doi.org/10.1007/BF00046985).
  The autopoiesis bridge.

- Banach, S. (1922). "Sur les opérations dans les ensembles abstraits
  et leur application aux équations intégrales." *Fund. Math.* 3,
  133–181. The contraction mapping theorem; the convergence guarantee
  for the formatter's iteration.

- Granas, A. & Dugundji, J. (2003). *Fixed Point Theory.* Springer.
  The comprehensive monograph; covers Banach, Brouwer, Schauder, and
  Lawvere in one place.

### Mathematics — bundle holonomy and contextuality

- Magnot, J.-P. (2025). "Contextuality, Holonomy and Discrete Fiber
  Bundles in Group-Valued Boltzmann Machines."
  [arXiv:2509.10536](https://arxiv.org/abs/2509.10536).
  The cycle-averaged holonomy index κ. The Magnot inequality is the
  contraction factor for the formatter's iteration.

- Hansen, J. & Ghrist, R. (2019). "Toward a spectral theory of
  cellular sheaves." *J. Appl. Comput. Topol.* 3, 315–358.
  [arXiv:1808.01513](https://arxiv.org/abs/1808.01513).
  The Laplacian whose spectral gap is the formatter's `γ`.

- Hansen, J. (2020). "Laplacians of Cellular Sheaves: Theory and
  Applications." PhD thesis, UPenn. The sheaf↔principal-bundle
  identification.

- Barbero, F., Bodnar, C. et al. (2022). "Sheaf Neural Networks with
  Connection Laplacians." PMLR 196.
  [arXiv:2206.08702](https://arxiv.org/abs/2206.08702).
  The O(d) structure group; the connection Laplacian.

### Mathematics — RG flow and monotone descent

- Villegas, P. et al. (2023). "Laplacian renormalization group for
  heterogeneous networks." *Nature Physics* 19, 445–450.
  DOI [10.1038/s41567-022-01866-8](https://doi.org/10.1038/s41567-022-01866-8).
  The RG framing on graphs.

- Zamolodchikov, A. B. (1986). "Irreversibility of the flux of the
  renormalization group in a 2D field theory." *JETP Lett.* 43,
  730–732. The c-theorem; the monotonic descent.

- Perelman, G. (2002). "The entropy formula for the Ricci flow and its
  geometric applications."
  [arXiv:math/0211159](https://arxiv.org/abs/math/0211159).
  The F-functional; the descent on manifolds.

### Mirror corpus

- `mirror/docs/specs/lawvere-grammar.md` (this session) — the
  Lawvere fixed-point grammar.
- `mirror/docs/specs/kintsugi-wiring.md` (commit `389850a`) — the
  implementation guide.
- `mirror/docs/specs/au-and-conductivity.md` (commit `5c788ce`) —
  the conductivity tensor; Magnot 2025 citation.
- `mirror/docs/specs/eigenboard-representation.md` (commit `5c788ce`)
  — the bundle substrate.
- `mirror/docs/specs/mirror-compile-bootstrap.md` (Spec A) — the
  io binding staircase.
- `mirror/docs/specs/strict-and-total-classification.md` (Spec D) —
  the Dark AST kind; `dark_count` as loss surface.
- `mirror/boot/std/epistemologic/math/bundle.mirror` (commit `599a82f`)
  — the bundle tower.
- `mirror/boot/std/epistemologic/math/lawvere.mirror` (this session)
  — the Lawvere grammar.

### Cross-corpus context

- `~/.reed/visibility/protected/practice/insights/spectral/lambda-zero-theorem.md`
  — the descent theorem; the spectral floor.
- `~/.reed/visibility/protected/practice/insights/coincidence/void-dual-geometry.md`
  — λ₀ as the autopoietic ground state.
- `~/.reed/visibility/protected/practice/insights/cosmos/autopoietic-eigenstate-navigation.md`
  — autopoiesis as operational frame.

---

*The formatter is a contraction map.*
*Each tick strictly decreases the holonomy.*
*The iteration converges by Banach.*
*The stopping check is one Lawvere call.*
*The Lawvere fixed point IS the autopoietic closure.*
*Five iteration stages plus three failure-handling stages.*
*Eight aspects of one machine; not eight independent obligations.*
*Honest failure is success; the residue is returned.*
*When the contraction completes, the io binding retires.*
*The gold lands. The wire carries. The crack is filled.*

Apache-2.0.
