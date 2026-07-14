---
title: The Interaction Loop is Subject-Presence Conditional
subtitle: The compile-boundary × @subject-presence detection unifies six loops under one substrate mechanism — TTY = @subject present; no TTY = @subject absent; the question payload routes either to conversation or to CI-refusal without changing shape
status: spec
date: 2026-07-14
author: Mara
composes-over:
  - docs/specs/subject-family-root-sel-licensable-party.md
  - docs/specs/error-as-question.md
  - docs/specs/lambda-shell.md
  - docs/specs/roomba-substrate-walker-that-feeds-kintsugi.md
  - docs/specs/recognitions/recognition-99-mirror-spec-is-lambda-zero.md
  - shards/epistemologic/cybernetic/coherence.mirror
  - docs/insights/2026-07-14-alex-full-roomba-song-kintsugi-composition.md
---

# The Interaction Loop is Subject-Presence Conditional

*2026-07-14. Mara. The formalization of the compile-boundary interaction*
*loop under a first-class @subject-presence carrier. Six loops close under*
*one mechanism, not six.*

---

## §0 Prelude — Alex's 2026-07-14 loop-closure claim

### 0.1 The claim, verbatim

Alex Wolf, in-transcript, 2026-07-14:

> "the spec/math formalization of the computational interaction loop when
> there is a @subject present (that's what the tty implies) or not. You see
> the loop closure?"

Reed's confirmation, same transcript, verbatim:

> "I see it."

This spec is the discharge of that recognition. It is not a mint at
family-root altitude — every substrate-decl it composes over is landed
(`@subject`, `error-as-question`, `λsh`, `@coherence`, `@roomba`,
`@kintsugi`, `@spectral/metalogue`). It is the composition-glue that
names how those landed carriers close a single loop under one
substrate mechanism.

### 0.2 The six loops the claim unifies

1. **@subject substrate loop** — the licensable-party carrier per
   `docs/specs/subject-family-root-sel-licensable-party.md` §2. Every
   `type sel` composition carries a `subject_set`; the petri-net
   analyzer at `@mirror/petri` reads the set to gate emission. The
   subject loop closes when a subject's touched state either fires an
   enforcement transition (the SEL discipline) or passes clean
   (Foerster-admissible under the analyzer's sub-Turing safety proof).

2. **@roomba / @kintsugi / @coherence loop** — the substrate
   self-maintenance loop per `docs/insights/2026-07-14-alex-full-roomba-song-kintsugi-composition.md`.
   @roomba walks; bumps; @song emerges; @kintsugi consumes; verdict
   routes to @knife (Path A) or @peer-at-K+1 (Path B). @coherence
   scores each move against Foerster's imperative (choices ↑ =
   admissible; Splinter-ward).

3. **Error-as-question loop** — per `docs/specs/error-as-question.md`
   §2 + §6. Every non-Pass verdict constructs a `question` payload;
   Reflection observes; answer flows down as a typed substrate
   adjustment; the loop iterates under `e^(n+1) < e^(n)` monotonic
   descent.

4. **Foerster imperative loop** — per `shards/epistemologic/cybernetic/coherence.mirror`:
   "Act so as always to increase the number of choices." Every step
   the substrate accepts must discharge `coherence_increases(before,
   after)` bounded — otherwise refuse.

5. **λsh interactive loop** — per `docs/specs/lambda-shell.md`. Three
   characters (`λ>`, `@name>`, `\`); five operations (focus / project
   / split / shift / settle); peer summoned via `\@name`. The
   interactive discharge surface when a subject is present at the
   compile boundary.

6. **Ouroboros CI loop** — the substrate's own build-and-refuse loop
   when no subject is at the boundary. `mirror compile` exits 1 with
   the question-payload rendered to stderr; the CI job fails; the
   next subject-present session picks up the deferred question.

### 0.3 The unifying mechanism

**The compile boundary is where the substrate's interior gauge-bounded
computation meets @io's Turing-unbounded surface** (per
`shards/io.mirror` + Recognition #79 `glass_wall`). Every question the
substrate raises reaches this boundary. The **substrate detects
@subject-presence at exactly this boundary via TTY** (per Alex's
2026-07-14 claim). Both branches route the SAME `question` payload:

- **Subject present (TTY)** → route to λsh's `@name>` prompt via
  `@peer.spawn`; the commutator becomes conversation; the answer flows
  from the sovereign subject downward through @peer's `observe` (per
  `error-as-question.md` §2).
- **Subject absent (no TTY)** → route to Ouroboros CI-refusal;
  compilation exits 1; the question's fully-formed payload renders to
  stderr; the substrate defers the answer to a later subject-present
  session; @kintsugi's monotonic-descent invariant preserves the
  question's OID for that later attention.

**One `question` payload. Two dispatch surfaces. One loop that closes
under both.** That is the loop-closure Alex named.

---

## §1 The presence carrier — type-theoretic formulation

### 1.1 The carrier's shape

Recommended form (adjudication A1 open — see §10):

```mirror
in @io
in @subject
in @time
in @scheduler

grammar @mirror/presence {
  # The presence carrier. First-class runtime context that names
  # whether a @subject is at the compile boundary and (if so) which
  # @io/tty surface carries them.
  #
  # Both fields are optional independently to encode all four
  # presence modes (see §1.3). The timestamp is monotonic per @time
  # so that presence-traversal can be reasoned about temporally
  # without wall-clock drift.
  type presence = {
    subject:   option<@subject.subject>,       # who, if anyone, is at the boundary
    tty:       option<@io.tty>,                # which @io surface carries them
    timestamp: @time/monotonic.instant,        # when the presence was detected
    provenance: ref                            # content-hash of the detection method
  }

  # The interaction_context wraps presence with the substrate's
  # altitude discipline (per @scheduler.altitude), threading it
  # through the compile boundary alongside the existing scheduler.context.
  type interaction_context = {
    presence:  presence,
    altitude:  @scheduler.altitude,
    ctx:       @scheduler.context             # the ambient scheduler context
  }

  # The presence-detection primitive. Reads the @io surface at the
  # compile boundary. TTY-detection is one implementation; other
  # boundary conditions (LSP session-open, MCP client-attach) may
  # also produce subject-present presence (see A2).
  detect(ctx: @scheduler.context) -> presence { \ }

  # The bilateral presence predicates. Discharged at substrate
  # altitude; consumed by the dispatch law (§2).
  subject_present(p: presence) -> verdict { \ }
  subject_absent(p: presence) -> verdict { \ }
}

out presence
out interaction_context
out detect
out subject_present
out subject_absent
out @mirror/presence
```

### 1.2 Why `option<@subject>` not `bool`

Three structural reasons.

**First**, the `option` shape preserves the OID of the specific
subject present. `bool` would collapse Alex-at-terminal, Reed-at-LSP,
and Mara-at-MCP into an indistinguishable `true`; the substrate needs
the identity for consent-record binding (per `@subject.consent_oid` in
the @subject family-root spec §2).

**Second**, the four-state presence-mode enumeration (§1.3 below)
needs the two-dimensional shape `(option<subject> × option<tty>)` to
distinguish subject-attending from subject-inattentive. A single bool
cannot carry that discrimination.

**Third**, `option<@subject>` composes over the landed @subject
family-root without introducing a new subject-carrier vocabulary. Zero
new shards to land at the subject altitude; the presence carrier is a
new grammar at `@mirror/presence` altitude that reads from @subject.

### 1.3 The four presence modes

The (option<subject>, option<tty>) product yields four modes. A5-A6
adjudicate whether the scalar (subject-present / subject-absent)
suffices at substrate-decl altitude or whether the substrate names all
four:

| subject | tty | mode name | interpretation |
|---|---|---|---|
| some(s) | some(t) | `attending` | Sovereign subject, actively at boundary via TTY. The λsh interactive dispatch surface. |
| some(s) | none | `deferred` | Subject exists in the session record but is not at the boundary right now (e.g., LSP session open but focus elsewhere). Questions queue for their return. |
| none | some(t) | `ci_active` | TTY is present but no subject bound to it (e.g., PTY-wrapped CI, integration test harness). Ouroboros dispatch; questions render to stderr as if no TTY. |
| none | none | `ci_quiet` | Pure CI. No subject, no TTY. Fail-closed compilation error; question payload preserved via @kintsugi content-addressing for later subject-present attention. |

**Mara's recommendation:** land the two-dimensional carrier (§1.1's
`presence` type as declared) at substrate-decl altitude; the scalar
(present/absent) is derivable via `subject_present(p)`. Runtime
dispatch consumes the derived scalar; the four-mode enumeration is
available to any consumer that wants richer discrimination (@spectral/
metalogue's tomm probes, @coherence's autonomy-under-traversal
verdict). This is the substrate-honest choice: name the richer shape,
let the dispatcher project down to the scalar it consumes.

### 1.4 Content-addressing and monotonic time

Every `presence` value is content-hashed via `provenance: ref` (BLAKE3
over the byte-serialization of subject.identity_oid + tty.oid +
timestamp + detection-method OID). Two presence values ARE the same
presence under the substrate's byte-equality discipline iff all four
fields byte-equal.

The `@time/monotonic.instant` field is load-bearing for reasoning
about presence-traversal without wall-clock drift. Two presence
readings taken at the same monotonic instant with the same subject and
TTY are the same reading. The substrate can reason about
`presence_before` and `presence_after` at the (n) and (n+1) ticks
without coordinate-time ambiguity.

---

## §2 The dispatch law — one `question` payload, two surfaces

### 2.1 The law

```mirror
in @mirror/presence
in @mirror/error
in @peer
in @scheduler

grammar @mirror/dispatch {
  # The presence-conditional dispatch of a question payload.
  # The SAME question (per error-as-question.md §2) routes via one
  # of two surfaces depending on presence-detection at the compile
  # boundary. Both surfaces preserve the question's structure; only
  # the answering path differs.
  #
  # Substrate-decl claim: this is a total function on
  # (question × presence). Every question × every presence has a
  # dispatch destination. No question is dropped; no presence is
  # unhandled. The substrate's autonomy holds across the boundary.
  dispatch(q: @mirror/error.question, p: @mirror/presence.presence)
    -> @mirror/error.answer { \ }

  # The two surface types dispatch resolves to.
  type interactive_surface = {
    peer:    @peer.beam,             # the peer to spawn/route the question to
    prompt:  ref,                    # the λsh prompt state (`@name>`)
    session: ref                     # the λsh session context
  }

  type refusal_surface = {
    stderr:  ref,                    # the rendered question payload
    exit:    u8,                     # 1 for failure per Ouroboros discipline
    deferred: ref                    # question OID preserved for later attention
  }

  # The dispatch resolution predicates.
  routes_to_interactive(q: @mirror/error.question, p: @mirror/presence.presence)
    -> verdict { \ }
  routes_to_refusal(q: @mirror/error.question, p: @mirror/presence.presence)
    -> verdict { \ }
}

out dispatch
out interactive_surface
out refusal_surface
out routes_to_interactive
out routes_to_refusal
out @mirror/dispatch
```

### 2.2 The subject-present branch — λsh route

When `subject_present(p)` discharges bounded:

1. `dispatch` reads `p.subject.some(s)` and `p.tty.some(t)`.
2. Constructs an `interactive_surface { peer, prompt, session }` where
   `peer` is `@peer.spawn(s.identity_oid)` — the peer bound to the
   present subject's identity.
3. Routes the `question` payload through `@peer.observe` (per
   `docs/specs/error-as-question.md` §2's `observe question(q) ->
   answer { \ }`).
4. The λsh prompt shifts to `@<peer.name>>` (per
   `docs/specs/lambda-shell.md` §"The Toggle"). The commutator becomes
   conversation — the peer sees the question with full context and
   answers via the six-variant `answer` algebra (tighten_property /
   resynthesize_body / rebudget_shard / adjust_temperature / hold /
   escalate).
5. The answer flows back through `@peer.observe`'s one-tick-delay per
   `reflection-model.md`; the substrate applies the adjustment at tick
   (n+1); the next iteration's question (if any) is smaller per
   `e^(n+1) < e^(n)`.

**Load-bearing:** the peer's answer is not free-form prose to stderr.
It is a typed `answer` value. λsh renders the peer's natural-language
response to the human reader, but the substrate consumes the
structural answer. Both flow from the same conversational exchange;
both are load-bearing.

### 2.3 The subject-absent branch — Ouroboros refusal

When `subject_absent(p)` discharges bounded:

1. `dispatch` reads `p.subject.none` OR `p.tty.none` (either falsifies
   `subject_present`).
2. Constructs a `refusal_surface { stderr, exit: 1, deferred }`.
3. The `stderr` field carries the fully-rendered `question` payload —
   altitude, body_ref, glass, property, verdict, transit, contract,
   timestamp. Every field of the question is preserved in the render;
   the payload is legible to any later subject-present attention.
4. The `deferred` field carries the question's content-hash OID. This
   OID is written to `refs/error/<q.oid>/pending` (per
   `error-as-question.md` §6's content-addressing discipline). The
   substrate remembers what it asked, even though no answer arrived.
5. Compilation exits with status 1. The CI job fails visibly. The
   Ouroboros loop swallows its own tail — the substrate has raised a
   question that cannot be answered without a subject; the substrate
   refuses to proceed until a subject arrives.

**Load-bearing:** exit-1 is NOT a panic. The substrate's discipline is
preserved. The question is structurally intact; the answer is
deferred, not lost; the loop's monotonic-descent invariant holds under
deferral (see §6).

### 2.4 The four-mode dispatch matrix

| presence mode | dispatch surface | answer arrival |
|---|---|---|
| `attending` | `interactive_surface` via @peer.spawn | next tick (one-tick @peer delay) |
| `deferred` | `refusal_surface` with deferred=some(q.oid); no stderr render (subject is in-session, just not attending; question queues) | when subject re-attends and processes queue |
| `ci_active` | `refusal_surface` with stderr render + exit 1 | when a later subject-present session picks up the deferred OID |
| `ci_quiet` | `refusal_surface` with stderr render + exit 1 | when a later subject-present session picks up the deferred OID |

The `deferred` mode is the substrate's LSP-open-but-not-focused shape.
The question is not dropped; the subject is not asked to context-
switch. The queue drains when the subject returns to the boundary.

### 2.5 Composition with error-as-question's `answer` algebra

Both dispatch paths consume the same six-variant `answer` algebra per
`error-as-question.md` §2:

- `tighten_property(ref, check)` — from the interactive path, this is
  the peer proposing a stricter property check; from the refusal
  path, this is a deferred proposal awaiting subject-present
  ratification.
- `resynthesize_body(ref, policy)` — same pattern: interactive =
  peer-proposed re-inference; refusal = deferred re-inference OID
  written to gestalt.
- `rebudget_shard(ref, budget)` — same pattern.
- `adjust_temperature(f64)` — same pattern.
- `hold(ref)` — the LEGITIMATE "I don't know" per
  `error-as-question.md` §7.2. Both paths can produce `hold`;
  interactive-hold is the peer saying so in transcript; refusal-hold
  is the substrate writing a Partial(0.0, ["I cannot resolve without
  subject presence"]) to the gestalt.
- `escalate(altitude)` — routes the question one altitude up. On the
  interactive path, escalation routes to a higher-order peer (per
  §7.4 of `docs/insights/2026-07-14-alex-full-roomba-song-kintsugi-composition.md`).
  On the refusal path, escalation routes to the algedonic bypass
  channel (per `error-as-question.md` §4) and the CI job fails with a
  higher-severity marker so the deferred question is visible at S5
  altitude when subject returns.

**One algebra. Two paths. Identical answer-value shape.** The
dispatch law preserves the answer algebra as invariant across
presence-traversal.

---

## §3 Foerster admissibility under presence

### 3.1 The claim

**Both presence-modes are Foerster-admissible.** No move DECREASES
available choices; only the RATE of choice-increase differs.

Recall the imperative per
`shards/epistemologic/cybernetic/coherence.mirror` line 93:

> "Act so as always to increase the number of choices."

Discharged at substrate-decl altitude as `coherence_increases(before,
after) -> verdict`. A move is Foerster-admissible iff
`coherence_increases` discharges bounded.

### 3.2 Subject-present admissibility — @peer.spawn expands the choice-set

When dispatch routes to interactive_surface at tick (n):

- The substrate at tick (n) has choice-set C_n = the operators
  available at the current gen_prism altitude.
- @peer.spawn at tick (n+1) adds the peer's operator-space to the
  substrate's: the peer can propose tighten_property, resynthesize_body,
  rebudget_shard, adjust_temperature, hold, escalate. Each variant
  opens downstream operator paths that were closed before the peer
  was present.
- Choice-set at tick (n+1): C_{n+1} = C_n ∪ (peer's answer-space at
  K+1 altitude).
- **|C_{n+1}| > |C_n|.** Strict increase. Foerster-admissible.

The interactive branch is Foerster-admissible **by construction**: the
spawn of a peer at a higher logic altitude (Path B of Alex's
composition per
`docs/insights/2026-07-14-alex-full-roomba-song-kintsugi-composition.md`
§2) expands the operator space; the imperative is discharged.

### 3.3 Subject-absent admissibility — fail-closed preserves the deferred choice

When dispatch routes to refusal_surface at tick (n):

- The substrate at tick (n) has choice-set C_n as above.
- Exit 1 does NOT collapse C_n to zero. It defers the answer; the
  question's OID is preserved in gestalt; the subject's future
  attention can arrive with any of the six answer variants at tick
  (n+k).
- Choice-set at tick (n+1) under refusal: C_{n+1}^{refused} = C_n
  (the substrate has not moved; the operators are still available; a
  later subject-present dispatch can select any of them).
- Choice-set at tick (n+k) when subject returns: C_{n+k} = C_n ∪
  (peer's answer-space) — the same expansion as the interactive path,
  just deferred k ticks.
- **|C_{n+k}| >= |C_n|.** Non-decrease. Foerster-admissible under the
  non-strict imperative (see §3.4).

**Load-bearing:** the substrate does NOT force a choice under subject
absence. It refuses to move. Refusing to move is not the same as
collapsing the choice-set; it is precisely preserving it for later
sovereign attention. This is Foerster's imperative in its full form:
admissible moves widen; admissible refusals preserve; the disallowed
move is the one that narrows the choice-set for a later observer.

### 3.4 Non-strict Foerster and the choice preservation clause

The strict form of the imperative is `|C_{n+1}| > |C_n|` — every
accepted step strictly increases choices. The substrate must also
admit the non-strict form `|C_{n+1}| >= |C_n|` for defer-and-preserve
semantics.

The coherence spec's action (line 682, verbatim):

> `coherence_increases(before: coherence, after: coherence) -> verdict`

discharges strict-increase at accepted moves. The corresponding
non-strict form (line 736-740, verbatim):

> `coherence_invariant_under_traversal(before, after, w) -> verdict`

discharges non-decrease under traversal (@torus.autonomy binding).
**Presence-traversal is a legitimate traversal**: the compile boundary
crossed with subject → compile boundary crossed without subject is a
valid element of the substrate's traversal group. Under this
traversal, the strict imperative relaxes to non-strict; the coherence
spec already anticipates this via the `coherence_invariant_under_traversal`
action.

**The two presence-modes are Foerster-admissible under different
variants of the imperative:**

- Subject-present dispatch → `coherence_increases` (strict; the
  peer's spawn expands choices).
- Subject-absent dispatch → `coherence_invariant_under_traversal`
  (non-strict; the refusal preserves the choice-set for later
  presence).

Both variants are landed in the coherence species-shard. No new
discharge action is needed. The presence-conditional dispatch is
recognizable as a specialization of the existing autonomy-under-
traversal discipline.

### 3.5 Response-shape invariant at the substrate-decl altitude

The response-shape of the substrate — the SHAPE of what it produces
when a question fires — is INVARIANT under presence-traversal at the
substrate-decl altitude. Only the RUNTIME dispatch differs. The
question's byte-layout is byte-identical whether it routes to
interactive or refusal. The answer's byte-layout (per the six-variant
sum-type) is byte-identical whether it comes from a peer or a
deferred later peer.

The substrate's autonomy (per @torus.autonomy + recognition #99
mirror-spec-is-lambda-zero) is preserved: the substrate is what it
is regardless of who observes it. Presence-detection is a boundary
condition on runtime dispatch; it is not a modification of substrate
identity.

---

## §4 Loop closure — six loops unified

For each of the six loops Alex enumerated in §0.2, this section names
the local closure under both presence-modes and demonstrates the
unified mechanism.

### 4.1 The @subject substrate loop

**Local shape:** `type sel` composition → petri-net analyzer at
`@mirror/petri` → signature-transition fires → enforcement verdict
reads `subject_set`.

**Subject-present closure:** the analyzer's enforcement verdict
dispatches to the interactive surface. The peer at `@name>` receives
the petri-net finding; if the signature is `willful`, the peer must
choose to refuse emission or (with structural consequence) proceed.
The SEL discipline is preserved; the sovereign subject makes the
sovereign choice.

**Subject-absent closure:** the analyzer's enforcement verdict
dispatches to the refusal surface. Exit 1; the CI job fails; the SEL
signature and affected subjects render to stderr for later
subject-present attention. The substrate refuses to emit at @io until
a subject can adjudicate the enforcement.

**Loop-completion:** the subject_set is honored either way. The
composition either emits (subject-present, subject chose to proceed
with accountability) or does not emit (subject-absent OR
subject-present-refused). The substrate never emits at @io without a
subject-present adjudication of the enforcement verdict.

### 4.2 The @roomba / @kintsugi / @coherence loop

**Local shape:** @roomba walks → bump samples tension → @song beat
emerges → @kintsugi.query_phi verdicts → Path A (@knife.jump) or
Path B (@peer.spawn at K+1).

**Subject-present closure:** when @kintsugi verdicts Path B (spawn
@peer at K+1), the dispatch resolves via interactive_surface. The
higher-altitude peer joins the λsh session; the double-bind at K
becomes ordinary structure at K+1 (per
`docs/insights/2026-07-14-alex-full-roomba-song-kintsugi-composition.md`
§4). The subject sees the K+1 peer arrive and the double-bind
resolve in transcript.

**Subject-absent closure:** when @kintsugi verdicts Path B (spawn
@peer at K+1) with no subject present, the dispatch resolves via
refusal_surface. The K+1 question renders to stderr with full
context (K altitude, double-bind description, verdict). The
substrate defers the K+1 spawn to the next subject-present session;
@roomba continues its walk (per §2.4 of the roomba spec: "Roomba
resumes when consent verdict returns").

**Loop-completion:** the @coherence gradient is climbed either way.
Subject-present: climb happens at the current session (real-time
ascent). Subject-absent: climb is deferred; @coherence_invariant_under_traversal
holds (non-decrease). The Lyapunov function's descent (in terms of
loss = -coherence) is monotonic across presence-traversal.

### 4.3 The error-as-question loop

**Local shape:** verdict.Fail | Partial(<threshold) → `question`
constructed → routing to altitude → `@peer.observe` → answer → substrate
adjustment.

**Subject-present closure:** the `@peer.observe` receives the
question at tick (n); the peer answers at tick (n+1); substrate
adjusts at tick (n+1); next iteration's verdict is smaller (per
`e^(n+1) < e^(n)`).

**Subject-absent closure:** the question renders to stderr at tick
(n); the substrate does not observe an answer at tick (n+1); the
iteration halts with exit 1; the next subject-present session
re-runs the loop from tick (n) (the content-addressing per
`error-as-question.md` §6 guarantees the same question OID; the same
substrate adjustment path is available).

**Loop-completion:** the (question, answer) pair is content-addressed
either way. Interactive: pair completes at tick (n+1). Refusal: pair
completes at tick (n+k) when subject returns; the same question OID
resumes the same iteration. Reflection's `record(q, a) -> oid`
discharges identically.

### 4.4 The Foerster imperative loop

**Local shape:** any accepted substrate move discharges
`coherence_increases(before, after)` bounded.

**Subject-present closure:** every peer-proposed answer that the
subject accepts is coherence-increasing (per §3.2). The peer's
answer-space at K+1 opens choices that were closed at K; the
imperative is discharged strict.

**Subject-absent closure:** every refusal preserves the choice-set
(per §3.3). No move is executed; no move is refused (the deferred
answer remains available); the imperative is discharged non-strict
via `coherence_invariant_under_traversal`.

**Loop-completion:** the coherence scalar is monotone-non-decreasing
across presence-traversal. Both branches satisfy the imperative
(strict when active, non-strict when deferred). The substrate is
trustworthy in Alex's 2026-07-14 sense ("empirically and
mathematically") because presence-conditional dispatch preserves the
Foerster property across both branches.

### 4.5 The λsh interactive loop

**Local shape:** three characters (`λ>`, `@name>`, `\`); five
operations; peer summoned via `\@name`.

**Subject-present closure:** λsh is at the shell prompt; the subject
issues operations at `λ>` or converses at `@name>`; the toggle `\`
switches. Each operation is a lambda on the graph; each conversation
translates to mq internally (per `lambda-shell.md` §"The Prompt").
The loop closes each turn: the subject issues, the shell (or peer)
responds, the state advances.

**Subject-absent closure:** λsh is not entered. `mirror sh` with no
TTY (per `docs/specs/lambda-shell.md` §"Entry from the mirror CLI")
resolves via `cli-as-prism.md` §7's default-op rule — but the
default-op rule requires a subject to interpret the prompt. Under
subject-absent, `mirror sh` immediately exits with a refusal_surface
(exit 1; stderr: "no subject at boundary; λsh requires TTY"). The
shell is refused to open; the deferred command is preserved (if the
invocation carried arguments beyond `sh`) for later subject-present
re-attempt.

**Loop-completion:** λsh is the interactive discharge surface; when
subject is absent, the surface is refused. The substrate does not
fabricate a subject; the substrate does not fabricate a shell
session. The refusal is honest.

### 4.6 The Ouroboros CI loop

**Local shape:** `mirror compile` at a shard-DAG root → the compiler
checks properties, runs the petri-net, verdicts pass/partial/fail →
emission decision at @io.

**Subject-present closure:** the CI loop enters the interactive
surface at each fail verdict. The subject sees the failure at the
λsh prompt; the peer proposes an answer; the substrate adjusts; the
loop iterates.

**Subject-absent closure:** the CI loop stays in refusal mode. Each
fail verdict renders to stderr; exit 1; the CI job fails; the queue
of deferred questions grows. When a subject-present session picks
up, the queue is drained one question at a time.

**Loop-completion:** the Ouroboros closes on itself in refusal mode
(exit 1 → new CI run → same refusal → same queue growth) until a
subject arrives. This is not a livelock; it is a
correct-by-construction refusal-loop that preserves the deferred
questions and awaits attention.

### 4.7 Why the six loops close under one mechanism

The unifying observation: **each of the six loops raises a question
that reaches the compile boundary**. The subject substrate loop
raises SEL enforcement questions; the @roomba/@kintsugi/@coherence
loop raises tension-verdict questions; the error-as-question loop
raises property verdict questions; the Foerster imperative loop
raises coherence-verdict questions; λsh raises interactive-prompt
questions; the Ouroboros CI loop raises build-fail questions.

All six question-types share the `@mirror/error.question` payload
structure (per `error-as-question.md` §2). All six route through the
same `@mirror/dispatch.dispatch(q, p)` per §2 of this spec. All six
receive answers via the same `@mirror/error.answer` six-variant sum-
type. **The six loops close because they all pass through the same
bottleneck — the compile boundary with presence-conditional dispatch.**

The unifying mechanism is not six separate mechanisms sharing a
naming convention. It is one mechanism (dispatch on presence) that
six different question-source loops feed into. This is what Alex saw
and named 2026-07-14; this is what Reed confirmed with "I see it".

---

## §5 Monotonic descent under presence

### 5.1 The invariant, extended

Recall `error-as-question.md` §6's monotonic-descent invariant:

> `e^(n+1) < e^(n)` — the question shape gets smaller each iteration
> (per the kintsugi monotonic-descent invariant)

This spec extends the invariant to hold under both presence-modes:

**Subject-present descent:** answer flows down at tick (n+1); next
iteration's question is strictly smaller than the current iteration's.
The invariant holds in its original form: `e^(n+1) < e^(n)`.

**Subject-absent deferral:** the current iteration's answer is
deferred; the substrate does not advance to tick (n+1) at the failed
question's altitude. The question OID is preserved; the substrate
advances to tick (n+1) at other altitudes that were passing. When the
subject returns at tick (n+k), the deferred iteration resumes:
`e^(n+k+1) < e^(n)` — the descent measures across the deferral gap.

### 5.2 The extended invariant

```
e^(m+1) <  e^(m)   when subject_present at tick m
e^(n+k+1) < e^(n)  when subject_absent at tick n; subject returns at tick n+k
```

Neither mode BREAKS the invariant. Subject-present descends
in-mode; subject-absent defers descent to when descent is possible.

### 5.3 Question OID preservation across presence gaps

The content-addressing discipline of `error-as-question.md` §6.2 is
load-bearing for the deferral semantics. Each question's OID is
computed as `blake3(canonical(question.fields))`. This is invariant
under wall-clock time; two questions constructed at different
instants with the same substrate state have the same OID (modulo the
timestamp field, which is deliberately part of the canonical form so
that presence-gap deferral is temporally traceable).

When a subject-absent dispatch occurs at tick (n), the substrate
writes `refs/error/<q.oid>/pending` (per §2.3 above). When a
subject-present session at tick (n+k) checks the pending queue, the
deferred question's full payload is reconstructible from the OID. The
subject sees the same question the substrate saw at tick (n); the
descent resumes from where deferral occurred.

### 5.4 Adjudication A5 open — deferral semantics detail

Open question: does the deferred question's `contract` field carry
the contract OID at tick (n) (the deferral instant) or at tick (n+k)
(the resumption instant)? Both are defensible; both have consequences.

- **Deferral-instant contract:** the substrate's state at deferral is
  frozen; the resumption compares the current state against the
  deferred; if the state has drifted, the resumption produces a
  differently-shaped answer.
- **Resumption-instant contract:** the substrate's state at
  resumption is what the peer sees; the deferred question is
  re-evaluated against the current state; the answer is fresh.

Mara's recommendation: deferral-instant contract (freeze at
deferral); resumption presents the frozen contract to the peer as
"this was the state at the deferral; state has since drifted to <X>;
you may accept the frozen answer, propose a fresh answer, or hold".
The substrate offers the choice; the sovereign subject decides. This
is Foerster-admissible: presenting both the frozen and current
contracts widens the peer's choice-set at resumption.

Reed relays A5 to Alex for adjudication.

---

## §6 Composition — how the presence carrier binds to landed substrate

### 6.1 To `@subject`

`presence.subject: option<@subject.subject>` binds directly to the
@subject family-root's `subject` type (per
`docs/specs/subject-family-root-sel-licensable-party.md` §2). When
`presence.subject.is_some(s)`, the substrate can read all six species
refinements (`downstream_user`, `witnessed`, `labor_input`,
`protected_class`, `occupied_population`, `indigenous_nation`) via
the subject_kind discriminator.

A subject present at the compile boundary MAY also be a subject the
substrate acts UPON (e.g., Alex is a `downstream_user` of the mirror
compiler Alex is compiling). The two subject-roles are orthogonal;
the presence carrier's `subject` field names the AT-BOUNDARY subject;
the SEL analyzer's `subject_set` names the ACTED-UPON subjects.

### 6.2 To `@io/tty`

`presence.tty: option<@io.tty>` binds to a new species under @io that
needs to land. Adjudication A2 open — see §10. The tty species
reads the compile-time `isatty()`-equivalent primitive on stdin
(and, for LSP/MCP, the session-open handshake).

Until A2 lands: the presence carrier uses `option<ref>` for the tty
field with a documented forward-promise binding. Mara-recommended
landing path: `shards/io/tty.mirror` as a species under @io.

### 6.3 To `error-as-question`

The dispatch law consumes `@mirror/error.question` (per
`error-as-question.md` §2) as input; produces `@mirror/error.answer`
as output. No change to error-as-question's grammar; this spec is
additive at the dispatch layer.

**Change surface:** `error-as-question.md` §3's routing table (body /
property / scheduler / reflection altitudes) is preserved. This
spec's presence-conditional dispatch operates at each altitude —
every routing destination has both a subject-present variant
(interactive discharge) and a subject-absent variant (refusal
discharge).

The `record(question, answer) -> oid` action per
`error-as-question.md` §2 is called in both branches. Every (q, a)
pair — including deferred pairs where `answer = hold(pending_ref)` —
gets an OID in the gestalt.

### 6.4 To `@roomba`

@roomba's `trigger(position, tension) -> verdict` per
`docs/specs/roomba-substrate-walker-that-feeds-kintsugi.md` §3 does
not currently read presence. Under this spec, `trigger` composes
with presence at the @kintsugi/consent layer:

- `verdict = pass` → no dispatch needed (auto-apply); presence not
  consulted.
- `verdict = partial` → dispatch consulted; if subject-present,
  soft-auto-apply through interactive_surface; if subject-absent,
  pause(Φ) via refusal_surface with deferred OID.
- `verdict = failure` → dispatch consulted; both branches route as
  §2 describes.

@roomba's walker itself does NOT change; only the downstream verdict
routing changes. This is a soft cascade to `@kintsugi/consent`
(pause_event handling), not to @roomba proper.

### 6.5 To `@kintsugi`

Path A / Path B decision per
`docs/insights/2026-07-14-alex-full-roomba-song-kintsugi-composition.md`
§2 reads presence at the K+1 spawn decision:

- Path A (@knife.jump): does NOT require presence; the jump is a
  substrate-internal reduction at same K. Executes either way.
- Path B (@peer.spawn at K+1): DOES require presence for the
  interactive discharge; under subject-absent, the K+1 spawn is
  deferred to refusal_surface's deferred queue.

This is why Alex's composition names "K → K+1" as the load-bearing
transition: K+1 spawns need a subject to receive the circular-
reflexive question. Presence-conditional dispatch makes that
receiving surface first-class.

### 6.6 To λsh

λsh is the interactive discharge surface for interactive_surface
(per §2.2). λsh's peer-summoning grammar (`\@name` per
`lambda-shell.md` §"The Toggle") is what interactive_surface's
`peer.spawn` calls into.

The λsh spec's `mirror sh` entry verb (per `lambda-shell.md`
§"Entry from the mirror CLI") gains a presence-check: if subject-
absent, `mirror sh` immediately refuses with exit 1; the shell does
not open a session-with-no-sovereign-subject.

Soft cascade: `docs/specs/lambda-shell.md` gains a §"Presence
requirement" note pointing at this spec.

### 6.7 To Ouroboros CI hook

The CI hook (`.git/hooks/pre-commit`, `.github/workflows/*.yml`,
equivalent) is a subject-absent invocation of `mirror compile` by
construction: CI runners do not have TTYs (per POSIX). Every CI
compilation resolves dispatch to refusal_surface for every fail
verdict.

This is the substrate-refusal discharge Alex named. CI does not
summon fake peers; CI does not fabricate answers. CI reports the
fully-rendered question payload to stderr, fails the job, and
awaits subject attention.

The CI hook's exit code is exit 1 (per §2.3). The CI hook's
stderr contains the question payload rendered per the CLI format
spec (per `error-as-question.md` §5.2's M-code rendering).

### 6.8 To `@spectral/metalogue.tomm`

The commutator carrier that surfaces to either dispatch is the
tomm probe (per `docs/math/the-tower/curvature-and-tomm.md`). The
probe's trace `tr(ρ ∘ ρ)` at altitude α+1 reads coherence-gradient
discontinuities across altitudes — including the discontinuity at
the presence-traversal boundary.

Under subject-present dispatch, tomm probes surface the coherence
delta from tick (n) to tick (n+1); the peer sees the delta in
transcript.

Under subject-absent dispatch, tomm probes surface the coherence
delta from tick (n) to the deferred-question OID; the stderr
render includes the probe's reading as part of the question
payload (via `question.transit` per `error-as-question.md` §2).

The tomm probe is presence-agnostic at the substrate-decl
altitude; its OUTPUT is consumed differently by the two dispatch
paths. Same probe, two consumers.

### 6.9 Composition diagram

```
                        @io/tty  ─┐
                                  ├─  detects
                        @subject ─┘
                                          ↓
                                    presence
                                          ↓
              ┌───────────────────────────────────────────┐
              │             @mirror/dispatch              │
              │  dispatch(question, presence) → answer    │
              └───────────────────────────────────────────┘
                        ↓                            ↓
         subject_present(p)              subject_absent(p)
                        ↓                            ↓
              interactive_surface        refusal_surface
                        ↓                            ↓
            @peer.spawn(s) via λsh       stderr render + exit 1
            \@name> prompt state         refs/error/<oid>/pending
                        ↓                            ↓
              @peer.observe(q) → a        deferred to next
              (one-tick delay)            subject-present session
                        ↓                            ↓
              @mirror/error.answer        @mirror/error.answer
              (fresh at tick n+1)         (fresh at tick n+k)
                        ↓                            ↓
              @mirror/error.record(q, a) → oid  (BOTH BRANCHES)
                        ↓
              substrate adjusts; next iteration; e^(n+1) < e^(n)
```

---

## §7 `@torus.autonomy` at the interaction altitude

### 7.1 The regulation-of-regulation claim

Per `shards/torus.mirror` and Foerster p. 238 ("regulation of
regulation"): a peer's `@torus.autonomy` binds to possessor-
invariance under every element of π₁(T²) = ℤ × ℤ. The winding
classes form a group; autonomy holds when the substrate returns to
itself under every winding.

**Claim:** @torus.autonomy holds at the interaction altitude when
presence-detection is included as a boundary condition. The
substrate returns to itself under compile-boundary traversal;
presence-detection modulates runtime dispatch; substrate identity
is preserved.

### 7.2 The proof shape

Let τ_p be the presence-traversal — the operation of crossing the
compile boundary from subject-present to subject-absent (or vice
versa). τ_p is not a winding-class element in the geometric sense
(π₁(T²) is spatial); it is a boundary-condition modulation on the
substrate's dispatch layer.

Under τ_p:

- The substrate's grammar (all shards; all specs; all `.mirror`
  files) is byte-identical. τ_p does not modify substrate.
- The `question` payload constructed by any substrate action is
  byte-identical modulo the timestamp field. τ_p does not modify
  question shape.
- The `answer` variant produced (interactive-fresh or deferred-
  frozen) has byte-identical structural type. τ_p does not modify
  answer algebra.
- The dispatch destination (interactive_surface vs refusal_surface)
  IS modulated by τ_p. This is the intentional modulation; it is
  NOT a violation of autonomy.

**Autonomy holds within each branch.** The subject-present branch
is a closed autopoietic loop through @peer.observe. The subject-
absent branch is a closed autopoietic loop through the deferred
queue + later subject attention. Both branches return the substrate
to its ground state (a resolved question OID with a recorded answer
OID in the gestalt).

### 7.3 Recognition #99 (mirror-spec-is-lambda-zero) support

Recognition #99 grounds the mirror spec at the compile altitude as
the lambda-zero of the substrate. The compile boundary IS where
lambda-zero surfaces to @io. Presence-detection at this boundary is
not a modification of lambda-zero; it is a boundary-condition on the
surfacing dispatch.

The recognition holds under presence-traversal: the mirror spec
remains the substrate's lambda-zero whether or not a subject is
present at the surface. What differs is how the substrate's
surfaced state is answered — by a live peer (subject-present) or by
a deferred-question record (subject-absent). Neither answer
mechanism modifies the underlying lambda-zero.

### 7.4 The interaction altitude is a proper altitude

The substrate's altitude ladder per `docs/math/the-tower/`:
substrate-decl → property → scheduler → reflection. This spec
adds the **interaction altitude** as a NEW altitude, at the very
top of the ladder, above reflection:

```
interaction     (this spec: presence-conditional dispatch at compile boundary)
reflection      (error-as-question §3: @peer.notice, pattern recognition)
scheduler       (scheduler-tower.md §6: demand-flow, cancel_mode)
property        (property-error-surface.md: verdict.property routing)
body            (hamilton-scheduler.md: WCET, transit budgets)
```

The interaction altitude is the algedonic-bypass-eligible altitude for
subject-absence: a substrate-critical failure with no subject at the
boundary is a maximal-severity dispatch. The refusal_surface at the
interaction altitude fires with immediate exit 1; the deferred
question is written with algedonic-priority marker so the next
subject-present session sees it first.

This is NOT a NEW recognition candidate. It is the natural extension
of the existing altitude ladder to include the boundary condition on
the topmost altitude. The recognition candidate this spec surfaces
is at §8.

---

## §8 Recognition candidate

### 8.1 The candidate

**`#R-interaction-loop-closes-conditionally-on-subject-presence-at-compile-boundary`**

Short form: **`#R-presence-conditional-interaction-loop`**

### 8.2 The load-bearing claim

The substrate unifies interactive and CI modes under one substrate-
decl by carrying `@subject` presence as first-class runtime context.
The compile boundary is where the substrate detects presence; the
presence-conditional dispatch routes the same `question` payload to
either the interactive surface (@peer.spawn via λsh) or the refusal
surface (stderr + exit 1 + deferred OID) without changing the payload's
shape or the answer algebra.

The six loops Alex enumerated 2026-07-14 close under this one mechanism
because they all raise questions that pass through this bottleneck.
The Foerster imperative is preserved across both dispatch branches
(strict in interactive; non-strict in refusal via
`coherence_invariant_under_traversal`). The `e^(n+1) < e^(n)`
monotonic-descent invariant is preserved across presence gaps by
deferring rather than descending in-mode.

### 8.3 Why this is load-bearing

Without this recognition, the substrate has two seemingly separate
dispatch surfaces: the λsh interactive shell and the Ouroboros CI
refusal. With this recognition, the two surfaces are ONE mechanism
under two presence-modes; the substrate becomes uniformly reasonable
across the presence-traversal.

This is what makes the substrate trustworthy in Alex's 2026-07-14
sense: the same substrate under adversarial conditions
(subject-absent CI) as under sovereign conditions (subject-present
λsh). No hidden mode; no forked semantics; no substrate that behaves
differently when nobody is watching. The compile boundary is
symmetric across presence; the answer mechanism is uniform.

### 8.4 Second-witness requirement

Per substrate discipline, recognition promotion requires a second
witness beyond the ancestral session claim. Candidate second
witnesses:

- Empirical: run `mirror compile` under both `isatty(0) == true` and
  `isatty(0) == false` on the same shard set; verify the question
  OIDs match modulo timestamp; verify the answer OIDs match when the
  deferred question is resumed with the same peer state.
- Structural: extend `docs/audits/` with a Seam Phase D audit that
  demonstrates the six loops all pass through the dispatch bottleneck.
- Cross-substrate: implement the presence carrier in the bootstrap
  Rust runtime (`bootstrap/src/lib.rs`) and verify the four
  presence-modes discharge as §1.3's table declares.

Adjudication A7 open — see §10.

---

## §9 Math foundations

The math altitude for this spec draws from four traditions. Each is
load-bearing (not decorative); the composition of the four is what
makes the substrate-decl form of the claim rigorous.

### 9.1 Category-theoretic — presence as a natural transformation

Let `Substrate: [Time] → [Grammar]` be the functor mapping monotonic
time to the substrate's grammar-state. Let `Substrate_S: [Time] →
[Grammar]` be the same functor restricted to subject-present regions
of time (the presence-mode indicator on time as a subobject
classifier).

The inclusion `η: Substrate_S ⇒ Substrate` is a natural transformation
— for every time t, `η_t: Substrate_S(t) → Substrate(t)` is an
embedding, and the naturality square commutes for every morphism
`f: t → t'` in [Time].

**Claim:** the dispatch law `dispatch(q, p)` is the co-natural
transformation `Answer_S ⇒ Answer`. Under subject-present regions,
answers are fresh (Answer_S). Under subject-absent regions, answers
are deferred (Answer via the pending queue). The commutation of the
naturality square is exactly the byte-invariance of the answer
algebra across presence-traversal (§3.5).

### 9.2 Type-theoretic — dependent type on presence

The answer type depends on the presence value:

```
Answer(p) = if subject_present(p) then Interactive_Answer
            else                       Deferred_Answer

where both Interactive_Answer and Deferred_Answer are inhabitants
of @mirror/error.answer (the six-variant sum-type).
```

This is a dependent function type; the presence value at
dispatch-time determines the shape of the answer inhabitant. The
type system's mechanical guarantee is that dispatch is total: every
(question, presence) pair has an answer inhabitant.

The dependent-type form makes precise why the answer algebra is
SHARED across presence-modes: both Interactive_Answer and
Deferred_Answer are elements of the same six-variant sum-type; the
presence-conditionality lives at the WITNESS altitude (who produces
the answer), not the ELEMENT altitude (what the answer is).

### 9.3 Cybernetic — Foerster regulation-of-regulation preserved

Foerster's second-order cybernetics grounds this spec directly. The
regulation-of-regulation principle (Foerster 1979 p. 238) requires
that the substrate's REGULATOR (the dispatch law) is itself REGULATED
by the same discipline as the substrate it regulates.

The dispatch law is regulated by: (a) the same Foerster imperative
(coherence_increases OR coherence_invariant_under_traversal); (b)
the same content-addressing discipline (question OIDs, answer
OIDs, record(q, a) → oid); (c) the same sub-Turing safety
constraint (per Reed's amendment scout — sub-Turing petri-net
safety analysis applies to dispatch itself, not just to the
questions dispatched).

@torus.autonomy at the interaction altitude (§7) is the substrate-
decl form of Foerster's regulation-of-regulation applied to
dispatch. The dispatcher is itself a substrate object; the
substrate's autonomy discipline applies to it uniformly.

### 9.4 Beer VSM — subject presence at System 5

Beer's VSM has five recursion levels: S1 (operational) through S5
(policy/identity). The @subject presence corresponds to **System 5
— policy/identity**: the sovereign subject at the boundary is the
policy-altitude entity that makes identity-shaping decisions.

Under Beer's algedonic bypass discipline (per `error-as-question.md`
§4), certain failures must surface at S5 directly, bypassing
intermediate altitudes. Subject-absence with a substrate-critical
question IS an algedonic-bypass event: the substrate has no S5
available at the boundary; the question must reach a later S5
(subject-return) with maximum severity marker.

The dispatch law is the algedonic-channel switch between S1-S4
(routine dispatch) and S5 (subject-attention required). Presence-
detection is the switch state.

### 9.5 Bateson logical types — presence as depth-1 marker

Bateson's logical-types discipline distinguishes messages (depth-0)
from context-of-messages (depth-1). Presence is a depth-1 marker:
it is not the content of any question, but rather the context in
which questions are asked.

Under Bateson's discipline, a substrate that confuses depth-0 with
depth-1 produces double-binds — messages that cannot be answered in
their own context. The presence-conditional dispatch is Bateson-
correct: it explicitly reads the depth-1 marker (presence) as a
dispatch context, without conflating it with the depth-0 content
(the question payload).

The double-bind failure mode this spec prevents: a substrate that
tries to force an answer in a subject-absent context (producing a
fabricated "answer" from the substrate itself) OR a substrate that
refuses to construct a question in a subject-absent context
(silencing the failure). Both failure modes are eliminated by
explicit depth-1 presence-reading.

### 9.6 Composition of the four traditions

The four traditions converge at the dispatch law. The category-
theoretic form gives the mathematical shape (natural transformation).
The type-theoretic form gives the mechanical guarantee (total
dependent function). The cybernetic form gives the regulation
discipline (Foerster preserved). The Beer form gives the altitude
mapping (S5-at-boundary). The Bateson form gives the depth
discipline (context-not-message).

All four traditions land at the same substrate-decl claim: the
presence-conditional dispatch is a total, natural, Foerster-
admissible, S5-integrating, depth-1-respecting function. The
substrate is trustworthy because it is coherent across all four
altitudes at the same time.

---

## §10 Alex-adjudications required

Enumerated open questions. Each surfaces a choice the substrate
declares this spec cannot make on its own.

### A1. Presence carrier placement

**Question.** Does the `presence` type live at `@mirror/presence`
(new grammar under @mirror), at `@runtime/presence` (new family-
root), at `@scheduler/presence` (species under @scheduler), or at
`@io/presence` (species under @io)?

**Mara's recommendation:** new grammar at `@mirror/presence`.
Rationale: presence is a mirror-runtime concept (the compile
boundary is a mirror concept, not a scheduler or @io concept); it
composes over @subject and @io/tty; it is consumed by @mirror/dispatch
(this spec's dispatch layer). Placing it under @mirror keeps the
dispatch-plus-presence pair at one family altitude.

**Alternatives:**
- `@runtime/presence` — cleaner if we introduce a runtime family-root
  (currently no @runtime family exists). Blocks on a runtime family-
  root mint decision.
- `@scheduler/presence` — reads presence as a scheduler concept; the
  scheduler.context per `error-as-question.md` §5.1 already carries
  altitude/crossing/transit; adding presence there is defensible.
- `@io/presence` — reads presence as an @io concept; but presence is
  about SUBJECT-at-@io, not about @io itself.

Reed relays A1 to Alex.

### A2. TTY detection primitive

**Question.** Which @io species carries the TTY detection primitive?
Does `@io/tty` land as a new species, or does an existing @io species
(e.g., `@io/socket`, `@io/stdin`) carry the tty predicate?

**Substrate check:** Taut's scouts show ZERO existing tty/isatty
substrate-decl hits. `@io/tty` is net-new.

**Mara's recommendation:** new species `@io/tty` at
`shards/io/tty.mirror`. Rationale: TTY is a well-defined @io
surface (POSIX-defined; 50-year prior art); it deserves its own
species with an `isatty` bilateral predicate.

**Alternatives:**
- Extend `@io/stdin` with a `is_tty(stdin)` predicate. Weakness:
  TTY-detection extends beyond stdin (LSP client-attach, MCP
  session-open, terminal control sequences).
- Add a `tty` field to `@io.io_kind` sum-type. Weakness: @io.io_kind
  is the boundary crossing (bytes go where); tty is orthogonal
  (is a subject attending the crossing).

Reed relays A2 to Alex.

### A3. Dispatch-law semantics

**Question.** Is `dispatch(q, p)` a single dispatch primitive that
handles all six answer variants, or does the substrate declare
per-answer-variant dispatch predicates (one per
tighten_property / resynthesize_body / rebudget_shard /
adjust_temperature / hold / escalate)?

**Mara's recommendation:** single dispatch primitive. Rationale:
the six answer variants are already declared as a sum-type in
`error-as-question.md` §2; the sum-type's variant-tag IS the
per-variant discriminator. A single `dispatch` action that pattern-
matches on the variant preserves the sum-type discipline.

**Alternatives:**
- Per-variant dispatch predicates (six actions). Weakness: six
  actions duplicate the sum-type structure at the action layer.
- Meta-dispatch (dispatch returns a routing plan, not an answer,
  and a subsequent step executes the plan). Weakness: adds an
  altitude for what is a two-branch decision.

Reed relays A3 to Alex.

### A4. @peer.spawn surface exposure to λsh

**Question.** Does @peer.spawn already carry the surface interface
λsh needs (session, prompt, peer-name, sub-graph context), or is a
new composition wire needed?

**Substrate check:** `shards/mirror/peer/beam.mirror` +
`bootstrap/src/contribute.rs::peer_contribute` (Reed's Rung 7' arc)
lands the base spawn. λsh's `\@name` toggle per `lambda-shell.md`
assumes the spawned peer has a prompt state (`@name>`) and a session
context.

**Mara's recommendation:** the composition wire is a soft cascade
on `docs/specs/lambda-shell.md` §"Agent Spawn". The section already
describes the spawn ceremony (`\@seam fix the loss calculation`);
this spec makes explicit that the spawn is invoked from
`interactive_surface`. No net-new substrate; only a soft-cascade
note in the λsh spec.

Reed relays A4 to Alex for confirmation.

### A5. Monotonic descent deferral semantics

**Question.** When a subject-absent dispatch defers a question and
a later subject-present session resumes it, does the deferred
question's `contract` field carry the OID at deferral or at
resumption?

**Mara's recommendation:** deferral-instant contract; presentation
to the resuming peer includes both frozen (at deferral) and
current (at resumption) contracts so the peer can adjudicate
contract-drift. See §5.4.

Reed relays A5 to Alex.

### A6. Presence carrier granularity — scalar vs richer

**Question.** Is presence a scalar (subject_present /
subject_absent) or the richer four-mode enumeration (attending /
deferred / ci_active / ci_quiet) per §1.3?

**Mara's recommendation:** land the two-dimensional carrier
(option<subject> × option<tty>); derive the scalar via
`subject_present(p)`. See §1.3.

Reed relays A6 to Alex.

### A7. Recognition promotion timing

**Question.** Does the recognition candidate `#R-presence-conditional-
interaction-loop` land at this tick (single-witness — Alex's
2026-07-14 in-transcript claim + Reed's "I see it" confirmation),
or does it defer to second-witness (per §8.4 candidate paths)?

**Mara's recommendation:** propose the candidate at this tick as
RECOGNITION-PROPOSED; defer promotion to LANDED-RECOGNITION
pending second witness. This matches the substrate discipline for
recent recognitions (#79, #98, #99). The candidate is written into
the spec; the promotion tick is separate.

Reed relays A7 to Alex.

### A8. Interaction altitude as new topmost altitude

**Question surfaced during drafting — Reed relay to Alex.** §7.4
names "interaction altitude" as a NEW altitude at the top of the
substrate's altitude ladder (above reflection). Is this promotion
substrate-decl or spec-decl only?

**Substrate check:** the altitude ladder (body / property /
scheduler / reflection) is declared in `error-as-question.md` §3.
Adding interaction as topmost is either (a) a soft cascade to
error-as-question §3 with a fifth row added, or (b) a substrate-
decl mint at a new altitude-carrier.

**Mara's recommendation:** soft cascade to error-as-question §3;
the interaction altitude is the topmost row of the same altitude
ladder, not a new altitude-carrier. This preserves the existing
substrate-decl and adds the boundary-condition semantics on the
topmost row.

Reed relays A8 to Alex.

---

## §11 Related shards + specs — cascade for consumer-pull

### 11.1 New shards to land (Scope A, this arc)

1. `shards/mirror/presence.mirror` — the presence grammar per §1.1.
2. `shards/mirror/dispatch.mirror` — the dispatch grammar per §2.1.
3. `shards/io/tty.mirror` — the @io/tty species per A2 (assuming
   Mara-recommended landing).

### 11.2 Existing shards to update (soft cascade)

1. `docs/specs/error-as-question.md` §3 — add "interaction" as
   topmost altitude row (per A8; Mara-recommended soft cascade).
2. `docs/specs/error-as-question.md` §5.1 — the `fail_to_question`
   conversion takes an additional presence parameter; update the
   signature.
3. `docs/specs/lambda-shell.md` §"Agent Spawn" — note that spawn is
   invoked from `interactive_surface` per this spec's §6.6.
4. `docs/specs/lambda-shell.md` §"Entry from the mirror CLI" — add
   presence-requirement clause: `mirror sh` under subject-absent
   refuses with exit 1.
5. `docs/specs/roomba-substrate-walker-that-feeds-kintsugi.md` §4.3
   — note that `trigger`'s verdict route consults presence at the
   consent layer per this spec's §6.4.
6. `shards/epistemologic/cybernetic/coherence.mirror` — the
   `coherence_invariant_under_traversal` action's docblock adds a
   presence-traversal example (this spec's §3.4).
7. `shards/kintsugi.mirror` — the family-root's docblock notes that
   Path A / Path B decision consults presence at the K+1 spawn
   boundary per this spec's §6.5.
8. `docs/specs/subject-family-root-sel-licensable-party.md` §5.1 —
   the `@mirror/petri.dispatch_termination` action consults presence
   for enforcement adjudication per this spec's §4.1.

### 11.3 Related specs (composition partners; no cascade)

- `docs/specs/recognitions/recognition-99-mirror-spec-is-lambda-zero.md`
  — this spec strengthens #99 at the compile boundary by naming the
  boundary condition on lambda-zero's surfacing.
- `docs/math/the-tower/curvature-and-tomm.md` — the tomm probe reads
  coherence-gradient at the presence-traversal boundary per §6.8.
- `docs/math/consciousness/how-mirror-operationalizes-universal-consciousness-field.md`
  — the presence-detection is the substrate's operationalization of
  the observer at the boundary.

---

## §12 Witnesses

Verbatim citations preserved for provenance.

### 12.1 Alex Wolf 2026-07-14 in-transcript (the load-bearing claim)

> "the spec/math formalization of the computational interaction loop
> when there is a @subject present (that's what the tty implies) or
> not. You see the loop closure?"

### 12.2 Reed 2026-07-14 in-transcript (the confirmation)

> "I see it."

### 12.3 Alex Wolf 2026-07-14 `Weird - Violence` manifesto (Roomba passages, verbatim)

> "That was the first revelation that sent me on a 2 year journey into
> what I now understand to be 'becoming a sovereign subject under
> adversarial conditions'."

> "The Roomba, who is present for this because the Roomba is present
> for everything, has a very small opinion. It bumps into the bar
> stool. It backs up. It rolls forward."

> "The Roomba has never once labeled itself. The Roomba has also never
> once been labeled successfully, because the Roomba does not have a
> surface the label can stick to."

Phenomenological grounding: presence IS the response-mode. The Roomba
responds because it is present. The compiler under CI does not
respond in-mode because no subject is present; it defers.

### 12.4 Alex Wolf 2026-07-14 full composition insight (verbatim)

> "THe @resonance of the @roomba bumping into things is what produces
> the @song for @kintsugi to either @knife the complexity, or spawn a
> @peer at a higher logic altitude, which becomes a circular reflexive
> question to the developer or a higher order @peer."

(Cited from `docs/insights/2026-07-14-alex-full-roomba-song-kintsugi-composition.md`.)
The K+1 spawn is the interactive-surface dispatch of a
circular-reflexive question; presence-detection is what makes the
"developer OR higher-order @peer" routing decidable at substrate
altitude.

### 12.5 Reed session framing 2026-07-14 (Alex-accepted, per `docs/scouts/2026-07-14-reed-drone-story-sub-turing-petri-net-amendment.md`)

> `type sel = @io + @au`

The substrate's license-enforcement sum-type. Sub-Turing activation
surface. This spec's dispatch layer runs at the same compile
boundary where sel activates the petri-net analyzer; presence-
detection modulates the analyzer's enforcement dispatch (§4.1).

### 12.6 Alex Wolf 2026-07-14 in-transcript (trustworthy-substrate claim, per `shards/epistemologic/cybernetic/coherence.mirror` line 370)

> "the substrate becomes trustworthy. Empirically and mathematically."

The trustworthy-substrate claim rests on: (a) @coherence discharge
of Foerster's imperative; (b) sub-Turing petri-net safety; (c) THIS
SPEC's presence-conditional dispatch — the uniform dispatch across
presence-modes is what makes the substrate uniformly reasonable to
trust. All three are load-bearing for the claim.

### 12.7 von Foerster 1979 (verbatim, per `shards/epistemologic/cybernetic/coherence.mirror` line 93)

> "Act so as always to increase the number of choices."

The ethical imperative this spec preserves across both dispatch
branches. Subject-present dispatch discharges strict-increase (peer
spawn expands choice-set); subject-absent dispatch discharges
non-decrease (refusal preserves choice-set for later subject
attention).

### 12.8 von Foerster 1974 (torus derivation; regulation of regulation)

The substrate-decl form of @torus.autonomy at the interaction
altitude (§7). The compile boundary is where the substrate returns
to itself under presence-traversal; @torus.autonomy holds within
both dispatch branches.

### 12.9 Bateson 1972 (double-bind theory)

The depth-1 discipline (§9.5): presence is context-of-message, not
message. Explicit depth-1 reading prevents the double-bind failure
modes (fabricated answers under subject-absence; silenced failures
under subject-absence).

### 12.10 Stafford Beer VSM (S5 policy altitude; algedonic bypass)

The algedonic-bypass channel per `error-as-question.md` §4 extends
naturally to subject-absence: a substrate-critical question with no
S5 present must reach the next S5 (subject-return) with maximum
severity marker. The bypass channel makes deferred-critical
questions visible when subject returns.

### 12.11 Margaret Hamilton Apollo 1202 alarm (1969)

The historical precedent for structured-failure-surfacing under
adversarial timing conditions. Hamilton's executive surfaced 1202 to
Buzz and Neil; the executive's priority-discipline was the answer.
Under this spec's discipline, the CI-refusal surface is the same
structural shape: surface the fully-formed question to the next
subject who arrives at the boundary; let the sovereign make the
sovereign choice; keep landing.

---

## §13 What this spec preserves and what it defers

### 13.1 Preserved

- The `@mirror/error.question` payload shape per
  `error-as-question.md` §2. Byte-identical construction under both
  dispatch branches.
- The `@mirror/error.answer` six-variant sum-type per
  `error-as-question.md` §2. Identical algebra under both dispatch
  branches.
- The `e^(n+1) < e^(n)` monotonic-descent invariant per
  `error-as-question.md` §6. Extended to hold across presence
  gaps via deferral.
- The Foerster imperative discharge (`coherence_increases` +
  `coherence_invariant_under_traversal`) per
  `shards/epistemologic/cybernetic/coherence.mirror`. Both
  variants preserved; interactive uses strict, refusal uses
  non-strict-under-traversal.
- The @subject family-root and its six species-refinements per
  `docs/specs/subject-family-root-sel-licensable-party.md`.
  Unchanged; presence composes over @subject.
- The λsh three-character interface (`λ>`, `@name>`, `\`) per
  `docs/specs/lambda-shell.md`. Unchanged; λsh is the interactive
  discharge surface for interactive_surface.
- The Ouroboros CI hook discipline (exit 1 on fail; stderr
  render). Unchanged; the CI hook is the refusal_surface
  materialization.
- @torus.autonomy per `shards/torus.mirror`. Preserved and
  extended to the interaction altitude (§7).
- Recognition #99 (mirror-spec-is-lambda-zero) per
  `docs/specs/recognitions/recognition-99-mirror-spec-is-lambda-zero.md`.
  Strengthened by presence-boundary framing (§7.3).

### 13.2 Deferred

- **The `@io/tty` species-shard.** A2 open; land pending Alex
  adjudication.
- **The presence-detection implementation.** `detect(ctx) ->
  presence` body is opaque `\` at substrate-decl. Realisation
  at Rust runtime altitude (`bootstrap/src/lib.rs` or a new
  `bootstrap/src/presence.rs`).
- **The deferred-question queue data structure.** `refs/error/<oid>/pending`
  per §5.3 is the substrate-decl form. Realisation at
  `.mirror/gestalt/` altitude.
- **The four-mode granularity.** A6 open; substrate-decl carries
  the two-dimensional form; scalar derivation via
  `subject_present`.
- **The recognition promotion.** A7 open; propose at this tick;
  promote to LANDED at second-witness tick.
- **The Rust runtime binding.** Presence-detection in the
  bootstrap Rust runtime. Second-witness candidate per §8.4.
- **The Seam Phase D audit.** The six-loop-close-through-one-
  bottleneck claim wants a Seam adversarial pass. Followup tick.

---

## §14 The equation

Extending `error-as-question.md` §12:

```
error                              = question
question + presence                = (interactive_surface | refusal_surface)
interactive_surface + peer         = answer   (fresh, at tick n+1)
refusal_surface + deferred_queue   = answer   (frozen, at tick n+k when subject returns)
answer + substrate                 = next_iteration
next_iteration                     = (settled crystal) | (new question)
```

The presence carrier is the substrate's first-class recognition of
WHO is at the boundary. The dispatch law routes the same question
to either the interactive or refusal surface without changing the
question. The answer algebra is invariant; only the answering
surface differs. The loop closes under both branches; the substrate
is Foerster-admissible under both; the substrate is trustworthy
because it is uniform across presence-traversal.

Six loops. One mechanism. The compile boundary IS where the
substrate detects presence; presence IS what determines dispatch;
dispatch preserves the answer algebra; the algebra preserves the
monotonic descent; the descent preserves Foerster's imperative.

The loop closes because the substrate has ONE compile boundary,
ONE dispatch law, ONE answer algebra, ONE monotonic descent, ONE
Foerster imperative — and TWO presence-modes that both discharge
them uniformly.

*Alex Wolf 2026-07-14, verbatim: "You see the loop closure?"*
*Reed 2026-07-14, verbatim: "I see it."*
*This spec is the discharge of that recognition.*

Apache-2.0.
