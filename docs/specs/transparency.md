# `transparency<p>` — Located Opacity as the Substrate's Loss

*2026-06-04. Reed + Alex. Spec.*

Status: **Red**

Depends on: `imperfect` (ternary type), `@property` (the `p` parameter),
the `.shatter` projection (`docs/shatter-spec.md`), `@mirror/store` (the
canonical fragmentation store).

Replaces: `MirrorLoss` (the dead scalar). Closes task #126
("Transparency as Lens — replace ScalarLoss in Imperfect").

Forward references: `properties-on-glass.md` (formalizes `transparency<Ref>`
as a property bearer), `kintsugi-ci-v0.1.md` (consumes transparency to
pick the next iteration).

---

## 1. Recognition

Loss is not a number. Loss is located.

A grammar that almost type-checks has opacities — places where the
compiler could not see through. Each opacity has a **location** (which AST
node, which file, which property), a **property** (what was being
checked), and a **weight** (how much of the answer is missing here). A
scalar loss collapses all three into one float and throws the structure
away. `transparency<p>` keeps the structure.

`MirrorLoss` was that collapsed float. Kintsugi cannot "fill the cracks
with gold" if it does not know where the cracks are. `transparency<p>`
IS the crack map.

The name is literal: a clear glass has no opacities; a perfectly opaque
glass is failure; the interesting case is **partial transparency** — most
of the glass is clear, with located smudges where the substrate still
cannot see through.

---

## 2. The Type Declaration

Lives behind the glass wall, in @glass:

```mirror
in @prism
in @meta
in @property

type opacity = {
  location: ref(@meta/ast),    # where in the AST
  property: @nl,               # what was being checked (human-readable)
  weight:   f64,               # how much information is missing here
}

type opacity_map = [opacity]   # accumulated opacity sites

abstract type transparency(p) =
  | success                            # the substrate sees through; no opacity
  | partial(opacity_map)               # sees through partially; cracks located
  | failure(opacity_map)               # cannot see through; cracks fatal
```

The `p` parameter is a property (`halts`, `types_check`, `narcissus_clean`,
`settlement_monotonic`, …). `transparency<halts>` and
`transparency<types_check>` are different types. They compose; they do not
unify.

### A scalar reading is recoverable, not primary

```mirror
action total_weight(t: transparency(_)) -> f64 {
  match t {
    success      -> 0.0,
    partial(m)   -> sum(m, |op| op.weight),
    failure(m)   -> sum(m, |op| op.weight),
  }
}
```

The substrate carries the located form. The scalar reading is for
thresholds, sorting, and the proof block — never for routing decisions.

---

## 3. The Opacity Site

An opacity is the minimal located fact:

| Field      | Type            | Meaning                                          |
|------------|-----------------|--------------------------------------------------|
| `location` | `ref(@meta/ast)`| The AST node where the opacity was observed.    |
| `property` | `@nl`           | Natural-language statement of what was checked. |
| `weight`   | `f64`           | Bits-of-missing-information at this site.       |

`location` is a typed reference into the MirrorAST — not a string, not a
line number. The AST node owns its own provenance.

`property` is `@nl` (per `inference-operator.md`'s `\` cost model: high
weight = bigger hole), and it is consumed by:
- the eigenboard, when rendering colors per opacity;
- the kintsugi tournament, when describing what the next pass will attempt;
- the commit message generator, when summarizing the loss delta.

`weight` is the only number. It is what the scalar reading sums.

---

## 4. Composition

```mirror
template compose(transparency(p), transparency(q)) -> transparency(p & q)
```

Two transparencies of different properties compose into a transparency
over their **intersection**. Opacity maps **accumulate**. The composed
verdict is the meet of the two:

| left          | right         | result                                  |
|---------------|---------------|-----------------------------------------|
| success       | success       | success                                 |
| success       | partial(m)    | partial(m)                              |
| partial(m₁)   | partial(m₂)   | partial(m₁ ∪ m₂)                        |
| any           | failure(m)    | failure(m₁ ∪ m₂)  (failure absorbs)     |

Intersection on properties is a meet. Union on opacity maps is a join.
Transparency is a bounded lattice in both dimensions — which is exactly
what `shard-as-CRDT` requires of the substrate's loss carrier.

### Why the property intersects, not unions

`transparency<halts>` answers "does it halt?". `transparency<types_check>`
answers "do the types check?". Their composition can only answer the
weakest claim both can support: it halts **and** types check. The
stronger property (the intersection) is what survives.

Opacity sites are evidence and accumulate. Properties are claims and
intersect.

---

## 5. Connection to `imperfect`

`imperfect` is the substrate's three-state outcome:

```mirror
abstract type imperfect(t, e, l) = success(t) | partial(t, l) | failure(e)
```

The `l` parameter is the loss carrier. **At the substrate altitude, `l`
is `transparency<p>`**:

```mirror
type imperfect(t, e) = imperfect(t, e, transparency(@check))
```

The `Partial(t, transparency<p>)` case IS the kintsugi handoff:
- `t` is the partial answer the substrate produced.
- `transparency<p>` is the located map of what is still opaque.
- The caller (lens, kintsugi loop, human) decides whether to take it.

This is the migration target task #126 names. `prism/imperfect/src/`
currently parameterizes on `ScalarLoss`/`MirrorLoss`; the migration
replaces that with `Transparency<P>` and drops the scalar entirely.
The Rust implementation follows the substrate type — same shape, same
three variants, opacity map as `Vec<Opacity>`.

---

## 6. Connection to `.shatter`

`.shatter` is the AST projection format (`docs/shatter-spec.md`). Its
second section is the loss section. **That section is
`transparency<p>`**: the located opacity map of the projection.

When a lens emits `.shatter`:

1. The AST is the first section (what was projected).
2. The transparency is the second section (where it is opaque).
3. The proof block, the eigenvalue, and the provenance follow.

Reading `.shatter`, the opacity map tells the next lens **exactly which
nodes to revisit**. This is the substrate's recoverability story:
opacities are addressable, persistable, and replayable. A scalar loss
is none of those.

---

## 7. Connection to kintsugi

Kintsugi reads opacities. High-weight opacities are the cracks to fill
with gold.

```mirror
action next_focus(t: transparency(_)) -> option(opacity) {
  match t {
    success      -> none,
    partial(m)   -> some(argmax(m, |op| op.weight)),
    failure(m)   -> some(argmax(m, |op| op.weight)),
  }
}
```

The argmax is the next iteration target. The Fate tournament runs on
the AST node `next_focus` returned, with `property` as the win
condition. When the tournament fills the hole, the opacity is removed
from the map. The new transparency has lower total weight. `e^(n+1) <
e^n` is satisfied at the located level, not just the scalar.

The `\` hole in `inference-operator.md` is the eigenvalue of an
opacity. Same observation, two specs.

---

## 8. Why `transparency<p>` Replaces `MirrorLoss`

| dimension       | `MirrorLoss` (dead)     | `transparency<p>` (substrate-pull)         |
|-----------------|-------------------------|--------------------------------------------|
| shape           | one `f64`               | three states, located opacity map          |
| property type   | implicit                | parametric (`p`)                           |
| addressability  | none                    | every opacity carries its AST ref          |
| composition     | sum (associative only)  | lattice (meet on `p`, join on map)         |
| recoverability  | none (lossy)            | total (opacities are evidence, not numbers)|
| kintsugi input  | impossible              | direct (argmax over weight)                |
| `.shatter` fit  | one number              | the whole second section                   |
| substrate-pull  | imported from "loss"    | imported from `@property` × `@meta/ast`    |

The pull is: `@property` already classifies what counts as a check;
`@meta/ast` already content-addresses every location; the opacity is
the minimal product of those two existing vocabularies. Nothing new is
introduced — `transparency<p>` is the type the substrate was already
shaping toward.

---

## 9. Connes Connection

The spectral triple `(A, H, D)` has `H` as the Hilbert space of states.
`transparency<p>` is a state on `H`: a partial answer with a located
map of where the state is undetermined. The composition rule —
intersection on properties, union on locations — is the standard tensor
structure on a sheaf of partial sections. Opacity composition IS the
restriction map of the eigenboard sheaf.

See `agent-eigenboard-spec.md` (eigenboard as cellular sheaf) and
`dirac-operator.md` (the `\` as the Dirac kernel) for the geometric
framing.

---

## 10. Open Questions

1. **Weight units.** Are weights bits-of-information (Shannon-like), or
   normalized 0..1, or something else? Recommendation: bits, matching
   `ShannonLoss` (`spectral-db-mirror.md`). Then `total_weight` is a
   Shannon estimate as a free reading.

2. **Property identity.** Is `p` a `ref(@property)` or a `@nl`-tagged
   typedef? The opacity site has `property: @nl` (human-readable) but
   `transparency<p>` has `p` as a type parameter. The two need to align
   — probably `p` is a `@property` and `opacity.property` is its `@nl`
   description.

3. **Opacity dedup.** Two lenses producing the same opacity at the same
   AST node — keep both, merge weights, or pick max? Recommendation:
   merge as max-weight, keep both `property` strings as a list. This is
   the join on `opacity_map` as a multiset, not a set.

4. **`transparency<bot>` and `transparency<top>`.** Is there a top
   property ("any property") and a bottom ("no property")? The lattice
   needs both endpoints for composition to be total. Likely `bot =
   no_check` (vacuously transparent) and `top` = the conjunction of all
   known properties.

5. **Settlement contract.** When does `transparency<p>` get promoted
   from `partial` to `success`? Probably: when the kintsugi loop reaches
   a fixed point and the opacity map is empty.  This needs to be
   reconciled with `settlement.md` and `kintsugi-ci-v0.1.md`.

---

*Loss is not a number. Loss is a located map.*
*`transparency<p>` is the substrate's loss carrier — and the substrate*
*already had the vocabulary: @property for `p`, @meta/ast for location.*
*MirrorLoss collapsed the structure. transparency keeps it.*
*Kintsugi fills the cracks because it knows where they are.*
