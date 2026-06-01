# Parametric types as Prism operations at the type layer

*2026-05-25. Reed + Alex. Research insight.*

Status: **Yellow** — recognition complete; vocabulary proposed; FP-heritage map established. No implementation yet; this document is the design substrate the next tick targets.

---

## Thesis

Mirror's parametric types are **the five Prism operations applied at the type layer** — not generic-style ad-hoc polymorphism, not C++ template instantiation, not Java erasure. The same five verbs that act on values (`focus`, `project`, `split`, `zoom`, `refract`) act on types, and they mean the same thing one level up. Two forms surfaced in conversation are load-bearing: **`zoom(T)`** is the annotation-only parametric form (same bytes, different lens; type checker distinguishes, runtime doesn't care) and **`refract(T)`** is the verified-construction form (building a `refract(T)` value IS the proof of having been T). This honours Haskell's Functor/Monad split — the annotation-vs-effect distinction — while making it *structurally* precise instead of organisationally precise. Where Haskell asks you to obey laws by convention, mirror's algebra encodes the law in which Prism operation you reached for.

---

## 1. Mirror's existing parametric vocabulary

What's already shipping in `boot/`:

| Type | Declaration | Parameters | File |
|---|---|---|---|
| `imperfect(value, loss, error)` | kernel | 3 | `boot/01a-error.mirror`, `boot/01-meta.mirror` |
| `option(a)` | naming surface over `imperfect(a, (), ())` | 1 | `boot/std/option.mirror` |
| `result(a, e)` | naming surface over `imperfect(a, (), e)` | 2 | `boot/std/result.mirror` |
| `list(a) = empty \| cons(a, list(a))` | recursive | 1 | `boot/std/list.mirror` |
| `beam(t)` | structural carrier with `luminosity(t, e, l)` | 1 (luminosity 3) | `boot/std/beam.mirror` |
| `luminosity(t, e, l) = light(t,l) \| dimmed(t,e,l) \| dark(e,l)` | aliased to `imperfect` directly: `luminosity = imperfect` | 3 | `boot/std/beam.mirror` |
| `crystal(oid)` | content-addressed witness | 1 (degenerate, oid is opaque) | `boot/std/mirror/spectral.mirror` |
| `gen_prism` | record of `{name, ref, head: oid, tick: u64}` | 0 (state polymorphism via the oid) | `boot/std/mirror/runtime/gen_prism.mirror` |
| `peer` | record of five mirrors (`identity, gestalt, tensions, eigenboard, shatter`) | 0 | `boot/std/peer.mirror` |
| `ast(g)`, `expression(g)`, `declaration(g)`, `pattern(g)`, `type_ref(g)` | grammar-parameterised AST | 1 | `boot/01-meta.mirror` |
| `process(actor, state)` | OTP primitive | 2 | `boot/02-actor.mirror`, `boot/std/beam.mirror` |
| `gen_server(state, message)` | OTP behaviour | 2 | `boot/std/beam.mirror` |
| `actor(id)`, `state(type)` | nominal carriers | 1 | `boot/02-actor.mirror` |
| `tick(type) -> tock(type)` | abstract io | 1 | `boot/00-prism.mirror` |

Three things to notice:

1. **`imperfect` is the kernel.** `option`, `result`, and `luminosity` are *aliases* over `imperfect(a, e, l)` with channels collapsed. This is already the Haskell `Either e` / `Maybe` collapse, but declared explicitly as `zoom lift` (annotation) and `fold collapse` (inverse) instead of left to typeclass instance derivation. See `option.mirror`: `zoom lift(option(a)) -> imperfect` and `fold collapse(imperfect) <= option`.
2. **The Prism verbs already appear as type-layer operators.** `zoom lift` is a *type-layer zoom* — same value, different declared shape. `fold collapse` is a *type-layer refract* — a structured destruction with a witness. Mirror has been doing this since the option/result grammars landed; what's new is naming it.
3. **`ast(g)` is the existence proof that grammar-parameterised types are wanted.** The AST is parameterised by the grammar it was parsed under — a `pattern(g)` from `@mirror/grammar` is not interchangeable with a `pattern(g)` from `@mirror/spec`, even though they share the same constructor names. This is exactly the Idris dependent-index pattern.

---

## 2. The FP-heritage mapping

For each Prism operation, the closest typeclass(es) from the FP literature. Be honest about partial matches.

| Prism op | Closest FP shape | Why it matches | Where it diverges |
|---|---|---|---|
| `focus` | **Comonad `extract`** + **Lens.get** | both name the act of looking at *the one* inside a context; `extract :: w a -> a` is `focus`'s value-layer form | mirror's `focus` is *also* the kernel type constructor (`focus type(id)` at `01-meta.mirror`); Haskell's Comonad is value-only |
| `project` | **Foldable `foldr`** + **Filterable** + **Optics' `Prism` (the partial getter)** | `project in` / `project out` decide *which channel matters*; foldable's accumulation is the iterative case | Mirror's `project` operates on grammar boundaries (`in @<ref>` / `out @<ref>`); Haskell has nothing this declarative at the module surface |
| `split` | **Applicative `<*>`** + **Comonad `duplicate`** + **Profunctor** | `split` is the only op that produces *more structure*; applicative's product (`(,) <$> f <*> g`) and comonad's `duplicate :: w a -> w (w a)` are both "branch the context" | mirror's split is also the kernel logical "or" / disjunction (`split \|(ref, ref)`); Haskell's Alternative `<\|>` is closer but lacks the structural duplication |
| `zoom` | **Profunctor `dimap`** + **Phantom types** + **Lens.over** + **Liquid Haskell refinement** | the annotation-only nature — same bytes, different declared shape — is exactly what phantom types and refinement types do | mirror makes zoom a *first-class verb*, not a type-system trick to be discovered; the runtime cost is zero by construction, not by erasure |
| `refract` | **Monad `>>=`** + **Traversable `sequence`** + **Indexed monad** | refract is *verified construction* — the `..` operator on a ref produces a settled crystal; `>>=` similarly produces a value whose existence witnesses the sequencing | Haskell's `>>=` doesn't content-address the result; mirror's refract produces an oid, which is the witness |

The deep observation: **five operations cover what Haskell carves into ~12 typeclasses**, because mirror's algebra was designed top-down from optics/physics/spectral-graph theory, whereas Haskell's hierarchy grew bottom-up from ML's type system + Wadler's monads paper (1992) + later additions (Applicative 2008, Foldable/Traversable 2014). Mirror compresses; Haskell accreted.

---

## 3. The `zoom(T)` / `refract(T)` distinction in the literature

This is where the FP heritage gets specific. The annotation-vs-verified split has been made repeatedly but never as the same axis under one name.

- **Phantom types** (Leijen & Meijer 1999, McBride's *Faking it*) — the canonical "same bytes, different declared shape" idiom. `zoom(T)` IS phantom types lifted to a first-class verb. Where Haskell uses `newtype Tagged s a = Tagged a` and hopes the user respects the tag, mirror uses `zoom lift(option(a)) -> imperfect` and the verb itself documents "this is an annotation, not a transformation."
- **Liquid Haskell** (Vazou et al. 2014) — refinement types as `{v: Int | v > 0}`. The refinement is annotation-only; the runtime value is just an `Int`. This is `zoom(positive_int)`. Mirror's substrate already does this through `epistemologic.property literal(declaration) -> verdict` — the verdict is the refinement, declared not derived.
- **Idris dependent indices** (Brady 2013) — `Vect n a` where `n :: Nat` is the length. The index is *verified* by construction — building a `Vect 3 a` IS the proof you provided three `a`s. This is `refract(T)`. The `ast(g)` parameter in `boot/01-meta.mirror` is mirror's version: building an `ast(@mirror/grammar)` IS the proof you parsed under that grammar.
- **Agda's setoid hell / Cubical Agda** — the propositional equality vs. judgemental equality split. `zoom(T)` is judgemental (the checker sees it but the runtime doesn't); `refract(T)` is propositional (the witness exists and can be inspected). Mirror sidesteps setoid hell because content-addressing replaces equality reasoning with oid equality — a categorical move Agda cannot make because Agda has no content-addressed substrate.
- **Lean 4 typeclasses with `instance`** — the recent shift to first-class typeclass resolution. Closer to mirror in spirit (instance search is explicit), but Lean's typeclasses are still open-ended; mirror's five-op closure is fixed.
- **OCaml functors-as-modules** (MacQueen 1984, Harper & Stone 2000) — module-level parametricity. OCaml's `module type S = sig type t end` and `module F (X: S) = ...` is closest to mirror's grammar-parameterised types. Mirror's `grammar @<name>(<tags>)` IS a module functor in this sense. The `peer` type — a record of five named mirrors — is structurally an OCaml first-class module.
- **F# computation expressions** — user-defined `do`-notation. The desugaring is local and syntactic. Mirror's `|>` and `|\>` (see the pipe-with-a-hole insight, same date) are a similar move: composition that's declared in the language and resolved by the substrate.
- **Scala cats-effect IO[A]** — the closest thing in mainstream FP to mirror's content-addressed `crystal(oid)`. `IO[A]` is a description of an effect; `crystal(oid)` is a content-addressed witness of an evaluation. Both delay until refraction.
- **Roc's platform model** — effects move to the platform layer; the language stays pure. Mirror does the same: `@io`, `@actor`, `@beam` are effect surfaces that delegate to platform substrates. Roc's tag unions are also structurally identical to mirror's `option`, `result` aliases.

**No FP language has made `zoom(T)` and `refract(T)` the same axis under the same verb.** The closest is Idris's `0 Nat` quantity (compile-time-only) vs. unrestricted (`Vect n a` where `n` is computational), but Idris frames it as a quantity discipline, not as two members of a five-verb algebra. Mirror's contribution is the *naming* — calling out that these are two Prism operations applied at the type layer, with the same names as the value-layer operations, with the same semantics shifted up one universe.

---

## 4. The minimum viable algebraic hierarchy for mirror

Haskell's tower: `Functor → Applicative → Monad → MonadTrans → MonadIO → …`. ~12 layers, decades of bikeshedding, the Applicative-Monad Proposal of 2014 to retrofit hierarchy correctness.

Mirror needs *less*. The hierarchy is fixed at five, and they're not stacked — they're co-equal verbs of the same algebra. The minimum viable land-it-first set:

1. **`zoom(T)` as the universal annotation form.** Lands first. Covers phantom types, refinement types, newtype wrappers, lens views. Has zero runtime cost by construction. The syntax already exists in `boot/std/option.mirror` (`zoom lift(option(a)) -> imperfect`); generalising it to a declaration form is the work.
2. **`refract(T)` as the verified-construction form.** Lands second. Covers indexed monads, dependent indices, content-addressed witnesses. Already partially present as the `..(ref)` operator in `boot/01-meta.mirror` and as `crystal(oid)` in `mirror/spectral.mirror`. Generalising to user types is the work.
3. **`focus(T)`, `project(T)`, `split(T)` at the type layer.** Land together. `focus(T)` is the singleton/identity type. `project(T)` is the filtered/narrowed type. `split(T)` is the product/coproduct constructor. These mostly already exist implicitly (struct fields are `focus`, enum variants are `split`, pattern destructuring is `project`); naming them as type-layer operations is the work.

**Deferred deliberately:**

- **Monad transformers.** Haskell needs them because monads don't compose. Mirror's `imperfect(value, loss, error)` is a single three-channel kernel; transformer stacking doesn't apply. The `beam` carrier composes channels by addition (path concat, loss sum, hole set union), not by stacking constructors.
- **Higher-kinded types beyond kind `*`.** Mirror's `ast(g)` is `* -> *`; mirror's `imperfect(a, l, e)` is `* -> * -> * -> *`. Both are first-order. Mirror does not need `(* -> *) -> *` (the Haskell `MonadTrans` shape) because there are no transformers to abstract over.
- **Constraint kinds / GADTs / type families.** Mirror's grammar parameter (`ast(g)`) does the work of GADTs by *being* the index. Constraint kinds are not load-bearing because the five-op closure is the constraint vocabulary.
- **Linearity / quantitative types (Linear Haskell, Idris 2 QTT).** Worth revisiting after `refract(T)` lands; the content-addressed substrate may already give linearity "for free" (an oid is consumed by being refracted away from).

---

## 5. Standard parametric types mirror should ship

The canonical set, by which file already declares them:

| Name | Shape | Status | File / proposal |
|---|---|---|---|
| `option(a)` | naming surface over `imperfect(a, (), ())` | shipped | `boot/std/option.mirror` |
| `result(a, e)` | naming surface over `imperfect(a, (), e)` | shipped | `boot/std/result.mirror` |
| `list(a)` | recursive `empty \| cons(a, list(a))` | shipped | `boot/std/list.mirror` |
| `imperfect(a, l, e)` | kernel three-channel | shipped | `boot/01a-error.mirror`, `boot/01-meta.mirror` |
| `beam(t)` | typed value carrier with path/loss/topology | shipped | `boot/std/beam.mirror` |
| `crystal(oid)` | content-addressed witness | shipped (degenerate) | `boot/std/mirror/spectral.mirror` |
| `gen_prism` | state-polymorphic process via oid | shipped (state untyped) | `boot/std/mirror/runtime/gen_prism.mirror` |
| `peer` | five-axis identity record | shipped | `boot/std/peer.mirror` |
| `ast(g)` | grammar-parameterised AST | shipped | `boot/01-meta.mirror` |
| `process(actor, state)` | OTP primitive | shipped | `boot/02-actor.mirror` |
| `gen_server(state, message)` | OTP behaviour | shipped | `boot/std/beam.mirror` |
| **`zoom(t)`** | the annotation form, generalised | **proposed** | this insight |
| **`refract(t)`** | the verified-construction form, generalised | **proposed** | this insight |
| **`crystal(t)`** | typed crystal (oid + phantom t) | **proposed** | extension of `mirror/spectral.mirror` |
| **`gen_prism(state)`** | typed-state actor | **proposed** | extension of `gen_prism.mirror` |

**Naming rationale.**

- `option` / `result` keep their FP-canonical names; the alias-over-imperfect framing is the contribution, not the rename.
- `imperfect` is mirror-native and load-bearing — Haskell has nothing this honest about three-channel failure (the closest is `These`, used rarely).
- `beam`, `crystal`, `gen_prism`, `peer` are mirror-native; their names map to physics/optics/spectral-graph theory, and that mapping is the point. Don't import Haskell's `IORef` / `MVar` / `STM` vocabulary; mirror's substrate is content-addressed, not heap-resident, and the names should reflect that.
- The proposed typed `crystal(t)` and `gen_prism(state)` close two gaps: today `crystal` is `crystal(oid)` (opaque) and `gen_prism`'s state is also untyped (an `oid` in the record). Adding a phantom `t` for crystal and a state-type parameter for gen_prism brings them into the parametric algebra without runtime cost.

---

## 6. Where mirror extends the FP heritage

Three concrete extensions, ordered by how much they novel:

1. **`zoom(T)` and `refract(T)` as the same axis of the same five-verb algebra.** No FP language has framed annotation-vs-verified as two operations of one algebra with shared names. Phantom types, refinement types, dependent indices, quantitative types — each is a separate language feature in Haskell/Idris/Lean. Mirror collapses them into two verbs that the type checker can name explicitly.
2. **Content-addressing as a typing primitive.** `crystal(oid)` is a typed reference whose identity is the hash of its content. Haskell's `Data.Hashable` produces hashes but the type system doesn't depend on them. Nix derivations content-address but don't expose this at the type layer. Mirror's `@mirror/spectral.crystallize(ast) -> crystal` makes the content hash the *typing witness*. This enables `gen_prism(state)` to have its state type checked by *what's in the oid*, not just by the type of the variable holding the oid.
3. **The autopoietic `gen_prism` as a typed actor whose state lives in the content-addressed substrate.** Closest analogue: Haskell's `Generic` + `TypeRep` reflection + `Cloud Haskell`'s static dictionaries. None compose. Mirror's `gen_prism` is a single grammar declaration that gets a typed state, a typed message, a typed tick function — because the state IS an oid and the oid IS the content-address of the typed crystal. The autopoietic claim (Soto-Andrade & Varela 1984, cited in `boot/std/cogito.mirror`) is verifiable as a Lawvere fixed point on the tick map. Haskell can express the tick map; it cannot verify the fixed point without leaving the type system.

A fourth, more speculative extension: **grammar-parameterised types as a substrate for cross-grammar verification.** `ast(@mirror/grammar)` and `ast(@mirror/spec)` are distinct types in mirror; in Haskell they would both be `AST` and the grammar context would have to be threaded as a phantom parameter or a state monad. Mirror's grammar parameter is *the* parameter, not an addition.

---

## 7. Where mirror diverges from the heritage

Name the divergences honestly:

1. **Sub-Turing by design.** Haskell is Turing-complete; mirror is not. `recursive types are recursive` but `unrestricted recursion` is not allowed at the grammar layer (see `boot/std/epistemologic/property/total_classification.mirror`). This means mirror's analogue of `fix :: (a -> a) -> a` does not exist as a first-class combinator. The Lawvere fixed-point machinery in `@cogito.autopoietic` substitutes for unbounded recursion, but it's verified, not assumed.
2. **Content-addressed substrate is not optional.** Haskell can be implemented on a heap, an arena, a GC, a manual allocator. Mirror cannot be implemented without git-as-backing-store (or an equivalent content-addressed object store). The `crystal(oid)` type *requires* that substrate. This is a divergence from FP heritage *because* FP heritage assumes a referentially-transparent value world; mirror assumes a referentially-transparent *and content-addressed* value world. The second condition is stronger.
3. **The five operations are a closed universe.** Haskell's typeclasses are open — anyone can declare a new `class`. Mirror's five Prism operations are fixed. This is a feature, not a limitation: every new grammar must express itself in those five verbs, which forces compositionality with the rest of the substrate. The cost: you cannot invent a new structural verb. The benefit: every grammar is composable with every other grammar through the same five-verb interface.
4. **`recover` / `rescue` are syntactic, not monadic.** Haskell uses `MonadError` / `ExceptT` for error handling; mirror uses `recover |result, loss| { … }` and `rescue |error| { … }` blocks at the grammar level. The error channel is built into `imperfect`; the syntax is built into the grammar. No transformer stacking. See `option.mirror`, `result.mirror`, `list.mirror`, `beam.mirror` for the consistent pattern.
5. **Effects are platform surfaces, not type-system distinctions.** `@io`, `@actor`, `@beam` are imported grammars; effectful actions are `abstract action` declarations resolved by the substrate at compile time via `|\>`. There is no `IO a` monad shape; there's `imperfect(a, loss, error)` with an `@io` boundary. Roc gets close to this with platforms; F# computation expressions get close at the syntactic layer; mirror is unique in unifying them under the Prism algebra.

---

## 8. Open questions

Things the FP literature has open or that mirror's algebra hasn't decided yet. Worth Alex's design attention:

- **Does `zoom(T)` need a Functor-style law?** Haskell's Functor law is `fmap id = id` and `fmap (f . g) = fmap f . fmap g`. The mirror analogue would be: zoom is identity-preserving and composes. Is that load-bearing for `zoom(T)` to be sound, or does it fall out of "same bytes"?
- **Does `refract(T)` need a Monad-law analogue?** Left identity, right identity, associativity. The mirror analogue would express these on `crystal`: `crystallize . recall = id` (recall round-trips); `crystallize` is associative across composition; etc. Probably yes — these are theorems the content-addressed substrate already satisfies, but stating them as required properties of `refract(T)` would make them verifiable.
- **Higher-kinded `zoom`?** Can `zoom` apply to a type constructor (`zoom(list)` annotating `list` as a whole, not `list(a)`)? Haskell's Higher-Kinded Polymorphism (HKP) hits this regularly. Mirror probably doesn't need it for v1, but the question will arise as soon as someone wants to write a generic "zoom every element" combinator.
- **What happens when `zoom(T)` and `refract(T)` interact?** Building `refract(zoom(T))` is conceptually "verified annotation" — a witness that the annotation was correct. Building `zoom(refract(T))` is conceptually "annotated witness" — a lens onto an already-verified value. These should mean different things. The composition table has not been worked out.
- **Does `gen_prism(state)` need a state-evolution constraint?** The tick function `(state, message) -> (state, emissions)` is currently typed by the state at each end. If `state_n: focus(Initial)` and `state_{n+1}: focus(Settled)`, the type system should refuse to send a settled state back to a tick that expects an initial state. This is indexed-monad territory (Atkey 2009); mirror's `crystal` chain could be its natural home.
- **Does the `peer` type's five-field shape ($focus, project, split, zoom, refract$ of `self`) need to be enforced by type checking, not just by convention?** Today `peer.mirror` declares five fields named after the operations; the connection to the operations themselves is documented but not type-checked. Making it structurally enforced (one field per operation, drop one and the algebra rejects the peer) would close the loop. This is the *strongest* form of the thesis: the parametric-type algebra is itself the five operations, and the peer's identity proves it.

---

## Closing

The FP heritage is forty years deep. Mirror should honour it: keep `option`, `result`, `list`; keep `map`, `filter`, `fold`, `and_then`. The names work; refusing them would cost more than it earns.

Where mirror differs is structural, not cosmetic. The five Prism operations apply at the type layer with the same names they apply at the value layer, because they ARE the same operations one universe up. `zoom(T)` is the annotation; `refract(T)` is the witness; the algebra is closed at five and that closure is the contribution.

The pipe-with-a-hole (`|\>`, same date) shows the value-layer side: Fate resolves the algorithm in context. The type-layer side: `zoom` and `refract` resolve the *shape* in context. Same algebra, different universe. Same five verbs.

Apache-2.0.
