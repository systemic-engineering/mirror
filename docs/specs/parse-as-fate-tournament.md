# parse-as-fate-tournament — `abstract` keyword, `\`-marker holes, engine collapse

*2026-05-26. Mara. Spec — architecture, not implementation.*

Status: **Yellow-going-Green.** Two structural recognitions emerged together
in conversation between Alex and Reed on 2026-05-26; five Alex corrections
followed on the same day (loss-from-`@epistemologic/properties`, v0/v1/v2
conductivity path, sub-Turing closes cardinality, no commit threshold,
`\`-marker hole syntax). After the corrections, the open list collapses to
one thread (learning loop scope) plus three named edge cases. No code lands
in this tick. The implementation sequence in §8 is the path; each step is a
subsequent tick.

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
   (parse capture) and constructor (AST interpolation). Three-way syntax:
   header inputs (`name: type`), body holes (`\name: type`), and body
   literals (bare names). The `\` marker IS the typed hole — same
   primitive as `\` in `boot/std/mirror/store/nix.mirror`: `\` = unfilled.

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

Inside `abstract(<inputs>) { ... }`:

- **Inputs flow in from the call site.** The header declares them by name
  and type; the caller supplies them at the specialisation site. Inputs
  resolve like any other identifier in scope.
- **Body holes are marked with `\`.** A `\name:type` in the body is a
  typed hole — a slot the consumer fills. The parser captures it from
  input; the constructor fills it from a caller-provided value. The
  position decides direction; the syntax is uniform.
- **Bare names in the body are literals.** A bare `zoom`, `(`, `->`, etc.
  inside the body appears in the AST as itself. Literals do not bind.
- **The block is not executed.** It is *measured*. Parsing measures the
  block against input; constructing measures the block against a
  hole-fill ledger. Both readings are structural queries on the same shape.

This IS quote/unquote semantics. Lisp's backquote-comma split is the
classical reference; mirror's contribution is to make hole-and-literal a
first-class three-way syntactic distinction (input / hole / literal) rather
than an implicit ledger. The `\` mark on a hole IS the substrate's
parked-action marker — same primitive, both contexts.

### 1.3 Header syntax — three-way bindings

```
abstract(<input>: <type>, ...) {
  <body referencing inputs, holes \name:type, and literals>
}
```

Three syntactic forms inside an `abstract` block:

| Form | Position | Meaning |
|------|----------|---------|
| `name: type` | header | **Input** — caller fills from scope at the call site |
| `\name: type` | body | **Hole** — typed-hole; consumer captures (parse) or fills (construct) |
| `name` (bare) | body | **Literal** — appears in the AST as itself |

The `\` is the typed-hole marker. It already lives in the substrate as the
parked-action marker — `boot/std/mirror/store/nix.mirror` declares `\` as
the body of e.g. `path(oid) -> text { \ }`, `adopt(path) -> oid { \ }`,
`tombstone_check(oid) -> option(tombstone) { \ }`, and others. Same
primitive: `\` = unfilled. Reusing it across both contexts (parked action
body, typed hole inside a shape) is consistent.

**Naming.** Mirror is lowercase throughout. No PascalCase. Identifier names,
type names, binding names — all lowercase.

The header is the **input ledger**. Everything the caller is expected to
supply is named there. Holes in the body are slots the *consumer* fills:
the parser captures them from input; the constructor fills them from
caller-provided continuations. There is no implicit capture and no hidden
output — every slot is either an input in the header or a `\`-marked hole
in the body.

### 1.4 Direction

The same `abstract` block has two readings:

- **Parse capture.** Holes are outgoing; the parser walks the body, matches
  literals positionally, and binds each `\name:type` to the matched
  fragment.
- **AST construction.** Holes are incoming; the caller fills each hole; the
  result is the body with each `\name:type` substituted.

The block doesn't change. The consumer changes. Parser-altitude and
constructor-altitude are the same shape, queried in different directions.

Inputs are direction-invariant — they always come from the call site
(§1.6's parameterised binding uses this for the form keyword). Literals are
also direction-invariant — they always appear in the AST as themselves.

**Empty `abstract { ... }`** — no header, no `\` marks — is an empty quote:
the AST literally. §7.5 records this as resolved.

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
glass lambda = abstract {
  zoom \name:identifier(\args:arg_list) -> \ret:type { \body:block }
}
```

The body IS the surface syntax. The bare `zoom` is a literal — the `zoom`
keyword as it appears in source. The `\name`, `\args`, `\ret`, `\body`
are typed holes — the parser captures them from input; the constructor
fills them from caller-provided values. The punctuation (`(`, `)`, `->`,
`{`, `}`) is literal.

Note: `io` becomes an outer attribute (`io glass lambda = abstract { ... }`)
rather than a record field. This matches the legacy surface
(`io action name(...) -> ret { ... }` — the `io` prefix is a posture, not a
field), and aligns with lambda.mirror's own note: *"io is an attribute on
lambda, not a separate variant."*

### 1.6 Parameterised — one abstract pattern, three shape variants

The symmetry across `lambda`, `property`, `fixed` (and the legacy
`refract <name> = <combinator>` line, which collapses to the same spine)
is structural. They share spine
(`<form> <name>(<args>) -> <type> { <body> }`) and differ only in the
leading form. The form keyword is an **input** to the abstract — supplied
at the specialisation site — while the spine itself is hole-and-literal:

```mirror
glass binding = abstract(form: op) {
  form \name:identifier(\args:arg_list) -> \ret:type { \body:block }
}

glass lambda   = binding(zoom)
glass property = binding(focus)
glass fixed    = binding(refract)
```

One abstract pattern; three shape variants from a single header input;
zero duplication. The five-op symmetry reaches into shape declarations
themselves. The body of the parameterised binding IS the surface; the
input `form` carries the five-op posture into the literal-position of
the spine.

The legacy `refract <name> = <combinator>` line collapses to
`refract <name>(...) -> ret { body }` — the same spine — and rides
`binding(refract)`.

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
plus a hole-fill ledger. Mirror's `abstract` block is the pattern; the input
fragment is the binary; the `\`-marked holes in the body are the ledger
slots.

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
The substrate declares the tensor (loss + conductivity) per altitude;
convergence is the commit.

This is what makes Phase 2 and Phase 5 the same mechanism. The Scheduler
Tower (Phase 5) coordinates which tournament runs when. Reflection (Phase 5)
observes outcomes and adjusts priors. The parser (Phase 2) is the first
tournament that runs. The substrate doesn't gain a new mechanism between
phases — the same mechanism gains new altitudes of consumer.

---

## 3. Tournament configuration — substrate-declared

For the **parse altitude**, the substrate declares two pieces — loss and
conductivity. Each lives in a `.mirror` file under `boot/std/`; none lives
in Rust. Cardinality is fixed by sub-Turing (§3.3); the stopping rule is
Hajek convergence (§3.5); learning closes the loop (§3.4).

### 3.1 Loss function — composite of `@epistemologic/properties`

**The loss function at any altitude is a composite of the
`@epistemologic/properties` declared at that altitude.** Not Shannon loss
plus Dark coverage. Not a hand-invented metric. The substrate-pull discipline
says facts the substrate can carry belong in the substrate; the loss
function IS a substrate fact; the properties at the relevant altitude
declare it.

Mechanism. `@epistemologic/property` (declared in
`boot/std/epistemologic/property.mirror`) gives the type:

```mirror
type verdict = pass | fail(diagnostic) | partial(f64, [diagnostic])
type check   = (ast) -> verdict
reflect(ast) -> [verdict] { \ }
```

Every concrete property check is a `check`. `reflect(ast)` returns the
verdict vector for every property inherited via `in` at the current
altitude. The tournament's loss for a trajectory is the composite of those
verdicts:

- `pass`             → contributes 0 to that axis
- `partial(f, _)`    → contributes `1 - f` (confidence-weighted)
- `fail(_)`          → contributes 1 (saturating)

The composite is the vector of per-property contributions. Trajectory
domination is component-wise on that vector. There is no separate
"Shannon" axis and no separate "Dark" axis — if Shannon-style entropy or
Dark-span coverage matter at parse altitude, they appear as properties
under `@epistemologic/property/...` declared at parse altitude.

Property composition is mechanical. The properties present in the
substrate today (`@epistemologic/property/io_safety`, `laws/monotonicity`,
`laws/causality`, `halts`, `total_classification`, `laws/monoidal`, `laws/functor`,
`laws/monad`, `frame_relativity`, `glass_wall`, `is_prism_record`,
`autopoietic`, `coincidence_matches`, `content_addressed`,
`filename_matches_glass`, `laws/duration_algebra`, `duplicate_variant`,
`benchmark`) are not specifically parse-altitude properties — they are the
cross-altitude library. **What properties land at parse altitude is a
substrate-design question, not a tournament-design question.** When
parse-altitude property files appear under
`boot/std/epistemologic/property/parse/...` (or wherever the substrate
places them), `reflect` at parse altitude returns their verdicts, and
those verdicts ARE the loss.

Forward claim: the spec assumes parse-altitude properties will be declared.
None exist yet. The implementation sequence (§8) does not unblock until
they do — or until the substrate-design question of what they are is
answered. This is the only remaining substrate work the loss function
needs.

### 3.2 Conductivity tensor — the v0 / v1 / v2 path

Alternatives that are siblings under the same `@mirror/glass/ast/shape/*`
parent conduct; alternatives across distant substrate paths do not. This is
the cellular sheaf restriction map (`project-eigenboard-is-sheaf`).

The research literature on graph-Laplacian / sheaf-Laplacian propagation
has independently converged on a hybrid substrate-prior + learned-residual
formulation; pure-learned conductivity is incoherent with the spec's Hajek
convergence claim unless weights freeze per round. The path is staged:

**v0 — substrate-hierarchy prior (the version that lands first).**

`F_substrate(u ◁ e)` is computed from `@mirror/glass/ast/shape/*` path
distance:

- Trajectories rooted under the same `glass <type>` declaration → high
  conductivity (close siblings in the same record union).
- Rooted under different `glass <type>` declarations but same parent
  substrate path (e.g., both under `@mirror/glass/ast/shape/`) → medium
  conductivity.
- Rooted under different parent paths → low conductivity.

Hajek-trivial: stationary by construction (the substrate hierarchy doesn't
change during a round). Substrate-pull aligned. Zero training data
required. Auditable — the tensor's entries derive deterministically from
file paths.

**v1 — + corpus-frequency residual.**

Once spectral-db has settled parses to count, a residual term is added:
alternatives that historically co-occurred at the same parse position
carry an additive bump. The residual is logged from settled parses and
applied as `F = F_substrate + α · F_corpus`, with `α` frozen per round to
preserve stationarity. Cheapest learned residual; lands when the corpus
exists.

**v2+ — + curvature-detected residual.**

Narrow scope: only edges that Balanced-Forman curvature flags as
over-squashing bottlenecks (Topping 2021; Nguyen PIORF 2024). The
residual `ΔF` is computed per round on negatively-curved edges; elsewhere
the tensor is unchanged. Hybrid stays stationary per round; only the
curvature-flagged subset is touched.

**Critical — pure-learned is forbidden.** A pure-learned conductivity
tensor (no substrate base) would break the Hajek 1988 stationarity
precondition cited in §2.1. The cooling schedule's asymptotic bound
requires the per-round dynamics to be stationary; freely-updating weights
are not stationary. The Bodnar / Topping / PIORF literature has
independently converged on hybrid for exactly this reason; nobody in the
cellular-sheaf community recommends pure-learned for new systems.

Citations:

- **Hajek 1988** — *Cooling Schedules for Optimal Annealing.* The
  stationarity precondition; the convergence bound used in §2.1.
- **Ford 2004** — *PEG: A Recognition-Based Syntactic Foundation.* The
  underlying parse-tree shape over which the tournament runs.
- **Bodnar et al. 2022** — *Neural Sheaf Diffusion* (arXiv 2202.04579).
  The cellular-sheaf formulation our restriction maps follow.
- **Topping et al. 2021** — *Understanding Over-Squashing via Curvature*
  (arXiv 2111.14522). Balanced-Forman curvature; the diagnostic for v2's
  narrow scope.
- **Opedal et al. 2023** — *Efficient Semiring-Weighted Earley Parsing*
  (arXiv 2307.02982). Semiring shape of per-trajectory loss aggregation.
- **Nguyen et al. 2024** — *PIORF: Physics-Informed Ollivier-Ricci Flow.*
  Curvature-targeted residual; v2's mechanism.

This re-uses `au-and-conductivity.md`'s tensor structure and
`eigenboard-representation.md`'s G-bundle framing without modification.
The parse-altitude tensor is one section of the substrate-wide bundle;
v0 is what lands at Step 8.

### 3.3 Tournament cardinality — sub-Turing closes the question

Bounded by **the active glass set at current parse state**. Period.

Glass declarations are finite files; the active glass set at any state is
finite by construction (sub-Turing commitment from
`docs/insights/2026-05-26-mirror-sub-turing-substrate-with-emergent-turing-completeness.md`).
Cardinality is always bounded — the substrate cannot declare an infinite
alternative set, so the tournament cannot face one.

Hajek 1988's convergence bound (cited in §2.1) requires finite cardinality
per round. The substrate's sub-Turing commitment delivers it for free. No
static cap is needed as a safety net; no dynamics question is open;
pathological grammars in the substrate-pull-forbidden sense cannot exist.

### 3.4 Learning — `eⁿ⁺¹ < eⁿ` at parse altitude

Fate model parameters update from settled parses. A successful parse —
where a winning trajectory survived the tournament and produced an AST
that downstream consumers accepted — reinforces the winning trajectory's
parameters. A failed parse (loss composite saturated; downstream rejected
the AST) does not reinforce.

The project CLAUDE.md proof — *"eⁿ⁺¹ < eⁿ. The system learns from its
errors. The errors get smaller."* — applies at parse altitude as elsewhere.
Parses get easier as the system parses more, *for grammars it has seen
before*. Novel grammars start from the substrate prior (the conductivity
tensor's hierarchy structure, v0) and learn from there.

Learning loop scope (which parses contribute) is open (§7.3).

### 3.5 Stopping rule — convergence IS the commit

There is no separate commit threshold. The Hajek convergence already
settles the winner; when the tournament converges, the winner commits.
That is the whole story.

Earlier drafts of this spec introduced a per-altitude "commit threshold"
imported from PEG cut-point / Prolog cut vocabulary. That import was an
error: tournament dynamics don't need a cut. The Lyapunov function
descends per round; when no trajectory can catch the leader under the
remaining cooling budget, the tournament has converged; that round is
the commit.

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
  forest is the tournament's alternative set; the loss is the composite
  of `@epistemologic/properties` at parse altitude; the stopping rule is
  Hajek convergence. The first tournament that runs.
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
- **Convergence point.** The round at which the winner's lead became
  uncatchable under the remaining cooling budget; the commit (§3.5).
- **Settlement verdict.** Did downstream accept the produced AST?

This is the same shape `kintsugi-tournament.md` §6 names for merge-altitude
tournaments, transferred to parse altitude. Reflection sees the structure;
the Scheduler Tower (Phase 5) routes the gestalt entry.

The gestalt entry IS the parse run's typed footprint. Replay reproduces
the tournament from the entry; learning reads the entry and updates the
prior.

---

## 7. Load-bearing decisions

After Alex's review of `9263ce5`, the open list collapses to **one open
thread plus three edge cases**. The rest is resolved.

### 7.1 Loss function composition — RESOLVED

The loss IS the composite of `@epistemologic/properties` at the relevant
altitude (§3.1). Mechanism: `reflect(ast)` returns the verdict vector;
`pass / partial / fail` map to per-axis contributions; trajectory
domination is component-wise.

What remains is a **substrate-design question, not a tournament-design
question**: which specific properties land at parse altitude. The
tournament reads whatever properties the substrate declares; the
declaration is a separate (sub)spec.

### 7.2 Conductivity tensor formulation — RESOLVED at v0

v0 (substrate-hierarchy prior) is the version that lands. v1 (+
corpus-frequency residual) and v2+ (+ curvature-detected residual) are
staged future work named in §3.2. Pure-learned is forbidden by the Hajek
stationarity precondition cited in §2.1; the literature has independently
converged on hybrid (Bodnar 2022, Topping 2021, Nguyen 2024).

### 7.3 Learning loop scope — open

Which parses contribute to Fate parameter updates? Candidates:

- **All settled parses.** Every parse that succeeded structurally feeds
  the prior, regardless of downstream verdict.
- **Type-checked parses only.** Only parses whose AST survived type-
  checking contribute. Filters out parses that produced syntactically
  valid but semantically empty ASTs.
- **Crystallised parses only.** Only parses whose AST ended up in
  spectral-db (i.e., the `mirror compile` artefact landed and was indexed)
  contribute. Strictest filter.

Reed's lean: type-checked parses. The all-settled bucket admits noise
(parses that look right but produce nonsense ASTs); the crystallised bucket
is too sparse to learn from. Type-check is the structural filter that says
"this parse produced a valid downstream artefact."

Resolution awaits the Phase 5 Reflection surface.

### 7.4 Three hole edge cases

The `\name:type` typed-hole syntax (§1.3) opens three sub-questions worth
naming. Each is interesting enough to surface; none is blocking on v0.

- **Repeated holes.** `\name:identifier ... \name:identifier` — do the
  two occurrences unify (Prolog-style, must match the same value) or are
  they independent captures with a shared lexical name? Reed's lean:
  unification. It matches the eigenboard-sheaf restriction shape (two
  positions in the same shape are the same cell of the sheaf) and produces
  the right semantics for patterns like "a binding whose name on the LHS
  must equal the name on the RHS."

- **Hierarchical holes.** `\node:expr` where `expr` is itself a glass
  shape declared with internal holes. Holes nest; the parser recurses
  through the nested shape; captured fragments are themselves abstract
  blocks with bound holes. Worth naming as a design surface; the recursive
  case falls out of the substrate-uniform parse loop without special-casing.

- **Hole-typed-by-input.** `abstract(t: type) { \x:t }` — a hole whose
  type is dynamically the value of the header input `t`. This probably
  falls out of substrate-uniform name lookup (the type position resolves
  the input the same way any other identifier does), but worth confirming
  in the first implementation tick that exercises it.

### 7.5 Empty header semantics — RESOLVED

Resolved in §1.4: empty `abstract { ... }` = empty quote. No header, no
`\` marks, all names literal; the block returns the AST literally. The
resolution is the analogue Alex named in review: empty header = empty
quote semantics.

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
a structural unlock. This spec is that unlock. Substrate already declares
`zoom abstract` (commit `7461782`); the engine is Fate, already on the
substrate-pull-reflex path; the math is Hajek 1988 + Lyapunov + cellular
sheaf, all already cited. Net: ~5 lines of keyword recognition + ~50 lines
of loader; ~320 lines deleted; every future shape costs zero Rust. The
error shape is `eⁿ⁺¹ < eⁿ` at parse altitude as elsewhere.

The Phase 1 audit named the lie. This spec names the truth.

---

## 11. Threads to track but not block on

Not open decisions — just surfaces that are sketched, not yet load-bearing,
and will land when a real grammar demands them.

- **Mixed parse/construct call sites** (§1.4). The surface syntax for
  partial hole-filling — some holes pre-filled by the caller, others
  captured from input — is the general case. The pure-parse and
  pure-construct ends are sketched; the mixed middle waits for a real
  use site.
- **Multiple `abstract` blocks in one glass declaration.** Composability
  rules for stacking patterns. Not surfaced yet; will land when a real
  grammar demands it.

The §7 open list (one thread + three edge cases) is the canonical
settle-next reference.

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
