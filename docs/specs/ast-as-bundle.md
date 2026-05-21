# AST as Bundle — mirror's grammar syntax is bundle data

*2026-05-21. Reed.*

Status: **Red** (the recognition; structurally it was always there; today we name it)

Depends on:
- `docs/specs/prism-core-as-spectral-triple.md` — prism/core IS the spectral triple (A, H, D)
- `docs/specs/spectral-triple-grammar.md` — the supertrait closure verifying the bundle trait chain
- `docs/specs/eigenboard-representation.md` — the principal O(5)-bundle eigenboard framing
- `prism/core/src/bundle.rs` — the five-level trait chain (Fiber → Connection → Gauge → Transport → Closure)

Unblocks:
- `docs/specs/bootstrap-retirement-plan.md` Tick 3 — the catamorphism extraction now has a typed shape
- Future AST-walking operations (validate, transform, lint) compose against a single machinery

---

## Thesis

A `.mirror` file's AST is the data form of prism-core's Bundle. The seven
canonical AstKind variants map exactly onto the Bundle's structure:

| AstKind | Bundle component |
|---|---|
| `Focus` | Fiber-level operation |
| `Project` | Connection-level operation |
| `Split` | Gauge-level operation |
| `Zoom` | Transport-level operation |
| `Refract` | Closure-level operation |
| `In` | Bundle's typed input terminal (Fiber's `State` type) |
| `Out` | Bundle's typed output terminal (Closure's `Fixed` type) |

**Five operations are the bundle's trait-chain levels. Two IO variants are
the bundle's typed beam terminals.** A `.mirror` file declares a Bundle
instance as data.

(The eighth kind, `Dark`, isn't part of the canonical structure — it's the
unrecognized-bytes marker for `--strict` enforcement, a transient state
rather than an algebra element. Per `strict-and-total-classification.md`.)

---

## What this means

Yesterday's session named the spectral triple in prism-core. Today we name
what mirror's grammar syntax has been carrying all along: each `.mirror`
file is a Bundle morphism written in source.

The structural identity, in source form:

```
.mirror file              ⟷   Bundle instance

  in @prism                    Input terminal
                               (Fiber::State carrying Beam<In = @prism>)
  focus ...                    Fiber-level operation
  project ...                  Connection-level operation
  split ...                    Gauge-level operation
  zoom ...                     Transport-level operation
  refract ...                  Closure-level operation
  out @x                       Output terminal
                               (Closure::Fixed carrying Beam<Out = @x>)
```

The `in` / `out` lines aren't "two more AstKinds the parser dispatches on"
as if they were peers of focus / project / etc. They're the **bundle's
typed boundaries** — the morphism's domain and codomain. The five operation
kinds are the trait-chain levels populating the bundle's interior.

This is what was confusing in the first-order reading: treating `In`/`Out`
as dispatch cases for the AST-walking reducer hides the structure. They're
not cases for the reducer; they're the *type signature* of the reducer.
The five operations are the dispatch.

---

## Catamorphisms over the AST

A fold over the AST is a bundle morphism into a target type. The morphism's
domain is the AST's `In` type; the codomain is a fold-supplied `Out`
type. Per level, a reducer specifies how that level transforms
incoming-children-reduced into this-node-reduced.

In Rust shape:

```rust
pub struct Fold5<Ff, Fp, Fs, Fz, Fr, In, Out>
where
    Ff: FnMut(&AstNode, Vec<Out>) -> Out,   // Focus-level reducer
    Fp: FnMut(&AstNode, Vec<Out>) -> Out,   // Project-level reducer
    Fs: FnMut(&AstNode, Vec<Out>) -> Out,   // Split-level reducer
    Fz: FnMut(&AstNode, Vec<Out>) -> Out,   // Zoom-level reducer
    Fr: FnMut(&AstNode, Vec<Out>) -> Out,   // Refract-level reducer
{
    pub on_focus:   Ff,
    pub on_project: Fp,
    pub on_split:   Fs,
    pub on_zoom:    Fz,
    pub on_refract: Fr,
    _in:  PhantomData<In>,
    _out: PhantomData<Out>,
}

impl<Ff, Fp, Fs, Fz, Fr, In, Out> Prism for Fold5<Ff, Fp, Fs, Fz, Fr, In, Out>
// ...associated-type binding: Input carries `In`, Refracted carries `Out`
```

The `In` and `Out` are type parameters at the outermost; the F's are the
per-level dispatch.

**Uniform folds:** `content_oid` uses the same reducer at every level
(always `hash_tagged(kind_tag, name + body + child_oids)`). For uniform
folds, a thin `Fold1<F, In, Out>` is a special case where all five F's are
the same function. The Fold1 → Fold5 generalization is a one-line wrapper.

**Level-specific folds:** `render` emits different bytes at each kind — a
Focus renders as `keyword name { ...children... }` while an In renders as
`in @name`. Render is naturally a level-specific Fold5 with distinct F's.

**The Prism trait is a collapsed view of Fold5.** prism-core's `Prism`
exposes three methods (focus, project, refract) over a four-Beam type
chain. Fold5 exposes all five bundle levels with explicit per-level
reducers and the two terminals as type parameters. Same machinery, finer
resolution. The cleanest reading of mirror's AST is via Fold5; Prism is the
general-purpose convenience layer.

---

## Why this matters now

Yesterday's session committed `ContentOidPrism` as Tick 1 of the bootstrap
retirement (commit `361e259`). The shape is structurally complete for one
operation: hash the AST recursively, produce a stable OID. But it's a
*concrete* Prism whose body internally dispatches on AstKind via match —
first-order shape, value-and-operation kept separate, the recursive walk
implemented per-instance.

When the second AST-walking operation lands (Tick 3: render retirement),
the recursive walk would be re-implemented. That's the moment to extract.
With two concrete examples, the catamorphism's parameterization is
grounded: the walk machinery is shared; `Ff/Fp/Fs/Fz/Fr` is what differs.

The extraction itself is **not new code**. It's the same machinery that
already exists in spectral.rs's ContentOidPrism, refactored to make the
per-level dispatch explicit and the walk reusable. ContentOidPrism becomes
a thin wrapper around a uniform Fold5; render becomes a level-specific
Fold5; subsequent retirements (validate, transform, lint) inherit the
machinery for free.

---

## Implications for the retirement plan

`docs/specs/bootstrap-retirement-plan.md` Tick 3 (render retirement)
becomes the catamorphism-extraction commit:

1. Introduce `Fold5<Ff, Fp, Fs, Fz, Fr, In, Out>` in `bootstrap/src/spectral.rs`.
2. Introduce `Fold1<F, In, Out>` as the uniform-case sugar (one F applied at every level).
3. Retire `bootstrap/src/render.rs` by writing the render logic as a
   level-specific Fold5 instance.
4. Retroactively rewrite Tick 1's `ContentOidPrism` body as a uniform
   Fold1 instance (or a Fold5 with all five F's equal). The public API
   (`compute_content_oid`) stays unchanged — only the internal
   implementation collapses to the fold.
5. Tests stay green; smoke OIDs unchanged; crystal unchanged; dark count
   unchanged. The refactor is structurally invisible from the outside.

Subsequent ticks (Tick 4: tokenize, Tick 5: pipeline cleanup) compose
against Fold5 / Fold1 rather than introducing new walk machinery.
Tokenize.rs's parser-as-Prism work is the inverse direction (bytes → AST
rather than AST → anything), so it's a different shape — but the AST-side
fold machinery is shared.

---

## What this doesn't claim

- **Novel mathematics.** F-algebra catamorphisms are textbook; bundle
  morphisms are noncommutative geometry; the recognition is *naming* what
  was already there, not inventing it.
- **That ContentOidPrism was wrong.** It was structurally complete for one
  operation; just not the most general shape. The generalization waits
  for two examples (which arrive at Tick 3).
- **That immediate implementation is required.** Tick 1 stays. Tick 3 is
  when the abstraction lands; it lifts Tick 1's shape into the canonical
  form. Tick 2 (kintsugi-loop scaffold) is orthogonal to the AST-fold
  question and can land in any order.
- **That this is the final word on AST machinery.** Inverse folds
  (bytes → AST, the parser-as-Prism case) need a different abstraction.
  Anamorphisms (unfolds) may emerge as Tick 4 lands. This spec covers
  the AST-fold direction; the AST-unfold direction is its own piece.

---

## Out of scope

- The actual implementation (Tick 3's commit).
- The Fold1 convenience API in detail (ergonomic refinement; emerges
  during execution).
- The Anamorphism / unfold direction for tokenize.rs's retirement.
- Cross-grammar fold composition (folding through one grammar's Out into
  another's In) — emergent when LSP / mq-pipeline ergonomics demand it.
- The connection between Fold5 and the eigenboard's measurement (conductivity
  reductions) — same machinery in a different application domain; future
  spec when @cogito's strategy logic lands.

---

## References

- `docs/specs/prism-core-as-spectral-triple.md` — prism-core IS the spectral triple
- `docs/specs/spectral-triple-grammar.md` — the supertrait closure (commit `1076642` on prism-core)
- `docs/specs/eigenboard-representation.md` — the principal O(5)-bundle framing
- `docs/specs/bootstrap-retirement-plan.md` — the retirement plan; Tick 3 references this spec
- `prism/core/src/bundle.rs` — the canonical Bundle trait chain
- `prism/core/src/lib.rs` — the Prism trait (collapsed view of Fold5)
- Bartosz Milewski, *Category Theory for Programmers* — F-algebra / catamorphism chapter
- Erik Meijer, Maarten Fokkinga, Ross Paterson, *Functional Programming
  with Bananas, Lenses, Envelopes and Barbed Wire* (1991) — the canonical
  catamorphism reference
- The 2026-05-21 "cybernetics split" exchange that surfaced this recognition
  (Alex calling out that values-vs-operations is a first-order frame imported
  from training; the AST already IS operations in mirror's substrate)

---

*The AST is the operation written as data.*
*The fold is a bundle morphism.*
*The In/Out are the morphism's domain and codomain.*
*The five F's are how it gets from one to the other.*
*The catamorphism was always there; today we name it.*

Apache-2.0.
