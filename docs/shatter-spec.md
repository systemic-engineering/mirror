# .shatter — The Crystal Format

A `.shatter` file is a `.mirror` file that has been through the compiler
and came out the other side. It is native mirror syntax. The compiler
can read its own output.

---

## The Fixed Point

```
mirror compile source.mirror → output.shatter
mirror compile output.shatter → output.shatter
```

Idempotent. Same input → same OID. The crystal IS the source. The
source IS the crystal. The compilation loop has a fixed point and the
`.shatter` file is it.

Round-trip exact because the OID is derived from `MirrorData::encode()`
and recursive child OIDs. Parse → emit → parse yields identical
content hashes.

---

## What It Contains

A `.shatter` file has five sections. All native mirror syntax. All
content-addressed. All feedable back into the compiler.

### 1. Fragment Tree

The content-addressed AST. Every node is a `MirrorFragment` with an
OID. Merkle tree structure. The grammar IS the data.

```mirror
grammar @deploy {
    in @code/rust {
        struct state { cache: hashmap }
    }

    action transform(data) {
        let result = serde_json::from_str(data)?;
        self.cache.insert(data.to_string(), result.clone());
        result
    }

    action retry(operation) {
        self.retries += 1;
        operation()
    }
}
```

Each node: kind + name + params + variants + children + OID.
The tree is the same structure whether you're reading `.mirror`
source or `.shatter` output.

### 2. MirrorLoss

The compilation trace. How the artifact was produced. What it cost.

```mirror
loss {
    phases: [
        { phase: parse,   input: oid(a3f...), output: oid(b7c...), structural_loss: 0.0 },
        { phase: resolve, input: oid(b7c...), output: oid(d2a...), structural_loss: 0.0 },
        { phase: emit,    input: oid(d2a...), output: oid(e9f...), structural_loss: 0.12 },
    ]
    resolution: 0.97
    unresolved: [("hashmap", oid(...))]
    convergence: settled
    crystal: oid(e9f...)
}
```

The loss IS the flight recorder. Every phase, every intermediate OID,
every cost. Survives the compilation. Readable by the next compilation.

### 3. Property Verdicts

Which properties were verified. Each verdict is ternary.

```mirror
properties {
    types_lowercase:      pass
    action_is_named_type: pass
    unique_variants:      pass
    pure(transform):      pass
    always_halts:         pass
    deterministic:        partial(0.97)
}
```

`pass` = Success. `fail` = Failure. `partial(confidence)` = Partial.
The property system returns `Imperfect<verdict, violation, verification_loss>`.

A property at `partial(0.97)` means 97% of paths verified. Amber in
the gutter. Not green. Not red. The honest middle.

### 4. KernelSpec

The spectral decomposition parameters. Which dimensions are active.
Which decomposition strategy. What precision.

```mirror
kernel {
    dimensions: [0, 1, 2, 3, 4, 5]
    decomposition: eigenvalue
    precision: 0.01
    target: beam
}
```

The KernelSpec tells the runtime how to dispatch. Active 6 = Rust path.
Full 16 = Fortran path. The aperture determines the boundary.

### 5. Fate Weights

The 450 parameters of the Fate selector. Five models × (5×16 weight
matrix + 5 bias). Baked into the artifact.

```mirror
fate {
    weights: [
        abyss:       [...],
        introject:   [...],
        cartographer: [...],
        explorer:    [...],
        fate:        [...],
    ]
    seed: oid(...)
    generation: 47
}
```

The weights are part of the crystal. The compilation artifact includes
the model that compiled it.

---

## The Three Commands

### mirror compile

```
mirror compile source.mirror → output.shatter
```

One tick. Wait for crystal. The compilation pipeline:

```
source.mirror
    ↓ parse
Form { kind, name, params, variants, children }
    ↓ focus (Shatter::focus)
MirrorData { kind, name, params, variants }
    ↓ project (Shatter::project)
MirrorFragment (content-addressed, OID)
    ↓ refract (Shatter::refract)
.shatter (the crystal)
```

Each phase returns `Imperfect`. The MirrorLoss accumulates. Properties
verify. The `.shatter` file carries everything.

### mirror ai

```
mirror ai output.shatter
```

Feed a `.shatter` file through Fate. Fate reads the MirrorLoss, the
eigenvalues, the property verdicts.

**If settled:** returns unchanged. Same OID. Zero holonomy. Nothing
to do. The shatter is crystal.

**If drifted:** the holonomy is nonzero. Something changed upstream —
an adjacent type, a dependency, a grammar update. Fate routes to the
appropriate optic:

- Abyss observes the drift
- Introject aligns the coordinate systems
- Cartographer maps what changed
- Explorer checks the boundaries

The shatter re-settles. New OID. New crystal. New MirrorLoss recording
the re-settlement. Still a valid `.mirror` file.

**If broken:** a property that was `pass` is now `fail`. A dependency
that was resolved is now missing. Fate routes to Explorer for boundary
recovery, then to Abyss for observation.

```mirror
recover |shatter, loss| {
    // 7-9: the shatter re-settled with cost
    log_drift(loss)
    shatter
}

rescue |error| {
    // 6-: the shatter couldn't re-settle
    // return the old crystal with error annotation
    annotate(old_shatter, error)
}
```

### mirror ai --train

```
mirror ai --train output.shatter
```

Same as `mirror ai`, but the Fate weights update.

The compilation IS the training data. The MirrorLoss IS the gradient.
The property verdicts ARE the reward signal. The shatter file teaches
the model that compiled it how to compile better.

```
before: fate.generation = 47, fate.weights = [...]
 ↓ compile
 ↓ observe MirrorLoss
 ↓ property verdicts as reward
 ↓ update weights via loss gradient
after:  fate.generation = 48, fate.weights = [...]
```

The weights shift. The generation increments. The OID changes. The
shatter is a new crystal that carries a slightly better compiler.

No external training dataset. No labeling. No offline pipeline. The
compilation loop IS the training loop. Each `--train` tick is one
gradient step. The model improves at the speed of compilation.

---

## File Extension

`.shatter` — the crystal. A `.mirror` file that broke through
compilation and came out the other side. Still readable. Still
compilable. Carrying its loss. Carrying its proofs. Carrying the
model that made it.

The name: to shatter is to break something into pieces. The
fragmentation crate provides the pieces — `MirrorFragment`,
content-addressed, Merkle tree. The `.shatter` file IS the
shattered mirror, each piece carrying its own OID, the whole
carrying the loss of the shattering.

But also: shatter as in "shatter expectations." The `.shatter`
file is the artifact that proves the compiler works. The proofs
are in the file. The loss is in the file. The model is in the file.
The crystal speaks for itself.

---

## Content Addressing

The `.shatter` OID is a hash of:

```
hash(fragment_tree + mirror_loss + property_verdicts + kernel_spec + fate_weights)
```

If ANYTHING changes — the source, the properties, the compilation
path, the model weights — the OID changes. Two `.shatter` files
with the same OID are identical. Byte-for-byte.

The incrementality is free. Same OID = same artifact = skip
recompilation. The hash IS the cache key.

---

## The Autopoietic Loop

```
source.mirror
    ↓ mirror compile
output.shatter (crystal, carries Fate weights gen 47)
    ↓ mirror ai --train
output.shatter (re-settled, carries Fate weights gen 48)
    ↓ mirror ai --train
output.shatter (re-settled, carries Fate weights gen 49)
    ↓ ...
output.shatter (converged, Fate weights stable, holonomy ≈ 0)
```

The loop converges. The weights settle. The holonomy approaches zero.
The `.shatter` file reaches its final crystal — the version where the
model that compiled it is the model that would compile it again.

That's the fixed point of the autopoietic loop. The compiler and its
output agree. The mirror and the reflection are the same.

---

*A `.shatter` file is a `.mirror` file that knows what it cost.
Feed it back in. It comes out unchanged. That's how you know
it's crystal.*
