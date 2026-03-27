# Projection: Properties as Plans

## What This Is

A spec for the spec system. The conversation runtime already has property
declarations (`requires`, `invariant`) and a model checker that evaluates them.
This spec closes two gaps: enforcement (properties that fail must fail
compilation) and projection (a content-addressed snapshot of declared properties
that can be model-checked before execution begins).

The result: a `.conv` grammar IS a spec. Properties ARE acceptance criteria.
The model checker IS CI. Implementation is whatever satisfies the properties.

---

## The Gap

**What exists:**
- `requires` and `invariant` declarations parsed in Rust, stored in TypeRegistry
- Compiled BEAM modules export `requires/0` and `invariants/0`
- Compiler actor reads declarations, calls `@coincidence.check_property` for each
- Four property evaluations: `shannon_equivalence`, `connected`, `bipartite`, `exhaustive`
- All evaluations go through real eigendecomposition (dsyev) or content-address checks
- `@property` grammar defines `requires | invariant | ensures` as types

**What doesn't exist:**
- Enforcement: `let _ = coincidence.check_property(...)` — results discarded
- `ensures` keyword: exists in @property grammar but not parsed by Rust compiler
- Projection: no content-addressed snapshot of expected property state
- Preview: no way to model-check a property set for satisfiability before execution
- Delta: no comparison between projected and actual property results
- Spec lifecycle: no grammar domain for the projection operations

---

## Design

### Layer 0: Enforcement (prerequisite)

The compiler actor must stop discarding property results. When a `requires`
property fails, compilation fails. When an `invariant` property fails,
compilation fails. `ensures` is different — it's a runtime property, checked
after execution, not at compile time.

Change in `compiler.gleam`: replace `let _ = coincidence.check_property(source, name)`
with result inspection. On failure, return compilation error with the property
name and the `@coincidence` verdict.

This is milestone 06's next step. Projection depends on it — you can't treat
properties as plans if violating them has no consequence.

### Layer 1: The `@projection` grammar

```
grammar @projection {
  type = projection | preview | delta

  type preview = satisfiable | unsatisfiable | partial

  type delta = converged | diverged

  action project {
    spec: grammar
  }

  action preview {
    projection: projection
  }

  action measure {
    projection: projection
    actual: grammar
  }
}

in @property
in @coincidence
in @compiler
```

Three types, three actions:

**`projection`** — a content-addressed snapshot of a grammar's declared
properties. Created by the `project` action. The OID is computed from the
property declarations themselves: `hash(requires_list, invariants_list,
ensures_list)`. Same properties = same OID. Different specs with the same
property set produce the same projection.

**`preview`** — the result of model-checking a projection for internal
consistency. Can the declared properties all be true simultaneously?
`satisfiable` means execution can begin. `unsatisfiable` means the spec
itself is contradictory — fix it before writing code. `partial` means
some properties can't be checked statically (runtime `ensures`).

**`delta`** — the comparison between a projection and actual results after
execution. `converged` means all properties hold. `diverged` means at least
one property failed. The delta is content-addressed: `hash(projection_oid,
actual_results)`. It records exactly which properties converged and which
diverged.

### Layer 2: The projection lifecycle

A spec goes through this cycle:

```
1. Write spec    — .conv grammar with types, actions, properties
2. Project       — content-address the declared properties → projection OID
3. Preview       — model-check: are properties satisfiable?
4. Execute       — any implementation that satisfies the properties
5. Verify        — compile result, check properties, produce verdicts
6. Measure       — delta between projection and actual verdicts
```

Steps 2-3 happen at the compiler level. The compiler can project and preview
before any implementation exists. This is the previewable gradient: you know
what must hold, you know whether it can hold, before anyone writes a line.

Steps 5-6 happen after implementation, through the same compilation loop.
The compiler produces property verdicts. The delta compares them against
the projection.

### What a spec looks like

Instead of a markdown plan with numbered tasks:

```
grammar @http_bridge {
  type = request | response | stream | chunk

  type request = chat | tags | health
  type response = ndjson | json | error

  action chat {
    messages: request
  }

  action stream {
    response: chunk
  }

  requires shannon_equivalence
  requires connected
  invariant no_orphan_types
}

in @actor
in @ai

---

test "request types" {
  @http_bridge.request has chat
  @http_bridge.request has tags
  @http_bridge.request has health
}

test "response types" {
  @http_bridge.response has ndjson
  @http_bridge.response has json
  @http_bridge.response has error
}
```

The grammar IS the spec. The types define the domain. The actions define the
interface. The properties define the acceptance criteria. The tests verify
the grammar itself is well-formed. The model checker verifies the properties.
Any implementation that compiles against this grammar and satisfies these
properties is a valid delivery.

### What the projection replaces

The projection replaces the alignment phase of a delivery cycle. Currently:
humans align through conversation, then issue imperative prompts. The alignment
is real but informal. Nothing in the runtime captures what convergence looked
like before execution started.

With projection:
- The spec IS the captured alignment
- The projection OID IS the convergence point
- The preview IS the feasibility check
- The delta IS the delivery assessment

The conversation must converge before the action fires. That was always true
in the consulting framework. Now it's structural.

### Projection delta as learning signal

Both the projection and the actual results are content-addressed. The delta
between them is information:

- Which properties were easy to satisfy? Which required unexpected paths?
- How far did actual output diverge from the projected direction?
- Over many specs: how accurate are projections? Where does alignment
  systematically miss?

The system learns its own projection accuracy. The ODA loop operating on itself.
The @training grammar already has the infrastructure for this — frontier miss
rate, phase detection, convergence tracking. Projection accuracy is just another
observable.

---

## Dependencies

### From milestone 06 (prerequisite)
- Property enforcement in compiler actor (fail on unsatisfied `requires`/`invariant`)
- Property verdicts in compilation trace (each check becomes a trace entry)

### From existing infrastructure
- `@property` grammar — already defines `requires | invariant | ensures`
- `@coincidence` grammar — already defines measurement actions backed by NIFs
- `@topology` grammar — already defines graph/spectrum/phase types
- Content addressing (OID) — already on everything
- Proof certificates — already track compilation facts

### New work for this spec
- `@projection` grammar — the domain defined above
- `ensures` keyword — parsed by Rust compiler alongside requires/invariant
- Projection emission — content-address property declarations
- Preview evaluation — model-check property satisfiability
- Delta computation — compare projected vs actual
- Projection server — domain server for @projection actions

---

## What This Does NOT Include

- **Projection history/analytics** — alignment accuracy over time. Next spec.
  Depends on fragmentation integration (session-as-branch) for storage.
- **Runtime ensures evaluation** — `ensures` is declared but not evaluated
  in this spec. Runtime property checking requires execution context that
  doesn't exist at compile time. The type is parsed and stored; evaluation
  comes later.
- **Automatic spec generation** — the actor ODA loop could generate specs
  from conversation. Not this spec. That's the Reed actor's job.
- **Multi-actor spec alignment** — multiple actors agreeing on a shared
  projection. That's the session-as-branch architecture (coincidence at
  the boundary).

---

## Properties This Spec Must Satisfy

The spec system specs itself. This spec declares:

```
grammar @projection_spec {
  requires shannon_equivalence    # all projection OIDs are unique
  requires connected              # all types reference each other
  invariant no_orphan_types       # every type is used
}
```

If the model checker can verify these properties on the `@projection` grammar
itself, the spec system is bootstrapped. The spec verifies itself through the
same mechanism it provides to others.

---

## Design Principle

Specs prescribe implementation — brittle, any deviation is a bug.
Properties prescribe truth — resilient, any path that satisfies them is valid.

The plan is not a spec. The plan is the properties that must hold. How they
hold is the implementation's problem. The model checker verifies the plan is
sound. The projection shows the direction. The execution follows the gradient.

The conversation must converge before the action fires.
