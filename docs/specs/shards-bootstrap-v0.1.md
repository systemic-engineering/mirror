# shards-bootstrap-v0.1 — substrate rewrite roadmap

*2026-06-04. Reed. Status: roadmap, not implementation. No `.mirror` files land with this commit.*

**The reframe in one sentence.** The canonical folder for mirror source files is now `shards/` (not `boot/`, and not `glass/` — that was an external metaphor superseded within the hour, per `[[architecture-shards-as-substrate-source]]`). Shards ARE the substrate's content-addressed unit; the substrate's source code lives in its own data structure; the recursive proof becomes literal. `boot/std/*.mirror` and `prism/*` become **reference material** — the templates the bottom-up rewrite ports against, not authoritative substrate.

**The pivot.** `shards/metalogue.mirror` lands first: the Batesonian metalogue of the language about itself. It declares the foundational prisms mirror uses to describe its own type system — `@loss`, `@transparency`, `@imperfect`, `@property_verdict`, `@ref`, `@diagnostic`, `@confidence`, `@opacity_map`. After the metalogue, work proceeds slow and steady, bottom-up: each tick ports another piece of `boot/` into `shards/` with proper substrate-pull discipline. The tokenizer becomes a `shards/*` declaration. The parser. The resolver. All of it.

**This IS v0.1's pipeline now.** The kintsugi-CI work (T11.4–T11.8 in `[[kintsugi-ci-v0.1]]`; T11.2.7 to refactor `Verdict` on top of the rewritten primitives) waits until the substrate primitives are declared.

> *Note on naming.* The previous draft of this spec used `glass/` for the top-level folder. The substrate has its own vocabulary for "the content-addressed unit it stores" — `shard`, established by `[[architecture-shard-as-crdt]]`, `[[architecture-shard-ref-as-prism]]`, and `SpectralUuid::EMPTY`. Reaching for `glass` was an external-metaphor reach; the correction lands as this upsert.
>
> **Two distinct uses of "glass" remain in mirror.** The TOP-LEVEL folder is now `shards/`. But `@mirror/glass/ast/*` — the typed AST representation that landed in Stage 1 of #94 — stays as-is. That's substrate-internal vocabulary for the AST altitude; it has nothing to do with the source-tree folder name. Don't conflate them.

---

## 0. Context — what this corrects

Four substrate-pull corrections landed today (2026-06-04) — all in the same direction: **use existing substrate vocabulary; don't reach for external metaphors.** The four corrections are folded into this upsert:

1. **`glass/` → `shards/`** (`[[architecture-shards-as-substrate-source]]`). The substrate already has a load-bearing name for the content-addressed unit it stores. `shards/` is that name. Using `glass/` for the source-tree folder was an external-metaphor reach. `@mirror/glass/ast/*` (substrate-internal AST vocabulary) stays.
2. **`grammar` + `type` → `prism`.** Per `boot/00-prism.mirror`: the substrate has ONE primitive — `prism` — that declares "a typed structure that comes with the five operations + whatever additional actions you declare." A prism IS a trait IS a type IS a grammar. The substrate doesn't distinguish. The previous draft's `grammar @metalogue { type X = ... }` was the wrong altitude; the substrate-correct form is `prism @metalogue { prism @X { ... } }`.
3. **Method signatures → typed lambdas with obligation bodies.** The Haskell-style `zero : () -> @loss` was wrong; the substrate-native form is `zero -> @loss { \ }` — a typed lambda with the **default obligation block**. The `{ \ }` is the substrate's marker for "this action is declared; its body is the obligation that lives elsewhere (typically at the glass wall — the Rust impl)."
4. **Declare each primitive with full algebraic structure, not just type signatures.** `@loss` isn't "the prism with four methods" — it's the **monoid prism**, with identity, associativity, and absorption. `@transparency` is a loss-implementer with audit-channel-specific operations. `@imperfect` is parametric with a bounded type parameter (`l <= @loss`). Each metalogue prism declares its WHOLE algebra.

Three landings on the substrate over the last two weeks make this reframe load-bearing:

- `[[architecture-glass-wall-substrate-types]]` (2026-06-04): `Imperfect`, `Transparency`, `PropertyVerdict` ship as Rust crates only. Mirror's grammar **consumes** them but does NOT **declare** them. The Verdict roundtripping in `kintsugi --ci` cannot be properly typed until the substrate declares the primitives at the substrate altitude. T11.2.6's `discrimination = success | partial | failure` was the halfway-house Alex caught.
- `[[feedback-tokenizer-is-grammar-bootstrapped]]` (2026-06-03): the substrate is grammar-bootstrapped — adding a typed construct is a `.mirror` addition, not a Rust change. The tokenizer reads boot grammars at startup. T11.2.5 routed the wrong way; the right answer is grammar declaration.
- `boot/std/mirror/glass/*` already exists. Stage 1 of #94 landed the additive surface for `glass/ast`, `glass/ast/shape`, `glass/ast/token`. The legacy `@mirror/grammar` stays parallel during the migration. The substrate already uses `glass` AS A TYPE-LAYER NAME (the AST). The source-tree folder name is independent.

The reframe pulls the structure all the way up: the **top-level folder** is `shards/`, not `boot/`, and not `glass/`. Existing `boot/std/mirror/glass/*` is a sub-tree under the legacy `boot/` root that should migrate to live at `shards/mirror/ast/*` — the inner `glass/` was redundant once the source-tree folder was renamed away from `glass/`.

---

## 1. The metalogue — what `shards/metalogue.mirror` contains

`shards/metalogue.mirror` is the foundational substrate declaration. It is the **metalogue** in Bateson's sense: a dialogue whose structure mirrors what it describes. The file declares — in mirror grammar — the prisms mirror uses to describe its own type system. The Rust crates `prism/imperfect/` and `prism/core/` BECOME the implementations of these declared prisms; the declarations become the source of truth.

### 1.1 The declaration order

Eight prisms, declared in dependency order, each with its full algebraic structure:

1. **`@loss`** — the **monoid prism**. Identity (`zero`), accumulation (`combine`, associative), absorption (`total`, the absorbing element). Predicate `is_zero` for the identity check. Every other variety carrier in the substrate is structured against this.
2. **`@ref`** — the `@`-prefixed substrate reference. Newtype over `text` with the leading-`@` invariant. The substrate's naming primitive.
3. **`@diagnostic`** — the per-issue message. Newtype over `text`. The no-bare-types discipline applied at the substrate edge.
4. **`@confidence`** — `f64` in `[0.0, 1.0]`. Newtype. Lower = less confident.
5. **`@property_verdict`** — per-location pass/partial/fail with confidence + diagnostics. Three variants; partial accumulates, fail dominates. Comes with `merge_with` (the lattice join).
6. **`@opacity_map`** — newtype over `map(@ref, @property_verdict)`. The substrate-private constructor for `@transparency`'s `opaque` variant — structural enforcement of the catastrophic-empty-map invariant from `prism/imperfect/src/transparency.rs`.
7. **`@transparency`** — the located-opacity loss. Two variants: `clear` (identity) and `opaque(@opacity_map)` (the substrate's audit channel). Implements `@loss` via `<= @loss`. The `combine` on `opaque` joins opacity maps key-wise; `opaque(empty_map)` is the catastrophic absorbing element.
8. **`@imperfect`** — the three-state functor. Variants `success(t)`, `partial(t, l)`, `failure(e, l)`. Parametric in `(t, e, l <= @loss)`.

### 1.2 The sketch — actual mirror grammar

Modeled directly on `boot/00-prism.mirror`'s declaration form: `prism @<name> { <action declarations with obligation bodies> }`. Each action is a typed lambda with the default obligation block `{ \ }`. The Rust impl in `prism/imperfect/src/lib.rs` and `prism/imperfect/src/transparency.rs` fulfills the obligations. **Sketch, not final:**

```mirror
in @prism
in @meta

# @metalogue — the substrate's declaration of its own variety vocabulary.
#
# Bateson's "metalogue": a dialogue whose structure mirrors what it
# describes. This file declares — in mirror grammar — the prisms
# mirror uses to describe its own type system. The Rust crates
# `prism/imperfect/` and `prism/core/` BECOME the implementations of
# these declared prisms; the declarations are the source of truth.
#
# A prism IS a trait IS a type IS a grammar. The substrate doesn't
# distinguish — there is ONE primitive (`prism`) that declares "a
# typed structure with the five operations + whatever actions you
# declare." Every action below uses the typed-lambda + default
# obligation block form (`{ \ }`) introduced in `boot/00-prism.mirror`:
# the substrate declares; the glass wall (Rust impl) fulfills.
#
# Cross-reference (each prism's Rust home):
#
#   @loss              → prism/imperfect/src/lib.rs (trait Loss)
#   @ref               → prism/core/src/named.rs (Named<P>) + content_addressed
#   @diagnostic        → prism/imperfect/src/transparency.rs (Diagnostic)
#   @confidence        → newtype over f64 in [0.0, 1.0]
#   @property_verdict  → prism/imperfect/src/transparency.rs (PropertyVerdict)
#   @opacity_map       → prism/imperfect/src/transparency.rs (OpacityMap)
#   @transparency      → prism/imperfect/src/transparency.rs (Transparency<P>)
#   @imperfect         → prism/imperfect/src/lib.rs (Imperfect<T, E, L>)

prism @metalogue {

  # ===== @loss: the monoid prism =====
  #
  # @loss forms a monoid under combine:
  #
  #   zero is identity      :  combine(zero, l)   = l
  #                            combine(l, zero)   = l
  #   combine is associative:  combine(a, combine(b, c)) = combine(combine(a, b), c)
  #   total is absorbing    :  combine(total, l)  = total
  #                            combine(l, total)  = total
  #
  # The five operations under @loss:
  #   focus    : the loss bit-count (Shannon weight)
  #   project  : extract the measurement source (which @ref)
  #   split    : decompose a combined loss into its constituents (the inverse of combine)
  #   lift     : annotation — the same loss at a different declared shape
  #   refract  : verified accumulation — produces zero only when no loss occurred
  prism @loss {
    zero            -> @loss            { \ }
    total           -> @loss            { \ }
    is_zero(@loss)  -> bool             { \ }
    combine(@loss, @loss) -> @loss      { \ }
  }

  # ===== @ref: the @-prefixed substrate reference =====
  #
  # Newtype over text with the leading-@ invariant. The substrate's
  # naming primitive: every prism, every shape lookup resolves
  # through a ref. The constructor is substrate-private; outside
  # @metalogue nothing can mint a ref that violates the invariant.
  #
  # The five operations under @ref:
  #   focus    : the underlying text
  #   project  : split the namespace path (@a/b/c → [@a, @b, @c])
  #   split    : the disjunction over the ref namespace tree
  #   lift     : raise / lower between namespace altitudes (the functor)
  #   refract  : verified-construction — produces a ref only when
  #              the leading-@ invariant holds
  prism @ref = newtype(text)

  # ===== @diagnostic: the no-bare-strings discipline =====
  #
  # Per `[[feedback-no-bare-types]]`: a diagnostic is not a string;
  # it's a typed envelope. Same shape as text at runtime, distinct
  # at the type altitude.
  prism @diagnostic = newtype(text)

  # ===== @confidence: bounded f64 in [0.0, 1.0] =====
  #
  # Refinement type. The constructor is total — out-of-range values
  # clamp to the nearest bound (an obligation enforced at the glass
  # wall). Forms a lattice under min/max; the `merge_with` on
  # @property_verdict uses min as the conservative combine.
  prism @confidence = newtype(f64)

  # ===== @property_verdict: per-location pass/partial/fail =====
  #
  # The three shapes a property check produces at one substrate
  # location. Pass is the identity; partial accumulates diagnostics;
  # fail dominates (the absorbing element under merge_with).
  #
  # Forms a join-semilattice:
  #   pass < partial(c, ds) < fail(d)
  #
  # merge_with is the lattice join — used by @transparency.combine
  # when two opacity maps collide on the same @ref key.
  prism @property_verdict =
    | pass
    | partial { confidence: @confidence, diagnostics: [@diagnostic] }
    | fail(@diagnostic)
  {
    merge_with(@property_verdict, @property_verdict) -> @property_verdict { \ }
    is_pass(@property_verdict) -> bool { \ }
  }

  # ===== @opacity_map: the substrate-private newtype =====
  #
  # The map carried inside @transparency.opaque. The constructor is
  # substrate-private (per the catastrophic-empty-map seam in
  # prism/imperfect/src/transparency.rs); outside @metalogue
  # nothing can forge the sentinel.
  #
  # Forms a join-semilattice under key-wise merge_with: for shared
  # keys, the @property_verdict join is taken; new keys are added.
  prism @opacity_map = abstract newtype(map(@ref, @property_verdict)) {
    merge(@opacity_map, @opacity_map) -> @opacity_map { \ }
    is_empty(@opacity_map) -> bool { \ }
  }

  # ===== @transparency: located opacity AS a loss =====
  #
  # Two variants: clear (the identity) and opaque (the audit channel).
  # `@transparency <= @loss` declares: @transparency implements the
  # @loss prism. The implementation:
  #
  #   zero       = clear
  #   total      = opaque(empty_map)        — the catastrophic sentinel
  #   is_zero(t) = match t { clear => true, opaque(_) => false }
  #   combine    = clear:        identity
  #                opaque(m1, m2): opaque(m1 merge m2)   key-wise verdict join
  #                opaque(empty), _: opaque(empty)       absorption
  #
  # Only minted via total() or by combine-driven absorption from
  # within the metalogue. Audit-channel-specific operations:
  #
  #   locate(@transparency, @ref) -> option(@property_verdict)
  #   witness(@transparency)      -> [@ref]
  #
  # The five operations under @transparency:
  #   focus    : the carried opacity map (or empty if clear)
  #   project  : the [@ref] of witnessed locations
  #   split    : decompose by location — one transparency per key
  #   lift     : view at a coarser/finer @ref granularity
  #   refract  : merge a sequence of transparencies (associative fold)
  prism @transparency =
    | clear
    | opaque(@opacity_map)
  {
    locate(@transparency, @ref) -> option(@property_verdict) { \ }
    witness(@transparency)      -> [@ref]                    { \ }
  }

  @transparency <= @loss

  # ===== @imperfect: the three-state functor =====
  #
  # Result extended with partial success. Parametric in (t, e, l)
  # where `l <= @loss` declares: l is bounded by the @loss prism
  # (l implements @loss). The bound is what makes combine and
  # zero meaningful at the variant boundary.
  #
  #   success(t)        observation succeeded, no loss
  #   partial(t, l)     observation succeeded with measured loss
  #   failure(e, l)     observation failed; the loss is the cost
  #
  # The five operations under @imperfect:
  #   focus    : the carried value (or the error for failure)
  #   project  : the carried loss (success projects to l.zero)
  #   split    : the | disjunction over the three variants
  #   lift     : annotation — re-shape t, e, or l without recompute
  #   refract  : verified accumulation — success-only chains preserve;
  #              partial threads the loss; failure absorbs
  prism @imperfect(t, e, l <= @loss) =
    | success(t)
    | partial(t, l)
    | failure(e, l)
  {
    recover(t, l) -> @imperfect { \ }
    rescue(e, l)  -> @imperfect { \ }
  }
}

out @loss
out @ref
out @diagnostic
out @confidence
out @property_verdict
out @opacity_map
out @transparency
out @imperfect
out @metalogue
```

### 1.3 Notes on the sketch

- **`prism @X { ... }` is the only primitive.** Resolved per Correction 2 + the Q1 question from the previous draft (RESOLVED): a prism's actions ARE its trait methods. There's no separate `abstract type` construct. The `prism @loss { ... }` declaration form is the substrate-correct shape — actions inside the block are the four monoid operations.
- **`l <= @loss` reads "l is bounded by the @loss prism."** Per Correction 2 + the Q2 question (RESOLVED): `<=` IS the implements relation at the prism altitude. Same relation as `boot/01-meta.mirror`'s `fold <=(ref, imperfect)` use, generalized correctly to the type-parameter bound. `@transparency <= @loss` (the standalone statement at the end of the @transparency block) declares the implementation; `@imperfect(t, e, l <= @loss)` declares the parameter bound.
- **The default obligation block `{ \ }`** comes from `boot/00-prism.mirror`'s `abstract io tick(type) -> tock(type) { \ }`. Every metalogue action uses this form; the Rust impl in `prism/imperfect/` fulfills the obligations. **Substrate declares; Rust implements.**
- **`prism @X = newtype(T)`** is the single-field newtype shorthand, lifting `boot/std/option.mirror`'s `type option(a) = some(a) | none` form to the prism altitude. `prism @ref = newtype(text)` declares a refinement with no body block; `prism @opacity_map = abstract newtype(map(...)) { ... }` declares one with a substrate-private constructor and a body block of actions.
- **Variant prism declarations** use the `|` disjunction from `boot/01-meta.mirror` (`split |(ref, ref)`). The body block (`{ merge_with ... }`) declares the actions that come with the variant prism.
- **Catastrophic-empty-map invariant** lifts from `pub(crate)` (Rust visibility) to `abstract newtype(...)` + the `out` list (substrate export discipline). The grammar exports the prism name but not the constructor.
- **Five operations come for free.** Every prism declared in `@metalogue` automatically carries `focus`, `project`, `split`, `lift`, `refract` (the `prism @prism { ... }` declaration in `boot/00-prism.mirror` is what makes this true). The comments in the sketch make their semantics explicit at each prism where they're load-bearing. `lift` is the functor operation (`lift f : T(A) -> T(B)` when `f : A -> B`); earlier drafts called this `zoom`, but the substrate already used the action name `lift` (`zoom lift(option(a)) -> imperfect` in `boot/std/option.mirror`), and the trait method and the action's name converge.

### 1.4 What the metalogue does NOT declare

To stay tight and avoid sprawl:

- **No `eh` / `imp` / `tri` bind operators.** Those are operations on `@imperfect`, not part of the prism's substrate declaration. They land in `shards/std/imperfect.mirror` once that file exists.
- **No `@metric` sub-prism.** `prism/imperfect/src/lib.rs` defines `Metric: Loss` with `distance_to` + `triangle`. The metric refinement is Connes' bounded-commutator condition; it's an addition to `@loss`, not a foundational primitive. Lands at `shards/metric.mirror` later if a downstream prism demands it.
- **No `@carrier` prism.** `prism/core/src/connection.rs` declares `Carrier: Clone + Default` with `compose` + `norm`. Used by `bundle.rs`. Lands at `shards/carrier.mirror` once the bundle altitude gets ported.
- **No `@luminosity`.** `prism/core/src/luminosity.rs` is a ternary (Light/Dimmed/Dark) used by `beam`. Adjacent but not foundational; can land in `shards/luminosity.mirror` when `shards/beam.mirror` ports.

The metalogue is **eight prisms**. Not nine; not seven. The eight foundational primitives any other substrate declaration in `shards/*` may depend on.

> **The metalogue is the substrate altitude of the prism algebra.** See `pq.md` §6.5 (LAPACKPrism — the numerical substrate) for how the same algebra plays at the WIRE altitude. The metalogue declares the substrate's own algebraic vocabulary; pq's three-op wire (focus/project/refract) is what composes when those prisms move across processes. Same algebra, different altitude.

---

## 2. The porting order

### 2.1 Tick decomposition (loose)

| Tick | Lands | Depends on | Notes |
|---|---|---|---|
| **T_shards.1** | `shards/metalogue.mirror` | — | Eight prisms, one file. The substrate's self-declaration. |
| **T_shards.2** | `shards/option.mirror`, `shards/result.mirror`, `shards/bool.mirror` | T_shards.1 (only `@imperfect`) | Naming surfaces on the kernel functor. Direct ports from `boot/std/`. |
| **T_shards.3** | `shards/std/list.mirror`, `shards/std/map.mirror`, `shards/std/set.mirror`, `shards/std/text.mirror`, `shards/std/number.mirror` | T_shards.1 | The pure-data collections; no metalogue dependency beyond `@loss`. Can port any order. |
| **T_shards.4** | `shards/mirror/ast/token.mirror`, `shards/mirror/ast/shape/*.mirror` | T_shards.1, T_shards.3 | Migrate the existing `boot/std/mirror/glass/ast/*` tree to its substrate-altitude home at `shards/mirror/ast/`. Re-canonicalises the substrate's own AST as a `shards/*` declaration. NOTE: the AST itself still uses `@mirror/glass/ast/*` as its substrate-internal vocabulary — that's distinct from the source-tree folder. |
| **T_shards.5** | `shards/epistemologic/property/*.mirror` | T_shards.1 (`@property_verdict`), T_shards.4 | Verdict-returning property surface lands on the metalogue's `@property_verdict`. Closes `[[properties-on-glass]]` §2.1's qualifier set on substrate-declared primitives. |
| **T_shards.6** | `shards/io.mirror`, `shards/file.mirror` (`@io` surface) | T_shards.1 | The IO grammars from `boot/02a-io.mirror` + `boot/std/file.mirror`. Substrate-pull discipline: `@io` lives in `shards/io.mirror` as the boundary. |
| **T_shards.7** | `shards/kintsugi.mirror` (`@verdict` rewrite) | T_shards.1, T_shards.5 | T11.2.7 lands here. The mirror-text verdict from `[[kintsugi-ci-v0.1]]` §1.4 retypes as `@imperfect(@verdict_value, @failure_reason, @transparency(@ref))`. |
| **T_shards.8** | `shards/mirror/tokenize.mirror`, `shards/mirror/parse.mirror`, `shards/mirror/resolve.mirror` | T_shards.4 | The substrate declares the tokenizer / parser / resolver behaviour as prisms. Rust impl in `bootstrap/src/*.rs` becomes implementation-of-declaration. Phase 2 of the road-to-1.0 lifts here. |
| **T_shards.9+** | Everything else under `boot/std/*` | varies | The remaining `boot/std/*` files (beam, peer, scheduler, cogito, sql, time, ...) port one tick at a time. Order driven by demand, not priority. |

### 2.2 Dependency-free ports (early or late, doesn't matter)

These have NO metalogue dependency beyond `@loss` (which they don't measure against) — they can land any time after T_shards.1:

- `shards/bool.mirror` (trivial split)
- `shards/std/list.mirror`, `shards/std/set.mirror`, `shards/std/map.mirror`
- `shards/std/text.mirror`, `shards/std/number.mirror`, `shards/std/order.mirror`
- `shards/std/cli.mirror`, `shards/std/time.mirror`, `shards/std/git.mirror`

### 2.3 Metalogue-dependent ports

These wait on T_shards.1:

- `shards/option.mirror`, `shards/result.mirror` — both currently declare themselves "imperfect without loss / error" via `lift` + `fold collapse`. They need `@imperfect` declared first. (The legacy form `zoom lift(option(a)) -> imperfect` collapses post-rename to `lift(option(a)) -> imperfect` — trait method and action name converge.)
- `shards/epistemologic/property/*.mirror` — every property returns a `verdict` (≡ `@property_verdict` from metalogue). The verdict union and `@transparency(@ref)` composition are the load-bearing primitives.
- `shards/kintsugi.mirror` — Verdict refactor (T11.2.7) requires `@imperfect`, `@transparency`, `@property_verdict`, and `@ref` declared at substrate altitude. Mara's `discrimination` halfway-house is replaced by the proper instance: `@imperfect(@verdict_value, @failure_reason, @transparency(@ref))`.
- `shards/mirror/tokenize.mirror`, `shards/mirror/parse.mirror` — return `@imperfect(@ast, @parse_error, @transparency(@ref))`. The tokenizer's substrate declaration ports here (T_shards.8); the Rust impl in `bootstrap/src/tokenize.rs` becomes implementation-of-declaration.

### 2.4 Where each `prism/*` crate's surface lands

| Rust crate | Shards declaration | Tick |
|---|---|---|
| `prism/imperfect/src/lib.rs` (`Loss`, `Imperfect`) | `shards/metalogue.mirror` | T_shards.1 |
| `prism/imperfect/src/transparency.rs` | `shards/metalogue.mirror` | T_shards.1 |
| `prism/core/src/named.rs` (`Ref`, `Named<P>`) | `shards/metalogue.mirror` (`@ref` only); `Named<P>` lands later if needed | T_shards.1 |
| `prism/core/src/connection.rs` (`Carrier`) | `shards/carrier.mirror` (post-bundle) | T_shards.9+ |
| `prism/core/src/luminosity.rs` | `shards/luminosity.mirror` | T_shards.9+ |
| `prism/core/src/beam.rs` | `shards/beam.mirror` | T_shards.9+ |
| `prism/core/src/bundle.rs` | `shards/bundle.mirror` | T_shards.9+ |

The metalogue tick (T_shards.1) is the only tick that touches `prism/imperfect/`. Everything else lands later, demand-driven.

### 2.5 Where the tokenizer ports

The tokenizer's substrate-altitude declaration lives in **T_shards.8** (`shards/mirror/tokenize.mirror`). At that point, the Rust impl in `bootstrap/src/tokenize.rs` becomes the **implementation of a declaration** — same behaviour, but the source of truth shifts to the prism declaration. The `[[feedback-tokenizer-is-grammar-bootstrapped]]` discipline closes structurally: new keywords land in `shards/mirror/tokenize.mirror`, the bootstrap reads them at startup, no Rust touches.

The existing `boot/std/mirror/glass/ast/token.mirror` is the **template** the T_shards.8 port draws on. The lexical primitives there (`whitespace`, `identifier`, `reference`, etc.) carry forward; the new home is `shards/mirror/ast/token.mirror` (T_shards.4 migration).

---

## 3. The reference-material framing

This is the discipline that holds throughout the rewrite:

### 3.1 `boot/std/*` becomes read-only reference

Existing `boot/`, `boot/std/`, `boot/std/mirror/glass/` files are **templates the rewrite ports against**. They are not edited; new files land in `shards/` instead. The bootstrap continues to read `boot/` until T_shards.8 lands `shards/mirror/tokenize.mirror`, at which point the bootstrap's grammar loader gets pointed at `shards/`.

**The migration mechanic.** Each ported file lands in `shards/` with the same logical content but the corrected substrate-altitude shape (e.g., using `prism @X { ... }` rather than `grammar @X { type X = ... }`; using `@imperfect` from metalogue rather than ad-hoc enums). The old `boot/` file is removed in the SAME tick (no parallel surfaces; the substrate doesn't need a backward-compat layer for pre-v0.1).

### 3.2 `prism/*` Rust crates remain the implementation substrate

Per `[[architecture-fragmentation-is-the-rust-substrate]]`: `prism_core` stays deps-free; `prism/imperfect/` remains the canonical Rust home of `Imperfect`, `Transparency`, `PropertyVerdict`, `Diagnostic`. The Rust types ARE the runtime behaviour of the `shards/metalogue.mirror` declarations. No new crates are minted for this work; no existing crates are retired.

What changes is the **direction of truth**: today, the Rust types are the source of truth and `boot/*.mirror` consumes them ad-hoc. After T_shards.1, `shards/metalogue.mirror` is the source of truth and the Rust types are the implementation. The glass wall becomes load-bearing — and "glass wall" here means the boundary between substrate declarations and Rust impl, not the source-tree folder name.

### 3.3 The folder layout

The existing `boot/std/mirror/glass/` migrates to `shards/mirror/ast/*`. The inner `glass/` segment was redundant once the source-tree folder was renamed away from `glass/`; `@mirror/glass/ast/*` remains the substrate-internal vocabulary for the AST representation (Stage 1 of #94's typed AST primitives), but the path on disk drops the duplicated segment.

The metalogue itself lives at `shards/metalogue.mirror` — top level. The substrate's self-declaration is the root of the tree.

### 3.4 Kintsugi-CI is paused

The kintsugi-CI work (T11.4–T11.8 in `[[kintsugi-ci-v0.1]]`; T11.2.7 to refactor Verdict on the rewritten primitives) is **paused** until T_shards.1 + T_shards.7 land. The deliverable surface from `[[kintsugi-ci-v0.1]]` §1.1 is unchanged — what's changing is the substrate the deliverable rides on.

`[[kintsugi-ci-v0.1]]` needs a substrate prerequisite note added: "T11.2.7 + T11.4 wait on T_shards.7 (`shards/kintsugi.mirror` with metalogue primitives)."

---

## 4. Connection to existing work

### 4.1 Task #104 — Phase 1 boot grammar completion

`roadmap/pending/phase-1-boot-grammar.md` names two open tasks: `!=` tokenization (Task 2) and singularity types (Task 4, blocked on spec). **The shards rewrite IS what completes Phase 1.** The reframe pulls the work from "patch boot/" to "rewrite into shards/" — same exit criterion (`mirror compile shards/` produces zero holonomy; all prisms parse, resolve, verify), different home.

The Phase 1 exit criterion updates to: **`mirror compile shards/` produces zero holonomy.** The `boot/` tree is retired in the final migration tick once every dependent file has been ported.

`!=` tokenization (Task 2) lands as part of T_shards.4 (the AST migration). Singularity types (Task 4) remain blocked on spec authoring; they're orthogonal to the metalogue and can land any tick post-T_shards.1.

### 4.2 `[[kintsugi-ci-v0.1]]` — needs substrate-prerequisite note

The kintsugi-CI spec at `docs/specs/kintsugi-ci-v0.1.md` §1.4 documents the mirror-text verdict surface as substrate-pull-correct. T11.2.7 (the typed Verdict refactor) is already noted there as a future tick. This roadmap closes the loop: T11.2.7 ≡ T_shards.7 (`shards/kintsugi.mirror`), waits on T_shards.1.

**Owed edit:** `[[kintsugi-ci-v0.1]]` §11 (post-v0.1 surface) gets a paragraph noting the substrate-prerequisite. Lands when T_shards.1 lands (so the spec doesn't reference unbuilt work).

### 4.3 `[[properties-on-glass]]` — alignment check

`docs/specs/properties-on-glass.md` declares the per-glass property binding mechanism using `verdict := pass | fail(diagnostic) | partial(f64, [diagnostic])` (see `[[properties-on-glass]]` §2.1). This shape **matches the metalogue's `@property_verdict`** declaration in §1.2 exactly:

- `pass` ≡ `@property_verdict::pass`
- `fail(diagnostic)` ≡ `@property_verdict::fail(@diagnostic)`
- `partial(f64, [diagnostic])` ≡ `@property_verdict::partial { confidence, diagnostics }`

The metalogue's `@confidence` newtype is the bounded-f64 refinement; properties-on-glass uses bare `f64`. **The metalogue's discipline is stricter** — it forces confidence into a newtype. Properties-on-glass should adopt the newtype when T_shards.5 lands the property surface on metalogue primitives.

(Note: the `[[properties-on-glass]]` spec's name still uses "glass" — that spec was authored before the shards rename and refers to per-`glass/ast/*` properties, which is substrate-internal vocabulary, not the source-tree folder. The naming stays consistent with `@mirror/glass/ast/*`.)

No conflict. The alignment is structural.

---

## 5. What stays in v0.1's actual ship

The v0.1 deliverable from `[[kintsugi-ci-v0.1]]` §1.1 — `uses: systemic-engineering/mirror/actions/kintsugi@v0.1` — ships **unchanged**. The user-facing surface, the action.yml inputs, the verdict shape on the wire, the fixture corpora, the cut criterion: all unchanged.

What changes is the substrate underneath:

- **Before shards-bootstrap:** the mirror-text verdict is an ad-hoc set of `<key> <value>` records; `Verdict` in Rust is a hand-coded struct; the discrimination state is `success | partial | failure` (the halfway-house `boot/std/kintsugi.mirror` declares).
- **After shards-bootstrap:** the mirror-text verdict serialises an `@imperfect(@verdict_value, @failure_reason, @transparency(@ref))` — the substrate-altitude shape from `shards/metalogue.mirror`. The Rust `Verdict` becomes the implementation of the prism declaration. Same wire format (or a structurally cleaner one — that's a T_shards.7 design decision).

**The cut criterion stays the same.** The two fixture corpora (`fixtures/kintsugi-pass/`, `fixtures/kintsugi-partial/`) produce the same verdict shape. The byte-identical determinism check still passes. The recursive self-host still holds.

The only externally visible change is potentially a structurally cleaner verdict shape on stdout — and that's an option, not a requirement.

---

## 6. What this roadmap does NOT decide

To stay disciplined:

- **The catastrophic-empty-map enforcement surface on `@opacity_map`.** The metalogue sketch uses `abstract newtype(...)` as the substrate-private constructor pattern; whether the substrate needs a richer "constructor visibility" surface or whether `abstract newtype` is enough lands at T_shards.1 design tick.
- **Whether `shards/metalogue.mirror` lives at the root of `shards/` or under `shards/mirror/`.** Reed's lean: the metalogue is THE substrate primitive of the whole language, so it sits at `shards/metalogue.mirror` (top level), not under any sub-namespace. The substrate's self-declaration is the root of the tree.
- **The `bootstrap/src/grammar.rs` loader's pivot from `boot/` to `shards/`** — happens at T_shards.8, not before. The bootstrap continues reading `boot/` until the parser substrate moves.
- **Order of T_shards.2 vs T_shards.3** — both depend only on T_shards.1; either can land first. Doesn't matter.

---

## 7. The discipline check (what makes this slow and steady)

The reframe risks turning into a 50-file rewrite sprint. The discipline that prevents that:

1. **One tick at a time.** T_shards.1 ships and stabilises before T_shards.2 begins. The substrate's own prism vocabulary settles first; everything else compounds.
2. **No backward-compat layers.** Per `[[feedback-no-compat-shim]]`: when a file ports from `boot/` to `shards/`, the `boot/` file is removed in the same tick. Two surfaces for the same thing is the halfway-house this whole rewrite exists to retire.
3. **TDD at the substrate altitude.** Per `[[feedback-always-tdd-no-shortcuts]]` + `[[feedback-write-red-in-session]]`: each tick writes a RED test against the new `shards/*` declaration first (`mirror compile shards/<file>` produces the expected OID; a downstream property check witnesses the new prism), then makes it green. Reed writes the RED; sub-agents land the GREEN.
4. **Substrate-pull-correct always.** Per `[[feedback-substrate-pull]]`: if a tick "needs" a Rust change to land a `shards/*` declaration, the substrate is being pulled the wrong way. The exception is `shards/metalogue.mirror` itself, which by definition is the moment the substrate teaches itself a new vocabulary — and even then, the work is prism declaration first, with the Rust impl already in place.

The exit gate for the whole sequence: `mirror compile shards/` produces zero holonomy; the bootstrap reads from `shards/` instead of `boot/`; the `boot/` tree is removed. **At that point, mirror has declared itself in mirror, all the way down.** And because the source tree IS shards — the substrate's own content-addressed unit — `mirror kintsugi shards/metalogue.mirror` is literally the substrate running on its own source code stored in its own data structure, navigable through the same MCP tools an agent uses for any other shard. The recursive proof is not metaphorical.

---

## 8. Open substrate questions surfaced during recon

These are not blockers — they are decisions that need to land in the T_shards.1 design tick or before:

1. **Does the `prism @X = newtype(T)` shorthand need a separate parsing surface, or does it desugar to `prism @X { focus T }` under the hood?** The metalogue sketch uses the shorthand for `@ref`, `@diagnostic`, `@confidence`. T_shards.1 confirms the desugaring.
2. **Cyclic dependencies between metalogue prisms.** Reviewed: `@property_verdict` depends on `@confidence` + `@diagnostic`; `@opacity_map` depends on `@ref` + `@property_verdict`; `@transparency` depends on `@opacity_map`; `@imperfect` depends on `@loss` only. **No cycles.** Declaration order in §1.1 is the topological sort.
3. **Does the bootstrap's grammar loader need to learn about `shards/` before T_shards.1 lands?** No. T_shards.1 lands a `.mirror` file under `shards/`; the bootstrap doesn't load it. The metalogue is declarative-only until T_shards.8 moves the loader. Between T_shards.1 and T_shards.8, `shards/*` is read by Reed + sub-agents + downstream specs; the bootstrap continues using `boot/`.
4. **Where does Phase 1 Task 2 (`!=` tokenization) land?** Folded into T_shards.4 (the AST migration). The fix is a one-line addition to `shards/mirror/ast/token.mirror`; the bootstrap reads it at startup once T_shards.8's loader pivots.
5. **How does the five-operations-come-for-free property interact with newtype prisms?** A `prism @ref = newtype(text)` automatically carries focus/project/split/lift/refract — what do they mean for a single-field newtype over text? The sketch's comments on `@loss`, `@transparency`, `@imperfect` are explicit; the newtype prisms inherit text's operations under the newtype wrapper. T_shards.1 confirms.

---

## 9. References

- `[[architecture-shards-as-substrate-source]]` — the 2026-06-04 correction that gave the source-tree folder its substrate-native name.
- `[[architecture-glass-wall-substrate-types]]` — the 2026-06-04 recognition that minted this reframe.
- `[[architecture-shard-as-crdt]]` — the substrate's vocabulary for "content-addressed unit it stores."
- `[[architecture-shard-ref-as-prism]]` — ShardRef as the typed session handle; the substrate's content-addressed handle algebra.
- `[[feedback-tokenizer-is-grammar-bootstrapped]]` — the discipline that says: grammar additions, not Rust changes.
- `[[architecture-fragmentation-is-the-rust-substrate]]` — the dependency direction this roadmap respects.
- `[[architecture-kintsugi-variety-io]]` — the variety-handling spec the metalogue's `@imperfect` + `@transparency` make operational.
- `[[kintsugi-ci-v0.1]]` — the v0.1 deliverable surface. Unchanged; substrate underneath rewritten.
- `[[properties-on-glass]]` — the per-glass property binding mechanism. The metalogue's `@property_verdict` is its substrate primitive.
- `roadmap/pending/phase-1-boot-grammar.md` — Task #104. The shards rewrite IS its completion.
- `prism/imperfect/src/{lib,transparency}.rs` — Rust source of truth for the metalogue's foundational prisms.
- `boot/00-prism.mirror` — the substrate's actual `prism` declaration form. The metalogue sketch is modeled on it.
- `boot/01-meta.mirror` — where `<=` first appears (as `fold <=(ref, imperfect)`); the metalogue lifts it to a prism-bound and an implements-relation.
- `boot/01a-error.mirror` — the record-style declaration; the metalogue uses the parametric variant style instead.
- `boot/std/{option,result,bool,text,list}.mirror` — idioms the new `shards/*` declarations port against (at a higher altitude than `00-prism.mirror`).
- `boot/std/mirror/glass/ast/*` — the existing AST tree that migrates to `shards/mirror/ast/*` at T_shards.4.
- `/Users/alexwolf/dev/projects/prism/docs/specs/pq.md` §6.5 — how the prism algebra plays at the wire altitude; the metalogue is the same algebra at the substrate altitude.

---

**The metalogue is the substrate's first sentence about itself. Everything else is what follows from it. And because the source lives in shards, the sentence is content-addressed; the substrate carries itself.**
