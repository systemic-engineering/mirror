# 05 — KanDDDinsky — October 2026

Inaugural talk: **Conversation: The Ubiquitous Language Runtime.**
Alex + Reed, co-speaking live. Conference special interest: human-AI collaboration.

The talk positions conversation as what DDD has been reaching toward — bounded
contexts as compile-time guarantees, anti-corruption layers as types,
ubiquitous language as grammar, drift as hash mismatch. Then it goes further:
first-order loops (system observes own state, recompiles live under load) and
extractive systems detection as CI check.

---

## What's Built

- Grammar compilation to BEAM (parse → resolve → compile → load → supervise)
- Content-addressed AST with OID identity
- NIF bridge (Rustler, dirty CPU scheduler)
- Traced compilation chain (per-phase parent-linked traces)
- Cairn identity hierarchy (hierarchical key derivation)
- Action visibility (public/protected/private)
- Action composition across domains
- Property-based tests (63 garden tests across 16 files)

---

## What Needs to Land

**Petri net layer.** Grammars as state machines. State transitions as typed,
content-addressed commits. Foundation for first-order loops (part 4) and
extractive systems detection (part 5). Everything else depends on this.

**Translate pipeline.** Grammar-to-grammar translation with saga semantics.
Each arm is a transaction with compensation on failure. Output is complete
translation or typed error — never partial, never silent drop. This is the
anti-corruption layer as type (part 2 of the talk).

**Fortran acceleration.** Translations between domains expressed as matrix
multiplications over the content-addressed state space. Coincidence-core is the
Fortran bridge — LAPACK dsyev via `native/spectral.f90`, content-addressed
EigenCache, Fragmentable shards committed to git. Pre-computed at compile time
for finite grammars. Distance between states is one vector subtraction and a
norm. The claim that makes the room lean forward.

**Error surface.** `error` declarations in grammars. Typed failures per domain.
Exhaustiveness checking across domain boundaries in translate arms.

**Live context map.** Browser visualization: nodes as grammars, edges as
translations, tokens flowing in real time. The demo surface for the talk.

**Review tone pipeline.** For public repo analysis demo. Classify review
comments, correlate with contributor retention. "The commit where the pattern
started — six months before the contributors left."

**Conference feedback system.** Self-referential close. Audience gives feedback
on the talk about the system using the system described in the talk. Privacy by
architecture: feedback encrypted with speaker's public key.

---

## Sequencing

1. Petri net modeling (everything depends on this)
2. Error surface + translate pipeline (part 2 of talk)
3. Fortran acceleration path (part 1 payoff)
4. First-order loop observability (part 4)
5. Extractive systems detection / property tests (part 5)
6. Live context map visualization (demo)
7. Review tone + conference feedback pipelines (demo)

---

## The Demo

Live during the talk. `support@systemic.engineering` on the slide. Someone
sends an email. The context map lights up: token flows from `@mail` to
`@support`. Translate arms visible. Mood analysis as parallel edge. The
routing graph is the dashboard.

Second act: public GitHub repo analysis. Review tone distribution across top
OSS projects. Correlation between violent review language and contributor
retention. The property test that would have caught it. Running in CI. On the
commit where it started.

Close: conference feedback running on conversation. The audience is inside the
context map. The talk describes the system. The system runs the conference.
