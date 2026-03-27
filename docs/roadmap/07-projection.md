# 07 — Projection: Properties as Plans

## Status: Design

A delivery cycle with an AI actor is an ODA loop. The conversation runtime
can formalize the alignment phase — the part where participants converge on
what "done" means — before any execution begins.

The plan is the properties. Not the implementation. The model checker verifies
properties. The implementation is whatever satisfies them.

---

## Insight

Current state: humans align through conversation, then issue imperative prompts.
The alignment is real but informal. Nothing in the runtime captures what
convergence looked like before execution started.

Projection makes this structural:

1. **Align on properties** — what must be true when this is done
2. **Emit a projection** — a vector projected onto the expected completion state
3. **Model check the projection** — are the properties consistent, satisfiable, decidable
4. **Execute** — any implementation path that satisfies the properties
5. **Verify** — model checker confirms properties hold on the result

The projection is a direction, not a destination. A gradient, not a contract.
The actual output can diverge in detail as long as the properties hold.

---

## Architecture

### Properties as plans

A plan is a set of properties declared in .conv grammar. The plan does not
prescribe implementation — it prescribes constraints. The model checker
verifies the constraints are satisfiable before execution begins.

This is TDD at the language level. Red phase: properties declared, not yet
satisfied. Green phase: implementation satisfies all properties. The grammar
enforces the ordering — you cannot skip to execution without the types checking.

```
in @project

type plan {
  requires connectivity
  requires shannon_equivalence
  invariant no_orphan_types
  ensures response_time < threshold
}
```

The plan and the test suite and the verification are the same artifact.

### Projection as vector operation

The conversation crate already supports vector projections. A projection
collapses a vector onto an axis — directional, lossy, informative.

Applied temporally: project the current state vector onto the expected
completion state. The projection is content-addressed (gets an OID). It
becomes a verifiable artifact in the store.

```
current state  ──projection──▶  expected properties
                                    │
                              model check
                                    │
                              satisfiable? ──▶ execute
                              unsatisfiable? ──▶ realign
```

### Projection delta as learning signal

Both the projection and the final result are content-addressed. The delta
between them is information:

- How far did actual output diverge from the projected direction?
- Which properties were easy to satisfy? Which required unexpected paths?
- Over time: how accurate are projections? Where does alignment systematically miss?

The system learns its own projection accuracy. The CA loop operating on itself.

---

## Connection to existing milestones

- **[06 — Model Checker](06-model-checker.md)**: properties infrastructure is built.
  Projection uses the same `requires` / `invariant` / `ensures` declarations.
  Enforcement (06's next step) is a prerequisite — properties must fail
  compilation before they can serve as plans.

- **[04 — Fortran Bridge](04-fortran-bridge.md)**: eigenvalue evaluation backs
  property checks. Projection verification goes through the same dsyev path.

- **[03 — Shipping](03-shipping.md)**: projections are content-addressed git
  objects. The projection, the plan, and the result all live in the same store.

---

## What's needed

### From milestone 06
- Property enforcement (fail compilation on unsatisfied property)
- Property results in proof certificate (trace entries for each check)
- Properties as grammar actions (definitions in .conv, not Rust match arms)

### New work
- `@projection` grammar domain — `project | preview | delta | alignment`
- Projection emission in compiler actor — after property declaration, before execution
- Delta computation — diff between projected and actual property satisfaction
- Alignment history — projection accuracy over time, stored in fragmentation

---

## Design principle

The plan is not a spec. The plan is the properties that must hold. How they
hold is the implementation's problem. The model checker verifies the plan is
sound. The projection shows the direction. The execution follows the gradient.

Specs prescribe implementation — brittle, any deviation is a bug.
Properties prescribe truth — resilient, any path that satisfies them is valid.

The conversation must converge before the action fires.
