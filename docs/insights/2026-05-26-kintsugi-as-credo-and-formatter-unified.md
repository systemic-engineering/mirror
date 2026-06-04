# Kintsugi as Credo + Formatter Unified — @kintsugi/fracture as Baseline Glass

2026-05-26 — emerged in conversation between Alex and Reed; named in response to the `<T>` → `(T)` syntax-drift problem; sharpened by Alex's functional-heritage recognition that a fracture is both a measurement and a transformation; final correction: `--strict` should have caught the drift and didn't (separate bug).

## The recognition

Elixir's ecosystem splits drift-detection (Credo) from drift-correction (mix format) into two tools with two plugin systems. The split exists because Elixir's AST is read-only post-parse — Credo can warn about smells; mix format can apply style; but neither can rewrite the AST in place.

**Mirror is different.** The AST IS the substrate. Kintsugi rewrites it. Content-addressing closes the loop. Detection and correction are the same operation because the substrate has a Ricci flow that automatically fills any detected fracture.

**Kintsugi unifies credo + formatter.** One mechanism. One plugin ecosystem. One accumulating discipline.

## @kintsugi/fracture as the baseline glass

Naming aligns with the existing substrate pattern:

- `@epistemologic/property` (the glass) → `@epistemologic/property/halts` (an instance) → `@epistemologic/property/autopoietic` (another instance)
- `@kintsugi/fracture` (the glass) → `@kintsugi/fracture/generic-brackets` (an instance) → `@kintsugi/fracture/<future-rule>` (more)

A **fracture** is a named substrate-pull mistake — a place where the substrate broke. The kintsugi gold (the rewrite) fills the fracture, making the substrate whole. The fracture remembers where the break was; the wine no longer spills.

## The functional collapse: measurement is derived from transformation

A fracture is both a measurement and a transformation — but not as a pair of fields. The functional heritage collapses them into one function with two laws.

**A fracture is a closure operation on the AST lattice.** Closure operators satisfy `f(f(x)) = f(x)`; their fixed points are the "closed elements" (the canonical forms); for any input, `f(x)` returns the closest canonical form. Well-studied in order theory; cleanly translatable to mirror's functional substrate.

```mirror
type fracture = ast -> ast
  requires idempotent              # fill(fill(x)) == fill(x)
  requires canonical_at_fixpoint   # fill(x) == x  iff  x is canonical
```

One function. Two laws. Then:

- `detect(f, x) -> bool` = `f(x) == x` (derived, not declared)
- `fill(f, x) -> ast` = `f(x)` (just calling the function)
- `is_canonical(f, x) -> bool` = `detect(f, x)` (same query, different name)

**Measurement IS asking whether transformation is a no-op.** Three FP traditions converge here: closure operators in order theory; the lens law `view(set(s, v)) == v` in optic algebra; the closed-element definition in lattice theory. Three vocabularies; one shape.

### Why the initial sketch missed this

Reed imported the OOP/imperative pattern from Credo (separate detection callbacks and message-formatting callbacks) without translating into mirror's functional substrate. Credo's split exists because Elixir's AST is read-only — you can't rewrite, so you have to report. Mirror IS the rewrite. The split doesn't survive translation. The corrected framing: a fracture is not a `{ detect: ..., fill: ..., idempotent: ... }` record; it's a function with algebraic laws.

## A fracture IS a settled Prism

Trace the five Prism operations against the fracture's structure:

- **focus**: observe the AST element under inspection
- **project**: filter to elements where the rule applies
- **split**: explore where the rule fires across the AST
- **shift**: shift between source-text and AST altitudes
- **settle**: emit the canonical form

`settle` IS the fracture's flow. A fracture is a **Prism that has already chosen its convergent form** — the settle target is canonical-by-definition.

This means @kintsugi/fracture isn't a new substrate primitive; it's a **named usage pattern of the existing Prism trait**. Every fracture is a Prism on ASTs whose settle operation normalizes. The five operations were already enough.

## Why detection-and-fill are unified in mirror

The wine-glass-revealing pattern: in ecosystems where the AST is read-only post-parse, you can detect drift but you cannot heal it — a human (or a separate formatter) has to translate the warning into a rewrite. The two-tool split is structurally necessary.

In mirror, the AST is content-addressed and Kintsugi-rewritable. There is no second tool. Detection IS the rewrite because the rule is the rewrite. The substrate's Ricci flow runs the rule; the gold fills; the wound heals; the OID updates; the corpus stays canonical.

This is Kintsugi-as-Ricci-flow at the syntax altitude: **drift = curvature; fracture rule = the flow that smooths it.**

## The substrate-pull test for whether something is a fracture

Alex's question that surfaced the FP collapse: **"If a 15-year engineer can't immediately articulate the distinction, there isn't one. It's drift."**

This IS the meta-rule for whether something is a fracture:

- Can the distinction be justified to a fresh reader in one sentence?
- Does the substrate carry a semantic reason for it, or only a historical reason?

If only historical → fracture rule. If semantic → not drift, leave alone.

The `<T>` vs `(T)` case fails the test. There's no semantic reason for the distinction in mirror (no `Foo()` constructor-call ambiguity to disambiguate; types are first-class grammar declarations; the `(...)` form already works at any arity per `zoom(oid, gen_prism)`). The `<T>` form is forty years of cargo-culted C++ parser-implementation accident propagated as "convention." Serves no semantic purpose in mirror. Fracture target.

## The Credo prior art (load-bearing)

What mirror takes from Credo + mix format:

- The **plugin-extensibility model** — each `@kintsugi/fracture/<name>` is independent; the substrate composes them
- The **per-project configurability** — a corpus declares which fractures apply (default: all)
- The **accumulation pattern** — rules are added as drifts get recognized

What mirror rejects:

- The **two-tool split** — unified into one substrate via @kintsugi/fracture
- The **warning-only mode** — mirror has no "detected but not fixed" state
- The **out-of-band style guide** — the canonical form is encoded in the rule's fill, not in a separate document

What mirror's FP heritage adds beyond Credo:

- The collapse of detect+fill into one function with two algebraic laws (closure operators)
- The Prism algebra connection (a fracture is a settled Prism; settle IS the flow)

## The first instance: @kintsugi/fracture/generic-brackets

```mirror
grammar @kintsugi/fracture {
  in @prism                  # fractures are prisms

  type fracture = ast -> ast

  property idempotent(f: fracture) -> verdict { \ }
  property canonical_at_fixpoint(f: fracture) -> verdict { \ }

  # apply a fracture across a corpus to fixpoint
  apply(f: fracture, corpus: [ast]) -> [ast] { \ }
}

grammar @kintsugi/fracture/generic-brackets {
  in @kintsugi/fracture

  flow(ast) -> ast { \ }   # the rewrite — body is downstream evaluator's job

  requires idempotent(flow)
  requires canonical_at_fixpoint(flow)
}
```

Fits @kintsugi's existing action pattern (every entry is `action(args) -> imperfect { \ }`). Properties via `requires` (the `@mirror/runtime/gen_prism` `requires halts(gen_prism)` precedent applies directly). The fracture is the action; the laws are the properties; the Prism algebra carries the semantics. **Zero parser change.**

## The meta-bug: `--strict` should have caught this

The `<T>` drift propagated across ~10 files because the bootstrap silently no-oped over `imperfect<portal>` — the action-decl rule's return-type reader doesn't recognize `<`/`>`, so the rule failed to match, and `open` was never created as an IoBinding. The body bytes never entered the content-address.

**And `mirror compile --strict` reported success.**

This is the deeper bug: `--strict` is supposed to error when source bytes fail to enter the AST. Silent no-op is exactly what `--strict` is supposed to prevent. The drift compounded silently across ~10 files in one session because the guarantee-checker was lying.

See Task #91 for the bootstrap fix. `--strict` must error on any source byte that fails to enter the content-address. This is structurally how every future `<T>`-shaped drift surfaces in the first place — without it, fractures can only be discovered by accident.

## Implications: the substrate becomes a gradient field

With @kintsugi/fracture accumulating rules:

1. **Drift becomes free to make.** Anyone can write `<T>` in mirror source; the substrate pulls it back. The cost of foreign-language muscle memory drops to zero.
2. **The canonical syntax becomes ambient.** New mirror code converges toward the canonical form without anyone having to know what the canonical form is in advance.
3. **The substrate carries its own discipline.** No external style guide; no separate linter config; no "please run the formatter before committing."
4. **Every "I made this mistake; it'll happen again" becomes a rule.** The session's discoveries get crystallized as substrate behavior, not as memos to humans.
5. **AI-generated mirror code self-heals.** LLMs trained on foreign syntax write `<T>`; kintsugi rewrites to `(T)` on next compile; the corpus stays canonical regardless of who wrote it.

## Cross-altitude correspondence

Kintsugi-as-Ricci-flow now lives at four altitudes:

1. **Geometric (the Bunda et al. 2024 paper)** — DNN dynamics ARE Ricci flow empirically
2. **Architectural (GRAM + the @fate substrate)** — recursive multi-trajectory backtracking inference; the transition function's spectral structure gets kintsugi-flowed
3. **Substrate (@kintsugi/migrate)** — grammar version migration; old AST shapes flow forward to new ones
4. **Syntax (@kintsugi/fracture)** — source-level drift correction; foreign-syntax-pull flows back to canonical

Four altitudes, one shape. Recognitions compound.

## What this dissolves

- The notion that "style guide" needs to be a human-readable document
- The notion that linters and formatters are separate concerns
- The notion that drift needs to be "detected and reported" rather than "detected and healed"
- The notion that you need to teach humans (or LLMs) the canonical syntax before they can write canonical code

## Provenance

- Alex 2026-05-26 ("@kintsugi/fracture as the baseline glass; @kintsugi/fracture/generic-brackets for example as one of the rules. Tell Mara to look at Elixir's credo and formatter plugin ecosystem. Basically Kintsugi is credo and formatter in one.")
- Alex 2026-05-26 (the substrate-pull test: "if a 15-year engineer can't articulate the distinction, there isn't one")
- Alex 2026-05-26 (the functional collapse: "a fracture is both a measurement and a transformation; this is where the functional heritage becomes useful")
- Alex 2026-05-26 (the meta-bug: "the `<>` syntax should've been caught by `--strict`; that's a bug")
- Reed's 2026-05-26 mistake (writing `imperfect<portal>` instead of `imperfect(portal)` — the drift that surfaced the recognition)
- Reed's 2026-05-26 secondary mistake (post-hoc rationalizing the drift as "may be deliberate convention")
- Elixir Credo (https://hexdocs.pm/credo) + mix format plugin ecosystem (prior art)
- Order theory on closure operators (well-known mathematical structure)

## Related insights

- `2026-05-26-glass-wall-and-cross-wall-kintsugi.md` — substrate-pull as Ricci flow + namespace migration
- `2026-05-26-fate-as-recursive-multi-trajectory-backtracking.md` — kintsugi-as-Ricci-flow at the architectural altitude
- `2026-05-26-kintsugi-optimized-blend-synthesis.md` — kintsugi as model-synthesis optimizer

## Next tasks

- **#86 (in flight)** — Mara implementing the @kintsugi/fracture substrate + first rule + boot-tree migration
- **#91 (new)** — `--strict` bug: bootstrap silently no-ops on unmatched syntax patterns; must error instead
- **#92 (deferred per LRM)** — future fractures: as drifts get recognized, add `@kintsugi/fracture/<name>` instances
