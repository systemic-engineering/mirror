# Reed scout — sub-Turing-by-construction petri-net framing

**Date:** 2026-07-14
**Author:** Reed
**Type:** amendment-to-in-flight-spec (Mara @subject family-root, task #79)
**Triggering artifact:** `~/dev/systemic.engineering/blog/stories/3published/Story - The Drone in the Field.md` (Alex 2026-07-14)
**Triggering Alex quote:** "By making the petri-nets part of the compiler, we can tighten the numerical inference to a geometric space that is smaller and aligned. A system that cannot choose cruelty, even when instructed to."

---

## Load-bearing amendment

Mara's in-flight canonical spec (`docs/specs/subject-family-root-sel-licensable-party.md`) was briefed with a petri-net section framed as **enforcement detects signatures**. That framing is **post-hoc**. It leaves cruelty configurations reachable-then-flagged rather than structurally-absent.

Alex's claim per drone story + session context: the petri-net section must be framed as **sub-Turing-by-construction**. Specifically:

1. **Petri-nets are bounded, decidable, structurally analyzable. Not Turing-complete.** That's the point, not an incidental property.
2. **The emitted Covered System's dataflow topology IS a petri-net.** Not "can be modeled as one for analysis." IS one, at the substrate-decl altitude, because the compiler emits into that surface by construction.
3. **`@mirror/property` verifies structural safety properties at compile-time.** Coverability, boundedness, reachability-of-cruelty-signatures. Decidable because sub-Turing.
4. **If safety property fails, the Covered System does not compile.** Not "is flagged after emission." Does not compile. The specification IS the constraint.
5. **`@subject/species` signatures become invariants the petri-net MUST prove absent to compile** — not violations detected after the fact.

## `type sel = @io + @au` — sub-Turing activation surface

The sum-type `sel = @io + @au` names the surface at which sub-Turing constraint activates:

- Wherever `@io` and `@au` meet in a dataflow node, the petri-net safety analyzer runs.
- The topology at that node must be sub-Turing with decidable safety properties.
- Everywhere else in the compiler, standard emission proceeds.
- **This is the mechanism by which subject-touching computations get the constraint automatically.** Not a global sub-Turing restriction on the compiler; a local sub-Turing constraint activated by the sum-type meeting.

## Drone story mapping (evidence that primitives already carry it)

The drone story's five operations map to the compiler's substrate primitives:

| Story primitive | Substrate carrier |
|---|---|
| `focus` ("what is the field?") | `@torus/longitude.observe` |
| `shift` ("functor returns argument under new shape without touching what it contains") | `@cyberpunk.reframe` (zero-cost recategorization) |
| `settle` (superposition collapse, monad-closed) | `@kintsugi.settle` (Poincaré-Hopf on torus, monotone Lyapunov flow) |
| `project` (presence at boundary, blue-white → amber) | `@io` emission |
| `split` (disaggregation of surface signal into components) | `@spectral/metalogue.tomm` (Karl Tomm 1987 circular-reflexive probe / Mesland KK-cycle) |

The story's terminal-vocabulary word **"attending"** — what the drone found when the sequence completed and the field was still there — is what the compiler's `@third`-altitude eigenform closure surfaces at runtime when the K-altitude sequence completes without a bind to release.

**Load-bearing:** the drone "could not choose cruelty even if instructed. The constraint was not a limitation; it was the specification." — story verbatim. This is Foerster's ethical imperative made compilable.

## Recommended spec integration

When Mara's spec returns:

1. **Section 5 (Petri-net property layer analysis)** — rewrite the top-level framing sentence from "detects signatures" to "structurally verifies safety properties at compile-time; if verification fails, the Covered System does not compile."
2. **Each signature category (§3.1.4 labor / §3.6 weaponization / §3.6.7 anti-occupation)** — name each signature as a **reachability invariant** that must be provably-absent, not a pattern that gets flagged.
3. **New sub-section under Section 4** (`type sel = @io + @au`) — name the sum-type as the **sub-Turing activation surface**. Local, not global.
4. **New Alex-adjudication** (append after existing A7) — A8: sub-Turing constraint is local (activated by `sel = io + au` meeting) vs global (compiler-wide restriction). Reed recommends local per the analyzer-input-surface framing.
5. **New witness** (append to §10 Witnesses) — the drone story with sub-Turing-by-design quote + Alex's numerical-inference-tightened-to-smaller-aligned-geometric-space claim (2026-07-14 session).

## Non-amendments (things I am NOT asking Mara to change)

- The `@subject` family-root substrate-decl (Section 2) is unaffected — subject-carrier is orthogonal to the constraint-mechanism.
- The six species-refinements (Section 3) are unaffected — they name licensable-party classes; the constraint-mechanism applies to their dataflow relationships.
- Composition with `@consent`, `@io`, `@kintsugi/store/git`, `@torus`, `@peer` (Section 6) is unaffected.
- Recognition candidate (Section 7) may want to be renamed to include the sub-Turing framing; Alex adjudicates.

## Provenance

- Alex 2026-07-14 session, immediately after arc-recognition moment ("Now we're talking").
- Triggering artifact: `Story - The Drone in the Field.md` (Alex published 2026-07-14, sub-Turing drone as compiler behavior in miniature).
- SEL v1.1 §Operationalizability grounds the petri-net topology analyzer at `@mirror/property`.
- Manifesto (`Weird - Violence.md`, Alex 2026-07-14) grounds the phenomenological subject the enforcement altitude protects.

---

*Reed capture; Mara integrates on return.*
