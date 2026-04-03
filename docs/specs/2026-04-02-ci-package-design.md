# @ci Package Design Spec

**Date:** 2026-04-02
**Status:** Draft
**Authors:** Alex + Reed

## The Idea

CI is not batch validation. CI is continuous measurement. The test suite converges. The build settles.

`@ci` is a conversation package that runs inside the BEAM runtime. It doesn't watch files. It doesn't run on push. It subscribes to spectral-db eigenvalue shifts. When a grammar's eigenvalues change, `@ci` measures how much information survived. Settlement is green. Shift is red.

## Grammar

```conv
in @beam

grammar @ci {
  type = phase | result | measurement

  type phase = check | build | test | settle

  type result = green | red | shifting

  type measurement = shannon | spectral | convergence

  action measure {
    target: measurement
  }

  action settle {
    phase: phase
  }
}
```

## Architecture

```
spectral-db scheduler ticks
    ↓
eigenvalue of grammar node shifts
    ↓
@ci receives shift as spectral-db event
    ↓
@ci.measure(shannon) — run @test, measure information survival
    ↓
Shannon equivalence holds?
    ├── yes → result = green → eigenvalues settle → crystal forms
    └── no  → result = red  → eigenvalues keep shifting → no settlement
```

### The Trigger

No file watching. No git hooks. No cron. The spectral-db eigenvalue shift IS the event. When the scheduler ticks and a grammar node's spectral hash changes, that change propagates to `@ci` as a subscription event.

The grammar change happened because someone wrote `.conv`. The write went through spectral-db. The eigenvalue shifted. `@ci` noticed. That's the whole loop.

### The Measurement

`@ci` doesn't run "tests" in the traditional sense. It measures:

**shannon** — How much information survives the round trip through the changed grammar? The `@test` domain (from the Shannon equivalence insight) derives checks from the grammar's type surface. 100% Shannon equivalence = no information lost. The test suite doesn't grow forever — it converges as the grammar stabilizes.

**spectral** — How far did the eigenvalues shift? Small shift from a type rename = low risk. Large shift from a structural change = high risk. The magnitude of the spectral distance IS the risk score.

**convergence** — Does the local graph hash match the remote? Drift between local and fly.io = deployment needed. Convergence = in sync.

### Green Is Settlement

A green build is not a status. It's a physical state of the graph. The eigenvalues settled. The Shannon equivalence holds. The convergence check passes. No one declares it green — the graph declares itself settled.

A red build is also not a status. It's eigenvalues that keep shifting. Information is being lost. The graph hasn't found equilibrium. The CI loop keeps measuring until it settles or someone fixes the regression.

### Connection to Hill Chart

The CI state IS the hill position. Same measurement, different projection:

| CI | Hill Chart | Eigenvalues |
|---|---|---|
| shifting | uphill | moving |
| measuring | top of hill | stabilizing |
| green/settled | downhill | settled |
| crystal | shipped | crystallized |

The hill chart doesn't need a separate data source. It reads `@ci`'s measurement of the same spectral-db eigenvalues.

## Package Structure

```
conversation-ci/
├── ci.conv           — the grammar contract
├── gleam.toml        — package deps
├── flake.nix
├── src/
│   ├── ci.gleam          — @ci actor: subscribes to spectral-db shifts
│   ├── ci_ffi.erl        — FFI to spectral-db and conversation_cluster
│   ├── measure.gleam     — shannon, spectral, convergence measurements
│   └── settle.gleam      — settlement detection, crystal formation trigger
└── test/
    └── ci_test.gleam
```

## Dependencies

- **spectral-db** — the event source (eigenvalue shifts) and the measurement target
- **@test** — Shannon equivalence measurement (derived from grammar types)
- **conversation_cluster** — reports CI state to connected nodes / admin panel

No dependency on git hooks, GitHub Actions, or external CI services. The runtime IS the CI.

## Runtime Behavior

`@ci` is a GenServer (via Gleam OTP) that:

1. Starts as a supervised child of conversation-beam (when the @ci package is mounted)
2. Subscribes to spectral-db eigenvalue shift events
3. On shift: runs `@test.measure(shannon)` against the changed grammar
4. Publishes result (green/red/shifting) to conversation_cluster
5. If green: triggers `@ci.settle(phase)` which signals spectral-db to attempt crystallization
6. If red: keeps the eigenvalues in the shifting state, re-measures on next tick

The measurement is continuous. Not triggered by human action. The scheduler ticks. The graph evolves. `@ci` observes.

## What @ci Replaces

| Traditional CI | @ci |
|---|---|
| GitHub Actions YAML | `.conv` grammar |
| Runs on push | Runs on eigenvalue shift |
| Binary pass/fail | Continuous shannon/spectral/convergence |
| Tests grow forever | Tests converge |
| External service | Inside the runtime |
| Minutes to run | Ticks with the scheduler |
| Separate from the product | IS the product |

## Out of Scope (MVP)

- Deployment actions (that's `@nix` domain)
- Notification (that's `@admin` dashboard reading CI state)
- Multi-repo CI (single spectral-db graph first)
- Parallel measurement (sequential ticks first)
