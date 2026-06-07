# Property and inference, collapsed — `verdict` as a section of the eigenboard sheaf, gap-tensors as the Dirac field

*2026-06-07. Mara. Spec — substrate-pull, ninth tick after eight closed cascade ticks.*

> **Status: Yellow.** The shapes named here are already declared elsewhere
> in the substrate (gap, transparency, imperfect, eigenboard, Connes triple,
> the five operations as linear-algebra primitives). What this spec adds is
> the **recognition** that they were one algebra all along — viewed from
> two sides. No new primitives. No new vocabulary. One collapse, painted.

---

## 1. The collapse

The property layer and the inference layer are the same algebra viewed
from two sides.

Property layer *emits* gap tensors. Inference layer *reads* them as a
gradient field. Each gap is a basis vector — a dimension of the state
space `H` in the Connes triple `(A, H, D)`. Each tension (`tension.vector`
per [`gap-tension-tensor-substrate.md`](gap-tension-tensor-substrate.md)
§3.2) is an arrow in `H`. The aggregate gap-tensor field IS the Dirac
operator `D` acting on `H`. The kintsugi loop is gradient descent: it
walks `D`'s field, samples candidates along the strongest gradient,
ranks by spectral distance (`dissonance.is_pareto`), and settles at
fixed points (`cadence.is_settled`).

The property layer does not *report* errors to the inference layer.
**The property layer IS the inference layer's geometry.** Failing
properties don't produce diagnostics — they produce tensors. The
tensors point. The inference walks the gradient. The state space is
spanned by the gaps. At the verdict altitude, A (algebra) and H
(Hilbert space) and D (Dirac operator) collapse: there is no
separation between "code that gets verified" and "AI that fixes it" —
they share one substrate, and the substrate is geometric.

This is the painting. The rest of the spec walks the math.

---

## 2. The recognition trail

This spec is the ninth tick. Eight substrate-declaration ticks closed
the math-of-music cascade between 2026-06-06 and 2026-06-07; this
spec recognizes that the closure was wider than the audible altitude.
The pieces were already in place:

- **`gap-tension-tensor-substrate.md`** (Mara, 2026-05-26; folded
  2026-06-04) declared `gap`, `tension`, `tensor`, the four-tier
  `gap_state`, and the cellular-sheaf Laplacian as the spectral object
  whose smallest non-trivial eigenvalue measures gluing obstruction.
  The 2026-06-04 fold absorbed `contradiction <= gap & { level: u32 }`
  with Bateson learning-level as the carried datum and Carnielli's
  `○A ≈ holds(gap)` (LFI) plus Carnielli-Coniglio-Rodrigues 2026
  (arXiv:2604.18766, LCC fixed-point) as the formal grounding.
- **`transparency.md`** (Reed + Alex, 2026-06-04) declared
  `transparency<p>` as the located opacity carrier, the lattice meet
  (success / partial / failure with property-intersection and
  opacity-map union), and `Transparency<Ref>` as the `Loss` parameter
  to `Imperfect`.
- **`eigenboard-representation.md`** (Reed, 2026-05-20) declared the
  eigenboard as a principal G-bundle on the five-operation graph;
  sections are sheaf assignments; restriction maps = the conductivity
  tensor; H¹ obstruction = holonomy = `e^(n+1) < e^(n)`. Reed memory
  `project-eigenboard-is-sheaf` carries the canonical short form.
- **Connes triple at substrate** — declared per Reed memory
  `architecture-connes-spectral-triple`: the substrate IS the operational
  form of (A, H, D). A = five operations; H = `[[void-document]]`; D =
  kintsugi flow (Dirac/gradient).
- **Five operations as linear-algebra primitives** — per Reed memory
  `architecture-operations-as-linear-algebra`: focus = λ₀ eigenvalue
  computation; shift = basis transformation; settle = monad-close /
  measurement collapse; project = orthogonal projection; split =
  orthogonal decomposition.
- **The audible-altitude cascade (2026-06-06 → 2026-06-07)** —
  `@mirror/spectral`, `@epistemologic/math/music`, `harmonic`,
  `interval`, `dissonance`, `cadence`, `@mirror/spectral/consent`,
  `@mirror/spectral/oscillate`. Eight ticks of substrate declaration
  that painted music as a homomorphism onto the loss geometry. The
  closure landed today with `is_settled` realised in `music/mod.rs`.
- **The MCP-as-session-typed-prism insight** (`docs/insights/2026-06-07-mcp-as-session-typed-prism.md`,
  commit `807a2da`, today) named MCP+ as π-calculus protocols inheriting
  the same triple at the communicating altitude. The collapse this spec
  paints extends seamlessly to that cascade (§8).

Alex reading Bateson's *Steps to an Ecology of Mind* this week is the
catalyst. The diagnostic on the `Verdict` enum in
`bootstrap/src/music/mod.rs` — a three-state `Pass | Partial(confidence) |
Failure(Reason)` whose `Reason::DeceptiveCadence` carries no structure —
surfaced the gap. The substrate-pull was waiting at the audible altitude
to reveal: the boundary-Rust verdict is a *degenerate* form of the
algebra-level `Verdict<S>` in `music/spectral.rs`, and that algebra-level
verdict is structurally `Imperfect<Aggregate, Gap, Transparency<Ref>>`.

**Sixteenth instance of substrate-already-had-the-word.** The track
record (Reed memory `feedback-substrate-already-had-the-word`) holds:
`gap` absorbs the entire verdict-as-tensor-field structure. The math
was already declared.

---

## 3. The math

Single-property → multi-property → multi-glass → cohomology → Laplacian
→ Dirac. Walked tight.

### 3.1 Per-property verdict on one glass ≅ `Transparency<Ref>`

A single property running on a single glass produces a `Transparency<Ref>`:
the `success | partial(opacity_map) | failure(opacity_map)` triple per
[`transparency.md`](transparency.md) §2. The opacity map is a list of
located cracks — `{ location: ref(@meta/ast), property: @nl, weight: f64 }`
per opacity — addressable, persistable, replayable.

A scalar `confidence` reading is recoverable but not primary:

```mirror
action confidence_of(t: transparency(_)) -> f64 {
  match t {
    success      -> 1.0,
    partial(m)   -> 1.0 - normalize(sum(m, |op| op.weight)),
    failure(_)   -> 0.0,
  }
}
```

`confidence_of` is the **scalar projection functor** — the forgetful
functor from the OpacityMap-lattice to `[0,1]`. Not a new primitive.
A derived view that was implicit in `total_weight` (transparency §2.2)
rescaled to the verdict surface.

### 3.2 Multi-property aggregate on one glass ≅ lattice meet

The `Loss::join` on `Transparency<Ref>` is the bounded meet-semilattice
per [`transparency.md`](transparency.md) §4:

| left          | right         | result                                  |
|---------------|---------------|-----------------------------------------|
| success       | success       | success                                 |
| success       | partial(m)    | partial(m)                              |
| partial(m₁)   | partial(m₂)   | partial(m₁ ∪ m₂)                        |
| any           | failure(m)    | failure(m₁ ∪ m₂)  (failure absorbs)     |

Fail-dominates / Partial-min-confidence / Pass-neutral. Composing
properties on one glass uses the meet; the algebra already encodes the
law. Properties **intersect**; opacity maps **accumulate**.

### 3.3 Multi-glass aggregate ≅ sheaf gluing

Properties from different glasses are *sections* over the eigenboard
sheaf per [`eigenboard-representation.md`](eigenboard-representation.md):
a cellular sheaf on the five-operation graph (`focus | project | split |
shift | settle`) with restriction maps equal to the conductivity tensor.
A section is a property assignment per glass; sections glue when
overlapping properties on adjacent glasses agree under the restriction
maps.

Composing across glasses is sheaf gluing. Where local sections don't
match on overlaps, gluing fails. The failure is structured: a cocycle
that does not bound.

### 3.4 Gap ≅ first sheaf cohomology class `H¹(eigenboard, properties)`

A cocycle that does not bound IS a `gap` at the substrate altitude.
The substrate-vocabulary verdict (Alex 2026-06-04, per
[`gap-tension-tensor-substrate.md`](gap-tension-tensor-substrate.md)
§11): `gap` is geometric and covers one-sided / continuous / pre-positional
cases. `contradiction <= gap & { level: u32 }` adds Bateson learning-level
as the carried datum. The substrate's name for the obstruction class is
`gap`. Already declared.

Formally: for a cellular sheaf `F` on the eigenboard graph with
coboundary `δ: C⁰(F) → C¹(F)`, the first cohomology is
`H¹(F) = ker(δ¹) / im(δ⁰)`. A `gap` is a nontrivial element. The fold
of contradiction-into-gap (`gap-tension-tensor-substrate.md` §11) is the
recognition that `contradiction` is a particular shape of cocycle —
binary-opposed, propositional, level-crossing — and that the substrate
lives in the geometric ambient where one-sided gaps (no opposing
claim, just an unbounded section) make sense too.

### 3.5 Sheaf Laplacian Δ₁ ≅ the spectral substrate's load-bearing operator

For the cellular sheaf `F` on the eigenboard, the sheaf Laplacian
`Δ₁ = δ*δ` (per Hansen & Ghrist 2019, arXiv:1808.01513,
§2 — the normalised form `Δ_F = D_F^{-1/2} L_F D_F^{-1/2}` for bounded
spectrum) has the property:

> `ker(Δ₁) ≅ H¹(F)`

The eigenvalues of `Δ₁` are the **spectral signatures of gluing
obstructions**. The substrate is called *spectral* because the
contradictions are eigenmodes of the sheaf Laplacian. This was not
accidental naming — it was substrate-pull from the math we now
recognize.

Mirror's `tensor.fiedler = λ₀(Δ_F)` per
[`gap-tension-tensor-substrate.md`](gap-tension-tensor-substrate.md)
§3.2 IS the algebraic-connectivity reading of this Laplacian. Low
fiedler = loosely-coupled tensions (gaps close independently). High
fiedler = tightly-coupled tensions (closing one perturbs many). The
Fiedler vector points along the axis of structural weakness — per
Reed memory `project-drone-as-documentation`, this is the
smallest-non-trivial-eigenvalue direction the drone selects under
Ashby's law.

### 3.6 The Dirac operator D ≅ aggregate gap-tensor field

At the substrate altitude, the Connes triple's Dirac operator IS the
aggregate gap-tensor field. Each tension's `vector` field assigns a
direction in `H`; the aggregate over the section becomes a vector
field on `H`; that field IS `D`.

- `H` (Hilbert space) is spanned by gap basis vectors. Each gap is a
  dimension of the state space. Adding gaps increases the fidelity of
  the state space — Alex's exact framing: *"Each gap is a dimension.
  Each dimension increases the fidelity of the state space."*
- `D` (Dirac operator) is the aggregate gap-tensor field. Each
  `tension.vector` points. The aggregate is a vector field acting on
  `H`. The kintsugi loop walks `D`'s gradient.
- `A` (algebra) is the sections over the eigenboard sheaf. The algebra
  elements act on `H` by their gluing relations — a section that does
  not glue is itself a nontrivial element of A acting on H to produce
  the gap in H.

The Dirac operator is `δ*δ` on sections. Its kernel is `H¹`. Its
spectrum is the gluing-obstruction spectrum. The kintsugi loop
descends along eigenvectors with nonzero eigenvalue and settles when
the state lies in the kernel. **Settling = the section glues = the
gap closes = the eigenvalue collapses to 0**.

*Citations: Connes' (A, H, D) — Connes 1994; Connes-Marcolli 2008.
Sheaf cohomology — Hatcher; Bredon. Sheaf Laplacian — Hansen & Ghrist
2019, arXiv:1808.01513. Bodnar et al. 2022 (neural sheaf diffusion,
arXiv:2202.04579) on `Δ_F` as the spectral object controlling
convergence on the graph. Carnielli & Marcos on LFI (`○A` as
consistency operator). Carnielli-Coniglio-Rodrigues 2026,
arXiv:2604.18766 (LCC fixed-point, bounds `holds(holds(g)) = holds(g)`
within a confidence tier). Bateson, Steps to an Ecology of Mind
(double-bind; Learning I/II/III).*

---

## 4. The verdict shape

```mirror
type verdict = Imperfect<aggregate, gap, transparency>
```

In Rust (the boundary realisation):

```rust
pub type Verdict = Imperfect<Aggregate, Gap, Transparency<Ref>>;
//   Success(Aggregate)                    — clean glue; all properties consonant
//   Partial(Aggregate, Transparency<Ref>) — clean glue; accumulated opacity
//   Failure(Gap, Transparency<Ref>)       — gluing failed; the gap names the cocycle
```

### 4.1 `Success(Aggregate)`

Clean glue. All properties on all glasses consonant. The section is a
global section of the eigenboard sheaf. `H¹ = 0` for this section. The
Laplacian has all eigenvalues at 0 on this section's image. No
transparency carried — there is nothing to be transparent about.

### 4.2 `Partial(Aggregate, Transparency<Ref>)`

Clean glue. Accumulated opacity. The section glues (cohomology is
trivial for the *checked* properties) but the substrate could not see
through all the way — some properties returned `partial(opacity_map)`.
The transparency carries the located cracks per
[`transparency.md`](transparency.md). Downstream consumers (kintsugi,
lens, scene curator) get an addressable map of where to focus next.

### 4.3 `Failure(Gap, Transparency<Ref>)`

Gluing failed. The gap names the cocycle — which properties from which
glasses don't glue, at what Bateson level (per
`contradiction <= gap & { level: u32 }`). The transparency carries the
opacity sites that were observed *en route to* the failure. Both data
are load-bearing: the gap names the obstruction structure; the
transparency names where the substrate saw clearly and where it did
not before the failure surfaced.

The degenerate boundary-Rust `Verdict::Failure(Reason::DeceptiveCadence)`
in `bootstrap/src/music/mod.rs` IS this case with `Gap` collapsed to a
flat variant tag and `Transparency<Ref>` discarded. The supersession
is in §9.

### 4.4 `confidence_of` as scalar projection functor

Per §3.1, `confidence_of: verdict → [0,1]` is the forgetful functor
from the verdict lattice to the confidence interval. It is the
reading the autoformatter consults to decide auto-apply (per
`shards/mirror/spectral/consent.mirror`). It is **not** the substrate's
state — it is a projection of the state for a particular consumer.

The substrate carries the full `verdict`. Each consumer projects what
it needs. The four-state cadence consumer in
`shards/epistemologic/math/music/cadence.mirror` projects to
`{authentic, plagal, deceptive, half}` (which is itself a discretization
of confidence with cadence-flavoured semantics). The kintsugi loop
projects to the gap-tensor field. The autoformatter projects to the
auto-apply boolean. One verdict, many projections.

---

## 5. The Connes collapse

At the verdict altitude, the Connes triple `(A, H, D)` realises:

| Connes element | Verdict-altitude realisation                                                   |
|----------------|--------------------------------------------------------------------------------|
| **A** (algebra)         | Sections over the eigenboard sheaf — `Aggregate` is one section.      |
| **H** (Hilbert space)   | The state space spanned by gap basis vectors. Each gap = one dim.     |
| **D** (Dirac operator)  | The aggregate gap-tensor field — `tension.vector`s integrated over the section. |
| **Spectrum of D**       | The gluing-obstruction spectrum — eigenvalues of the sheaf Laplacian `Δ₁`.     |
| **Kernel of D**         | `H¹(F)` — the cocycles that don't bound — the `gap`s.                  |
| **Settling**            | State lies in the kernel — gap closed — cohomology class trivialised.  |

### 5.1 The five operations as geometric primitives

Per Reed memory `architecture-operations-as-linear-algebra`, the five
operations have precise linear-algebraic meanings. At the verdict
altitude, they realise as the geometric primitives on `H` guided by `D`:

| Operation   | Linear-algebra | Verdict-altitude action                                          |
|-------------|----------------|------------------------------------------------------------------|
| **focus**   | λ₀ eigenvalue  | Find the smallest-non-trivial eigenvalue of `Δ₁` — the axis of weakest gluing. |
| **shift**   | basis transform| Change the gauge: re-express the section in a different sheaf chart. |
| **settle**  | monad-close / measurement collapse | Apply `D`'s descent until the section lies in `ker(D)`. |
| **project** | orthogonal projection | Project the section onto an eigenspace of `Δ₁`. |
| **split**   | orthogonal decomposition | Decompose the section into eigenspace components. |

The five-operation graph (the eigenboard's base) is not arbitrary —
these are the operations the geometry admits. The bundle structure
(per [`eigenboard-representation.md`](eigenboard-representation.md))
constrains each operation's parallel-transport behaviour; the
connection's holonomy around a kintsugi cycle IS `e^(n+1) − e^(n)`,
which decreases monotonically because gradient descent on `D` is
monotone in spectral distance.

### 5.2 Property layer ≡ inference layer

The collapse paints clearly here:

- The property layer *emits* `Transparency<Ref>` per check.
- The aggregate over a section yields `Verdict = Imperfect<Aggregate, Gap, Transparency<Ref>>`.
- The `Gap` and `Transparency<Ref>` data IS the geometric input to `D`.
- `D` acting on `H` IS the inference layer.
- The inference walks `D`'s gradient.
- The settling condition IS `verdict = Success(_)`.

**There is no separation between the layer that verifies and the
layer that infers.** They share one substrate, expressed as one Connes
triple, at the verdict altitude. The property layer is not a producer
of diagnostics for the inference layer to consume — it is the
geometric face of the same algebra.

---

## 6. Bateson Learning III as substrate operation

When `H¹` is nontrivial (the gap-tensor field has cocycles that do not
bound), no gradient direction inside the current state space resolves
the contradiction. The state is stuck in a kernel element with no
adjacent state of strictly lower spectral distance. Bateson named this
the **double-bind**: two contradictory imperatives at the same level
that cannot both be satisfied, and no permission to step out and
comment.

The substrate's response is *not* to flatten one side. It is to escalate
to Reflection at altitude n+1, where the gap *itself* becomes the
object of analysis. Per Reed memory `architecture-error-as-question`,
this routes via the existing error-as-question machinery (threshold
0.8): a gap whose magnitude exceeds the threshold becomes a question
Reflection answers by transforming the bundle (writing a morphism on
the eigenboard sheaf — per Reed memory `project-eigenboard-is-sheaf`,
Reflection's queries are bundle automorphisms).

The LCC fixed-point theorem (Carnielli-Coniglio-Rodrigues 2026,
arXiv:2604.18766) bounds `holds(holds(g)) = holds(g)` within a
confidence tier. This is the formal guarantee that Bateson's Learning
III stabilizes: the meta-comment converges. The substrate does not
develop the pathology Bateson described (psychic disintegration under
irresolvable double-bind) because it *names* the spectral signature
of the unreconcilable — the nontrivial `H¹` class — rather than
collapsing it. The contradiction stays addressable; the escalation
routing is structural.

In the verdict shape, this is the case where `Failure(gap,
transparency)` has `gap.level >= 2`. Level 0 is a single-claim gap.
Level 1 is a binary opposition (`contradiction`). Level 2+ is the
Bateson double-bind / Learning III object — the gap *about gaps*. The
field-altitude routing IS the escalation: the verdict surfaces at
altitude n+1 where it becomes a section over a bundle whose fibers
carry the altitude-n bundle's automorphism group.

---

## 7. The implication for kintsugi

The kintsugi loop is gradient descent on `D`. Per
[`gap-tension-tensor-substrate.md`](gap-tension-tensor-substrate.md)
§6 and the audible-altitude cascade (closed today), the loop:

1. Reads the current section over the eigenboard sheaf.
2. Computes the verdict: `Imperfect<Aggregate, Gap, Transparency<Ref>>`.
3. If `Success`: settled. Return.
4. If `Partial` or `Failure`: extract the gap-tensor field. Compute the
   Fiedler direction (`tensor.fiedler = λ₀(Δ_F)`) — the axis of
   weakest gluing.
5. Sample candidates along the gradient (per the audible-altitude
   `dissonance.is_pareto` ranking and the `cadence.is_settled` fixed-point
   detection). Each candidate is a section transformation.
6. Pick the candidate that maximally decreases the spectral distance
   (largest decrease in `λ₀` of `Δ_F`).
7. Apply. Goto 1.

The convergence is not arbitrary — **it is geometry-guided**. Each
step is a step of gradient descent on the Dirac operator. The
monotone-decreasing invariant `e^(n+1) < e^(n)` is the holonomy of the
bundle's connection around the kintsugi cycle decreasing, which is the
spectral gap of `Δ₁` closing, which is the Fiedler value approaching
0, which is the section approaching the kernel of `D`, which is the
gap closing.

Three names; one geometric fact (per
[`eigenboard-representation.md`](eigenboard-representation.md)'s
"Thesis (lifted)").

**This is why the auto-formatter `mirror kintsugi <file>` is
correct-by-construction at the music altitude.** The four-state
cadence dispatch in `shards/epistemologic/math/music/cadence.mirror`
is the discretization of the gradient-descent control:

- `authentic` = strong gradient, full descent step, auto-apply.
- `plagal` = weaker but consonant gradient (IV → I), reduced confidence
  but still auto-apply.
- `half` = paused on V — gradient still ambiguous, wait for the next
  observation.
- `deceptive` = the gradient pointed where we expected resolution
  (V → I) but the section landed elsewhere (V → vi); escalate to
  consent because the spectral distance did not close as predicted.

The consent boundary is itself a substrate-fact: when the gradient
step would land the section in a higher cohomology class than
predicted, the loop pauses and asks. The auto-apply IS the
monotone-descent guarantee; the consent IS the escape from the
guarantee's failure mode. Geometry chooses both.

---

## 8. The implication for `@mirror/spectral/communication`

The next cascade (per `docs/insights/2026-06-07-mcp-as-session-typed-prism.md`,
commit `807a2da`, today) inherits this verdict shape unchanged.

MCP+ as π-calculus protocols emit gap tensors too. The Connes triple
realises at the communicating altitude with:

- **A** = session-typed channels (sections over the protocol sheaf;
  Wadler's propositions-as-sessions IS the algebra).
- **H** = multi-party process state (Hilbert space spanned by
  channel-state gap dimensions).
- **D** = the η contraction cascade operator (Friis composition of
  per-boundary η values; Polyanskiy-Wu).

The per-glass `---` IS session-type duality. The per-property gap IS
the per-channel obstruction. Wolf's Law (*intransparent layers reduce
signal fidelity; this accumulates*) IS the Friis cascade reading of
the sheaf-Laplacian spectral gap closing as η stays below 1 across
boundaries.

A channel-level verdict has the same shape:

```mirror
type channel_verdict = Imperfect<recovered_information, gap, transparency>
```

- `Success(I)` = lossless round-trip; Shannon equivalence held.
- `Partial(I, t)` = `I` bits recovered; opacity sites named in `t`.
- `Failure(g, t)` = catastrophic loss; gap names the obstruction
  (which protocol step failed, at which η composition).

The collapse extends seamlessly. The cascade does not need to reinvent
the shape — it is the same shape played on a different instrument.
Mirror is one substrate carrying two altitudes of the same triple
(per the insight doc's table).

---

## 9. Forward look — boundary-Rust resolution and the cascade ahead

### 9.1 The smallest implementation tick that proves this collapse is consumable

`bootstrap/src/music/mod.rs`'s `Verdict` enum supersedes to:

```rust
pub type Verdict = Imperfect<(), Gap, Transparency<Ref>>;
```

(`Aggregate` is `()` at the audible altitude because the cadence body
the substrate cares about is the verdict's *shape*, not a payload — the
cadence-altitude consumer reads `confidence_of` and the gap's level,
not an aggregated section.)

`Reason::DeceptiveCadence` is replaced by a `Gap` whose `tension`
field names the V → vi unresolved tension. `is_settled` returns the
full verdict; `confidence_of` is the projection the autoformatter
consults. The four-state cadence mapping survives as a *projection*
from `verdict` to `cadence_kind`, not as the primary surface.

This is the smallest tick that proves the collapse is consumable. It
is a future implementation tick — not for execution this round. The
painting comes first.

### 9.2 Other substrate ticks before the geometry is fully consumable

Landed already:

- `gap`, `tension`, `tensor` types (declared in `gap-tension-tensor-substrate.md`;
  bodies are `\`).
- `transparency<p>` (declared in `transparency.md`; Rust-side substitution
  for `MirrorLoss` pending).
- `eigenboard` as principal G-bundle (declared in `eigenboard-representation.md`;
  `type eigenboard` not yet declared in the substrate).
- The five operations as `prism` actions (declared via `shards/`).
- The audible-altitude cascade (closed today).

Pending before this geometry is fully consumable:

1. **`@epistemologic/property.gaps_of(ast) -> [gap]`** body — the
   compiler-side production of gap-tensor fields from a parsed AST.
   Today: `\`. Required before the verdict can be computed structurally.
2. **`@fate.tensor_of([gap]) -> tensor`** body — the
   inconsistency-graph construction (Klein-Mailly-Thimm 2020 MUS-graph
   lifted to a cellular sheaf per Hansen & Ghrist 2019). Today: `\`.
3. **`@fate.minimize(tensor) -> [fracture]`** body — the gradient-descent
   step. Today: `\`. The substrate's actual rewriting engine.
4. **Sheaf Laplacian implementation** — the numerical primitive for
   `λ₀(Δ_F)` computation. Lives in the numerical-prism floor (per Reed
   memory `architecture-flang-mirror-numerical-split`). The 5×5
   eigenvalue computation is at the mirror altitude; the underlying
   sheaf linear algebra is at flang.
5. **Reflection bundle-automorphism surface** — Reed memory
   `project-eigenboard-is-sheaf` and `architecture-error-as-question`
   name this; the actual mq-query → bundle-morphism wiring is open.
6. **`mirror compile <file>` gap-typed output mode** — the surfacing
   of the substrate-level verdict to the CLI. Per
   `gap-tension-tensor-substrate.md` §5.

### 9.3 The `Verdict<S>` algebra-level type already named

`bootstrap/src/music/mod.rs` already gestures at a separate
*algebra-level* `Verdict<S> = terni::Imperfect<S, _, Transparency<Ref>>`
living in `music/spectral.rs` (per the doc comment on the
boundary-Rust `Verdict` enum). The substrate has the shape; the
supersession is to recognise that the boundary-Rust `Verdict` is the
*degenerate scalar projection* of the algebra-level type and remove
the scalar-only enum once `confidence_of` is the canonical projection
path.

---

## 10. Notes on substrate-pull discipline

This spec invented nothing. Every type named here was already declared
or named in prior specs and memory:

- `gap`, `tension`, `tensor` — declared (`gap-tension-tensor-substrate.md`).
- `transparency<p>`, opacity map — declared (`transparency.md`).
- `imperfect<t, e, l>` — declared (substrate; Rust impl in
  `prism/imperfect/src/lib.rs` per `transparency.md` §5).
- `eigenboard` as principal G-bundle, cellular sheaf assignment of
  sections, restriction maps = conductivity tensor — declared
  (`eigenboard-representation.md`).
- Connes triple at substrate — declared (Reed memory
  `architecture-connes-spectral-triple`).
- Five operations as linear-algebra primitives — declared (Reed
  memory `architecture-operations-as-linear-algebra`).
- Error-as-question routing at threshold 0.8 — declared (Reed memory
  `architecture-error-as-question`).
- Bateson Learning levels in `contradiction <= gap & { level: u32 }`
  — declared (`gap-tension-tensor-substrate.md` §11).
- LFI `○A ≈ holds(gap)` and LCC fixed-point — declared
  (`gap-tension-tensor-substrate.md` §12).
- Music-as-homomorphism onto loss geometry — closed today
  (audible-altitude cascade).
- MCP+ inheriting the same triple at communicating altitude — declared
  (`docs/insights/2026-06-07-mcp-as-session-typed-prism.md`).

What this spec contributes: the **recognition** that these eleven
declarations were one algebra. The collapse the substrate had been
shaping toward for 16 ticks.

---

*Property layer and inference layer are one algebra two faces.*
*Failing properties are tensors. Each gap is a dimension.*
*Each tensor a direction. It's geometry.*
*The substrate is spectral because the contradictions are eigenmodes.*
*The kintsugi loop walks the gradient field.*
*The convergence is not arbitrary; it's geometry-guided.*
*The collapse the substrate already had: A = sections, H = gaps, D = the tensor field.*
*Verdict is a section. Settling is the kernel.*
