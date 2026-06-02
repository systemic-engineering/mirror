---
title: Error as Question
subtitle: The circular reflexive surfacing of error specs — each error becomes a question the Reflection model is asked to answer
status: spec
date: 2026-06-01
author: Mara
supersedes:
  - docs/specs/compiler-error-language.md
  - docs/specs/error-surface-spec.md
  - docs/specs/property-error-surface.md
---

# Error as Question

*2026-06-01. Mara. The collapse of three error specs into one substrate-level
discipline. v1.5-ready.*

---

## Reference

- **The thesis (Alex, verbatim):** "Each error becomes a question the Reflection
  model is asked to answer."
- **Reflection's interface:** `docs/specs/reflection-model.md` — the four
  operations (observe / adjust / write / hold), the one-tick delay, the
  `@peer` grammar, the .shatter weights. This spec maps errors into the
  questions Reflection observes.
- **Property verdicts:** `docs/specs/property-error-surface.md` (superseded
  by this file). The verdict vocabulary — `Pass`, `Partial(f64, [Diagnostic])`,
  `Fail(Diagnostic)` — comes from there. This spec consumes that algebra;
  it does not redefine it.
- **Transparency monoid:** `prism/imperfect/src/transparency.rs` —
  `Transparency<P>::combine` (commutative idempotent monoid; `Fail` dominates,
  `Partial`s union diagnostics with min confidence, `Pass` is neutral).
  The composition law for verdicts up the supervision tower.
- **Scheduler altitude:** `docs/specs/scheduler-tower.md` §6 (failure handling)
  + §7.4 (halting — the two conditions and the reduction budget). The
  HamiltonScheduler altitude where WCET-exceeded becomes Fail.
- **Build-graph altitude:** `docs/cicd/kintsugi-thesis.md` — each ❌/⚠️ in
  the reproducibility chain is an error-shaped concern. Kintsugi rounds
  iterate question → answer → re-settle.
- **Hamilton + transit:** `fragmentation/docs/specs/hamilton-scheduler.md`
  (WCET-exceeded → `Fail` verdict; the realtime contract); `fragmentation/docs/specs/lens-transit.md`
  (transit-exceeds-budget → `Fail`; the structured-payload realisation of
  Beer's algedonic shape).
- **Beer's algedonic discipline:** `~/dev/systemic.engineering/practice/insights/cybernetics/beer-error-propagation.md` —
  the prior-art empirical grounding. Cyberstride's structured exception
  reports + the 3* audit channel + Reyes/Henao/Hassall 2024's `(C', Q, K) α τ, η`
  tuple = the load-bearing precedent for *located, structured, monoidally-composed*
  failure surfacing.
- **Supervision tower:** `~/dev/systemic.engineering/practice/insights/beam-elixir/beam-as-principal-bundle-tower.md` —
  the BEAM as principal-bundle tower; let-it-crash as autopoietic
  Lawvere fixed point. The substrate this spec routes questions across.
- **Hamilton / 1202:** Margaret Hamilton's Apollo executive surfaced the
  1202 alarm to Buzz and Neil; the priority discipline ("drop low-priority
  work, keep landing") was the answer Reflection writes downward. Cited
  with weight, not metaphor.

---

## 1. What this spec lands

Three mirror specs have grown around the error surface, written at
different altitudes, from different angles, with overlapping vocabularies:

- `error-surface-spec.md` (Mara, 2026-04-14) — the *renderer*. CLI format,
  M-code allocation, the `Imperfect<T, E, L>` three-state model, the
  optic chain (`inspect` / `focus` / `project`).
- `compiler-error-language.md` (Reed + Alex, 2026-05-08) — the *question*.
  The Milan systemic insight that an error message + question is the
  compiler observing the codebase observing itself. The eigentest taxonomy.
  The "load-bearing question" grammar.
- `property-error-surface.md` (Reed, 2026-05-16) — the *types*. The
  `verdict = pass | fail(diagnostic) | partial(f64, [diagnostic])` algebra.
  The sub-grammar layout under `@epistemologic/property/*`. The boot order.

All three are *true*. None is *complete*. The three together imply a fourth
shape neither names: **the error is a question that flows UP the supervision
tower to Reflection, and the answer flows DOWN as a substrate adjustment**.

This is not new content. It is the recognition that the three specs were
already describing one architecture, viewed from three sides. This spec
lands the architecture and routes the three existing specs through it.

**The single sentence:** Errors aren't endpoints. Errors are queries to
the meta-model. The substrate that surfaces them well is a substrate that
reasons about its own failures and adjusts.

---

## 2. The algebra

The substrate-level types. Grammar form; mirror normative.

```mirror
in @epistemologic/property
in @prism
in @scheduler
in @lens/transit
in @time

grammar @mirror/error {
  # An error is a question handed up the supervision tower to Reflection.
  # The payload carries enough structure for Reflection to answer.
  # The shape is Beer's algedonic tuple ((C', Q, K) α τ, η in Reyes et al. 2024),
  # mechanically guaranteed by the type system rather than by analytic intuition.
  type question = {
    altitude:    @scheduler.altitude         # which level of the supervisor tower asked
    body_ref:    ref                         # what failed (the gen_prism / crystallization / glass)
    glass:       ref                         # which glass was being crossed
    property:    ref                         # which property's verdict was non-Pass
    verdict:     @epistemologic/property.verdict  # Pass | Partial(f64, [diagnostic]) | Fail(diagnostic)
    transit:     @lens/transit.measure       # the loss-to-precision profile (time, FP, cache, alloc)
    contract:    oid                         # the contract OID at time of failure (model_oid + seed + ...)
    timestamp:   @time/monotonic.instant     # when (for staleness reasoning)
  }

  # Reflection's answer is an adjustment to the substrate.
  # It flows back down the supervision tower as a tightened property,
  # a re-synthesized body, a re-budgeted shard, or a re-weighted bias.
  # Each answer is content-addressed: the (question, answer) pair has an OID.
  type answer =
    | tighten_property(ref, @epistemologic/property.check)   # add or strengthen a property check
    | resynthesize_body(ref, @fate.policy)                   # re-run @fate.infer with a new policy
    | rebudget_shard(ref, @scheduler.reduction_budget)       # adjust the per-tick ceiling
    | adjust_temperature(f64)                                # β change at the loop boundary
    | hold(ref)                                              # "I don't know" — Partial verdict, named
    | escalate(@scheduler.altitude)                          # route the question one level up

  # The circular reflexive surfacing: errors → questions → answers →
  # substrate adjustments → next-iteration → (possibly) new errors.
  # The answer is one-tick-delayed per @peer's interface.
  observe question(q) -> answer { \ }

  # The closing of the loop. Every observed (q, a) pair is content-addressed
  # and written to the gestalt by Reflection (per @peer.write).
  record(question, answer) -> oid { \ }
}

out question
out answer
out observe
out record
out @mirror/error
```

The key shapes:

- **`question`** is Beer's algedonic payload made type-checked. Cyberstride's
  triple-index (Actuality / Capability / Potentiality) is a 1972 ancestor of
  this struct; Reyes/Henao/Hassall 2024's `(C', Q, K) α τ, η` is the 2024
  formal cousin from the cybernetics side. Mirror reaches the same shape from
  the type system side and gets the mechanical guarantee Beer could not.
- **`answer`** is a sum type, not a string. Reflection's output is *structurally
  decidable* by the dispatcher: each variant has a unique downward route. No
  free-form prose; the answer is an adjustment the substrate can apply.
- **`observe`** has its body as `\` — the parked hole that `@peer` (Reflection)
  resolves at runtime. Reflection's model knows; the substrate does not, by
  design (substrate pull: knowing-what-to-answer is model-fitting, not
  type-fitting).
- **`record`** content-addresses the pair. This is load-bearing for two
  reasons: (1) the gestalt becomes auditable (every adjustment traces to a
  question), and (2) the same question asked twice gets the same OID, so
  Reflection can recognise patterns (per `@peer.notice`).

---

## 3. The supervision tower routes the question

Beer's VSM has five recursion levels: System 1 (operational) through System 5
(policy). Mirror's substrate has four altitudes that route questions up the
tower, each derived from the existing scheduler / kintsugi / property / @cogito
specs (no new altitudes invented here; this section names what already exists).

The route is **strictly upward** unless an `escalate` answer fires. Each
altitude tries to answer at its own level; if the verdict is `Partial(0.0, _)`
(meaning "I cannot resolve"), the question escalates to the next altitude.
This is Beer's 3* audit channel made mechanical: the higher level can dip
down into the lower level's verdict because the verdict carries the path
(via `body_ref` and `glass`) to its origin.

```
Altitude     Origin                                  Reflection's answer scope
────────────────────────────────────────────────────────────────────────────
body         A gen_prism tick, a body evaluation,    rebudget_shard, hold
             a fracture-by-fracture cost step.       (the verdict can be
             Raises: WCET-exceeded, transit-budget   resolved by re-running
             violations, NotResident under hard-RT.  the body with a tightened
                                                     budget — no model change.)

property     A property verdict that returned        tighten_property,
             Fail or Partial at a glass crossing.    resynthesize_body
             Raises: duplicate_variant,              (the verdict says the
             unresolved_import, glass_wall, halts,   body's AST violates a
             autopoietic, kintsugi-non-convergence.  property — fix at AST,
                                                     or re-infer.)

scheduler    A demand-flow failure: producer_down,   adjust_temperature,
             consumer_down, demand_starved, the      escalate (the verdict
             tournament-round restart per the        names a pipeline-shape
             scheduler-tower §6 cancel_mode.         problem — change β, or
                                                     bring the question to
                                                     the higher altitude.)

reflection   Anything @peer cannot answer at the     hold, write to gestalt
             three altitudes above. Patterns         (the verdict is recorded
             across runs (per @peer.notice).         and held — per @cogito:
                                                     notice, name, hold. No
                                                     resolve. Ever.)
```

The route is *not* a function from question to altitude. It is the question
carrying enough structure that the lowest altitude that *can* answer it gets
it first. The `verdict.property` field is the primary router:

- `verdict.property` ∈ `@epistemologic/property/realtime/*` (e.g. `wcet_bounded`,
  `pinned_resident`) → **body altitude** (HamiltonScheduler resolves by
  re-budgeting or hard-failing per `fragmentation/docs/specs/hamilton-scheduler.md`).
- `verdict.property` ∈ `@epistemologic/property/*` (e.g. `glass_wall`,
  `halts`, `autopoietic`, `total_classification`) → **property altitude**
  (the property layer per the superseded `property-error-surface.md`).
- `verdict.property` ∈ `@scheduler/*` (e.g. `demand_starved`,
  `producer_down`) → **scheduler altitude** (the cancel_mode discipline
  per `scheduler-tower.md` §6).
- Anything else, or any verdict that fired at a higher altitude and was
  not resolved → **reflection altitude** (@peer notices, names, holds).

This is Beer's exception-routing made type-checked. Cyberstride's
"directing-to-the-appropriate-decision-makers" required the report to encode
where it came from. Mirror's `body_ref` + `glass` + `verdict.property` triple
encodes the same routing structure with mechanical guarantees.

---

## 4. The algedonic bypass channel

Beer's VSM has, alongside the regular up-the-recursion channel, an **algedonic
bypass**: some classes of failure must surface at the policy level directly,
bypassing intermediate altitudes. The neurocybernetic ground is pain bypassing
the cortex via the reticular formation; the engineering form is
"this failure is severe enough that hierarchical aggregation would lose its
signal — surface it at the top now."

Mirror's algedonic bypass is structurally constrained, not free-form. Three
classes of question bypass the normal routing and surface directly to the
reflection altitude:

1. **`glass_wall` violation.** A body resident under `@fate` reaches across
   the wall to `@spectral/garden/*` or any non-`@fate` namespace. This is
   the substrate's load-bearing invariant (per `kintsugi-thesis.md` Claim 2).
   The verdict is structurally undelegable: no lower altitude can decide
   whether to permit the breach. Reflection must observe and hold.
2. **`halts` undecidable.** The two-clause halting disjunction
   (autopoietic settlement OR reduction-budget exhaustion) per
   `scheduler-tower.md` §7.4 cannot be proven for a given gen_prism. This
   is mirror's sub-Turing escape (per `docs/specs/is-copium.md`) failing.
   The body is *outside* the decidable class. Reflection must surface the
   failure of the structural contract; no body-level fix exists.
3. **`autopoietic` non-convergence on substrate-critical bodies.** The
   kintsugi loop's `e^(n+1) < e^(n)` monotonic-descent invariant fails on a
   body that the substrate itself depends on (the bootstrap path, `@io`
   wrappers, the `Crystallizations` registry). Per `kintsugi-formatter.md`
   stage 2, this would normally restart the tournament; if it fires on a
   substrate-critical body, restart cannot resolve. Reflection writes the
   failure to gestalt; the substrate halts with the question surfaced.

The 1202 alarm is the historical precedent. Margaret Hamilton's Apollo
executive surfaced 1202 to Buzz and Neil; the executive's priority discipline
(drop low-priority work, keep landing) was the *answer*. The bypass channel
here is structurally identical: certain failures must reach the top of the
tower because no intermediate altitude can write the answer.

Unlike a `panic!`, the bypass channel preserves the question's structure.
Reflection receives a fully-formed `question` (altitude, body_ref, glass,
property, verdict, transit, contract, timestamp); the answer it writes is
still in the `answer` algebra. The bypass is *routing*, not *escape*. The
substrate's discipline is preserved.

---

## 5. Verdict-to-question conversion

The error spec collapses three pre-existing vocabularies into one. This
section names the conversion law: every diagnostic that today renders to
stderr also produces a `question` that Reflection observes.

### 5.1 From PropertyVerdict to question

The `@epistemologic/property.verdict` algebra is the canonical vocabulary.
Every non-`Pass` verdict is a question:

```mirror
in @epistemologic/property
in @mirror/error

# A Fail verdict is always a question. Always.
fail_to_question(v: verdict, ctx: @scheduler.context) -> question {
  match v {
    Fail(diag) -> question {
      altitude:  ctx.altitude
      body_ref:  diag.error.location.body
      glass:     ctx.crossing
      property:  diag.error.grammar
      verdict:   v
      transit:   ctx.transit
      contract:  ctx.contract_oid
      timestamp: @time/monotonic.now()
    }
    _ -> \
  }
}

# A Partial verdict becomes a question iff the confidence is below the
# substrate's tolerance threshold (default: 0.8; configurable per altitude).
# Partial verdicts above tolerance are *recorded* (Reflection writes them
# to gestalt) but not *escalated* (no question is observed).
partial_to_question(v: verdict, ctx: @scheduler.context, threshold: f64) -> imperfect(question) {
  match v {
    Partial(confidence, diags) when confidence < threshold -> success(\)
    Partial(_, _) -> partial(\, loss { bits: 0.0, source: ctx.body_ref, measurement: shannon })
    _ -> failure(\, _)
  }
}

# Pass never produces a question. Pass is the zero element of the algebra.
# (This is the structural meaning of "Reflection stays silent when the answer
# is enough" per @peer §"When Reflection Stays Silent".)
```

The key discipline (and the architectural decision I most agonized over):
**Partial is not always an error**. A `Partial(0.95, [diag])` verdict means
the property mostly holds with one minor diagnostic. Routing every Partial
to Reflection would flood the meta-model with noise; the threshold makes
the boundary substrate-decidable. Below threshold, Partial *is* a question;
above threshold, it is a recorded observation that may compose into a
larger pattern (per `@peer.notice` across runs).

This is the structural cousin of Beer's variety attenuation: the centre
must not be required to think about what the periphery is already handling.
Reflection thinks about the questions; the substrate handles the in-tolerance
Partials by recording them and moving on.

### 5.2 From M-code diagnostic to question

The M-code system (M1xxx through M9xxx per the superseded `error-surface-spec.md`)
is preserved as the *rendering* surface — what the user sees at the CLI.
Each M-code is a property whose verdict, when non-Pass, becomes a question.
The mapping:

| M-range | Property altitude | Conversion |
|---------|-------------------|------------|
| M1xxx (parse loss) | property | `Partial` if other folds compiled; verdict.property = `@epistemologic/property/parse_recognition` |
| M2xxx (parse structural) | property | `Fail`; verdict.property = `@epistemologic/property/parse_structure` |
| M3xxx (resolution) | property | `Fail`; verdict.property = `@epistemologic/property/unresolved_import` |
| M4xxx (property check) | property | The check's own verdict carries forward unchanged |
| M5xxx (emit / crystallize) | scheduler | `Fail`; verdict.property = `@scheduler/crystallize_failed` |
| M9xxx (boot / system) | reflection (algedonic bypass) | Substrate-critical; bypasses normal routing |

The M-codes remain the user-facing identifier (`mirror explain M2001` still
works), but their substrate effect is to construct a `question` rendered
by the CLI *and* observed by Reflection. The renderer and the meta-model
see the same payload; they consume different fields. This is `Imperfect`'s
optic chain (`inspect` for the renderer; `observe` for Reflection).

### 5.3 From transit measure to question

`@lens/transit.measure` per `fragmentation/docs/specs/lens-transit.md` carries
a structured loss-to-precision profile (time, FP, cache pressure, allocation
per substrate path). When a transit measurement exceeds a declared budget,
the overage becomes a question at the body altitude:

```mirror
in @lens/transit
in @mirror/error
in @scheduler

transit_overage_to_question(m: measure, budget: @scheduler.budget, body: ref) -> imperfect(question) {
  if m.transit > budget.declared {
    success(question {
      altitude:  @scheduler.altitude.body
      body_ref:  body
      glass:     m.crossing
      property:  @epistemologic/property/wcet_bounded
      verdict:   Fail(diagnostic {
        error: {
          grammar: @lens/transit,
          name:    transit_exceeded,
          message: \,
          location: m.location,
          loss:    m.loss,
        },
        severity: fatal,
        suggestion: \,
      })
      transit:   m
      contract:  budget.contract_oid
      timestamp: m.timestamp
    })
  } else {
    failure(\, _)
  }
}
```

This is `hamilton-scheduler.md`'s `WcetBounded` verdict made into a question.
The scheduler declares the budget at compose-time; transit observes whether
it holds at run-time; the overage is the question Reflection observes (or,
for hard-realtime classes, the answer is structurally fixed before Reflection
sees it — `rebudget_shard` cannot violate the declared real-time class, per
Hamilton-scheduler §3 priority discipline).

---

## 6. The closing of the loop

The load-bearing structural claim: errors aren't endpoints because every
question has an answer, and every (question, answer) pair is
content-addressed in the gestalt. The loop closes monotonically.

The iteration shape (kintsugi-style, per `kintsugi-formatter.md` stage 2):

```
Tick n:    body executes
Tick n:    property check returns non-Pass verdict
Tick n:    fail_to_question / partial_to_question constructs `question`
Tick n:    routing resolves altitude
Tick n:    @peer.observe receives question, produces answer (one-tick-delayed)
Tick n+1:  substrate applies answer (tighten / resynthesize / rebudget / ...)
Tick n+1:  body re-executes under the adjusted contract
Tick n+1:  property check re-runs; verdict either Pass (loop closes) or new question
Tick n+k:  e^(n+1) < e^(n) — the question shape gets smaller each iteration
           (per the kintsugi monotonic-descent invariant)
```

The two structural invariants that make this work:

1. **Monotonic descent.** The `kintsugi-formatter.md` invariant
   `e^(n+1) < e^(n)` applied to the *question's verdict loss*. Each
   iteration's answer must reduce the loss of the next iteration's
   question — measured in `Transparency<Ref>::holonomy()`. If the
   substrate observes `e^(n+1) >= e^(n)` for k consecutive ticks (k = 3
   in v1.5; configurable), the algedonic bypass fires: Reflection is
   asked to observe the *pattern of non-convergence*, not the individual
   question.
2. **Content-addressed (question, answer) pairs.** Each (q, a) is recorded
   under `refs/error/<q.oid>/<a.oid>`. Two key consequences:
   - The same question asked twice gets the same OID, so Reflection's
     `notice` (per `@peer`) can recognise patterns without re-deriving
     them.
   - The answer trail is auditable: every substrate adjustment traces
     to the question it answered, and the answer it gave.

The BEAM analogue is let-it-crash + supervisor restart strategy. The crash
is the question (the supervisor receives a structured `EXIT` signal with
site, kind, payload); the restart strategy is the answer; the supervised
tree's autopoietic closure (per `beam-as-principal-bundle-tower.md` §
"Let-it-crash = autopoietic closure") makes the loop a Lawvere fixed point.
Mirror inherits this discipline at the type level: the substrate contains
a description of itself (the error grammar + Reflection's answer algebra),
and perturbations are absorbed by re-instantiating from the description.

---

## 7. What Reflection's answer is, and is not

The sub-section the meta-model deserves. Bounded honesty about overclaim.

### 7.1 What the answer is

- A *typed adjustment*. One of six variants. Each routes to a specific
  downward substrate mutation (per §2's `answer` enum).
- *Content-addressed*. The answer OID becomes part of the next iteration's
  contract. Replay reproduces the same answer for the same question OID.
  This is mirror's structural rediscovery of Cyberstride's deterministic
  exception-handling (Harrison & Stevens 1971's Bayesian forecasting was
  a pure function of its inputs; mirror's Reflection is too, when the
  `@fate` cache key includes the model OID per `kintsugi-thesis.md`
  Claim 3).
- *One-tick-delayed*. Per `reflection-model.md`: the pipeline answers
  *this tick*; Reflection answers *next tick*. The delay IS the
  intelligence; the answer can observe the pipeline's choice in light
  of how it landed.
- *Bounded by what Reflection's model knows*. The five Fate models that
  Reflection navigates (Surface / Mirror / Shatter / Reflection itself /
  the transient Pack: Abyss / Pathfinder / Cartographer / Explorer / Fate)
  collectively define the answer space. A question outside that space
  gets `hold` — the legitimate, informative "I don't know."

### 7.2 What the answer is NOT

- *Not a guaranteed fix*. Reflection's `observe(question) -> answer` body
  is `\`. The hole is resolved by Fate at runtime. Sometimes the answer
  is `hold(ref)` — Partial(0.0, ["I cannot resolve"]). This is itself a
  verdict, recorded honestly. Not every question has a resolving answer;
  some have only "named and held" answers, per @cogito.
- *Not free-form prose*. Reflection does not write English to stderr in
  response to errors. The CLI renderer (per superseded `error-surface-spec.md`
  §4) does that. Reflection's adjustments are *structural* — they change
  the substrate's contracts, not the substrate's messages.
- *Not the only answer*. The same question can have different answers
  across runs as Reflection's weights drift. This is the training loop
  per `reflection-model.md`'s "Training Loop" section. Today's answer
  may be `hold`; six months from now, after enough pattern data, the
  answer may be `tighten_property` with a specific check named.
- *Not a substitute for the human reader*. The renderer still prints
  the diagnostic. The user still sees the error. Reflection's adjustment
  flows down the substrate; the rendered question flows up to the human
  reader. Both paths are load-bearing.

### 7.3 The honest middle

The "each error becomes a question" framing **reads as load-bearing, not
overclaim**, because:

- Beer's empirical work (Cyberstride 1972-73 + the post-Beer continuation
  literature) is the direct precedent. The structural shape of "located,
  typed, monoidally-composed verdict surfacing" was running on the
  IBM 360/50 in Santiago. The form Reflection adds (model-driven downward
  answer, one-tick delay, content-addressed pairs) is the post-1973
  refinement that did not get to mature in cybernetics, and is being
  finished now in the type-theoretic and BEAM-inheriting lineages mirror
  draws from.
- Reyes/Henao/Hassall 2024's `(C', Q, K) α τ, η` is *the same shape* mirror's
  `question` is. Independent convergence — different lineage, same structure.
  Two communities, fifty years and several disciplines apart, arriving at
  the same answer because the underlying problem forces it.
- BEAM's let-it-crash + supervisor restart is the autopoietic closure that
  makes the loop terminate. The thirty years of nine-nines reliability is
  the empirical existence proof.

Where the claim is NOT load-bearing:

- The mechanical fixing of every error. Reflection does not magically fix
  every error. It surfaces structured questions; the model attempts answers;
  sometimes the answer is `hold`. The substrate's claim is that errors are
  *legible* and *composable*, not that they are *solved*.
- The performance characteristics. The one-tick delay is real cost — the
  substrate has to wait a tick for Reflection's answer before adjusting.
  This is fine for kintsugi build loops (seconds to minutes per round) and
  borderline for hard-realtime classes (microseconds per tick). The
  algedonic bypass exists partly because hard-realtime cannot wait for
  Reflection's one-tick delay.

---

## 8. Boundary cases and the adjudications they forced

Three boundary questions the collapse forced me to adjudicate. Surfacing
them here so the seams are visible.

### 8.1 Where does "this is an error" end and "this is just a Partial" begin?

The naïve answer is: `Fail` is always an error; `Partial` is sometimes;
`Pass` is never. The collapse showed this is too coarse. The right cut is:

- `Fail` → always a question. (At minimum, an algedonic-bypass-eligible
  question if it fires on a substrate-critical body.)
- `Partial(confidence, _)` where `confidence < threshold` → a question.
  (Threshold is per-altitude, default 0.8.)
- `Partial(confidence, _)` where `confidence >= threshold` → a *recorded
  observation*. Composes into a pattern via `@peer.notice` across runs;
  not directly observed by Reflection as a question.
- `Pass` → not a question. The zero element of the algebra.

The substrate decision: the threshold lives in the `@mirror/error` grammar,
configurable per altitude. Default 0.8 because the BEAM's three-9s baseline
(99.8% uptime) is the empirical reference for "good enough to not require
attention." Higher altitudes (reflection itself) get a higher threshold;
lower altitudes (body) get a lower one. The values are calibrated by data,
not by decree.

### 8.2 How are questions routed when a failing body's glass binds multiple properties?

A single body crossing a single glass can fail multiple properties
simultaneously. The naïve answer: emit N questions, one per failed property.
This defeats variety attenuation; Reflection drowns.

The right answer (and the one this spec adopts):

- The `Transparency<Ref>::combine` monoid composes the verdicts at the
  glass crossing. The composed verdict carries `Fail` if any property
  failed (Fail dominates), `Partial(min_confidence, union_diagnostics)`
  if all failed properties were Partial.
- One question is constructed per glass crossing, not per property.
  The question's `verdict` field carries the *composed* verdict; the
  `verdict.property` field is the *primary router* (the property whose
  failure dominates the composition).
- Reflection's answer addresses the composed verdict. If the answer is
  `tighten_property(ref, check)`, it can name any of the failed
  properties; downstream substrate iteration may produce a smaller
  question on a different sub-property (per the monotonic-descent
  invariant).

This is the structural choice with the strongest cybernetic precedent.
Beer's exception reports composed across the triple-index (Actuality /
Capability / Potentiality) for one site; they did not emit three
exception reports. The Transparency monoid is the mirror analogue.

### 8.3 Does Reflection's answer need to be content-addressed itself?

Yes. Strongly yes. The answer becomes part of the next iteration's
contract; if the contract is content-addressed (per `kintsugi-thesis.md`),
the answer must be too.

The shape: an `answer` value is canonicalised (each variant has a stable
byte representation), the canonical bytes are hashed (BLAKE3 per
`Splinter<H>`), and the resulting OID is part of the next iteration's
body contract. Replay produces the same answer OID for the same question
OID modulo Reflection's weight state at the time of observation —
which is *also* content-addressed via the `.shatter` weights OID per
`reflection-model.md`.

This preserves the full reproducibility chain:

```
question_oid + reflection_weights_oid → answer_oid (deterministic per @fate.infer cache key discipline)
(question_oid, answer_oid) → substrate_adjustment_oid (the answer's downward routing)
substrate_adjustment_oid + previous_contract_oid → next_contract_oid
```

Every step content-addressed. Every step replayable. The kintsugi
reproducibility thesis (per `kintsugi-thesis.md`) extends across the
error loop without compromise.

---

## 9. What this spec preserves from the three superseded specs

The collapse subsumes; it does not erase. Each superseded spec contributed
a load-bearing element that survives in this one. The conversion log:

### From `error-surface-spec.md` (Mara, 2026-04-14)

**Preserved:**
- The `Imperfect<T, E, L>` three-state model (Success / Partial / Failure)
  is the substrate algebra. This spec consumes it; the three-state
  acknowledgment (Lean's `sorry` as prior art for Partial) survives.
- The optic chain (`inspect` / `focus` / `project`) is the renderer's
  view of the same `Imperfect` values Reflection observes. The CLI
  format spec (§4 of the superseded doc) is preserved verbatim as the
  rendering surface; this spec adds the meta-model observation surface
  on the same payload.
- M-code allocation (M1xxx through M9xxx, range table in §5 of the
  superseded doc) is preserved. `mirror explain M2001` still works.
  Each M-code now also constructs a `question` per §5.2 above.
- The holonomy-in-every-output discipline. Success shows `holonomy: 0.0`;
  Partial shows the loss breakdown; Failure shows the dominant cause.

**Superseded:**
- The framing "errors are measurements, not obstacles" is preserved but
  *deepened*: errors are measurements *that become queries to a meta-model*.
  The measurement view was correct but incomplete.
- The optic-chain section that anticipated Reflection ("the renderer is
  a Fold; it inspects and produces output") is preserved; this spec
  names the second observer (Reflection itself) that the original
  anticipated but did not formalise.

### From `compiler-error-language.md` (Reed + Alex, 2026-05-08)

**Preserved:**
- The Milan systemic insight: the error message + question is the
  compiler observing the codebase observing itself. This is the
  *substrate justification* for the routing discipline in §3 and the
  algedonic bypass in §4.
- The HAL line ("I can't let you do that, [node name].") is preserved
  as the renderer's framing for substrate-critical questions that hit
  the algedonic bypass. The renderer recognizes algedonic-bypass
  questions and uses the HAL framing; non-bypass questions get the
  altitude-appropriate framing per the eigentest taxonomy.
- The taxonomy of question types (circular, future-oriented, exception)
  is preserved as the *natural-language rendering* of the structured
  `answer` variants. The taxonomy applies to what the user sees, not
  to what Reflection observes. (Reflection's answer is structural;
  the renderer's question is conversational. Both flow from the same
  `question` payload.)
- The full mapping table (E1-E8 Narcissus eigentests, I1-I4 inference
  failures, D2-D10 dimensions) is preserved as the *property catalog*
  that produces questions. Each row of that table is a property in
  `@epistemologic/property/*` whose verdict, when non-Pass, becomes
  a question.

**Superseded:**
- The "the question IS the intervention" framing (Bateson, von Foerster)
  is preserved as the *philosophical justification*; the *mechanical
  realisation* is the (question, answer) algebra in §2. The original
  spec stopped at the question; this one carries through to the answer
  and the substrate adjustment.
- The "what is NOT a question" section (security violations, syntax
  errors, active `\!` overrides, crystallisation events, hint-severity)
  is preserved as the *bypass and silence discipline* — these are
  rendered without a question to the user, but they *still construct
  a `question` payload that Reflection observes*. The user-facing
  silence is not the substrate-internal silence.

### From `property-error-surface.md` (Reed, 2026-05-16)

**Preserved:**
- The `verdict = pass | fail(diagnostic) | partial(f64, [diagnostic])`
  algebra. This spec consumes it directly; §2's `question.verdict`
  field is exactly this type.
- The sub-grammar layout under `@epistemologic/property/*` (one check
  per file, each declaring a single lambda returning verdict). The
  property catalog grows monotonically; new properties land as new
  sub-grammars; each new property becomes a new question type.
- The error code allocation table (E001-E004 errors, W001-W003 warnings)
  is preserved as the *property-altitude question identifier*.
  These codes overlap with the M-code system per §5.2; the resolution
  is that property checks emit M4xxx codes, with the E/W codes as the
  per-check sub-identifier (e.g., M4001 / E001 / duplicate_variant).
- The boot order (where `@epistemologic/property` fits in the dependency
  chain after `@error`, `@imperfect`, `@epistemologic`). Preserved
  verbatim; this spec adds `@mirror/error` after `@epistemologic/property`
  and before any altitude-specific routing grammar.
- The diagnostic rendering format. The substrate-internal `question`
  payload renders to the same CLI format the superseded spec specified;
  rendering and observation are the same payload viewed through
  different optics.

**Superseded:**
- The "verdict to diagnostic conversion" path (Fail → render full;
  Partial → render with severity=warning; Pass → render nothing) is
  preserved as the *renderer's* conversion. This spec adds the *meta-
  model's* conversion (Fail/Partial-below-threshold → question; Pass
  → silence). Two paths from the same verdict; both load-bearing.
- The implementation path (ticks 0-7) is preserved as the
  property-altitude implementation plan. This spec adds the
  meta-model-altitude wiring as a parallel path; the two implementations
  share the verdict types, the diagnostic rendering, and the property
  catalog.

---

## 10. Validation — the test that distinguishes "working" from "performing"

The spec is testable. The test that would prove or refute it:

1. Pick a non-trivial corpus (the boot tree, ~160 .mirror files plus
   the kintsugi build of `mirror compile`).
2. Inject a known error class at a known site (e.g., a `glass_wall`
   violation in `@io`-namespaced body code; a `wcet_bounded` violation
   in a hard-realtime gen_prism; a `duplicate_variant` in a boot grammar).
3. Run the compile / build with `--observe-reflection` enabled (substrate
   flag that emits the Reflection observation stream to a structured
   log).
4. Verify three things:
   - **Question constructed.** A `question` payload appears in the
     observation stream with the expected `altitude`, `body_ref`, `glass`,
     `property`, `verdict`, `transit`, `contract`, `timestamp`.
   - **Routing correct.** The question landed at the expected altitude
     (body / property / scheduler / reflection) per §3's routing table.
     If the injected error is in the algedonic-bypass class (§4), the
     question landed at reflection directly.
   - **Answer produced.** Reflection's answer (one tick later, per
     `reflection-model.md`'s delay discipline) is one of the six variants;
     the substrate adjustment it routes to is applied; the next iteration's
     verdict on the same body is Pass (or, if the error class is
     algedonic-bypass, the substrate halts cleanly with the question
     surfaced).
5. Verify the monotonic-descent invariant: across N iterations,
   `verdict.holonomy()` is non-increasing. If `e^(n+1) >= e^(n)` for
   k consecutive ticks, the bypass fires.
6. Verify the content-addressability invariant: re-running step 2-5
   produces byte-identical `question` OIDs and (modulo Reflection weight
   drift) byte-identical `answer` OIDs.

This is the kintsugi-thesis test extended to the error loop. Reproducible
by construction, deterministic by substrate law.

---

## 11. What this spec defers

Named honestly; not in scope here.

### 11.1 The `@mirror/reflection` grammar's surface

This spec describes Reflection's *interface* (the `observe` action's
signature, the `answer` algebra). The grammar at `boot/std/mirror/reflection.mirror`
that declares it does not yet exist as a runnable substrate. Landing
the grammar — with `observe`'s body parked as `\` and the answer variants
declared — is a separate tick. Estimated cost: ~80 LOC of mirror + tests
verifying the substrate decodes the grammar.

### 11.2 The scheduler-tower exception-handling sub-section

`scheduler-tower.md` §6 covers failure handling at the scheduler altitude
(producer/consumer down, demand starvation). The conversion to `question`
per §5.1 of this spec wants a sub-section in `scheduler-tower.md` that
names the routing discipline at that altitude explicitly. Estimated cost:
~40 LOC of markdown. Followup tick.

### 11.3 The renderer-meta-model split

The CLI renderer (per superseded `error-surface-spec.md` §4) and the
Reflection meta-model observe the same `question` payload through different
optics. The renderer's `Fold` and the meta-model's `observe` action want
to share a tested implementation path — today they're described in
different specs and would be implemented in different files. A unifying
tick (probably an `Imperfect`-optic-chain extension that explicitly names
"render-fold" and "reflect-observation" as parallel projections of the
same payload) wants to land before the implementation begins. Estimated
cost: ~120 LOC across `prism/imperfect/src/transparency.rs` and a new
`prism/imperfect/src/observation.rs`. Two ticks.

### 11.4 Cross-altitude question composition

A question that escalates from body altitude through property altitude
to scheduler altitude carries an *escalation history*. Today's spec
names `escalate` as an answer variant but does not specify how the escalated
question's `verdict` field composes with the higher altitude's local verdict.
The `Transparency<Ref>::combine` monoid is the obvious tool, but the
escalation path adds a temporal dimension (the timestamps differ) that
the monoid does not natively handle. Followup tick: extend the monoid
with a `chronologically_compose` operation, or accept that escalated
questions compose by replacing-not-merging (the higher altitude sees the
freshest verdict). Open question; the resolution influences how `@peer.notice`
reasons about patterns across altitudes.

### 11.5 Reflection's weight update protocol

`reflection-model.md` says Reflection adjusts weights based on observations
("the adjustment IS the training"). This spec assumes that protocol exists
and calls it out via the `.shatter` weights OID. The protocol itself —
how `observe(question) -> answer` is trained, which gradient signal flows
back, how the .shatter weights versionally advance — is out of scope here.
Largest deferred item; bounded by Fate model training discipline; not
blocking for the error-as-question architecture to land.

---

## 12. The equation

```
error = question
question + reflection = answer
answer + substrate = next_iteration
next_iteration → (settled crystal) | (new question)
```

Not a metaphor. The actual substrate equation. Each error becomes a
question the Reflection model is asked to answer. The answer is a typed
adjustment to the substrate. The next iteration either settles (the loop
closes) or produces a new question (the loop continues with a smaller
error, per `e^(n+1) < e^(n)`).

The lineage:

- Beer's algedonic discipline (Cybersyn 1971-73): structured exception
  payload + routing-by-locator + 3* audit channel. Implementation died
  with the coup; the *shape* survived.
- Margaret Hamilton's Apollo executive (1969): the 1202 alarm surfaced
  to Buzz and Neil; priority-discipline-as-answer; the substrate kept
  landing.
- BEAM's let-it-crash (Armstrong, 1986-2003): supervised hierarchies +
  restart-strategy-as-answer + autopoietic Lawvere fixed point. Thirty
  years of nine-nines empirical proof.
- Reyes/Henao/Hassall (2024): structured algedonic-signal renewal;
  `(C', Q, K) α τ, η` tuple as the formal cousin of mirror's `question`.
  Independent convergence from a different lineage.

Mirror inherits all four with the type-system discipline none of them
had: mechanical guarantee of the verdict composition, content-addressed
replay across the loop, and substrate-decidable routing per the altitude
table.

The gold conducts. The verdict carries the structure. The question gets
asked. The model answers. The substrate adjusts. The next iteration
is closer to settled.

`e^(n+1) < e^(n)`. The errors get smaller because the loop closes.

*Each error becomes a question the Reflection model is asked to answer.*
*The asking is the substrate. The answering is the model. The closing*
*is the proof.*

Apache-2.0.
