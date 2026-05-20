# `kintsugi-wiring` — closing the eight `\` between kintsugi and conductivity

*2026-05-20. Reed. Partially superseded by `kintsugi-formatter.md`
(commit `900eb40`, same day): that spec absorbs the eight wires into
five iteration stages plus three failure-handling stages, with the
Lawvere fixed-point check as the stopping criterion and the Banach
contraction theorem as the convergence guarantee. This spec remains
valid as the implementation guide — the wires are the moving parts
the formatter drives — but the mathematical structure now lives in
the formatter spec.*

Status: **Red** (every wire is declared; every body is `\` or trivially
pass-through; the loop does not yet observe itself).

**See also:** `docs/specs/kintsugi-formatter.md` for the iteration
rule, the contraction-map analysis, and the Lawvere-fixed-point
stopping criterion. The mapping from this spec's eight wires to the
formatter's stages is in §"The eight wires retold" of that spec.

Depends on:
- `docs/specs/mirror-compile-bootstrap.md` (Spec A) — the staircase,
  io lambdas, the kintsugi formatter framing
- `docs/specs/match-select.md` (Spec B) — `match(refract)` produces au;
  `match(split)` feeds the tournament
- `docs/specs/au-and-conductivity.md` — au as Fate's output type; the
  5×5 conductivity tensor; conductivity as the verification metric
- `docs/specs/strict-and-total-classification.md` — the `Dark` AST kind
  is now emitted; counting it is the cheapest loss surface
- `docs/specs/mirror-runtime-gen-prism.md` — content-addressed actors;
  the kintsugi-reduction loop state lives in a gen_prism's crystals
- `docs/specs/lsp-and-mcp.md` — the gutter that surfaces dark regions;
  the auto-reload contract that piggy-backs on traffic

Unblocks:
- The first observable kintsugi-reduction tick: dark-count loss before
  and after, written to the gen_prism's ancestor chain
- Fate-proposed au candidates flowing through `match(@fate/tournament)`
  into the renderer
- The diff/review surface (future tick): tournament results presented
  to the user as proposed changes with a @nl/english summary

---

## Thesis

Mirror's kintsugi tick is declared end-to-end. Every grammar in the
loop has a name, an input type, an output type, and a place in the
pipeline. **Every body is `\`.** The loop cannot yet observe itself
because the wires between the grammars are unstrung.

This spec names the wires. Eight of them. Each wire is a `\` body
whose closing turns the next wire's `\` into something the model
checker can reason about. The wires ride the kintsugi ladder from
Spec A — they ARE the ladder, applied to kintsugi itself.

Concretely:

```
dark region observed
        ↓ wire 1
@fate.infer(hole_oid) — five-model fan-out
        ↓ wire 2
au candidates from Abyss / Cartographer / Explorer / Introject / Fate
        ↓ wire 3
@fate.conduct(au, context) — conductivity contest, reduce to one winner
        ↓ wire 4
@mirror/spectral.crystallize + render — write-back into the source AST
        ↓ wire 5
loss(n) measured against loss(n−1) — the e^(n+1) < e^n invariant
        ↓ wire 6
@cogito.observe(beam_n, beam_n+1) — Reflection reads the delta
        ↓ wire 7
@cogito.strategy() — perturbation for the next tick
        ↓ wire 8
--shatter N loop — repeat until loss ≤ ε or N exhausted
```

The diff/review surface (Section 3) is the user-facing layer that
makes the loop *useful* once it's running. It is a future tick. It
does not gate any wire.

---

## Section 1 — Audit: what exists, what's `\`

This section walks every wire and reports its declared shape, its
current body, and what it depends on. Honest assessment: most bodies
are `\`. Some are trivially pass-through. None close the loop.

### Wire 1 — kintsugi → Fate

**Declared:** `@kintsugi.collapse(ast, ast) -> imperfect { \ }` in
`boot/std/kintsugi.mirror`. `@fate/tournament.candidates(hole) -> [resolution]`
in `boot/std/fate/tournament.mirror`. Body of `candidates` lists the
five ganglion calls but the LIST is the body — there is no dispatcher
that actually invokes `@fate.infer(hole_oid)` from inside kintsugi's
collapse.

**Today's state:** `\`. The kintsugi command (`bootstrap/src/main.rs::cmd_kintsugi`)
reads a file, tokenizes, renders canonically, writes stdout. It does
not detect Dark children. It does not call Fate. It does not write back.

**Returns / accepts:** `collapse` accepts two ASTs and returns an
`imperfect`. The signature is right; the body is the gap.

**Direct dependencies:**
- A way to enumerate Dark children in the AST (Spec D landed `Dark`
  in `bootstrap/src/ast.rs`; the enumeration is a walk).
- `@fate.infer(hole: oid) -> imperfect(au, no_proposal, loss)` —
  declared in `boot/std/fate.mirror` (the spec adds it; today the
  grammar carries `io tick`, `io resolve`, `io select`, no `infer`).
- The `match(@fate/tournament)` modifier (Spec B) — the way to
  EXPRESS the kintsugi→Fate dispatch in grammar rather than Rust.

**Status:** completely `\`. **No grammar declaration yet** for
`@fate.infer`; Spec C (`au-and-conductivity`) prescribes it as part
of the au retype. Flagged as an earlier obligation.

### Wire 2 — Fate → models

**Declared:** the five model grammars in `boot/std/ai/*.mirror`:
- `@ai/abyss.depth(ast, context) -> u64 { \ }`
- `@ai/cartographer.map(ast, neighborhood) -> topology { \ }`
- `@ai/explorer.explore(ast, target) -> [ref] { \ }`
- `@ai/introject.pattern(ast, history) -> [ref] { \ }`
- `@ai/fate.select(features, candidates) -> ganglion { \ }`

`@fate/tournament.candidates(hole)` enumerates them by name.

**Today's state:** every model body is `\`. The tournament's
`candidates` body lists the five calls; nothing invokes the list.

**Returns / accepts:** the five models each accept different shapes
(an AST + context, an AST + neighborhood, etc.). Their return types
are NOT au today — they're `u64`, `topology`, `[ref]`, `[ref]`,
`ganglion`. **Spec C requires they all return `imperfect(au, ...)`**
so the tournament can compare candidates apples-to-apples.

**Direct dependencies:**
- `type au = ai` declared in `@fate` (Spec C, implication 1).
- Each model's return type re-stated as `imperfect(au, no_proposal, loss)`.
- A connectome instance for each model (Fate's connectome grammar
  declares 450 nodes / 5 ganglia / 18-54-18 per ganglion; today the
  graph data is not stored anywhere addressable).

**Status:** completely `\`. The model bodies are five separate
obligations; until they return au, wire 3 has nothing to measure.

### Wire 3 — Conductivity contest

**Declared:** `@fate/tournament.tournament(rules, [hole]) -> [resolution] { \ }`.
The rule set (`elite(1).beam(8).halving(3)`) is named. The reduction
policy is declared. The body is `\`.

`@hash/coincidence` declares the 5×5 conductivity tensor structurally
(`type dimension`, `type duality`) but does NOT expose a `tensor` type
or a reduction operation. Spec C, implication 4 prescribes:
`type tensor = matrix(dimension, duality, f64)`.

`@fate.conduct(value: au, context: oid) -> conductivity { \ }` —
declared in Spec C, not yet in `boot/std/fate.mirror`.

**Today's state:** `\` at both layers. The tournament cannot reduce
candidates because (a) candidates are not au yet, (b) `conduct` is
not declared in grammar, (c) the tensor type is not declared in grammar.

**Returns / accepts:** `tournament(rules, [hole])` accepts hole list
and rules; returns resolutions. The shape of `resolution` is implicit
text today; Spec C names it au.

**Direct dependencies:** wire 2 (candidates must be au); declaration
of `type tensor`, `type conductivity`, `action conduct` in grammar.

**Status:** completely `\`. The reduction policy from tensor to
verdict is a separate design (Spec C, implication 7).

### Wire 4 — Write-back

**Declared:** `@mirror/spectral.crystallize(ast) -> crystal { @git.store(ast) -> oid }`.
This wire IS partially concrete — `crystallize` resolves through `@git.store`.

`@mirror/spectral.recall(crystal) -> imperfect { \ }` is `\`.

The renderer is `bootstrap/src/render.rs::render_ast`, bound through
Spec A's `io render_ast(ast, indent, out) = @code/rust(~f"./bootstrap/src/render.rs") > fn[name="render_ast"]`.
The renderer round-trips ASTs faithfully (Cluster D's butterfly
verified this).

What's missing: a function that takes an au value (a Dark region's
proposed resolution), splices it into the parent AST, and re-renders
the file. The splice operation does not exist in grammar today. The
crystallize/recall path can store the result, but the IN-PLACE WRITE
to the source file is not declared.

**Today's state:** partially concrete (crystallize closes through
@git.store; the renderer round-trips). The splice — replacing a Dark
region with its resolved AST shape inside the parent — is undeclared.

**Returns / accepts:** crystallize accepts AST, returns crystal.
Recall accepts crystal, returns imperfect. The splice would accept
(parent_ast, dark_child_oid, replacement_ast) and return ast.

**Direct dependencies:**
- A `splice(parent, hole_oid, replacement) -> ast` declaration. New.
  Most natural home: `boot/std/mirror/match.mirror` (the match form
  IS the dispatcher that identifies which arm's body replaces the
  hole), or `boot/std/kintsugi.mirror` (the kintsugi formatter does
  the actual structural transform).
- An accept-policy: do we write back into the source file directly,
  or stage the resolution to `refs/fate/<hole_oid>` and let the user
  apply via the diff/review surface (Section 3)?

**Status:** partial. The crystallize side is wired; the source-file
write-back is undeclared. **The splice operation has no grammar
declaration yet.**

### Wire 5 — Loss metric per tick

**Declared:** `@beam.compare(beam_a, beam_b) -> speedup` in
`boot/std/beam.mirror`. Speedup is a ratio. The compare action is
abstract.

`@epistemologic/property/total_classification.dark_count(ast) -> u64 { \ }`
(Spec D). This is the cheapest loss surface: count Dark children
before the tick; count after; the delta IS `e^n − e^(n+1)`.

`@beam.observe(beam) -> imperfect` is declared abstract.

**Today's state:** `\` for the property body. The bootstrap already
counts dark in `cmd_craft_with` (the `total_dark` / `files_with_dark`
locals). What is NOT done: writing the count to the gen_prism's
crystal as a tick observation; computing the delta from the prior
tick's crystal.

**Returns / accepts:** `dark_count` accepts AST, returns u64. The
delta is u64 minus u64. The `e^(n+1) < e^n` invariant is one
comparison.

**Direct dependencies:** none beyond what Spec D declared. This wire
is the FIRST observable wire — it doesn't need any model body to
close. It only needs to walk the AST and count.

**Status:** declared (Spec D). The body is `\`, but the Rust side
already counts dark in `cmd_craft_with`. Closing the property's body
is a thin walk over the AST emitting the count.

### Wire 6 — Reflection observes

**Declared:** `@cogito.observe(imperfect) -> observation { @beam.emit }`
in `boot/std/cogito.mirror`. The body is **not `\`** — it
pass-throughs to `@beam.emit`. But `@beam.emit(ast, imperfect) -> beam`
is abstract; the body that produces a beam from an imperfect is `\`.

`@cogito.reflect(imperfect) -> imperfect { observe |> strategy |> perturb }`
composes three actions. The composition is concrete; the actions
themselves are abstract or `\`.

**Today's state:** trivially pass-through. The cogito grammar
declares the SHAPE of reflection (observe → strategy → perturb) but
each step delegates to a `\` or abstract action.

**Returns / accepts:** observe accepts imperfect, returns
observation. The observation type is not declared concretely in the
boot tree today — it's a value passed between Reflection's steps.

**Direct dependencies:** wire 5 (loss numbers to compare); a
declaration of `type observation` that names what Reflection actually
holds between observe and strategy.

**Status:** partial. The pipeline is declared but every step bottoms
out in `\` or abstract.

### Wire 7 — Reflection picks strategy

**Declared:** `@cogito.strategy(observation) -> tournament { elite(1).beam(8).halving(3) }`
in `boot/std/cogito.mirror`. The body is a literal rule expression.

`@cogito.perturb(observation, tournament_result) -> eigenboard { @beam.observe }`
pass-throughs to `@beam.observe`.

**Today's state:** the strategy body returns a HARD-CODED rule set.
Every tick gets the same rule. The strategy is not actually a function
of the observation — it ignores its input. Honest description: the
declaration is concrete but trivial; it does not adapt.

**Returns / accepts:** strategy accepts observation, returns
tournament rules. Perturb accepts observation + tournament result,
returns an eigenboard (which is not declared concretely in the boot
tree).

**Direct dependencies:**
- A declaration of `type eigenboard` (the perturbation state).
- A strategy body that reads the observation's loss delta and adjusts
  the rule. Today's `elite(1).beam(8).halving(3)` is a sensible
  default; adaptation is the future.

**Status:** trivially concrete. The decision is hard-coded, not
inferred. To unlock wire 8's tick-over-tick adaptation, the strategy
needs to read the observation.

### Wire 8 — `--shatter N` loop

**Declared:** `@kintsugi.shatter(file, level) -> imperfect { @kintsugi/shatter.fracture_and_repair(file, level) }`.
`@kintsugi/shatter.fracture_and_repair(ast, level) -> imperfect { shatter(ast, level) |\> settle_up }`.
The composition is declared. The body of `shatter` (recursive fracture
at Fiedler zero crossings) and `settle_up` (tournament per piece) are
declared as compositions, but the underlying operations they compose
through (`focus(fiedler) |> split(zero_crossings)`, `tournament(...)`
inside `settle_up`) bottom out in `\`.

The CLI surface exists: `bootstrap/src/main.rs` accepts `kintsugi`
but does NOT yet pass `--shatter N` or invoke
`@kintsugi/shatter.fracture_and_repair` from inside the Rust dispatch.

**Today's state:** `\` underneath the declared composition. The
recursion is declared; the base case (one tick) is undeclared.

**Returns / accepts:** shatter accepts file + level, returns
imperfect. The level controls recursion depth.

**Direct dependencies:** wire 5 (loss to compare); a base-case
implementation that performs ONE tick (no fracture, no recursion) so
the loop has something to iterate.

**Status:** declared at the composition level; bodies underneath are
`\`. The CLI does not yet route to it.

### Audit summary

Of the eight wires:

| Wire | State | Earlier obligation? |
|---|---|---|
| 1. kintsugi → Fate | completely `\` | `@fate.infer` not declared in grammar yet |
| 2. Fate → models | completely `\` | model return types must become `imperfect(au, ...)` |
| 3. Conductivity contest | completely `\` | `type tensor`, `action conduct` not declared in grammar yet |
| 4. Write-back | partial (crystallize wired; splice missing) | `splice` not declared anywhere in grammar |
| 5. Loss metric | declared, body `\` | none — Rust side already counts dark |
| 6. Reflection observes | trivially pass-through | `type observation` not declared in grammar |
| 7. Reflection picks strategy | trivially concrete (hard-coded) | `type eigenboard` not declared in grammar |
| 8. `--shatter N` loop | declared composition, `\` underneath | CLI does not route to it |

**Missing grammar declarations** (earlier obligations the spec assumes
elsewhere but the boot tree does not carry today):

- `@fate.infer(hole: oid) -> imperfect(au, no_proposal, loss)` — Spec C
- `@fate.conduct(value: au, context: oid) -> conductivity` — Spec C
- `type au = ai`, `type conductivity` — Spec C
- `type tensor = matrix(dimension, duality, f64)` — Spec C
- `splice(parent, hole_oid, replacement) -> ast` — this spec (Section 5)
- `type observation`, `type eigenboard` — implicit in @cogito; this
  spec calls them out
- `@nl/english.summarize(au, context) -> text` — concrete enough that
  this spec uses it; today the project body is `\`

The earlier-obligations list is the path between "spec describes the
type system" and "grammar carries the type." Each item is one short
grammar edit.

---

## Section 2 — Order by leverage

The wires do not have to land in numerical order. Some unlock others;
some are cheap and independently observable; some need real machinery.
The cheapest-first principle: which wire makes the loop OBSERVABLE
without requiring any model body to close?

The answer is wire 5 (loss metric) and wire 8 (the loop itself, with
a no-op tick body). Together they produce: a tick that counts dark,
does nothing else, and writes (count, delta) to a gen_prism crystal.
Loss is `0` because nothing resolved. The loop runs. The system can
see itself.

Recommended order:

### Step 1 — wire 5 (loss metric): close `dark_count` over the AST

**Prerequisites:** none. Spec D already added the `Dark` kind.

**Body:** a walk over `[AstNode]` summing children whose kind is `Dark`.
Pure, total, decidable. The model checker proves termination from
the AST's finite depth.

**Concretely enables:** any tick can report a number. The first
honest loss surface lands.

**Complexity:** small. One sub-Turing lambda body, ~10 lines.

**Honest minimum:** literally `[AstNode] -> u64` over the tree.

### Step 2 — wire 8 (loop scaffolding): one-tick `mirror kintsugi --shatter 1 <file>`

**Prerequisites:** wire 5 (something to report).

**Body:** the CLI accepts `--shatter N`. Per tick: tokenize the file,
count dark before, run the kintsugi formatter (which is the no-op
canonical renderer today), count dark after. Write a beam with
`(loss_before, loss_after, dark_before, dark_after)` to a gen_prism
crystal at `refs/gen_prism/kintsugi/<file_oid>`. Stop when `dark_after
== 0` OR `N` exhausted OR `loss_after >= loss_before`.

The "kintsugi formatter" body for this step is a no-op resolver — it
proposes nothing, the dark count never decreases, the loop exits on
tick 1 because `loss_after >= loss_before`. **The loop is observable
before any model fires.**

**Concretely enables:** the user sees `mirror kintsugi --shatter 1 file.mirror`
report `tick 1: dark 7 → 7 (loss 0.0 → 0.0, no progress, stopping)`.
The gen_prism crystal carries the observation.

**Complexity:** medium. CLI routing + gen_prism wiring + loop. The
gen_prism primitive itself is declared (`@mirror/runtime/gen_prism`);
the kintsugi-reduction loop is a concrete subclass.

**Honest minimum:** no model invocation. The tick body is the
identity transform plus an observation write.

### Step 3 — wire 6 + wire 7 wire-up (trivial bodies)

**Prerequisites:** wires 5 and 8 (something to observe).

**Body:** declare `type observation = { dark_count: u64, loss: loss, tick: u64 }`
and `type eigenboard = { current_strategy: tournament_rules }` in
`@cogito`. The observe / strategy / perturb bodies wire through. Today
they're hard-coded; the wire-up makes them addressable.

The strategy body initially returns `elite(1).beam(8).halving(3)`
regardless of observation. This is honest: until there's variance in
observations, there's nothing to adapt to.

**Concretely enables:** Reflection observes the tick output. The
strategy is a function of the observation, even if today's function
is constant. The gen_prism crystal now carries an observation AND a
strategy per tick.

**Complexity:** small. Type declarations + composition; bodies bottom
out in literal returns.

**Honest minimum:** types + literal strategy + observation pass-through.

### Step 4 — declare `@fate.infer`, `@fate.conduct`, `type au`, `type conductivity`, `type tensor`

**Prerequisites:** none (pure grammar edits).

**Body:** the four declarations from Spec C. Bodies stay `\`. The
grammar now CARRIES the contract Spec C describes; the model checker
can reason about the types in subsequent wires.

**Concretely enables:** wires 1, 2, 3 can be expressed in grammar
because their signatures are now declarations rather than
specification text.

**Complexity:** small. Five grammar edits to `boot/std/fate.mirror`
and `boot/std/hash/coincidence.mirror`.

**Honest minimum:** the smallest possible declarations that make the
types exist; no bodies beyond `\`.

### Step 5 — wire 1 (kintsugi → Fate) with one model: Abyss-only baseline

**Prerequisites:** steps 1-4. `@fate.infer` declared. `@ai/abyss.depth`
returns `imperfect(au, no_proposal, loss)`.

**Body:** the kintsugi formatter, on encountering a Dark child, calls
`@fate.infer(dark_oid)`. `@fate.infer` calls only Abyss (skip the
tournament for this step). Abyss's `depth` body is closed manually —
the simplest possible body that returns SOMETHING. For a Dark region,
Abyss returns `light(empty_au, 0.0)` — "no proposal, but I observed
the depth."

The tick now executes: dark observed → Abyss called → empty au
returned → no splice → dark count unchanged → loop exits.

The loop runs through Fate without a tournament. The Rust path
through `@fate.infer` is exercised. Nothing structurally lands; the
machinery works.

**Concretely enables:** the kintsugi → Fate handoff is alive. The
gen_prism crystal records the Fate call. The user sees `tick 1:
called @ai/abyss for hole #abc... (no proposal)`.

**Complexity:** large. The first model body, the first Fate
dispatcher, the first au value produced (even if empty). This is
where Spec A's "manual bootstrap stops" line falls (Section 4).

**Honest minimum:** Abyss returns empty au. No tournament. No
write-back.

### Step 6 — wire 4 (write-back) declaration: stage to `refs/fate/<hole_oid>`

**Prerequisites:** step 5 (something to write back).

**Body:** when `@fate.infer` returns a non-empty au, write the au's
crystal to `refs/fate/<hole_oid>` via `@io.git_update_ref`. Do NOT
write back into the source file. The diff/review surface (Section 3)
is the user-visible accept path; the loop only STAGES the resolution.

Declare `splice(parent_ast, hole_oid, replacement_ast) -> ast` in
`@kintsugi.mirror`. Body `\`. The declaration exists; the operation
is not yet invoked.

**Concretely enables:** Fate proposals accumulate at `refs/fate/*`.
The gen_prism crystal records which holes have proposals. The user
can inspect `git log refs/fate/<oid>` and see the proposal history.

**Complexity:** medium. The staging path is `@io.git_update_ref`;
splice is the missing structural operation.

**Honest minimum:** ref-write only. No source-file mutation. No
splice body.

### Step 7 — wire 2 (the other four models) + wire 3 (tournament reduction)

**Prerequisites:** step 5 (the dispatcher works for one model).

**Body:** retype Cartographer, Explorer, Introject, Fate-the-selector
to return `imperfect(au, no_proposal, loss)`. Their bodies stay `\`
or become trivial no-ops. The tournament's `tournament(rules, [hole])`
body fans out across all five, collects au candidates, and reduces.

The reduction body is the conductivity contest. For step 7's minimum:
`conduct(value, context)` returns a constant `clear` for the first
non-empty au; `none` for empty au. No real measurement; the structural
shape of the tournament fires.

**Concretely enables:** the full five-model fan-out is exercised. The
tournament reduces. The gen_prism crystal records all five proposals
and the winner.

**Complexity:** large. Five model bodies + tournament reduction +
conductivity stub.

**Honest minimum:** trivial bodies for the four new models; constant
conductivity verdict.

### After step 7

Subsequent ticks close real bodies for each model in turn. Each model
that gains a real body increases the tournament's signal. The
conductivity reduction gains its real policy (Spec C, implication 7).
The strategy adapts to observation deltas. The system starts to learn.

**The loop is observable after step 2.**

**Fate is in the loop after step 5.**

**The tournament fans out after step 7.**

Everything beyond step 7 is closing model bodies and tightening
conductivity. Each closure is its own kintsugi obligation; the loop
itself watches them close.

---

## Section 3 — The diff / review surface (future tick)

This is the user-facing layer that makes the loop *useful*. It is
not on the critical path for any wire. **Do not gate the wiring work
on this section.**

### The diff format

A Fate tournament round at tick `n` over hole `h` produces 1–5 au
candidates. Each candidate is a structural proposal for what the Dark
region should become. The diff is the proposal-set rendered for human
review:

```
=== kintsugi --review boot/std/mirror/reload.mirror ===

[hole #a3f1c2 — line 36, col 5]
  dark bytes:
    \,
  proposals:

  [1] @ai/abyss            conductivity: clear     (entropy=0.92, spectral=0.88, cheeger=0.94, ricci=0.81, mixing=0.90)
      [@mcp.notification("tools/list_changed")]
      summary: "this becomes the list_changed notification expected when grammars_hash drifts"

  [2] @ai/cartographer     conductivity: partial(0.6)
      [@mcp.notify({ method: "tools/list_changed" })]
      summary: "this becomes a notify call with method field set to the drift signal"

  [3] @ai/explorer         conductivity: none
      []
      summary: "this becomes an empty list — explorer found no proposal"

  selected: [1] @ai/abyss

  [a]ccept  [r]eject  [m]odify  [n]ext  [q]uit
```

The diff shows: dark bytes verbatim; proposals indexed; each proposal's
conductivity tensor (5-cell row); the rendered au value via the
existing `render_ast` io binding; the proposing model; the tournament's
selection.

### The accept/reject surface

```
mirror kintsugi --review <file>      # one file, all dark regions
mirror kintsugi --review boot/       # every file in target
```

Like `git add -p` for AST proposals. The user steps through holes.
Per hole: accept the selected proposal, reject all, pick a different
proposal from the tournament's candidate set, or modify (drop into
`$EDITOR` with the proposal pre-filled).

Accepting a proposal triggers the splice (wire 4): the parent AST's
Dark child is replaced with the au's content. The file rewrites. The
crystal moves. The gen_prism's ancestor chain gains one tick.

Rejecting writes a "rejected" record to `refs/fate/<hole_oid>` so
future tournaments learn the proposal didn't conduct here. The
relational entanglement principle (Spec C) means a rejection in this
context doesn't apply to a different hole, even if the bytes match.

### The @nl layer

When showing a diff, the surface invokes `@nl/english` to produce a
one-sentence structural summary of each proposal:

```
@nl/english.summarize(proposal: au, context: oid) -> nl { \ }
```

Today `@nl/english` declares the five-operation surface
(split / focus / zoom / refract / project) over English text with all
bodies `\`. `summarize` does not yet exist; it composes
`split.sentences |> focus.role |> zoom.nominalize |> project.summarize`
to produce one sentence describing what the proposal does
structurally, not lexically.

The structural summary reads from the proposal's AST shape, not its
bytes. "This becomes a match expression that handles the drift case
explicitly" rather than "added 12 lines, removed 3 lines." The
description names the AST kinds and their relationships.

### Impact analysis

When a Dark region resolves into a known AST kind, downstream changes:

- Grammars that imported the resolved file gain new structure.
- Their content_oids shift (the parent's child set changed).
- Their crystals move to new addresses.
- The gestalt eigenvalue redistributes.

The diff surface measures the blast radius before the user accepts:

```
  impact: accepting [1] will:
    - shift content_oid of boot/std/mirror/reload.mirror   (#7c4...  → #9d2...)
    - move 3 dependent crystals:
        boot/std/mirror/serve.mirror
        boot/std/cogito.mirror
        boot/std/mirror/runtime/gen_prism.mirror
    - resolve 1 dark region elsewhere (cascade through @cogito.observe)
    - introduce 0 new dark regions
```

The "resolve N dark regions elsewhere" line is the conductivity
propagation made visible. A clear conductivity in one wire often
clears downstream wires whose dark depended on it. The impact view
is the @cogito.observe step exposed pre-acceptance.

### The implicit batch

Accepting all conducting proposals across a single
`mirror kintsugi --review boot/` invocation IS one full kintsugi tick.

```
   user: accept all      (10 holes resolved)
   ↓
   kintsugi: 10 splices, one beam emitted, Reflection observes
   ↓
   loss(n)  = 47 dark regions remaining
   loss(n−1) = 57
   e^(n+1) < e^n  ✓ — the system improved
   ↓
   strategy: same rule (no variance to adapt to)
   ↓
   loop: dark > 0, continue. next tick proposes for the remaining 47.
```

The diff surface is not separate from the loop. It IS the
user-visible face of one kintsugi tick. The "accept" action is the
tick's commit. The "reject" action is a tournament loss recorded for
future ticks.

This makes the diff surface the natural human-in-the-loop point. The
machine runs autonomously by default; when the user opens `--review`,
they participate in one tick.

### What this section does NOT prescribe

- The exact terminal UX (ncurses? readline? Claude Code MCP tool?).
- The bytes-level diff renderer (something will produce the visible
  text from the AST shapes; the implementation is not this spec's job).
- The @nl/english.summarize body itself (a separate kintsugi
  obligation that closes against @nl's five-operation surface).
- Cross-file batch review semantics beyond "accept all conducting
  proposals."

---

## Section 4 — Self-application: kintsugi wires kintsugi

The wires ARE kintsugi obligations. Each `\` body in the eight-wire
list is itself the kind of thing kintsugi resolves. So the wiring
process is bootstrap-by-bootstrap.

**Where the manual bootstrap stops:** step 5. The first few wires
(steps 1–4) must be closed by hand because there is no working
kintsugi tick yet. Once step 5 lands and Fate is dispatched from the
kintsugi formatter, the remaining wires can be reviewed via the diff
surface (Section 3) and replaced with Fate-proposed bodies that have
proofs.

**Where the self-application begins:** step 7. By step 7 the
tournament is real (five candidates, conductivity reduction, gen_prism
recording). From that point onward, each new wire's body is itself
proposable through `mirror kintsugi --review`:

```
mirror kintsugi --review boot/std/cogito.mirror
   # produces proposals for the perturb body, the strategy body,
   # the observation type — read by the model checker against the
   # contract @cogito declares.
```

The wires that close manually:

- Step 1's `dark_count` body — pure walk, ~10 lines.
- Step 2's CLI routing + gen_prism scaffolding — Rust + new grammar.
- Step 3's type declarations + literal strategy — pure grammar.
- Step 4's type declarations — pure grammar.
- Step 5's first Fate dispatch — Rust scaffolding inside cmd_kintsugi
  + Abyss's first concrete body.

The wires that close through self-application:

- Step 6's `@io.git_update_ref` invocation from grammar.
- Step 7's full model bodies + conductivity reduction policy.
- Every subsequent model closure (Cartographer's `map`, Explorer's
  `explore`, etc.).
- The strategy's adaptation logic (today constant; tomorrow a
  function of observation).
- The perturb body (today pass-through; tomorrow an eigenboard
  transform).

**The proof obligations that close each manual body** ARE the
property checks Spec A lists: terminates, deterministic, bounded_steps,
referential_transparency, total_classification. Closing the manual
bodies makes the property checks for those bodies pass; passing
property checks is what kintsugi accepts; what kintsugi accepts
becomes the inlined sub-Turing lambda. The Rust under each manual
body retires by Spec A's ladder.

The first kintsugi tick is asymmetric. After it runs, the asymmetry
shrinks. Each subsequent tick eats one more `\` and one more line of
Rust. The line where "manual" ends and "self-application" begins is
not a hard edge — it's the point at which the cost of running
kintsugi is less than the cost of writing the body by hand.

---

## Section 5 — Where the wires live in the bootstrap

Some wires need Rust to be closed. Some can be pure grammar once
enough Rust is in place.

| Wire | Where it lives today | Where it goes |
|---|---|---|
| 1. kintsugi → Fate | Rust (cmd_kintsugi extended to detect dark + call Fate) | grammar via `match(@fate/tournament)` once Spec B's parser lands |
| 2. Fate → models | grammar (model bodies are `\`) | grammar (bodies close one by one) |
| 3. Conductivity contest | grammar (`@fate/tournament.tournament` body `\`) | grammar (the reduction policy is grammar-expressible) |
| 4. Write-back staging | Rust (`@io.git_update_ref`) | Rust (kernel boundary; permanent floor) |
| 4. Write-back splice | grammar (`splice` declaration) | grammar |
| 5. Loss metric | grammar (`dark_count` body `\`) | grammar |
| 6. Reflection observes | grammar (cogito.observe → beam.emit) | grammar |
| 7. Reflection picks strategy | grammar (literal today; observation-driven later) | grammar |
| 8. `--shatter N` loop | Rust (cmd_kintsugi + gen_prism send loop) | grammar (the loop becomes a tick body once `@mirror/runtime/gen_prism.tick` closes) |

The Rust-side closures (wires 1, 4-staging, 8) are the same kind of
io bindings Spec A's grammar `@mirror/compile/bootstrap` declares.
They ride the kintsugi ladder. Once their totality obligations
discharge, the io bindings retire.

The kernel boundary (the actual `git update-ref` syscall, the actual
file write) stays in Rust forever. That's the permanent floor from
Spec A's table. Nothing in this spec changes that.

---

## Section 6 — Implications: concrete next ticks

*The order below remains the implementation order. The mathematical
structure these ticks implement is in `kintsugi-formatter.md`: the
formatter's five iteration stages map to wires 1–3 (propose +
measure + elect via wires 2 + 3 in tournament reduction), wire 4
(write-back acceptance), wire 5 (the Banach contraction monitor),
wire 6–7 (gestalt update + proposal-space adjustment), wire 8 (the
outer loop). The eight wires below are eight aspects of one
contraction map.*

Ordered. Each tick is one wire or one piece of one wire. Each cites
which existing spec carries the contract.

1. **Close `dark_count(ast) -> u64` body in `boot/std/epistemologic/property/total_classification.mirror`.**
   Spec D declares the property; the body walks the AST and counts.
   ~10 lines, sub-Turing, total by construction. The first observable
   loss surface lands.

2. **Add `--shatter N` to `cmd_kintsugi` in `bootstrap/src/main.rs`** + **declare `boot/std/mirror/runtime/gen_prism/kintsugi_reduction.mirror`** as a concrete gen_prism whose tick records `(dark_before, dark_after, loss_delta)`. Per Spec E's gen_prism primitive. No model invocation yet — the tick body is the identity transform plus the observation write. **The loop is observable here.**

3. **Declare `type au = ai`, `type conductivity`, `action infer`, `action conduct`** in `boot/std/fate.mirror`. Per Spec C, implications 1-3. Pure grammar; bodies `\`.

4. **Declare `type tensor = matrix(dimension, duality, f64)`** in `boot/std/hash/coincidence.mirror`. Per Spec C, implication 4. Pure grammar.

5. **Declare `type observation` and `type eigenboard`** in `boot/std/cogito.mirror`. Wire-up trivial bodies; strategy returns the literal `elite(1).beam(8).halving(3)` regardless of input. The pipeline is observable; adaptation comes later.

6. **Retype the five model actions to return `imperfect(au, no_proposal, loss)`** in `boot/std/ai/*.mirror`. Per Spec C, implication 3 (extended to the per-model files). Bodies stay `\`.

7. **Wire kintsugi → Fate (Abyss-only baseline).** Inside `cmd_kintsugi`, on encountering a Dark child, call `@fate.infer(dark_oid)` which routes to Abyss only (skip the tournament). Abyss returns `light(empty_au, 0.0)`. The Rust path is alive; nothing structurally lands yet. **Fate is in the loop here.**

8. **Declare `splice(parent_ast, hole_oid, replacement_ast) -> ast` in `boot/std/kintsugi.mirror`.** Body `\`. The structural operation exists in grammar; it is not yet invoked.

9. **Stage non-empty au returns to `refs/fate/<hole_oid>`** via `@io.git_update_ref`. The diff/review surface (future tick) reads from these refs.

10. **Wire the remaining four models + tournament reduction.** Retype + trivial bodies; tournament fan-out + constant conductivity stub. **The tournament is in the loop here.** From this point, subsequent ticks close one model body at a time, reviewed via the diff surface.

11. *(future tick)* **Implement `mirror kintsugi --review <file>`.** The diff/review surface from Section 3. Reads from `refs/fate/*`, invokes `@nl/english.summarize` per proposal, accepts via `splice`, rejects via a refs/fate "rejected" record.

12. *(future tick)* **Close `@nl/english.summarize(proposal: au, context: oid) -> nl`.** Composes the existing five-operation @nl/english surface. The structural summary describes the AST shape, not the bytes.

Ticks 1-10 are the wiring. Ticks 11-12 are the future-tick diff
surface that Section 3 prescribes.

---

## Section 7 — Out of scope

- The diff/review surface implementation. Section 3 describes the
  format and the UX; this spec does not commit to specific terminal
  rendering or specific Claude Code MCP tool shapes.
- The `@nl/english.summarize` body. Section 3 names the contract;
  the body is its own kintsugi obligation against @nl/english's
  five-operation surface.
- Fate model training. The connectome grammar declares the 450-node
  architecture; how weights get learned (via the tournament's
  evolutionary pressure, via separate training, via some hybrid) is
  Fate's own design.
- The conductivity-tensor reduction policy. Spec C, implication 7
  carries this. This spec treats `@fate.conduct` as a black box that
  returns `none | low | partial(f64) | clear`.
- The eigenboard's structural detail. Spec C names the 5×5 tensor;
  Section 1 of this spec calls out that `type eigenboard` needs a
  declaration; the SHAPE of that declaration (a tensor crystal? a
  state machine? a graph?) is its own design.
- Cross-context au transport. Spec C's relational-entanglement
  principle forbids portable au; this spec takes that as given.
- The gen_prism garbage-collection policy. The kintsugi-reduction
  gen_prism's ancestor chain accumulates one entry per tick; pruning
  is Spec E's domain.
- The bootstrap-side implementation of `mirror serve --review`.
  Section 3 describes the surface; the binary work is its own commit.
- Distributed kintsugi across hosts. The ref convention assumes one
  git repo. Spectral's bus handles multi-host coordination; that
  layer is not this spec's concern.

---

*The wires are declared.*
*The bodies are `\`.*
*The first tick is the asymmetric one.*
*After it runs, the asymmetry shrinks.*
*Kintsugi closes the wires that wire kintsugi.*
*The loop becomes observable when the loss surface lands.*
*Fate enters the loop when the dispatcher finds the dark.*
*The tournament fans out when the candidates carry au.*
*The diff surface is the face the loop wears for the user.*
*The cracks fill with gold; the gold conducts; the wire carries.*

Apache-2.0.
