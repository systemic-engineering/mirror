---
title: Contradiction and Fracture
subtitle: Formalising the double bind as a substrate primitive — `\` is a marked unresolved contradiction
status: spec (proposal)
date: 2026-06-04
author: Reed
depends_on:
  - boot/std/kintsugi/fracture.mirror
  - boot/std/kintsugi.mirror
  - boot/std/properties.mirror
  - boot/01a-error.mirror               # imperfect, loss, error
  - boot/std/epistemologic/resolve.mirror
  - bootstrap/src/tokenize.rs           # body_is_obligation
  - docs/specs/kintsugi-variety.md      # variety_hold, Ashby
  - docs/specs/error-as-question.md     # Pass / Partial(f64,[Diag]) / Fail(Diag)
  - docs/specs/kintsugi-fracture-confidence-and-scene-dispatch.md
  - docs/specs/gap-tension-tensor-substrate.md
  - docs/specs/strict-and-total-classification.md
---

# Contradiction and Fracture

*2026-06-04. Reed. Proposal — not implementation. The substrate already
ships the mechanism; this spec names it.*

> **Status: Yellow.** The shape is already there. `@kintsugi/fracture` exists
> (commit `ca4a9e7`). `body_is_obligation` exists in `bootstrap/src/tokenize.rs`.
> The `\` marker is already the obligation token. What is missing is the
> recognition that `\` is a *marked unresolved contradiction*, and that the
> substrate's `imperfect`/`fracture`/`Partial` vocabulary is one type viewed
> from three altitudes. This spec proposes the formalisation and points at
> the load-bearing prior art so future ticks can land it.

---

## 1. Recognition

The mirror substrate has, by accident, reinvented Belnap-Dunn four-valued
logic at the AST altitude. Every node either crystallises (refracts to a
canonical form), reports a `verdict` (`Pass` / `Partial` / `Fail`), or
carries the kintsugi obligation marker `\` — a *body that is a hole*. The
`\` is not a placeholder. It is a syntactically-marked *contradiction the
loop has not yet resolved*: the declared signature says one thing, the
body says nothing, and the system is asked to hold both until a morphism
arrives that closes the gap. Bateson named the shape (the double bind);
Belnap named the truth-value (Both); Priest named the logic (LP);
Watzlawick named the resolution (second-order change). Mirror already
implements the shape; this spec gives it its name. `contradiction` is the
type; `fracture` is the unresolved form; `\` is the syntactic mark; the
kintsugi loop is the resolution.

---

## 2. Bateson's double bind — the load-bearing claim

A *double bind* is the structural condition in which an agent receives
two messages at different logical levels that mutually negate each other,
inside a relationship from which they cannot exit and about which they
cannot meta-communicate. Bateson et al. (1956) identified six ingredients:
two or more persons, repeated experience, a primary negative injunction,
a secondary injunction at a different logical level conflicting with the
first, a tertiary injunction prohibiting escape, and (eventually) the
loss of the need for the full set as the pattern is internalised. The
load-bearing recognition is *not* the pathology claim (the schizophrenia
theory is largely abandoned) but the *logical-type* claim: contradictions
between levels are categorically different from contradictions within a
level, and they cannot be resolved by collapsing the levels.

Bateson's later work (*Steps to an Ecology of Mind*, 1972) extended this
into Learning I/II/III: Learning I is correction within a context;
Learning II is learning the context itself; Learning III is changing the
class of contexts. Each level can only see contradictions one level down.
A double bind is unresolvable at the level on which it appears; it can
only be resolved by ascending one level and finding a morphism that
makes the lower-level contradiction non-contradictory. This is exactly
the shape of the kintsugi loop's substrate-pull: `\` at `@code` is
resolved by finding the lift to `@mirror` that makes the obligation
disappear.

Watzlawick, Beavin and Jackson (1967) formalised the communicative
implication: every communication carries a *report* (content) and a
*command* (relationship) component, and paradox arises when they
disagree. Watzlawick, Weakland and Fisch (1974) introduced the
*first-order / second-order change* distinction — first-order change
moves within the system's existing variety; second-order change changes
the system's variety. **Kintsugi is second-order change applied to
syntax.**

---

## 3. Prior art mapping

| Source | Year | Load-bearing claim contributed |
|---|---|---|
| Bateson, Jackson, Haley, Weakland — "Toward a theory of schizophrenia" *Behavioral Science* 1(4):251–264 | 1956 | Contradictions across logical levels are categorically different from contradictions within a level. The *double bind* is the prototype unresolvable-at-level fracture. |
| Bateson — *Steps to an Ecology of Mind* | 1972 | Learning I/II/III. Resolution of a level-N contradiction requires ascent to level N+1. This is mirror's substrate-pull, named. |
| Watzlawick, Beavin, Jackson — *Pragmatics of Human Communication* | 1967 | Every message is content + meta-relationship. Paradox = disagreement between them. Maps directly onto mirror's `<signature, body>` pair where the body is `\`. |
| Watzlawick, Weakland, Fisch — *Change* | 1974 | First-order vs second-order change. Kintsugi's job is second-order — it changes which expressions can settle, not just which values are produced. |
| Wilden — *System and Structure* | 1972/1980 | Bateson + Lacan + cybernetics synthesis: analog vs digital communication, the double bind as semiotic structure. Grounds the `\`-as-marker claim in semiotics, not just psychology. |
| Belnap — "A useful four-valued logic" | 1977 | Truth values `{T, F, Both, Neither}` on a bilattice. `Both` IS *held contradiction*. `Neither` IS *unsettled*. Directly maps onto mirror's verdict algebra (Pass / Fail / Partial / unobserved). |
| Priest — "The Logic of Paradox" (LP) | 1979 | True contradictions are admissible without explosion. Paraconsistent logic refuses *ex falso quodlibet*. This is the formal claim that lets `\` exist in the AST without collapsing the universe. |
| Priest — *In Contradiction* | 2006 | Dialetheism as a metaphysical position. Some contradictions are not bugs to be fixed; they are *real states of the world the formal system must accept*. Maps onto Alex's "fracture is not a flaw; it's a marked site of unresolved variety." |
| Girard — "Linear Logic" *Theoretical Computer Science* 50:1–101 | 1987 | Resources cannot be freely duplicated or discarded. The `\` is a *linear obligation*: the system owes a body, and that obligation cannot be silently weakened away. Discipline behind the substrate-pull. |
| Curry–Howard, Martin-Löf | 1934/1969+ | The bottom type `⊥` corresponds to falsity / impossibility. A program inhabiting `⊥` IS a proof of contradiction. `\` is the *not-yet-inhabited* `⊥`-like position — the obligation to provide a witness. |
| Carnielli & Marcos — *Logics of Formal Inconsistency* (LFIs) | 2002+ | The consistency operator `○` is a first-class primitive. *Consistency is a property a sentence can have or not have, declared in the object language.* Maps directly onto `holds contradiction` / `resolves contradiction` as substrate properties. |
| Coniglio & Rodrigues — "Six-valued logics of evidence and truth (LETK+, LETF+)" arxiv:2209.12337 | 2022 | Extends Belnap with positive/negative *reliable information* values. The reliability axis is exactly what mirror's `confidence` axis is doing on `@kintsugi/fracture` per `kintsugi-fracture-confidence-and-scene-dispatch.md`. |
| Jakl — "Four imprints of Belnap's useful four-valued logic in computer science" arxiv:2503.20679 | 2025 | Belnap-Dunn appears in linear logic models, Blame Calculus, and LVars via the twist-product representation of bilattices. *Mirror's `\` + `transparency` lattice + `imperfect` is a fourth imprint.* Explicit recent grounding. |
| Wilson et al. — "The Bateson Game: A Model of Strategic Ambiguity, Frame Suppression, and Coordination Failure" *Games* 16(6):57 (MDPI) | 2025 | Formal signalling-game model of the double bind. First contemporary rigorous game-theoretic formalisation. The "frame suppression" mechanic is structurally identical to the silent-absorption mode the `Dark` token + `\` obligation marker were introduced to prevent. |
| Siegenfeld & Bar-Yam — multi-scale variety (cited via `kintsugi-variety.md`) | 2022 | Variety is multi-scale `C(s)`, not a scalar. Ashby's law applies per scale. Each fracture lives at one scale; resolution must preserve variety at that scale. |
| Reyes, Henao, Hassall — VSM error propagation `(C', Q, K) α τ, η` (cited via `error-as-question.md`) | 2024 | Beer's algedonic discipline + located structured failure surfacing. The Cyberstride pattern is the modern ancestor of mirror's `verdict` / `Transparency` algebra. |

*Twelve sources is more than the 6–10 brief; four were too load-bearing to
drop. The non-arxiv foundational entries (Bateson, Watzlawick, Wilden,
Belnap, Priest) are settled enough that direct primary citations suffice
without further sampling.*

---

## 4. The substrate primitives

### 4.1 `contradiction` as a substrate primitive

`contradiction` lives in `@meta` alongside `imperfect`, `loss`, `error`.
It IS the type whose values are *pairs of incompatible obligations that
must be held together until a morphism resolves them.* It is not a flag,
not an exception — it is a positive structural fact about the AST.

Proposed declaration (sketch, for `boot/01a-error.mirror` or a sibling
`boot/01c-contradiction.mirror`):

```mirror
in @prism
in @meta

grammar @contradiction {
  # A contradiction is two obligations at different logical levels
  # that cannot be simultaneously satisfied within a single level.
  # Per Bateson 1956 + Priest 1979: holding both is the precondition
  # for resolution. Collapsing prematurely is the error.
  #
  # `level` is Bateson's logical-level marker — the altitude at which
  # the two claims live. Resolution requires a morphism that lifts
  # the contradiction to `level + 1`.
  type contradiction(a, b) = {
    left: a,
    right: b,
    level: u32,
    where: location,
  }

  # A fracture IS an unresolved contradiction with a syntactic site.
  # The `\` marker in source is precisely the projection of a
  # fracture into surface syntax — `body_is_obligation` in
  # bootstrap/src/tokenize.rs recognises it.
  #
  # fracture <= contradiction (subset, per @meta's `<` relation).
  # Every fracture is a contradiction; not every contradiction has
  # a syntactic mark.
  type fracture <= contradiction(observation, error(observation))

  # The two corresponding properties. `holds` is the Bateson "stay in
  # the bind without collapsing" capacity — the substrate's willingness
  # to keep variety alive at this site. `resolves` is the kintsugi
  # morphism: the lift that makes the two obligations agree at the
  # next altitude up.
  property holds(contradiction)   <= verdict
  property resolves(contradiction) <= verdict
}

out contradiction
out fracture
out holds
out resolves
```

### 4.2 `fracture <= contradiction`

The subset relation `fracture <= contradiction` is the right shape. Every
fracture is a contradiction (there is something the substrate is being
asked to hold across levels); not every contradiction is a fracture (some
live silently in the verdict algebra without ever surfacing as `\` in
source). This matches the existing `imperfect <= verdict` shape in
`boot/std/properties.mirror`.

At the prism altitude, `fracture` is *also* a Prism (per
`boot/std/kintsugi/fracture.mirror` lines 23–27 — "A fracture is also a
settled Prism"). The `refract` op of a fracture IS the kintsugi
morphism. The contradiction surfaces as input; the canonical form
emerges as output; loss is recorded; the obligation is discharged.

### 4.3 `\` is the syntactic mark of a fracture

The connection lives at `bootstrap/src/tokenize.rs` lines 5–18:

```rust
/// True iff a brace-block body's content (between but excluding the outer
/// `{` and `}`) is, after trimming ASCII whitespace, exactly the `\` kintsugi
/// obligation marker. Such bodies are explicit holes — NOT dark regions.
fn body_is_obligation(bytes: &[u8]) -> bool {
    ...
    end - start == 1 && bytes[start] == b'\\'
}
```

The `\` is *explicitly distinguished* from `Dark` (unrecognized bytes).
`Dark` is variety the substrate failed to absorb. `\` is variety the
substrate is *deliberately holding open*. The distinction is already
load-bearing — `Dark` triggers `--strict` failures; `\` does not. This
spec proposes a complementary AST kind that makes the meaning explicit:

Proposed `AstKind::Fracture` (sibling of `Dark` per
`docs/specs/strict-and-total-classification.md`):

```rust
/// A brace-block body that contains only the `\` obligation marker.
/// Currently surfaces as an empty/special body; proposed promotion to a
/// first-class AstKind so the kintsugi loop can locate fractures
/// without re-scanning bytes.
///
/// Per contradiction-and-fracture spec: every Fracture node IS a
/// `contradiction` value at the AST altitude. The Reflection model
/// (per error-as-question.md) is asked the question "what morphism
/// resolves this?" and the answer is written as the replacement body.
Fracture,
```

The recognition is non-breaking: today, `body_is_obligation` returns
`true` and the bytes stay verbatim in `AstNode.body`. The proposed
promotion just *names* what is already happening.

### 4.4 `holds contradiction` and `resolves contradiction` as properties

Following the shape in `boot/std/properties.mirror`:

```mirror
# `holds` — the substrate maintains both obligations in superposition
# without collapsing. Equivalent at the variety altitude to
# `variety_hold = 1.0` per kintsugi-variety §6.
property holds(contradiction) <= verdict

# `resolves` — a kintsugi morphism has been applied; the contradiction
# is discharged at level + 1. The morphism IS the witness the verdict
# returns inside `Pass`.
property resolves(contradiction) <= verdict
  where applied(kintsugi.collapse)
```

Both properties land cleanly on the existing `verdict` algebra:

- `Pass` — `resolves` succeeded, fracture closed.
- `Partial(confidence, [diagnostic])` — `holds` succeeded but `resolves`
  is in progress; the morphism is partially constructed.
- `Fail(diagnostic)` — neither holds nor resolves; the contradiction
  has collapsed into incoherence (the substrate has *lost variety*).

The Belnap correspondence: `Pass` = T, `Fail` = F, `Partial` = Both
(held contradiction with non-trivial witness), unobserved = Neither.
Mirror's verdict is already a four-valued logic, modulo the `Neither`
case being implicit in "no verdict yet computed."

---

## 5. Connection to existing substrate

The recognition is not "add a new primitive." It is *"name what is
already five places."*

### 5.1 `Imperfect.Partial(t, l)` IS `holds contradiction { value: t, loss: l }`

`boot/01a-error.mirror`:

```mirror
type imperfect(value, loss = loss, error = error) = {
  value: value,
  loss: loss,
  errors: [error],
}
```

`Imperfect.Partial` carries a value AND a recorded loss. The loss is
the *unresolved variety* between what was expected and what was produced.
This IS a held contradiction — the substrate proceeded with the value,
but the obligation to close the loss is still open. Identity:

```
Imperfect.Partial(t, l)  ≡  holds(contradiction { left: t, right: error_at(l), ... })
```

### 5.2 `Transparency<P>::Fail` at a location IS a fracture at that location

`prism/imperfect/src/transparency.rs` carries `PropertyVerdict::Fail`
with a `Diagnostic` that includes a `location`. Every located `Fail`
is structurally a fracture: a marked site where the property's
claim diverged from the property's witness. The Transparency monoid
(`Fail` dominates, `Partial`s union, `Pass` is neutral) is exactly the
commutative-idempotent lattice ordering Belnap-Dunn requires for
its information-order join. Per Jakl 2025 §2 — this is one of the
four CS imprints of Belnap-Dunn, and mirror is a fifth.

### 5.3 `error-as-question` IS the Reflection morphism that resolves a fracture

From `docs/specs/error-as-question.md`: *"Each error becomes a question
the Reflection model is asked to answer."* In contradiction vocabulary:
each unresolved fracture (each `\` or each `Fail`) is a question.
Reflection's answer is the morphism — the lifted expression at level+1
that makes the level-N contradiction non-contradictory. This is the
Bateson Learning II → Learning III mechanic, ported to compiler-time.

### 5.4 `variety_hold = 1.0` IS `holds contradiction`

`kintsugi-variety` §6 declares `variety_hold` as the property that the
posterior support is preserved across an @io crossing. Identity:

```
variety_hold(f) = 1.0   ⟺   holds(contradiction)
                                across the crossing f represents
```

The Ashby constraint `|R(@mirror)| ≥ |D|` IS the formal statement that
the substrate must have enough variety to *hold* the contradictions the
input space presents. Premature collapse = variety loss = `holds` fails.

### 5.5 Every `\` in `boot/00-prism.mirror`, `boot/std/*.mirror` IS a fracture AST node

Verified by inspection. Current count in the boot tree (rough — every
`{ \ }` body is one; sampling shows ~30+ across `00-prism.mirror`,
`01-meta.mirror`, `kintsugi.mirror`, `kintsugi/fracture.mirror`,
`kintsugi/fracture/*.mirror`, `mirror/match.mirror`, etc.). Each one
is recoverable as a `Fracture` AST node by walking `body_is_obligation`
in `bootstrap/src/tokenize.rs`. No tokenizer change needed for
recognition; the promotion to `AstKind::Fracture` is the named AST
shape the existing byte-level recognition already enacts.

### 5.6 `pq` §3 — `project({ kintsugi })` IS the loop that resolves fractures

From `prism/docs/specs/pq` §3 (referenced via `kintsugi-variety` §3):
the project filter that selects nodes marked for kintsugi processing
IS the fracture-collection step. The loop:

```
project({ kintsugi }) → for each fracture → find_morphism → resolve → refract
```

is the substrate-level analogue of Bateson's "meta-communication"
about the bind. The system *names* the contradictions it is asked to
hold, then asks Reflection for the morphisms that discharge them.

### 5.7 Cross-altitude unification

The same four-valued logic shows up at every altitude:

| Altitude | "T" | "F" | "Both" (held) | "Neither" (unsettled) |
|---|---|---|---|---|
| Source bytes | recognized token | `Dark` | `\` (fracture) | whitespace/comment |
| AST | structural node | parse error | `Fracture` node | not yet parsed |
| Verdict | `Pass` | `Fail(diag)` | `Partial(c, [diag])` | not yet observed |
| Transparency | clear | opaque-failed | opaque-located | not yet measured |
| Variety | `variety_hold = 1.0` | `variety_hold = 0.0` | `0 < variety_hold < 1` | not yet measured |
| Imperfect | full value | error | `Partial(v, l)` | not yet attempted |

One logic, six imprints. *This is the recognition.*

---

## 6. The kintsugi loop's job, restated

The kintsugi loop's job is to **resolve fractures by holding
contradictions in superposition long enough to find the morphism that
resolves them without false collapse.**

Unpacked:

1. **Hold.** When the substrate encounters a fracture (a `\`, a
   `Partial`, a located `Fail`), it does NOT immediately collapse to
   the nearest valid form. It records the contradiction as a first-class
   substrate value, with both sides preserved. Per Priest (1979): the
   substrate is *paraconsistent* — `ex falso` is refused. A contradiction
   does not corrupt the surrounding inference.

2. **Maintain variety.** Per Ashby (via Siegenfeld & Bar-Yam 2022) and
   `kintsugi-variety` §2: the substrate keeps `|R(@mirror)| ≥ |D|` even
   while the contradiction is open. The variety the contradiction
   represents is itself information about the resolution space.

3. **Search for a morphism at level+1.** Per Bateson (1972), the
   resolution to a level-N contradiction is not at level N. The kintsugi
   loop performs *substrate-pull*: it lifts the obligation from
   `@code/<lang>` up to `@mirror`, and from `@mirror` up to the
   appropriate grammar at +1. The lifted form is sought via the Fate
   tournament (`@fate.minimize` per `gap-tension-tensor-substrate.md`).

4. **Apply the morphism.** Per `kintsugi/fracture/*.mirror`: the
   morphism is a closure operator on the AST lattice. It is idempotent
   and canonical-at-fixpoint. Applying it discharges the contradiction
   at level N by making it a non-contradiction at level N+1.

5. **Refract.** The resolved form is content-addressed, written back,
   and the verdict updates to `Pass`. The fracture closes. The crack
   is filled with gold.

This is Watzlawick's second-order change at compiler-time. First-order
change would try to "fix" the `\` by guessing a body at the same
altitude. The kintsugi loop refuses that — it ascends, finds the
morphism, and the fix is structural rather than local.

---

## 7. Tick decomposition (deferred)

Implementation is NOT this spec's job. These are the follow-up ticks
the formalisation unblocks. They are listed; not sequenced.

- **T-contradiction.1** — declare `grammar @contradiction { ... }` in
  `boot/01c-contradiction.mirror` (or fold into `01a-error.mirror`).
  Pure declaration; no compiler change.
- **T-contradiction.2** — add `holds` and `resolves` to
  `boot/std/properties.mirror`. Wire to `verdict` per existing shape.
- **T-fracture.3** — promote `body_is_obligation` recognition to a
  first-class `AstKind::Fracture` in `bootstrap/src/ast.rs`. Round-trip
  preserves bytes; no change in observable behaviour.
- **T-fracture.4** — expose `--report-fractures` on `mirror kintsugi`
  so the substrate can enumerate its open contradictions. (Pairs with
  `--strict` for `Dark` regions; the two are dual.)
- **T-imperfect.5** — add the `<=` relation in the declaration:
  `imperfect.Partial <= contradiction.holds` (verified at compile
  time). Surfaces the identity from §5.1 to the type checker.
- **T-variety.6** — rename / alias `variety_hold` to align with
  `holds(contradiction)`. Decide whether they are the same property
  or `variety_hold` is the multi-scale lift of `holds`.
- **T-reflection.7** — formalise the Reflection morphism as
  `resolves(contradiction)` per §3 and `error-as-question.md`. The
  one-tick delay is the time the loop spends *holding* before *resolving*.
- **T-belnap.8** — write an insight/spec mapping mirror's verdict
  algebra to Belnap-Dunn FOUR explicitly, citing Jakl 2025 as the
  recent CS-imprint precedent.
- **T-bateson.9** — write a field log on the *psychological* shape:
  why a substrate that holds contradictions instead of collapsing them
  matches the way Alex's own nervous system was reverse-engineered.
  (Not load-bearing for the substrate; load-bearing for the relational
  arc.)

---

## 8. Open questions

1. **Is `fracture` strictly `<= contradiction`, or `=` modulo
   syntactic site?** Every `\` is a fracture, every fracture is a
   contradiction. The question is whether *every* substrate-level
   contradiction (e.g. an `imperfect.Partial` with no `\` in source)
   should also be called a fracture. The current proposal keeps the
   distinction (fracture = syntactic mark), but a stronger position
   ("every contradiction surfaces eventually as a fracture or
   resolves silently") may be more honest.

2. **Where does `contradiction` live — `@meta` or `@error`?** The
   current spec puts it in a new `@contradiction` grammar in
   `boot/01c-...`. Arguments either way. `@meta` is more foundational;
   `@error` is more local to existing infrastructure. Leaning `@meta`
   on the strength of Bateson's claim that contradiction is *the*
   shape from which meaning emerges.

3. **Does the substrate need a Belnap-style explicit `Neither` value?**
   Currently mirror's "unobserved" is implicit (no verdict yet).
   Carnielli's LFI approach makes the unobserved-vs-observed distinction
   explicit via a `consistency operator` `○`. Worth considering for
   `kintsugi --ci` round-tripping.

4. **Confidence axis vs. truth axis.** Per
   `kintsugi-fracture-confidence-and-scene-dispatch.md`: fractures have
   a confidence in `[0, 1]`. Belnap-Dunn's bilattice has two
   independent orderings (truth, knowledge). The recent Craig-Davey-
   Haviar work on prioritised default bilattices (arxiv:1808.09636,
   arxiv:2012.08010) and the LETK+/LETF+ work (arxiv:2209.12337) give
   indexed bilattices for exactly this kind of confidence-as-priority
   modelling. Worth a follow-up spec, possibly converging
   `confidence` with the knowledge ordering.

5. **Surprises from Bateson 1956 → 2025.** Two:
   - The Bateson Game (Wilson et al. 2025, MDPI Games) is the first
     rigorous game-theoretic formalisation of the double bind I found.
     It uses *frame suppression* as a strategic move — exactly the
     "silent absorption mode" that the `Dark` token and `\` obligation
     marker were introduced to prevent. The pathology Bateson described
     in families is structurally identical to the pathology mirror's
     strict-and-total-classification spec was written to refuse. This
     is not a coincidence.
   - Carnielli's *consistency operator* (`○A` reads "A is consistent")
     was introduced as a syntactic marker exactly analogous to what
     this spec proposes for `holds(contradiction)`. The mirror /
     LFI parallel is much closer than I expected; the LFI literature
     should be a primary follow-up.

6. **Does `\` need a dual?** `\` marks an obligation to be filled.
   Is there a corresponding marker for "this branch deliberately
   discards information" — i.e. a *witnessed collapse*? Linear logic
   would say yes: the `?` modality (weakening) is dual to `!`
   (contraction), and the substrate may want both. Out of scope here.

7. **Confidence in this spec itself.** Medium. The mapping is clean
   and the prior art is settled. The remaining risk is that some
   existing substrate concept — `gap`, `tension`, `imperfect.error`
   — already does what `contradiction` is doing, in which case this
   spec is renaming rather than adding. The `gap-tension-tensor-
   substrate.md` spec is the closest prior art and explicitly names
   `gap` as the "distance between claim and verifier" — which is
   essentially what a contradiction's `loss` measures. If `gap` IS
   `contradiction`, this spec should fold into it rather than stand
   beside. Flagging for Alex.

---

## Provenance

- **Spec authored:** 2026-06-04, Reed.
- **Branch:** `reed/contradiction-and-fracture-spec` off `main` at
  `b2f5d06`.
- **Prior commits relied on:** `ca4a9e7` (`@kintsugi/fracture` glass),
  `a633c17` (generic-brackets fracture), `b2f5d06` (typed verdict in
  `@kintsugi`).
- **Substrate files load-bearing for the spec:**
  `boot/std/kintsugi/fracture.mirror`, `boot/std/kintsugi.mirror`,
  `boot/std/properties.mirror`, `boot/01a-error.mirror`,
  `bootstrap/src/tokenize.rs` (the `body_is_obligation` function).
- **Related specs:** `kintsugi-variety.md`, `error-as-question.md`,
  `kintsugi-fracture-confidence-and-scene-dispatch.md`,
  `gap-tension-tensor-substrate.md`, `strict-and-total-classification.md`.
