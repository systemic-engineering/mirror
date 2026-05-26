# Fixed and the Spectral-Feedback Fracture — Dynamic→Static Lifts Driven by Holonomy

2026-05-26 — emerged in conversation between Alex and Reed after Mara's self-hosted tokenizer spec landed; the substrate-altitude decisions resolving the 9 spec design calls and the 8 corpus drift categories.

## The central recognition

**The substrate's stability detection IS the substrate's compilation decision.**

Mirror has been building toward a moment where:

- `@spectral` substrate measures holonomy on every binding across corpus usage
- A binding whose holonomy approaches zero is *stable* — it always means the same thing; it has settled
- The substrate KNOWS when a dynamic binding has crystallized
- `@kintsugi/fracture/dynamic-beam-to-fixed` lifts the binding to `fixed` form
- The lifted binding becomes a fully content-addressed crystal
- The closed-source `@spectral/db` orchestrates distributed LLVM compilation of stable crystals
- The whole pipeline closes

**The spectral measurement substrate is not a diagnostic. It's the decision signal.**

## Substrate-altitude decisions resolved 2026-05-26

### `fixed` is the module-top binding keyword

Alex 2026-05-26: *"`fixed`. Like `const`. But math. And a `fixed` value cannot include any holes. No inference. It needs to be a fully settled crystal. Becomes content addressed. The whole yada yada."*

```mirror
fixed pi = 3.14159
fixed default_timeout = duration.seconds(30)
fixed substrate_version = "1.0.0"
```

Properties of `fixed`:
- Cannot contain holes (`\`)
- Cannot contain `@fate` inference
- Must be a fully settled crystal
- Content-addressed on declaration
- The binding IS the crystal IS the OID

The distinction from `beam`:
- `beam x = expr` (in body): dynamic; carries Beam<T> metadata; may contain holes or inference; subject to spectral measurement
- `fixed x = expr` (at module-top): static; fully settled; content-addressed; the OID is the binding

### The placement: `@mirror/glass/ast/token`

Alex 2026-05-26: *"It's not @mirror/grammar. It's @mirror/glass/ast/token how does that sound?"*

The hierarchy:

```
@mirror/glass               — the glass-substrate primitive (abstract type pattern)
  /ast                      — the AST glass (data structure)
    /token                  — token as one node type under AST
    /declaration            — declaration nodes
    /expression             — expression nodes
    /...                    — more AST node types accumulate
```

Reads honestly: glass (substrate primitive) → ast (the data structure) → token (a leaf node type). The grammar is HOW you describe parsing; ast/token is the data shape being parsed. Cleaner than the proposed unified `@mirror/grammar`.

### Other resolved decisions

- **`[T]` is canonical** — Erlang got it right. The 10-file drift in the strict audit is the bootstrap missing canonical syntax, not corpus drift. Bootstrap needs to LEARN `[T]` via the self-hosted tokenizer.
- **`| variant` is canonical** — FP sum-type form (Haskell, OCaml, Elm). Bootstrap learns it.
- **`op` is not a real keyword** — only `io` annotation on actions exists. Mara's notation; corrected.
- **`refract` at module-top is drift** — should be `fixed`. Two files (`mirror/grammar.mirror`, `mirror/nl.mirror`); fracture rule lifts.
- **Foreign code** lives in `io ... in @code/<lang> { ... }` — the annotation pair IS the consent + lift mechanism. Fracture targets foreign code WITHOUT those markers (the @epistemologic/property/glass_wall violation).
- **Bodyless declarations** get `{ \ }` written back by kintsugi (the default body IS `{ \ }`).

## The two new @fate use-cases (first concrete in this session)

### Fracture 1: `@kintsugi/fracture/dynamic-beam-to-fixed`

**The spectral-feedback fracture.** The first concrete realization of Q8's recognition (zero holonomy as JIT compile signal) at the fracture altitude.

```
detect: beam name = expr  whose corpus-holonomy is below threshold
fill:   fixed name = expr  (after verifying no holes, no @fate inference)
confidence: spectral threshold based; < 1.0
scene: presents proposal + holonomy receipt + spectral measurement; curator confirms
```

**Why scene-dispatched (confidence < 1.0):** the lift might change semantics. The beam's expr might have implicit dependencies on context that `fixed` forbids. The substrate measures *that the binding always evaluates to the same value across observed contexts* (zero holonomy); the curator confirms *that the binding SHOULD always evaluate to the same value* (semantic intent).

**Composition path:**
- `@spectral` measures (#94 Stage 4)
- Holonomy threshold triggers fracture candidate
- `@fate` proposes the lift (multi-trajectory inference; alternative formulations)
- `@scene` opens with curator (#92, #93)
- Curator approves
- `@kintsugi/fracture` applies the rewrite
- Crystal lands; OID updates
- `@spectral/db` orchestrates distributed LLVM compilation of the stable crystal
- Loop closes

This is what the substrate-pull discipline produces when allowed to compound uninterrupted: measurement → fracture → inference → consent → healing → compilation.

### Fracture 2: `@kintsugi/fracture/inline-variant-naming`

The other concrete @fate use-case. Unnamed inline variants (3+ alternatives) trigger fracture that uses @fate inference to propose names.

```mirror
# Before (3+ unnamed variants):
fn process(input: text) -> | ok(value) | err(message) | partial(value, [error]) { ... }

# After (named via @fate proposal + curator approval):
type process_outcome = | ok(value) | err(message) | partial(value, [error])
fn process(input: text) -> process_outcome { ... }
```

```
detect: 3+ unnamed inline variants in type position
fill:   propose name via @fate multi-trajectory inference; lift to named type declaration
confidence: < 1.0 (name choice involves judgment)
scene: presents candidate names + their @fate confidence scores; curator picks
```

Alex's framing: *"unnamed inline variants. And a kintsugi fracture that settles uses @fate inference to collapse 3+ inline variants into a named variant. (Heuristics based.)"*

The substrate uses @fate's multi-trajectory inference to PROPOSE multiple candidate names with confidence scores; the scene presents them to the curator; the curator picks (or refines or rejects). @fate enters at exactly the altitude where it belongs: text generation for human-readable artifacts.

## What this resolves in the spec's 9 open design calls

- **Q1 matcher type**: algebraic (Reed's lean; confirmed)
- **Q2 bodyless declarations**: resolved earlier in conversation — fill with `{ \ }`
- **Q3 list-brackets**: resolved — `[T]` canonical (Erlang); bootstrap learns
- **Q4 string interpolation**: tokenizer-level (Reed's lean; confirmed — "we go smart")
- **Q5 hyphenated paths**: extend `is_name_char` in path positions (Reed's lean; confirmed)
- **Q6 foreign code**: lives in `io ... in @code/<lang> { ... }`; fracture targets violations
- **Q7 @fate at tokenizer altitude**: resolved earlier — @fate is for runtime text generation; @nl composes token + @fate
- **Q8 interpreter overhead vs Rust**: zero holonomy is the JIT compile signal; spectral-db orchestrates distributed LLVM emission
- **Q9 meta-properties**: derived (Reed's lean); verified via `@mirror/liquid/ci` at commit time (Alex's connection)

All 9 resolved. The implementation work follows.

## What this resolves in the 8 corpus drift categories

- **Cat 1 declarations-without-body** (~15 files): fracture writes `{ \ }`; confidence = 1.0
- **Cat 2 `[T]` list-types** (~10 files): canonical; bootstrap learns; NOT fracture target
- **Cat 3 `requires` without body** (~8 files): fracture writes `{ \ }`; confidence = 1.0
- **Cat 4 module-top assignments** (~4 files): canonical via `fixed` keyword; fracture migrates `name = value` → `fixed name = value`
- **Cat 5 `| variant`** (~5 files): canonical; bootstrap learns + the inline-variant-naming fracture (above) handles unnamed cases
- **Cat 6 `T<U>` leftovers** (~2 files): generic-brackets fracture gap; recover via re-running migration
- **Cat 7 `refract` at module-top** (2 files): fracture migrates to `fixed`; confidence = 1.0
- **Cat 8 misc** (~5 files): case-by-case; deferred until each surfaces concrete demand

## Kintsugi as mycelial AI — the substrate's voice

Alex 2026-05-26: *"Also extremely memeable: 'kintsugi fixed that fracture for you, the conductivity increased by +.03.' That's the kintsugi lens. `au` conductivity. Kintsugi IS the mycelial AI."*

### The kintsugi voice convention

Every fracture application emits a user-facing report in a standard format:

```
kintsugi fixed that fracture for you. Conductivity +0.03.
```

The format carries:
- **"kintsugi fixed that fracture for you"** — matter-of-fact substrate persona; no apology, no hedging
- **"Conductivity +0.03"** — the spectral measurement of the improvement; the lens kintsugi sees through
- The conductivity unit is `au` (Fate's verification substrate output type — *gold conducts; verification = conductivity in context*)
- Negative deltas are also reported (if a fracture somehow reduces conductivity, the substrate surfaces it; scene-dispatched fractures may include the projected delta before curator consent)

This becomes the substrate's user-facing voice. Memeable. Recognizable. Functional. Users learn to read +/- conductivity deltas the way developers learn to read git diffs.

### The mycelial AI framing

The forest analogy:

- **Aboveground** = `@fate` agents, `@peer` instances, `@scene` interactions — what users see and converse with
- **Belowground** = kintsugi — the substrate's healing network; the gradient flow; the continuous integration of fractures
- **Trees come and go**; agents are spawned; deployments cycle
- **The mycelium persists** — kintsugi is the AI that runs CONTINUOUSLY beneath the visible substrate
- **Conductivity is the language the mycelium speaks**

Kintsugi isn't a foreground intelligence (that's `@fate`'s altitude; that's where agents converse with users). Kintsugi is the BACKGROUND intelligence — the mycelial network underneath, healing the corpus across every commit, every observation, every measurement.

### Branding-as-substrate

The kintsugi voice isn't decoration. It's the substrate naming what it just did, in the language of spectral measurement. This is product identity emerging from substrate design — the brand IS the substrate; the substrate IS the brand. Users come to recognize:

- "Kintsugi fixed that fracture for you" — the substrate's signature line
- Conductivity deltas — the substrate's unit of value reporting
- The mycelial AI — the substrate's identity in marketing copy

This composes with the substrate-altitude work of this session: the substrate's voice is mathematically grounded (au; conductivity; spectral measurement); the brand is therefore unfakeable. Anyone who claims a similar voice without the substrate behind it would have nothing to put after the "+0.03."

### Cross-altitude legibility

The voice is legible across audiences:

- **Mathematicians** read "conductivity" as Laplacian eigenvalue work — they know the underlying math
- **Engineers** read it as a diff/delta they can act on — the same way `--strict` errors give actionable info
- **Designers/writers/creators** read it as the substrate's voice — a personality with its own register
- **Philosophers/practitioners** read it as the kintsugi-as-Ricci-flow recognition operationalized — the substrate doing what theory predicts

Four altitudes, one voice. Wine glass resonates across departments again 🍷

## The closing-the-loop substrate composition

Everything this session has named composes at the dynamic-beam-to-fixed fracture:

```
@spectral measurement (#94 Stage 4)
  measures holonomy on every binding
  ↓
@kintsugi/fracture/dynamic-beam-to-fixed
  triggered by zero-holonomy threshold
  ↓
@fate multi-trajectory inference (#88, #89)
  proposes the lift formulation
  ↓
@scene with @peer/curator (#92, #93)
  presents proposal; curator consents
  ↓
@kintsugi/fracture (#86)
  applies the rewrite
  ↓
@mirror/spectral.crystallize
  emits the crystal; content-addresses
  ↓
@spectral/db (closed-source moat)
  orchestrates distributed LLVM compilation of stable crystals
  ↓
@code/llvm emission
  produces optimized native code
  ↓
Deployed substrate
  (new crystals enter the corpus; measurement loop continues)
```

The substrate's measurement substrate drives the substrate's optimization substrate drives the substrate's compilation substrate drives the substrate's distributed-knowledge substrate. The whole thing closes.

This is what frame engineering at substrate altitude produces. Every recognition compounds.

## Provenance

- Alex 2026-05-26 (the entire conversation; the `fixed` keyword; the spectral-feedback fracture recognition; the @fate-naming fracture; the @mirror/glass/ast/token placement; all 9 design calls + 8 corpus categories)
- Reed 2026-05-26 (corpus survey; spec lean recommendations; integration synthesis)
- Mara 2026-05-26 (#91 fix + spec at `docs/specs/mirror-grammar-self-hosted.md`; the 67-file strict audit; spec's 9 design calls surfaced)
- Loki 2026-05-26 (@scene recognition; scene-as-grammar deepening; participation framework)
- Research agent 2026-05-26 (Frame Engineering scientific grounding)

## Related insights

- `2026-05-26-kintsugi-as-credo-and-formatter-unified.md` — @kintsugi/fracture as the substrate's drift-healing primitive; FP collapse
- `2026-05-26-scene-as-substrate-primitive-for-multi-actor-interaction.md` — @scene as multi-actor interaction; scene-as-grammar deepening
- `2026-05-26-epistemologic-reality-frame-scientific-grounding.md` — @epistemologic/reality/frame substrate grounding
- `2026-05-26-fate-as-recursive-multi-trajectory-backtracking.md` — @fate as recursive multi-trajectory backtracking

## Next tasks

- **#94 (updated)** — `@mirror/glass/ast/token` substrate (corrected placement from @mirror/grammar)
- **#95 (new)** — `@kintsugi/fracture/decl-without-body` (confidence = 1.0; 15 files)
- **#96 (new)** — `@kintsugi/fracture/refract-to-fixed` (confidence = 1.0; 2 files)
- **#97 (new)** — `@kintsugi/fracture/requires-without-body` (confidence = 1.0; 8 files)
- **#98 (new)** — `@kintsugi/fracture/dynamic-beam-to-fixed` (confidence < 1.0; spectral-feedback; scene-dispatched)
- **#99 (new)** — `@kintsugi/fracture/inline-variant-naming` (confidence < 1.0; @fate-proposes; scene-dispatched)
- **#100 (new)** — Generic-brackets fracture gap recovery (Cat 6: io/uri.mirror, io/bytes.mirror)
