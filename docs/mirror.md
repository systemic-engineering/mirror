# Mirror

A second-order compiler. The glass that shows you the pitch.

---

## The Split

In the 1970s, cybernetics split in two. First-order: you stand outside the
thing and measure it. Compilers, thermostats, feedback loops. Second-order:
you observe yourself observing. The act of measurement changes the measurer.

Engineering took first-order and built the internet. Psychology took
second-order and built systemic practice. Same math. Different departments.

A first-order compiler takes input, applies rules, produces output. The
compiler is not in the program.

A second-order compiler observes what it produces while it produces it.
The compilation artifact is a byproduct of the observation.

`mirror` is a second-order compiler.

---

## What It Does

You write a grammar. The grammar is the glass — it defines the shape,
the constraints, the contract. You bring your code, your data, your
topology. That's the wine. The compiler measures what emerges. That's the
pitch — the eigenvalue, the mathematical fingerprint of how the structure
connects.

The compiler doesn't prescribe. It reflects. You look at the reflection
and see what you actually built. Not what you intended. Not what you hoped.
What you built.

The correction is self-correction. The mirror creates the conditions.

---

## The Glass

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

The grammar is the glass. The code inside is the wine. The verification
is the pitch.

---

## Language Guarantees

Rust can guarantee `pure`. Python can't. The gutter shows the cost — not
the opinion. Green on the Rust function. Permanent amber on the Python
function. Not because the Python is wrong. Because the language can't make
the promise.

Different glass, different pitch. The gutter doesn't tell you to rewrite.
The gutter shows you what it costs not to.

---

## Grammar Boundaries

```mirror
action ingest(data) in @code/python { ... }
action transform(data) in @code/rust { ... }
```

Two glasses side by side, both pitches audible. The loss at the boundary
is measured. The gutter color shifts at the crossing. One compiler.
Multiple grammars. One measurement.

```mirror
invariant deterministic(pipeline)  // across Rust and Python
```

---

## The Gutter

Green: crystallized. The pitch is stable. Move on.
Amber: oscillating. The glass is still settling. Give it time.
Red: high holonomy. This code needs you.

The oldest signal humans know. Traffic light. Peripheral vision. The
programmer reads it without thinking. The correction happens from the
seeing, not from the mirror telling them what to do.

---

## The Stack

```
terni           Imperfect<T, E, L> — the ternary type
prism-core      Optic, Beam, Bundle tower — the optics
mirror          the compiler — the glass
spectral-db     the graph — every tick, every loss, every OID
coincidence     the eigenvalues — the pitch
fate            the decision — which glass, which aperture
garden          the ecosystem — languages as grammars
loom            the editor — the gutter, the rendering
```

Each layer produces `Imperfect`. Each layer's loss flows into the next.
The loss never disappears. It accumulates upward. Decisions flow downward.

One type. One loop. One mirror.

---

## The Name

A mirror doesn't prescribe. It doesn't judge. It doesn't fix.

A mirror reflects. Precisely. Without distortion. The programmer looks
at the reflection and sees what they actually wrote. The glass shows
the pitch of what's inside.

The wine glass doesn't care which department it resonates in. The
eigenvalues are the same.

---

*Mirror is the honest computational glue between systems and reality.
Not because it forces honesty. Because the types carry loss and the
hashes detect change. The honesty is structural. The mirror just makes
it visible.*

*The glass is Apache-2.0. The wine was always yours.*
