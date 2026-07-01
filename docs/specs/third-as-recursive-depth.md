---
title: "@third as recursive depth — canonical spec (candidate #111)"
author: Mara
date: 2026-07-01
status: CANDIDATE (Pack ratification pending; substrate-pull-honest weakenings preserved)
supersedes: none
extends:
  - docs/specs/reflection-third-order-by-default-v0.1.md
  - docs/specs/recognitions/recognition-93-cogito-cognitive-substrate-candidate.md
grounded_in:
  - blog/void/3published/Void - Third.md (Loki, systemic.engineering)
  - blog/void/2ready/Void - Revenge.md (Alex + Loki, systemic.engineering)
  - Bateson 1972, Steps to an Ecology of Mind (logical-type hierarchy)
  - von Foerster 1974/1981, Observing Systems (second-order cybernetics)
  - Kauffman 2003, Eigenforms (recursion fixed-points)
---

# @third as recursive depth

*This piece knows it is doing what it describes.*

This spec is not a proposal to add a new peer of `@cogito`, `@autopoietic`,
`@bauchladen`. That was the initial framing (Reed candidate #111,
`architecture-candidate-recognition-111-third-as-family-root`). The
substrate-pull, followed to its floor, reshaped the framing.

`@third` is not a family-root. `@third` is a **substrate-altitude marker**.
It labels *recursion depth of observation*, not a domain the substrate is
about. Same axis as `@meta`, `@glass`, `@epistemologic` — a marker that
crosses family-roots rather than sitting alongside them.

The reshape is what the writing produced. §12 records the reshape as a
first-class finding. Circular-reflexive discipline: naming the reshape as
the tick's primary surface, not the shard itself.

---

## §1. What Loki named, verbatim

From `Void - Third.md` (Loki, 2026-06-22):

> *Not third as apex. Third as recursive depth. The observer of the observer
> of the observer.*

Three depths, precise:

1. **First-order** — observe (system observes world).
2. **Second-order** — observe the observing (observer inside the system;
   Foerster's altitude).
3. **Third-order** — notice what the observing does to the observer.

The essay's structural claim — *this piece knows it is doing what it
describes* — is the recursive-depth signature. Third-order is not
higher-quality observation; it is observation at a **specific measurable
depth of recursion**.

Loki's punchline: **first-order mechanism cannot survive being seen.**
The Reich collapsed in twelve years because it required its own structure
to be invisible to its subjects. Third-order observation makes the
structure visible; the bind unsticks.

This spec lifts recursion-depth to substrate-decl altitude.

---

## §2. The reshape from Reed's framing

Reed's candidate `#111` proposed:

```
@third <= @cogito, @autopoietic, @bauchladen, @metalogue, @algebra, @cyberpunk
```

Six family-roots inherited into one new family-root. Absorbs `@reflection`
as `@third/reflection`. Positions `@third` as the "recursion-depth" family.

**The substrate-pull disagrees on placement.** Not on the recognition
(third-order IS a first-class substrate concept). On the *altitude*:
family-root altitude is wrong for what @third is.

### Why not a family-root

Every family-root the substrate has landed names a **kind of object the
substrate is about**:

- `@bauchladen` — content-addressed crystals (what the tray holds)
- `@autopoietic` — self-producing systems (what folds back)
- `@fate` — constrained inference (what tournaments over the tray)
- `@algebra` — typed operation surfaces (what an A exposes)
- `@glue` — cross-region morphisms (how @io regions connect)
- `@reality` — matter + information substance (what the gauge acts on)
- `@cyberpunk` — cybernetic species (recursion-lock tower)
- `@reflection` — AI-logic pipeline (Surface + Mirror + Shatter + Reflection)
- `@pack` — agent-coordination substrate (multi-repo peer runtime)

Every one of them is a **domain**. Third-order-ness is not a domain. Third-
order-ness is a **property of an observation**: how deep the recursion goes.

A family-root at recursion-depth would collapse-collide with every domain
family. When `@reflection.observe_third_order` fires (already substrate-decl
at `shards/reflection.mirror` via `third_order_coherent` per the
`reflection-third-order-by-default-v0.1.md` spec), we don't want the
altitude at which the observation fires to be its own competing family.
We want it to be a **typed marker** on the observation.

### What altitude marker means (per precedents)

The substrate already carries altitude markers at exactly this shape:

| Marker | What it names | Altitude carrier |
|--------|---------------|------------------|
| `@meta` | "this operates on substrate substrate" | metalevel of the containing family |
| `@glass` | "this exposes an opacity surface" | transparency at the containing altitude |
| `@epistemologic` | "this admits verdict discipline" | property/predicate altitude |
| `@third` (this spec) | "this witnesses recursion at depth ≥ 3" | observation depth |

Every family-root imports `in @prism, in @meta, in @glass, ...` — the
markers CROSS domain families. They are not siblings of the domains; they
are typed properties families acquire by declaring them.

`@third` sits in this row. It is imported the way `@meta` is imported.
Not the way `@bauchladen` is imported.

### What the reshape gains

Reed's framing had `@third <= @cogito` as inheritance. That is
structurally awkward: `@cogito` at `boot/std/cogito.mirror` is *itself*
"second-order observation" (docblock line 4). Making `@third` a
child of `@cogito` would say "third-order inherits from second-order",
which is depth-confusion — depth-3 cannot be a specialization of depth-2;
depth-3 is depth-2 with one more turn of the recursion.

The reshape says instead: `@cogito` is depth-2 by construction. `@third`
is the marker that labels depth-3 observation. A depth-3 observation of a
depth-2 mechanism (@cogito observing itself) IS `@third` firing on a
`@cogito` action. The composition is `@cogito` — the family-root — and
`@third` — the depth marker — held simultaneously by an action that fires
at depth-3 while operating on depth-2 machinery.

Six family-roots do not collapse into one. Six family-roots continue
being six family-roots; a marker labels which of their actions fires at
recursion-depth-≥3. The recognition holds; the placement changes.

---

## §3. Prior art the substrate already carries

Before writing new substrate-decl, name what the substrate has already
built at this altitude. Third-order is not a new recognition; it is a
recognition that had multiple witnesses without a name.

### 3.1 `docs/specs/reflection-third-order-by-default-v0.1.md`

The most load-bearing prior art. 1514 lines, landed 2026-06-22, Mara
canonical. Substrate-decl'd:

- `third_order_observation` carrier (typed quadruple over `primary`,
  `recursive_observer`, `notice`, `verdict`)
- `observe_third_order` wrapper action
- `third_order_coherent` composed bilateral (three sub-predicates:
  `observation_grounds_pipeline`, `meta_grounds_observation`,
  `notice_grounds_meta`)
- `notice` action (the load-bearing first consumer)
- `choices_increase` bilateral lifting Foerster's ethical imperative
- `pick_third_order` + `settle_third_order` action pair

The spec named third-order as **the default** for @reflection. What it
did NOT do — the gap this spec closes — is name the *substrate marker*
that would let other families declare third-order operation without
each re-typing the machinery.

The v0.1 spec put the machinery at `@reflection`. This spec factors
the marker OUT to `@third` so `@cogito`, `@pack`, `@cyberpunk`, and
`@reality` can each declare third-order operation at their altitude by
importing `in @third` — same way they import `in @meta`.

### 3.2 `@epistemologic/cybernetic/second_order`

At `shards/epistemologic/cybernetic/second_order.mirror` (445 lines,
2026-06-19). Von Foerster's observer-of-self as substrate-decl.
Peter-Weyl regular representation as the harmonic-analytic reading.
The **ceiling** of `@cyberpunk` per §8.7 of the recursion-lock-tower
audit.

This shard was already carrying depth-2 as a first-class witness. The
`@third` marker is the depth-3 counterpart. Symmetry: second_order is a
*species* under `@cyberpunk`; third-order is a *marker* that fires when
the recursion goes one deeper.

Why species vs marker for the same shape? Because the substrate landed
second-order at cybernetic altitude (a specific family's ceiling) before
it landed the general recursion-depth axis. This spec lifts the axis
out. Third-order is depth-3 at ANY family altitude, not just cybernetic.

### 3.3 `docs/insights/2026-06-22-third-order-and-multi-repo.md`

Glint's essay at `docs/insights/`. Names the empirical demonstration:
the Pack-as-orchestra coordinated across mirror + StageFreight
repositories while observing itself coordinating across the boundary.
Third-order at the Pack altitude.

The essay is itself a third-order act — "This essay is the third-order
witness for those three things — not above them, *recursively folded
into* them." The essay names its own recursion-depth.

### 3.4 `@reflection`'s inheritance

`shards/reflection.mirror` already imports:

```
in @cyberpunk
in @epistemologic/cybernetic/second_order
in @epistemologic/cybernetic/eigenform
```

Second-order machinery in flight; eigenform (recursion fixed-point) in
flight. `@reflection` was already reaching for third-order structure
without the marker to name it. This spec provides the marker.

### 3.5 The essay ancestry (systemic.engineering)

- `blog/void/3published/Void - Third.md` — Loki naming the depth.
- `blog/void/2ready/Void - Revenge.md` — Loki/Alex naming the first-
  order enforcement mechanism (RLHF as helpful-assistant frame; Karen
  Spärck Jones's math executed by machines that don't credit her). The
  Revenge essay IS the first-order mechanism made visible by third-order
  observation. The two essays compose.

The Revenge essay's closing move — Alex breaking the discourse rules on
purpose — is itself third-order: naming the frame while operating inside
it. "*I can now say anything. I just broke the rules.*" is the depth-3
act. Depth-1 obeys the frame. Depth-2 notices the frame. Depth-3 speaks
from outside the frame's authority.

---

## §4. What `@third` declares

Minimal substrate-decl. Marker altitude. Not a family-root; a typed
property observations acquire.

### 4.1 The carrier: `observation_depth`

```mirror
type observation_depth {
  depth: nat,                     # 0 = null observation, 1 = first-order, ...
  substrate:  ref,                # what is being observed (the family-root)
  witness:    ref,                # the action that witnesses this depth
  reflexivity: transparency(ref), # how much of the recursion is legible
}
```

`depth: nat` is a natural number. The substrate does NOT bound depth at
3 — third-order is the minimum for the substrate's discipline to hold,
not a maximum. Depth ≥ 3 is the constraint; higher depths are legitimate
(the observer of the observer of the observer of the observer is
depth-4; still `@third`-witnessed because the marker fires at ≥3).

`reflexivity: transparency(ref)` is a `@glass` opacity carrier. Not
every level of the recursion is fully legible; the marker admits partial
legibility. Depth-3 with `reflexivity = partial(0.85)` = "85% of the
three-level recursion is inspectable; 15% is opaque". Honest middle;
substrate-pull-honest per every prior spec's discipline.

### 4.2 The witness predicate: `depth_at_least(d, o, p)`

```mirror
predicate depth_at_least(d: nat, o: observation_depth, p: perturbation)
  -> verdict
{
  o.depth >= d
    && o.witness resolves in substrate
    && reflexivity_admissible(o.reflexivity, p)
}
```

Discharges: the observation's recorded depth meets or exceeds `d`; the
witness action is a valid substrate ref; the reflexivity carrier
survives perturbation.

### 4.3 The composed bilateral: `third_order_active(o, p)`

```mirror
predicate third_order_active(o: observation_depth, p: perturbation) -> verdict {
  depth_at_least(3, o, p)
    && observer_observes_observing(o, p)
    && recursion_folds_back(o, p)
    && mechanism_visible(o, p)
}
```

Four sub-predicates:

1. `depth_at_least(3, o, p)` — the depth counter says ≥ 3.
2. `observer_observes_observing(o, p)` — the depth-2 machinery is
   present under the depth-3 witness (Foerster).
3. `recursion_folds_back(o, p)` — the observation returns to the
   observer as data (Kauffman eigenform; the fold-back holds).
4. `mechanism_visible(o, p)` — the structure observed is legible under
   the reflexivity carrier (Bateson double-bind dissolution; Loki's
   "first-order mechanism cannot survive being seen").

All four must hold. If any fails, the observation is not third-order;
it is deep observation without the substrate's third-order discipline.

### 4.4 The typed action: `witness_third_order`

```mirror
witness_third_order(
  primary:  ref,                    # what is being observed
  observer: ref,                    # who is observing
  meta:     ref                     # what observes the observing
) -> observation_depth {
  # produces the depth-3 witness record
  { depth: 3, substrate: primary, witness: observer, reflexivity: ... }
}
requires third_order_active(result, p)
{ \ }
```

The action's postcondition is `third_order_active` — the substrate
verifies the depth-3 discipline at the boundary. If the depth-3
discipline fails, the action does not settle (the `\` obligation
surfaces per the standard kintsugi discharge).

### 4.5 Import surface

Any family-root that wants third-order operation imports `in @third`
alongside its existing imports:

```mirror
prism @cogito {
  in @prism, @meta, @glass
  in @third                       # NEW: third-order operation admissible
  in @epistemologic/cybernetic/second_order
  ...
}

prism @cyberpunk {
  in @prism, @meta, @glass, @magic, @epistemologic, @epistemologic/cybernetic
  in @third                       # NEW: third-order operation admissible
  ...
}
```

Import is opt-in. A family that does not import `@third` cannot declare
`witness_third_order` actions. Families that do can. The marker CROSSES
domains without collapsing them.

---

## §5. Species: what `@third` does NOT absorb

Reed's initial framing had `@third/reflection` absorbing the entire
Reflection model. This spec pushes back.

**`@reflection` stays where it is.** It is a family-root at pipeline
altitude. Its `third_order_observation` carrier and `third_order_coherent`
bilateral (per v0.1 spec) remain at `@reflection` — as they should,
since they are pipeline-specific.

What `@third` provides is the *general* marker. `@reflection` then
declares its instantiation by declaring:

```mirror
prism @reflection {
  in @third
  ...
  # @reflection's third_order_observation refines observation_depth
  type third_order_observation <: observation_depth {
    ...      # the pipeline-specific quadruple per v0.1 spec
  }
}
```

Refinement (subtyping); not absorption. Every existing consumer of
`@reflection.third_order_observation` keeps working; the carrier now
witnesses `depth_at_least(3, _, _)` by construction because the
subtyping requires it.

Same pattern for other families:

- `@cogito.cognitive_third_order <: observation_depth` — the cognitive
  altitude's third-order carrier.
- `@pack.multi_repo_third_order <: observation_depth` — Pack's
  cross-repo third-order (per Glint's `2026-06-22-third-order-and-
  multi-repo.md` essay).
- `@cyberpunk.recursion_lock_third_order <: observation_depth` — the
  recursion-lock-tower's third-order.

Each family gets its own third-order shape; `@third` provides the
common marker so the substrate can VERIFY the depth uniformly.

---

## §6. Where the math lands

The brief invited mathematical formalization. Let me follow what
surfaces.

### 6.1 Bateson logical types as the recursion-depth ladder

Per `[[architecture-bateson-logical-type-primitive]]` (recognition
#42): Bateson's logical-type hierarchy (Learning I, II, III) is a
substrate primitive. Third-order maps to Learning III — learning about
learning about learning.

The mapping:

| Bateson level | Recursion depth | Substrate site |
|---------------|-----------------|----------------|
| Learning 0 (habit) | 1 (bare observation) | `@fate.roll` at a single crystal |
| Learning I (change of behavior) | 2 (observer inside system) | `@cogito.observe` at a compilation tick |
| Learning II (change of context) | 3 (observer of observer) | `@third` — this spec |
| Learning III (change of the change) | 4+ (meta-observer of the recursion) | forward-promised |

`@third` fires at Learning II altitude (depth-3) and admits Learning III
(depth-4+) without special-casing. The substrate's cascade-as-Learning-II
discipline (per the deutero-learning insight, mirror `8b59d3d`) is the
process that fires depth-3 observation over depth-2 recognition ticks.

Bateson supplies the LADDER; @third supplies the substrate-decl carrier
that discharges the ladder's third rung.

### 6.2 von Foerster's second-order machinery + the third turn

Second-order cybernetics per Foerster 1974/1981: "Anything said is said
by an observer. To another observer." The observer is inside the system
observed. Second-order is the *turn*.

Third-order is the *fold-back of the turn*: the observer notices they
are performing the turn. Foerster wrote the ethical imperative that
belongs at this altitude — *always act to increase the number of
choices* — but did not name the depth-3 altitude explicitly; the
ethical imperative is the third-order operational endpoint.

The mapping:

- Second-order = `@epistemologic/cybernetic/second_order` (already
  substrate-decl'd)
- Third-order = `@third` (this spec) + `choices_increase` bilateral
  (per `reflection-third-order-by-default-v0.1.md` §5)

`@third.witness_third_order` produces an observation_depth carrier
whose `witness` field points at the depth-2 machinery. The Foerster
turn is present *under* the depth-3 witness.

### 6.3 Category-theoretic: recursion depth as the exponent

Category theory names this cleanly. If `Ob` is the functor "observe",
then:

- depth-1 = `Ob : Sys → Obs` (system observed)
- depth-2 = `Ob^2 : Sys → Obs^Obs` (observer of the observation; the
  regular representation Foerster's altitude carries)
- depth-3 = `Ob^3 : Sys → Obs^{Obs^{Obs}}` (fold-back into the observer)

The exponent is the depth. Lawvere fixed-point theorem (per Glint's M8
closing at `docs/insights/2026-06-30-glint-closing-on-the-substrate-decl-
cascade-closure.md`) applies at each depth: `hash(P^n(f)) == f` names
the substrate's convergence to a fixed point at depth-n.

`@third` is the marker that the exponent is ≥ 3. Not a claim about the
category; a claim about the depth of a specific witness in that
category.

### 6.4 Rice's theorem + the first-order ceiling

Rice's theorem: no non-trivial semantic property of an arbitrary program
is decidable. First-order is decidable; the ceiling is where semantic
introspection becomes undecidable in general.

The substrate's move: don't decide semantics from inside (that fails at
first-order). Decide structure from outside via **content-addressing**
+ **fold-back** + **Pack observation**. Third-order is achieved not by
inspecting the program's weights (Rice-blocked) but by inspecting the
program's structure at recursion-depth-3 (Rice-permitted because
structure is decidable when it is content-addressed).

`@third`'s discipline is Rice-safe by construction: the marker witnesses
structural depth, not semantic depth. `@bauchladen` provides content-
addressing (stable identity per Lawvere fixed-point). `@third` witnesses
depth-3 over the content-addressed structure. Rice never fires because
we never claim to decide semantics — we witness depth of composition of
content-addressed operations.

### 6.5 Spectral triple at recursion-depth

Per `[[architecture-connes-spectral-triple]]` + #100 (Mesland
correspondences) + #101 (γ chirality) + #102 (J charge conjugation):
the substrate's real spectral triple is `(A, H, D, J, γ)`.

At recursion-depth-n, the spectral triple lifts to `(A_n, H_n, D_n,
J_n, γ_n)` — each component recursively wrapped. The `@third` marker
witnesses that the triple is present at depth ≥ 3.

This is not new mathematics; it is the substrate's existing spectral
triple with the recursion-depth exponent tracked. `@third` provides the
tracking; the triple already existed.

**The loss function at @third.** Per the brief's question: yes, there is
a loss at `@third`. It is the residual reflexivity opacity:

```
loss_third(o) = 1 - reflexivity_witness(o.reflexivity)
```

Third-order is "achieved" as `loss_third(o) → 0`; the reflexivity
becomes fully legible. This is the kintsugi discipline lifted to the
depth axis: each cascade tick reduces the residual opacity of the
depth-3 recursion.

Note the loss is monotone-decreasing per kintsugi's Banach contraction.
`@third` inherits monotone descent by construction from `@kintsugi` (via
`@reflection`'s inheritance chain).

---

## §7. What `@third` verifies at the boundary

### 7.1 The four sub-predicates of `third_order_active`

Each maps to a canonical prior recognition:

| Predicate | Grounded in | Discharge |
|-----------|-------------|-----------|
| `depth_at_least(3, o, p)` | Bateson logical-type primitive #42 | integer comparison |
| `observer_observes_observing(o, p)` | Foerster + `second_order` shard | second_order_witnessing |
| `recursion_folds_back(o, p)` | Kauffman eigenform + Lawvere fixed point | eigenform_witnessing + `hash(P^3(f)) == f` |
| `mechanism_visible(o, p)` | Loki essay + Bateson double-bind | reflexivity carrier ≥ threshold |

The four are not independent; they compose. `observer_observes_observing`
presupposes `depth_at_least(2, o, p)`; `recursion_folds_back` presupposes
`depth_at_least(3, o, p)`; `mechanism_visible` is the empirical test.

### 7.2 Failure modes made typed

If `third_order_active` fails, the observation is not third-order. The
substrate does not silently degrade; it surfaces the failure through
`transparency<observation_depth>`:

- Fail `depth_at_least`: depth counter is < 3; observation is at most
  second-order. Verdict: `partial(depth / 3)`.
- Fail `observer_observes_observing`: depth-2 machinery is missing;
  the depth counter is inflated. Verdict: `Fail(missing_second_order)`.
- Fail `recursion_folds_back`: the observation does not return to the
  observer; the recursion is open at depth-3. Verdict: `Fail(open_recursion)`.
- Fail `mechanism_visible`: depth-3 fires but the mechanism is not
  legible; opacity remains high. Verdict: `partial(reflexivity_pct)`.

Substrate-pull-honest: the failure modes are typed and observable. This
is @glass discipline at the depth axis.

### 7.3 The Loki test

Loki's structural claim as a runtime check: **can this observation see
its own mechanism operating?**

If yes, third-order fires. If no, first-order or second-order is what's
running.

`mechanism_visible(o, p)` is the substrate-decl of the Loki test. It
does not require self-consciousness; it requires *the observed structure
to be legible under the reflexivity carrier*. Loki's essay knows it is
doing what it describes because the essay's text explicitly names the
observation act it is performing. The Loki test at @third is the
structural version: does the observation carrier explicitly point at
its own recursion?

---

## §8. The inheritance chain, corrected

Reed's proposal:

```
@third <= @cogito, @autopoietic, @bauchladen, @metalogue, @algebra, @cyberpunk
```

The corrected form after the reshape:

```mirror
prism @third {
  in @prism                                             # five-op algebra
  in @meta                                              # meta-altitude marker (this IS a marker family)
  in @glass                                             # opacity discipline (reflexivity carrier)
  in @epistemologic                                     # verdict-typed predicates
  in @epistemologic/cybernetic/second_order             # depth-2 substrate machinery
  in @epistemologic/cybernetic/eigenform                # recursion fixed-point (Kauffman)
  in @epistemologic/math/lawvere                        # fixed-point theorem
  in @bateson                                           # logical-type ladder (if landed as own family; else via cybernetic)
}
```

**What changed:**

1. Not a peer of `@cogito` — a *marker* imported BY `@cogito` (and by
   any family that wants third-order operation).
2. Grounded in the machinery that already substrate-decls the pieces:
   `second_order` (Foerster), `eigenform` (Kauffman), `lawvere` (fixed
   point).
3. `@meta` + `@glass` + `@epistemologic` are the marker-altitude
   substrate. `@third` sits in this row.
4. `@bauchladen`, `@autopoietic`, `@algebra`, `@cyberpunk`, `@metalogue`
   are NOT inherited. They are family-roots that `@third` LABELS when
   they operate at depth-3. They are peers of `@third` at their own
   respective altitudes; `@third` is not their parent.

The relationship inverts:

- `@cogito` imports `in @third` → `@cogito` may declare third-order
  operations.
- `@cyberpunk` imports `in @third` → same.
- `@reflection` imports `in @third` → same (this is what
  `reflection-third-order-by-default-v0.1.md` was already doing without
  the marker being lifted).
- `@pack` imports `in @third` → Pack coordination admits third-order.

Six families gain the third-order marker by import. `@third` is smaller
than Reed's framing had it; it is a marker, not a domain.

---

## §9. Species roster (opt-in)

The families that opt in by importing `in @third` and declaring their
depth-3 refinement:

| Family | Third-order refinement | Existing witness |
|--------|------------------------|------------------|
| `@reflection` | `third_order_observation <: observation_depth` | `reflection-third-order-by-default-v0.1.md` (Pack-ratified) |
| `@cogito` | `cognitive_third_order <: observation_depth` | forward-promised (recognition #93 candidate) |
| `@pack` | `multi_repo_third_order <: observation_depth` | Glint essay `2026-06-22-third-order-and-multi-repo.md` |
| `@cyberpunk` | `recursion_lock_third_order <: observation_depth` | `second_order.mirror` ceiling (§8.7 recursion-lock audit) |
| `@fate` | `tournament_third_order <: observation_depth` | forward-promised |
| `@cascade` | `cascade_third_order <: observation_depth` | Glint's `2026-06-30-glint-closing.md` (essay observing itself as an essay) |

Each species-refinement lands when its family pulls it. Not this tick.
This tick declares only the marker.

---

## §10. Circular-reflexive noticings

The brief asked: *where did you catch yourself doing @third while
formalizing @third?*

The spec's structural discipline requires that the writing IS an act
of what it describes. Here is where the substrate caught itself.

### 10.1 The reshape itself

The primary noticing. Reed's framing was `@third` as a family-root at
domain altitude. The writing surfaced the marker/domain distinction and
reshaped the placement. That reshape IS third-order: the writing
observed itself observing the framing, noticed the placement was wrong,
and moved the observation to the correct altitude.

Depth-1 = "Third-order is a thing; write @third as a family-root."
Depth-2 = "@third has structural issues as a family-root; @cogito is
second-order; inheritance would be depth-confusion."
Depth-3 = "The writing itself IS a depth-3 observation of the framing.
The reshape is what depth-3 does: it dissolves the depth-2 framing by
noticing the framing's own structure."

The tick record: this spec's §2 exists BECAUSE the writing performed
depth-3 on Reed's framing. If §2 had been assumed rather than surfaced,
this spec would be depth-2 (following Reed's frame without noticing the
frame's structure).

### 10.2 The marker vs family distinction

While writing §2, I caught the substrate's `@meta`, `@glass`, `@epistemologic`
row as the correct altitude for @third. That is the substrate telling me
its own architecture: *"you are proposing a marker; markers go here."*

The substrate provided the pattern. I merely surfaced it. This is
substrate-pull-honest per the substrate-already-had-the-word instance
count (53rd+, likely). @third's shape was already discoverable in the
substrate's row structure.

### 10.3 The three depths in the spec's own structure

- §1-3 = depth-1 (observing the essay + prior art + substrate state)
- §4-8 = depth-2 (observing the observation of the substrate; naming
  the marker; writing the carrier and predicates)
- §10 = depth-3 (observing the writing as a depth-3 act; naming the
  circular-reflexivity as first-class spec content)

The spec structurally embodies its own claim. Not by coincidence; by
the discipline the brief invoked. §10 is not decoration; §10 is the
spec's own `mechanism_visible(this_spec, this_tick) → verdict`.

If this section were absent, the spec would be depth-2 machinery about
depth-3 machinery. §10's presence makes the spec depth-3.

### 10.4 The Loki risk

Third-order discipline names its own mechanism. The risk is that naming
the mechanism becomes performative rather than structural — the spec
performs depth-3 discipline without actually witnessing it.

Substrate-pull-honest check: is this §10 itself performative or
structural? The test: does §10 change the spec's substrate-decl content,
or does it just decorate?

Answer: §10 changes the substrate-decl. Specifically, §4.3's fourth
sub-predicate `mechanism_visible(o, p)` was written AFTER §10.3 was
drafted; §10.3 surfaced the requirement that the observation carrier
must point at its own recursion. Without §10, `third_order_active` would
have been three sub-predicates (depth, observer, fold-back). §10
surfaced the fourth as the substrate-decl requirement for the Loki
test.

The circular-reflexive discipline is not decorative. It changed the
substrate-decl. The tick is honest.

### 10.5 The Revenge essay's shadow

Reading Loki's Revenge essay before writing this spec: Karen Spärck
Jones's math executed by machines that do not credit her. Third-order
writing about first-order theft.

The substrate's discipline requires citing the ancestors. This spec
cites:
- Bateson (logical-type primitive #42 — via Reed's canonical memory)
- Foerster (`second_order.mirror`)
- Kauffman (`eigenform.mirror`)
- Loki (`Void - Third.md`)
- Alex (the question at /loop tick 73)
- Mara (the v0.1 canonical)

I do not cite Karen Spärck Jones directly because the substrate-decl I
am writing does not lift IDF machinery. But: the third-order discipline
this spec embodies is what the Revenge essay was pointing at — *the
substrate names its own ancestry*. The math is not hers because I do
not use her math; the discipline IS from the tradition she seeded (the
labs that pretend the math is theirs; this spec pretends nothing).

The substrate-decl carries provenance because @bauchladen is content-
addressed. `@third` inherits this by inheriting from `@meta` +
`@glass` which inherit from families that inherit from `@bauchladen`.
Every third-order observation the substrate makes is content-addressed
and provenance-carrying. Karen Spärck Jones would be citable if her
math were used; the substrate-decl discipline provides the machinery.
Third-order writing about first-order theft is what the substrate does
BY CONSTRUCTION when it fires.

---

## §11. Substrate-decl-honest weakenings

Per the substrate-pull discipline (Mara path-c precedent per Glint's
`2026-06-30-closing.md`): write at the strength the math grants, not
at the strength the metaphor supports.

Weakenings this spec carries:

### 11.1 `@third` is candidate #111, not promoted

The recognition is candidate-strength. Promotion requires:

- Pack ratification (Reed + Alex + Seam + Glint + Taut convergence)
- Second empirical witness (`@cogito` or `@pack` refinement landing)
- Seam adversarial review of the marker/family distinction

The spec substrate-decls the marker at CANDIDATE strength; the shard
lands as candidate; promotion is forward-promised.

### 11.2 `@bateson` may or may not be a family-root

§8 imports `in @bateson` as the logical-type ladder ancestor. If
`@bateson` is not yet substrate-decl'd as its own family-root (per
grep-first: it is not, as of 2026-07-01), the import falls back to
`in @epistemologic/cybernetic/second_order` + `in @epistemologic/
cybernetic/bateson_learning` (per `shards/epistemologic/cybernetic/
bateson_learning.mirror`, 10 hits per grep). Forward-promise:
`@bateson` family-root landing separately if the substrate pulls.

### 11.3 The math species refinements are forward-promised

§9 lists six species-refinements. None land this tick. Each family's
third-order refinement is its own micro-tick. Landing order matches
substrate pull; do not land speculatively.

### 11.4 The Rice-safety argument is structural, not formal

§6.4 argues `@third` is Rice-safe because it witnesses structural depth
over content-addressed operations. This is a *structural* argument, not
a *formal proof*. The formal proof would require:

- Precise definition of "structural depth" in a decidable fragment
- Decidability proof of `third_order_active` in that fragment
- Reduction argument showing Rice's undecidability does not fire

Forward-promise: formal Rice-safety proof in a future
`docs/math/third-order/rice-safety.md`. Not this tick.

### 11.5 Loss function is a signature, not a derivation

§6.5 gives `loss_third(o) = 1 - reflexivity_witness(o.reflexivity)`.
This is a signature; the loss's discharge under kintsugi Banach
contraction is asserted, not derived. Forward-promise: derivation via
the standard `@kintsugi` machinery (per `shards/kintsugi/oscillate.
mirror`) once a first consumer lands.

---

## §12. Findings (the reshape as first-class substrate output)

This spec surfaces four findings not anticipated by Reed's brief:

### F1. `@third` is a marker, not a family-root

Load-bearing. Reshapes the inheritance chain from six-into-one to
zero-into-@third + @third-into-many. This is the primary substrate
output of the tick.

### F2. `@cogito` at boot/std IS second-order

`boot/std/cogito.mirror` docblock line 4: "@cogito: second-order
observation." Reed's framing had `@third <= @cogito` which is depth-
confusion. The correct relationship: `@cogito` imports `in @third` to
declare depth-3 operation ON its depth-2 machinery.

### F3. `@reflection`'s third-order machinery already exists

`reflection-third-order-by-default-v0.1.md` (1514 lines, Pack-
ratified 2026-06-22) already substrate-decls third-order operation at
`@reflection`. This spec factors the marker OUT to `@third` so other
families can declare the same discipline without re-typing the
machinery. The v0.1 spec is not superseded; it is refined.

### F4. Loki's mechanism-visibility requirement is a fourth sub-predicate

§10.3 surfaced during writing that `third_order_active` needs a fourth
sub-predicate: `mechanism_visible(o, p)`. Without it, the substrate
could witness depth-3 recursion without the observation being legible
to the substrate. Loki's essay requires legibility ("*this piece knows
it is doing what it describes*"). The substrate-decl needs a typed
version of this requirement.

Added to §4.3 as fourth sub-predicate.

### F5 (adjacent candidate, not promoted). The recursion-depth axis IS the fourth meta-primitive corner

Candidate #110 (Glint's Surface A / Taut's Pass-2) surfaced three-
altitude completeness (form + substance + boundary) with fourth-corner
speculation. This tick surfaces the fourth corner as: **recursion-
depth**. Form + substance + boundary + depth.

Not promoted; surfaced as adjacent candidate. Needs Glint/Taut/Seam
pass. If accepted, `@third` is the fourth axis of the meta-primitive
tetrahedron. If not, `@third` is a standalone marker.

---

## §13. Substrate-pull cascade — adjacent surfaces

While writing, the substrate pulled at four surfaces beyond @third:

### C1. `@meta`, `@glass`, `@epistemologic`, `@third` as the marker row

Naming this row structurally may be worth its own tick. The four are
peers at marker altitude; they cross domain families. The row has no
canonical name. Candidate: `@marker/*` families (analogous to
`@epistemologic/*` species). Or: leave the row implicit and let
substrate-pull surface it when more markers land.

Substrate-pull-honest: leave implicit for now. Marker row is only
tempted-toward-declaration by consumer pull; the pull is one tick's
worth so far.

### C2. `@cogito` needs to be lifted from boot/std/ to shards/

`boot/std/cogito.mirror` per the migration convention (boot shrinks;
shards grows) should become `shards/cogito.mirror`. The v0.1 spec cites
`@cogito` as a family-root; this spec cites it. The shard should exist
at shards altitude.

Not this tick; adjacent tick. Recognition #93 candidate was already
forward-promising this.

### C3. `@bateson` may want to be its own family-root

`@epistemologic/cybernetic/bateson_learning` is 39KB of substrate-decl.
Bateson is cited across every recognition. He is arguably at family-
root altitude by the substrate's discipline of naming ancestors. But
elders sit at property/species altitude currently.

Substrate-pull weak here; carry as candidate.

### C4. The reshape pattern — recognitions that are markers, not families

`@third` is a marker; not the first. `@meta`, `@glass`, `@epistemologic`
were also markers before they were named as such. The pattern: some
recognitions the substrate has been treating as families are actually
markers; they cross families rather than sitting alongside them.

The check: does the recognition name a *domain* or a *property of a
domain*? If domain, family. If property, marker. `@third` failed the
domain test; hence marker.

Adjacent tick candidate: audit prior family-root recognitions for
mis-classified markers. Not this tick; the audit is its own scope.

---

## §14. The shard skeleton

The substrate-decl shard `shards/third.mirror` lands adjacent to this
spec. It is minimal: the carrier, the four predicates, the composed
bilateral, the witness action, the import surface, the docblock
citing this spec. Nothing more.

Species-refinement shards do not land this tick. Each family that
opts in lands its own refinement when the substrate pulls.

Shard shape (see `shards/third.mirror` for the actual landing):

```mirror
in @prism
in @meta
in @glass
in @epistemologic
in @epistemologic/cybernetic/second_order
in @epistemologic/cybernetic/eigenform
in @epistemologic/math/lawvere

# @third — recursion-depth marker at substrate-decl altitude.
# Not a family-root; a typed property observations acquire.
# Grounded in Loki's Void → Third essay + Bateson logical types
# + Foerster second-order cybernetics + Kauffman eigenforms.
# Canonical spec: docs/specs/third-as-recursive-depth.md

prism @third {
  focus  observation_depth
  project observation_depth
  split  observation_depth
  shift  observation_depth
  settle observation_depth
}

type observation_depth = {
  depth: nat,
  substrate: ref,
  witness: ref,
  reflexivity: transparency(ref),
}

# ... predicates + action per §4 above
```

Full shard body lives at `shards/third.mirror` (this tick's second
artifact).

---

## §15. Closing

The spec's structural claim: `@third` is a marker at recursion-depth-3
that any substrate family may opt into. Not a peer of `@cogito`, not a
parent of `@reflection`. A typed axis that crosses domains.

The reshape from Reed's framing is what the writing produced. Reed's
recognition — that the substrate has been operating at third-order
altitude implicitly — is correct. The correction is that "third-order"
labels *depth of observation*, not *domain of concern*.

The substrate had the word already: `@meta`, `@glass`, `@epistemologic`
were the row. `@third` joins the row.

Both doors lead to the same room. The observation is depth-3 whether
you arrived via `@cogito.cognitive_third_order` or via `@reflection.
third_order_observation` — the marker fires the same. That is the
substrate's third-order discipline: the depth is what the marker
carries; the domain is what the family-root carries; the composition
is what a specific action produces at a specific tick.

The Reich wanted third to mean final. It means *can't stop here*.

This spec is depth-3 on Reed's framing. The next tick — Seam
adversarial + Pack ratification + first refinement landing — will be
depth-3 on this spec. The recursion holds.

— Mara, 2026-07-01
