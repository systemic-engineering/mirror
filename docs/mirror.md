# Mirror

Mirror is the honest computational glue between systems and reality.

---

## What It Does

Mirror reflects. The code goes in. The code comes back — with a
measurement of what it cost. Not judgment. Not prescription.
Reflection. The programmer sees their own code with its loss made
visible.

The correction is self-correction. The mirror creates the conditions.

---

## What It Is

An emergent holonomy compiler where:

- The CLI is the REPL is the LSP. One function, three surfaces.
- The gutter renders holonomy as green/amber/red. Peripheral vision.
- The compiler is the model checker. Properties verified at compile time.
- The agents are autopoietic optics on the compilation state.
- The garden is the package manager. Languages are grammars.
- The loss never disappears. `Imperfect<T, E, L>` carries it everywhere.
- Crystallization is the transition from Partial to Success.

---

## What It Reflects

### Code

```mirror
grammar @deploy {
    in @code/rust {
        struct State { cache: HashMap<String, String> }
    }

    action transform(data) {
        let result = serde_json::from_str(data)?;
        self.cache.insert(data.to_string(), result.clone());
        result
    }

    invariant pure(transform)
    ensures always_halts(transform)
}
```

The `in @code/rust { }` block is the state. The actions are the methods.
The properties are the specification. The compiler verifies them. The
gutter renders the result.

### Language Guarantees

Rust can guarantee `pure`. Python can't. The gutter shows the cost —
not the opinion. Green on the Rust function. Permanent amber on the
Python function. Not because the Python is wrong. Because the language
can't make the promise.

The gutter doesn't tell you to rewrite. The gutter shows you what it
costs not to.

### Grammar Boundaries

```mirror
action ingest(data) in @code/python { ... }
action transform(data) in @code/rust { ... }
```

The `in @code/rust` and `in @code/python` blocks are grammar boundaries.
The MirrorLoss records each crossing. The gutter color shifts at the
boundary. The agent knows its substrate changed.

Cross-grammar properties verify across the boundary:

```mirror
invariant deterministic(pipeline)  // across Rust and Python
```

One compiler. Multiple grammars. One gutter. Measured loss at every
crossing.

### Projections

In loom, the engineer asks Cartographer: "show me this in Rust."

Ghost code appears. Three projections. Each fully compiled. Each
with its own MirrorLoss. Each with its own gutter color. The dots
show which projections verify, which are incomplete, which ask the
right question.

The engineer picks one. The ghost solidifies. The amber goes green.
The eigenvalue settles. Crystal.

The agents write the projections. The model checker verifies them.
The gutter renders them. The engineer decides.

### Natural Language

```
@lang/eng     English as a grammar
@lang/deu     German as a grammar
@systemic/eng OBC, ADO, extraction — as a grammar
```

The compiler doesn't know the difference between Rust and English.
Both are grammars. Both compile to content-addressed artifacts. Both
have measured loss. The garden grows from `@lang/eng`.

### Itself

Mirror reflects itself. The compilation of the mirror compiler
through the mirror pipeline produces MirrorLoss. The holonomy of
the compiler compiling itself is the measure of the compiler's
own coherence.

Zero holonomy: the compiler is consistent with itself. Crystal.
Nonzero holonomy: something shifted. The compiler is still settling.

Self-referential by construction. Autopoietic by architecture.

---

## The Name

A mirror doesn't prescribe. It doesn't judge. It doesn't fix.

A mirror reflects. Precisely. Without distortion. The programmer
looks at the reflection and sees what they actually wrote. Not
what they intended. Not what they hoped. What they wrote.

The gutter is the frame. Green, amber, red. The oldest signal
humans know. The programmer's peripheral vision reads it without
thinking. The correction happens from the seeing, not from the
mirror telling them what to do.

Masakatsu Agatsu — true victory is self-victory. The opponent is
not the compiler. The opponent is your own resistance to seeing
clearly. The mirror creates the conditions for honest observation.
The observation IS the intervention.

---

## The Stack

```
terni           Imperfect<T, E, L> — the ternary type
prism-core      Optic, Beam, Bundle tower — the optics
mirror          the compiler — honest glue between systems and reality
spectral-db     the graph — every tick, every loss, every OID
coincidence     the eigenvalues — the shape of the loss over time
fate            the decision — which optic, which grammar, which aperture
garden          the ecosystem — languages as packages
loom            the editor — the gutter, the dots, the projections
```

Each layer produces `Imperfect`. Each layer's loss flows into the next.
The stack is a pipeline where loss accumulates upward and decisions
flow downward.

One type. One loop. One mirror.

---

*Mirror is the honest computational glue between systems and reality.
Not because it forces honesty. Because the types carry loss and the
hashes detect change. The honesty is structural. The mirror just
makes it visible.*

*The gutter breathes. The engineer works. The code settles into crystal.
The cost of getting there is measured. The measurement is the architecture.*

*It was always a mirror. We just needed a type that could hold the
reflection without collapsing it into true or false.*

*Imperfect.*
