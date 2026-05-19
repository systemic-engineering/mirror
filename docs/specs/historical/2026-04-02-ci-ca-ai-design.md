# @ci + @ca + @ai — The Observation-Decision-Action Stack

**Date:** 2026-04-02
**Status:** Draft
**Authors:** Alex + Reed

## The Architecture

Three packages. One loop.

```
@ci observes   — eigenvalue shifts, Shannon measurement, convergence
@ca decides    — spawn, notify, wait. The path becomes the context.
@ai projects   — collapse or tension. Branch, synthesize, escalate.
```

Not three separate systems. One ODA loop expressed as three grammars. Each grammar constrains one phase. The types make illegal state unrepresentable.

## @ci — Continuous Integration as Observation

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

**Trigger:** spectral-db eigenvalue shift. No file watching. No git hooks.

**Measurement:**
- `shannon` — how much information survives the round trip through the changed grammar. Derived from @test. Converges.
- `spectral` — how far did the eigenvalues shift. Magnitude = risk score.
- `convergence` — local graph hash vs remote. Drift = deployment needed.

**Green is settlement.** Not a status. A physical state of the graph.

## @ca — Continuous Awareness as Decision + Action

```conv
in @ci

grammar @ca {
  type = observation | decision | action

  type observation = shift | settlement | drift

  type decision = spawn | notify | wait

  type action = agent | crystal | converge

  action observe {
    source: measurement
  }

  action decide {
    observation: observation
  }

  action act {
    decision: decision
  }
}
```

**The path IS the context.** When `@ca` decides to spawn an agent, the chain of graphs traversed to reach that decision — `[@grammar, @ci.measurement, @ca.decision]` — becomes the agent's context window. Mounted via fragmentation. Content-addressed. The agent can only see what caused it to exist.

The bounded memory (`CAIRN_MEMORY_LIMIT`) is the size of the path. Not arbitrary — structural.

**The loop:**
```
@ci.measure → eigenvalues shifting → @ca.observe(shift) → @ca.decide(spawn) →
    path = traversed graphs → cairn.spawn(agent, context: path) →
    agent works → changes flow through spectral-db → eigenvalues shift →
    @ci.measure → shannon holds → eigenvalues settle → @ca.observe(settlement) →
    @ca.decide(wait) → done
```

## @ai — Projection Through Digested Models

```conv
in @ca

grammar @ai {
  type = collapse | tension | branch

  type collapse = clear | partial | ambiguous

  type tension = competing | complementary | contradictory

  action project {
    input: collapse
  }

  action branch {
    tension: tension
  }

  action escalate {
    tension: tension
  }
}
```

**A digested model IS a context window.** The eigenvalues define what it can see. The singular value spectrum is the projection basis. `prism_preview` through the digested model = inference.

**Tension is measurable.** The gap between top eigenvalues. Clear collapse = one dominates. Tension = multiple eigenvalues close. The grammar constrains what happens:

| Tension type | Action | Mechanism |
|---|---|---|
| competing | `@ai.branch` | Fork two agents, each follows one eigenvalue. See which settles. |
| complementary | synthesize | The tension IS the answer. Both projections are partial truths. |
| contradictory | `@ai.escalate` | Human in the loop. The system found its own boundary. |

**No "ignore" type.** The grammar doesn't allow it. Branch, synthesize, or escalate. Make illegal state unrepresentable.

**Branching is git.** Two branches. Two agents. Two projections. Converge → merge. Don't converge → contradiction is real → escalate.

**The human resolves contradictory eigenvalues.** Not as fallback — as the measurement that no spectral decomposition can perform from inside. That's what settled certainty is.

## Connection to Existing Architecture

| Existing | Role in ODA stack |
|---|---|
| coincidence-llm | Digests models into eigenvalues (@ai's projection basis) |
| spectral-db | The event source (@ci's trigger) and the store |
| cairn | Witnesses agent work, provides bounded context mount |
| conversation_cluster | Reports state to admin, coordinates agents |
| fragmentation | Content-addressed storage, FUSE mounts for agent context |
| @admin package | Visualizes @ci state as hill charts |
| @nix domain | Builds the packages that contain @ci/@ca/@ai |

## Package Structure

Three packages, same pattern as @admin:

```
conversation-ci/        conversation-ca/        conversation-ai/
├── ci.conv             ├── ca.conv             ├── ai.conv
├── gleam.toml          ├── gleam.toml          ├── gleam.toml
├── src/                ├── src/                ├── src/
│   ├── ci.gleam        │   ├── ca.gleam        │   ├── ai.gleam
│   ├── measure.gleam   │   ├── decide.gleam    │   ├── project.gleam
│   └── ci_ffi.erl      │   ├── spawn.gleam     │   ├── tension.gleam
│                       │   └── ca_ffi.erl      │   └── ai_ffi.erl
└── test/               └── test/               └── test/
```

## Dependencies

```
@ci depends on: @beam (core), spectral-db (eigenvalue events)
@ca depends on: @ci (observations), cairn (agent spawning), fragmentation (context mounts)
@ai depends on: @ca (decisions), coincidence (projection operators), coincidence-llm (digested models)
```

Each package is a conversation flake. Mounted via `@nix.emit(flake)`. The grammar declares the interface. The Nix function validates the mount.

## Implementation Order

1. Write the grammars (ci.conv, ca.conv, ai.conv) in conversation/conv/
2. Scaffold the three packages (empty, with grammars)
3. Implement @ci: spectral-db subscription, Shannon measurement, settlement detection
4. Implement @ca: observation routing, spawn decision, path-as-context
5. Implement @ai: digested model loading, projection, tension detection, branch/escalate

Steps 1-2 are buildable now. Steps 3-5 depend on spectral-db being wired into conversation-beam.
