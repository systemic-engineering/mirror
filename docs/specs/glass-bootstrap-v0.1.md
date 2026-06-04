# glass-bootstrap-v0.1 — substrate rewrite roadmap

*2026-06-04. Reed. Status: roadmap, not implementation. No `.mirror` files land with this commit.*

**The reframe in one sentence.** The canonical folder for mirror files is now `glass/` (not `boot/`). Glass IS the substrate, and mirror declaring itself IS the glass wall. `boot/std/*.mirror` and `prism/*` become **reference material** — the templates the bottom-up rewrite ports against, not authoritative substrate.

**The pivot.** `glass/metalogue.mirror` lands first: the Batesonian metalogue of the language about itself. It declares the foundational types mirror uses to describe its own type system — `Imperfect<T, E, L>`, `Transparency<P>`, `PropertyVerdict`, `Diagnostic`, `Ref`. After the metalogue, work proceeds slow and steady, bottom-up: each tick ports another piece of `boot/` into `glass/` with proper substrate-pull discipline. The tokenizer becomes a `glass/*` declaration. The parser. The resolver. All of it.

**This IS v0.1's pipeline now.** The kintsugi-CI work (T11.4–T11.8 in `[[kintsugi-ci-v0.1]]`; T11.2.7 to refactor `Verdict` on top of the rewritten primitives) waits until the substrate primitives are declared.

---

## 0. Context — what this corrects

Three landings on the substrate over the last two weeks make this reframe load-bearing:

- `[[architecture-glass-wall-substrate-types]]` (2026-06-04): `Imperfect`, `Transparency`, `PropertyVerdict` ship as Rust crates only. Mirror's grammar **consumes** them but does NOT **declare** them. The Verdict roundtripping in `kintsugi --ci` cannot be properly typed until the substrate declares the primitives at the substrate altitude. T11.2.6's `discrimination = success | partial | failure` was the halfway-house Alex caught.
- `[[feedback-tokenizer-is-grammar-bootstrapped]]` (2026-06-03): the substrate is grammar-bootstrapped — adding a typed construct is a `.mirror` addition, not a Rust change. The tokenizer reads boot grammars at startup. T11.2.5 routed the wrong way; the right answer is grammar declaration.
- `boot/std/mirror/glass/*` already exists. Stage 1 of #94 landed the additive surface for `glass/ast`, `glass/ast/shape`, `glass/ast/token`. The legacy `@mirror/grammar` stays parallel during the migration. The substrate already knows the word `glass` and uses it as a type primitive.

The reframe pulls the structure all the way up: the **top-level folder** is `glass/`, not `boot/`. Existing `boot/std/mirror/glass/*` is a sub-tree under the legacy `boot/` root that should migrate to live at `glass/mirror/glass/*` (or `glass/mirror/ast/*` — see §3.3). The substrate naming aligns top-to-bottom.

---

## 1. The metalogue — what `glass/metalogue.mirror` contains

`glass/metalogue.mirror` is the foundational substrate declaration. It is the **metalogue** in Bateson's sense: a dialogue whose structure mirrors what it describes. The file declares — in mirror grammar — the primitives mirror uses to describe its own type system. The Rust crates `prism/imperfect/` and `prism/core/` BECOME the implementations of these declared types; the declarations become the source of truth.

### 1.1 The declaration order

Eight types, declared in dependency order:

1. **`loss`** — the monoid trait. Identity (`zero`), accumulation (`combine`), absorption (`total`).
2. **`ref`** — the `@`-prefixed substrate reference. Newtype over `text` with the leading-`@` invariant.
3. **`diagnostic`** — the per-issue message. Newtype over `text`. The no-bare-types discipline applied at the substrate edge.
4. **`confidence`** — `f64` in `[0.0, 1.0]`. Newtype. Lower = less confident.
5. **`property_verdict`** — per-location pass/partial/fail with confidence + diagnostics. The variant `partial` carries `confidence` + `[diagnostic]`; `fail` carries one `diagnostic`.
6. **`opacity_map`** — newtype over `map(ref, property_verdict)`. The crate-private constructor for `transparency`'s `opaque` variant — the structural enforcement of the catastrophic-empty-map invariant from `prism/imperfect/src/transparency.rs`.
7. **`transparency`** — the located-opacity loss. Two variants: `clear` (identity) and `opaque(opacity_map)` (the substrate's audit channel). The `loss` instance is structural.
8. **`imperfect`** — the three-state functor. Variants `success(t)`, `partial(t, l)`, `failure(e, l)`. Parametric in `t`, `e`, `l: loss`.

### 1.2 The sketch — actual mirror grammar

Modeled on the idiom in `boot/01a-error.mirror`'s record style + `boot/std/result.mirror`'s parametric variant style. **Sketch, not final**:

```mirror
in @prism
in @meta

# @metalogue — the substrate's declaration of its own variety vocabulary.
#
# Bateson's "metalogue": a dialogue whose structure mirrors what it
# describes. This file declares — in mirror grammar — the primitives
# mirror uses to describe its own type system. The Rust crates
# `prism/imperfect/` and `prism/core/` become the implementations of
# these declared types; the declarations are the source of truth.
#
# Cross-reference (each type's Rust home):
#
#   loss               → prism/imperfect/src/lib.rs (trait Loss)
#   ref                → prism/core/src/named.rs (Named<P>) + content_addressed
#   diagnostic         → prism/imperfect/src/transparency.rs (Diagnostic)
#   confidence         → newtype over f64 in [0.0, 1.0]
#   property_verdict   → prism/imperfect/src/transparency.rs (PropertyVerdict)
#   opacity_map        → prism/imperfect/src/transparency.rs (OpacityMap)
#   transparency       → prism/imperfect/src/transparency.rs (Transparency<P>)
#   imperfect          → prism/imperfect/src/lib.rs (Imperfect<T, E, L>)

grammar @metalogue {
  # ===== loss: the monoid trait =====
  # Loss forms a monoid: zero() is identity, combine is associative,
  # total() is the absorbing element. Used by every other variety
  # carrier in the substrate.
  abstract type loss {
    zero  : () -> loss
    total : () -> loss
    is_zero : loss -> bool
    combine : (loss, loss) -> loss
  }

  # ===== ref: the @-prefixed substrate reference =====
  # Newtype over text with the leading-@ invariant. The substrate's
  # naming primitive: every grammar, every type, every shape lookup
  # resolves through a ref.
  type ref(text)

  # ===== diagnostic: the no-bare-strings discipline =====
  type diagnostic(text)

  # ===== confidence: bounded f64 in [0.0, 1.0] =====
  type confidence(f64)

  # ===== property_verdict: per-location pass/partial/fail =====
  # The three shapes a property check produces at one substrate
  # location. Partial accumulates diagnostics; fail dominates.
  type property_verdict =
    | pass
    | partial { confidence: confidence, diagnostics: [diagnostic] }
    | fail(diagnostic)

  # ===== opacity_map: the crate-private newtype =====
  # The map carried inside transparency.opaque. The constructor is
  # substrate-private (per Seam I1); outside @metalogue nothing can
  # forge the catastrophic-empty-map sentinel.
  abstract type opacity_map(map(ref, property_verdict))

  # ===== transparency: located opacity as loss =====
  # Two variants: clear (the identity) and opaque (the audit channel).
  # Implements `loss`. Catastrophic absorption: opaque(empty_map) is
  # the absorbing element under combine. Only minted via total() or
  # by combine-driven absorption from within the grammar.
  type transparency =
    | clear
    | opaque(opacity_map)

  # transparency is a loss
  transparency <= loss

  # ===== imperfect: the three-state functor =====
  # Result extended with partial success. Success preserves; partial
  # carries the value with measured loss; failure carries the cost.
  # Parametric in (t, e, l: loss).
  type imperfect(t, e, l: loss) =
    | success(t)
    | partial(t, l)
    | failure(e, l)
}

out loss
out ref
out diagnostic
out confidence
out property_verdict
out opacity_map
out transparency
out imperfect
out @metalogue
```

### 1.3 Notes on the sketch

- **`abstract type loss`** — the substrate doesn't yet have a clean "trait" surface in mirror grammar. The `abstract` keyword from `boot/00-prism.mirror` is the closest existing idiom. The metalogue tick will need to decide: does mirror grammar admit method signatures inside `abstract type`, or does this lift to a `grammar @loss { ... }` block whose actions ARE the trait methods? The Rust impl tells us the four methods are `zero`, `total`, `is_zero`, `combine`; the grammar declaration form is the open question.
- **Newtypes via `type ref(text)`** — `boot/std/option.mirror` and `boot/std/result.mirror` already use this form. Substrate-native. The record-style declaration in `boot/01a-error.mirror` (`type loss = { bits, source, measurement }`) is the alternative; for single-field newtypes the parametric form reads cleaner.
- **`transparency <= loss`** — `boot/01-meta.mirror` declares `fold <=(ref, imperfect)`. The `<=` is the substrate-altitude "implements" relation. The grammar declaration "transparency is a loss" lands as `transparency <= loss`.
- **`opacity_map(map(ref, property_verdict))`** — `boot/std/map.mirror` exists with the `map(k, v)` form. The crate-private discipline lifts to mirror via the `abstract` keyword: outside `@metalogue`, the variant constructor cannot be named.
- **Catastrophic-empty-map invariant** — the Rust crate enforces this via `pub(crate)` on `OpacityMap`. In mirror, the equivalent is the `abstract` constructor + the `out` list: the grammar exports the type name but not the variant constructor. Substrate-pull-correct: the invariant moves from "Rust visibility modifier" to "substrate-level export discipline."

### 1.4 What the metalogue does NOT declare

To stay tight and avoid sprawl:

- **No `eh` / `imp` / `tri` bind operators.** Those are operations on `imperfect`, not part of the type's substrate declaration. They land in `glass/std/imperfect.mirror` once that file exists.
- **No `Metric` sub-trait.** `prism/imperfect/src/lib.rs` defines `Metric: Loss` with `distance_to` + `triangle`. The metric refinement is Connes' bounded-commutator condition; it's an addition to `loss`, not a foundational primitive. Lands at `glass/metric.mirror` later if a downstream type demands it.
- **No `Carrier` trait.** `prism/core/src/connection.rs` declares `Carrier: Clone + Default` with `compose` + `norm`. Used by `bundle.rs`. Lands at `glass/carrier.mirror` once the bundle altitude gets ported.
- **No `Luminosity`.** `prism/core/src/luminosity.rs` is a ternary (Light/Dimmed/Dark) used by `beam`. Adjacent but not foundational; can land in `glass/luminosity.mirror` when `glass/beam.mirror` ports.

The metalogue is **eight types**. Not nine; not seven. The eight foundational primitives any other substrate declaration in `glass/*` may depend on.

---

## 2. The porting order

### 2.1 Tick decomposition (loose)

| Tick | Lands | Depends on | Notes |
|---|---|---|---|
| **T_glass.1** | `glass/metalogue.mirror` | — | Eight types, one file. The substrate's self-declaration. |
| **T_glass.2** | `glass/option.mirror`, `glass/result.mirror`, `glass/bool.mirror` | T_glass.1 (only `imperfect`) | Naming surfaces on the kernel functor. Direct ports from `boot/std/`. |
| **T_glass.3** | `glass/std/list.mirror`, `glass/std/map.mirror`, `glass/std/set.mirror`, `glass/std/text.mirror`, `glass/std/number.mirror` | T_glass.1 | The pure-data collections; no metalogue dependency beyond `loss`. Can port any order. |
| **T_glass.4** | `glass/mirror/ast/token.mirror`, `glass/mirror/ast/shape/*.mirror` | T_glass.1, T_glass.3 | Migrate the existing `boot/std/mirror/glass/ast/*` tree to its substrate-altitude home at `glass/mirror/ast/`. Re-canonicalises the substrate's own AST as a `glass/*` declaration. |
| **T_glass.5** | `glass/epistemologic/property/*.mirror` | T_glass.1 (`property_verdict`), T_glass.4 | Verdict-returning property surface lands on the metalogue's `property_verdict`. Closes `[[properties-on-glass]]` §2.1's qualifier set on substrate-declared primitives. |
| **T_glass.6** | `glass/io.mirror`, `glass/file.mirror` (`@io` surface) | T_glass.1 | The IO grammars from `boot/02a-io.mirror` + `boot/std/file.mirror`. Substrate-pull discipline: `@io` lives in `glass/io.mirror` as the boundary. |
| **T_glass.7** | `glass/kintsugi.mirror` (`verdict` rewrite) | T_glass.1, T_glass.5 | T11.2.7 lands here. The mirror-text `verdict` from `[[kintsugi-ci-v0.1]]` §1.4 retypes as `imperfect(verdict_value, failure_reason, transparency(ref))`. |
| **T_glass.8** | `glass/mirror/tokenize.mirror`, `glass/mirror/parse.mirror`, `glass/mirror/resolve.mirror` | T_glass.4 | The substrate declares the tokenizer / parser / resolver behaviour as grammar. Rust impl in `bootstrap/src/*.rs` becomes implementation-of-declaration. Phase 2 of the road-to-1.0 lifts here. |
| **T_glass.9+** | Everything else under `boot/std/*` | varies | The remaining `boot/std/*` files (beam, peer, scheduler, cogito, sql, time, ...) port one tick at a time. Order driven by demand, not priority. |

### 2.2 Dependency-free ports (early or late, doesn't matter)

These have NO metalogue dependency beyond `loss` (which they don't measure against) — they can land any time after T_glass.1:

- `glass/bool.mirror` (trivial split)
- `glass/std/list.mirror`, `glass/std/set.mirror`, `glass/std/map.mirror`
- `glass/std/text.mirror`, `glass/std/number.mirror`, `glass/std/order.mirror`
- `glass/std/cli.mirror`, `glass/std/time.mirror`, `glass/std/git.mirror`

### 2.3 Metalogue-dependent ports

These wait on T_glass.1:

- `glass/option.mirror`, `glass/result.mirror` — both currently declare themselves "imperfect without loss / error" via `zoom lift` + `fold collapse`. They need `imperfect` declared first.
- `glass/epistemologic/property/*.mirror` — every property returns a `verdict` (≡ `property_verdict` from metalogue). The verdict union and `Transparency<Ref>` composition are the load-bearing primitives.
- `glass/kintsugi.mirror` — Verdict refactor (T11.2.7) requires `imperfect`, `transparency`, `property_verdict`, and `ref` declared at substrate altitude. Mara's `discrimination` halfway-house is replaced by the proper instance: `imperfect(verdict_value, failure_reason, transparency(ref))`.
- `glass/mirror/tokenize.mirror`, `glass/mirror/parse.mirror` — return `imperfect(ast, parse_error, transparency(ref))`. The tokenizer's substrate declaration ports here (T_glass.8); the Rust impl in `bootstrap/src/tokenize.rs` becomes implementation-of-declaration.

### 2.4 Where each `prism/*` crate's surface lands

| Rust crate | Glass declaration | Tick |
|---|---|---|
| `prism/imperfect/src/lib.rs` (`Loss`, `Imperfect`) | `glass/metalogue.mirror` | T_glass.1 |
| `prism/imperfect/src/transparency.rs` | `glass/metalogue.mirror` | T_glass.1 |
| `prism/core/src/named.rs` (`Ref`, `Named<P>`) | `glass/metalogue.mirror` (`ref` only); `Named<P>` lands later if needed | T_glass.1 |
| `prism/core/src/connection.rs` (`Carrier`) | `glass/carrier.mirror` (post-bundle) | T_glass.9+ |
| `prism/core/src/luminosity.rs` | `glass/luminosity.mirror` | T_glass.9+ |
| `prism/core/src/beam.rs` | `glass/beam.mirror` | T_glass.9+ |
| `prism/core/src/bundle.rs` | `glass/bundle.mirror` | T_glass.9+ |

The metalogue tick (T_glass.1) is the only tick that touches `prism/imperfect/`. Everything else lands later, demand-driven.

### 2.5 Where the tokenizer ports

The tokenizer's substrate-altitude declaration lives in **T_glass.8** (`glass/mirror/tokenize.mirror`). At that point, the Rust impl in `bootstrap/src/tokenize.rs` becomes the **implementation of a declaration** — same behaviour, but the source of truth shifts to the grammar. The `[[feedback-tokenizer-is-grammar-bootstrapped]]` discipline closes structurally: new keywords land in `glass/mirror/tokenize.mirror`, the bootstrap reads them at startup, no Rust touches.

The existing `boot/std/mirror/glass/ast/token.mirror` is the **template** the T_glass.8 port draws on. The lexical primitives there (`whitespace`, `identifier`, `reference`, etc.) carry forward; the new home is `glass/mirror/ast/token.mirror` (T_glass.4 migration).

---

## 3. The reference-material framing

This is the discipline that holds throughout the rewrite:

### 3.1 `boot/std/*` becomes read-only reference

Existing `boot/`, `boot/std/`, `boot/std/mirror/glass/` files are **templates the rewrite ports against**. They are not edited; new files land in `glass/` instead. The bootstrap continues to read `boot/` until T_glass.8 lands `glass/mirror/tokenize.mirror`, at which point the bootstrap's grammar loader gets pointed at `glass/`.

**The migration mechanic.** Each ported file lands in `glass/` with the same logical content but the corrected substrate-altitude shape (e.g., using `imperfect` from metalogue rather than ad-hoc enums). The old `boot/` file is removed in the SAME tick (no parallel surfaces; the substrate doesn't need a backward-compat layer for pre-v0.1).

### 3.2 `prism/*` Rust crates remain the implementation substrate

Per `[[architecture-fragmentation-is-the-rust-substrate]]`: `prism_core` stays deps-free; `prism/imperfect/` remains the canonical Rust home of `Imperfect`, `Transparency`, `PropertyVerdict`, `Diagnostic`. The Rust types ARE the runtime behaviour of the `glass/metalogue.mirror` declarations. No new crates are minted for this work; no existing crates are retired.

What changes is the **direction of truth**: today, the Rust types are the source of truth and `boot/*.mirror` consumes them ad-hoc. After T_glass.1, `glass/metalogue.mirror` is the source of truth and the Rust types are the implementation. The glass wall becomes load-bearing.

### 3.3 The folder name question (open)

The existing `boot/std/mirror/glass/` is a *path that already uses the word "glass"*. The natural top-level migration target is:

- **Option A:** `glass/mirror/glass/ast/*` — preserves the existing path structure verbatim, just under a new root. Awkward double-glass.
- **Option B:** `glass/mirror/ast/*` — drops the redundant inner `glass/`, since the top-level folder name already carries it. Cleaner.

This roadmap assumes **Option B**. The migration tick (T_glass.4) will need to confirm with Reed before landing.

### 3.4 Kintsugi-CI is paused

The kintsugi-CI work (T11.4–T11.8 in `[[kintsugi-ci-v0.1]]`; T11.2.7 to refactor Verdict on the rewritten primitives) is **paused** until T_glass.1 + T_glass.7 land. The deliverable surface from `[[kintsugi-ci-v0.1]]` §1.1 is unchanged — what's changing is the substrate the deliverable rides on.

`[[kintsugi-ci-v0.1]]` needs a substrate prerequisite note added: "T11.2.7 + T11.4 wait on T_glass.7 (`glass/kintsugi.mirror` with metalogue primitives)."

---

## 4. Connection to existing work

### 4.1 Task #104 — Phase 1 boot grammar completion

`roadmap/pending/phase-1-boot-grammar.md` names two open tasks: `!=` tokenization (Task 2) and singularity types (Task 4, blocked on spec). **The glass rewrite IS what completes Phase 1.** The reframe pulls the work from "patch boot/" to "rewrite into glass/" — same exit criterion (`mirror compile glass/` produces zero holonomy; all grammars parse, resolve, verify), different home.

The Phase 1 exit criterion updates to: **`mirror compile glass/` produces zero holonomy.** The `boot/` tree is retired in the final migration tick once every dependent file has been ported.

`!=` tokenization (Task 2) lands as part of T_glass.4 (the AST glass migration). Singularity types (Task 4) remain blocked on spec authoring; they're orthogonal to the metalogue and can land any tick post-T_glass.1.

### 4.2 `[[kintsugi-ci-v0.1]]` — needs substrate-prerequisite note

The kintsugi-CI spec at `docs/specs/kintsugi-ci-v0.1.md` §1.4 documents the mirror-text verdict surface as substrate-pull-correct. T11.2.7 (the typed Verdict refactor) is already noted there as a future tick. This roadmap closes the loop: T11.2.7 ≡ T_glass.7 (`glass/kintsugi.mirror`), waits on T_glass.1.

**Owed edit:** `[[kintsugi-ci-v0.1]]` §11 (post-v0.1 surface) gets a paragraph noting the substrate-prerequisite. Lands when T_glass.1 lands (so the spec doesn't reference unbuilt work).

### 4.3 `[[properties-on-glass]]` — alignment check

`docs/specs/properties-on-glass.md` declares the per-glass property binding mechanism using `verdict := pass | fail(diagnostic) | partial(f64, [diagnostic])` (see `[[properties-on-glass]]` §2.1). This shape **matches the metalogue's `property_verdict`** declaration in §1.2 exactly:

- `pass` ≡ `property_verdict::pass`
- `fail(diagnostic)` ≡ `property_verdict::fail(diagnostic)`
- `partial(f64, [diagnostic])` ≡ `property_verdict::partial { confidence, diagnostics }`

The metalogue's `confidence` newtype is the bounded-f64 refinement; properties-on-glass uses bare `f64`. **The metalogue's discipline is stricter** — it forces confidence into a newtype. Properties-on-glass should adopt the newtype when T_glass.5 lands the property surface on metalogue primitives.

No conflict. The alignment is structural.

---

## 5. What stays in v0.1's actual ship

The v0.1 deliverable from `[[kintsugi-ci-v0.1]]` §1.1 — `uses: systemic-engineering/mirror/actions/kintsugi@v0.1` — ships **unchanged**. The user-facing surface, the action.yml inputs, the verdict shape on the wire, the fixture corpora, the cut criterion: all unchanged.

What changes is the substrate underneath:

- **Before glass-bootstrap:** the mirror-text verdict is an ad-hoc set of `<key> <value>` records; `Verdict` in Rust is a hand-coded struct; the discrimination state is `success | partial | failure` (the halfway-house `boot/std/kintsugi.mirror` declares).
- **After glass-bootstrap:** the mirror-text verdict serialises an `imperfect(verdict_value, failure_reason, transparency(ref))` — the substrate-altitude shape from `glass/metalogue.mirror`. The Rust `Verdict` becomes the implementation of the grammar declaration. Same wire format (or a structurally cleaner one — that's a T_glass.7 design decision).

**The cut criterion stays the same.** The two fixture corpora (`fixtures/kintsugi-pass/`, `fixtures/kintsugi-partial/`) produce the same verdict shape. The byte-identical determinism check still passes. The recursive self-host still holds.

The only externally visible change is potentially a structurally cleaner verdict shape on stdout — and that's an option, not a requirement.

---

## 6. What this roadmap does NOT decide

To stay disciplined:

- **The exact grammar of `abstract type loss`** — does mirror grammar admit method signatures inside `abstract type`, or does the trait lift to a `grammar` block? Decided at T_glass.1 design tick.
- **Folder layout Option A vs Option B** (see §3.3). Decided at T_glass.4 before the AST migration lands.
- **Whether `glass/metalogue.mirror` lives at the root of `glass/` or under `glass/mirror/`.** Reed's lean: the metalogue is THE substrate primitive of the whole language, so it sits at `glass/metalogue.mirror` (top level), not under any sub-namespace. The substrate's self-declaration is the root of the tree.
- **The `bootstrap/src/grammar.rs` loader's pivot from `boot/` to `glass/`** — happens at T_glass.8, not before. The bootstrap continues reading `boot/` until the parser substrate moves.
- **Order of T_glass.2 vs T_glass.3** — both depend only on T_glass.1; either can land first. Doesn't matter.

---

## 7. The discipline check (what makes this slow and steady)

The reframe risks turning into a 50-file rewrite sprint. The discipline that prevents that:

1. **One tick at a time.** T_glass.1 ships and stabilises before T_glass.2 begins. The substrate's own type vocabulary settles first; everything else compounds.
2. **No backward-compat layers.** Per `[[feedback-no-compat-shim]]`: when a file ports from `boot/` to `glass/`, the `boot/` file is removed in the same tick. Two surfaces for the same thing is the halfway-house this whole rewrite exists to retire.
3. **TDD at the substrate altitude.** Per `[[feedback-always-tdd-no-shortcuts]]` + `[[feedback-write-red-in-session]]`: each tick writes a RED test against the new `glass/*` declaration first (`mirror compile glass/<file>` produces the expected OID; a downstream property check witnesses the new type), then makes it green. Reed writes the RED; sub-agents land the GREEN.
4. **Substrate-pull-correct always.** Per `[[feedback-substrate-pull]]`: if a tick "needs" a Rust change to land a `glass/*` declaration, the substrate is being pulled the wrong way. The exception is `glass/metalogue.mirror` itself, which by definition is the moment the substrate teaches itself a new vocabulary — and even then, the work is grammar declaration first, with the Rust impl already in place.

The exit gate for the whole sequence: `mirror compile glass/` produces zero holonomy; the bootstrap reads from `glass/` instead of `boot/`; the `boot/` tree is removed. **At that point, mirror has declared itself in mirror, all the way down.** The glass wall is the substrate.

---

## 8. Open substrate questions surfaced during recon

These are not blockers — they are decisions that need to land in the T_glass.1 design tick or before:

1. **Does `Transparency` need a separate `Loss` trait declaration first, or can the trait + impl land in the same metalogue file?** The Rust crate ships them together (`prism/imperfect/src/{lib,transparency}.rs`). The metalogue sketch above puts them in one file. This roadmap assumes that's correct; T_glass.1 confirms.
2. **Cyclic dependencies between metalogue primitives.** Reviewed: `property_verdict` depends on `confidence` + `diagnostic`; `opacity_map` depends on `ref` + `property_verdict`; `transparency` depends on `opacity_map`; `imperfect` depends on `loss` only. **No cycles.** Declaration order in §1.1 is the topological sort.
3. **Does the bootstrap's grammar loader need to learn about `glass/` before T_glass.1 lands?** No. T_glass.1 lands a `.mirror` file under `glass/`; the bootstrap doesn't load it. The metalogue is declarative-only until T_glass.8 moves the loader. Between T_glass.1 and T_glass.8, `glass/*` is read by Reed + sub-agents + downstream specs; the bootstrap continues using `boot/`.
4. **Where does Phase 1 Task 2 (`!=` tokenization) land?** Folded into T_glass.4 (the AST migration). The fix is a one-line addition to `glass/mirror/ast/token.mirror`; the bootstrap reads it at startup once T_glass.8's loader pivots.

---

## 9. References

- `[[architecture-glass-wall-substrate-types]]` — the 2026-06-04 recognition that minted this reframe.
- `[[feedback-tokenizer-is-grammar-bootstrapped]]` — the discipline that says: grammar additions, not Rust changes.
- `[[architecture-fragmentation-is-the-rust-substrate]]` — the dependency direction this roadmap respects.
- `[[architecture-kintsugi-variety-io]]` — the variety-handling spec the metalogue's `imperfect` + `transparency` make operational.
- `[[kintsugi-ci-v0.1]]` — the v0.1 deliverable surface. Unchanged; substrate underneath rewritten.
- `[[properties-on-glass]]` — the per-glass property binding mechanism. The metalogue's `property_verdict` is its substrate primitive.
- `roadmap/pending/phase-1-boot-grammar.md` — Task #104. The glass rewrite IS its completion.
- `prism/imperfect/src/{lib,transparency}.rs` — Rust source of truth for the metalogue's foundational types.
- `boot/std/{option,result,bool,text,list}.mirror` — idioms the new `glass/*` declarations port against.
- `boot/std/mirror/glass/ast/*` — the existing glass tree that migrates to `glass/mirror/ast/*` at T_glass.4.

---

**The metalogue is the substrate's first sentence about itself. Everything else is what follows from it.**
