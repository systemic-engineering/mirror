# `@kintsugi/fracture` — confidence + `@scene` dispatch

*2026-05-26. Mara. Proposal — not implementation.*

Status: **Yellow** (recognition crystallized; substrate primitives partially
in place; depends on `@scene` substrate landing per
`docs/insights/2026-05-26-scene-as-substrate-primitive-for-multi-actor-
interaction.md`; no grammar changes yet)

Depends on:
- `boot/std/kintsugi/fracture.mirror` (commit `ca4a9e7`) — the
  `@kintsugi/fracture` glass; the closure-operator type; the two laws.
- `boot/std/kintsugi/fracture/generic-brackets.mirror` (commit `a633c17`)
  — the first concrete fracture; confidence = 1.0 by Alex's substrate-pull
  test.
- `docs/insights/2026-05-26-kintsugi-as-credo-and-formatter-unified.md`
  — the FP collapse: detection IS transformation; one function with two
  laws.
- `docs/insights/2026-05-26-scene-as-substrate-primitive-for-multi-
  actor-interaction.md` — the `@scene` glass; the three load-bearing
  properties (consent, exit, endpoint); the alignment reframe.
- `docs/insights/2026-05-26-glass-wall-and-cross-wall-kintsugi.md` —
  the kintsugi-as-Ricci-flow framing; the four altitudes.

Unblocks (deferred per LRM until consumers surface):
- `@kintsugi/fracture/skills-to-scenes` — migrating skill-shaped
  declarations to scene-shaped (confidence < 1.0).
- An interactive `mirror kintsugi --propose` workflow surfacing
  fracture proposals as scenes the curator enters.
- Cross-curator fracture catalogs (different curators publish
  different confidence assignments for the same fracture function).
- Probability typing in the substrate (`@epistemologic/probability` or
  similar) — the type-level home for `confidence` once it's needed
  beyond this composition.

---

## 1. The recognition

A fracture rule lives on a confidence axis. At `confidence = 1.0`, the
rule's transformation is autonomously applied — the substrate's Ricci
flow runs unsupervised, the gold fills the wound, the OID updates, the
corpus stays canonical. At `confidence < 1.0`, autonomous application
would be coercive: the rule's pull is real but a curator's judgment is
load-bearing on whether THIS instance should heal.

**The composition Alex named** (2026-05-26, paraphrased by Reed):
kintsugi formatting has confidence levels. `confidence = 1.0` →
kintsugi applies the fracture autonomously. `confidence < 1.0` →
kintsugi enters a `@scene` with the curator. The scene's structural
properties (consent, exit, endpoint) carry the human-in-the-loop
discipline.

**Why this matters structurally:** formatter mode (auto-apply) and
linter mode (interactive report) collapse into one mechanism
parameterized by confidence. Per
`2026-05-26-kintsugi-as-credo-and-formatter-unified.md`, mirror already
unified credo + format into a single fracture mechanism via the FP
collapse (detection IS transformation). This proposal extends the
collapse one altitude further: **auto-apply IS scene-entry-at-
confidence-1.0**. The interactive linter is the same mechanism at
lower confidence. One substrate; two visible behaviors; the difference
lives in a single field.

The recognition aligns with the `@scene` insight's reframe: the
industry treats "format" and "lint" as separate tools because the
underlying mechanism is single-actor. Mirror's substrate is multi-
actor by `@scene`; once we have scenes, the bifurcation disappears
structurally.

---

## 2. The augmented type shape

Reed's sketch as starting point:

```mirror
type fracture_with_confidence = {
  flow: fracture,
  confidence: zoom(probability, fracture),    # 0.0 - 1.0
}

apply(f: fracture_with_confidence, corpus: [ast]) -> [ast] {
  f.confidence
  |> beam {
       1.0 => f.flow |> apply_all(corpus)
       _   => scene @fracture_proposal {
                participants: [@peer/curator, @peer/kintsugi]
                setting: corpus
                invariants: [the_fracture_is_idempotent, consent_of_curator]
                close(verdict) -> crystal { \ }
              } |> enter
     }
}
```

Refinements against repo state:

### 2.1 `fracture` stays the closure operator; `confidence` lives outside

The existing `boot/std/kintsugi/fracture.mirror` declares
`type fracture = ast -> ast` with two laws (idempotent,
canonical_at_fixpoint). The proposal keeps this untouched — a fracture
is still a function with algebraic laws. `confidence` is a *property
of a rule's pull*, not of the closure operator itself. Wrapping
rather than mutating preserves the substrate-pull discipline: a
fracture-as-function composes with everything else that takes
fractures (`apply`, `idempotent`, `canonical_at_fixpoint`); a
fracture-with-confidence is the *adopted* form that carries dispatch
policy.

### 2.2 The wrapping type

```mirror
grammar @kintsugi/fracture {
  # existing — unchanged
  type fracture = ast -> ast
  property idempotent(f: fracture) -> verdict { \ }
  property canonical_at_fixpoint(f: fracture) -> verdict { \ }
  apply(f: fracture, corpus: [ast]) -> [ast] { \ }

  # NEW — confidence-aware adopted form
  type adopted_fracture = {
    flow:       fracture,
    confidence: probability,
    curator:    zoom(oid, peer),
  }

  # NEW — confidence-aware dispatch
  apply_adopted(f: adopted_fracture, corpus: [ast]) -> imperfect([ast]) { \ }
}
```

Four notes on the refinement:

1. **`probability` over `zoom(probability, fracture)`.** Reed's sketch
   typed `confidence` as `zoom(probability, fracture)` — a value at
   the type-layer application of `probability` to `fracture`. The
   simpler shape `probability` types confidence as a bare value in
   [0.0, 1.0]; the `zoom(probability, fracture)` form is correct only
   if we want each fracture's confidence to be its own subtype of
   probability. We probably don't — confidence is contextual, not
   intrinsic; see §5 open call 1.

2. **`curator` is a typed field, not an implicit scene participant.**
   Naming the curator at the adopted-fracture altitude makes the
   responsibility legible: who decided this confidence? Who would
   the scene seat at the table? `zoom(oid, peer)` aligns with
   `boot/std/spectral/portal.mirror`'s field-typing pattern.

3. **`apply_adopted` returns `imperfect` not `[ast]`.** The scene path
   can fail (curator declines; scene times out; consent withdraws);
   the auto-apply path can fail (the corpus violates a fracture
   invariant). Imperfect is honest; bare `[ast]` would be lying about
   the scene's exit guarantee. Aligns with `apply`'s upstream contract
   in `boot/std/kintsugi/fracture.mirror`.

4. **The `apply` action stays in place.** Existing fracture
   instances (e.g. `@kintsugi/fracture/generic-brackets`) keep
   working — they declare their `flow` and the two laws; the substrate
   defaults to `confidence = 1.0` via the adopted-form constructor.
   No breaking change at v0.

### 2.3 The dispatch shape

The `beam { ... }` pipe-in pattern aligns with the @scene insight's
three-form recognition (move, scene, render-target) — the dispatch
selects a render-target for the action. Concretely:

```mirror
apply_adopted(f: adopted_fracture, corpus: [ast]) -> imperfect([ast]) {
  f.confidence
  |> beam {
       1.0 => apply(f.flow, corpus) |> ok
       _   => scene @fracture_proposal {
                participants: [f.curator, @peer/kintsugi]
                setting:      corpus
                invariants:   [
                  idempotent(f.flow),
                  canonical_at_fixpoint(f.flow),
                  consent_of_all_participants,
                ]
                close(verdict: scene_outcome) -> crystal { \ }
              } |> enter
     }
}
```

The `1.0` arm is total: the fracture has already satisfied its two
laws (verified via `requires`); `apply` runs unsupervised; the
corpus updates to fixpoint; `ok([ast])` returns. The `_` arm enters
the scene; the scene's `enter` returns `imperfect([ast])` natively
(it can fail to close, or close with `decline` rather than `accept`).

The scene's invariants compose:

- `idempotent(f.flow)` — the fracture's own law, restated at the
  scene altitude so the curator sees what's being committed to.
- `canonical_at_fixpoint(f.flow)` — likewise.
- `consent_of_all_participants` — `@scene`'s structural property; the
  scene cannot enter without all participants (including the AI
  peer) consenting.

The `close(verdict)` returns a crystal — the content-addressed
artifact recording what the scene was and what it produced. Per
`@scene`'s `scene_outcome` open call (insight §5.3), the crystal
includes which exit each participant took, which invariants held,
and the produced artifacts (the rewritten corpus, or the unchanged
corpus + a declined-verdict record).

---

## 3. What this gives `@kintsugi/fracture/generic-brackets` retroactively

The first concrete fracture, `@kintsugi/fracture/generic-brackets`,
rewrites foreign angle-bracket parametrics `<T>` to mirror's canonical
parens `(T)`. Per Alex's substrate-pull test
(`2026-05-26-kintsugi-as-credo-and-formatter-unified.md` §"The
substrate-pull test"): "if a 15-year engineer can't articulate the
distinction, there isn't one." The `<T>` vs `(T)` case fails the test
— no semantic reason in mirror; pure cargo-culted C++ parser accident.
Confidence = 1.0.

Under this proposal:

```mirror
grammar @kintsugi/fracture/generic-brackets {
  in @kintsugi/fracture

  flow(ast) -> ast { \ }
  requires idempotent(flow)
  requires canonical_at_fixpoint(flow)

  # NEW — adopted-form confidence (defaulted at 1.0; curator @peer/kintsugi)
  confidence = 1.0
  curator    = @peer/kintsugi
}
```

The rule fires uniformly. The substrate applies it autonomously. No
scene needed. The curator is the substrate itself — `@peer/kintsugi`
as the agent that owns the kintsugi-formatter responsibility (see §5
open call 2). Every `<T>` in every mirror corpus heals back to `(T)`
on next compile, regardless of who wrote it (human, LLM, future agent).

The `6e32af3` commit (`♻️ boot: apply @kintsugi/fracture/generic-
brackets across boot tree`) IS the substrate already doing this; the
proposal's job is to make the dispatch structural rather than ad-hoc.

---

## 4. What this enables for future fractures

The `@scene` insight's §5 names a deferred task:

> **#94 (deferred per LRM)** — fracture rule for skill-shaped
> declarations → scene-shaped (`@kintsugi/fracture/skills-to-scenes`);
> accumulates when the substrate has both forms and we want to migrate
> the corpus.

This is the worked example of `confidence < 1.0`:

```mirror
grammar @kintsugi/fracture/skills-to-scenes {
  in @kintsugi/fracture

  flow(ast) -> ast { \ }   # the rewrite — skill-shaped → scene-shaped
  requires idempotent(flow)
  requires canonical_at_fixpoint(flow)

  confidence = 0.6
  curator    = @peer/curator
}
```

Why `confidence < 1.0` here:

- The fracture's TARGET is correct (skills ARE scenes-of-arity-one;
  every skill-shaped declaration has a scene-shaped equivalent).
- But the *appropriate scene shape* for any given skill requires
  human judgment: who are the participants (just the agent? agent +
  user? agent + multiple users?); what consent property applies;
  what the closing crystal should record. The substrate cannot
  guess these from the skill declaration alone.
- Auto-applying at confidence = 1.0 would coerce decisions the
  curator should make. The right behavior is: surface the proposal,
  enter the scene, let the curator shape the scene-shaped output.

With confidence = 0.6, the dispatch routes through `@scene
@fracture_proposal`. The scene's participants are the substrate's
kintsugi peer and the curator (probably the human author of the
skill). The scene's setting is the corpus. The scene's invariants
include the fracture's two laws PLUS consent. The scene closes when
the curator accepts (scene-shaped output replaces the skill
declaration) or declines (no change). Either close produces a crystal
recording the outcome.

**Other future fractures with `confidence < 1.0`:**

- `@kintsugi/fracture/sigil-rename` — when AGENTS.md's sigil-naming
  rule changes (e.g. someone renames `~mq` to `~mirror_query`), the
  fracture knows the rewrite but the curator decides per-corpus
  whether to apply at this time.
- `@kintsugi/fracture/match-to-select` — Spec B has two close
  forms (match and select); when one supersedes the other, the
  migration confidence is rule-clear but per-corpus contextual.
- `@kintsugi/fracture/extract-property` — when a repeated
  invariant emerges as a candidate `@epistemologic/property/*`,
  the fracture proposes; the curator decides whether the
  extraction is ripe.

Each future fracture rides this same shape. The substrate accumulates
rules; each rule declares its own confidence; the dispatch is
uniform.

---

## 5. Open design calls

This is a proposal, not a commitment. Six open calls flagged for the
implementation tick:

### 5.1 Confidence: static rule property vs dynamic per-instance property

The proposal types `confidence` as a static field on the
`adopted_fracture` record — set once, in the grammar that declares
the rule. But confidence is plausibly contextual: a fracture might
have `confidence = 1.0` in one corpus and `confidence = 0.6` in
another (different curator's risk tolerance; different corpus's
maturity). The choice:

- **Static** (current proposal): confidence is part of the rule's
  declaration. Simple. The grammar declares its level of pull.
  Curators who disagree fork the fracture into their own namespace.
- **Dynamic**: confidence is computed per-instance from the corpus +
  curator + gestalt. More expressive. Harder to verify; the static
  laws (idempotent, canonical_at_fixpoint) no longer pin behavior
  without also pinning confidence.
- **Curator-overridable**: static default; the curator's gestalt can
  shift it via per-corpus config. Likely the right middle.

Deferred to the implementation tick; the proposal-side commitment is
that confidence IS a typed field somewhere.

### 5.2 The `@peer/curator` identity

The proposal references `@peer/curator` and `@peer/kintsugi` as
participant identities. Neither exists yet. The `@peer` glass
(`docs/specs/peer-glass.md`) types a peer as five-axis identity
gestalt. Open questions:

- Is `@peer/curator` a single identity (the substrate's authorial
  peer) or a per-corpus role (whoever owns the corpus this fracture
  fires in)?
- Is `@peer/kintsugi` the substrate itself (kintsugi-as-peer) or a
  named agent (some specific instance running the formatter)?
- How does the scene's participant list resolve these — is the
  curator looked up from the corpus's metadata, or is it the user
  running `mirror kintsugi`?

Likely shape: `@peer/curator` is a role typed against the corpus's
`@peer` field; `@peer/kintsugi` is the substrate's autopoietic-
formatter peer (one identity per `gen_prism` lineage). But the
composition with `@peer`'s five-axis fixed point needs design work
before the scene can structurally type-check.

### 5.3 Scene invariants compose with fracture laws

The scene's invariants list includes the fracture's two laws
(`idempotent(f.flow)`, `canonical_at_fixpoint(f.flow)`) PLUS the
scene's own properties (`consent_of_all_participants`,
`all_participants_can_exit`, `the_scene_has_an_endpoint`). The
composition is natural — both are `requires`-clause-shaped — but the
@scene insight's §5.2 flags `obligation` as a type we don't have
yet. The fracture's two laws are `property` returns; the scene's
three are `requires` clauses. Are these the same type? Different
types? The proposal punts: name them all as scene invariants and let
the `@scene` substrate's `obligation` type land before pinning the
shape.

### 5.4 Scene-entry sync vs async

The scene's `enter(participant)` returns `imperfect`. Two possible
semantics:

- **Sync**: `apply_adopted` blocks until the scene closes; the
  return is the scene's crystal.
- **Async**: `apply_adopted` returns immediately with a handle
  (probably the scene's content-addressed identity); the substrate
  continues; the scene's crystal lands later in the corpus state.

Probably async (the brief's hint): the substrate continues; the
scene's crystal becomes part of the corpus state once closed. This
aligns with `@scene`'s long-running examples (the bar persists
across many enter/exit cycles per the insight's §2). But sync is
simpler for the v0 implementation. The choice affects what
`apply_adopted`'s return type carries — a final corpus, or a
promise-typed-as-corpus.

Deferred. Likely: implement sync first; refactor to async when the
first long-running fracture-scene surfaces.

### 5.5 Confidence ranges + composition

The proposal uses two values — `1.0` (auto-apply) and `< 1.0`
(scene-dispatch). But probability is continuous. What does
`confidence = 0.95` mean structurally — auto-apply with a logged
note? Auto-apply with a low-priority scene proposal? Scene-dispatch
with a pre-filled curator-recommendation?

Likely shape: a threshold (initially 1.0) above which auto-apply
fires; below the threshold, scene-dispatch. The threshold itself is
per-curator gestalt configuration (a risk-tolerance dial). But this
opens the question of how multiple adopted fractures with different
confidences compose when run together — does the corpus enter ONE
big scene (all proposals at once), or N scenes (one per proposal),
or a meta-scene (the bar) where individual fracture-scenes are inner
scenes? The `@scene` insight's §6 ("Long-running scenes") frames
this as scene-of-scenes; deferred until the implementation tick has
more than one `< 1.0` fracture to test against.

### 5.6 Backward compatibility with bare `fracture`

The existing `boot/std/kintsugi/fracture.mirror` declares `apply(f:
fracture, corpus) -> [ast]` taking a bare fracture. The proposed
`apply_adopted` takes `adopted_fracture`. Do we keep both, or migrate
bare-fracture call sites to adopted-form?

Likely: keep both. `apply` is the low-level primitive (substrate-
internal); `apply_adopted` is the user-facing dispatcher. Substrate
calls into the fracture's grammar can use either form depending on
whether they care about confidence. This preserves the closure-
operator framing at the algebra altitude (fractures compose as
closure operators on the AST lattice, per the FP collapse) while
letting the dispatch altitude carry the confidence-aware behavior.

---

## 6. Provenance

- **Alex 2026-05-26** — the originating compositional recognition
  (paraphrased by Reed): "Kintsugi formatting has confidence levels.
  `confidence = 1.0` → kintsugi applies the fracture autonomously.
  `confidence < 1.0` → kintsugi enters a `@scene` with the curator."
- **Reed 2026-05-26** — the sketch of the composition (the
  `fracture_with_confidence` type, the `beam { 1.0 => ... | _ => scene
  ... }` dispatch shape, the scene's invariant list including the
  fracture's two laws).
- **Loki 2026-05-26** — the originating `@scene` insight that this
  proposal composes against; the three load-bearing properties
  (consent, exit, endpoint); the rendering-is-local deepening.
- **Mara 2026-05-26** — integration into the existing `@kintsugi/
  fracture` substrate; refinement of the wrapping type (`adopted_
  fracture` vs mutating `fracture`); the six open design calls; the
  worked examples (skills-to-scenes, sigil-rename, etc.); recognition
  that auto-apply IS scene-entry-at-confidence-1.0.
- **`@kintsugi/fracture/generic-brackets`** (commit `a633c17`) — the
  first concrete fracture; the proposal's retroactive shape.
- **Elixir Credo + mix format prior art** — the plugin-extensibility
  + per-project-config pattern (via the kintsugi-as-credo-and-
  formatter-unified insight).
- **Theater + family-systems prior art** — the scene-as-substrate-
  primitive prior art (via the @scene insight's cross-domain table).

---

## What this proposal does not commit to

- No grammar changes land here. `boot/std/kintsugi/fracture.mirror`
  is unchanged. The `adopted_fracture` type, the `apply_adopted`
  action, and the `@peer/curator` / `@peer/kintsugi` participants
  are all proposals for a future tick.
- No `@scene` substrate is created. The proposal depends on the
  @scene insight landing as substrate (deferred per LRM in its own
  insight § "Next tasks"); this proposal is what the @scene
  substrate's first non-Silicon-Venue consumer will look like.
- No `probability` type is created. Confidence is a typed value
  in [0.0, 1.0]; whether that type lives in `@epistemologic/
  probability`, `@mirror/number`, or somewhere else is deferred.
- No fracture-catalog discovery mechanism. The substrate's behavior
  when there are multiple adopted fractures applicable to one
  corpus (§5.5) is named as an open call, not designed.

---

*Auto-apply IS scene-entry-at-confidence-1.0.*
*The bifurcation between formatter and linter dissolves at the substrate.*
*One mechanism; one type; one dispatch; two visible behaviors.*

Apache-2.0.
