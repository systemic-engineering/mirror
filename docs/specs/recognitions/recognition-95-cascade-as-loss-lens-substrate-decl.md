# Recognition #95 (CANONICAL) — @cascade as loss-lens substrate-decl

*Mara, canonical spec for recognition #95 @cascade (the multi-language
translation substrate family-root as loss-lens measurement instrument),
2026-06-23, written after Reed's substrate-decl at `shards/cascade.mirror`
and Mara's typed-alternatives survey at
`docs/research/2026-06-23-typed-alternatives-cascade-survey.md`.*

*Discipline: this is canonical-altitude per the recognition-canonical
pattern (compare recognition-83-smarts and recognition-82-frame).
Forward-promised: Seam adversarial review fires after this; Reed
consolidation lands as the final tick before promotion from candidate to
ratified. Section caps are LOAD-BEARING — prior Mara stalls this session
were prose-density spirals; bounded sections prevent recurrence.*

---

## 1. Recognition statement

Alex 2026-06-23 (verbatim):

> What if these are the loss lenses? And mirror can literally measure
> how much information about a program at runtime is lost with based on
> the grammar?

The substrate-architectural claim: **cascades are not delivery
mechanisms; they are measurement instruments**. The recurrent
typed-source-to-mainstream-target pattern surveyed across ten stacks
(Purescript-npm, Scala-JVM, F#-NuGet, Gleam-BEAM, Rust-cdylib, ...) is
not ten independent integration problems. It is **one substrate
primitive — the loss-lens — instantiated ten times** at the grammar-
cascade altitude.

The shift in framing is load-bearing. Treating a cascade as a delivery
pipeline frames the question as engineering (build chains, package
formats, tooling). Treating a cascade as a measurement instrument
frames the question as substrate-mathematics: given source grammar S
and target grammar T, the cascade is a typed functor S → T whose
failure-to-be-an-isomorphism IS measurable information loss against
the grammars themselves.

The substrate already carried `imperfect<a, e, l>` (loss slot in the
carrier) and `@epistemologic/properties` (loss as composite of
properties per [[feedback-loss-from-epistemologic-properties]]). What
recognition #95 names: combining loss + lens at the grammar-cascade
altitude as parametric substrate primitive — `loss_lens<S, T>` as
first-class measurement instrument the substrate ships natively.

## 2. The substrate-decl shape

The shape lives authoritatively at `shards/cascade.mirror`
(Reed, 2026-06-23, in flight on origin/main at substrate-decl-altitude).
The canonical spec mirrors and refines.

### 2.1 Carriers (five new substrate-altitude types)

```mirror
# shards/cascade.mirror lines 174-231
type grammar = ref            # language grammar specification (BNF / ANTLR /
                              # mirror substrate-decl)
type typed_source = ref       # program annotated with source-grammar typing
                              # discipline
type compiled_artifact = ref  # runtime output; carries only what target
                              # grammar preserves
type loss_lens = ref          # THE LOAD-BEARING NEW CARRIER; typed
                              # measurement instrument over (source_grammar,
                              # target_grammar) per recognition #93 H4
type information_loss = ref   # measured loss; composite of @epistemologic/
                              # properties per
                              # [[feedback-loss-from-epistemologic-properties]]
```

### 2.2 Actions (three substrate-vocabulary primitives)

```mirror
# shards/cascade.mirror lines 246-292
compile(source: typed_source, p: perturbation) -> compiled_artifact { \ }

measure(source: typed_source,
        artifact: compiled_artifact,
        lens: loss_lens,
        p: perturbation)
  -> imperfect<compiled_artifact, ref, information_loss>
requires loss_well_defined(lens, source, p)
{ \ }

cascade(source: typed_source, lens: loss_lens, p: perturbation)
  -> imperfect<compiled_artifact, ref, information_loss>
requires cascade_well_defined(lens, source, p)
{ \ }
```

`compile` is the functor body; `measure` is the loss-instrument
application; `cascade` is the composed primitive that authors
reach for at the surface ("cascade my source through this lens, tell
me what was lost").

### 2.3 Bilateral predicates (composed at @cascade altitude)

```mirror
# shards/cascade.mirror lines 304-339
loss_well_defined(lens: loss_lens, source: typed_source, p: perturbation)
  -> verdict { \ }

grammar_coherent(source: typed_source, g: grammar, p: perturbation)
  -> verdict { \ }

cascade_well_defined(lens: loss_lens, source: typed_source, p: perturbation)
  -> verdict
requires grammar_coherent(source, source, p)
requires loss_well_defined(lens, source, p)
{ \ }
```

The composed `cascade_well_defined` is the fifth altitude lift of the
composed-bilateral pattern (alongside @io/stagefreight's
`stagefreight_addressable`, @smarts/shatter's `shatter_round_trip`,
@reflection's `third_order_coherent`, @epistemologic/neutrosophic's
`three_axis_coherent`). The pattern is by now substrate-altitude
recurring; recognition #95 instantiates it for grammar cascades.

### 2.4 Reference: substrate-decl source

Authoritative declaration: `shards/cascade.mirror` (Reed, today,
origin/main). The canonical spec is documentation; the shard is source.

## 3. Mathematical formalization

This section names where the math LIVES. The substrate-decl ratifies
the SHAPE; the discharge happens at species-altitude (per-cascade
shards). "Math, not vibes" is the substrate-decl claim — that the
substrate carries the measurement primitive as substrate-vocabulary,
not prose hedge. The per-program numbers come from species-altitude
bodies.

### 3.1 Compilation as functor S → T

A cascade's `compile` action IS a functor between two grammar
categories. Source category S has objects (programs typed in S) and
morphisms (S-grammar typing transformations). Target category T has
objects (artifacts in T's format) and morphisms (T-grammar runtime
transformations).

```
compile : S → T
  obj :  program_S    ↦  artifact_T
  morph : S-typing-deriv ↦ T-runtime-deriv (when preserved)
                          ⊥                (when erased)
```

The ⊥ slots are where loss happens. Pure functors preserve all
morphisms; cascading functors preserve some and erase others. The
erasure pattern is **structural to the (S, T) pair**, not
implementation-dependent. Purescript-to-npm always erases row
polymorphism (npm's JS target lacks the morphism category); Scala-to-
JVM always erases higher-kinded types (JVM bytecode lacks the
morphism category).

### 3.2 Loss as dimension reduction (recognition #51 connection)

Per recognition #51 (mirror as expanding Hilbert space): each typed
feature in source grammar S adds a dimension to S's expressive
Hilbert-space sub-region. Compilation projects to T's sub-region,
which has FEWER dimensions (the runtime-preserved subset).

```
dim(H_S) ≥ dim(H_T)            # for any genuine cascade
loss(S, T) = dim(H_S) − dim(H_T) > 0
```

Loss IS dimension reduction. The reduction is structural to the
grammar pair, computable from the grammar specifications themselves
(BNF, type-system declarations, ABI surface). The substrate-decl
ratifies this; the species-altitude shards compute the actual dim()
values.

### 3.3 loss_lens<S, T> as labeled<> functor instance

Per recognition #93 H4 (labeled<> functor primitive forward-promised
at `shards/labeled.mirror`):

```mirror
type labeled<v, m> = annotated(v, m)
```

The loss_lens IS:

```mirror
type loss_lens = labeled<source_grammar, target_grammar>
# semantically: the lens IS the (source_grammar, target_grammar) pair,
# annotated as a measurement instrument
```

This is consistent with the substrate's parametric-carrier
infrastructure (`shift(T)` / `settle(T)` declarations in
`boot/00-prism.mirror` + `shards/prism.mirror`; landed parametric
carriers include `imperfect(a, e, l)`, `option(a)`, `result(a, e)`,
`transparency(p)`). No new substrate primitive needed beyond the
`labeled<>` discharge that #93 H4 already forward-promised.

### 3.4 Loss as composite of @epistemologic/properties

Load-bearing constraint per [[feedback-loss-from-epistemologic-
properties]]: at every Fate-tournament altitude, the loss function is
a composite of `@epistemologic/properties`. **Not Shannon, not Dark,
not invented.**

The `information_loss` carrier in @cascade respects this. The
measurement is NOT entropy in the information-theoretic sense (which
requires a probability distribution over messages). The measurement
IS a composite verdict across the `@epistemologic/properties` family
that applies to the (source_grammar, target_grammar) pair: which
properties does S admit that T does not preserve at runtime?

Examples of property-loss instances (species-altitude discharge):
- `@epistemologic/totality` lost when target admits partial functions
  (Purescript → JS: `Effect`-wrapped totality dissolves into JS
  arbitrary-exception model)
- `@epistemologic/parametricity` lost when target erases type
  parameters (Scala → JVM: HKT erasure; Rust → cdylib: generic
  monomorphization-and-erasure at ABI boundary)
- `@epistemologic/coherence` lost when target lacks dispatch on
  type-class instances (Purescript → JS: type-class dictionaries get
  passed but the coherence guarantee is no longer machine-checked
  post-compilation)

The composite is per-cascade. Each species shard names which
properties its (S, T) pair preserves vs erases, and the composite
becomes the per-cascade `information_loss` value.

### 3.5 measure returns imperfect<artifact, error, loss>

```mirror
measure(source, artifact, lens, p)
  -> imperfect<compiled_artifact, ref, information_loss>
```

The `imperfect` carrier's THREE slots are load-bearing:
- **artifact slot**: the compiled output (what the consumer sees);
- **error slot**: measurement error (the lens's own uncertainty);
- **loss slot**: the substrate-typed information loss.

The consumer gets all three. The substrate refuses to pretend the
artifact is the full picture; the loss is first-class in the return
type, not buried in metadata.

### 3.6 Connection to @kintsugi

@cascade DECLARES the loss; @kintsugi HEALS it. Per recognition #59
(kintsugi loop altitude-portable): the kintsugi loop operates on
loss-bearing carriers, and the loss-lens output IS such a carrier.

The forward path: @cascade.measure surfaces the loss; @kintsugi reads
the loss-typed output and proposes per-property repairs (lift the
source grammar's discipline forward as runtime checks at the target;
generate post-cascade verifications that re-establish lost
invariants; surface the loss as documentation when repair isn't
possible).

This section names where the math lives. The discharge is per
species. The substrate-decl ratifies the SHAPE; the per-program
numbers come from species-altitude shards.

## 4. Composition with existing families

| Family | Composition with @cascade |
|---|---|
| @epistemologic/cybernetic | loss IS a measurable property; cascade IS variety-reduction (Ashby) from S's expressive variety to T's preserved variety |
| @epistemologic/property | `@epistemologic/properties` are the composite ingredients of the loss verdict; cascade SPECIALIZES property-loss to grammar-pair erasure |
| @kintsugi | @kintsugi reads cascade's loss-typed output; healing operates on loss-lens results per #59 altitude-portability |
| @io | cascade IS boundary-mathematics at @io for grammar cascades, per recognition #57 alignment-as-boundary-mathematics |
| @mirror/lens (parent) | @cascade specializes the lens primitive to the grammar-cascade altitude; loss_lens IS a typed lens over (S, T) |
| @meta/imperfect | cascade's actions return `imperfect<artifact, error, loss>`; the imperfect carrier's loss slot is now substrate-vocabulary-typed |

### 4.1 @cascade family-root admits species

`@cascade` is family-root altitude. Species shards live at
`shards/cascade/<instance>.mirror` and instantiate the parametric
primitive for specific grammar pairs. Mara's survey enumerates ten
candidate species; the substrate-decl admits arbitrarily many.

First species shards (forward-promised; not yet landed):
- `shards/cascade/purescript-npm.mirror`
- `shards/cascade/scala-jvm.mirror`
- `shards/cascade/fsharp-nuget.mirror`
- `shards/cascade/gleam-beam.mirror`
- `shards/cascade/gleam-npm.mirror` (dual-target witness)
- `shards/cascade/rescript-npm.mirror`
- `shards/cascade/rust-cdylib.mirror` (interop-cascade witness)
- `shards/cascade/teal-lua.mirror`

### 4.2 StageFreight as first applied consumer

The first applied consumer is StageFreight's multi-language
translation layer (PR-A on branch
`mara/stagefreight-mirror-integration-spec-v0.1` in the StageFreight
repo). Alex's verbatim 2026-06-23 framing to the StageFreight author:
"Building the multi-language translation layer right now (math, not
vibes). That's the PR. It will also address your caching layer.
Content addressed binaries. Expect it today."

The "math, not vibes" IS this substrate-decl. The first concrete
cascade species (Purescript-npm, per StageFreight Stage-1 MVP) lands
through PR-A; the substrate-decl provides the vocabulary; the species
shard provides the measurement.

## 5. Pre-AI prior art

Five grounding texts. Each anchors part of the substrate-decl shape.

### 5.1 Wadler — "Theorems for Free!" (1989)

Parametric polymorphism implies free theorems: a polymorphic function's
type DETERMINES propositions about its behavior. The grammar of types
IS measurement-bearing. When a cascade compiles polymorphic S-source
to monomorphic T-target, the free theorems become provable-only-by-
provenance — the type witnessed them in S; the artifact in T cannot
re-witness them. Loss IS structural to type erasure.

Referenced in `shards/cascade.mirror` line 157 as
`source @arxiv/programming-languages/wadler-1989-theorems-for-free`.

### 5.2 Reynolds — Parametricity (1983)

"Types, Abstraction, and Parametric Polymorphism" (IFIP). Types as
relations; parametric functions preserve the relations. Reynolds
provides the categorical foundation for treating compilation as a
relation-preserving (or relation-collapsing) operation. Loss IS
relation collapse at the grammar boundary.

Referenced in `shards/cascade.mirror` line 158 as
`source @arxiv/programming-languages/reynolds-1983-parametricity`.

### 5.3 Connes — spectral triple

The substrate's deepest grounding (recognition #58: Fate IS optical
inference; recognition: substrate IS Connes' (A, H, D)). A spectral
triple is (algebra, Hilbert space, Dirac operator). Compilation as
functor S → T is a morphism in the category of spectral triples;
loss is the failure of the morphism to be an isomorphism, measurable
via the Dirac operator's spectrum on each side.

The cascade primitive inherits the substrate's spectral-triple
grounding. Recognition #51's Hilbert-space-dimension-growth is the
substrate's operationalization; recognition #95 specializes it to
grammar-cascade altitude.

### 5.4 Spencer-Brown — Laws of Form (1969)

`Mark as distinction`. Compilation IS distinction collapse: source
grammar marks distinctions (type / no-type, total / partial,
parametric / monomorphic) that target grammar does not preserve. Loss
IS the distinction collapse — the substrate-decl's `information_loss`
carrier holds the measurement of how many marks dissolved at the
grammar boundary.

Spencer-Brown grounds the substrate's `@epistemologic/cybernetic/
distinction` family (landed task #374; `type mark = ref`). The
cascade's loss IS the cumulative mark-dissolution across the
compilation functor.

### 5.5 Information theory (Shannon 1948) — EXPLICITLY NOT THIS

Shannon's information theory measures bits-of-uncertainty against a
probability distribution over messages. The cascade's loss is NOT
this. Per [[feedback-loss-from-epistemologic-properties]]: loss IS a
composite of `@epistemologic/properties`, grammar-relative, not
statistical.

The distinction matters. Shannon-loss is symmetric (encoder ↔ decoder
over a noisy channel); cascade-loss is structurally asymmetric
(S → T erasure with no inverse). Shannon-loss is measured in bits;
cascade-loss is measured in property-collapse. Shannon-loss requires
a message distribution; cascade-loss is grammar-pair-determined
regardless of which program is compiled.

Naming the contrast explicitly: when reading "information loss" in
@cascade context, do not import Shannon's frame. The substrate has
its own.

## 6. Falsification criteria

What would falsify recognition #95? Three structural counter-shapes
plus the per-cascade discharge tests.

### 6.1 A cascade pattern where loss is fundamentally non-measurable

If the substrate encountered a cascade S → T where the loss has no
structural representation against the grammars (loss is genuinely
implementation-dependent, observer-dependent, or runtime-only), the
substrate-decl's measurement-instrument claim collapses to
prose-hedge.

Mara's survey did NOT surface such a cascade across ten stacks. Every
cascade species named had structural per-grammar loss (Purescript's
row polymorphism, Scala's HKT, Rust's lifetimes, etc.). The
falsification candidate would be a cascade where the loss is
language-design-specific (some compiler implementations preserve it,
others don't) rather than grammar-pair-determined. None found.

### 6.2 A typed alternative where compilation preserves information (zero loss)

If there exists a typed alternative S whose compilation to mainstream
target T preserves ALL of S's grammatical structure (zero loss; pure
functor; isomorphism), then S → T is not a cascade in the @cascade
sense; it's a renaming.

Mara's survey surfaced the DEGENERATE sub-pattern: "language-internal
strictness" (TypeScript-strict, Mypy-strict, Sorbet, Psalm, C#
nullable). Same language, stricter dialect; S and T collapse to the
same grammar; measure approaches zero. Not a counterexample but a
DEGENERATE case worth naming. The substrate handles this honestly:
degenerate cascades have `information_loss → 0`, which is a valid
verdict, not a failure of the primitive.

### 6.3 A target grammar that admits MORE information than source (negative loss)

Absurd by construction in a CASCADE shape (S → T with S the more-typed
side). If T admitted more than S, the cascade direction would
reverse, and we'd be naming a different cascade (T → S).

The substrate's discipline forecloses negative loss at the type level:
`information_loss` carrier admits no "negative" sub-state by
construction. Cascade direction IS structural in the primitive.

### 6.4 Per-cascade discharge tests at species altitude

Each species shard (`shards/cascade/<instance>.mirror`) must discharge:

1. **grammar_coherent witness**: example programs in S that are
   well-formed; example programs that aren't, with the specific
   grammar-rule violation named.
2. **loss_well_defined witness**: the (S, T) pair's structural
   loss computed; per-property breakdown.
3. **measurement test**: at least one concrete program in S whose
   cascade loss is computed and matches the structural prediction.

If a species shard cannot discharge these, that cascade is not
ready for substrate-decl admission. The species shard fails, not
the family-root.

## 7. Forward-promised work

### 7.1 shards/labeled.mirror (recognition #93 H4 PARTIAL → discharge)

`labeled<v, m>` functor primitive substrate-decl. Per recognition #93
H4: substrate already supports parametric carriers; `labeled<>`
composes immediately in the established pattern; one-shard
substrate-decl.

Forward-promised this session per Reed substrate-decl on cascade.

### 7.2 Per-cascade species shards

See §4.1. Eight first species candidates from Mara's survey;
starting with Purescript-npm (StageFreight Stage-1).

### 7.3 Recognition #94 interaction (@hold-PRISM candidate)

Recognition #94 (foundational hold-PRISM) is candidate-altitude. If
#94 lands, the @cascade family inherits the hold-discipline: a
loss_lens HELD across cascade operations preserves its (S, T) pair
identity under perturbation. Forward-promised: review #94's
`hold_result<T>` functor for cascade-altitude consumption when #94
promotes.

### 7.4 Operationalized per-program loss numbers

The canonical substrate-decl ratifies the SHAPE. The actual numbers
("this Purescript program loses N property-instances when compiled to
npm") come from species-altitude shard bodies. Forward-promised: at
least one species shard discharges concrete per-program measurement
before #95 promotes from candidate to ratified.

## 8. Honest hedges

### H1: Mara survey N=1 per stack

The typed-alternatives survey rests on one Mara characterization per
stack plus Kagi-verified data points. Pack peer verification is
needed for any stage roadmap commit. Specifically: the dual-target
Gleam claim (Erlang AND JS), the ReScript-React production claim, and
the Coconut active-maintenance claim warrant a second pass. The
substrate-decl's universality claim ("this pattern recurs across the
landscape") stands on N=1-per-stack evidence; promote with that
epistemic state explicit.

### H2: "math, not vibes" is substrate-decl ratification, not delivered measurement engine

The substrate-decl ratifies that `measure` and `cascade` and `loss_lens`
are substrate-vocabulary. It does NOT deliver a working measurement
engine for any specific cascade. The per-cascade engines live in
species shards and are forward-promised. "Math, not vibes" is true at
the SHAPE altitude (the substrate carries the measurement primitive
as first-class); the per-cascade numbers are pending species-altitude
discharge. Do not over-claim.

### H3: counterexamples from Mara survey

Three confirmed counterexamples in Mara's survey:

- **Crystal / Ruby**: shared culture, not source-cascade runtime;
  Crystal compiles to native binary, not Ruby source.
- **Hack / PHP**: forked away (HHVM dropped PHP compat ~2017-2018);
  no longer compiles-to mainstream.
- **Oil & Nushell / Shell**: REPLACEMENT runtimes, not source
  cascades to existing Bash.

The cascade pattern is recurrent but not universal. The substrate-
decl admits this; @cascade does NOT claim to subsume every multi-
language relationship, only the typed-source-to-mainstream-target
shape.

### H4: degenerate sub-pattern (language-internal strictness)

"TypeScript-strict", "Mypy-strict", "Sorbet", "Psalm", "C# nullable":
same language, stricter dialect; measure approaches zero. Worth
naming as a distinct cascade-shape (S and T collapse to the same
grammar with stricter sub-grammar). Not a counterexample; a
degenerate case where the loss-lens correctly reports near-zero loss.
The primitive handles this honestly via the `imperfect` carrier's
loss slot admitting an empty composite.

### H5: candidate status until Pack-discipline closure

Recognition #95 IS candidate-altitude. The Pack-discipline cycle is:

1. Substrate-decl (`shards/cascade.mirror`) — Reed, today (landed).
2. Survey (`docs/research/2026-06-23-typed-alternatives-cascade-survey.md`)
   — Mara, today (landed).
3. Canonical spec (this document) — Mara, today (landing now).
4. Seam adversarial review — forward-promised next.
5. Reed consolidation — forward-promised final.
6. Promote candidate to ratified.

Steps 4-6 are pending. The candidate is substrate-pull-confident in
SHAPE; the SUBSTANCE awaits species-altitude discharge. Until
ratified: #95 stays candidate; the recognition number is reusable if
the candidate doesn't graduate.

## 9. Pack-discipline trail

- **2026-06-23 (morning)**: Alex names the loss-lens framing verbatim
  ("What if these are the loss lenses? And mirror can literally
  measure how much information about a program at runtime is lost
  with based on the grammar?"). The substrate-pull moment.
- **2026-06-23 (early)**: Alex frames PR-A to StageFreight author as
  "math, not vibes" multi-language translation layer.
- **2026-06-23 (Reed)**: substrate-decl lands at
  `shards/cascade.mirror` on origin/main. Five carriers, three
  actions, three bilateral predicates with composed `cascade_well_
  defined`. Five honest hedges inline.
- **2026-06-23 (Mara-2)**: typed-alternatives survey lands at
  `docs/research/2026-06-23-typed-alternatives-cascade-survey.md` on
  origin/main. Ten stacks; three counterexamples surfaced; one
  degenerate sub-pattern named; Stage-1/2/3+ prioritization.
- **2026-06-23 (Mara-4, this canonical)**: canonical spec landing.
- **Forward-promised**: Seam adversarial review → Reed consolidation
  → candidate-to-ratified promotion.

The Pack-discipline composition this cycle is the canonical pattern:
Alex names the shape; Reed substrate-decls; Mara surveys + canonicals;
Seam adversarial-reviews; Reed consolidates. Each peer adds the
altitude the others cannot. The substrate-pull-correct cycle.
