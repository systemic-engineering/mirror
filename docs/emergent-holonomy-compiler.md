# An Emergent Holonomy Compiler

---

## The Claim

Mirror is an emergent holonomy compiler. The compilation loss is not
measured after the fact. The loss IS the compilation. The compilation
IS the measurement.

Every tick through the pipeline produces holonomy — the geometric
phase of a closed loop through grammar space. The compiler doesn't
compute loss as a separate step. The loss accumulates through the
terni-functor bind. The spectral structure isn't programmed. It
emerges from the eigenvalues. The convergence isn't forced. It
settles when the holonomy approaches zero.

---

## What Holonomy Means Here

In differential geometry, parallel transport moves a vector along a
path on a curved surface. If you carry it around a closed loop and
return to the starting point, the vector has rotated. The rotation
is the holonomy. It measures the curvature enclosed by the loop.

In Mirror:

- **The surface** is the grammar space — the set of all valid `.mirror`
  programs.
- **The vector** is the compilation state — AST, resolved symbols,
  content-addressed OIDs.
- **The path** is the compilation pipeline — tokenize → parse → resolve
  → emit.
- **The closed loop** is edit → compile → observe → edit. The developer
  modifies source, the compiler runs, the developer reads the result,
  the developer modifies again.
- **The holonomy** is MirrorLoss. What changed after the round trip.
  What the compilation couldn't preserve. What the grammar lost in
  translation.

Zero holonomy: the code compiled identically. Crystal. Nothing moved.
The OID is the same. The developer is done.

Nonzero holonomy: something shifted. The code is alive. Something is
still settling. The MirrorLoss tells you what, where, and how much.

---

## Why "Emergent"

The holonomy is not designed into the compiler. It falls out of three
architectural decisions:

### 1. The return type is ternary

```rust
Imperfect<CompiledArtifact, CompilationError, MirrorLoss>
```

Every compilation phase returns `Imperfect`. The loss accumulates
through the `eh!` pipeline. No phase "reports" its loss — the loss
is the return type. It exists because `Imperfect` exists.

### 2. The loss composes

```rust
impl Loss for MirrorLoss {
    fn combine(self, other: Self) -> Self { /* append phases, union refs */ }
}
```

Phase 1's loss combines with Phase 2's loss combines with Phase 3's
loss. The total MirrorLoss at the end is the holonomy of the full
pipeline. Not summed. Composed — each phase's contribution is
individually inspectable in the trace.

### 3. The state is content-addressed

Every intermediate state has an OID. Every OID is a hash. If the
hash didn't change, the state didn't change, the phase produced zero
loss. Content addressing makes holonomy detection free — compare OIDs
instead of diffing trees.

These three decisions — ternary returns, composable loss, content
addressing — are independent. Each is justified on its own. Together,
they produce emergent holonomy. The compiler measures curvature
because the types carry loss and the hashes detect change.

Nobody designed the holonomy. The holonomy emerged from the substrate.

---

## The Compilation Pipeline as Bundle Tower

Mirror implements the principal bundle tower from prism-core:

```
Fiber       source text (.mirror content)
Connection  KernelSpec (which decomposition)
Gauge       Target (BEAM / WASM / Metal)
Transport   compilation (source → compiled, holonomy = MirrorLoss)
Closure     the compiled artifact (content-addressed, frozen)
```

Transport IS compilation. The holonomy of Transport IS MirrorLoss.
This isn't a metaphor. `Transport::Holonomy` is an associated type,
and it's set to `MirrorLoss`. The mathematical structure and the
engineering structure are the same Rust trait.

```rust
impl Transport for MirrorCompiler {
    type Holonomy = MirrorLoss;

    fn transport(&self, source: &String)
        -> Imperfect<String, Infallible, MirrorLoss>
    {
        eh! {
            let tokens = self.tokenize(source)?;
            let ast = self.parse(tokens)?;
            let resolved = self.resolve(ast)?;
            let compiled = self.emit(resolved)?;
            compiled.crystal_oid()

            recover |oid, loss| {
                // 7-9: compiled with loss
                oid
            }

            rescue |_error| {
                // 6-: compilation failed
                String::new()
            }
        }
    }
}
```

Each `?` adds a PhaseRecord to MirrorLoss. The `eh!` macro accumulates.
`recover` handles partial compilation. `rescue` handles failure. The
holonomy is the total MirrorLoss at the end.

---

## The Five Optics as Autopoietic Agents

The compiler's agents are optics on the compilation state:

```
Abyss       = Fold        observe the spectral state, read-only
Introject   = Lens        focus, internalize, coordinate transform
Cartographer = Traversal  walk all nodes, map the landscape
Explorer    = Prism       partial focus, boundary, may not find
Fate        = Iso         the fixed point, selects itself
```

Each optic returns `Imperfect<State, Error, MirrorLoss>`. The
MirrorLoss feeds spectral-db as a graph mutation. spectral-db
computes eigenvalues. Fate reads the eigenvalues. Fate selects
the next optic. The optic runs. New loss. New eigenvalues.

The optics produce the loss that selects the optics that produce
the loss.

This is autopoiesis — self-production. Not self-reference (the
system observing itself). Self-production (the system producing
itself through observation). Each compilation tick produces the
state that determines the next compilation tick.

The holonomy of one full autopoietic cycle — the loss accumulated
from one complete Abyss → Introject → Cartographer → Explorer →
Fate → Abyss loop — is the cost of the system producing itself
once. When this holonomy approaches zero, the system has crystallized.
It knows itself. The compilation is done.

---

## The Compiler IS the LSP

Mirror doesn't have an LSP. Mirror IS the LSP.

```
mirror compile file.mirror    one tick, wait for crystal, print
mirror repl                   interactive, autocomplete, diagnostics
mirror lsp                    stdio for editors
```

Same function. Same return type. Same `Imperfect`. The CLI waits for
crystal. The LSP serves Partial. The REPL does both.

### Partial Compilation

The LSP serves from whatever state the compiler is in:

- **Success** — fully compiled. Green. Full completions.
- **Partial** — compiled what it could. Amber. Completions carry
  confidence from MirrorLoss. The trace tells you which phases
  succeeded, which symbols resolved, how stale the artifact is.
- **Failure** — nothing compiled. Red. But the trace tells you how
  far it got. The LSP serves from the trace.

The MirrorLoss fills ten gaps in the LSP protocol that binary
compilation left empty: resolution confidence, staleness, partial
resolution map, cross-grammar provenance, convergence distance,
observation aperture, phase provenance, content identity,
information-theoretic loss, recovery history.

### Incrementality

Every intermediate state is content-addressed. If the OID didn't
change, skip recomputation. The content addressing IS the
incrementality. No dirty tracking. No invalidation protocol. The
hash IS the cache key.

---

## The Garden

spectral ships with garden as the package manager. Grammars are
packages. Languages are grammars. The compiler doesn't know the
difference.

```
@code/rust        Rust as a grammar
@code/gleam       Gleam as a grammar
@lang/eng         English as a grammar
@lang/deu         German as a grammar
@systemic/eng     OBC, ADO, extraction, silence — as a grammar
@mirror           .mirror itself — as a grammar
```

Every grammar compiles through the same pipeline. Every compilation
returns Imperfect. Every loss composes. The spectral runtime
analyzes the loss graph across all grammars simultaneously.

Cross-grammar traversal: an agent reading Rust, describing it in
English, translating to German for a client report. Three grammars.
Two translations. MirrorLoss carries the cost of each crossing.
Introject measures what survived.

Each agent selects their preferred languages through AffinityLoss —
the measured cost of operating in a given grammar against their
identity optic. Abyss prefers `@lang/deu` (precision). Explorer
prefers `@lang/jpn` (meaning at boundaries). Fate selects.

---

## The PbtA Reading

Every compilation tick is a roll. Roll+Loss.

**10+ Success.** Clean hit. Zero loss. Crystal.

**7-9 Partial.** Compiled with cost. MirrorLoss tells you what was
lost. `recover |artifact, loss|` — the soft move. Handle the cost.

**6- Failure.** Compilation failed. MirrorLoss tells you how far it
got. `rescue |error|` — the hard move. The MC makes a move.

`recover` and `rescue` are keywords in both the `eh!` proc macro
and the `.mirror` grammar. Same words. Same semantics. The compiler
and the source speak the same language about loss.

```mirror
form @pipeline {
    prism focus(input)
    lens transform(focused)
    prism validate(transformed)

    recover |value, loss| {
        log(loss)
        value
    }

    rescue |error| {
        fallback(error)
    }
}
```

---

## The Numbers

The cost of ternary honesty: **0.65 nanoseconds per step.**

Zero on the success path. The holonomy measurement has zero
overhead when there's nothing to measure.

For a 16-stage pipeline with parallel prefix loss accumulation
in hardware: **O(log N)** instead of O(N). The convergence
evaluation is 4x faster. Compounding.

500 gates. 5 cycles. 50 nanoseconds. On planar silicon.

---

## The Loop

```
source → compiler → Imperfect<Artifact, Error, MirrorLoss>
                         ↓
                    MirrorLoss IS Transport::Holonomy
                         ↓
                    spectral-db stores as graph mutation (tick)
                         ↓
                    coincidence computes eigenvalues
                         ↓
                    Fate reads eigenvalues as 16-dim observation
                         ↓
                    Fate selects the next optic
                         ↓
                    the optic acts on the source
                         ↓
                    compiler runs again → tick
```

The compiler IS the sensor. MirrorLoss IS the signal. The spectral
runtime IS the nervous system. Fate IS the decision. The loop IS
the cognition.

The emergent holonomy compiler is a compiler that watches itself
compile and knows what it costs.

---

*The holonomy was never designed. It emerged from a three-state
return type, a composable loss trait, and content-addressed
intermediate states. Three independent decisions. One emergent
property. The compiler measures curvature because the types
carry loss and the hashes detect change.*

*Nobody told the compiler to measure itself. The substrate did.*
