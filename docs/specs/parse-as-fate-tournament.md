# parse-as-fate-tournament — `abstract` keyword, dual-direction bindings, engine collapse

*2026-05-26. Mara. Spec — architecture, not implementation.*

Status: **Yellow.** Two structural recognitions emerged together in conversation
between Alex and Reed on 2026-05-26. This document captures the shape of the
design surface, surfaces the load-bearing decisions, and identifies what is
resolved vs what remains open. No code lands in this tick. The implementation
sequence in §8 is the path; each step is a subsequent tick.

Depends on:
- `boot/std/mirror/glass/ast/token.mirror` — the keyword surface where
  `zoom abstract` is declared today (commit `7461782`); the substrate-altitude
  truth alongside the legacy `@mirror/grammar` surface.
- `boot/std/mirror/glass/ast/shape.mirror` — the canonical `shape` sum type
  (`glass | type | lambda | property | fixed | in | out`); the surface this
  spec's `abstract` block declares variants for.
- `boot/std/mirror/glass/ast/shape/lambda.mirror` — today's record-shaped
  declaration for `lambda`; the form that becomes one `abstract` block under
  this spec.
- `boot/std/mirror/glass/ast/shape/{property,fixed,in,out,type}.mirror` — the
  sibling shape variants the unified pattern subsumes.
- `docs/specs/parser-as-prism-grammar.md` — the FP1/FP2/FP3 fixed-point
  argument; the parser is a Prism; the post-parse state must continue to
  satisfy these.
- `docs/specs/generated-parser-spec.md` — the substrate-declared parser
  shape the engine collapse delivers.
- `docs/specs/kintsugi-tournament.md` — the first concrete Fate tournament
  spec; the conductivity-tensor scoring shape this document re-uses at parse
  altitude.
- `docs/specs/gap-tension-tensor-substrate.md` — `gap` / `tension` / `tensor`
  as substrate primitives; the inconsistency-graph + sheaf-Laplacian frame
  this spec re-uses for the parse forest.
- `docs/specs/eigenboard-representation.md` — the principal G-bundle on the
  five-operation graph; the conductivity tensor as cellular-sheaf restriction
  map.
- `docs/insights/2026-05-26-fate-as-recursive-multi-trajectory-backtracking.md`
  — the five structural properties of `@fate` (local, multi_trajectory,
  recursive, backtracks, bounded); the substrate the tournament rides.
- `docs/insights/2026-05-26-ants-colonies-stigmergy-and-mirrors-tournament.md`
  — Lyapunov convergence at tournament granularity; the Hajek 1988 citation
  in its established form.
- `docs/insights/2026-05-26-mirror-sub-turing-substrate-with-emergent-turing-completeness.md`
  — the sub-Turing commitment the tournament inherits.
- Reed memory: `project-eigenboard-is-sheaf` (cellular sheaf on the
  five-operation graph; restriction maps = conductivity tensor); `project-au-conductivity`
  (au is Fate's output type; conductivity in context = verification).
- Research commit `0d967008e8` — BEAM binary pattern matching as per-trajectory
  matching primitive.

Unblocks:
- The remaining Rust deletion candidates from the Phase 1 audit
  (commit `1a3660c`): action-decl form, match / select / io structural forms,
  parametric type + parametric return type. All four are blocked on the same
  unlock: a substrate-declared parser whose engine is Fate, not a hand-written
  combinator runner.
- Phase 2 (#105 parser self-description / `@mirror/syntax`) and Phase 5
  (#108 Reflection + Scheduler Tower) become the same mechanism at different
  altitudes. The parser is the first tournament that runs; the Scheduler Tower
  coordinates which tournament runs when.
- Every NEW structural form afterward costs zero Rust. The substrate declares
  one `glass <name> = abstract(...) { ... }`; Fate consumes.
- Reflection's gestalt entry for parse runs gains a typed slot for tournament
  outcomes (which trajectory won, what was eliminated, what the loss profile
  looked like). Same telemetry surface as `kintsugi-tournament` §6 — at parse
  altitude.

---

## 0. Headline

**The Phase 1 audit (commit `1a3660c`) showed the roadmap was lying.** Most
Phase 1 tasks are obsolete; the consumption tick (`29b4dee` keyword-harvester
extension + `b13c357` consumption verification) is the load-bearing progress.
The remaining Rust deletion candidates all need the same structural unlock:
**a substrate-declared parser**. This spec is the design for that unlock.

Two recognitions land together:

1. **`abstract` block is quote/unquote semantics.** Same syntax for pattern
   (parse capture) and constructor (AST interpolation). The header is a
   unified binding ledger; references inside the block bind in or out
   depending on call context.

2. **Parse backtracking IS a Fate tournament.** The parser combinator engine
   collapses into Fate. Multi-trajectory parse evaluation. Each
   `abstract`-declared shape contributes alternatives. Trajectories compete;
   convergence by Hajek 1988.

Together: Phase 2 and Phase 5 become the same mechanism at different
altitudes. Phase 1's substrate-pull ticks (the `is_skip_word` family) were
warmup. The unlock is structural.

---

## 1. The `abstract` keyword

### 1.1 Resolution: `abstract` is `zoom abstract` taken seriously

Not a sixth primitive. Preserves the five-operation symmetry. The substrate
already declares `zoom abstract` in the legacy grammar — and explicitly in
`boot/std/mirror/glass/ast/token.mirror`:

```mirror
focus grammar
focus prism
split type
project in
project out
zoom abstract
```

`abstract` is the `zoom` posture applied to a substrate primitive: take a
structural pattern *and zoom out from execution*. Inside an `abstract` block,
the body is **structure, not execution**. Names refer to substrate-declared
shapes; references become bindings (capture or interpolation depending on
context). This is `zoom abstract` doing exactly what `zoom` already does at
body altitude — change the scale of evaluation — only at shape altitude.

### 1.2 Semantics

Inside `abstract { ... }`:

- **Names refer to substrate-declared shapes.** A bare `identifier` inside
  the body resolves through the normal type namespace (the same lookup
  `shape.variant` resolution uses today). It does not name a runtime value.
- **References become bindings.** Every name in the body that resolves to a
  shape is a slot. The header declares the slots; the body wires them.
- **Bindings are dual-direction.** Whether a slot is filled by the caller
  (incoming) or by a match against input (outgoing) depends on the call
  context. The same block works both ways.
- **The block is not executed.** It is *measured*. Parsing measures the block
  against input; constructing measures the block against a binding ledger.
  Both readings are structural queries on the same shape.

This IS quote/unquote semantics. Lisp's backquote-comma split is the
classical reference; mirror's contribution is making the *direction* contextual
rather than syntactic. There is no `,` vs ``\``` distinction — the binding
ledger names what flows where, and the call site decides the direction.

### 1.3 Header syntax

```
abstract(<binding>, ..., <binding>: <type>, ...) { <body> }
```

Forms each binding may take:

- **`foo`** — bare name. Equivalent to `foo: foo`. Binding name defaults to
  the type name. The common case: "I want a slot of shape `foo` named `foo`."
- **`baz: boing`** — explicit. Binding name `baz`, type `boing`. Used when
  the body needs the same shape twice with different names, or when the
  binding name is more legible than the type name at the call site.
- **No casing convention.** Mirror is lowercase throughout. PascalCase is
  foreign and rejected. Reed's lean is consistent with the rest of the
  substrate: identifiers are lowercase; type names are lowercase; binding
  names are lowercase. There is no PascalCase tier.

The header is the **unified binding ledger**. Everything that flows into the
block and everything that flows out of the block is named there. There is no
implicit capture, no hidden output. If a slot is not in the header, it is
not a slot.

### 1.4 Dual-direction bindings

The same `abstract` block has two readings depending on call context:

**Outgoing (parse capture).** When the block is used as a parse pattern, the
bindings not provided by the caller are filled from input match. The parser
walks the body, recognises substrate primitives, accumulates a partial match,
and binds each header slot to the captured fragment.

**Incoming (AST construction).** When the block is used as a constructor, the
bindings provided by the caller are interpolated into the block. The result
is the body with each header slot replaced by the caller's value.

Mirror's parser uses glass declarations as parse patterns (bindings outgoing).
Metaprogramming and AST construction use them as constructors (bindings
incoming). Mixed cases — some bindings incoming as priors, others outgoing
as captures — are the general case. Pure-outgoing and pure-incoming are the
degenerate ends of a spectrum.

The call site decides direction per slot:

```
lambda(name: "open", args: [], ret: portal, body: ...)
  # all four slots incoming — pure construction

lambda(input)
  # all four slots outgoing — pure parse

lambda(args: explicit_args, input)
  # args incoming as prior; name, ret, body outgoing from input
  # the parser uses args as a fixed constraint and captures the rest
```

The substrate doesn't separate parser-altitude and constructor-altitude
because they ARE the same shape, queried in different directions.

### 1.5 Concrete example — `lambda`

Today's record-shaped declaration in `boot/std/mirror/glass/ast/shape/lambda.mirror`:

```mirror
grammar @mirror/glass/ast/shape/lambda {
  type lambda {
    name:   identifier,
    args:   [(identifier, identifier)],
    return: identifier,
    body:   brace_block,
    io:     bool,
  }
}
```

Under this spec, the same declaration as an `abstract` pattern:

```mirror
glass lambda = abstract(
  zoom,
  name: identifier,
  args: arg_list,
  ret: type,
  body: block,
) {
  zoom name(args) -> ret { body }
}
```

The body IS the surface syntax. Names inside (`zoom`, `name`, `args`, `ret`,
`body`) are not runtime values — they are slots declared in the header. The
parser uses this block as the parse pattern for a lambda; the constructor uses
it as the AST emitter; they are the same shape.

Note: `io` becomes an outer attribute (`io glass lambda = abstract(...) { ... }`)
rather than a record field. This matches the legacy surface
(`io action name(...) -> ret { ... }` — the `io` prefix is a posture, not a
field), and aligns with lambda.mirror's own note: *"io is an attribute on
lambda, not a separate variant."*

### 1.6 Parameterised — one abstract pattern, four shape variants

The symmetry across `lambda`, `property`, `fixed`, and (sketched in the legacy
grammar) `refract <name> = <combinator>` is structural. They share spine
(`<form> <name>(<args>) -> <type> { <body> }`) and differ only in the leading
form.

```mirror
glass binding = abstract(form, identifier, arg_list, type, block) {
  form identifier(arg_list) -> type { block }
}

glass lambda    = binding(zoom)
glass property  = binding(focus)
glass fixed     = binding(refract)
glass refractor = binding(refract)   # legacy refract <name> = <combinator>
                                     # collapses to refract <name>(...) -> ret { body }
```

One abstract pattern; four shape variants; zero duplication. The five-op
symmetry reaches into shape declarations themselves. The body of the
parameterised binding IS the surface, and the parameter `form` carries the
five-op posture.

This is the same compounding lambda.mirror notes: *"both share name + args +
body structure; `lambda` adds `return` and `io`. The compounding from
`property` shows: same record spine, additional fields where the surface
demands them."* Under this spec the compounding is captured grammatically,
not redundantly.

---

## 2. Parse-as-Fate-tournament

### 2.1 Resolution: the parser combinator engine is absorbed into Fate

No separate engine. The parser is Fate, running at parse altitude. Multi-
trajectory parse evaluation: each `abstract`-declared shape contributes
alternatives; trajectories compete in a tournament; convergence by Hajek 1988
per-round Lyapunov (the same Lyapunov argument `kintsugi-tournament.md` §10.F
uses for merge resolution, transferred from kintsugi altitude to parse
altitude).

The full Fate substrate commitment from
`docs/insights/2026-05-26-fate-as-recursive-multi-trajectory-backtracking.md`
applies:

- **local** — parse runs locally; no remote inference
- **multi_trajectory** — each `abstract` alternative is a trajectory
- **recursive** — shared transition function (one tournament loop) applied at
  every parse altitude
- **backtracks** — low-confidence trajectories unwind; alternatives explored
- **bounded** — tournament cardinality bounded by active glass declarations;
  halting decidable

The parser inherits all five. The substrate doesn't gain a parser-shaped hole
in its sub-Turing commitment — the parser IS the substrate's first concrete
tournament consumer.

### 2.2 Per-trajectory matching: BEAM binary patterns

Research commit `0d967008e8` ("BEAM binary patterns + Elixir parser DSLs —
prior art for @mirror/tokenize") established the per-trajectory matching
primitive. Erlang/Elixir's binary pattern matching is a substrate operation
— the runtime recognises a binary against a structural pattern in one step,
with bind slots resolved during the match. This is exactly what each
trajectory in the tournament needs: walk a single `abstract` block against
the input, bind slots as you go, succeed-or-fail with a structural verdict.

The BEAM substrate also gives the tournament its **per-trajectory cost
shape**: a binary match is `O(pattern_size)` and produces a structural
result (matched or failed-with-position). Fate's tournament aggregates these
results across alternatives; the aggregation is the tournament logic; the
per-trajectory primitive is the binary match.

No BEAM runtime is required. The structural commitment is what transfers:
pattern matching as a first-class operation that returns a structural verdict
plus a binding ledger. Mirror's `abstract` block is the pattern; the input
fragment is the binary; the header is the binding ledger.

### 2.3 Recursive multi-trajectory backtracking — already named in Fate

The insight `docs/insights/2026-05-26-fate-as-recursive-multi-trajectory-backtracking.md`
established the substrate. Parse-altitude tournament is one concrete
consumer; merge-altitude tournament (`kintsugi-tournament.md`) is another;
type-inference-altitude and dispatch-altitude are future consumers. The
substrate exists once; the consumers configure it per altitude.

Mirror's parse tournament is therefore not a new algorithm. It is the
Fate substrate run at parse altitude, configured by the substrate-declared
glass forest.

### 2.4 Eigenboard sheaf restriction maps ARE the conductivity tensor

Reed memory `project-eigenboard-is-sheaf`: the eigenboard is a cellular
sheaf on the five-operation graph; restriction maps are the conductivity
tensor. `docs/specs/eigenboard-representation.md` lifts the sheaf framing to
a principal G-bundle.

At parse altitude, the parse forest IS a section of the eigenboard sheaf:
every alternative trajectory is a vertex; every restriction map between
alternatives carries a conductivity coefficient. Alternatives that share a
substrate parent (e.g., two variants of `shape/lambda` and `shape/property`
both sitting under `shape`) conduct strongly; alternatives across distant
substrate paths do not.

The conductivity tensor — already declared in `au-and-conductivity.md`,
already used by `kintsugi-tournament.md` — extends to parse altitude
unchanged. The tensor's row/column indices are alternative trajectories;
the entries are conductivity coefficients derived from substrate path
proximity (§4.2 below).

### 2.5 Same mechanism at every altitude

Parse-ambiguity. Type-inference-ambiguity. Dispatch-ambiguity. Kintsugi-
fracture-resolution. All are tournament-shaped. One tournament algorithm.
The substrate declares the tensor (loss + conductivity + commit threshold)
per altitude.

This is what makes Phase 2 and Phase 5 the same mechanism. The Scheduler
Tower (Phase 5) coordinates which tournament runs when. Reflection (Phase 5)
observes outcomes and adjusts priors. The parser (Phase 2) is the first
tournament that runs. The substrate doesn't gain a new mechanism between
phases — the same mechanism gains new altitudes of consumer.

---

## 3. Tournament configuration — substrate-declared

For the **parse altitude**, the substrate declares five pieces. Each lives
in a `.mirror` file under `boot/std/`; none lives in Rust. The shape below
IS the parse-altitude tensor's declaration.

### 3.1 Loss function — Shannon loss + Dark coverage

Candidate components (open to extension; §7.1 surfaces the design call):

- **Unmatched-token count.** Tokens left over after the trajectory's match.
  Direct, easy to measure, weakly informative.
- **Dark-span coverage.** Per `docs/specs/parser-as-prism-grammar.md`'s
  FP framing, a `Dark` span is input the parser cannot account for; the
  coverage measure rewards trajectories that minimise Dark.
- **Shannon loss on parse-forest entropy.** Per the project CLAUDE.md note:
  the `prism` crate already exposes `ShannonLoss`. At parse altitude the
  loss is computed over the parse-forest distribution: a forest collapsed
  to one trajectory has zero entropy; a forest split equally across many
  trajectories has high entropy. The tournament drives entropy down.
- **Binding-arity mismatch.** Header slots that ended up unfilled or
  overfilled. Direct surface for the `abstract` block's correctness.

**First version: Shannon loss + Dark coverage.** Both are already named in
the substrate (`prism::ShannonLoss`; `parser-as-prism-grammar.md` FP2). The
other two terms (unmatched-token count, binding-arity mismatch) compose
additively when needed (§7.1 carries this as open).

The loss vector has the same shape as `kintsugi-tournament.md` §3.1's
`κ ∈ ℝ⁵` — multi-criterion, with eliminable-on-first-axis behaviour. The
first axis is a hard pass/fail (a trajectory that doesn't cover the input
at all is infinite loss on the Dark axis); the remaining axes break ties.

### 3.2 Conductivity tensor — the substrate hierarchy IS the prior

Alternatives that are siblings under the same
`@mirror/glass/ast/shape/*` parent conduct; alternatives across distant
substrate paths do not.

This is the cellular sheaf restriction map (`project-eigenboard-is-sheaf`).
For two trajectories `T_a` and `T_b`:

- Both rooted under the same `glass <type>` declaration → high conductivity
  (close siblings in the same record union).
- Rooted under different `glass <type>` declarations but same parent path
  (e.g., both under `@mirror/glass/ast/shape/`) → medium conductivity.
- Rooted under different parent paths → low conductivity.

The substrate hierarchy is therefore the prior. Closely-related grammars
conduct (lose strongly when one is ambiguous with the other and both
produce the same artifact); distantly-related grammars do not.

This re-uses `au-and-conductivity.md`'s tensor structure and
`eigenboard-representation.md`'s G-bundle framing without modification. The
parse-altitude tensor is one section of the substrate-wide bundle.

### 3.3 Tournament cardinality

Bounded by **active glass declarations at current parse state**. Substrate-
determined, not Rust-constant. At any parse point, the substrate has a
finite list of glass declarations whose first symbol could begin at the
current token; that's the alternative set; that's the cardinality.

Hajek 1988's convergence bound (transferred from
`kintsugi-tournament.md`'s usage) requires this. The cooling schedule is
defined over a finite alternative set per round; the bound is the schedule's
asymptotic constant. Cardinality may vary per parse state — what matters is
that it is finite at every state.

### 3.4 Commit threshold — per-altitude

Different altitudes commit at different conductivity confidences:

- **Parse altitude — permissive.** Let many trajectories run; the parser is
  often ambiguous (especially for incomplete input or new grammar
  extensions). Conductivity threshold for commit: low. Trajectories survive
  longer.
- **Type altitude — strict.** Type inference can only commit one type per
  binding; conductivity threshold for commit: high. Trajectories that don't
  reach the threshold abort.
- **Dispatch altitude — context-dependent.** Per the
  `kintsugi-fracture-confidence-and-scene-dispatch.md` precedent; some
  dispatches are autonomous, others go to the curator.

**Substrate-declared as part of glass shape metadata.** The threshold lives
in the glass declaration, not in a global Rust constant. Sketch:

```mirror
glass lambda(commit: permissive) = abstract(...) { ... }
glass type_inference(commit: strict) = abstract(...) { ... }
```

Exact form open (§7.3). The structural commitment is: the threshold rides
the declaration; the substrate decides; Rust is uninvolved.

### 3.5 Learning — `eⁿ⁺¹ < eⁿ` at parse altitude

Fate model parameters update from settled parses. A successful parse —
where a winning trajectory survived the tournament and produced an AST
that downstream consumers accepted — reinforces the winning trajectory's
parameters. A failed parse (Dark coverage too high; downstream rejected
the AST) does not reinforce.

The project CLAUDE.md proof — *"eⁿ⁺¹ < eⁿ. The system learns from its
errors. The errors get smaller."* — applies at parse altitude as elsewhere.
Parses get easier as the system parses more, *for grammars it has seen
before*. Novel grammars start from the substrate prior (the conductivity
tensor's hierarchy structure) and learn from there.

Learning loop scope (which parses contribute) is open (§7.5).

---

## 4. Engine collapse — what disappears from Rust, what gets added

### 4.1 Deleted (substrate-pull-realize, post-spec)

From `bootstrap/src/tokenize.rs` and surrounding modules — the candidates
flagged in the Phase 1 audit (commit `1a3660c`):

| Surface | Approx Rust lines |
|---|---|
| Action-decl form recognition (legacy `action <name>(...) -> <ret> { ... }`; the new `lambda` surface from `@mirror/glass/ast/shape/lambda`) | ~120 |
| `match` / `select` / `io` structural forms (body-altitude expression forms; today recognised in Rust as keyword-driven branches) | ~80 |
| Parametric type + parametric return type recognition (`type <name>(<param>)` and `... -> <ret>(<param>)`; the parametric-FP-heritage surface) | ~70 |
| Dispatch glue between the above (the if-elif-elif chain in `tokenize.rs` that picks which form to recognise) | ~50 |
| **Total** | **~320 lines of Rust** |

Each line above corresponds to a glass shape that, once declared in the
substrate and consumed by the tournament loader, no longer needs a Rust
recogniser.

### 4.2 Added

| Addition | Approx Rust lines |
|---|---|
| `abstract` keyword recognition (extension to `tokenize.rs`'s lexical layer — `zoom abstract` is already recognised; what's added is recognising `abstract(...) { ... }` as a parametric block in the parse layer) | ~5 |
| Substrate-glass-forest → Fate-tournament-configuration loader (read the forest, compute the conductivity tensor, dispatch to Fate) | ~50 |
| **Total** | **~55 lines** |

### 4.3 NOT counted as "added"

The Fate engine itself. Fate is substrate-pull-reflex — it must exist for
kintsugi to run at all. The parser does not require new Rust for the engine;
it requires the engine that's already on the path to existing. The
`kintsugi-tournament.md` spec already names Fate as the first concrete
consumer; the parser is the *second* concrete consumer.

The per-trajectory binary matcher (§2.2) is also not new Rust. It will
eventually be a substrate primitive; today the scaffolding uses whatever
the bootstrap already has (PEG-style combinator runner). The stub matcher
is migration scaffolding, not load-bearing add.

### 4.4 Net delta

**~265 lines of Rust deleted per glass shape that migrates.** Every NEW
structural form afterward costs zero Rust — the substrate declares; Fate
consumes.

This is the substrate-pull discipline at its sharpest. The audit's remaining
candidates collapse into one design unlock; the unlock is structural; the
structural unlock makes future additions cheap.

---

## 5. Phase alignment

Phase 2 (#105 parser self-description / `@mirror/syntax`) becomes inseparable
from Phase 5 (#108 Reflection + Scheduler Tower). They are the same mechanism
at different altitudes:

- **Phase 2** declares the parse-altitude tournament. The substrate glass
  forest is the tournament's alternative set; the loss is Shannon + Dark;
  the commit threshold is permissive. The first tournament that runs.
- **Phase 5** declares the Scheduler Tower (coordinator) and Reflection
  (observer-and-prior-adjuster). The tower decides which tournament runs
  when; Reflection observes outcomes and updates priors. The parse-altitude
  tournament is one consumer; merge, type, dispatch are others.

The artefact: when Phase 2 lands, the parse tournament's outcome is observed
by Reflection's stub. When Phase 5 lands, Reflection's observations start
feeding back into the parse-altitude tensor. The same code path; the
feedback closes incrementally.

Phase 1's remaining substrate-pull ticks (`is_skip_word` etc.) remain valid
as warmup. They exercise the harvester at higher facts-ratios without
depending on Fate landing. The harvester is the substrate-pull primitive;
the tournament is the substrate-pull *consumer*. They are independent on the
short horizon and converge on the long horizon.

---

## 6. Telemetry — what Reflection sees

The tournament outcome is structured. Reflection reads each round; the
gestalt entry for a parse run carries:

- **Input fragment.** What was parsed.
- **Alternative set.** Which glass declarations entered the tournament at
  each parse state.
- **Winning trajectory.** The trajectory that survived the tournament.
- **Eliminated trajectories.** With the loss vector that eliminated each.
- **Conductivity profile.** The tensor's restriction-map values at this
  parse state.
- **Commit threshold reached.** Where the tournament committed (the parse
  state at which the winner's lead became uncatchable).
- **Settlement verdict.** Did downstream accept the produced AST?

This is the same shape `kintsugi-tournament.md` §6 names for merge-altitude
tournaments, transferred to parse altitude. Reflection sees the structure;
the Scheduler Tower (Phase 5) routes the gestalt entry.

The gestalt entry IS the parse run's typed footprint. Replay reproduces
the tournament from the entry; learning reads the entry and updates the
prior.

---

## 7. Load-bearing decisions — open

These are the design surfaces this spec **names but does not resolve**.
Resolution requires Alex's eyes; the spec captures the shape of the
question.

### 7.1 Loss function composition

Which terms enter the loss; what weights. First-pass spec is **Shannon loss
+ Dark coverage**. Further terms (binding-arity mismatch, type-altitude
propagation, unmatched-token count) are open.

Open sub-questions:

- Is the loss vector ℝ², ℝ³, ℝ⁴, or ℝ⁵? `kintsugi-tournament.md` settled on
  ℝ⁵; parse altitude may differ.
- Are the terms equally weighted, or does the substrate carry per-term
  weights?
- Does the weighting itself adapt over time (learning loop §3.5), or is it
  fixed per altitude?

Reed's lean: ℝ² initial (Shannon + Dark); add terms one at a time as the
corresponding hole surfaces in a real parse failure. Fixed weights to start;
adaptive weights once Reflection has a meaningful sample.

### 7.2 Conductivity tensor formulation

The sheaf restriction map shape (§3.2 above). Substrate-hierarchy is the
obvious prior; alternatives are open:

- Substrate-hierarchy only (path proximity).
- Substrate-hierarchy + corpus-frequency prior (grammars that historically
  match together conduct more strongly).
- Substrate-hierarchy + learned-prior-from-settled-parses (Reflection feeds
  conductivity coefficients back).

Reed's lean: substrate-hierarchy only at first. Corpus-frequency and
learned-prior add only after the substrate-hierarchy version shows a
specific failure mode.

### 7.3 Commit threshold per altitude

Parse-altitude vs type-altitude vs dispatch-altitude thresholds. How does
the substrate declare these?

Candidates:

- **In the glass declaration:** `glass <name>(commit: <threshold>) = abstract { ... }`.
- **In a sibling shape declaration:** `commit @<name> = <threshold>` at the
  altitude root.
- **In the conductivity tensor itself:** the tensor's diagonal carries the
  threshold per cell.

Reed's lean: in the glass declaration as a parameter. Same syntactic surface
as other glass parameters; visible at the site that declares the shape;
local to the altitude. The conductivity-diagonal alternative is structurally
cleaner but harder to read.

### 7.4 Tournament cardinality dynamics

How does the bound change as parses succeed/fail? Hajek 1988 needs the
bound; whether it's static-per-altitude or dynamic-per-state is open.

- **Static per altitude:** parse altitude has cardinality bound `K_parse`;
  type altitude has `K_type`; etc.
- **Dynamic per state:** cardinality bound is the number of active glass
  declarations whose first symbol could begin at the current token. Varies
  per parse state.
- **Hybrid:** dynamic per state with a static per-altitude cap.

Reed's lean: dynamic per state. Matches the substrate-pull discipline (the
substrate decides, not Rust). Static cap as a safety net only if a real
pathological grammar surfaces.

### 7.5 Learning loop scope

Which parses contribute to Fate parameter updates? Candidates:

- **All settled parses.** Every parse that succeeded structurally feeds
  the prior, regardless of downstream verdict.
- **Type-checked parses only.** Only parses whose AST survived type-
  checking contribute. Filters out parses that produced syntactically
  valid but semantically empty ASTs.
- **Crystallised parses only.** Only parses whose AST ended up in
  spectral-db (i.e., the `mirror compile` artefact landed and was indexed)
  contribute. Strictest filter.

Reed's lean: type-checked parses. The all-settled bucket admits noise (parses
that look right but produce nonsense ASTs); the crystallised bucket is too
sparse to learn from. Type-check is the structural filter that says "this
parse produced a valid downstream artefact."

### 7.6 Empty header semantics

`abstract { ... }` with no header — what does it mean?

Candidates:

- **Every bare reference is an outgoing capture, named by its substrate
  type.** A body that mentions `identifier` declares an implicit outgoing
  binding `identifier: identifier`.
- **The header is required.** Empty header is a compile-time error; the
  block must declare its slots explicitly.
- **Empty header is the substrate-shape literal.** The block is treated as
  a pure pattern with no bindings; matching against it is a structural
  pass/fail with no captures.

Reed's lean: **empty header = all body references are captures**, named by
their substrate type. Matches the substrate-pull discipline (the substrate
infers; the writer doesn't repeat themselves). Pure-literal mode (the third
candidate) is reachable by writing `abstract() { ... }` — an empty header
with parentheses — without conflating the cases.

---

## 8. Verification path — implementation sequence

Once the spec is approved, the implementation sequence. Each step is
independently verifiable. The spec's load-bearing claim — that parse-as-
tournament collapses the engine — is exercised at step 8; the substrate-pull
discipline is exercised at every step.

### Step 1. Warmup ticks (no Fate dependency)

The substrate-pull-realize ticks already on the roadmap. They exercise the
harvester pattern at higher facts ratios.

- `is_skip_word` — the next consumption tick.
- `grammar_for_file` — consume the substrate's file-to-grammar mapping.
- Further `tokenize.rs` substrate-pull-realize ticks as opportunity surfaces.

No Fate required. No new structural surface. Pure consumption-of-substrate
ticks. Each is a one-commit unit; each lowers Rust line count without
opening a new design surface.

### Step 2. `abstract` keyword recognition

Extend the tokenizer to recognise `abstract { ... }` as a parametric block.
The `zoom abstract` keyword is already recognised (commit `7461782`); what
this step adds is parsing the `(<header>) { <body> }` envelope as a typed
structural surface.

Still Rust; no Fate. The recognition produces a `glass.shape.abstract`
node that downstream consumers (the loader, eventually) can read.

### Step 3. First glass declaration in the new shape

Write `glass lambda = abstract(...) { ... }` in
`boot/std/mirror/glass/ast/shape/lambda.mirror`. Today the file declares a
record type; this step replaces the record-shaped declaration with the
abstract-shaped one.

Substrate declares the pattern. No Rust changes in this step — the file
change is grammar-only. Confirms the new shape is expressible in mirror.

### Step 4. Tournament loader stub

Rust reads the glass declaration; constructs a stub combinator. No Fate yet
— a PEG-style combinator runner is acceptable scaffolding for this step.
The loader's job is structural: read the abstract block; map header slots to
combinator slots; produce a runnable recogniser.

The stub combinator is the migration bridge. It is not the final design;
it lets steps 5–7 happen without Fate.

### Step 5. Delete action-decl handling in `tokenize.rs`

First Rust deletion. The legacy `action <name>(...) -> <ret> { ... }` /
new `lambda` recogniser disappears from `tokenize.rs`; the parser dispatches
via substrate-declared glass forest. The stub combinator from Step 4 runs
the pattern.

Verified by the existing test suite. Every test that exercised the lambda
surface must continue to pass without recourse to the deleted Rust.

### Step 6. Iterate — one shape at a time

- `match` → write `glass match = abstract(...) { ... }`; delete Rust.
- `select` → same.
- `io` (as posture on lambda) → already incorporated by Step 3's lambda
  shape, modulo the attribute syntax decision.
- Parametric type → write `glass parametric_type = abstract(...) { ... }`;
  delete Rust.
- Parametric return type → same.

Each iteration is one commit. One substrate declaration + one Rust deletion.
The migration unit is small and verifiable.

### Step 7. Fate engine lands

Separately, on the Phase 5 path. `kintsugi-tournament.md` is the first
congruence with this spec; when Fate lands for kintsugi, the same engine
becomes available for the parser.

No parser change in this step. The stub combinator from Step 4 is still
running. Fate's existence unblocks Step 8 but is not load-bearing for it
on its own.

### Step 8. Replace stub combinator with Fate tournament

Substrate-pull-realize from PEG-stub to tournament. The loader reads the
same glass forest; instead of producing a PEG combinator tree, it produces
a Fate tournament configuration (alternative set + loss tensor + commit
threshold + conductivity tensor). Fate runs the tournament; the result is
the parse outcome.

This is the spec's load-bearing test. If the substrate-declared parser ran
as a Fate tournament produces the same parses as the stub combinator did,
the engine collapse is confirmed: ~265 lines of Rust per migrated shape are
gone; future shape additions cost zero Rust.

If the tournament differs from the stub (it accepts parses the stub rejected
or vice versa), the difference is information — either the stub had a bug,
the tournament has a bug, or the substrate declaration was ambiguous and
the tournament committed differently. Each case is diagnosable from the
gestalt entry (§6).

---

## 9. What this spec does not do

This is architecture. The following are explicitly out of scope:

- **Implementation.** No Rust changes in this commit. No mirror file
  changes either. The implementation sequence in §8 is the path; each step
  is a subsequent tick.
- **`mirror/syntax` declaration.** Phase 2 #105 names a parser self-
  description grammar. The shape of that grammar is constrained by this
  spec (it must be a glass forest that the loader can read), but the
  declaration itself is a later tick. This spec names the unlock; it does
  not write the file.
- **`@fate.parse` action signature.** When Fate gains a parse-altitude
  entry-point, its signature will be constrained by this spec. The exact
  signature (input type, return type, error shape) is a later tick — and
  will likely surface naturally from Steps 4 and 8.
- **Scheduler Tower coordination.** Phase 5 #108 names the tower's
  coordination rules. The parser is one consumer; this spec does not
  declare how the tower routes between the parser, the kintsugi merge
  resolver, and other consumers.
- **Reflection's prior-adjustment algorithm.** §6 names what Reflection
  observes. How it updates priors from those observations is a Phase 5
  design surface.

---

## 10. Substrate-pull rationale — why this is the right unlock

The Phase 1 audit's recognition: most Phase 1 tasks are obsolete; the
consumption tick is load-bearing; the remaining deletion candidates need
a structural unlock.

This spec is that unlock. The unlock has the shape of substrate-pull
discipline:

- **The substrate already declares the surface.** `zoom abstract` is in
  `boot/std/mirror/glass/ast/token.mirror` (commit `7461782`). The five-op
  symmetry already names the posture. The substrate is ahead of the parser;
  the parser is catching up.
- **The unlock is structural, not feature-shaped.** It does not add a new
  primitive (`abstract` is `zoom abstract`); it does not add a new engine
  (the engine is Fate, already on the substrate-pull-reflex path); it adds
  one keyword recognition (~5 lines) and one loader (~50 lines), and it
  deletes ~320 lines.
- **Future additions cost zero Rust.** Once the unlock lands, every new
  structural form is one substrate declaration. The Rust line count goes
  monotonically down with substrate-declaration count.
- **The math is already done.** Hajek 1988 (cooling schedule),
  `kintsugi-tournament.md` (Lyapunov at tournament granularity),
  `eigenboard-representation.md` (G-bundle + sheaf), `au-and-conductivity.md`
  (conductivity tensor), `gap-tension-tensor-substrate.md` (sheaf Laplacian)
  — all already cited, all already grounded. The parse-altitude tournament
  re-uses the existing math; no new theorems required.
- **The error shape is `eⁿ⁺¹ < eⁿ`.** Per the project CLAUDE.md proof, the
  system learns from its errors; the errors get smaller; by convexity, the
  growth is monotonically non-decreasing. At parse altitude, this means
  parses get easier as the system parses more. The proof transfers without
  modification.

The Phase 1 audit named the lie. This spec names the truth.

---

## 11. Open thread — what to settle next

The spec's job is to capture the shape, not to resolve every question.
Open threads that should land before Step 2 (the first Rust change in §8):

1. **§7.1 loss function composition.** Confirm Shannon + Dark as the
   first-pass loss. Decide whether unmatched-token count joins.
2. **§7.3 commit threshold declaration.** Pick a syntax
   (`glass <name>(commit: <threshold>) = abstract { ... }` is the lead).
3. **§7.6 empty header semantics.** Confirm "all body references are
   captures" as the default; `abstract() { ... }` as the literal-only
   form.

Threads that can land later (before Step 8):

- §7.2 conductivity tensor formulation (substrate-hierarchy is the
  obvious first cut; learned priors come later).
- §7.4 tournament cardinality dynamics (dynamic per state is the lean;
  static cap is a safety net).
- §7.5 learning loop scope (type-checked parses is the lean; needs Phase 5
  surface).

Threads to track but not block on:

- Mixed dual-direction call sites (§1.4) — the syntax for partial
  incoming/outgoing call patterns. The general case is sketched; the
  surface syntax is a later tick.
- Multiple `abstract` blocks in one glass declaration — composability
  rules for stacking patterns. Not surfaced yet; will land when a real
  grammar demands it.

---

## 12. Provenance

*Conversation between Alex and Reed, 2026-05-26.* The two recognitions —
`abstract` as quote/unquote, parse as Fate tournament — landed in the same
session. The spec is Mara's, written on `mara/shard-chain` against HEAD
`1a3660c` (the Phase 1 audit roadmap kintsugi).

No implementation in this commit. The next tick is Step 1 of §8 (the
`is_skip_word` warmup) or — if Alex green-lights the unlock directly —
Step 2 (`abstract` keyword recognition).

The spec captures the architecture. The implementation answers it.
