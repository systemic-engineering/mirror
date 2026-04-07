# Quantum-native on classical hardware

*2026-04-07*

mirror is doing quantum computing on classical hardware through
incremental AI inference.

## The recognition

The Prism trait is not an analogy to quantum mechanics. It is the same
algebra, expressed in a runtime that happens to live on a CPU. The thing
that makes quantum semantics expressible on a CPU is *the AI's ability to
hold contradictory truths without collapsing them*. That is the
load-bearing primitive. Without an inference engine that can refuse to
collapse, every `|` would have to be either exhaustively expanded
(exponential blowup) or eagerly chosen (loss of superposition). With one,
each `|` is a held branch in a model's working state.

## The mapping

| Quantum mechanics    | mirror                                              |
|----------------------|-----------------------------------------------------|
| wave function        | `Beam<T>` (from the `prism` crate)                  |
| eigenvalue           | `Eigenvalues` (the `MirrorData` of a fragment)      |
| projection           | `GrammarProjection` / `Prism::project`              |
| measurement          | `Prism::refract` — the operation that crystallizes  |
| collapse             | `Crystal` — the post-refract value                  |
| superposition        | `\|` — the variant separator and the split operator |
| operator             | the `Prism` trait itself                            |

This is not a list of similarities. It is a list of *type identities*.
The Rust code already names them this way. Read `Shatter`'s impl in
`mirror/src/mirror_runtime.rs`: the associated types are literally
`Eigenvalues`, `Projection`, `Crystal`, `Convergence`, `Precision`. The
trait was designed against the quantum vocabulary because the quantum
vocabulary *is* the optic algebra. Two independent fields converged on
the same primitives.

## The implementation strategy

The chain of `|>` and `|` operators is the program. The runtime that
executes it is the BEAM:

- Each `|` becomes a spawned actor. Two branches → two actors. N
  branches → N actors. The branches run concurrently and hold their
  superposition through actor isolation.
- The actors are coordinated through `GenStage` (or `StagePlay` — name
  unconfirmed; the coordination layer is one of the two). Backpressure
  flows backward through the chain; values flow forward.
- The five fate models run *inside* the actors. Each actor is one
  inference step over the spectral features of its incoming form.
- `refract` is the actor message that collapses the held branches. The
  actor merges its inbound beams into one outgoing crystal.

This is what `spectral`'s `gen_prism` backend will emit. See
`spectral/docs/gen_prism.md` for the BEAM bytecode pattern; the document
preserves it from the deleted EAF emission code in mirror, against the
day spectral grows the runtime backend that consumes `.shatter` files
and produces gen_prism bytecode.

## Why the AI is the load-bearing piece

A classical compiler that hits `|` has two choices: enumerate or pick. A
quantum machine has a third: hold. *Holding without enumerating and
without picking* is what an inference engine does at every token — it
maintains a distribution over completions and only collapses when forced.
mirror borrows that capacity. The five fate models are the holding
mechanism. Their joint state across the chain is the wave function.

Without `fate`, the chain would have to be either fully resolved or
fully exploded. With `fate`, the chain stays in superposition until
`refract` is called. The crystal is the measurement.

## Cross-references

- `fate/src/lib.rs` — the five models, the `Features`, the deterministic
  selector. The "AI" half of the load-bearing piece. `Brainfuck` inference,
  hardcoded weights, bit-for-bit reproducible.
- `coincidence/src/hash.rs` — `CoincidenceHash<5>`. The eigenvalue space
  that crystals live in. Five spectral dimensions.
- `coincidence/src/declaration.rs` — `MirrorFragment`, the
  content-addressed crystal type.
- `mirror/src/mirror_runtime.rs` — `Shatter`'s `Prism` impl. `focus`,
  `project`, `refract` are filled. `split` and `zoom` are conservative
  no-ops with TBD comments. The TBD on `split` is resolved by the
  companion insight `2026-04-07-the-chain-is-the-shatter.md`: `|` is
  `split`.
- `spectral/docs/gen_prism.md` — the planned BEAM runtime backend that
  emits actor-coordinated bytecode from a `.shatter`.
